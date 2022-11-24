use std::fmt;

use super::super::cpu::Cpu;
use super::call::Call;
use super::condition::Condition;
use super::operation::Operation;

pub struct ConditionalCall {
    cond: Condition,
}

impl ConditionalCall {
    pub fn new(cond: Condition) -> Self {
        ConditionalCall { cond }
    }
}

impl Operation for ConditionalCall {
    fn run(&self, cpu: &mut Cpu) {
        if self.cond.check(cpu) {
            Call.run(cpu);
        } else {
            // Increment PC counter.
            cpu.read_u16();
            // Don't have a better way of doing this at this time.
            cpu.remaining_cycles = cpu.remaining_cycles.saturating_sub(12);
        }
    }
}

impl fmt::Display for ConditionalCall {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CALL {},a16", self.cond)
    }
}

#[cfg(test)]
mod test {
    use crate::memory::address_space::AddressSpace;
    use crate::memory::ram::Ram;

    use super::Condition;
    use super::ConditionalCall;
    use super::Cpu;
    use super::Operation;

    fn with_ram(data: Vec<u8>) -> Cpu {
        let mut ram = Ram::new(0, data.len() as u16);
        for (i, n) in data.iter().enumerate() {
            ram.set_byte(i as u16, *n);
        }
        Cpu::new(ram)
    }

    #[test]
    fn display_trait() {
        let op = ConditionalCall::new(Condition::C);
        assert_eq!(format!("{op}"), "CALL C,a16");
    }

    #[test]
    fn example_1_from_gameboy_programming_manual() {
        let mut cpu = with_ram(vec![0x00; 0xFFFF]);

        // Examples: When PC = 7FFCh
        cpu.registers.set_pc(0x7FFC);

        // Increment PC by 1 (We read a byte in order get op_code)
        cpu.registers.incr_pc();

        cpu.registers.set_sp(0xFFFE);
        cpu.mmu.set_byte(0x7FFD, 0x34);
        cpu.mmu.set_byte(0x7FFE, 0x12);

        cpu.registers.set_z_flag(true);

        ConditionalCall::new(Condition::NZ).run(&mut cpu);

        assert_eq!(
            cpu.registers.pc(),
            0x7FFF,
            "PC should be incremented to next operation"
        );
    }

    #[test]
    fn example_2_from_gameboy_programming_manual() {
        let mut cpu = with_ram(vec![0x00; 0xFFFF]);

        // Examples: When PC = 8000h and SP = FFFEh
        cpu.registers.set_pc(0x8000);

        // Increment PC by 1 (We read a byte in order get op_code)
        cpu.registers.incr_pc();

        cpu.registers.set_sp(0xFFFE);
        cpu.mmu.set_byte(0x8001, 0x34);
        cpu.mmu.set_byte(0x8002, 0x12);

        cpu.registers.set_z_flag(true);

        ConditionalCall::new(Condition::Z).run(&mut cpu);

        // Jumps to address 1234h
        let pc = cpu.registers.pc();
        assert_eq!(
            pc, 0x1234,
            "PC should store new address 0x1234, but instead is set to {pc:#06x}"
        );
        // (FFFDH) ← 80H
        assert_eq!(
            cpu.mmu.get_byte(0xFFFD),
            0x80,
            "(0xFFFD) should be set to 0x80"
        );
        // (FFFCH) ← 03H
        assert_eq!(
            cpu.mmu.get_byte(0xFFFC),
            0x03,
            "(0xFFFC) should be set to 0x03"
        );
        // SP ← FFFCH
        assert_eq!(
            cpu.registers.sp(),
            0xFFFC,
            "SP should be set to address where return address stored"
        );
    }
}
