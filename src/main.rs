//! Simple Rust TCP server
//! Dwight J. Browne
//! Will either accept arguments from the command line or ENV vars
//! -p  port #
//! SERVER_PORT
//!


use std::env;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::str;
use std::thread;
use chrono::Utc;
use structopt::StructOpt;

fn handle_client(mut stream: TcpStream) {
    const K_BUF_SIZE: usize = 1024;
    let mut data = [0 as u8; K_BUF_SIZE];
    while match stream.read(&mut data) {
        Ok(size) => {
            if size > 0 {
                println!("{}: RECEIVED: {}", Utc::now(), str::from_utf8(&data[..size]).unwrap());

                // dump the data
                match stream.write(&data[0..size]) {
                    Ok(n) => {
                        println!("{}: wrote {} bytes", Utc::now(), n);
                    }
                    Err(e) => {
                        println!("{}: can't write data!  {}", Utc::now(), e);
                    }
                }
                match stream.flush() {
                    Ok(_) => {}
                    Err(e) => {
                        println!("{}: can't flush {}", Utc::now(), e);
                    }
                }

                println!("{}: sent data! {} bytes", Utc::now(), size);
            }
            true
        }
        Err(_) => {
            println!("Error. terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn main() {
    const K_PORT: &str = "3333";
    // check for command line args
    #[derive(StructOpt, Debug)]
    #[structopt(rename_all = "kebab-case")]
    struct Opt {
        #[structopt(default_value = K_PORT, short)]
        port: i32,
    }

    let opt = Opt::from_args();
    let _port: i32;
    // check for environment variables
    let env_port = env::var("SERVER_PORT").unwrap_or(K_PORT.to_string());
    // see if we have any input args
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        println!("Have input args: Overriding default  vars with the following:");
        println!("{:#?}", &opt);
        _port = opt.port;
    } else {
        println!("Using environment vars: with the following values");
        println!("port: SERVER_PORT    {}",env_port);
        _port = env_port.parse::<i32>().unwrap();
    }

    let bind_string = format!("0.0.0.0:{}", &_port);
    let listener = TcpListener::bind(&bind_string).unwrap();
    // new thread for each new connection
    println!("{}: Server is listening {}", Utc::now(), &bind_string);
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("\n{}: New connection: {}", Utc::now(), stream.peer_addr().unwrap());
                thread::spawn(move || {
                    // connection succeeded
                    handle_client(stream)
                });
            }
            Err(e) => {
                println!("Error: {}", e);
                // connection failed
            }
        }
    }
    // close the socket server
    drop(listener);
}
