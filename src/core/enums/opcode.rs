#[derive(Clone,Copy, PartialEq)]
pub enum Opcode{
    ADD, 
    ADDI,
    SUB, 
    LOAD, 
    STORE, 
    BEQ,
}