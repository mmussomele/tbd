pub mod application;
pub mod transport;
pub mod network;
pub mod ethernet;

pub enum Packet<'a> {
    App(application::Packet<'a>),
    L4(transport::Packet<'a>),
    L3(network::Packet<'a>),
    L2(ethernet::Packet<'a>),
}
