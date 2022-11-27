use bitfield::bitfield;
use std::fmt;

bitfield! {
    pub struct Registers(u128);
    impl Debug;

    // Accumulator
    // An 8-bit register for storing data and the results of arithmetic and logical operations.
    pub u8, a, set_a: 127, 120;

    // Flags Register (4 bit flags detailed below)
    pub u8, f, set_f: 119, 112;

    // Zero Flag: Set to 1 when the result of an operation is 0; otherwise reset.
    pub bool, z_flag, set_z_flag: 119;
    // Sub Flag: Set to 1 following execution of the substruction instruction, regardless of the result.
    pub bool, n_flag, set_n_flag: 118;
    // Half-Carry Flag: Set to 1 when an operation results in carrying from or borrowing to bit 3.
    pub bool, h_flag, set_h_flag: 117;
    // Carry Flag: Set to 1 when an operation results in carrying from or borrowing to bit 7.
    pub bool, cy_flag, set_cy_flag: 116;

    // Auxilary Registers to the accumulator.
    pub u8, b, set_b: 111, 104;
    pub u8, c, set_c: 103, 96;
    pub u8, d, set_d: 95, 88;
    pub u8, e, set_e: 87, 80;
    pub u8, h, set_h: 79, 72;
    pub u8, l, set_l: 71, 64;

    // Auxilary Register Pairs
    // 16-bit registers that function as data pointers.
    pub u16, af, set_af: 127, 112;
    pub u16, bc, set_bc: 111, 96;
    pub u16, de, set_de: 95, 80;
    pub u16, hl, set_hl: 79, 64;

    // Program Counter
    // A 16-bit register that holds the address data of the program to be executed next.
    pub u16, pc, set_pc: 63, 48;

    // Stack Pointer
    // A 16-bit register that holds the starting address of the stack area of memory.
    pub u16, sp, set_sp: 47, 32;
}

impl Registers {
    pub fn new() -> Self {
        Registers::default()
    }

    pub fn incr_pc(&mut self) {
        self.set_pc(self.pc().wrapping_add(1));
    }

    pub fn incr_sp(&mut self) {
        self.set_sp(self.sp().wrapping_add(1));
    }

    pub fn decr_sp(&mut self) {
        self.set_sp(self.sp().wrapping_sub(1));
    }

    pub fn incr_hl(&mut self) {
        self.set_hl(self.hl().wrapping_add(1));
    }

    pub fn decr_hl(&mut self) {
        self.set_hl(self.hl().wrapping_sub(1));
    }
}

impl Default for Registers {
    fn default() -> Self {
        //          A F  B C  D E  H L  PC   SP   ---- ----
        Registers(0x01B0_0013_00D8_014D_0000_FFFE_0000_0000)
    }
}

impl fmt::Display for Registers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let a = self.a();
        let b = self.b();
        let c = self.c();
        let d = self.d();
        let e = self.e();
        let h = self.h();
        let l = self.l();
        let pc = self.pc();
        let sp = self.sp();
        let cy = self.cy_flag();
        let hc = self.h_flag();
        let z = self.z_flag();
        let n = self.n_flag();

        write!(f, "A: {a:#04X} | B: {b:#04X} | C: {c:#04X} | D: {d:#04X} | E: {e:#04X} | H: {h:#04X} | L: {l:#04X} | SP: {sp:#04X} | PC: {pc:#04X} | Z: {z:?} | N: {n:?} | H: {hc:?} | C: {cy:?}")
    }
}

#[cfg(test)]
mod test {
    use super::Registers;

    #[test]
    fn defaults_set_correctly() {
        let reg = Registers::default();
        assert!(reg.z_flag(), "Z Flag should be set by default");
        assert!(!reg.n_flag(), "N Flag should not be set by default");
        assert!(reg.h_flag(), "H Flag should be set by default");
        assert!(reg.cy_flag(), "CY Flag should be set by default");
        assert_eq!(reg.a(), 0x01);
        assert_eq!(reg.f(), 0xB0);
        assert_eq!(reg.b(), 0x00);
        assert_eq!(reg.c(), 0x13);
        assert_eq!(reg.d(), 0x00);
        assert_eq!(reg.e(), 0xD8);
        assert_eq!(reg.h(), 0x01);
        assert_eq!(reg.l(), 0x4D);
        assert_eq!(reg.pc(), 0x0000,);
        assert_eq!(reg.sp(), 0xFFFE,);
    }

    #[test]
    fn flags_correspond_to_bits_in_f_register() {
        let mut reg = Registers::new();

        reg.set_f(0b0000_0000);
        assert!(!reg.z_flag(), "Z Flag should not be set");
        assert!(!reg.n_flag(), "N Flag should not be set");
        assert!(!reg.h_flag(), "H Flag should not be set");
        assert!(!reg.cy_flag(), "CY Flag should not be set");

        reg.set_f(0b1000_0000);
        assert!(reg.z_flag(), "Z Flag should be set");

        reg.set_f(0b0100_0000);
        assert!(reg.n_flag(), "N Flag should be set");

        reg.set_f(0b0010_0000);
        assert!(reg.h_flag(), "H Flag should be set");

        reg.set_f(0b0001_0000);
        assert!(reg.cy_flag(), "CY Flag should be set");
    }

    #[test]
    fn wide_registers() {
        let mut reg = Registers::new();

        reg.set_b(0x01);
        reg.set_c(0x02);
        assert_eq!(reg.bc(), 0x0102);

        reg.set_d(0x03);
        reg.set_e(0x04);
        assert_eq!(reg.de(), 0x0304);

        reg.set_h(0x05);
        reg.set_l(0x06);
        assert_eq!(reg.hl(), 0x0506);
    }

    #[test]
    fn incr_pc_increments_program_counter() {
        let mut reg = Registers::new();
        reg.set_pc(0x0001);
        reg.incr_pc();
        assert_eq!(reg.pc(), 0x0002);
    }

    #[test]
    fn incr_pc_wraps_when_out_of_bounds() {
        let mut reg = Registers::new();
        reg.set_pc(0xffff);
        reg.incr_pc();
        assert_eq!(reg.pc(), 0x0000);
    }

    #[test]
    fn incr_sp_increments_stack_pointer() {
        let mut reg = Registers::new();
        reg.set_sp(0x0001);
        reg.incr_sp();
        assert_eq!(reg.sp(), 0x0002);
    }

    #[test]
    fn incr_sp_wraps_when_out_of_bounds() {
        let mut reg = Registers::new();
        reg.set_sp(0xffff);
        reg.incr_sp();
        assert_eq!(reg.sp(), 0x0000);
    }

    #[test]
    fn decr_sp_decrements_stack_pointer() {
        let mut reg = Registers::new();
        reg.set_sp(0x0002);
        reg.decr_sp();
        assert_eq!(reg.sp(), 0x0001);
    }

    #[test]
    fn decr_sp_wraps_when_out_of_bounds() {
        let mut reg = Registers::new();
        reg.set_sp(0x0000);
        reg.decr_sp();
        assert_eq!(reg.sp(), 0xffff);
    }
}
