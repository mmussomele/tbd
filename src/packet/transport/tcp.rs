use packet::ParseError;
use packet::pop_u16;
use packet::pop_u32;

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

#[derive(Default)]
pub struct Parsed<'a> {
    header: Header<'a>,
    data: &'a [u8],
}

impl<'a> Header<'a> {
    const TCP_HEADER_MIN_SIZE: usize = 20;

    const NS_MASK: u16 = 0b100000000;
    const CWR_MASK: u16 = 0b010000000;
    const ECE_MASK: u16 = 0b001000000;
    const URG_MASK: u16 = 0b000100000;
    const ACK_MASK: u16 = 0b000010000;
    const PSH_MASK: u16 = 0b000001000;
    const RST_MASK: u16 = 0b000000100;
    const SYN_MASK: u16 = 0b000000010;
    const FIN_MASK: u16 = 0b000000001;

    pub fn parse(data: &'a [u8]) -> Result<Parsed<'a>, ParseError> {
        if data.len() < Header::TCP_HEADER_MIN_SIZE {
            return Err(ParseError { reason: String::from("TCP packet too small") });
        }

        let mut parsed = Parsed { ..Default::default() };
        let (source, data) = pop_u16(data);
        let (dest, data) = pop_u16(data);
        let (seq, data) = pop_u32(data);
        let (ack_no, data) = pop_u32(data);
        let (offset_and_flags, data) = pop_u16(data);
        let (window_size, data) = pop_u16(data);
        let (checksum, data) = pop_u16(data);
        let (urgent_ptr, data) = pop_u16(data);
        let data_offset = (offset_and_flags >> 12) as u8;

        // offset is the length of the header in words.
        // The first 5 words have already been popped, so the end of the header is 4*(offset-5).
        let header_end = 4 * ((data_offset as usize) - 5);

        if data.len() < header_end {
            return Err(ParseError { reason: String::from("TCP packet too small") });
        }

        let options = &data[..header_end];
        let data = &data[header_end..];

        let flags = offset_and_flags & 0b111111111;
        let ns = (flags & Header::NS_MASK) > 0;
        let cwr = (flags & Header::CWR_MASK) > 0;
        let ece = (flags & Header::ECE_MASK) > 0;
        let urg = (flags & Header::URG_MASK) > 0;
        let ack = (flags & Header::ACK_MASK) > 0;
        let psh = (flags & Header::PSH_MASK) > 0;
        let rst = (flags & Header::RST_MASK) > 0;
        let syn = (flags & Header::SYN_MASK) > 0;
        let fin = (flags & Header::FIN_MASK) > 0;

        // XXX: Verify checksum?

        Ok(Parsed {
            header: Header {
                source,
                dest,
                seq,
                ack_no,
                data_offset,
                ns,
                cwr,
                ece,
                urg,
                ack,
                psh,
                rst,
                syn,
                fin,
                window_size,
                checksum,
                urgent_ptr,
                options,
            },
            data: data,
        })
    }
}
