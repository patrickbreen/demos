// torrent

use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::collections::BTreeMap;

use crypto::sha1::Sha1;
use crypto::digest::Digest;

use crate::bencoding::{Decoded, Decoder, Encoder, encode_decoded};



pub struct Torrent {
    filename: String,
    meta_info: BTreeMap<String, Decoded>,
    info_hash: String,
    name: String,
    length: usize,
}


impl Torrent {
    pub fn new(filename: String) -> Torrent {
        // openfile
        let mut f = File::open(&filename).unwrap();
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer);

        // read meta info dict
        let meta_info = Decoder::new(buffer.clone()).decode().unwrap().dict.unwrap();
        let info = meta_info.get("info").unwrap();

        // get info hash
        let mut hasher = Sha1::new();
        hasher.input(&encode_decoded(info));
        let info_hash = hasher.result_str();

        let name = "".to_string();
        let length = 0;

        // return
        Torrent {filename: filename, meta_info: meta_info, info_hash: info_hash, name: name, length: length}
    }

    pub fn announce(&self) -> String {
        self.meta_info.get("announce").unwrap().sval.as_ref().unwrap().clone()
    }
    pub fn multi_file(&self) -> bool {
        self.meta_info.get("info").unwrap().dict.as_ref().unwrap().contains_key("files")
    }
    pub fn piece_length(&self) -> usize {
        self.meta_info.get("info").unwrap().dict.as_ref().unwrap().get("piece length").unwrap().int.unwrap() as usize
    }
    pub fn total_size(&self) -> usize {
        self.length
    }
    pub fn pieces(&self) -> Vec<String> {
        let data = self.meta_info.get("info").unwrap().dict.as_ref().unwrap().get("pieces").unwrap().sval.as_ref().unwrap().clone();
        let mut pieces: Vec<String> = Vec::new();
        let mut offset = 0;
        let length = data.len();

        while offset < length {
            pieces.push(data.get(offset..offset+20).unwrap().to_string());
            offset += 20;
        }

        pieces
    }
    pub fn output_file(&self) -> String {
        self.meta_info.get("info").unwrap().dict.as_ref().unwrap().get("piece length").unwrap().sval.as_ref().unwrap().clone()
    }
    pub fn to_string(&self) -> String {
        // TODO
        "".to_string()
    }
}


mod tests {
    // import parent scope
    use super::*;

    #[test]
    fn test() {
        // TODO
        assert!(true);
    }
}
