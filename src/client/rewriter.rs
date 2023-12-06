use std::collections::HashMap;

use http::Response;
use hyper::Body;

use simple_proxy::proxy::{
    error::MiddlewareError,
    middleware::{
        Middleware,
        MiddlewareResult::{self, Next},
    },
    service::{ServiceContext, State},
};

use async_trait::async_trait;

use serde_json::{Map, Value};

use super::utils::{get_body_json, is_json};
use crate::{
    chat::{REMOTE, SERVERS},
    CHAT_PORT,
};

#[derive(Clone)]
pub struct Rewriter {}

fn repl_affinities(obj: &mut Map<String, Value>) -> Option<()> {
    let affinities = obj.get_mut("chat.affinities")?.as_object_mut()?;
    let mut i = 1;
    let mut d: HashMap<String, String> = HashMap::new();
    for o in affinities {
        i += 1;
        let local_serv = format!("{}{}", "127.1.0.", i);
        let remote = o.1.as_str()?.to_owned();
        d.insert(local_serv.clone(), remote);
        *o.1 = local_serv.into();
    }
    SERVERS.get_or_init(|| d);
    Some(())
}

fn set_port(obj: &mut Map<String, Value>) -> Option<()> {
    let port = u16::try_from(obj.get("chat.port")?.as_number()?.as_u64()?).unwrap();
    REMOTE.get_or_init(|| port);
    Some(())
}

fn set_local(obj: &mut Map<String, Value>) -> Result<(), MiddlewareError> {
    if !obj.contains_key("chat.affinities") {
        return Ok(());
    }
    let _ = repl_affinities(obj);
    let _ = set_port(obj);
    obj.extend([
        ("chat.port".to_owned(), CHAT_PORT.into()),
        ("chat.allow_bad_cert.enabled".to_owned(), true.into()),
        ("chat.host".to_owned(), "127.0.0.1".into()),
    ]);
    tokio::spawn(crate::chat::serv());
    Ok(())
}

#[async_trait]
impl Middleware for Rewriter {
    fn name() -> String {
        String::from("Rewriter")
    }

    async fn after_request(
        &mut self,
        _res: Option<&mut Response<Body>>,
        _ctx: &ServiceContext,
        _state: &State,
    ) -> Result<MiddlewareResult, MiddlewareError> {
        if let Some(res) = _res {
            if is_json(res) {
                let mut body = get_body_json(res).await?;
                if let Some(obj) = body.as_object_mut() {
                    let _ = set_local(obj);
                    *res.body_mut() = serde_json::to_vec(&obj)?.into();
                }
            }
        }
        Ok(Next)
    }
}

impl Rewriter {
    pub fn new() -> Self {
        Rewriter {}
    }
}

impl Default for Rewriter {
    fn default() -> Self {
        Self::new()
    }
}
