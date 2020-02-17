// tracker
//
//
// This module defines the tracker
//
// structs:
// Tracker
// TrackerResponse
//

extern crate rand;
extern crate url;
extern crate reqwest;

use std::str;
use std::collections::BTreeMap;

use rand::Rng;
use url::form_urlencoded;
use reqwest::blocking;

use crate::bencoding::{Decoded, Decoder};
use crate::torrent::Torrent;


pub struct TrackerResponse {
    response: BTreeMap<Vec<u8>, Decoded>,
}

impl TrackerResponse {
    pub fn new(response: BTreeMap<Vec<u8>, Decoded>) -> TrackerResponse {
        TrackerResponse {response: response}
    }
    pub fn failure(&self) -> Option<Vec<u8>> {
        if self.response.contains_key(&b"failure_response".to_vec()) {
            return Some(self.response.get(&b"failure_response".to_vec()).unwrap().bytes.as_ref().unwrap().to_vec());
        }
        None
    }
    fn get_or_0(&self, key: &[u8]) -> u64 {
        let s = self.response.get(key);
        if s.is_none() {
            return 0;
        }
        s.unwrap().int.unwrap() as u64
    }
    pub fn interval(&self) -> u64 {
        self.get_or_0(b"interval")
    }
    pub fn complete(&self) -> u64 {
        self.get_or_0(b"complete")
    }
    pub fn incomplete(&self) -> u64 {
        self.get_or_0(b"incomplete")
    }
    pub fn peers(&self) -> Vec<(String, u16)> {
        let peers = self.response.get(&b"peers".to_vec()).unwrap();

        if peers.type_name == "list" {
            panic!("No implementation for multifile.");
        }

        let raw_peers: Vec<u8> = peers.bytes.as_ref().unwrap().to_vec();
        let formatted_peers: Vec<(String, u16)> = TrackerResponse::format_peers(raw_peers);
        formatted_peers
    }

    pub fn to_string(&self) -> String {
        // TODO
        format!("incomplete: TODO")
    }

    fn format_peers(raw_peers: Vec<u8>) -> Vec<(String, u16)> {
        let mut ret = Vec::new();
        let mut i = 0;
        while i + 8 < raw_peers.len() {
            let ip_bytes = &raw_peers[i..i+4];
            let mut str_bytes: Vec<String> = Vec::new();
            for byte in ip_bytes {
                str_bytes.push(str::from_utf8(&[*byte]).unwrap().to_string());
            }
            let ip = str_bytes.connect(":");


            let port_bytes = &raw_peers[i+4..i+6];
            let port : u16 = port_bytes[0] as u16 * port_bytes[1] as u16;


            ret.push((ip, port));
            i += 6;
        }

        ret
    }
}

pub struct Tracker {
    torrent: Torrent,
    peer_id: Vec<u8>,
}

impl Tracker {
    pub fn new(torrent: Torrent) -> Tracker {
        Tracker {
            torrent: torrent,
            peer_id: Tracker::calculate_peer_id(),
        }
    }
    pub fn connect(&self, uploaded: usize, downloaded: usize, first: bool) -> TrackerResponse {
        // its called connect, but it's actually stateless
        
        let encoded_params: String = form_urlencoded::Serializer::new(String::new())
            .append_pair("info_hash", str::from_utf8(&self.torrent.info_hash).unwrap())
            .append_pair("peer_id", str::from_utf8(&self.peer_id).unwrap())
            .append_pair("port", "6889")
            .append_pair("uploaded", &uploaded.to_string())
            .append_pair("downloaded", &downloaded.to_string())
            .append_pair("left", &(self.torrent.total_size() - downloaded).to_string())
            .append_pair("compact", "1")
            .finish();
        let encoded_event: String = form_urlencoded::Serializer::new(String::new())
            .append_pair("event", "started")
            .finish();

        let mut url = str::from_utf8(&self.torrent.announce()).unwrap().to_string() + "?" + &encoded_params;

        if first {
            url += "&";
            url += &encoded_event;
        }

        let mut resp = reqwest::blocking::get(&url).unwrap();

        if resp.status() != 200 {
            panic!("response status: {}", resp.status());
        }

        // get response data bytes
        let mut bytes: Vec<u8> = Vec::new();
        resp.copy_to(&mut bytes);
        self.raise_for_error(&bytes);

        let decoded_dict: BTreeMap<Vec<u8>, Decoded> = Decoder::new(bytes).decode().unwrap().dict.unwrap();
        TrackerResponse::new(decoded_dict)
    }

    fn raise_for_error(&self, data: &[u8]) {
        for i in 0usize..data.len()-7 {
            if b"failure" == &data[i..i+7] {
                panic!("error");
            }
        }
    }
 
    fn calculate_peer_id() -> Vec<u8> {
        let mut vec = b"-PC0001-".to_vec();
        let mut rng = rand::thread_rng();

        for i in 0..12 {
            let n = rng.gen_range(48, 58);
            vec.push(n);
        }
        vec
    }
}


#[cfg(test)]
mod tests {
    // import parent scope
    use super::*;

    fn test_response() {
        let response = TrackerResponse::new(BTreeMap::new());
    }

    fn test_tracker() {
        // smoke test
        let torrent = Torrent::new(b"data/ubuntu-18.04.3-desktop-amd64.iso.torrent".to_vec());
        let tracker = Tracker::new(torrent);
    }
}
