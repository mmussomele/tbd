pub mod ipv4;

pub enum Packet<'a> {
    IPV4(ipv4::Packet<'a>),
    // TODO: IPV6
}
