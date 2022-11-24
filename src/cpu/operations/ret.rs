use std::fmt;

use super::super::cpu::Cpu;
use super::operation::Operation;

pub struct Ret;

impl Operation for Ret {
    fn run(&self, cpu: &mut Cpu) {
        let sp = cpu.registers.sp();
        let l = cpu.mmu.get_byte(sp) as u16;
        let h = cpu.mmu.get_byte(sp + 1) as u16;
        cpu.registers.set_sp(sp + 2);
        cpu.registers.set_pc(l | h << 8);
    }
}

impl fmt::Display for Ret {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RET")
    }
}

#[cfg(test)]
mod test {
    use crate::cpu::operations::call::Call;
    use crate::memory::address_space::AddressSpace;
    use crate::memory::ram::Ram;

    use super::Cpu;
    use super::Operation;
    use super::Ret;

    fn with_ram(data: Vec<u8>) -> Cpu {
        let mut ram = Ram::new(0, data.len() as u16);
        for (i, n) in data.iter().enumerate() {
            ram.set_byte(i as u16, *n);
        }
        Cpu::new(ram)
    }

    #[test]
    fn display_trait() {
        let op = Ret;
        assert_eq!(format!("{op}"), "RET");
    }

    #[test]
    fn example_from_gameboy_programming_manual() {
        let mut cpu = with_ram(vec![0x00; 0xFFFF]);

        // Examples: When PC = 8000h and SP = FFFEh
        cpu.registers.set_pc(0x8000);

        // Increment PC by 1 (We read a byte in order get op_code)
        cpu.registers.incr_pc();

        cpu.registers.set_sp(0xFFFE);
        cpu.mmu.set_byte(0x8001, 0x34);
        cpu.mmu.set_byte(0x8002, 0x12);

        Call.run(&mut cpu);

        Ret.run(&mut cpu);

        assert_eq!(
            cpu.registers.pc(),
            0x8003,
            "PC should return to previous address"
        );
    }
}
