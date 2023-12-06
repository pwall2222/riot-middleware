use once_cell::sync::Lazy;
use std::sync::RwLock;

use crate::chat::Handler;

use super::handle_data;

pub static IN_XML_MESSAGES: Lazy<RwLock<Vec<Vec<u8>>>> = Lazy::new(|| RwLock::new(Vec::new()));
pub static OUT_XML_MESSAGES: Lazy<RwLock<Vec<Vec<u8>>>> = Lazy::new(|| RwLock::new(Vec::new()));

pub async fn data_server(data: &Vec<u8>, handler: Handler) {
    let space = match handler {
        Handler::Incoming => &IN_XML_MESSAGES,
        Handler::Outgoing => &OUT_XML_MESSAGES,
    };
    let mut messages = space.write().unwrap();
    messages.push(data.to_vec());
    match handler {
        Handler::Outgoing => (),
        Handler::Incoming => {
            handle_data(data);
        }
    }
}
