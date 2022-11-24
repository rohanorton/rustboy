use std::fmt;

use super::super::cpu::Cpu;
use super::operation::Operation;

pub struct LdHlSp;

impl Operation for LdHlSp {
    fn run(&self, cpu: &mut Cpu) {
        let r8 = cpu.read_u8() as i8;
        let a = cpu.reg.sp();
        let b = r8 as u16;
        cpu.reg.set_z_flag(false);
        cpu.reg.set_n_flag(false);
        cpu.reg.set_cy_flag((a & 0x00ff) + (b & 0x00ff) > 0x00ff);
        cpu.reg.set_h_flag((a & 0x000f) + (b & 0x000f) > 0x000f);
        cpu.reg.set_hl(a.wrapping_add(b));
    }
}

impl fmt::Display for LdHlSp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LD HL,SP+r8")
    }
}

#[cfg(test)]
mod test {
    use crate::memory::address_space::AddressSpace;
    use crate::memory::ram::Ram;

    use super::Cpu;
    use super::LdHlSp;
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
        let op = LdHlSp;
        assert_eq!(format!("{op}"), "LD HL,SP+r8");
    }

    #[test]
    fn ld_a16_sp_example_from_gameboy_programming_manual() {
        // Stores the lower byte of SP at address nn specified by the 16-bit
        // immediate operand nn and the upper byte of SP at address nn + 1.
        let mut cpu = with_ram(vec![0x00; 0xFFFF]);

        cpu.reg.set_pc(0x0000);
        cpu.mmu.set_byte(0x0000, 0x02);

        // When SP = 0xFFF8,
        cpu.reg.set_sp(0xFFF8);

        // LDHL SP, 2
        LdHlSp.run(&mut cpu);

        // HL←0xFFFA,CY←0,H←0,N←0,Z←0
        let hl = cpu.reg.hl();
        assert_eq!(
            hl, 0xFFFA,
            "HL register should be set to 0xFFFA, but is {hl:#06x}"
        );
        assert!(!cpu.reg.cy_flag(), "Carry flag should be unset");
        assert!(!cpu.reg.h_flag(), "Half-Carry flag should be unset");
        assert!(!cpu.reg.z_flag(), "Zero flag should be unset");
        assert!(!cpu.reg.n_flag(), "Subtract flag should be unset");
    }
}
