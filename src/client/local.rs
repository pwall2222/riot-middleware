use std::str::FromStr;

use http::{Request, Response, Uri};
use hyper::{client::HttpConnector, Body, Client};

use simple_proxy::proxy::{
    error::MiddlewareError,
    middleware::{
        Middleware,
        MiddlewareResult::{self, Next},
    },
    service::{ServiceContext, State},
};

use async_trait::async_trait;

use super::utils::{get_body, inject_new_uri};

#[derive(Clone)]
pub struct Local {
    host: String,
    client: Client<HttpConnector>,
}

#[async_trait]
impl Middleware for Local {
    fn name() -> String {
        String::from("Local")
    }

    fn before_request(
        &mut self,
        req: &mut Request<Body>,
        ctx: &ServiceContext,
        state: &State,
    ) -> Result<MiddlewareResult, MiddlewareError> {
        let _ = self.set_state(ctx.req_id, state, req.uri().to_string());
        Ok(Next)
    }

    async fn after_request(
        &mut self,
        _res: Option<&mut Response<Body>>,
        ctx: &ServiceContext,
        state: &State,
    ) -> Result<MiddlewareResult, MiddlewareError> {
        if let Some(res) = _res {
            let uri = Uri::from_str(&self.get_state(ctx.req_id, state)?.unwrap())?;
            let path: Option<http::uri::PathAndQuery> = uri.path_and_query().cloned();
            let body = get_body(res).await?;
            let mut req = Request::new(Body::from(body));
            inject_new_uri(&mut req, &self.host, path)?;
            let local = self.client.request(req).await?;
            *res.body_mut() = local.into_body();
        }
        Ok(Next)
    }
}

impl Local {
    pub fn new(host: &str) -> Self {
        Local {
            host: host.to_string(),
            client: Client::new(),
        }
    }
}
