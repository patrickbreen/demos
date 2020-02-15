// tracker
//
//
// This module defines the tracker
//
// structs:
// Tracker
// TrackerResponse
//
use std::collections::BTreeMap;


pub struct TrackerResponse {
    response: BTreeMap<String, String>,
}

impl TrackerResponse {
    pub fn new(response: BTreeMap<String, String>) -> TrackerResponse {
        TrackerResponse {response: response}
    }
    pub fn failure(&self) -> Option<String> {
        if self.response.contains_key("failure_response") {
            return Some(self.response.get("failure_response").unwrap().to_string());
        }
        None
    }
    fn get_or_0(&self, key: &str) -> u64 {
        let s = self.response.get(key);
        if s.is_none() {
            return 0;
        }
        s.unwrap().parse().unwrap()
    }
    pub fn interval(&self) -> u64 {
        self.get_or_0("interval")
    }
    pub fn complete(&self) -> u64 {
        self.get_or_0("complete")
    }
    pub fn incomplete(&self) -> u64 {
        self.get_or_0("incomplete")
    }
    pub fn peers(&self) -> Vec<(String, u64)> {
        let s = self.response.get("peers").unwrap();

        let raw_peers: Vec<&str> = s.split(",").collect();
        let mut formatted_peers: Vec<(String, u64)> = Vec::new();

        for peer in raw_peers {
            let attrs: Vec<&str> = peer.split(":").collect();
            let ip_addr = attrs.get(0).unwrap().to_string();
            let port: u64 = attrs.get(0).unwrap().parse().unwrap();
            formatted_peers.push((ip_addr, port));
        }
        formatted_peers
    }
    pub fn to_string(&self) -> String {
        format!("incomplete: ")
    }
}

pub struct Tracker {}

impl Tracker {
    pub fn new() -> Tracker {
        Tracker {}
    }
    pub fn connect(&self) -> TrackerResponse {
        TrackerResponse::new(BTreeMap::new())
    }
    pub fn close(&self) {
    }
    pub fn raise_for_error(&self) {
        panic!("error");
    }
    fn construct_tracker_parameters(&self) {
    }
}

fn calculate_peer_id() -> [u8;20] {
    [0; 20]
}

fn decode_port(port: [u8;4]) -> u32 {
    0
}


#[cfg(test)]
mod tests {
    // import parent scope
    use super::*;

    fn test_response() {
        let response = TrackerResponse::new(BTreeMap::new());
    }

    fn test_tracker() {
        let tracker = Tracker::new();
    }
}
