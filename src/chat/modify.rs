use std::sync::Arc;

use once_cell::sync::Lazy;
use tokio::sync::{Mutex, RwLock};

use indexmap::IndexSet;

use http::{HeaderValue, Method, Request};
use hyper::{Body, Client};

use crate::client::get_body;

use super::Handler;

use thiserror::Error;

pub static MODIFY: Lazy<RwLock<ModifyXMPP>> = Lazy::new(|| RwLock::new(ModifyXMPP::new()));

#[derive(Error, Debug)]
pub enum ModifyError {
    #[error("from hyper")]
    HyperError(#[from] hyper::Error),
    #[error("no modification took place")]
    NoModify,
}

#[derive(Clone, Debug)]
pub struct ModifyXMPP {
    middlewares: Arc<Mutex<IndexSet<String>>>,
}

impl ModifyXMPP {
    pub fn new() -> Self {
        ModifyXMPP {
            middlewares: Arc::new(Mutex::new(IndexSet::new())),
        }
    }

    pub async fn add_middleware(
        &mut self,
        middleware: String,
    ) -> Result<(), std::net::AddrParseError> {
        let _ = middleware.parse::<std::net::SocketAddr>()?;
        self.middlewares.lock().await.insert(middleware);
        Ok(())
    }

    pub async fn remove_middleware(&mut self, middleware: String) -> bool {
        self.middlewares.lock().await.remove(&middleware)
    }

    /// Will hang if the server is unresponsive which is bad
    pub async fn run_request(
        self,
        data: &Vec<u8>,
        handler_type: &Handler,
    ) -> Result<Vec<u8>, ModifyError> {
        let mut data = data.to_vec();
        for server in self.middlewares.lock().await.iter() {
            match modify_http(&data, server, handler_type).await {
                Err(err) => match err {
                    ModifyError::NoModify => (),
                    _ => return Err(err),
                },
                Ok(new) => data = new,
            }
        }
        Ok(data)
    }
}

fn gen_request(data: &Vec<u8>, server: &str, handler_type: &Handler) -> Request<Body> {
    let path = match handler_type {
        Handler::Incoming => "/incoming",
        Handler::Outgoing => "/outgoing",
    };
    let uri = format!("{}{}", server, path);
    let body = data.clone();
    let body = Body::from(body);
    let content = HeaderValue::from_str("application/xml").unwrap();
    let request = Request::builder()
        .uri(uri)
        .method(Method::POST)
        .header("content-type", content)
        .body(body)
        .unwrap();

    request
}

async fn modify_http(
    data: &Vec<u8>,
    server: &str,
    handler_type: &Handler,
) -> Result<Vec<u8>, ModifyError> {
    let client = Client::new();
    let request = gen_request(data, server, handler_type);
    let mut res = client.request(request).await?;
    if res.status().as_u16() == 204 {
        return Err(ModifyError::NoModify.into());
    }
    let data = get_body(&mut res).await.unwrap();
    Ok(data.to_vec())
}
