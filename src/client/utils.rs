use http::{uri::Parts, HeaderValue, Request, Response, Uri};
use hyper::{body::Bytes, Body};

use simple_proxy::proxy::error::MiddlewareError;

use serde_json::Value;

pub fn is_json(res: &Response<Body>) -> bool {
    if let Some(content) = res.headers().get("content-type") {
        if let Ok(content) = content.to_str() {
            return content == "application/json";
        }
    }
    false
}

pub async fn get_body(res: &mut Response<Body>) -> Result<Bytes, MiddlewareError> {
    let body = res.body_mut();
    let body = hyper::body::to_bytes(body).await?;
    Ok(body)
}

pub async fn get_body_json(res: &mut Response<Body>) -> Result<Value, MiddlewareError> {
    let body = get_body(res).await?;
    let value: Value = serde_json::from_slice(&body)?;
    Ok(value)
}

pub fn inject_new_uri(
    req: &mut Request<Body>,
    host: &str,
    path: Option<http::uri::PathAndQuery>,
) -> Result<(), MiddlewareError> {
    {
        let headers = req.headers_mut();
        headers.insert("host", HeaderValue::from_str(host).unwrap());
        headers.remove("accept-encoding");
    }

    let mut parts = Parts::default();
    parts.scheme = Some("https".parse()?);
    parts.authority = Some(host.parse()?);
    parts.path_and_query = path;

    *req.uri_mut() = Uri::from_parts(parts)?;

    Ok(())
}
