use hyper::{Body, Request};

use simple_proxy::{
    middlewares::router::MatchedRoute,
    proxy::{
        error::MiddlewareError,
        middleware::{
            Middleware,
            MiddlewareResult::{self, Next},
        },
        service::{ServiceContext, State},
    },
};

use async_trait::async_trait;

use serde_json::to_string as json;

use super::utils::inject_new_uri;

#[derive(Clone)]
pub struct Redirector {}

#[async_trait]
impl Middleware for Redirector {
    fn name() -> String {
        String::from("Rediretor")
    }

    fn before_request(
        &mut self,
        req: &mut Request<Body>,
        context: &ServiceContext,
        state: &State,
    ) -> Result<MiddlewareResult, MiddlewareError> {
        let path = req.uri().path_and_query().cloned();
        let riot = "clientconfig.rpg.riotgames.com";
        inject_new_uri(req, riot, path)?;
        self.set_state(
            context.req_id,
            state,
            json(&MatchedRoute {
                uri: req.uri().to_string(),
                public: true,
            })?,
        )?;
        Ok(Next)
    }
}

impl Redirector {
    pub fn new() -> Self {
        Redirector {}
    }
}

impl Default for Redirector {
    fn default() -> Self {
        Self::new()
    }
}
