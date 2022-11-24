use std::fmt;

use super::super::cpu::Cpu;
use super::operation::Operation;
use super::targets::ArithmeticTarget16Bit;

pub struct Dec16 {
    target: ArithmeticTarget16Bit,
}

impl Dec16 {
    pub fn new(target: ArithmeticTarget16Bit) -> Self {
        Dec16 { target }
    }
}

impl Operation for Dec16 {
    fn run(&self, cpu: &mut Cpu) {
        let value = self.target.value(cpu);
        let new_value = value.wrapping_sub(1);
        self.target.set_value(cpu, new_value);
    }
}

impl fmt::Display for Dec16 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DEC {}", self.target)
    }
}

#[cfg(test)]
mod test {
    use crate::memory::void::Void;

    use super::ArithmeticTarget16Bit;
    use super::Cpu;
    use super::Dec16;
    use super::Operation;

    fn empty() -> Cpu {
        Cpu::new(Void)
    }

    #[test]
    fn decrements_register() {
        let mut cpu = empty();
        cpu.registers.set_bc(0x1000);
        Dec16::new(ArithmeticTarget16Bit::BC).run(&mut cpu);
        assert_eq!(cpu.registers.bc(), 0x0FFF);
    }

    #[test]
    fn display_trait() {
        let op = Dec16::new(ArithmeticTarget16Bit::BC);
        assert_eq!(format!("{op}"), "DEC BC");
    }

    #[test]
    fn example_from_gameboy_programming_manual() {
        let mut cpu = empty();

        // When DE = 235Fh,
        cpu.registers.set_de(0x235F);

        // INC DE
        Dec16::new(ArithmeticTarget16Bit::DE).run(&mut cpu);

        // DE ← 235Eh
        assert_eq!(cpu.registers.de(), 0x235E);
    }
}
