use super::super::transport;

pub struct Packet<'a> {
    pub header: Header<'a>,
    pub next: transport::Packet<'a>,
}

#[derive(Default)]
pub struct Header<'a> {
    pub version: u8, // only lower 4 bits used
    pub ihl: u8, // only lower 4 bits used
    pub dscp: u8, // only lower 6 bits used
    pub ecn: u8, // only lower 2 bits used
    pub total_length: u16,
    pub identification: u16,
    pub df: bool,
    pub mf: bool,
    pub fragment_offset: u16, // only lower 13 bits used
    pub ttl: u8,
    pub protocol: u8,
    pub checksum: u16,
    pub source: u32,
    pub dest: u32,
    pub options: &'a [u8],
}
