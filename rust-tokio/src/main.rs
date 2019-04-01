extern crate tokio;

use std::net::Shutdown;
use tokio::io;
use tokio::net::TcpListener;
use tokio::prelude::*;

fn main() {
    let addr = "127.0.0.1:10086".parse().unwrap();
    let listener = TcpListener::bind(&addr).expect("unable to bind TCP listener");

    let server = listener
        .incoming()
        .map_err(|e| eprintln!("accept failed = {:?}", e))
        .for_each(|sock| {
            let buffer = io::read(sock, vec![0; 1024])
                .and_then(|(sock, _, _)| {
                    io::write_all(
                        sock,
                        "HTTP/1.1 204 No Content\r\nServer: rust-tokio\r\n\r\n",
                    )
                })
                .and_then(|(sock, _)| sock.shutdown(Shutdown::Both))
                .and_then(|_| Ok(()))
                .map_err(|e| {
                    if e.kind() != std::io::ErrorKind::NotConnected {
                        eprintln!("error in read = {:?}", e)
                    }
                });

            tokio::spawn(buffer);

            Ok(())
        });

    tokio::run(server);
}
