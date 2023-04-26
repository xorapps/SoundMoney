use async_std::{
    io::ReadExt,
    net::{SocketAddr, TcpListener, TcpStream},
    stream::StreamExt,
    sync::{Arc, RwLock},
    task,
};
use borsh::BorshDeserialize;
use lazy_static::lazy_static;
use std::collections::HashMap;
use tide::{
    http::{headers::CONTENT_TYPE, mime},
    Body, Response, StatusCode,
};

lazy_static! {
    static ref CDN_MEMDB: Arc<RwLock<CdnMemDB>> = Arc::new(RwLock::new(CdnMemDB::init()));
}

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    femme::start();

    spawn_listener().await.unwrap();

    http_server().await.unwrap();

    Ok(())
}

pub type TxSignature = blake3::Hash;
pub type HtmlResponseString = String;

pub async fn http_server() -> Result<(), std::io::Error> {
    let mut app = tide::new();
    app.with(tide::log::LogMiddleware::new());
    app.at("/").get(|_| async { Ok("Validator Service CDN") });
    app.at("/tx/:signature").get(handler);
    app.at("/cdn/").serve_dir("resources/")?;

    app.listen("127.0.0.1:6364").await?;

    Ok(())
}

pub async fn spawn_listener() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:6365").await?;

    task::spawn(async move {
        let mut incoming = listener.incoming();

        while let Some(stream) = incoming.next().await {
            let stream = stream.unwrap();

            task::spawn(async {
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
        }
    });

    Ok(())
}

#[derive(Debug, Default)]
pub struct CdnMemDB {
    db: HashMap<TxSignature, HtmlResponseString>,
}

impl CdnMemDB {
    pub fn init() -> Self {
        CdnMemDB::default()
    }

    pub fn set(&mut self, tx: &str, response: &str) -> &mut Self {
        self.db
            .insert(blake3::hash(tx.as_bytes()), response.to_owned());

        self
    }

    pub fn get(&self, tx: &str) -> Option<&HtmlResponseString> {
        self.db.get(&blake3::hash(tx.as_bytes()))
    }
}

async fn handler(req: tide::Request<()>) -> tide::Result {
    let mut segments = req.url().path_segments().unwrap();
    segments.next();
    let tx = segments.next().unwrap();

    if let Some(response_data) = CDN_MEMDB.read().await.db.get(&blake3::hash(tx.as_bytes())) {
        let mut res = Response::new(StatusCode::Ok);
        res.set_body(Body::from_string(response_data.to_owned()));
        res.set_content_type(mime::HTML);
        Ok(res)
    } else {
        Ok("Transaction Signature Not Found".into())
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

    let (tx, html_response) = <(String, HtmlResponseString)>::try_from_slice(&stream_data)?;

    dbg!(&tx);

    CDN_MEMDB.write().await.set(&tx, &html_response);

    for key in CDN_MEMDB.read().await.db.keys() {
        println!("TX: {}", key);
    }

    Ok(stream.peer_addr()?)
}
