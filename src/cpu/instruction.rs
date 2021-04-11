pub enum Instruction {
    ADD(ArithmeticTarget),
}

// target register for arithmetic (F is reserved for flags)
pub enum ArithmeticTarget {
    A, B, C, D, E, H, L,
}
