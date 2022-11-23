use std::fmt;

use super::super::cpu::Cpu;
use super::operation::Operation;
use super::targets::ArithmeticTarget8Bit;

pub struct Adc {
    target: ArithmeticTarget8Bit,
}

impl Adc {
    pub fn new(target: ArithmeticTarget8Bit) -> Self {
        Adc { target }
    }

    // TODO: This is awful... There must be a better way!?
    fn carrying_add(x: u8, y: u8, cy: bool) -> (u8, bool) {
        let mut res: u8 = x;
        let mut res_cy = false;

        for n in [y, cy as u8] {
            let (new_res, new_cy) = res.overflowing_add(n);
            res = new_res;
            res_cy = new_cy || res_cy;
        }

        (res, res_cy)
    }
}

impl Operation for Adc {
    fn execute(&self, cpu: &mut Cpu) {
        let value = self.target.value(cpu);

        let (new_value, did_overflow) =
            Self::carrying_add(cpu.registers.a(), value, cpu.registers.cy_flag());

        cpu.registers.set_z_flag(new_value == 0);
        cpu.registers.set_n_flag(false);
        cpu.registers.set_cy_flag(did_overflow);
        cpu.registers
            .set_h_flag(((cpu.registers.a() & 0xF) + (value & 0xF)) & 0x10 != 0);

        // Set result in accumulator
        cpu.registers.set_a(new_value);
    }
}

impl fmt::Display for Adc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ADC A,{}", self.target)
    }
}

#[cfg(test)]
mod test {
    use crate::memory::void::Void;

    use super::Adc;
    use super::ArithmeticTarget8Bit;
    use super::Cpu;
    use super::Operation;

    fn empty() -> Cpu {
        Cpu::new(Void)
    }

    #[test]
    fn adds_register_and_carry_flag_to_accumulator() {
        let mut cpu = empty();
        cpu.registers.set_a(0x01);
        cpu.registers.set_c(0x02);
        cpu.registers.set_cy_flag(true);
        Adc::new(ArithmeticTarget8Bit::C).execute(&mut cpu);
        assert_eq!(cpu.registers.a(), 0x04);
    }

    #[test]
    fn sets_zero_flag_when_result_eq_0() {
        let mut cpu = empty();
        cpu.registers.set_a(0xF0);
        cpu.registers.set_c(0x0F);
        cpu.registers.set_cy_flag(true);
        Adc::new(ArithmeticTarget8Bit::C).execute(&mut cpu);
        assert!(cpu.registers.z_flag());
    }

    #[test]
    fn unsets_zero_flag_when_result_ne_0() {
        let mut cpu = empty();
        cpu.registers.set_a(0x00);
        cpu.registers.set_c(0x01);
        cpu.registers.set_cy_flag(true);
        Adc::new(ArithmeticTarget8Bit::C).execute(&mut cpu);
        assert!(!cpu.registers.z_flag());
    }

    #[test]
    fn unsets_sub_flag() {
        let mut cpu = empty();
        cpu.registers.set_a(0x02);
        cpu.registers.set_c(0x04);
        cpu.registers.set_cy_flag(true);
        Adc::new(ArithmeticTarget8Bit::C).execute(&mut cpu);
        assert!(!cpu.registers.n_flag());
    }

    #[test]
    fn sets_carry_flag_on_overflow() {
        let mut cpu = empty();
        cpu.registers.set_a(0xFF);
        cpu.registers.set_c(0x01);
        cpu.registers.set_cy_flag(false);
        Adc::new(ArithmeticTarget8Bit::C).execute(&mut cpu);
        assert!(cpu.registers.cy_flag());
    }

    #[test]
    fn unsets_carry_flag_when_no_overflow() {
        let mut cpu = empty();
        cpu.registers.set_a(0xFE);
        cpu.registers.set_c(0x01);
        cpu.registers.set_cy_flag(false);
        Adc::new(ArithmeticTarget8Bit::C).execute(&mut cpu);
        assert!(!cpu.registers.cy_flag());
    }

    #[test]
    fn sets_halfcarry_flag_on_lower_nibble_overflow() {
        let mut cpu = empty();
        cpu.registers.set_a(0x0F);
        cpu.registers.set_c(0x01);
        cpu.registers.set_cy_flag(false);
        Adc::new(ArithmeticTarget8Bit::C).execute(&mut cpu);
        assert!(cpu.registers.h_flag());
    }

    #[test]
    fn unsets_halfcarry_flag_when_no_lower_nibble_overflow() {
        let mut cpu = empty();
        cpu.registers.set_a(0x0E);
        cpu.registers.set_c(0x01);
        cpu.registers.set_cy_flag(false);
        Adc::new(ArithmeticTarget8Bit::C).execute(&mut cpu);
        assert!(!cpu.registers.h_flag());
    }

    #[test]
    fn display_trait() {
        let op = Adc::new(ArithmeticTarget8Bit::C);
        assert_eq!(format!("{op}"), "ADC A,C");
    }

    #[test]
    fn example_from_gameboy_programming_manual() {
        let mut cpu = empty();

        // When A=E1h,E=0Fh and CY=1,
        cpu.registers.set_a(0xE1);
        cpu.registers.set_e(0x0F);
        cpu.registers.set_cy_flag(true);

        // ADC A, E
        Adc::new(ArithmeticTarget8Bit::E).execute(&mut cpu);

        // A←F1h,Z←0,H←1,CY←0
        assert_eq!(cpu.registers.a(), 0xF1);
        assert!(!cpu.registers.z_flag());
        assert!(cpu.registers.h_flag());
        assert!(!cpu.registers.cy_flag());
    }
}
