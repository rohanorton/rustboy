use super::address_space::AddressSpace;
use super::boot_rom::create_boot_rom;
use super::rom::Rom;
use super::void::Void;

use std::fs::File;
use std::io::Read;

pub struct Cartridge {
    spaces: Vec<Box<dyn AddressSpace>>,
    void: Box<dyn AddressSpace>,
}

impl Cartridge {
    pub fn load(filename: &str) -> std::io::Result<Self> {
        let file_rom = Box::new(Self::load_file(filename)?) as Box<dyn AddressSpace>;
        let boot_rom = Box::new(create_boot_rom()) as Box<dyn AddressSpace>;

        let spaces = vec![boot_rom, file_rom];
        Ok(Cartridge {
            spaces,
            void: Box::new(Void {}),
        })
    }

    fn load_file(filename: &str) -> std::io::Result<Rom> {
        log::debug!("Reading ROM File {}", filename);

        let mut file = File::open(filename)?;
        let mut contents = vec![];

        file.read_to_end(&mut contents)?;

        log::debug!("Loaded {} bytes", contents.len());

        Ok(Rom::new(0x0100, contents))
    }

    fn get_space(&mut self, addr: u16) -> &mut Box<dyn AddressSpace> {
        self.spaces
            .iter_mut()
            .find(|space| space.accepts(addr))
            .unwrap_or(&mut self.void)
    }
}

impl AddressSpace for Cartridge {
    fn accepts(&self, addr: u16) -> bool {
        self.spaces.iter().any(|space| space.accepts(addr))
    }

    fn set_byte(&mut self, addr: u16, byte: u8) {
        self.get_space(addr).set_byte(addr, byte);
    }

    fn get_byte(&mut self, addr: u16) -> u8 {
        self.get_space(addr).get_byte(addr)
    }
}
