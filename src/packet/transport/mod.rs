mod tcp;
mod udp;

pub enum Header<'a> {
    TCP(tcp::Header<'a>),
    UDP(udp::Header),
}
