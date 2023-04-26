use borsh::BorshDeserialize;
use lazy_static::lazy_static;
use std::net::SocketAddr;
use tokio::{
    io::AsyncReadExt,
    net::{TcpListener, TcpStream},
};
use xor_mailer::{Mailer, MailerConfig};
use xor_mailer_common::Envelope;

lazy_static! {
    static ref MAILER_CONFIG: MailerConfig = MailerConfig::init();
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6363").await.unwrap();

    println!("Running on socket 127.0.0.1:6363");

    while let Ok((stream, _socket_addr)) = listener.accept().await {
        tokio::spawn(async move {
            match handle_client(stream).await {
                Ok(addr) => {
                    println!("DISCONNECTED device[{}:{}]", addr.ip(), addr.port())
                }
                Err(error) => {
                    eprintln!("{:?}", error);
                }
            }
        })
        .await
        .unwrap();
    }
}

async fn handle_client(mut stream: TcpStream) -> anyhow::Result<SocketAddr> {
    println!("↓[CONNECTED] device[{}]", stream.peer_addr()?);

    let mut buffer = [0; 4096];
    let mut stream_data: Vec<u8> = Vec::new();
    let mut bytes_read: usize;

    loop {
        bytes_read = stream.read(&mut buffer).await?;

        if bytes_read == 0 {
            let peer = stream.peer_addr()?;
            return Ok(peer);
        }

        // Check if the current stream is less than the buffer capacity, if so all data has been received
        if buffer[..bytes_read].len() < 4096 {
            stream_data.append(&mut buffer[..bytes_read].to_owned());

            break;
        }
        // Append data to buffer
        stream_data.append(&mut buffer[..bytes_read].to_owned());
    }

    let envelope = Envelope::try_from_slice(&stream_data)?;

    Mailer::new(&*MAILER_CONFIG)
        .add_envelope(envelope)
        .send()
        .await
        .unwrap();

    Ok(stream.peer_addr()?)
}
