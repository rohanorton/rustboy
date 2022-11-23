use std::fmt;

use super::super::cpu::Cpu;
use super::operation::Operation;
use super::targets::ArithmeticTarget16Bit;

pub struct AddHl {
    target: ArithmeticTarget16Bit,
    cycles: u8,
}

impl AddHl {
    pub fn new(target: ArithmeticTarget16Bit, cycles: u8) -> Self {
        AddHl { target, cycles }
    }
}

impl Operation for AddHl {
    fn execute(&self, cpu: &mut Cpu) -> u8 {
        let value = self.target.value(cpu);
        let (new_value, did_carry) = cpu.registers.hl().overflowing_add(value);

        cpu.registers.set_n_flag(false);
        cpu.registers.set_cy_flag(did_carry);
        cpu.registers.set_h_flag(false);
        cpu.registers
            .set_h_flag(((cpu.registers.hl() & 0xFFF) + (value & 0xFFF)) & 0x1000 != 0);

        cpu.registers.set_hl(new_value);

        self.cycles
    }
}

impl fmt::Display for AddHl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ADD HL,{}", self.target)
    }
}

#[cfg(test)]
mod test {
    use crate::memory::void::Void;

    use super::AddHl;
    use super::ArithmeticTarget16Bit;
    use super::Cpu;
    use super::Operation;

    fn empty() -> Cpu {
        Cpu::new(Void)
    }

    const CYCLE_COUNT: u8 = 4;

    #[test]
    fn returns_cycle_count() {
        let mut cpu = empty();

        let op = AddHl::new(ArithmeticTarget16Bit::BC, CYCLE_COUNT);

        let res = op.execute(&mut cpu);

        assert_eq!(
            res, CYCLE_COUNT,
            "Returned value should match cycle count passed to constructor"
        );
    }

    #[test]
    fn increments_register() {
        let mut cpu = empty();
        cpu.registers.set_bc(0x010F);
        cpu.registers.set_hl(0x0201);

        let op = AddHl::new(ArithmeticTarget16Bit::BC, CYCLE_COUNT);

        op.execute(&mut cpu);

        assert_eq!(cpu.registers.hl(), 0x0310);
    }

    #[test]
    fn display_trait() {
        let op = AddHl::new(ArithmeticTarget16Bit::BC, CYCLE_COUNT);
        assert_eq!(format!("{op}"), "ADD HL,BC");
    }

    #[test]
    fn example_1_from_gameboy_programming_manual() {
        let mut cpu = empty();

        // When HL = 8A23h, BC = 0605h,
        cpu.registers.set_hl(0x8A23);
        cpu.registers.set_bc(0x0605);

        // ADDHL,BC
        AddHl::new(ArithmeticTarget16Bit::BC, CYCLE_COUNT).execute(&mut cpu);

        // HL←9028h,H←1,N←0,CY←0
        assert_eq!(cpu.registers.hl(), 0x9028);
        assert!(cpu.registers.h_flag());
        assert!(!cpu.registers.n_flag());
        assert!(!cpu.registers.cy_flag());
    }

    #[test]
    fn example_2_from_gameboy_programming_manual() {
        let mut cpu = empty();

        // When HL = 8A23h
        cpu.registers.set_hl(0x8A23);

        // ADDHL,HL
        AddHl::new(ArithmeticTarget16Bit::HL, CYCLE_COUNT).execute(&mut cpu);

        // HL←1446h,H←1,N←0,CY←1
        assert_eq!(cpu.registers.hl(), 0x1446);
        assert!(cpu.registers.h_flag());
        assert!(!cpu.registers.n_flag());
        assert!(cpu.registers.cy_flag());
    }
}
