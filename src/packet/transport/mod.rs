mod tcp;
mod udp;

pub enum Packet<'a> {
    TCP(tcp::Packet<'a>),
    UDP(udp::Packet<'a>),
}
