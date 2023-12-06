use std::sync::Arc;

use tokio::{net::TcpStream, sync::Mutex};

use tokio::io::{ReadHalf, WriteHalf};
use tokio_native_tls::TlsStream;

pub type Stream = TlsStream<TcpStream>;
pub type Reader = ReadHalf<Stream>;
pub type Writer = WriteHalf<Stream>;
pub type WriterArc = Arc<Mutex<Writer>>;

#[derive(Debug, Clone, Copy)]
pub enum Handler {
    /// Data coming from the external XMPP server
    Incoming,
    /// Data coming from the RiotClient
    Outgoing,
}
