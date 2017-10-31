use super::super::application;

pub struct Packet<'a> {
    header: Header,
    next: application::Packet<'a>,
}

#[derive(Default)]
pub struct Header {
    source: u16,
    dest: u16,
    length: u16,
    checksum: u16,
}
