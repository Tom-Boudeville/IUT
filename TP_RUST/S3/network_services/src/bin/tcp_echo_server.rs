extern crate structopt;

use tokio::{net::TcpListener, io::{AsyncReadExt, AsyncWriteExt}};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Options {
    host: String,
    port: u16,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let options = Options::from_args();
    let server = TcpListener::bind(format!("{}:{}", options.host, options.port)).await?;

    println!("Server listening on {}:{}", options.host, options.port);

    loop {
        let (mut stream, _) = server.accept().await?;

        tokio::spawn(async move {
            let (mut reader, mut writer) = stream.split();
            let mut buf: [u8; 1024] = [0; 1024];

            let bytes_reader = reader.read(&mut buf).await?;
            let received_message = std::str::from_utf8(&buf[..bytes_reader])?;

            writer.write_all(received_message.as_bytes()).await?;
            writer.flush().await?;

            Ok::<(), anyhow::Error>(())
        });
    }
}
