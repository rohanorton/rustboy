use super::operation::Operation;
use super::targets::ArithmeticTarget8Bit;

use super::rl::Rl;
use super::rlc::Rlc;
use super::rr::Rr;
use super::rrc::Rrc;
use super::sla::Sla;
use super::sra::Sra;

// Macro to simplify op-code match creation. Wraps result in Box, to prevent type error.
macro_rules! boxed_operation {(
    $op_code:expr, { $($lhs:expr => $rhs:expr, $cycles:expr;)+ }
) => (
    match $op_code {
        $($lhs => (::std::boxed::Box::new($rhs), $cycles),)+
        _ => panic!("Unimplemented Extended Op Code {:#02x}", $op_code),
    }
)}

pub fn lookup_extended_op_code(op_code: u8) -> (Box<dyn Operation>, u8) {
    boxed_operation!(op_code, {
        0x00 => Rlc::new(ArithmeticTarget8Bit::B), 8;
        0x01 => Rlc::new(ArithmeticTarget8Bit::C), 8;
        0x02 => Rlc::new(ArithmeticTarget8Bit::D), 8;
        0x03 => Rlc::new(ArithmeticTarget8Bit::E), 8;
        0x04 => Rlc::new(ArithmeticTarget8Bit::H), 8;
        0x05 => Rlc::new(ArithmeticTarget8Bit::L), 8;
        0x06 => Rlc::new(ArithmeticTarget8Bit::HLAddr), 16;
        0x07 => Rlc::new(ArithmeticTarget8Bit::A), 8;
        0x08 => Rrc::new(ArithmeticTarget8Bit::B), 8;
        0x09 => Rrc::new(ArithmeticTarget8Bit::C), 8;
        0x0A => Rrc::new(ArithmeticTarget8Bit::D), 8;
        0x0B => Rrc::new(ArithmeticTarget8Bit::E), 8;
        0x0C => Rrc::new(ArithmeticTarget8Bit::H), 8;
        0x0D => Rrc::new(ArithmeticTarget8Bit::L), 8;
        0x0E => Rrc::new(ArithmeticTarget8Bit::HLAddr), 16;
        0x0F => Rrc::new(ArithmeticTarget8Bit::A), 8;

        0x10 => Rl::new(ArithmeticTarget8Bit::B), 8;
        0x11 => Rl::new(ArithmeticTarget8Bit::C), 8;
        0x12 => Rl::new(ArithmeticTarget8Bit::D), 8;
        0x13 => Rl::new(ArithmeticTarget8Bit::E), 8;
        0x14 => Rl::new(ArithmeticTarget8Bit::H), 8;
        0x15 => Rl::new(ArithmeticTarget8Bit::L), 8;
        0x16 => Rl::new(ArithmeticTarget8Bit::HLAddr), 16;
        0x17 => Rl::new(ArithmeticTarget8Bit::A), 8;
        0x18 => Rr::new(ArithmeticTarget8Bit::B), 8;
        0x19 => Rr::new(ArithmeticTarget8Bit::C), 8;
        0x1A => Rr::new(ArithmeticTarget8Bit::D), 8;
        0x1B => Rr::new(ArithmeticTarget8Bit::E), 8;
        0x1C => Rr::new(ArithmeticTarget8Bit::H), 8;
        0x1D => Rr::new(ArithmeticTarget8Bit::L), 8;
        0x1E => Rr::new(ArithmeticTarget8Bit::HLAddr), 16;
        0x1F => Rr::new(ArithmeticTarget8Bit::A), 8;

        0x20 => Sla::new(ArithmeticTarget8Bit::B), 8;
        0x21 => Sla::new(ArithmeticTarget8Bit::C), 8;
        0x22 => Sla::new(ArithmeticTarget8Bit::D), 8;
        0x23 => Sla::new(ArithmeticTarget8Bit::E), 8;
        0x24 => Sla::new(ArithmeticTarget8Bit::H), 8;
        0x25 => Sla::new(ArithmeticTarget8Bit::L), 8;
        0x26 => Sla::new(ArithmeticTarget8Bit::HLAddr), 16;
        0x27 => Sla::new(ArithmeticTarget8Bit::A), 8;
        0x28 => Sra::new(ArithmeticTarget8Bit::B), 8;
        0x29 => Sra::new(ArithmeticTarget8Bit::C), 8;
        0x2A => Sra::new(ArithmeticTarget8Bit::D), 8;
        0x2B => Sra::new(ArithmeticTarget8Bit::E), 8;
        0x2C => Sra::new(ArithmeticTarget8Bit::H), 8;
        0x2D => Sra::new(ArithmeticTarget8Bit::L), 8;
        0x2E => Sra::new(ArithmeticTarget8Bit::HLAddr), 16;
        0x2F => Sra::new(ArithmeticTarget8Bit::A), 8;
    })
}
