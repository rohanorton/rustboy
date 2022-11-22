use super::address_space::AddressSpace;
use log;

pub struct Void;

impl AddressSpace for Void {
    fn accepts(&self, _addr: u16) -> bool {
        true
    }

    fn set_byte(&mut self, addr: u16, byte: u8) {
        log::debug!("Writing value {:#04x} to void address {:#06x}", byte, addr);
    }

    fn get_byte(&mut self, addr: u16) -> u8 {
        log::debug!("Reading void address {:#06x}", addr);
        0x00
    }
}
