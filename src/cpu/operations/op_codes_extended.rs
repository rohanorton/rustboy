use super::operation::Operation;
use super::targets::ArithmeticTarget8Bit;

use super::rlc::Rlc;
use super::rrc::Rrc;

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
    })
}
