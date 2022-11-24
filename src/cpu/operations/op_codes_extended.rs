use super::operation::Operation;

use super::nop::Nop;

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
        0x00 => Nop, 4;
    })
}
