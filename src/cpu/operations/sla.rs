use std::fmt;

use super::super::cpu::Cpu;
use super::operation::Operation;
use super::targets::ArithmeticTarget8Bit;

// Shifts the contents of operand to the left.
pub struct Sla {
    operand: ArithmeticTarget8Bit,
}

impl Sla {
    pub fn new(operand: ArithmeticTarget8Bit) -> Self {
        Sla { operand }
    }
}

impl Operation for Sla {
    fn execute(&self, cpu: &mut Cpu) {
        let x = self.operand.value(cpu);
        let shift_x = x << 1;
        self.operand.set_value(cpu, shift_x);
        cpu.registers.set_cy_flag(x >> 7 != 0);
        cpu.registers.set_z_flag(shift_x == 0);
        cpu.registers.set_h_flag(false);
        cpu.registers.set_n_flag(false);
    }
}

impl fmt::Display for Sla {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SLA {}", self.operand)
    }
}

#[cfg(test)]
mod test {
    use crate::memory::address_space::AddressSpace;
    use crate::memory::ram::Ram;
    use crate::memory::void::Void;

    use super::ArithmeticTarget8Bit;
    use super::Cpu;
    use super::Operation;
    use super::Sla;

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
    fn display_trait() {
        let op = Sla::new(ArithmeticTarget8Bit::C);
        assert_eq!(format!("{op}"), "SLA C");
    }

    #[test]
    fn example_1_from_gameboy_programming_manual() {
        let mut cpu = empty();

        // When D = 80h, and CY = 0,
        cpu.registers.set_d(0x80);
        cpu.registers.set_cy_flag(false);

        // SLA D
        Sla::new(ArithmeticTarget8Bit::D).execute(&mut cpu);

        // D←00h,CY←1,Z←1,H←0,N←0
        assert_eq!(cpu.registers.d(), 0x00);
        assert!(cpu.registers.cy_flag(), "Carry flag should be set");
        assert!(cpu.registers.z_flag(), "Zero flag should be set");
        assert!(!cpu.registers.h_flag(), "Half-Carry flag should not be set");
        assert!(!cpu.registers.n_flag(), "Subtract flag should not be set");
    }

    #[test]
    fn example_2_from_gameboy_programming_manual() {
        let mut cpu = with_ram(vec![0x00; 0xFFFF]);

        // When (HL) = FFh, and CY = 0,
        cpu.mmu.set_byte(cpu.registers.hl(), 0xFF);
        cpu.registers.set_cy_flag(false);

        // SLA (HL)
        Sla::new(ArithmeticTarget8Bit::HLAddr).execute(&mut cpu);

        // (HL)←FEh,CY←1,Z←0,H←0,N←0
        assert_eq!(cpu.mmu.get_byte(cpu.registers.hl()), 0xFE);
        assert!(cpu.registers.cy_flag(), "Carry flag should be set");
        assert!(!cpu.registers.z_flag(), "Zero flag should not be set");
        assert!(!cpu.registers.h_flag(), "Half-Carry flag should not be set");
        assert!(!cpu.registers.n_flag(), "Subtract flag should not be set");
    }
}
