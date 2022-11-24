use std::fmt;

use super::super::cpu::Cpu;
use super::operation::Operation;
use super::targets::ArithmeticTarget8Bit;

// Shifts the contents of operand to the right.
pub struct Srl {
    operand: ArithmeticTarget8Bit,
}

impl Srl {
    pub fn new(operand: ArithmeticTarget8Bit) -> Self {
        Srl { operand }
    }
}

impl Operation for Srl {
    fn run(&self, cpu: &mut Cpu) {
        // Shifts the contents of operand m to the right. That is, the contents
        // of bit 7 are copied to bit 6 and the previous contents of bit 6 (the
        // contents before the copy operation) are copied to bit 5. The same
        // operation is repeated in sequence for the rest of the operand.
        // The contents of bit 0 are copied to CY, and the content of bit 7 is reset.
        let x = self.operand.value(cpu);
        let shift_x = x >> 1;
        self.operand.set_value(cpu, shift_x);
        cpu.reg.set_cy_flag(x & 1 != 0);
        cpu.reg.set_z_flag(shift_x == 0);
        cpu.reg.set_h_flag(false);
        cpu.reg.set_n_flag(false);
    }
}

impl fmt::Display for Srl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SRL {}", self.operand)
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
    use super::Srl;

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
        let op = Srl::new(ArithmeticTarget8Bit::C);
        assert_eq!(format!("{op}"), "SRL C");
    }

    #[test]
    fn example_1_from_gameboy_programming_manual() {
        let mut cpu = empty();

        // When A = 01h, CY + 0,
        cpu.reg.set_a(0x01);
        cpu.reg.set_cy_flag(false);

        // SRL A
        Srl::new(ArithmeticTarget8Bit::A).run(&mut cpu);

        // A←00h,CY←1,Z←1,H←0,N←0
        assert_eq!(cpu.reg.a(), 0x00);
        assert!(cpu.reg.cy_flag(), "Carry flag should be set");
        assert!(cpu.reg.z_flag(), "Zero flag should be set");
        assert!(!cpu.reg.h_flag(), "Half-Carry flag should not be set");
        assert!(!cpu.reg.n_flag(), "Subtract flag should not be set");
    }

    #[test]
    fn example_2_from_gameboy_programming_manual() {
        let mut cpu = with_ram(vec![0x00; 0xFFFF]);

        // When (HL) = FFh, CY + 0,
        cpu.mmu.set_byte(cpu.reg.hl(), 0xFF);
        cpu.reg.set_cy_flag(false);

        // SRL (HL)
        Srl::new(ArithmeticTarget8Bit::HLAddr).run(&mut cpu);

        // (HL)←7Fh,CY←1,Z←0,H←0,N←0
        assert_eq!(cpu.mmu.get_byte(cpu.reg.hl()), 0x7F);
        assert!(cpu.reg.cy_flag(), "Carry flag should be set");
        assert!(!cpu.reg.z_flag(), "Zero flag should not be set");
        assert!(!cpu.reg.h_flag(), "Half-Carry flag should not be set");
        assert!(!cpu.reg.n_flag(), "Subtract flag should not be set");
    }
}
