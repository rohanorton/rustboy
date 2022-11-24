use std::fmt;

use super::super::cpu::Cpu;
use super::operation::Operation;
use super::targets::ArithmeticTarget16Bit;

pub struct AddHl {
    target: ArithmeticTarget16Bit,
}

impl AddHl {
    pub fn new(target: ArithmeticTarget16Bit) -> Self {
        AddHl { target }
    }
}

impl Operation for AddHl {
    fn run(&self, cpu: &mut Cpu) {
        let value = self.target.value(cpu);
        let (new_value, did_carry) = cpu.reg.hl().overflowing_add(value);

        cpu.reg.set_n_flag(false);
        cpu.reg.set_cy_flag(did_carry);
        cpu.reg.set_h_flag(false);
        cpu.reg
            .set_h_flag(((cpu.reg.hl() & 0xFFF) + (value & 0xFFF)) & 0x1000 != 0);

        cpu.reg.set_hl(new_value);
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

    #[test]
    fn increments_register() {
        let mut cpu = empty();
        cpu.reg.set_bc(0x010F);
        cpu.reg.set_hl(0x0201);
        AddHl::new(ArithmeticTarget16Bit::BC).run(&mut cpu);
        assert_eq!(cpu.reg.hl(), 0x0310);
    }

    #[test]
    fn display_trait() {
        let op = AddHl::new(ArithmeticTarget16Bit::BC);
        assert_eq!(format!("{op}"), "ADD HL,BC");
    }

    #[test]
    fn example_1_from_gameboy_programming_manual() {
        let mut cpu = empty();

        // When HL = 8A23h, BC = 0605h,
        cpu.reg.set_hl(0x8A23);
        cpu.reg.set_bc(0x0605);

        // ADDHL,BC
        AddHl::new(ArithmeticTarget16Bit::BC).run(&mut cpu);

        // HL←9028h,H←1,N←0,CY←0
        assert_eq!(cpu.reg.hl(), 0x9028);
        assert!(cpu.reg.h_flag());
        assert!(!cpu.reg.n_flag());
        assert!(!cpu.reg.cy_flag());
    }

    #[test]
    fn example_2_from_gameboy_programming_manual() {
        let mut cpu = empty();

        // When HL = 8A23h
        cpu.reg.set_hl(0x8A23);

        // ADDHL,HL
        AddHl::new(ArithmeticTarget16Bit::HL).run(&mut cpu);

        // HL←1446h,H←1,N←0,CY←1
        assert_eq!(cpu.reg.hl(), 0x1446);
        assert!(cpu.reg.h_flag());
        assert!(!cpu.reg.n_flag());
        assert!(cpu.reg.cy_flag());
    }
}
