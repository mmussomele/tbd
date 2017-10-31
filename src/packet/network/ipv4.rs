use super::super::transport;

pub struct Packet<'a> {
    header: Header<'a>,
    next: transport::Packet<'a>,
}

#[derive(Default)]
pub struct Header<'a> {
    version: u8, // only lower 4 bits used
    ihl: u8, // only lower 4 bits used
    dscp: u8, // only lower 6 bits used
    ecn: u8, // only lower 2 bits used
    total_length: u16,
    identification: u16,
    df: bool,
    mf: bool,
    fragment_offset: u16, // only lower 13 bits used
    ttl: u8,
    protocol: u8,
    checksum: u16,
    source: u32,
    dest: u32,
    options: &'a [u8],
}
