use super::network;

pub struct Packet<'a> {
    header: Header,
    next: network::Packet<'a>,
}

#[derive(Default)]
pub struct Header {
    source: [u8; 6],
    dest: [u8; 6],
    ethertype: u16,
}
