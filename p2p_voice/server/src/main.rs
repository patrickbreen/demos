use std::io::{self, Write, BufReader, BufRead};
use std::thread;
use std::sync::mpsc;
use std::net::{TcpStream, TcpListener, ToSocketAddrs};

use std::time::{Duration, Instant};

use std::collections::HashMap;

use serde_json::{Result, Value};

fn get_message(reader: &mut BufReader<TcpStream>) -> String {
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    line
}

#[derive(Debug)]
struct State {
    last_hello: Instant,
    id_call_avaliability_req: String,
    ip_addr: String,
}


fn main() {
    let host = "localhost:9000";
    let mut addrs_iter = host.to_socket_addrs().unwrap();
    let addr = addrs_iter.next().unwrap();

    let listener = TcpListener::bind(addr).unwrap();
    println!("server started. listening on {}", addr.to_string());

    let mut state_map: HashMap<String, State> = HashMap::new();
    let mut streams: Vec<TcpStream> = Vec::new();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let addr = stream.peer_addr().unwrap();
        let mut reader: BufReader<TcpStream> = BufReader::new(stream.try_clone().unwrap());
        println!("connection from {}", addr.to_string());
        println!("ttl: {:?}", listener.ttl());

        let message = get_message(&mut reader);

        let json_value: Value = serde_json::from_str(&message).unwrap();
        let connection_id = json_value["id"].as_str().unwrap().to_string();
        // let id_call_request = json_value["id_call_request"].as_str().unwrap().to_string();

        let state = State {
            last_hello: Instant::now(),
            id_call_avaliability_req: "".to_string(),
            ip_addr: addr.to_string(),
        };


        // TODO: test if there is an avaliability request
        // and if so, return avaliability response

        // also return a "call punch" request back to the client if nessisary

        state_map.insert(connection_id, state);
        println!("state: {:?}", state_map);
    }

    // TODO: simultaniuously listen over UDP for incoming call requests (from both clients)
    //  and respond to each client with the appropriate address
}