use packet::ParseError;
use packet::pop_u8;
use packet::pop_u16;
use packet::pop_u32;
use packet::pop_u128;

#[derive(Default)]
pub struct Header {
    version: u8, // only lower 4 bits used
    traffic_class: u8,
    flow_label: u32, // only lower 20 bits used
    payload_length: u16,
    next_header: u8,
    hop_limit: u8,

    source: u128,
    dest: u128,
}

#[derive(Default)]
pub struct Parsed<'a> {
    header: Header,
    data: &'a [u8],
}

impl<'a> Header {
    const IPV6_HEADER_SIZE: usize = 40;

    const VERSION_MASK: u32 = 0xf000000;
    const TRAFFIC_MASK: u32 = 0x0ff0000;
    const FLOW_MASK: u32 = 0x000fffff;

    pub fn parse(data: &'a [u8]) -> Result<Parsed<'a>, ParseError> {
        if data.len() < Header::IPV6_HEADER_SIZE {
            return Err(ParseError { reason: String::from("IPv4 packet too small") });
        }

        let (version_traffic_flow, data) = pop_u32(data);
        let version = ((version_traffic_flow & Header::VERSION_MASK) >> 28) as u8;
        let traffic_class = ((version_traffic_flow & Header::TRAFFIC_MASK) >> 20) as u8;
        let flow_label = version_traffic_flow & Header::FLOW_MASK;

        let (payload_length, data) = pop_u16(data);
        let (next_header, data) = pop_u8(data);
        let (hop_limit, data) = pop_u8(data);
        let (source, data) = pop_u128(data);
        let (dest, data) = pop_u128(data);

        Ok(Parsed {
            header: Header {
                version,
                traffic_class,
                flow_label,
                payload_length,
                next_header,
                hop_limit,
                source,
                dest,
            },
            data,
        })
    }
}
