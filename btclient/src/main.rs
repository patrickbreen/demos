#[macro_use]
extern crate structure;
extern crate crypto;

mod bencoding;
mod torrent;
mod client;
mod protocol;
mod tracker;

use crypto::sha1::Sha1;
use crypto::digest::Digest;
use bencoding::{Decoder, Encoder};

use tracker::Tracker;
use torrent::Torrent;
use client::TorrentClient;





// this file just takes command line arguments, sets up the client, and runs it inside of an async
// event loop

fn main() {
    println!("Hello, world!");

    // start with getting a tracker:
    let torrent = Torrent::new(b"data/ubuntu-16.04.6-desktop-amd64.iso.torrent".to_vec());
    let tracker = Tracker::new(torrent);
    let tracker_response = tracker.connect(0, 0, false);
    let peers = tracker_response.peers();
    println!("peers: {:?}", peers);

 }
