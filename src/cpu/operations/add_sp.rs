use std::fmt;

use super::super::cpu::Cpu;
use super::operation::Operation;

pub struct AddSp {
    cycles: u8,
}

impl AddSp {
    pub fn new(cycles: u8) -> Self {
        AddSp { cycles }
    }
}

impl Operation for AddSp {
    fn execute(&self, cpu: &mut Cpu) -> u8 {
        let r8 = cpu.read_u8() as i8;
        let a = cpu.registers.sp();
        let b = r8 as u16;
        cpu.registers.set_z_flag(false);
        cpu.registers.set_n_flag(false);
        cpu.registers
            .set_cy_flag((a & 0x00ff) + (b & 0x00ff) > 0x00ff);
        cpu.registers
            .set_h_flag((a & 0x000f) + (b & 0x000f) > 0x000f);
        cpu.registers.set_sp(a.wrapping_add(b));

        self.cycles
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
    use crate::memory::void::Void;

    use super::AddSp;
    use super::Cpu;
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

    const CYCLE_COUNT: u8 = 4;

    #[test]
    fn returns_cycle_count() {
        let mut cpu = empty();
        let op = AddSp::new(CYCLE_COUNT);
        let res = op.execute(&mut cpu);
        assert_eq!(
            res, CYCLE_COUNT,
            "Returned value should match cycle count passed to constructor"
        );
    }

    #[test]
    fn adds_byte_to_sp() {
        let mut cpu = with_ram(vec![0x41]);
        cpu.registers.set_pc(0x0000); // ensure first byte read
        cpu.registers.set_sp(0x010F);
        let op = AddSp::new(CYCLE_COUNT);
        op.execute(&mut cpu);
        assert_eq!(cpu.registers.sp(), 0x0150);
    }

    #[test]
    fn subtracts_byte_from_sp_when_negative() {
        // -2 in Two's Complement
        let neg_two = 0xFE;
        let mut cpu = with_ram(vec![neg_two]);
        cpu.registers.set_pc(0x0000); // ensure first byte read
        cpu.registers.set_sp(0x010F);
        let op = AddSp::new(CYCLE_COUNT);
        op.execute(&mut cpu);
        assert_eq!(cpu.registers.sp(), 0x010D);
    }

    #[test]
    fn display_trait() {
        let op = AddSp::new(CYCLE_COUNT);
        assert_eq!(format!("{op}"), "ADD SP,r8");
    }

    #[test]
    fn example_from_gameboy_programming_manual() {
        // r8 = 2
        let mut cpu = with_ram(vec![0x02]);
        cpu.registers.set_pc(0x0000); // ensure first byte read

        // SP = FFF8h
        cpu.registers.set_sp(0xFFF8);

        // ADDSP,2
        AddSp::new(CYCLE_COUNT).execute(&mut cpu);

        // SP←0xFFFA,CY←0,H←0,N←0,Z←0
        assert_eq!(cpu.registers.sp(), 0xFFFA);
        assert!(!cpu.registers.z_flag());
        assert!(!cpu.registers.h_flag());
        assert!(!cpu.registers.n_flag());
        assert!(!cpu.registers.cy_flag());
    }
}