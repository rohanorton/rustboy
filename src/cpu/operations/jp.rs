use std::fmt;

use super::super::cpu::Cpu;
use super::operation::Operation;
use super::targets::AddressTarget;

pub struct Jp {
    operand: AddressTarget,
}

impl Jp {
    pub fn new(operand: AddressTarget) -> Self {
        Jp { operand }
    }
}

impl Operation for Jp {
    fn run(&self, cpu: &mut Cpu) {
        let addr = self.operand.value(cpu);
        cpu.reg.set_pc(addr);
    }
}

impl fmt::Display for Jp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "JP {}", self.operand)
    }
}

#[cfg(test)]
mod test {
    use crate::memory::address_space::AddressSpace;
    use crate::memory::ram::Ram;

    use super::AddressTarget;
    use super::Cpu;
    use super::Jp;
    use super::Operation;

    fn with_ram(data: Vec<u8>) -> Cpu {
        let mut ram = Ram::new(0, data.len() as u16);
        for (i, n) in data.iter().enumerate() {
            ram.set_byte(i as u16, *n);
        }
        Cpu::new(ram)
    }

    #[test]
    fn display_trait() {
        let op = Jp::new(AddressTarget::A16);
        assert_eq!(format!("{op}"), "JP a16");
    }

    #[test]
    fn example_from_gameboy_programming_manual() {
        let mut cpu = with_ram(vec![0x00; 0xFFFF]);

        // JP 8000h
        cpu.reg.set_pc(0x0000);
        cpu.mmu.set_byte(0x0000, 0x00);
        cpu.mmu.set_byte(0x0001, 0x80);

        Jp::new(AddressTarget::A16).run(&mut cpu);

        // Jump to 8000h.
        assert_eq!(cpu.reg.pc(), 0x8000);
    }
}
