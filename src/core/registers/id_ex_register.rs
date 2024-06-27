use getset::{Getters, Setters};

use crate::core::instruction::Instruction;

#[derive(Getters, Setters)]
#[getset(get = "pub", set = "pub")]
pub struct IdExRegister{
    instruction: Option<Instruction>, 
    operand1_value: i32, 
    operand2_value: i32, 
    destination: usize, 
    pc: usize, 
    old_pc: usize, 
}
impl IdExRegister{
    pub fn new() -> Self{
        IdExRegister{instruction: None, operand1_value: 0, operand2_value: 0, destination: 0, pc: 0, old_pc: 0}
    }
}