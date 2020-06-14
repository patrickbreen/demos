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


// TODO!!!
// implement PeerConnection, PeerStreamIterator, and Test


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

struct Interested {
}

impl PeerMessage for Interested {
    // The interested message is fix length and has no payload other than the
    // message identifiers. It is used to notify each other about interest in
    // downloading pieces.

    fn encode(&mut self) -> Vec<u8> {
        let s = structure!(">Ib");
        s.pack(1, BitField::INTERESTED as i8).unwrap()
    }
    fn decode(data: Vec<u8>) -> Self {
        panic!("Not implemented");
    }
    fn to_string() -> String {
        "Interested".to_string()
    }
}


struct NotInterested {
}

impl PeerMessage for NotInterested {
    // The not interested message is fix length and has no payload other than the
    // message identifier. It is used to notify each other that there is no
    // interest to download pieces.

    fn encode(&mut self) -> Vec<u8> {
        panic!("Not implemented");
    }
    fn decode(data: Vec<u8>) -> Self {
        panic!("Not implemented");
    }
    fn to_string() -> String {
        "NotInterested".to_string()
    }
}


struct Choke {
}

impl PeerMessage for Choke {
    // The choke message is used to tell the other peer to stop send request
    // messages until unchoked.

    fn encode(&mut self) -> Vec<u8> {
        panic!("Not implemented");
    }
    fn decode(data: Vec<u8>) -> Self {
        panic!("Not implemented");
    }
    fn to_string() -> String {
        "Choke".to_string()
    }
}



struct UnChoke {
}

impl PeerMessage for UnChoke {
    // Unchoking a peer enables that peer to start requesting pieces from the
    // remote peer.

    fn encode(&mut self) -> Vec<u8> {
        panic!("Not implemented");
    }
    fn decode(data: Vec<u8>) -> Self {
        panic!("Not implemented");
    }
    fn to_string() -> String {
        "UnChoke".to_string()
    }
}



struct Have {
    index: u32,
}

impl Have {
    fn new(index: u32) -> Have {
        Have { index: index }
    }
}

impl PeerMessage for Have {
    // Represents a piece successfully downloaded by the remote peer. The piece
    // is a zero based index of the torrents pieces

    fn encode(&mut self) -> Vec<u8> {
        let s = structure!(">IbI");
        s.pack(5, BitField::HAVE as i8, self.index).unwrap()
    }
    fn decode(data: Vec<u8>) -> Self {
        let s = structure!(">IbI");
        let  (message_length, message_type, index) = s.unpack(data).unwrap();

        Have::new(index)
    }
    fn to_string() -> String {
        "Have".to_string()
    }
}



struct Request {
    index: u32,
    begin: u32,
    length: u32,
}

impl Request {
    fn new(index: u32, begin: u32, length: u32) -> Request {
        Request {
            index: index,
            begin: begin,
            length: length,
        }
    }
}

impl PeerMessage for Request {
    // The message used to request a block of a piece (i.e. a partial piece).
    // The request size for each block is 2^14 bytes, except the final block
    // that might be smaller (since not all pieces might be evenly divided by the
    // request size).

    fn encode(&mut self) -> Vec<u8> {
        let s = structure!(">IbIII");
        s.pack(13, BitField::REQUEST as i8, self.index, self.begin, self.length).unwrap()
    }
    fn decode(data: Vec<u8>) -> Self {
        let s = structure!(">IbIII");
        let  (message_length, message_type, index, begin, length) = s.unpack(data).unwrap();

        Request::new(index, begin, length)
    }
    fn to_string() -> String {
        "Request".to_string()
    }
}


struct Piece {
    index: u32,
    begin: u32,
    block: Vec<u8>,
}

impl Piece {
    const LENGTH: usize = 9;


    fn new(index: u32, begin: u32, block: Vec<u8>) -> Piece {
        Piece {
            index: index,
            begin: begin,
            block: block
        }
    }
}

impl PeerMessage for Piece {
    // A block is a part of a piece mentioned in the meta-info. The official
    // specification refer to them as pieces as well - which is quite confusing
    // the unofficial specification refers to them as blocks however.

    // So this class is named `Piece` to match the message in the specification
    // but really, it represents a `Block` (which is non-existent in the spec).

    fn encode(&mut self) -> Vec<u8> {
        let message_length = Piece::LENGTH + self.block.len();
        let s = structure!(">IbII");
        let mut buf = s.pack(message_length as u32, BitField::PIECE as i8, self.index, self.begin).unwrap();
        buf.append(&mut self.block.clone()); // clone so that block isn't emptied
        buf
    }
    fn decode(data: Vec<u8>) -> Self {
        let s = structure!(">IbII");
        let  (message_length, message_type, index, begin) = s.unpack(data[..13].to_vec()).unwrap();
        let block = data[13..].to_vec();

        Piece::new(index, begin, block)
    }
    fn to_string() -> String {
        "Piece".to_string()
    }
}


struct Cancel {
    index: u32,
    begin: u32,
    length: u32,
}

impl Cancel {
    fn new(index: u32, begin: u32, length: u32) -> Cancel {
        Cancel {
            index: index,
            begin: begin,
            length: length,
        }
    }
}

impl PeerMessage for Cancel {
    // The cancel message is used to cancel a previously requested block (in fact
    // the message is identical (besides from the id) to the Request message).

    fn encode(&mut self) -> Vec<u8> {
        let s = structure!(">IbIII");
        s.pack(13, BitField::CANCEL as i8, self.index, self.begin, self.length).unwrap()
    }
    fn decode(data: Vec<u8>) -> Self {
        let s = structure!(">IbIII");
        let  (message_length, message_type, index, begin, length) = s.unpack(data).unwrap();

        Cancel::new(index, begin, length)
    }
    fn to_string() -> String {
        "Cancel".to_string()
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
 
