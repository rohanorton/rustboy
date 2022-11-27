use crate::cpu::Cpu;
use crate::memory::cartridge::Cartridge;
use crate::memory::mmu::Mmu;
use crate::memory::ram::Ram;

static FOUR_KB: u16 = 0x1000;
static EIGHT_KB: u16 = 0x2000;

pub struct GameBoy {
    cpu: Cpu,
}

impl GameBoy {
    pub fn load_cartridge(filename: &str) -> std::io::Result<Self> {
        let game_rom = Cartridge::load(filename)?;

        let mut mmu = Mmu::new();

        // 8000-9FFF: 8 KiB Video RAM (VRAM)
        mmu.add_address_space(Ram::new(0x8000, EIGHT_KB));

        // A000-BFFF: 8 KiB External RAM
        mmu.add_address_space(Ram::new(0xA000, EIGHT_KB));

        // C000-CFFF: 4 KiB Work RAM (WRAM)
        mmu.add_address_space(Ram::new(0xC000, FOUR_KB));

        // D000-DFFF: 4 KiB Work RAM (WRAM)
        mmu.add_address_space(Ram::new(0xD000, FOUR_KB));

        // E000-FDFF: Mirror of C000~DDFF (ECHO RAM)

        // FE00-FE9F: Sprite attribute table (OAM)
        mmu.add_address_space(Ram::new(0xFE00, 0x00A0));

        // FEA0-FEFF: Not Usable

        // FF00-FF7F: I/O Registers
        mmu.add_address_space(Ram::new(0xFF00, 0x007F));

        // FF80-FFFE: High RAM (HRAM)
        mmu.add_address_space(Ram::new(0xFF80, 0x007F));

        // FFFF-FFFF: Interrupt Enable register (IE)

        // 0000-3FFF: 16 KiB ROM bank 00
        // 4000-7FFF: 16 KiB ROM Bank 01~NN
        mmu.add_address_space(game_rom);

        let cpu = Cpu::new(mmu);

        Ok(GameBoy { cpu })
    }

    pub fn run(&mut self) {
        self.cpu.run();
    }
}
