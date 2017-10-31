use super::Entity;
use super::super::packet;

pub struct Link<'a> {
    pub dest: &'a Entity,
}

impl<'a> Link<'a> {
    pub fn transmit(&self, p: packet::Packet) {
        self.dest.process(p);
    }
}
