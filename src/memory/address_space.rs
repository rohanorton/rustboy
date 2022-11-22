pub trait AddressSpace {
    fn accepts(&self, addr: u16) -> bool;
    fn set_byte(&mut self, addr: u16, byte: u8);
    fn get_byte(&mut self, addr: u16) -> u8;
}
