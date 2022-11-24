use std::fmt;

use super::super::cpu::Cpu;
use super::operation::Operation;
use super::targets::PushPopTarget;

pub struct Push {
    src: PushPopTarget,
}

impl Push {
    pub fn new(src: PushPopTarget) -> Self {
        Push { src }
    }
}

impl Operation for Push {
    fn run(&self, cpu: &mut Cpu) {
        let qq = self.src.value(cpu);
        let qq_h = (qq & 0x00FF) as u8;
        let qq_l = (qq >> 8) as u8;

        cpu.registers.decr_sp();
        cpu.mmu.set_byte(cpu.registers.sp(), qq_h);
        cpu.registers.decr_sp();
        cpu.mmu.set_byte(cpu.registers.sp(), qq_l);
    }
}

impl fmt::Display for Push {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PUSH {}", self.src)
    }
}

#[cfg(test)]
mod test {
    use crate::memory::address_space::AddressSpace;
    use crate::memory::ram::Ram;

    use super::Cpu;
    use super::Operation;
    use super::Push;
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
        let op = Push::new(PushPopTarget::BC);
        assert_eq!(format!("{op}"), "PUSH BC");
    }

    #[test]
    fn example_from_gameboy_programming_manual() {
        let mut cpu = with_ram(vec![0x00; 0xFFFF]);

        // Example doesn't give BC values, so just setting arbitrarily
        cpu.registers.set_bc(0x239F);

        // When SP = FFFEh
        cpu.registers.set_sp(0xFFFE);

        // PUSH BC
        Push::new(PushPopTarget::BC).run(&mut cpu);

        // (FFFDh) ← B, (FFFCh) ← B, SP ← FFFCh
        assert_eq!(cpu.mmu.get_byte(0xFFFD), cpu.registers.c());
        assert_eq!(cpu.mmu.get_byte(0xFFFC), cpu.registers.b());
        assert_eq!(cpu.registers.sp(), 0xFFFC);
    }
}
