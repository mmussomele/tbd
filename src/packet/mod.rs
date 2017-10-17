use std::error;
use std::fmt;

mod application;
mod transport;
mod network;
mod datalink;

pub enum Header<'a> {
    App(application::Header<'a>),
    L4(transport::Header<'a>),
    L3(network::Header),
    L2(datalink::Header),
}

#[derive(Debug)]
pub struct ParseError {
    reason: String,
}

impl error::Error for ParseError {
    fn description(&self) -> &str {
        &self.reason[..]
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.reason)
    }
}

fn pop_u16(data: &[u8]) -> (u16, &[u8]) {
    (((data[0] as u16) << 8) | data[1] as u16, &data[2..])
}

fn pop_u32(data: &[u8]) -> (u32, &[u8]) {
    (
        ((data[0] as u32) << 24)
            | ((data[1] as u32) << 16)
            | ((data[2] as u32) << 8)
            | data[3] as u32
    , &data[4..])
}
