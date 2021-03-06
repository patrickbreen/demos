// torrent

use std::str;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::collections::BTreeMap;

use crypto::sha1::Sha1;
use crypto::digest::Digest;

use crate::bencoding::{Decoded, Decoder, Encoder, encode_decoded};


#[derive(Debug, Clone)]
pub struct Torrent {
    pub filename: Vec<u8>,
    pub meta_info: BTreeMap<Vec<u8>, Decoded>,
    pub info_hash: String,
    pub name: Vec<u8>,
    pub length: usize,
}


impl Torrent {
    pub fn new(filename: Vec<u8>) -> Torrent {
        // openfile
        let mut f = File::open(str::from_utf8(&filename).unwrap()).unwrap();
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer);

        // read meta info dict
        let meta_info = Decoder::new(buffer.clone()).decode().unwrap().dict.unwrap();
        let info = meta_info.get(&b"info".to_vec()).unwrap();

        // get info hash
        let mut hasher = Sha1::new();
        hasher.input(&encode_decoded(info));
        let info_hash = hasher.result_str();

        let name = meta_info.get(&b"info".to_vec()).unwrap().dict.as_ref().unwrap().get(&b"name".to_vec()).unwrap().bytes.as_ref().unwrap().clone();
        let length = meta_info.get(&b"info".to_vec()).unwrap().dict.as_ref().unwrap().get(&b"length".to_vec()).unwrap().int.unwrap() as usize;

        // return
        Torrent {filename: filename, meta_info: meta_info, info_hash: info_hash.to_string(), name: name, length: length}
    }

    pub fn announce(&self) -> Vec<u8> {
        self.meta_info.get(&b"announce".to_vec()).unwrap().bytes.as_ref().unwrap().clone()
    }
    pub fn multi_file(&self) -> bool {
        self.meta_info.get(&b"info".to_vec()).unwrap().dict.as_ref().unwrap().contains_key(&b"files".to_vec())
    }
    pub fn piece_length(&self) -> usize {
        self.meta_info.get(&b"info".to_vec()).unwrap().dict.as_ref().unwrap().get(&b"piece length".to_vec()).unwrap().int.unwrap() as usize
    }
    pub fn total_size(&self) -> usize {
        self.length
    }
    pub fn pieces(&self) -> Vec<Vec<u8>> {
        let data = self.meta_info.get(&b"info".to_vec()).unwrap().dict.as_ref().unwrap().get(&b"pieces".to_vec()).unwrap().bytes.as_ref().unwrap().clone();
        let mut pieces: Vec<Vec<u8>> = Vec::new();
        let mut offset = 0;
        let length = data.len();

        while offset < length {
            pieces.push(data.get(offset..offset+20).unwrap().to_vec());
            offset += 20;
        }

        pieces
    }
    pub fn output_file(&self) -> Vec<u8> {
        self.meta_info.get(&b"info".to_vec()).unwrap().dict.as_ref().unwrap().get(&b"piece length".to_vec()).unwrap().bytes.as_ref().unwrap().clone()
    }
    pub fn to_string(&self) -> String {
        // TODO
        panic!("TODO");
        "".to_string()
    }
}


fn setup() -> Torrent {
    Torrent::new(b"data/ubuntu-16.04.6-desktop-amd64.iso.torrent".to_vec())
}

mod tests {
    // import parent scope
    use super::*;

    #[test]
    fn test_instantiate() {
        // no panics
        let t = setup();
    }

    #[test]
    fn test_announce() {
        let t = setup();
        assert_eq!(b"http://torrent.ubuntu.com:6969/announce".to_vec(), t.announce());
    }

    #[test]
    fn test_length() {
        let t = setup();
        assert_eq!(524288, t.piece_length());
    }

    #[test]
    fn test_file() {
        let t = setup();
        assert_eq!(b"data/ubuntu-16.04.6-desktop-amd64.iso.torrent".to_vec(), t.filename);
        assert_eq!(1664614400, t.length);
    }

    #[test]
    fn test_hash() {
        let t = setup();
        assert_eq!("ee55335f2acde309fa645fab11c04750d7e45fa1", t.info_hash);
    }

    #[test]
    fn test_total_size() {
        let t = setup();
        assert_eq!(1664614400, t.total_size());
    }

    #[test]
    fn test_pieces() {
        let t = setup();
        assert_eq!(3175, t.pieces().len());
    }
}
