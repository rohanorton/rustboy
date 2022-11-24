use super::operation::Operation;
use super::targets::ArithmeticTarget8Bit;

use super::bit::Bit;
use super::rl::Rl;
use super::rlc::Rlc;
use super::rr::Rr;
use super::rrc::Rrc;
use super::sla::Sla;
use super::sra::Sra;
use super::srl::Srl;
use super::swap::Swap;

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

        0x30 => Swap::new(ArithmeticTarget8Bit::B), 8;
        0x31 => Swap::new(ArithmeticTarget8Bit::C), 8;
        0x32 => Swap::new(ArithmeticTarget8Bit::D), 8;
        0x33 => Swap::new(ArithmeticTarget8Bit::E), 8;
        0x34 => Swap::new(ArithmeticTarget8Bit::H), 8;
        0x35 => Swap::new(ArithmeticTarget8Bit::L), 8;
        0x36 => Swap::new(ArithmeticTarget8Bit::HLAddr), 16;
        0x37 => Swap::new(ArithmeticTarget8Bit::A), 8;
        0x38 => Srl::new(ArithmeticTarget8Bit::B), 8;
        0x39 => Srl::new(ArithmeticTarget8Bit::C), 8;
        0x3A => Srl::new(ArithmeticTarget8Bit::D), 8;
        0x3B => Srl::new(ArithmeticTarget8Bit::E), 8;
        0x3C => Srl::new(ArithmeticTarget8Bit::H), 8;
        0x3D => Srl::new(ArithmeticTarget8Bit::L), 8;
        0x3E => Srl::new(ArithmeticTarget8Bit::HLAddr), 16;
        0x3F => Srl::new(ArithmeticTarget8Bit::A), 8;

        0x40 => Bit::new(0, ArithmeticTarget8Bit::B), 8;
        0x41 => Bit::new(0, ArithmeticTarget8Bit::C), 8;
        0x42 => Bit::new(0, ArithmeticTarget8Bit::D), 8;
        0x43 => Bit::new(0, ArithmeticTarget8Bit::E), 8;
        0x44 => Bit::new(0, ArithmeticTarget8Bit::H), 8;
        0x45 => Bit::new(0, ArithmeticTarget8Bit::L), 8;
        0x46 => Bit::new(0, ArithmeticTarget8Bit::HLAddr), 16;
        0x47 => Bit::new(0, ArithmeticTarget8Bit::A), 8;
        0x48 => Bit::new(1, ArithmeticTarget8Bit::B), 8;
        0x49 => Bit::new(1, ArithmeticTarget8Bit::C), 8;
        0x4A => Bit::new(1, ArithmeticTarget8Bit::D), 8;
        0x4B => Bit::new(1, ArithmeticTarget8Bit::E), 8;
        0x4C => Bit::new(1, ArithmeticTarget8Bit::H), 8;
        0x4D => Bit::new(1, ArithmeticTarget8Bit::L), 8;
        0x4E => Bit::new(1, ArithmeticTarget8Bit::HLAddr), 16;
        0x4F => Bit::new(1, ArithmeticTarget8Bit::A), 8;

        0x50 => Bit::new(2, ArithmeticTarget8Bit::B), 8;
        0x51 => Bit::new(2, ArithmeticTarget8Bit::C), 8;
        0x52 => Bit::new(2, ArithmeticTarget8Bit::D), 8;
        0x53 => Bit::new(2, ArithmeticTarget8Bit::E), 8;
        0x54 => Bit::new(2, ArithmeticTarget8Bit::H), 8;
        0x55 => Bit::new(2, ArithmeticTarget8Bit::L), 8;
        0x56 => Bit::new(2, ArithmeticTarget8Bit::HLAddr), 16;
        0x57 => Bit::new(2, ArithmeticTarget8Bit::A), 8;
        0x58 => Bit::new(3, ArithmeticTarget8Bit::B), 8;
        0x59 => Bit::new(3, ArithmeticTarget8Bit::C), 8;
        0x5A => Bit::new(3, ArithmeticTarget8Bit::D), 8;
        0x5B => Bit::new(3, ArithmeticTarget8Bit::E), 8;
        0x5C => Bit::new(3, ArithmeticTarget8Bit::H), 8;
        0x5D => Bit::new(3, ArithmeticTarget8Bit::L), 8;
        0x5E => Bit::new(3, ArithmeticTarget8Bit::HLAddr), 16;
        0x5F => Bit::new(3, ArithmeticTarget8Bit::A), 8;

        0x60 => Bit::new(4, ArithmeticTarget8Bit::B), 8;
        0x61 => Bit::new(4, ArithmeticTarget8Bit::C), 8;
        0x62 => Bit::new(4, ArithmeticTarget8Bit::D), 8;
        0x63 => Bit::new(4, ArithmeticTarget8Bit::E), 8;
        0x64 => Bit::new(4, ArithmeticTarget8Bit::H), 8;
        0x65 => Bit::new(4, ArithmeticTarget8Bit::L), 8;
        0x66 => Bit::new(4, ArithmeticTarget8Bit::HLAddr), 16;
        0x67 => Bit::new(4, ArithmeticTarget8Bit::A), 8;
        0x68 => Bit::new(5, ArithmeticTarget8Bit::B), 8;
        0x69 => Bit::new(5, ArithmeticTarget8Bit::C), 8;
        0x6A => Bit::new(5, ArithmeticTarget8Bit::D), 8;
        0x6B => Bit::new(5, ArithmeticTarget8Bit::E), 8;
        0x6C => Bit::new(5, ArithmeticTarget8Bit::H), 8;
        0x6D => Bit::new(5, ArithmeticTarget8Bit::L), 8;
        0x6E => Bit::new(5, ArithmeticTarget8Bit::HLAddr), 16;
        0x6F => Bit::new(5, ArithmeticTarget8Bit::A), 8;

        0x70 => Bit::new(6, ArithmeticTarget8Bit::B), 8;
        0x71 => Bit::new(6, ArithmeticTarget8Bit::C), 8;
        0x72 => Bit::new(6, ArithmeticTarget8Bit::D), 8;
        0x73 => Bit::new(6, ArithmeticTarget8Bit::E), 8;
        0x74 => Bit::new(6, ArithmeticTarget8Bit::H), 8;
        0x75 => Bit::new(6, ArithmeticTarget8Bit::L), 8;
        0x76 => Bit::new(6, ArithmeticTarget8Bit::HLAddr), 16;
        0x77 => Bit::new(6, ArithmeticTarget8Bit::A), 8;
        0x78 => Bit::new(7, ArithmeticTarget8Bit::B), 8;
        0x79 => Bit::new(7, ArithmeticTarget8Bit::C), 8;
        0x7A => Bit::new(7, ArithmeticTarget8Bit::D), 8;
        0x7B => Bit::new(7, ArithmeticTarget8Bit::E), 8;
        0x7C => Bit::new(7, ArithmeticTarget8Bit::H), 8;
        0x7D => Bit::new(7, ArithmeticTarget8Bit::L), 8;
        0x7E => Bit::new(7, ArithmeticTarget8Bit::HLAddr), 16;
        0x7F => Bit::new(7, ArithmeticTarget8Bit::A), 8;
    })
}
