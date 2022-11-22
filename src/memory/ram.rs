use super::address_space::AddressSpace;

pub struct Ram {
    size: u16,
    offset: u16,
    space: Vec<u8>,
}

impl Ram {
    pub fn new(offset: u16, size: u16) -> Self {
        Self {
            offset,
            size,
            space: vec![0x00; size.into()],
        }
    }

    fn get_index(&self, addr: u16) -> usize {
        assert!(self.accepts(addr), "OutOfBoundsError");
        (addr - self.offset) as usize
    }
}

impl AddressSpace for Ram {
    fn accepts(&self, addr: u16) -> bool {
        addr >= self.offset && addr < (self.offset + self.size)
    }

    fn set_byte(&mut self, addr: u16, byte: u8) {
        let index = self.get_index(addr);
        self.space[index] = byte;
    }

    fn get_byte(&mut self, addr: u16) -> u8 {
        self.space[self.get_index(addr)]
    }
}

#[cfg(test)]
mod test {
    use super::Ram;
    use crate::memory::address_space::AddressSpace;

    static ADDRESS: u16 = 0x1234;
    const OUT_OF_BOUNDS_ADDRESS: u16 = 0xFFFF;
    static SIZE: u16 = 0xAAAA;
    static NO_OFFSET: u16 = 0x0000;

    #[test]
    fn accepts_returns_true_if_in_bounds() {
        let ram = Ram::new(NO_OFFSET, SIZE);
        let accepts = ram.accepts(ADDRESS);
        assert!(
            accepts,
            ".accepts() should return true for in-bounds lookup"
        );
    }

    #[test]
    fn accepts_returns_true_if_in_bounds_lowest_byte_included() {
        let ram = Ram::new(NO_OFFSET, SIZE);
        let accepts = ram.accepts(0);
        assert!(
            accepts,
            ".accepts() should return true for in-bounds lookup"
        );
    }

    #[test]
    fn accepts_returns_true_if_in_bounds_highest_byte_not_included() {
        let ram = Ram::new(NO_OFFSET, SIZE);
        let accepts = ram.accepts(SIZE);
        assert!(
            !accepts,
            ".accepts() should return true for in-bounds lookup"
        );
    }

    #[test]
    fn accepts_returns_false_if_in_bounds() {
        let ram = Ram::new(NO_OFFSET, SIZE);
        let accepts = ram.accepts(OUT_OF_BOUNDS_ADDRESS);
        assert!(
            !accepts,
            ".accepts() should return false for out-of-bounds lookup"
        );
    }

    #[test]
    fn returns_byte() {
        let mut ram = Ram::new(NO_OFFSET, SIZE);

        let result = ram.get_byte(ADDRESS);

        assert_eq!(result, 0x00);
    }

    #[test]
    fn writes_to_ram() {
        let mut ram = Ram::new(NO_OFFSET, SIZE);

        ram.set_byte(ADDRESS, 0x10);

        let byte = ram.get_byte(ADDRESS);
        assert_eq!(byte, 0x10);
    }

    #[test]
    #[should_panic(expected = "OutOfBoundsError")]
    fn panics_on_out_of_bounds_get() {
        let mut ram = Ram::new(NO_OFFSET, SIZE);
        ram.get_byte(OUT_OF_BOUNDS_ADDRESS);
    }

    #[test]
    #[should_panic(expected = "OutOfBoundsError")]
    fn panics_on_out_of_bounds_set() {
        let mut ram = Ram::new(NO_OFFSET, SIZE);
        ram.set_byte(OUT_OF_BOUNDS_ADDRESS, 0x10);
    }
}
