use super::link;
use super::Entity;
use super::super::packet;
use packet::ethernet;
use packet::network;

use std::collections::HashMap;

#[derive(Default)]
pub struct Router<'a> {
    links: &'a [link::Link<'a>], // Set of links, index == port
    table: HashMap<u32, usize>, // Map from IP -> port
    ip: u32, // unused for now
    mac: [u8; 6],
}

impl<'a> Entity for Router<'a> {
    fn process(&self, p: packet::Packet) {
        match p {
            packet::Packet::L2(e) => self.handle_l2(e),
            _ => (), // Ignore any non-ethernet packets.
            // TODO: Add a class of control messages for route computation.
            // TODO: Resolve next hop MAC using ARP.
        }
    }

    // TODO: This should be removed eventually, when ARP is implemented.
    fn mac(&self) -> [u8; 6] {
        self.mac
    }
}

impl<'a> Router<'a> {
    fn handle_l2(&self, mut e: ethernet::Packet<'a>) {
        if !(e.header.dest == self.mac) {
            return; // drop the packet
        }

        let l;
        match e.next {
            network::Packet::IPV4(ref mut n) => l = self.handle_ipv4(n),
        }

        match l {
            Some(ref l) => {
                e.header.source = self.mac;
                e.header.dest = l.dest.mac();
                l.transmit(packet::Packet::L2(e));
            }
            None => (), // drop the packet
        }
    }

    fn handle_ipv4(&self, n: &mut network::ipv4::Packet<'a>) -> Option<&link::Link> {
        n.header.ttl -= 1;
        if n.header.ttl <= 0 {
            return None;
        }

        match self.table.get(&n.header.dest) {
            Some(i) => Some(&self.links[*i]),
            None => return None,
        }
    }
}
