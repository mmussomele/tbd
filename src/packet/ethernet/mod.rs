use super::network;

pub struct Packet<'a> {
    pub header: Header,
    pub next: network::Packet<'a>,
}

#[derive(Default)]
pub struct Header {
    pub source: [u8; 6],
    pub dest: [u8; 6],
    pub ethertype: u16,
}
