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
        if self.response.contains_key(&b"failure reason".to_vec()) {
            return Some(self.response.get(&b"failure reason".to_vec()).unwrap().bytes.as_ref().unwrap().to_vec());
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
        while i + 6 <= raw_peers.len() {
            let ip_bytes = &raw_peers[i..i+4];
            let mut str_bytes: Vec<String> = Vec::new();
            for byte in ip_bytes {
                str_bytes.push(byte.to_string());
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

    fn ok_response() -> TrackerResponse {
        let mut dict = BTreeMap::new();
        dict.insert(b"complete".to_vec(), Decoded{
            type_name: "int".to_string(),
            int: Some(5500),
            bytes: None,
            list: None,
            dict: None});
        dict.insert(b"incomplete".to_vec(), Decoded{
            type_name: "int".to_string(),
            int: Some(240),
            bytes: None,
            list: None,
            dict: None});
        dict.insert(b"interval".to_vec(), Decoded{
            type_name: "int".to_string(),
            int: Some(1800),
            bytes: None,
            list: None,
            dict: None});
        dict.insert(b"peers".to_vec(), Decoded{
            type_name: "bytes".to_string(),
            int: None,
            bytes: Some(b"V\x04\x18s\xc8\xdb\xb0\x1fc\xb2\xc7\x98V=G\x95\xa4\x1dP\x1e\xbf\xb0\xc8\xd5%\xbb\x01=\\\x1e\xb2\xdaf\x1b\x1a\xe1Oq\xee+\xc8\xd5S\xe3\x89M\xcc\xc7U\x0fXe\xc8\xd5\xbc\xe8\xd5\xde\xc8\xd5\xb9\x15\xd9J\xfa\x00\x05'Y\x1f\x1e\xc9\xc0\x00\xab*\x1a\xe1\xbcI\xa0P\x84\x99P\xdb\xe3\xd5\xcd-P\xb1\xa5\x93\x1a\xe1\xc0\x83,\t\xe7\x86I\n\xdae\xc8\xd5\xcc\xbbd\x13\x9c\xbf.)F\x17\x1a\xe1k\xbc\xea\xed\xbe\xa0\xb0O\x9b\xf3Z$P\xea,Q\xee;\xc0\x83,\\\xe5\x07e\xb8\x80\r\xed2\xb7\x0e\xa2N\xc8\xd5\x1f\x19\x1f\xda\xc8\xd5\x05'T\r\xc8\xd5[\xc4\xc2%\xc8\xd5^\x17%F\xc8\xd5^\x17\xdd\xd5\xaf\xd5\x83\xf7\x13t\xdcV\xc3.\xbbA\xc8\xd5\x051N\xd9\xc8\xd5OxV\x80\x1a\xecO\xf3\x97\xf9\x1a\xe1\xa3\xac\x84\xe6(\xcc\xd9\xe0L2\xffE\xd9X[\xc2\x1a\xe1\x02\x1d\x160\xc8\xd5PG\x81t\xe3\xc1\xce\xe1R\xa1\xb3\x9f\xc5\x94a\xa2\xc8\xd5H\xb9\xe3}\xe7\xd3\xc2\xe2\x9bK\xde\xa7_\xb6/\xe6\xd1*Pc1k\x1a\xeaG\xcf.(\xc8\xd5\xc3\x9a\xf0\x03\xb4\x06^\xf5.\x9dL\x18".to_vec()),
            list: None,
            dict: None});
        TrackerResponse::new(dict)
    }

    #[test]
    fn test_failed_response() {
        let mut dict = BTreeMap::new();
        dict.insert(b"failure reason".to_vec(), Decoded{
            type_name: "bytes".to_string(),
            int: None,
            bytes: Some(b"You failed!".to_vec()),
            list: None,
            dict: None});
        let response = TrackerResponse::new(dict);
        assert_eq!(b"You failed!".to_vec(), response.failure().unwrap());
    }

    #[test]
    fn test_successful_response_no_failure() {
        let response = ok_response();
        assert!(response.failure().is_none());
    }
    #[test]
    fn test_successful_response_complete() {
        let response = ok_response();
        assert_eq!(response.complete(), 5500);
    }
    #[test]
    fn test_successful_response_incomplete() {
        let response = ok_response();
        assert_eq!(response.incomplete(), 240);
    }
    #[test]
    fn test_successful_response_interval() {
        let response = ok_response();
        assert_eq!(response.interval(), 1800);
    }
    #[test]
    fn test_successful_response_peer_string() {
        let response = ok_response();
        assert_eq!(response.peers().len(), 50);
    }

    #[test]
    fn test_tracker() {
        // smoke test
        let torrent = Torrent::new(b"data/ubuntu-18.04.3-desktop-amd64.iso.torrent".to_vec());
        let tracker = Tracker::new(torrent);
    }
}
