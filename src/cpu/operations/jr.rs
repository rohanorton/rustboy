use std::fmt;

use crate::cpu::operations::Operation;
use crate::cpu::Cpu;

// Jump Relative to PC
pub struct Jr;

impl Operation for Jr {
    fn run(&self, cpu: &mut Cpu) {
        let r8 = cpu.read_u8() as i8;
        let pc = cpu.reg.pc();
        let addr = pc.wrapping_add(r8 as u16);
        cpu.reg.set_pc(addr);
    }
}

impl fmt::Display for Jr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "JR r8")
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
        let op = Jr;
        assert_eq!(format!("{op}"), "JR r8");
    }

    #[test]
    fn changes_pc_counter_by_relative_amount() {
        let mut cpu = with_ram(vec![0x00; 0xFFFF]);

        cpu.reg.set_pc(0x00FF);
        // Set current byte to -2 in two's complement
        cpu.mmu.set_byte(0x00FF, 0xFE);

        Jr.run(&mut cpu);

        assert_eq!(
            cpu.reg.pc(),
            0x00FE,
            "Should have decremented counter by 2."
        );
    }
}
