#![deny(warnings)]
#![allow(unused_imports)]
#![allow(unused_variables)]

extern crate tokio;
extern crate tokio_codec;
extern crate byteorder;

use tokio::codec::Decoder;
use tokio::prelude::*;
use tokio_codec::BytesCodec;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{copy, shutdown};

use std::env;
use std::sync::{Arc, Mutex};
use std::io::{self, Read, Write};
use std::net::{Shutdown, SocketAddr};
use byteorder::{BigEndian, ReadBytesExt, ByteOrder}; // 1.2.7

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
        .for_each(move |client| {
            println!("Received a connection");

            let bytes = vec![0; 12];
            let processor = tokio::io::read_exact(client, bytes)
                .and_then(move |(socket, bytes)| {
                    println!("\nbytes: {:?}", bytes);
                    println!("Request Size {}", BigEndian::read_i32(&bytes));
                    println!("Request api_key {}", BigEndian::read_i16(&bytes[4..]));
                    println!("Request api_version {}", BigEndian::read_i16(&bytes[6..]));
                    println!("Request correlation_id {}", BigEndian::read_i16(&bytes[8..]));

                    Ok(())
                })
                .map_err(|_| ());
            tokio::spawn(processor);

            Ok(())
        });

    tokio::run(done);
    Ok(())
}