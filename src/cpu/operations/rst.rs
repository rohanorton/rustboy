use std::fmt;

use crate::cpu::operations::Operation;
use crate::cpu::Cpu;

pub struct Rst {
    dest: u8,
}

impl Rst {
    pub fn new(dest: u8) -> Self {
        Self { dest }
    }
}
impl Operation for Rst {
    fn run(&self, cpu: &mut Cpu) {
        let sp = cpu.reg.sp();
        let l = self.dest as u16;
        let h = 0;
        cpu.reg.set_sp(sp + 2);
        cpu.reg.set_pc(l | h << 8);
    }
}

impl fmt::Display for Rst {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RST {:02x}H", self.dest)
    }
}

#[cfg(test)]
mod test {
    use crate::cpu::operations::call::Call;
    use crate::memory::address_space::AddressSpace;
    use crate::memory::ram::Ram;

    use super::*;

    fn with_ram(data: Vec<u8>) -> Cpu {
        let mut ram = Ram::new(0, data.len() as u16);
        for (i, n) in data.iter().enumerate() {
            ram.set_byte(i as u16, *n);
        }
        Cpu::new(ram)
    }

    #[test]
    fn display_trait() {
        let op = Rst::new(0x18);
        assert_eq!(format!("{op}"), "RST 18H");
    }

    #[test]
    fn display_trait_includes_leading_zero() {
        let op = Rst::new(0x08);
        assert_eq!(format!("{op}"), "RST 08H");
    }

    #[test]
    fn example_from_gameboy_programming_manual() {
        let mut cpu = with_ram(vec![0x00; 0xFFFF]);

        // Examples: When PC = 8000h and SP = FFFEh
        cpu.reg.set_pc(0x8000);

        // Increment PC by 1 (We read a byte in order get op_code)
        cpu.reg.incr_pc();

        cpu.reg.set_sp(0xFFFE);
        cpu.mmu.set_byte(0x8001, 0x34);
        cpu.mmu.set_byte(0x8002, 0x12);

        Call.run(&mut cpu);

        Rst::new(0x08).run(&mut cpu);

        assert_eq!(
            cpu.reg.pc(),
            0x0008,
            "PC should returns to specified address"
        );
    }
}
