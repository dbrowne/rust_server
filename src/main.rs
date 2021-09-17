use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::str;

fn handle_client(mut stream: TcpStream) {
    const K_BUF_SIZE: usize = 1024;
    let mut data = [0 as u8;K_BUF_SIZE];
    while match stream.read(&mut data) {
        Ok(size) => {
            if(size>0) {
                println!("{}", str::from_utf8(&data[..size]).unwrap());

                // dump the data
                stream.write_all(&data[0..size]).unwrap();
                stream.flush();

                println!("sent data! {} bytes", size);
            }
            true
        },
        Err(_) => {
            println!("Error. terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    }{}
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    // new thread for each new connection
    println!("Server is listening on port 3333");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move || {
                    // connection succeeded
                    handle_client(stream)
                });
            }
            Err(e)=> {
                println!("Error: {}", e);
                // connection failed
            }
        }
    }
    // close the socket server
    drop(listener);
}
