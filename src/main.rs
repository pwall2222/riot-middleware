pub mod chat;
pub mod client;
pub mod data;
mod ports;
mod server;

use simple_proxy::{Environment, SimpleProxy};

use crate::client::{Redirector, Rewriter};
use crate::ports::MITM_PORT;
use crate::server::data_server;

pub use crate::ports::{CHAT_PORT, WEB_PORT};

#[tokio::main]
async fn main() {
    let mut proxy = SimpleProxy::new(MITM_PORT, Environment::Development);
    let redirector = Redirector::new();
    let rewriter = Rewriter::new();

    proxy.add_middleware(Box::new(redirector)).await;
    proxy.add_middleware(Box::new(rewriter)).await;

    std::thread::spawn(data_server);

    let _ = proxy.run().await;
}
