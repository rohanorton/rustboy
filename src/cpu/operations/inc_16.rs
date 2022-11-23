use std::fmt;

use super::super::cpu::Cpu;
use super::operation::Operation;
use super::targets::ArithmeticTarget16Bit;

pub struct Inc16 {
    target: ArithmeticTarget16Bit,
    cycles: u8,
}

impl Inc16 {
    pub fn new(target: ArithmeticTarget16Bit, cycles: u8) -> Self {
        Inc16 { target, cycles }
    }
}

impl Operation for Inc16 {
    fn execute(&self, cpu: &mut Cpu) -> u8 {
        let value = self.target.value(cpu);
        let new_value = value.wrapping_add(1);
        self.target.set_value(cpu, new_value);

        self.cycles
    }
}

impl fmt::Display for Inc16 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "INC {}", self.target)
    }
}

#[cfg(test)]
mod test {
    use crate::memory::void::Void;

    use super::ArithmeticTarget16Bit;
    use super::Cpu;
    use super::Inc16;
    use super::Operation;

    fn empty() -> Cpu {
        Cpu::new(Void)
    }

    const CYCLE_COUNT: u8 = 4;

    #[test]
    fn returns_cycle_count() {
        let mut cpu = empty();

        let op = Inc16::new(ArithmeticTarget16Bit::BC, CYCLE_COUNT);

        let res = op.execute(&mut cpu);

        assert_eq!(
            res, CYCLE_COUNT,
            "Returned value should match cycle count passed to constructor"
        );
    }

    #[test]
    fn increments_register() {
        let mut cpu = empty();
        cpu.registers.set_bc(0x0FFF);

        let op = Inc16::new(ArithmeticTarget16Bit::BC, CYCLE_COUNT);

        op.execute(&mut cpu);

        assert_eq!(cpu.registers.bc(), 0x1000);
    }

    #[test]
    fn display_trait() {
        let op = Inc16::new(ArithmeticTarget16Bit::BC, CYCLE_COUNT);
        assert_eq!(format!("{op}"), "INC BC");
    }

    #[test]
    fn example_from_gameboy_programming_manual() {
        let mut cpu = empty();

        // When DE = 235Fh,
        cpu.registers.set_de(0x235F);

        // INC DE
        let op = Inc16::new(ArithmeticTarget16Bit::DE, CYCLE_COUNT);

        op.execute(&mut cpu);

        // DE ← 2360h
        assert_eq!(cpu.registers.de(), 0x2360);
    }
}
