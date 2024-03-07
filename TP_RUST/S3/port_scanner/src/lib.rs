use std::net::ToSocketAddrs;
use std::time::Duration;

pub fn is_open_sync(host: &str, port: u16, timeout: u64) -> bool {
    let mut addrs_iter = format!("{}:{}", host, port).to_socket_addrs().unwrap();
    std::net::TcpStream::connect_timeout(&addrs_iter.next().unwrap(), Duration::from_secs(timeout)).is_ok()
}

#[derive(Debug, StructOpt)]
pub struct Options {
    #[structopt(short = "H", long = "host")]
    pub host: String,
    #[structopt(short = "p", long = "port-min")]
    pub port_min: u16,
    #[structopt(short = "P", long = "port-max")]
    pub port_max: u16,
    #[structopt(short = "t", long = "timeout")]
    pub timeout: u64,
}
