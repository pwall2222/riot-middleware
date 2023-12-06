use anyhow::Result;
use std::{
    net::{SocketAddr, ToSocketAddrs},
    sync::Arc,
};

use tokio::{
    net::{TcpListener, TcpStream},
    sync::Mutex,
};

use native_tls::{Identity, TlsConnector};

use tokio_native_tls::{self, TlsAcceptor, TlsStream};

use crate::CHAT_PORT;

use super::{handler, Handler, REMOTE, SERVERS};

fn get_remote(addr: &String) -> Option<(SocketAddr, &String)> {
    let host = SERVERS.get()?.get(addr)?;
    let port = *REMOTE.get()?;
    let addr = format!("{}:{}", host, port);
    let sock = addr.to_socket_addrs().ok()?.next()?;
    Some((sock, host))
}

async fn setup_server() -> Result<(TcpListener, TlsAcceptor)> {
    let addr = ("0.0.0.0", CHAT_PORT);
    let tcp: TcpListener = TcpListener::bind(&addr).await?;
    let der = include_bytes!("../../static/cert.p12");
    let cert = Identity::from_pkcs12(der, "ritoxmpp")?;
    let tls_acceptor = TlsAcceptor::from(native_tls::TlsAcceptor::builder(cert).build()?);
    Ok((tcp, tls_acceptor))
}

async fn setup_connection(
    tcp: &TcpListener,
    tls_acceptor: &TlsAcceptor,
) -> Result<(TlsStream<TcpStream>, TlsStream<TcpStream>)> {
    let (socket, _) = tcp.accept().await?;
    let local = &socket.local_addr()?.ip().to_string();
    let (remote, domain) = get_remote(local).unwrap();

    let tls_acceptor = tls_acceptor.clone();

    let client = TcpStream::connect(&remote).await?;
    let cx = TlsConnector::builder().build()?;
    let cx = tokio_native_tls::TlsConnector::from(cx);

    let client = cx
        .connect(domain.as_str(), client)
        .await
        .expect("accept error");
    let server = tls_acceptor.accept(socket).await.expect("accept error");
    Ok((client, server))
}

pub async fn serv() -> Result<()> {
    let (tcp, tls_acceptor) = setup_server().await?;

    loop {
        let (client, server) = setup_connection(&tcp, &tls_acceptor).await?;
        let (client_reader, client_writer) = tokio::io::split(client);
        let (server_reader, server_writer) = tokio::io::split(server);

        let client_writer = Arc::new(Mutex::new(client_writer));
        let server_writer = Arc::new(Mutex::new(server_writer));
        let server_buff: Arc<Mutex<Vec<u8>>> = Arc::new(Mutex::new(Vec::new()));
        let client_buff: Arc<Mutex<Vec<u8>>> = Arc::new(Mutex::new(Vec::new()));

        tokio::spawn(handler(
            server_reader,
            client_writer,
            server_buff,
            &Handler::Outgoing,
        ));

        tokio::spawn(handler(
            client_reader,
            server_writer,
            client_buff,
            &Handler::Incoming,
        ));
    }
}
