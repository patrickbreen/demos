
mod bencoding;
mod torrent;
mod client;


use torrent::Torrent;
use client::TorrentClient;


use bencoding::{Decoder, Encoder};



// this file just takes command line arguments, sets up the client, and runs it inside of an async
// event loop

fn main() {
    println!("Hello, world!");
 }
