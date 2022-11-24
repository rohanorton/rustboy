use std::fmt;

use super::super::cpu::Cpu;
use super::operation::Operation;
use super::targets::Ld16Target;

pub struct Ld16 {
    dest: Ld16Target,
    src: Ld16Target,
}

impl Ld16 {
    pub fn new(dest: Ld16Target, src: Ld16Target) -> Self {
        Ld16 { dest, src }
    }
}

impl Operation for Ld16 {
    fn run(&self, cpu: &mut Cpu) {
        let value = self.src.value(cpu);
        self.dest.set_value(cpu, value);
    }
}

impl fmt::Display for Ld16 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LD {},{}", self.dest, self.src)
    }
}

#[cfg(test)]
mod test {
    use crate::memory::address_space::AddressSpace;
    use crate::memory::ram::Ram;
    use crate::memory::void::Void;

    use super::Cpu;
    use super::Ld16;
    use super::Ld16Target;
    use super::Operation;

    fn empty() -> Cpu {
        Cpu::new(Void)
    }

    fn with_ram(data: Vec<u8>) -> Cpu {
        let mut ram = Ram::new(0, data.len() as u16);
        for (i, n) in data.iter().enumerate() {
            ram.set_byte(i as u16, *n);
        }
        Cpu::new(ram)
    }

    #[test]
    fn loads_value_from_one_register_into_another() {
        let mut cpu = empty();
        cpu.reg.set_bc(0x0000);
        cpu.reg.set_hl(0x1234);
        Ld16::new(Ld16Target::BC, Ld16Target::HL).run(&mut cpu);
        assert_eq!(cpu.reg.hl(), 0x1234);
    }

    #[test]
    fn display_trait() {
        let op = Ld16::new(Ld16Target::BC, Ld16Target::D16);
        assert_eq!(format!("{op}"), "LD BC,d16");
    }

    #[test]
    fn ld_a16_sp_example_from_gameboy_programming_manual() {
        // Stores the lower byte of SP at address nn specified by the 16-bit
        // immediate operand nn and the upper byte of SP at address nn + 1.
        let mut cpu = with_ram(vec![0x00; 0xFFFF]);

        cpu.reg.set_pc(0x0000);
        cpu.mmu.set_byte(0x0000, 0x00);
        cpu.mmu.set_byte(0x0001, 0xC1);

        // When SP = FFF8h,
        cpu.reg.set_sp(0xFFF8);

        // LD (C100h),SP
        Ld16::new(Ld16Target::A16, Ld16Target::SP).run(&mut cpu);

        // C100h ← F8h
        assert_eq!(cpu.mmu.get_byte(0xC100), 0xF8);
        // C101h ← FFh
        assert_eq!(cpu.mmu.get_byte(0xC101), 0xFF);
    }
}
