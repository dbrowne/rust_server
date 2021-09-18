use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::str;
use chrono::Utc;

fn handle_client(mut stream: TcpStream) {
    const K_BUF_SIZE: usize = 1024;
    let mut data = [0 as u8;K_BUF_SIZE];
    while match stream.read(&mut data) {
        Ok(size) => {
            if(size>0) {
                println!("{}: {}",Utc::now(), str::from_utf8(&data[..size]).unwrap());

                // dump the data
                match stream.write(&data[0..size]){
                    Ok(n) => {
                        println!("{}: wrote {} bytes",Utc::now(), n);
                    },
                    Err(e) =>{
                        println!("{}: can't write data!  {}",Utc::now(), e);
                    }
                }
                match stream.flush(){
                    Ok(_) =>{},
                    Err(e) =>{
                        println!("{}: can't flush {}",Utc::now(),e);
                    }
                }

                println!("{}: sent data! {} bytes",Utc::now(), size);
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
    println!("{}: Server is listening on port 3333",Utc::now());
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("{}: New connection: {}",Utc::now(), stream.peer_addr().unwrap());
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
