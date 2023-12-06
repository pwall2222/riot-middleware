use std::sync::Arc;

use tokio::sync::Mutex;

use tokio::io::{AsyncReadExt, AsyncWriteExt};

use super::{check_xml, Handler, Reader, Writer, WriterArc, MODIFY};

use crate::data::data_server;

#[allow(dead_code)]
fn print(buff: &[u8], color: &str) {
    println!("\x1b[{}m{}\x1b[0m", color, unsafe {
        String::from_utf8_unchecked(buff.into())
    });
}

pub async fn handler(
    mut reader: Reader,
    writer: WriterArc,
    buff: Arc<Mutex<Vec<u8>>>,
    handler_type: &Handler,
) {
    loop {
        let writer = writer.clone();
        let buff = buff.clone();
        let mut data = [0; 1024 * 64];
        let n = reader
            .read(&mut data)
            .await
            .expect("failed to read data from socket");

        if n == 0 {
            dbg!("Return");
            return;
        }
        let data = data[0..n].to_vec();
        data_handler(&data, buff, writer, handler_type).await;
    }
}

async fn data_handler(
    data: &Vec<u8>,
    buff: Arc<Mutex<Vec<u8>>>,
    writer: WriterArc,
    handler_type: &Handler,
) {
    let mut buff = buff.lock().await;
    let writer = writer.lock().await;
    if buff.len() == 0 && check_xml(data) {
        write(writer, data, handler_type).await;
        print(data, "95");
        return;
    }
    buff.extend(data);
    println!("Multi buff");
    print(&buff, "94");

    if !check_xml(&buff) {
        return;
    }
    write(writer, &buff, handler_type).await;

    buff.clear();
}

async fn write(
    mut writer: tokio::sync::MutexGuard<'_, Writer>,
    data: &Vec<u8>,
    handler_type: &Handler,
) {
    let data = MODIFY
        .read()
        .await
        .clone()
        .run_request(data, handler_type)
        .await
        .unwrap_or_else(|_| data.to_vec());

    let data = data.to_vec();
    writer.write_all(data.as_slice()).await.unwrap();
    let handler_type = handler_type.clone();
    tokio::spawn(async move { data_server(&data, handler_type).await });
}
