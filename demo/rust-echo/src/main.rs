#[macro_use]
extern crate log;
extern crate log4rs;

extern crate master;

use std::io::{Error, Read, Write};
use std::net::TcpStream;

fn handle_echo(mut stream: TcpStream) -> Result<(), Error> {
    info!("incoming from {}", stream.peer_addr()?);
    let mut buf = [0; 512];
    loop {
        let bytes_read = stream.read(&mut buf)?;
        if bytes_read == 0 {
            return Ok(());
        }
        stream.write(&buf[..bytes_read])?;
    }
}

fn handle_conn(stream: TcpStream) {
    let r = handle_echo(stream);
    match r {
        Ok(_v) => {}
        Err(_e) => {}
    }
}

fn main() {
    //    let addrs = "127.0.0.1:8188, 127.0.0.1:8288; 127.0.0.1:8388;".to_string();
    //    println!("start listen {}", addrs);
    //    master::tcp_service::start_alone(&addrs, handle_conn);

    //    master::tcp_service::start_daemon(handle_conn);
    master::tcp_service::start(handle_conn);
}
