// protocol
//
//
// This module defines connections
// 
//
// structs:
// PeerConnection
// PeerStreamIterator
// PeerMessage
// Handshake
// KeepAlive
// BitField
// Interested
// NotInterested
// Choke
// Unchoke
// Have
// Request
// Piece
// Cancel


const REQUEST_SIZE: usize = 16384;

// TODO
struct PeerConnection {
}

// TODO
struct PeerStreamIterator {
}

impl PeerStreamIterator {
    const CHUNK_SIZE: usize = 10*1024;

    fn new(buffer: Vec<u8>) -> PeerStreamIterator {
        PeerStreamIterator {}
    }

    fn parse(&self) {
    }

}


trait PeerMessage {
    const CHOKE: u8 = 0;
    const UNCHOKE: u8 = 1;
    const INTERESTED: u8 = 2;
    const NOTINTERESTED: u8 = 3;
    const HAVE: u8 = 4;
    const BITFIELD: u8 = 5;
    const REQUEST: u8 = 6;
    const PIECE: u8 = 7;
    const CANCEL: u8 = 8;
    const PORT: u8 = 9;
    // const HANDSHAKE: u8 = None;
    // const KEEPALIVE: u8 = None;
    
    fn encode(&mut self) -> Vec<u8>;
    fn decode(data: Vec<u8>) -> Self;
    fn to_string() -> String;
}

struct Handshake {
    info_hash: Vec<u8>,
    peer_id: Vec<u8>,
}

impl Handshake {
    const length: usize = 49 + 19;

    fn new(info_hash: Vec<u8>, peer_id: Vec<u8>) -> Handshake {
        Handshake {
            info_hash: info_hash,
            peer_id: peer_id,
        }
    }
}

impl PeerMessage for Handshake {
    fn encode(&mut self) -> Vec<u8> {
        let mut ret : Vec<u8> = Vec::new();
        ret.push(19);
        ret.append(&mut b"BitTorrent protocol".to_vec());
        ret.append(&mut b"\x00\x00\x00\x00\x00\x00\x00\x00".to_vec());
        ret.append(&mut self.info_hash);
        ret.append(&mut self.peer_id);
        assert!(ret.len() == Handshake::length);
        ret
    }

    fn decode(data: Vec<u8>) -> Handshake {
        if data.len() < Handshake::length {
            panic!("Data: {} is smaller than min: {}", data.len(), Handshake::length);
        }
        let info_hash = data[28..48].to_vec();
        let peer_id = data[48..68].to_vec();

        Handshake::new(info_hash, peer_id)
    }

    fn to_string() -> String {
        "Handshake".to_string()
    }
}

#[cfg(test)]
mod tests {
    // import parent scope
    use super::*;

    fn make_handshake() -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.append(&mut b"\x13BitTorrent protocol\x00\x00\x00\x00\x00\x00\x00\x00".to_vec());
        bytes.append(&mut b"CDP;~y~\xbf1X#'\xa5\xba\xae5\xb1\x1b\xda\x01".to_vec());
        bytes.append(&mut b"-qB3200-iTiX3rvfzMpr".to_vec());
        bytes
    }

    #[test]
    fn test_handshake_construction() {
        let mut handshake = Handshake::new(b"CDP;~y~\xbf1X#'\xa5\xba\xae5\xb1\x1b\xda\x01".to_vec(),
                                       b"-qB3200-iTiX3rvfzMpr".to_vec());
       assert_eq!(handshake.encode(), make_handshake());
    }

    #[test]
    fn test_handshake_parse() {
        let handshake = Handshake::decode(make_handshake());

        assert_eq!(b"CDP;~y~\xbf1X#'\xa5\xba\xae5\xb1\x1b\xda\x01".to_vec(), handshake.info_hash);
        assert_eq!(b"-qB3200-iTiX3rvfzMpr".to_vec(), handshake.peer_id);

    }
}
 
