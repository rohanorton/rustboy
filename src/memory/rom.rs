use super::address_space::AddressSpace;

pub struct Rom {
    size: u16,
    offset: u16,
    space: Vec<u8>,
}

impl Rom {
    pub fn new(offset: u16, space: Vec<u8>) -> Self {
        let size = match space.len().try_into() {
            Ok(len) => len,
            Err(_) => panic!("ROM Exceeded Max Size"),
        };
        Self {
            offset,
            size,
            space,
        }
    }

    fn get_index(&self, addr: u16) -> usize {
        assert!(self.accepts(addr), "OutOfBoundsError");
        (addr - self.offset) as usize
    }
}

impl AddressSpace for Rom {
    fn accepts(&self, addr: u16) -> bool {
        addr >= self.offset && addr < (self.offset + self.size)
    }

    fn set_byte(&mut self, _addr: u16, _byte: u8) {
        panic!("PermissionDenied: Attempted to write to read-only memory.")
    }

    fn get_byte(&mut self, addr: u16) -> u8 {
        self.space[self.get_index(addr)]
    }
}
