use packet::ParseError;
use packet::pop_u16;

#[derive(Default)]
pub struct Header {
    source: u16,
    dest: u16,
    length: u16,
    checksum: u16,
}

#[derive(Default)]
pub struct Parsed<'a> {
    header: Header,
    data: &'a [u8],
}

impl<'a> Header {
    const UDP_HEADER_SIZE: usize = 8;

    pub fn parse(data: &'a [u8]) -> Result<Parsed<'a>, ParseError> {
        if data.len() < Header::UDP_HEADER_SIZE {
            return Err(ParseError { reason: String::from("UDP packet too small") });
        }

        let mut parsed = Parsed { ..Default::default() };
        let (source, data) = pop_u16(data);
        let (dest, data) = pop_u16(data);
        let (length, data) = pop_u16(data);
        let (checksum, data) = pop_u16(data);

        // XXX: Verify checksum?

        Ok(Parsed {
            header: Header {
                source,
                dest,
                length,
                checksum,
            },
            data: data,
        })
    }
}
