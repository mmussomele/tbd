use super::packet;

mod router;
mod switch;
mod middlebox;
mod host;
mod link;

// An entity is any type than can process packets.
trait Entity {
    fn process(&self, p: packet::Packet);
    fn mac(&self) -> [u8; 6];
}
