pub enum Packet<'a> {
    Data(&'a [u8]),
}
