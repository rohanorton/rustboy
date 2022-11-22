use super::address_space::AddressSpace;
use super::void::Void;

pub struct Mmu {
    spaces: Vec<Box<dyn AddressSpace>>,
    void: Box<dyn AddressSpace>,
}

impl Mmu {
    pub fn new() -> Self {
        Mmu::default()
    }

    pub fn add_address_space<Space: AddressSpace + 'static>(&mut self, address_space: Space) {
        self.spaces.push(Box::new(address_space));
    }

    fn get_space(&mut self, addr: u16) -> &mut Box<dyn AddressSpace> {
        self.spaces
            .iter_mut()
            .find(|space| space.accepts(addr))
            .unwrap_or(&mut self.void)
    }
}

impl Default for Mmu {
    fn default() -> Self {
        Self {
            spaces: Vec::new(),
            void: Box::new(Void {}),
        }
    }
}

impl AddressSpace for Mmu {
    fn accepts(&self, _addr: u16) -> bool {
        true
    }

    fn set_byte(&mut self, addr: u16, byte: u8) {
        self.get_space(addr).set_byte(addr, byte);
    }

    fn get_byte(&mut self, addr: u16) -> u8 {
        self.get_space(addr).get_byte(addr)
    }
}

#[cfg(test)]
mod test {
    use super::Mmu;
    use crate::memory::address_space::AddressSpace;
    use crate::memory::ram::Ram;

    #[test]
    fn returns_null_byte_when_no_address_matches() {
        const ADDRESS: u16 = 0x1234;

        let mut mmu = Mmu::new();

        let result = mmu.get_byte(ADDRESS);

        assert_eq!(result, 0x00);
    }

    #[test]
    fn writes_to_void_when_no_address_matches() {
        const ADDRESS: u16 = 0x1234;

        let mut mmu = Mmu::new();

        // Should not panic.
        mmu.set_byte(ADDRESS, 0x10);

        let byte = mmu.get_byte(ADDRESS);
        assert_eq!(byte, 0x00);
    }

    #[test]
    fn setting_bytes_in_ram_can_be_read_again() {
        const ADDRESS: u16 = 0x1234;

        let mut mmu = Mmu::new();

        let size = 0xFFFF;
        let offset = 0x0000;

        let ram = Ram::new(offset, size);
        mmu.add_address_space(ram);

        mmu.set_byte(ADDRESS, 0x10);
        let byte = mmu.get_byte(ADDRESS);
        assert_eq!(byte, 0x10);

        // Double check to make sure has changed.
        mmu.set_byte(ADDRESS, 0xFF);
        let byte = mmu.get_byte(ADDRESS);
        assert_eq!(byte, 0xFF);
    }
}
