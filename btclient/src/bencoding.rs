// bencoding
//
//
// This file parses .torrent files

use std::str;
use std::collections::BTreeMap;

// start of integer
const TOKEN_INTEGER: u8 = b'i';

// start of list
const TOKEN_LIST: u8 = b'l';

// start of dict
const TOKEN_DICT: u8 = b'd';

// end of lists, dicts, and integers
const TOKEN_END: u8 = b'e';

// delimits the string length from data
const TOKEN_STRING_SEPERATOR: u8 = b':';



// ENCODER
fn encode_int(i: &i64) -> Vec<u8> {

    let mut ret: Vec<u8> = Vec::new();
    ret.push(TOKEN_INTEGER);
    ret.append(&mut i.to_string().as_bytes().to_vec());
    ret.push(TOKEN_END);

    return ret;
}

fn encode_string(value: &String) -> Vec<u8> {
    let len = value.as_bytes().len().to_string();
    let s = len + ":" + &value;
    return s.as_bytes().to_vec();
}


fn encode_list(data: &Vec<Box<Encodable>>) -> Vec<u8> {
   let mut ret: Vec<u8> = Vec::new();
   ret.push(TOKEN_LIST);
   for dat in data {
       let mut encoded = dat.encode();
       ret.append(&mut encoded);
   }
   ret.push(TOKEN_END);
   ret
}

fn encode_dict(data: &BTreeMap<Vec<u8>, Box<Encodable>>) -> Vec<u8> {

    let mut ret = Vec::new();
    ret.push(TOKEN_DICT);

    for (k, v) in data {
        ret.append(&mut k.encode());
        ret.append(&mut v.encode());
    }

    ret.push(TOKEN_END);
    ret
}

fn encode_bytes(value: &Vec<u8>) -> Vec<u8> {
    let mut copy = value.clone();
    let mut ret: Vec<u8> = copy.len().to_string().as_bytes().to_vec();
    ret.push(TOKEN_STRING_SEPERATOR);
    ret.append(&mut copy);
    ret

}

trait Encodable {
    fn encode(&self) -> Vec<u8>;
}

impl Encodable for i64 {
    fn encode(&self) -> Vec<u8> {
        encode_int(self)
    }
}

impl Encodable for String {
    fn encode(&self) -> Vec<u8> {
        encode_string(self)
    }
}

impl Encodable for Vec<u8> {
    fn encode(&self) -> Vec<u8> {
        encode_bytes(self)
    }
}

impl Encodable for Vec<Box<Encodable>> {
    fn encode(&self) -> Vec<u8> {
        encode_list(self)
    }
}

impl Encodable for BTreeMap<Vec<u8>, Box<Encodable>> {
    fn encode(&self) -> Vec<u8> {
        encode_dict(self)
    }
}

pub fn encode_decoded(data: &Decoded) -> Vec<u8> {
    let mut ret = Vec::new();
    
    match data.type_name.as_str() {
        "bytes" => ret.append(&mut data.bytes.as_ref().unwrap().encode()),
        "i64" => ret.append(&mut data.int.unwrap().encode()),
        "list" => {
            let list = data.list.as_ref().unwrap();
            ret.push(TOKEN_LIST);
            for elem in list {
                ret.append(&mut encode_decoded(elem));
            }
            ret.push(TOKEN_END);
        },
        "dict" => {
            let dict = data.dict.as_ref().unwrap();
            ret.push(TOKEN_DICT);

            for (k, v) in dict {
                ret.append(&mut k.encode());
                ret.append(&mut encode_decoded(v));
            }

            ret.push(TOKEN_END);
 
        },
        _ => panic!("unknown type_name"),
    }

    ret
}


pub struct Encoder {
    data: Box<Encodable>,
}

impl Encoder {
    pub fn new(data: Box<Encodable>) -> Encoder {
        Encoder {data: data}
    }

    pub fn encode(&self) -> Vec<u8> {
        self.data.encode()
    }
}

// DECODER


#[derive(Debug)]
pub struct Decoder {
    data: Vec<u8>,
    index: usize,
}

pub struct Decoded {
    pub type_name: String,
    pub bytes: Option<Vec<u8>>,
    pub int: Option<i64>,
    pub list: Option<Vec<Decoded>>,
    pub dict: Option<BTreeMap<Vec<u8>, Decoded>>,
}


