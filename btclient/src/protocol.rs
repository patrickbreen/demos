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

    // The handshake message is the first message sent and then received from a
    // remote peer.
    // The messages is always 68 bytes long (for this version of BitTorrent
    // protocol).
    // Message format:
    //     <pstrlen><pstr><reserved><info_hash><peer_id>
    // In version 1.0 of the BitTorrent protocol:
    //     pstrlen = 19
    //     pstr = "BitTorrent protocol".
    // Thus length is:
    //     49 + len(pstr) = 68 bytes long.

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


struct KeepAlive {
}

impl PeerMessage for KeepAlive {
    // The Keep-Alive message has no payload and length is set to zero.

    fn encode(&mut self) -> Vec<u8> {
        panic!("Not implemented");
    }
    fn decode(data: Vec<u8>) -> Self {
        panic!("Not implemented");
    }
    fn to_string() -> String {
        "KeepAlive".to_string()
    }
}

struct BitField {
    //TODO
    bitfield: Vec<u8>,
}

impl BitField {

    fn new(data: Vec<u8>) -> BitField {
        BitField { bitfield: data}
    }
}

impl PeerMessage for BitField {

    // The BitField is a message with variable length where the payload is a
    // bit array representing all the bits a peer have (1) or does not have (0).
    // Message format:
    //     <len=0001+X><id=5><bitfield>

    // for now I don't see a reason to treat this as bits, so I'll treat it as bytes

    fn encode(&mut self) -> Vec<u8> {
        let bits_length = self.bitfield.len() * 8;
        let s = structure!(">Ib");

        let mut buf = s.pack(1 + bits_length as u32, BitField::BITFIELD as i8).unwrap();

        let mut ret  = Vec::new();
        ret.append(&mut buf);
        ret.append(&mut self.bitfield);
        ret
    }
    fn decode(data: Vec<u8>) -> Self {

        // get message length
        let s_len = structure!(">Ib");
        let  (message_length, message_type) = s_len.unpack(data[0..5].to_vec()).unwrap();

        BitField::new(data[5..].to_vec())
    }
    fn to_string() -> String {
        "BitField".to_string()
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
 
