use std::fmt;

use crate::cpu::operations::Operation;
use crate::cpu::Cpu;

pub struct AddSp;

impl Operation for AddSp {
    fn run(&self, cpu: &mut Cpu) {
        let r8 = cpu.read_u8() as i8;
        let a = cpu.reg.sp();
        let b = r8 as u16;
        cpu.reg.set_z_flag(false);
        cpu.reg.set_n_flag(false);
        cpu.reg.set_cy_flag((a & 0x00ff) + (b & 0x00ff) > 0x00ff);
        cpu.reg.set_h_flag((a & 0x000f) + (b & 0x000f) > 0x000f);
        cpu.reg.set_sp(a.wrapping_add(b));
    }
}

impl fmt::Display for AddSp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ADD SP,r8")
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
    fn adds_byte_to_sp() {
        let mut cpu = with_ram(vec![0x41]);
        cpu.reg.set_pc(0x0000); // ensure first byte read
        cpu.reg.set_sp(0x010F);
        AddSp.run(&mut cpu);
        assert_eq!(cpu.reg.sp(), 0x0150);
    }

    #[test]
    fn subtracts_byte_from_sp_when_negative() {
        // -2 in Two's Complement
        let neg_two = 0xFE;
        let mut cpu = with_ram(vec![neg_two]);
        cpu.reg.set_pc(0x0000); // ensure first byte read
        cpu.reg.set_sp(0x010F);
        AddSp.run(&mut cpu);
        assert_eq!(cpu.reg.sp(), 0x010D);
    }

    #[test]
    fn display_trait() {
        let op = AddSp;
        assert_eq!(format!("{op}"), "ADD SP,r8");
    }

    #[test]
    fn example_from_gameboy_programming_manual() {
        // r8 = 2
        let mut cpu = with_ram(vec![0x02]);
        cpu.reg.set_pc(0x0000); // ensure first byte read

        // SP = FFF8h
        cpu.reg.set_sp(0xFFF8);

        // ADDSP,2
        AddSp.run(&mut cpu);

        // SP←0xFFFA,CY←0,H←0,N←0,Z←0
        assert_eq!(cpu.reg.sp(), 0xFFFA);
        assert!(!cpu.reg.z_flag());
        assert!(!cpu.reg.h_flag());
        assert!(!cpu.reg.n_flag());
        assert!(!cpu.reg.cy_flag());
    }
}
