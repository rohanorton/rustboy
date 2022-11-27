use std::fmt;

use super::condition::Condition;
use super::jp::Jp;
use super::targets::AddressTarget;
use crate::cpu::operations::Operation;
use crate::cpu::Cpu;

pub struct ConditionalJp {
    cond: Condition,
    operand: AddressTarget,
}

impl ConditionalJp {
    pub fn new(cond: Condition, operand: AddressTarget) -> Self {
        ConditionalJp { cond, operand }
    }
}

impl Operation for ConditionalJp {
    fn run(&self, cpu: &mut Cpu) {
        if self.cond.check(cpu) {
            Jp::new(self.operand).run(cpu);
        } else {
            // Read from operand, potentially increments program counter
            self.operand.value(cpu);
            // Don't have a better way of doing this at this time.
            cpu.remaining_cycles = cpu.remaining_cycles.saturating_sub(4);
        }
    }
}

impl fmt::Display for ConditionalJp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "JP {},{}", self.cond, self.operand)
    }
}

#[cfg(test)]
mod test {
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
        let op = ConditionalJp::new(Condition::C, AddressTarget::A16);
        assert_eq!(format!("{op}"), "JP C,a16");
    }

    #[test]
    fn example_1_from_gameboy_programming_manual() {
        let mut cpu = with_ram(vec![0x00; 0xFFFF]);

        // JP NZ 8000h
        cpu.reg.set_pc(0x0000);
        cpu.mmu.set_byte(0x0000, 0x00);
        cpu.mmu.set_byte(0x0001, 0x80);

        cpu.reg.set_z_flag(true);

        ConditionalJp::new(Condition::NZ, AddressTarget::A16).run(&mut cpu);

        // Increments PC by 2
        assert_eq!(cpu.reg.pc(), 0x0002);
    }

    #[test]
    fn example_2_from_gameboy_programming_manual() {
        let mut cpu = with_ram(vec![0x00; 0xFFFF]);

        // JP Z 8000h
        cpu.reg.set_pc(0x0000);
        cpu.mmu.set_byte(0x0000, 0x00);
        cpu.mmu.set_byte(0x0001, 0x80);

        cpu.reg.set_z_flag(true);

        ConditionalJp::new(Condition::Z, AddressTarget::A16).run(&mut cpu);

        assert_eq!(cpu.reg.pc(), 0x8000);
    }
}
