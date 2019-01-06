extern crate chrono;

use std::io::{Write, BufReader, BufRead};
use std::thread;
use std::net::{Shutdown, TcpStream, TcpListener, SocketAddr};
use std::vec::Vec;

use std::fs::File;
use std::io::prelude::*;

use chrono::prelude::*;


// TODO: make a list/map of URL_match -> req_handler


#[derive(Debug, Default)]
struct Request {
    verb: String,
    path: String,
    version: String,

    headers: Vec<String>,
    vars: Vec<String>,
}

impl Request {
    fn parse(bytes: Vec<u8>) -> Request {
        Request{..Default::default()}
    }

    fn to_bytes(&self) -> Vec<u8> {
        Vec::new()
    }
}


#[derive(Debug, Default)]
struct Response {
    code: String,
    headers: Vec<String>,

    body: String,
}

impl Response {
    fn parse(bytes: Vec<u8>) -> Response {
        Response{..Default::default()}
    }

    fn to_bytes(&self) -> Vec<u8> {
        Vec::new()
    }
}


// example write a simple string as a response
pub fn http_simple(addr: SocketAddr) {
    let listener = TcpListener::bind(addr).unwrap();
    println!("server started. listening on {}", addr);
    for stream in listener.incoming() {
        thread::spawn(|| {
            let mut stream = stream.unwrap();
            stream.write(b"Hello World\r\n").unwrap();
        });
    }
}

fn format_header(resp_type: String, resp_code: String) -> String {
    let dt = Local::now();
    let date = dt.format("%a, %d %b %Y %H:%M:%S %Z").to_string();
    let head = "HTTP/1.1 ".to_owned() + &resp_code + "\n" +
    "Date: " + &date + "\n" + 
    "Connection: close\n" + 
    "Server: Custom (rust)\n" +
    "Content-Type: " + &resp_type + "; charset=iso-8859-1\n\n";

    head
}

fn read_file(path: String) -> Vec<u8> {
    let file = File::open("files/".to_owned() + &path[1..]);
    match file {
        Ok(mut f) => {

            let mut contents = String::new();
            f.read_to_string(&mut contents).expect("couldn't read file");
            let ext = path.split(".").last().unwrap();

            let head = format_header("text/".to_string() + &ext.to_string(), "200 Ok".to_string());
            let ret = head.to_owned() + &contents;
            ret.into_bytes()
            
        },
        _ => {
            let head = format_header("text/*".to_string(), "404 Not Found".to_string());
            head.into_bytes()
        }
    }
}

fn rev_proxy_request(req: Vec<String>, addr: String) -> Vec<u8> {

    // so we could do this with some sort of HTTP library for correctness and completeness sake...
    // and maybe that's how I would do this in production, but this is for learning so I'm going to
    // use another raw TCP socket. Try to stop me.

    let mut new_req = req.clone();

    // put the new addr as the Host in the request header
    for i in 0..new_req.len() {
        let field = new_req[i].clone();
        if field.starts_with("Host") {
            new_req[i] = "Host: ".to_owned() + &addr;
        }
    }

    // make request
    let request = build_req(new_req);
    println!("about to send proxied request");
    let mut stream = TcpStream::connect(addr).unwrap();
    println!("connection made");

    // sent it
    stream.write(&request).unwrap();

    // read response
    let mut reader: BufReader<TcpStream> = BufReader::new(stream.try_clone().unwrap());

    let mut vec = Vec::new();
    let mut line = String::new();

    while !(line.as_str() == "\r\n") {
        line.clear();
        reader.read_line(&mut line).unwrap();
        vec.push(line.clone().to_string());
    }
    vec.push("\r\n".to_string());

    let response = vec.join("");

    stream.shutdown(Shutdown::Both).expect("shutdown call failed");

    println!("response success: {:?}", response);
    response.into_bytes()


}

// fn example_web_application_interface(req: Vec<String>) -> Vec<u8> {

//     handle request pseudocode:

//     if req[0] == "GET" {
//         id = some_pattern_matching(req[1]).unrap();
//         data = database.get(req[1:]).unwrap();
//     }
//     let head = format_header("application/json", "200 Ok")
//     let ret = head.to_owned() + &contents;
//     ret.into_bytes()
// }

fn handle(req: Vec<String>) -> Vec<u8> {

    let path = &req[1];

    // if path starts with /forward/ then proxy the request to another server
    let match_prefix = "/forward/";
    if path.starts_with(match_prefix) {
        let mut new_req = req.clone();
        // take "/forward" off of the front of the path
        new_req[1] = path.as_str().chars().skip(match_prefix.len()-1).collect();
        return rev_proxy_request(new_req, "172.217.9.132:80".to_string());
    }


    match path.as_str() {
        "" => read_file("/index.html".to_string()),
        "/" => read_file("/index.html".to_string()),
        _ => read_file(path.to_string())
    }

}


// for now the request is a vector of strings
// first string is verb, second is URL path, third is version
// and the rest are headers
fn parse_req(mut reader: BufReader<TcpStream>) -> Vec<String> {
    let mut vec = Vec::new();
    let mut line = String::new();

    reader.read_line(&mut line).unwrap();
    let first_line = line.clone();
    let fields: Vec<&str> = first_line.split_whitespace().collect();

    for field in fields {
        vec.push(field.trim().to_string());
    }

    while !(line.as_str() == "\r\n") {
        line.clear();
        reader.read_line(&mut line).unwrap();
        vec.push(line.clone().trim().to_string());
    }

    vec
}

fn build_req(req: Vec<String>) -> Vec<u8> {
    let mut resp = req[0].clone() +  " " + &req[1] + " " + &req[2] + "\n";

    for line in req[3..].iter() {
        resp = resp + line + "\n";
    }

    (resp + "\n").into_bytes()
}

fn http_server(addr: SocketAddr) {
    let listener = TcpListener::bind(addr).unwrap();
    println!("server started. listening on {}", addr);

    for stream in listener.incoming() {
        thread::spawn(move|| {
            let mut stream = stream.unwrap();
            println!("connection from {}", stream.peer_addr().unwrap().to_string());

            println!("starting to read from the stream.");
            let reader: BufReader<TcpStream> = BufReader::new(stream.try_clone().unwrap());
            let req = parse_req(reader);
            println!("done parsing");
            let resp = handle(req);
            println!("sending response");
            stream.write(&resp).unwrap();
            stream.shutdown(Shutdown::Both).expect("shutdown call failed");
        });
    }
}