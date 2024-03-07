use std::net::UdpSocket;
use clap::Parser;
use exquit_corpse::make_response;

#[derive(Parser)]
 struct Options{
    host: String,
    port: u16 ,
}


    #[tokio::main]
    async fn main() -> anyhow::Result<()> {
        let options = Options::parse();
        let listener = UdpSocket::bind(format!("{}:{}",options.host,options.port))?;
        let mut text = String::from("");
        let mut buf = vec![0u8; 8888];
        loop {
            let (size,addr) = listener.recv_from(&mut buf)?;
            
            let received_message = std::str::from_utf8(&buf [..size])?;
            println!("Received '{}' from '{}",received_message,addr);
            listener.send_to(make_response(received_message,&mut text).as_bytes(),&addr)?;
        }
    
    }