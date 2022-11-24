use std::fmt;

use super::super::cpu::Cpu;
use super::operation::Operation;
use super::targets::PushPopTarget;

pub struct Pop {
    src: PushPopTarget,
}

impl Pop {
    pub fn new(src: PushPopTarget) -> Self {
        Pop { src }
    }
}

impl Operation for Pop {
    fn run(&self, cpu: &mut Cpu) {
        let qq_l = cpu.mmu.get_byte(cpu.reg.sp()) as u16;
        cpu.reg.incr_sp();
        let qq_h = cpu.mmu.get_byte(cpu.reg.sp()) as u16;
        cpu.reg.incr_sp();
        let val = (qq_h << 8) + qq_l;
        self.src.set_value(cpu, val);
    }
}

impl fmt::Display for Pop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "POP {}", self.src)
    }
}

#[cfg(test)]
mod test {
    use crate::memory::address_space::AddressSpace;
    use crate::memory::ram::Ram;

    use super::Cpu;
    use super::Operation;
    use super::Pop;
    use super::PushPopTarget;

    fn with_ram(data: Vec<u8>) -> Cpu {
        let mut ram = Ram::new(0, data.len() as u16);
        for (i, n) in data.iter().enumerate() {
            ram.set_byte(i as u16, *n);
        }
        Cpu::new(ram)
    }

    #[test]
    fn display_trait() {
        let op = Pop::new(PushPopTarget::BC);
        assert_eq!(format!("{op}"), "POP BC");
    }

    #[test]
    fn example_from_gameboy_programming_manual() {
        let mut cpu = with_ram(vec![0x00; 0xFFFF]);

        // Setting BC to null to make clearer
        cpu.reg.set_bc(0x0000);

        // When SP = FFFCh, (FFFCh) = 5Fh, and (FFFDh) = 3Ch
        cpu.reg.set_sp(0xFFFC);
        cpu.mmu.set_byte(0xFFFC, 0x5F);
        cpu.mmu.set_byte(0xFFFD, 0x3C);

        // POP BC
        Pop::new(PushPopTarget::BC).run(&mut cpu);

        // B ← 3Ch, C ← 5Fh, SP ← FFFEh
        assert_eq!(cpu.reg.b(), 0x3C);
        assert_eq!(cpu.reg.c(), 0x5F);
        assert_eq!(cpu.reg.sp(), 0xFFFE);
    }
}
