pub const NONE:       u8 =  0;
pub const ASSIGNMENT: u8 =  1; // =
pub const OR:         u8 =  2; // or
pub const AND:        u8 =  3; // and
pub const EQUALITY:   u8 =  4; // == !=
pub const COMPARISON: u8 =  5; // < > <= >=
pub const TERM:       u8 =  6; // + -
pub const FACTOR:     u8 =  7; // * /
pub const UNARY:      u8 =  8; // ! -
pub const CALL:       u8 =  9; // . ()
pub const PRIMARY:    u8 = 10;