#[macro_use]
extern crate structure;
extern crate crypto;

mod bencoding;
mod torrent;
mod client;
mod protocol;
mod tracker;


use torrent::Torrent;
use client::TorrentClient;


use bencoding::{Decoder, Encoder};



// this file just takes command line arguments, sets up the client, and runs it inside of an async
// event loop

fn main() {
    println!("Hello, world!");
 }
