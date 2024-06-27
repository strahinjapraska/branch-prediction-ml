use getset::{Getters, Setters};

use crate::core::instruction::Instruction;

#[derive(Getters, Setters)]
#[getset(get = "pub", set = "pub")]
pub struct ExMemRegister{
    instruction: Option<Instruction>,
    result: i32, 
    destination: usize 
}

impl ExMemRegister{
    pub fn new() -> Self{
        ExMemRegister{instruction: None, result: 0, destination: 0}
    }
}