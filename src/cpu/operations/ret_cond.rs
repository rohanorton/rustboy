use std::fmt;

use super::super::cpu::Cpu;
use super::condition::Condition;
use super::operation::Operation;
use super::ret::Ret;

pub struct ConditionalRet {
    cond: Condition,
}

impl ConditionalRet {
    pub fn new(cond: Condition) -> Self {
        ConditionalRet { cond }
    }
}

impl Operation for ConditionalRet {
    fn run(&self, cpu: &mut Cpu) {
        if self.cond.check(cpu) {
            Ret.run(cpu);
        } else {
            // Don't have a better way of doing this at this time.
            cpu.remaining_cycles = cpu.remaining_cycles.saturating_sub(12);
        }
    }
}

impl fmt::Display for ConditionalRet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RET {}", self.cond)
    }
}

#[cfg(test)]
mod test {
    use crate::memory::address_space::AddressSpace;
    use crate::memory::ram::Ram;

    use super::Condition;
    use super::ConditionalRet;
    use super::Cpu;
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
        let op = ConditionalRet::new(Condition::C);
        assert_eq!(format!("{op}"), "RET C");
    }

    #[test]
    fn example_1_from_gameboy_programming_manual() {
        let mut cpu = with_ram(vec![0x00; 0xffff]);

        cpu.reg.set_z_flag(true);

        cpu.reg.set_pc(0x5000);
        cpu.reg.set_sp(0xfffc);
        cpu.mmu.set_byte(0xfffc, 0x03);
        cpu.mmu.set_byte(0xfffd, 0x80);

        // Condition met.
        ConditionalRet::new(Condition::Z).run(&mut cpu);

        assert_eq!(cpu.reg.pc(), 0x8003, "pc should return to previous address");
    }

    #[test]
    fn example_2_from_gameboy_programming_manual() {
        let mut cpu = with_ram(vec![0x00; 0xffff]);

        cpu.reg.set_z_flag(false);

        cpu.reg.set_pc(0x5000);
        cpu.reg.set_sp(0xfffc);
        cpu.mmu.set_byte(0xfffc, 0x03);
        cpu.mmu.set_byte(0xfffd, 0x80);

        // Condition not met.
        ConditionalRet::new(Condition::Z).run(&mut cpu);

        assert_eq!(cpu.reg.pc(), 0x5000, "PC unchanged");
    }
}
