use super::operations::op_codes::lookup_op_code;
use super::registers::Registers;
use crate::memory::address_space::AddressSpace;

pub struct Cpu {
    pub registers: Registers,
    pub mmu: Box<dyn AddressSpace>,
    _remaining_cycles: u8,
}

impl Cpu {
    pub fn new<Space: AddressSpace + 'static>(mmu: Space) -> Self {
        Cpu {
            registers: Registers::new(),
            mmu: Box::new(mmu),
            _remaining_cycles: 0,
        }
    }

    fn _execute(&mut self, op_code: u8) {
        let op = lookup_op_code(op_code);
        self._remaining_cycles = op.execute(self);
    }

    pub fn read_u8(&mut self) -> u8 {
        let res = self.mmu.get_byte(self.registers.pc());
        self.registers.incr_pc();
        res
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

        cpu._execute(op_code);

        assert_eq!(cpu.registers.a(), 0x0003);
    }

    #[test]
    fn executing_op_code_updates_remaining_cycles() {
        let mut cpu = empty();

        let op_code = 0x81; // ADD A, C
        assert_eq!(
            cpu._remaining_cycles, 0,
            "Remaining cycles should initially be 0"
        );

        cpu._execute(op_code);
        assert_eq!(
            cpu._remaining_cycles, 4,
            "Remaining cycles should be updated by execute"
        );
    }
}
