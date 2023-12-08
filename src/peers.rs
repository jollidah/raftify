use std::{collections::HashMap, net::ToSocketAddrs};

use serde::{Deserialize, Serialize};

use crate::Peer;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Peers {
    pub peers: HashMap<u64, Peer>,
}

impl Default for Peers {
    fn default() -> Self {
        Self::new()
    }
}

impl Peers {
    pub fn new() -> Self {
        Self {
            peers: HashMap::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.peers.is_empty()
    }

    pub fn get(&self, id: &u64) -> Option<&Peer> {
        self.peers.get(&id)
    }

    pub fn get_mut(&mut self, id: &u64) -> Option<&mut Peer> {
        self.peers.get_mut(&id)
    }

    pub fn remove(&mut self, id: &u64) -> Option<Peer> {
        self.peers.remove(&id)
    }

    pub fn add_peer<A: ToSocketAddrs>(&mut self, id: u64, addr: A) {
        let addr = addr.to_socket_addrs().unwrap().next().unwrap();
        let peer = Peer::new(addr);
        self.peers.insert(id, peer);
    }

    pub fn reserve_peer(&mut self, self_id: u64) -> u64 {
        let next_id = self.peers.keys().max().cloned().unwrap_or(1);
        let next_id = std::cmp::max(next_id + 1, self_id);
        next_id
    }

    pub fn get_node_id_by_addr<A: ToSocketAddrs>(&self, addr: A) -> Option<u64> {
        let addr = addr.to_socket_addrs().unwrap().next().unwrap();
        self.peers
            .iter()
            .find(|(_, peer)| peer.addr == addr)
            .map(|(id, _)| *id)
    }
}