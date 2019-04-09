
extern crate chrono;

use std::io::{self, Write, BufReader, BufRead};
use std::thread;
use std::net::{TcpStream, TcpListener, ToSocketAddrs};

mod http;
use http::http_simple;


// netcat client
fn netcat_client(addr: std::net::SocketAddr) {
    let mut stream = TcpStream::connect(addr).expect("could not connect to HOST");
    println!("Connection started....");


    let str_buf = String::new();
    let stdin = io::stdin();
    let mut stdin_reader = BufReader::new(stdin);
    let mut line = String::new();
    let mut reader: BufReader<TcpStream> = BufReader::new(stream.try_clone().unwrap());


    loop {
        line.clear();
        stdin_reader.read_line(&mut line).unwrap();

        // handle each line on stdin
        if !(line.as_str() == "") {
            stream.write(&line.clone().into_bytes());
            stream.write(b"\n");
            println!("wrote line");
        }

        // print out each line from the socket
        line.clear();
        reader.read_line(&mut line).unwrap();

        while !(line.as_str() == "") {
            line.clear();
            reader.read_line(&mut line).unwrap();
            println!("{:?}", line);
        }
    }
}

fn echo_server(addr: std::net::SocketAddr) {
    let listener = TcpListener::bind(addr).unwrap();
    println!("server started. listening on {}", addr.to_string());

    for stream in listener.incoming() {
        thread::spawn(move|| {
            let stream = stream.unwrap();
            println!("connection from {}", stream.peer_addr().unwrap().to_string());

            let mut line = String::new();
            
            let mut reader: BufReader<TcpStream> = BufReader::new(stream.try_clone().unwrap());
            loop {
                line.clear();
                reader.read_line(&mut line).unwrap();
                println!("{:?}", line);
            }
        });
    }
}

fn main() {
    let host = "localhost:9000";
    let mut addrs_iter = host.to_socket_addrs().unwrap();
    let addr = addrs_iter.next().unwrap();

    let time_out = 3;


    // comment in only one of the lines bellow to start one of the servers

    // --- HTTP servers ---
    http_simple(addr);
}