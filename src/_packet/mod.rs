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

fn pop_u8(data: &[u8]) -> (u8, &[u8]) {
    (data[0], &data[1..])
}

fn pop_u16(data: &[u8]) -> (u16, &[u8]) {
    (((data[0] as u16) << 8) | data[1] as u16, &data[2..])
}

fn pop_u32(data: &[u8]) -> (u32, &[u8]) {
    let (a, data) = pop_u16(data);
    let (b, data) = pop_u16(data);

    (((a as u32) << 16) | b as u32, data)
}

fn pop_u64(data: &[u8]) -> (u64, &[u8]) {
    let (a, data) = pop_u32(data);
    let (b, data) = pop_u32(data);

    (((a as u64) << 32) | b as u64, data)
}

fn pop_u128(data: &[u8]) -> (u128, &[u8]) {
    let (a, data) = pop_u64(data);
    let (b, data) = pop_u64(data);

    (((a as u128) << 64) | b as u128, data)
}
