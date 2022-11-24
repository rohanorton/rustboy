use super::operation::Operation;
use super::targets::ArithmeticTarget8Bit;

use super::bit::Bit;
use super::res::Res;
use super::rl::Rl;
use super::rlc::Rlc;
use super::rr::Rr;
use super::rrc::Rrc;
use super::set::Set;
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

        0x80 => Res::new(0, ArithmeticTarget8Bit::B), 8;
        0x81 => Res::new(0, ArithmeticTarget8Bit::C), 8;
        0x82 => Res::new(0, ArithmeticTarget8Bit::D), 8;
        0x83 => Res::new(0, ArithmeticTarget8Bit::E), 8;
        0x84 => Res::new(0, ArithmeticTarget8Bit::H), 8;
        0x85 => Res::new(0, ArithmeticTarget8Bit::L), 8;
        0x86 => Res::new(0, ArithmeticTarget8Bit::HLAddr), 16;
        0x87 => Res::new(0, ArithmeticTarget8Bit::A), 8;
        0x88 => Res::new(1, ArithmeticTarget8Bit::B), 8;
        0x89 => Res::new(1, ArithmeticTarget8Bit::C), 8;
        0x8A => Res::new(1, ArithmeticTarget8Bit::D), 8;
        0x8B => Res::new(1, ArithmeticTarget8Bit::E), 8;
        0x8C => Res::new(1, ArithmeticTarget8Bit::H), 8;
        0x8D => Res::new(1, ArithmeticTarget8Bit::L), 8;
        0x8E => Res::new(1, ArithmeticTarget8Bit::HLAddr), 16;
        0x8F => Res::new(1, ArithmeticTarget8Bit::A), 8;

        0x90 => Res::new(2, ArithmeticTarget8Bit::B), 8;
        0x91 => Res::new(2, ArithmeticTarget8Bit::C), 8;
        0x92 => Res::new(2, ArithmeticTarget8Bit::D), 8;
        0x93 => Res::new(2, ArithmeticTarget8Bit::E), 8;
        0x94 => Res::new(2, ArithmeticTarget8Bit::H), 8;
        0x95 => Res::new(2, ArithmeticTarget8Bit::L), 8;
        0x96 => Res::new(2, ArithmeticTarget8Bit::HLAddr), 16;
        0x97 => Res::new(2, ArithmeticTarget8Bit::A), 8;
        0x98 => Res::new(3, ArithmeticTarget8Bit::B), 8;
        0x99 => Res::new(3, ArithmeticTarget8Bit::C), 8;
        0x9A => Res::new(3, ArithmeticTarget8Bit::D), 8;
        0x9B => Res::new(3, ArithmeticTarget8Bit::E), 8;
        0x9C => Res::new(3, ArithmeticTarget8Bit::H), 8;
        0x9D => Res::new(3, ArithmeticTarget8Bit::L), 8;
        0x9E => Res::new(3, ArithmeticTarget8Bit::HLAddr), 16;
        0x9F => Res::new(3, ArithmeticTarget8Bit::A), 8;

        0xA0 => Res::new(4, ArithmeticTarget8Bit::B), 8;
        0xA1 => Res::new(4, ArithmeticTarget8Bit::C), 8;
        0xA2 => Res::new(4, ArithmeticTarget8Bit::D), 8;
        0xA3 => Res::new(4, ArithmeticTarget8Bit::E), 8;
        0xA4 => Res::new(4, ArithmeticTarget8Bit::H), 8;
        0xA5 => Res::new(4, ArithmeticTarget8Bit::L), 8;
        0xA6 => Res::new(4, ArithmeticTarget8Bit::HLAddr), 16;
        0xA7 => Res::new(4, ArithmeticTarget8Bit::A), 8;
        0xA8 => Res::new(5, ArithmeticTarget8Bit::B), 8;
        0xA9 => Res::new(5, ArithmeticTarget8Bit::C), 8;
        0xAA => Res::new(5, ArithmeticTarget8Bit::D), 8;
        0xAB => Res::new(5, ArithmeticTarget8Bit::E), 8;
        0xAC => Res::new(5, ArithmeticTarget8Bit::H), 8;
        0xAD => Res::new(5, ArithmeticTarget8Bit::L), 8;
        0xAE => Res::new(5, ArithmeticTarget8Bit::HLAddr), 16;
        0xAF => Res::new(5, ArithmeticTarget8Bit::A), 8;

        0xB0 => Res::new(6, ArithmeticTarget8Bit::B), 8;
        0xB1 => Res::new(6, ArithmeticTarget8Bit::C), 8;
        0xB2 => Res::new(6, ArithmeticTarget8Bit::D), 8;
        0xB3 => Res::new(6, ArithmeticTarget8Bit::E), 8;
        0xB4 => Res::new(6, ArithmeticTarget8Bit::H), 8;
        0xB5 => Res::new(6, ArithmeticTarget8Bit::L), 8;
        0xB6 => Res::new(6, ArithmeticTarget8Bit::HLAddr), 16;
        0xB7 => Res::new(6, ArithmeticTarget8Bit::A), 8;
        0xB8 => Res::new(7, ArithmeticTarget8Bit::B), 8;
        0xB9 => Res::new(7, ArithmeticTarget8Bit::C), 8;
        0xBA => Res::new(7, ArithmeticTarget8Bit::D), 8;
        0xBB => Res::new(7, ArithmeticTarget8Bit::E), 8;
        0xBC => Res::new(7, ArithmeticTarget8Bit::H), 8;
        0xBD => Res::new(7, ArithmeticTarget8Bit::L), 8;
        0xBE => Res::new(7, ArithmeticTarget8Bit::HLAddr), 16;
        0xBF => Res::new(7, ArithmeticTarget8Bit::A), 8;

        0xC0 => Set::new(0, ArithmeticTarget8Bit::B), 8;
        0xC1 => Set::new(0, ArithmeticTarget8Bit::C), 8;
        0xC2 => Set::new(0, ArithmeticTarget8Bit::D), 8;
        0xC3 => Set::new(0, ArithmeticTarget8Bit::E), 8;
        0xC4 => Set::new(0, ArithmeticTarget8Bit::H), 8;
        0xC5 => Set::new(0, ArithmeticTarget8Bit::L), 8;
        0xC6 => Set::new(0, ArithmeticTarget8Bit::HLAddr), 16;
        0xC7 => Set::new(0, ArithmeticTarget8Bit::A), 8;
        0xC8 => Set::new(1, ArithmeticTarget8Bit::B), 8;
        0xC9 => Set::new(1, ArithmeticTarget8Bit::C), 8;
        0xCA => Set::new(1, ArithmeticTarget8Bit::D), 8;
        0xCB => Set::new(1, ArithmeticTarget8Bit::E), 8;
        0xCC => Set::new(1, ArithmeticTarget8Bit::H), 8;
        0xCD => Set::new(1, ArithmeticTarget8Bit::L), 8;
        0xCE => Set::new(1, ArithmeticTarget8Bit::HLAddr), 16;
        0xCF => Set::new(1, ArithmeticTarget8Bit::A), 8;

        0xD0 => Set::new(2, ArithmeticTarget8Bit::B), 8;
        0xD1 => Set::new(2, ArithmeticTarget8Bit::C), 8;
        0xD2 => Set::new(2, ArithmeticTarget8Bit::D), 8;
        0xD3 => Set::new(2, ArithmeticTarget8Bit::E), 8;
        0xD4 => Set::new(2, ArithmeticTarget8Bit::H), 8;
        0xD5 => Set::new(2, ArithmeticTarget8Bit::L), 8;
        0xD6 => Set::new(2, ArithmeticTarget8Bit::HLAddr), 16;
        0xD7 => Set::new(2, ArithmeticTarget8Bit::A), 8;
        0xD8 => Set::new(3, ArithmeticTarget8Bit::B), 8;
        0xD9 => Set::new(3, ArithmeticTarget8Bit::C), 8;
        0xDA => Set::new(3, ArithmeticTarget8Bit::D), 8;
        0xDB => Set::new(3, ArithmeticTarget8Bit::E), 8;
        0xDC => Set::new(3, ArithmeticTarget8Bit::H), 8;
        0xDD => Set::new(3, ArithmeticTarget8Bit::L), 8;
        0xDE => Set::new(3, ArithmeticTarget8Bit::HLAddr), 16;
        0xDF => Set::new(3, ArithmeticTarget8Bit::A), 8;

        0xE0 => Set::new(4, ArithmeticTarget8Bit::B), 8;
        0xE1 => Set::new(4, ArithmeticTarget8Bit::C), 8;
        0xE2 => Set::new(4, ArithmeticTarget8Bit::D), 8;
        0xE3 => Set::new(4, ArithmeticTarget8Bit::E), 8;
        0xE4 => Set::new(4, ArithmeticTarget8Bit::H), 8;
        0xE5 => Set::new(4, ArithmeticTarget8Bit::L), 8;
        0xE6 => Set::new(4, ArithmeticTarget8Bit::HLAddr), 16;
        0xE7 => Set::new(4, ArithmeticTarget8Bit::A), 8;
        0xE8 => Set::new(5, ArithmeticTarget8Bit::B), 8;
        0xE9 => Set::new(5, ArithmeticTarget8Bit::C), 8;
        0xEA => Set::new(5, ArithmeticTarget8Bit::D), 8;
        0xEB => Set::new(5, ArithmeticTarget8Bit::E), 8;
        0xEC => Set::new(5, ArithmeticTarget8Bit::H), 8;
        0xED => Set::new(5, ArithmeticTarget8Bit::L), 8;
        0xEE => Set::new(5, ArithmeticTarget8Bit::HLAddr), 16;
        0xEF => Set::new(5, ArithmeticTarget8Bit::A), 8;

        0xF0 => Set::new(6, ArithmeticTarget8Bit::B), 8;
        0xF1 => Set::new(6, ArithmeticTarget8Bit::C), 8;
        0xF2 => Set::new(6, ArithmeticTarget8Bit::D), 8;
        0xF3 => Set::new(6, ArithmeticTarget8Bit::E), 8;
        0xF4 => Set::new(6, ArithmeticTarget8Bit::H), 8;
        0xF5 => Set::new(6, ArithmeticTarget8Bit::L), 8;
        0xF6 => Set::new(6, ArithmeticTarget8Bit::HLAddr), 16;
        0xF7 => Set::new(6, ArithmeticTarget8Bit::A), 8;
        0xF8 => Set::new(7, ArithmeticTarget8Bit::B), 8;
        0xF9 => Set::new(7, ArithmeticTarget8Bit::C), 8;
        0xFA => Set::new(7, ArithmeticTarget8Bit::D), 8;
        0xFB => Set::new(7, ArithmeticTarget8Bit::E), 8;
        0xFC => Set::new(7, ArithmeticTarget8Bit::H), 8;
        0xFD => Set::new(7, ArithmeticTarget8Bit::L), 8;
        0xFE => Set::new(7, ArithmeticTarget8Bit::HLAddr), 16;
        0xFF => Set::new(7, ArithmeticTarget8Bit::A), 8;
    })
}
