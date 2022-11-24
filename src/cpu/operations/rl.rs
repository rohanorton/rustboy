use std::fmt;

use super::super::cpu::Cpu;
use super::operation::Operation;
use super::targets::ArithmeticTarget8Bit;

// Rotates the contents of operand to the left.
pub struct Rl {
    operand: ArithmeticTarget8Bit,
}

impl Rl {
    pub fn new(operand: ArithmeticTarget8Bit) -> Self {
        Rl { operand }
    }
}

impl Operation for Rl {
    fn execute(&self, cpu: &mut Cpu) {
        let x = self.operand.value(cpu) as u16;
        let carry_bit = cpu.registers.cy_flag() as u16;
        let bit_7 = x >> 7;
        let rot_x = (((x << 1) + carry_bit) & 0xFF) as u8;
        self.operand.set_value(cpu, rot_x);
        cpu.registers.set_cy_flag(bit_7 != 0);
        cpu.registers.set_z_flag(rot_x == 0);
        cpu.registers.set_h_flag(false);
        cpu.registers.set_n_flag(false);
    }
}

impl fmt::Display for Rl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RL {}", self.operand)
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
    use super::Rl;

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
        let op = Rl::new(ArithmeticTarget8Bit::C);
        assert_eq!(format!("{op}"), "RL C");
    }

    #[test]
    fn example_1_from_gameboy_programming_manual() {
        let mut cpu = empty();

        // When L = 80h, and CY = 0,
        cpu.registers.set_l(0x80);
        cpu.registers.set_cy_flag(false);

        // RL L
        Rl::new(ArithmeticTarget8Bit::L).execute(&mut cpu);

        // L←00h,CY←1,Z←1,H←0,N←0
        assert_eq!(cpu.registers.l(), 0x00);
        assert!(cpu.registers.cy_flag(), "Carry flag should be set");
        assert!(cpu.registers.z_flag(), "Zero flag should be set");
        assert!(!cpu.registers.h_flag(), "Half-Carry flag should not be set");
        assert!(!cpu.registers.n_flag(), "Subtract flag should not be set");
    }

    #[test]
    fn example_2_from_gameboy_programming_manual() {
        let mut cpu = with_ram(vec![0x00; 0xFFFF]);

        // When (HL) = 11h, and CY = 0,
        cpu.mmu.set_byte(cpu.registers.hl(), 0x11);
        cpu.registers.set_cy_flag(false);

        // RL (HL)
        Rl::new(ArithmeticTarget8Bit::HLAddr).execute(&mut cpu);

        // (HL)←22h,CY←0,Z←0,H←0,N←0
        assert_eq!(cpu.mmu.get_byte(cpu.registers.hl()), 0x22);
        assert!(!cpu.registers.z_flag(), "Zero flag should not be set");
        assert!(!cpu.registers.cy_flag(), "Carry flag should not be set");
        assert!(!cpu.registers.h_flag(), "Half-Carry flag should not be set");
        assert!(!cpu.registers.n_flag(), "Subtract flag should not be set");
    }
}
