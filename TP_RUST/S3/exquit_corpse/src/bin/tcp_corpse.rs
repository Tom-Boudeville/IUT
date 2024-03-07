use exquit_corpse::make_response;
use tokio::{net::TcpListener, io::{AsyncReadExt, AsyncWriteExt}};
use clap::Parser;
use std::sync::{Arc, Mutex};

#[derive(Parser)]
struct Options {
    host: String,
    port: u16,
}

const BUFFER_SIZE: usize = 2048;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let options = Options::parse();
    let text = Arc::new(Mutex::new(String::from("")));

    let serveur = TcpListener::bind(format!("{}:{}", options.host, options.port)).await?;

    loop {
        let (mut stream, _) = serveur.accept().await?;
        let text_clone = Arc::clone(&text);

        tokio::spawn(async move {
            let (mut reader, mut writer) = stream.split();
            let mut buf = [0; BUFFER_SIZE];
            let size = reader.read(&mut buf).await?;
            let received_message = std::str::from_utf8(&buf[..size])?;

            // Utilisation de lock pour obtenir le MutexGuard
            let text_guard = text_clone.lock().unwrap();
            let response = make_response(received_message, &text_guard);

            // Écriture de la réponse dans le writer
            writer.write_all(response.as_bytes()).await?;
            writer.flush().await?;

            Ok::<(), anyhow::Error>(())
        });
    }
}
