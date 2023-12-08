use crate::chat::MODIFY;
use crate::data::{Friend, Presence, IN_XML_MESSAGES, OUT_XML_MESSAGES};
use crate::WEB_PORT;

use super::{FRIENDS, PRESENCES, SESSION};
use actix_web::{delete, get, put, web, App, HttpServer, Responder};

#[get("/friends")]
async fn friends() -> impl Responder {
    let data: Vec<Friend> = FRIENDS.read().unwrap().values().cloned().collect();

    web::Json(data)
}

#[get("/presences")]
async fn presences() -> impl Responder {
    let data: Vec<Presence> = PRESENCES.read().unwrap().values().cloned().collect();

    web::Json(data)
}

#[get("/session")]
async fn sessions() -> impl Responder {
    let data = SESSION.read().unwrap().clone();
    web::Json(data)
}

#[get("/xml/incoming")]
async fn xml_incoming() -> impl Responder {
    let data = IN_XML_MESSAGES.read().unwrap().clone();
    let data: Vec<String> = data
        .iter()
        .map(|bytes| String::from_utf8_lossy(bytes).into_owned())
        .collect();
    web::Json(data)
}

#[get("/xml/outgoing")]
async fn xml_outgoing() -> impl Responder {
    let data = OUT_XML_MESSAGES.read().unwrap().clone();
    let data: Vec<String> = data
        .iter()
        .map(|bytes| String::from_utf8_lossy(bytes).into_owned())
        .collect();
    web::Json(data)
}

#[put("/middleware")]
async fn add_middleware(middleware: String) -> impl Responder {
    let res = MODIFY.write().await.add_middleware(middleware).await;
    if res.is_err() {
        return (
            "Incorrect address".to_string(),
            http::StatusCode::BAD_REQUEST,
        );
    }
    ("Ok".to_string(), http::StatusCode::OK)
}

#[delete("/middleware")]
async fn rm_middleware(middleware: String) -> impl Responder {
    let res = MODIFY.write().await.remove_middleware(middleware).await;
    if !res {
        return ("Middleware doesn't exist", http::StatusCode::GONE);
    }
    ("Ok", http::StatusCode::OK)
}

#[actix_web::main]
pub async fn data_server() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(friends)
            .service(presences)
            .service(sessions)
            .service(add_middleware)
            .service(rm_middleware)
            .service(xml_outgoing)
            .service(xml_incoming)
    })
    .bind(("127.0.0.1", WEB_PORT))?
    .run()
    .await
}
