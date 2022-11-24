use super::operations::op_codes::lookup_op_code;
use super::operations::op_codes_extended::lookup_extended_op_code;
use super::registers::Registers;
use crate::memory::address_space::AddressSpace;

pub struct Cpu {
    pub registers: Registers,
    pub mmu: Box<dyn AddressSpace>,
    pub remaining_cycles: u8,
}

impl Cpu {
    pub fn new<Space: AddressSpace + 'static>(mmu: Space) -> Self {
        Cpu {
            registers: Registers::new(),
            mmu: Box::new(mmu),
            remaining_cycles: 0,
        }
    }

    pub fn execute(&mut self, op_code: u8) {
        self.internal_execute(op_code, false);
    }

    pub fn execute_extended(&mut self, op_code: u8) {
        self.internal_execute(op_code, true);
    }

    pub fn read_u8(&mut self) -> u8 {
        let res = self.mmu.get_byte(self.registers.pc());
        self.registers.incr_pc();
        res
    }

    pub fn read_u16(&mut self) -> u16 {
        let l = self.read_u8() as u16;
        let h = self.read_u8() as u16;
        (h << 8) | l
    }

    fn internal_execute(&mut self, op_code: u8, extended: bool) {
        let (op, cycles) = if extended {
            lookup_extended_op_code(op_code)
        } else {
            lookup_op_code(op_code)
        };
        self.remaining_cycles += cycles;
        op.execute(self);
    }
}

#[cfg(test)]
mod test {
    use crate::memory::void::Void;

    use super::Cpu;

    fn empty() -> Cpu {
        Cpu::new(Void)
    }

    #[test]
    fn execute_op_code() {
        let mut cpu = empty();
        cpu.registers.set_a(0x0001);
        cpu.registers.set_c(0x0002);
        let op_code = 0x81; // ADD A, C
        cpu.execute(op_code);
        assert_eq!(cpu.registers.a(), 0x0003);
    }

    #[test]
    fn executing_op_code_updates_remaining_cycles() {
        let mut cpu = empty();
        let op_code = 0x81; // ADD A, C
        assert_eq!(
            cpu.remaining_cycles, 0,
            "Remaining cycles should initially be 0"
        );
        cpu.execute(op_code);
        assert_eq!(
            cpu.remaining_cycles, 4,
            "Remaining cycles should be updated by execute"
        );
    }
}
