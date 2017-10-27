use packet::ParseError;
use packet::pop_u8;
use packet::pop_u16;
use packet::pop_u32;

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

#[derive(Default)]
pub struct Parsed<'a> {
    header: Header<'a>,
    data: &'a [u8],
}

impl<'a> Header<'a> {
    const IPV4_HEADER_MIN_SIZE: usize = 20;

    const VERSION_MASK: u8 = 0b11110000;
    const IHL_MASK: u8 = !Header::VERSION_MASK;
    const DSCP_MASK: u8 = 0b11111100;
    const ECN_MASK: u8 = !Header::DSCP_MASK;

    const DF_MASK: u16 = 0b0100000000000000;
    const MF_MASK: u16 = 0b0010000000000000;
    const OFFSET_MASK: u16 = 0b0001111111111111;

    pub fn parse(data: &'a [u8]) -> Result<Parsed<'a>, ParseError> {
        if data.len() < Header::IPV4_HEADER_MIN_SIZE {
            return Err(ParseError { reason: String::from("IPv4 packet too small") });
        }

        let (version_and_ihl, data) = pop_u8(data);
        let version = (version_and_ihl & Header::VERSION_MASK) >> 4;
        let ihl = version_and_ihl & Header::IHL_MASK;

        let (dscp_and_ecn, data) = pop_u8(data);
        let dscp = (dscp_and_ecn & Header::DSCP_MASK) >> 2;
        let ecn = dscp_and_ecn & Header::ECN_MASK;

        let (total_length, data) = pop_u16(data);
        let (identification, data) = pop_u16(data);

        let (flags_and_offset, data) = pop_u16(data);
        let df = (flags_and_offset & Header::DF_MASK) > 0;
        let mf = (flags_and_offset & Header::MF_MASK) > 0;
        let fragment_offset = flags_and_offset & Header::OFFSET_MASK;
        let (ttl, data) = pop_u8(data);
        let (protocol, data) = pop_u8(data);
        let (checksum, data) = pop_u16(data);
        let (source, data) = pop_u32(data);
        let (dest, data) = pop_u32(data);

        let header_end = 4 * ((ihl as usize) - 5);
        if data.len() < header_end {
            return Err(ParseError { reason: String::from("IPv4 packet too small") });
        }

        let options = &data[..header_end];
        let data = &data[header_end..];

        Ok(Parsed {
            header: Header {
                version,
                ihl,
                dscp,
                ecn,
                total_length,
                identification,
                df,
                mf,
                fragment_offset,
                ttl,
                protocol,
                checksum,
                source,
                dest,
                options,
            },
            data,
        })
    }
}
