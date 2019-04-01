#![deny(warnings)]
#![allow(unused_imports)]

extern crate tokio;
extern crate tokio_codec;

use tokio::codec::Decoder;
use tokio::prelude::*;
use tokio_codec::BytesCodec;
use tokio::net::{TcpListener, TcpStream};

use std::env;
use std::sync::{Arc, Mutex};
use std::io::{self, Read, Write};
use std::net::{Shutdown, SocketAddr};
use tokio::io::{copy, shutdown};

fn main() -> Result<(), Box<std::error::Error>> {
    let listen_addr = env::args().nth(1).unwrap_or("127.0.0.1:8888".to_string());
    let listen_addr = listen_addr.parse::<SocketAddr>()?;
    let listen_socket = TcpListener::bind(&listen_addr)?;
    println!("Listening on: {}", listen_addr);

    let server_addr = env::args().nth(2).unwrap_or("127.0.0.1:9999".to_string());
    let server_addr = server_addr.parse::<SocketAddr>()?;
    let _server_socket = TcpListener::bind(&server_addr)?;
    println!("Proxying to: {}", server_addr);

    let done = listen_socket
        .incoming()
        .map_err(|e| println!("failed to accept server socket; error = {:?}", e))
        .for_each(move |_client| {

            let server = TcpStream::connect(&server_addr);
            let _amounts = server.and_then(move |_server| {
                Ok(())
            });

            Ok(())
        });

    tokio::run(done);
    Ok(())
}

#[derive(Clone)]
struct MyTcpStream(Arc<Mutex<TcpStream>>);

impl Read for MyTcpStream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.lock().unwrap().read(buf)
    }
}

impl Write for MyTcpStream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.lock().unwrap().write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl AsyncRead for MyTcpStream {}

impl AsyncWrite for MyTcpStream {
    fn shutdown(&mut self) -> Poll<(), io::Error> {
        try!(self.0.lock().unwrap().shutdown(Shutdown::Write));
        Ok(().into())
    }
}
