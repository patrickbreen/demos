// client
//
//
// Define TorrentClient, Piece, Block, Piece Manager

use std::fs::File;
use std::path::Path;
use std::collections::HashMap;
use std::str;

use crypto::digest::Digest;
use crypto::sha1::Sha1;

use crate::torrent::Torrent;




pub struct TorrentClient {

}

impl TorrentClient {


    fn new() -> TorrentClient {
        TorrentClient {}
    }

    fn parse(&self) {
    }

}


struct Block {
    piece: u32,
    offset: u32,
    length: u32,
    status: u8,
    data: Vec<u8>
}

impl Block {

    // The block is a partial piece, this is what is requested and transferred
    // between peers.

    // A block is most often of the same size as the REQUEST_SIZE, except for the
    // final block which might (most likely) is smaller than REQUEST_SIZE.

    const MISSING: u8 = 1;
    const PENDING: u8 = 1;
    const RETRIEVED: u8 = 1;


    fn new(piece: u32, offset: u32, length: u32) -> Block {
        Block {
            piece: piece,
            offset: offset,
            length: length,
            status: Block::MISSING,
            data: Vec::new(),
        }
    }

}


struct Piece {
    index: usize,
    blocks: Vec<Block>,
    hash: Vec<u8>,
}

impl Piece {
    // The piece is a part of of the torrents content. Each piece except the final
    // piece for a torrent has the same length (the final piece might be shorter).

    // A piece is what is defined in the torrent meta-data. However, when sharing
    // data between peers a smaller unit is used - this smaller piece is refereed
    // to as `Block` by the unofficial specification (the official specification
    // uses piece for this one as well, which is slightly confusing).

    fn new(index: usize, blocks: Vec<Block>, hash: Vec<u8>,) -> Piece {
        Piece {
            index: index,
            blocks: blocks,
            hash: hash,
        }
    }

    fn reset(&mut self) {

        for block in &mut self.blocks {
            block.status = Block::MISSING;
        }
    }

    fn next_request(&mut self) -> Option<usize> {

        let mut n_request = None;

        for (i, mut block) in  self.blocks.iter_mut().enumerate() {
            if block.status == Block::MISSING {
                block.status = Block::PENDING;
                n_request = Some(i);
            }
        }
        return n_request;
    }

    fn block_received(&mut self, offset: u32, data: Vec<u8>) {

        for block in  self.blocks.iter_mut() {
            if block.status == Block::RETRIEVED {
                block.status = Block::PENDING;
                block.data = data;
                break;
            }
        }
        panic!("Trying to complete a non-existing block");
    }


    fn is_complete(&self) -> bool {
        for block in &self.blocks {
            if block.status != Block::RETRIEVED {
                return false;
            }
        }
        return true;
    }

    fn is_hash_matching(&mut self) -> bool {
        let mut hasher = Sha1::new();
        hasher.input(&self.data());
        let mut piece_hash: [u8; 20] = [0; 20];
        hasher.result(&mut piece_hash);
        return self.hash == piece_hash.to_vec();
    }

    fn data(&mut self) -> Vec<u8> {
        let mut ret = Vec::new();
        self.blocks.sort_by_key(|b| b.offset);
        for block in &self.blocks {
            ret.extend_from_slice(&block.data);
        }

        ret
    }

}


// TODO
struct PieceManager {
    torrent: Torrent,
    peers: HashMap<Vec<u8>,(String, u16)>,  
    pending_blocks: Vec<Block>,
    missing_pieces: Vec<Piece>,
    ongoing_pieces: Vec<Piece>,
    have_pieces: Vec<Piece>,
    total_pieces: usize,
    file: File,
}

impl PieceManager {


    // The PieceManager is responsible for keeping track of all the available
    // pieces for the connected peers as well as the pieces we have available for
    // other peers.
    
    // The strategy on which piece to request is made as simple as possible in
    // this implementation.

    const max_pending_time: u32 = 300*1000;

    fn new(torrent: Torrent) -> PieceManager {
        let file_str = str::from_utf8(&torrent.output_file()).unwrap().to_string();
        let file_path = Path::new(&file_str);
        let total_pieces = torrent.pieces().len();
        let piece_manager = PieceManager {
            torrent: torrent,
            peers: HashMap::new(),
            pending_blocks: Vec::new(),
            missing_pieces: Vec::new(),
            ongoing_pieces: Vec::new(),
            have_pieces: Vec::new(),
            total_pieces: total_pieces,
            file: File::open(file_path).unwrap(),
        };
        piece_manager.initiate_pieces();
        piece_manager
    }

    fn initiate_pieces(&self) {
        // TODO
    }


    fn close(&self) {
        // No implementation because file will auto-close when it goes out of scope
    }

    fn complete(&self) -> bool {
        self.have_pieces.len() == self.total_pieces
    }

    fn bytes_downloaded(&self) -> usize {
        self.have_pieces.len() * self.torrent.piece_length()
    }

    fn bytes_uploaded(&self) -> usize {
        // TODO - no support for uploading
        0
    }

    fn add_peer(&mut self, peer_id: Vec<u8>, bitfield: (String, u16)) {
        self.peers.insert(peer_id, bitfield);
    }

    fn update_peer(&mut self, peer_id: Vec<u8>, bitfield: (String, u16)) {
        if self.peers.contains_key(&peer_id) {
            self.peers.insert(peer_id, bitfield);
        }
    }

    fn remove_peer(&self) {
        // TODO
    }


}


mod tests {
    // import parent scope
    use super::*;

    #[test]
    fn test() {

        assert!(true);
    }
}
