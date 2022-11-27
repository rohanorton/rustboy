use std::fmt;

use super::condition::Condition;
use super::jr::Jr;
use crate::cpu::operations::Operation;
use crate::cpu::Cpu;

pub struct ConditionalJr {
    cond: Condition,
}

impl ConditionalJr {
    pub fn new(cond: Condition) -> Self {
        ConditionalJr { cond }
    }
}

impl Operation for ConditionalJr {
    fn run(&self, cpu: &mut Cpu) {
        if self.cond.check(cpu) {
            Jr.run(cpu);
        } else {
            // Read byte to increment program counter
            cpu.read_u8();
            // Remove cycles
            cpu.remaining_cycles = cpu.remaining_cycles.saturating_sub(4);
        }
    }
}

impl fmt::Display for ConditionalJr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "JR {},r8", self.cond)
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
        let op = ConditionalJr::new(Condition::C);
        assert_eq!(format!("{op}"), "JR C,r8");
    }

    #[test]
    fn changes_pc_counter_by_relative_amount_if_condition_met() {
        let mut cpu = with_ram(vec![0x00; 0xFFFF]);

        cpu.reg.set_pc(0x00FF);
        // Set current byte to -2 in two's complement
        cpu.mmu.set_byte(0x00FF, 0xFE);

        cpu.reg.set_z_flag(true);

        ConditionalJr::new(Condition::Z).run(&mut cpu);

        assert_eq!(
            cpu.reg.pc(),
            0x00FE,
            "Should have decremented counter by 2."
        );
    }

    #[test]
    fn changes_pc_counter_by_1_if_condition_not_met() {
        let mut cpu = with_ram(vec![0x00; 0xFFFF]);

        cpu.reg.set_pc(0x00FF);
        // Set current byte to -2 in two's complement
        cpu.mmu.set_byte(0x00FF, 0xFE);

        cpu.reg.set_z_flag(true);

        ConditionalJr::new(Condition::NZ).run(&mut cpu);

        assert_eq!(cpu.reg.pc(), 0x0100);
    }
}