impl Decoder {
    pub fn new(data: Vec<u8>) -> Decoder {
        Decoder {data: data, index: 0}
    }

    pub fn decode(&mut self) -> Option<Decoded> {

        let c = self.peek();

        match c {
            None => return None,
            Some(TOKEN_INTEGER) => {
                self.consume();
                return self.decode_int();
            },
            Some(TOKEN_END) => {
                return None
            },
            Some(TOKEN_LIST) => {
                self.consume();
                return self.decode_list();
            },
            Some(TOKEN_DICT) => {
                self.consume();
                return self.decode_dict();
            },
            Some(c) => {
                if b'0' <= c && c  <= b'9' {
                    return self.decode_bytes();
                } else {
                    panic!("invalid token at {}", self.index);
                }
            }
        }
    }

    fn peek(&self) -> Option<u8> {
        if self.index >= self.data.len() {
            return None;
        } 
        Some(self.data[self.index])
    }

    fn consume(&mut self) {
        self.index += 1;
    }

    fn read(&mut self, len: usize) -> Option<Vec<u8>> {
        if self.index + len > self.data.len() {
            panic!("tried to read beyond data");
        }
        let res = Some(self.data[self.index..(self.index+len)].to_vec());
        self.index += len;
        res
    }

    fn read_until(&mut self, token: u8) -> Option<Vec<u8>> {
        let mut local_index = self.index;
        for elem in &self.data[self.index..] {
            if *elem == token {
                let occurence = local_index;
                let result = &self.data[self.index..occurence];
                self.index += (occurence - self.index) + 1;
                return Some(result.to_vec());
            } else {
                local_index += 1;
            }
        }
            return None;
    }

    fn decode_int(&mut self) -> Option<Decoded> {
        let read = self.read_until(TOKEN_END);

        if read == None {
            return None;
        }
        let vec = read.unwrap();
        let i: i64 = String::from_utf8(vec).unwrap().parse().unwrap();

        Some(Decoded {
            type_name: "i64".to_string(),
            bytes: None,
            int: Some(i),
            list: None,
            dict: None
        })
    }

    fn decode_list(&mut self) -> Option<Decoded> {
        let mut ret = Vec::new();
        while self.data[self.index] != TOKEN_END {
            ret.push(self.decode().unwrap());
        }
        self.consume();
        Some(Decoded {
            type_name: "list".to_string(),
            bytes: None,
            int: None,
            list: Some(ret),
            dict: None
        })
    }

    fn decode_dict(&mut self) -> Option<Decoded> {
        let mut res = BTreeMap::new();
        while self.data[self.index] != TOKEN_END {
            let key = self.decode().unwrap().bytes.unwrap();
            let val = self.decode().unwrap();
            res.insert(key, val);
        }
        self.consume();

        Some(Decoded {
            type_name: "dict".to_string(),
            bytes: None,
            int: None,
            list: None,
            dict: Some(res)
        })
    }

    fn decode_bytes(&mut self) -> Option<Decoded> {
        let vec = self.read_until(TOKEN_STRING_SEPERATOR).unwrap();
        let len: usize = str::from_utf8(&vec).unwrap().parse().unwrap();
        let data = self.read(len).unwrap();

        Some(Decoded {
            type_name: "bytes".to_string(),
            bytes: Some(data),
            int: None,
            list: None,
            dict: None
        })
    }
}



#[cfg(test)]
mod tests {
    // import parent scope
    use super::*;


    // DECODING TESTS
    #[test]
    fn test_peek_itempotent() {
        let decoder = Decoder::new(b"12".to_vec());
        assert_eq!(b'1', decoder.peek().unwrap());
        assert_eq!(b'1', decoder.peek().unwrap());
    }

    #[test]
    fn test_peek_handles_end() {
        let mut decoder = Decoder::new(b"1".to_vec());
        decoder.index = 1;
        assert_eq!(None, decoder.peek());
    }

    #[test]
    fn test_read_until_found() {
        let mut decoder = Decoder::new(b"123456".to_vec());
        assert_eq!(b"123".to_vec(), decoder.read_until(b'4').unwrap());
    }

