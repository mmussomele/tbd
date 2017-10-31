use super::super::application;

pub struct Packet<'a> {
    header: Header<'a>,
    next: application::Packet<'a>,
}

#[derive(Default)]
pub struct Header<'a> {
    source: u16,
    dest: u16,
    seq: u32,
    ack_no: u32,
    data_offset: u8, // only lower 4 bits used
    ns: bool,
    cwr: bool,
    ece: bool,
    urg: bool,
    ack: bool,
    psh: bool,
    rst: bool,
    syn: bool,
    fin: bool,
    window_size: u16,
    checksum: u16,
    urgent_ptr: u16,
    options: &'a [u8],
}