    #[test]
    fn test_read_until_not_found() {
        let mut decoder = Decoder::new(b"123456".to_vec());
        assert_eq!(None, decoder.read_until(b'7'));
    }

    #[test]
    fn test_empty_bytes() {
        let res = Decoder::new(b"".to_vec()).decode();
        assert!(res.is_none());
    }

    #[test]
    fn test_integer() {
        let res = Decoder::new(b"i123e".to_vec()).decode().unwrap();
        assert!(res.int == Some(123));
    }

    #[test]
    fn test_bytes() {
        let res = Decoder::new(b"4:name".to_vec()).decode().unwrap();
        assert!(res.bytes == Some(b"name".to_vec()));
    }

    #[test]
    fn test_min_bytes() {
        let res = Decoder::new(b"1:a".to_vec()).decode().unwrap();
        assert!(res.bytes == Some(b"a".to_vec()));
    }

    #[test]
    fn test_string_with_space() {
        let mut decoder = Decoder::new(b"11:hello world".to_vec());
        let res = decoder.decode().unwrap();
        assert_eq!(res.bytes, Some(b"hello world".to_vec()));
        assert_eq!(decoder.index, 14);
    }

    #[test]
    fn test_list() {
        let res = Decoder::new(b"l4:spam4:eggsi1234ee".to_vec()).decode().unwrap();
        let list = res.list.unwrap();
        assert_eq!(list.len(), 3);
        assert_eq!(list[0].bytes.as_ref().unwrap(), &b"spam".to_vec());
        assert_eq!(list[1].bytes.as_ref().unwrap(), &b"eggs".to_vec());
        assert_eq!(list[2].int, Some(1234));
    }

    #[test]
    fn test_dict() {
        let res = Decoder::new(b"d4:key16:value14:key26:value2e".to_vec()).decode().unwrap();
        let dict = res.dict.unwrap();
        assert_eq!(dict.len(), 2);
        assert_eq!(dict.get(&b"key1".to_vec()).unwrap().bytes.as_ref().unwrap(), &b"value1".to_vec());
    }
    

    // ENCODING TESTS

    #[test]
    fn test_encode_integer() {
        let res = Encoder::new(Box::new(1234)).encode();
        assert_eq!(b"i1234e".to_vec(), res);
    }

    #[test]
    fn test_encode_string() {
        let res = Encoder::new(Box::new(b"blah".to_vec())).encode();
        assert_eq!(b"4:blah".to_vec(), res);
    }

    #[test]
    fn test_encode_list() {
        let l: Vec<Box<Encodable>> = vec![Box::new(b"potato".to_vec()), Box::new(b"carrot".to_vec()), Box::new(1234) ];
        let res = Encoder::new(Box::new(l)).encode();
        assert_eq!(b"l6:potato6:carroti1234ee".to_vec(), res);
    }
 
    #[test]
    fn test_encode_dict() {
        let mut d: BTreeMap<Vec<u8>,Box<Encodable>> = BTreeMap::new();
        d.insert(b"key1".to_vec(), Box::new(b"carrot".to_vec()));
        d.insert(b"key2".to_vec(), Box::new(1234));
        let res = Encoder::new(Box::new(d)).encode();
        assert_eq!(b"d4:key16:carrot4:key2i1234ee".to_vec(), res);
    }

    #[test]
    fn test_nested() {
        let l: Vec<Box<Encodable>> = vec![Box::new("potato".to_string()), Box::new("carrot".to_string()), Box::new(1234) ];
        let mut d: BTreeMap<Vec<u8>,Box<Encodable>> = BTreeMap::new();
        d.insert(b"key1".to_vec(), Box::new(b"carrot".to_vec()));
        d.insert(b"key2".to_vec(), Box::new(l));
        let res = Encoder::new(Box::new(d)).encode();
        assert_eq!(b"d4:key16:carrot4:key2l6:potato6:carroti1234eee".to_vec(), res);
    }

    #[test]
    fn test_round_trip() {
        let original = b"l4:spam4:eggsi1234ee";
        let decoded = Decoder::new(original.to_vec()).decode().unwrap();
        let encoded = encode_decoded(&decoded);

        assert_eq!(original.to_vec(), encoded);

    }


}
