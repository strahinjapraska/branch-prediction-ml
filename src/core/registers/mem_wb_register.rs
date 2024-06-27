use getset::{Getters, Setters};

use crate::core::instruction::Instruction;

#[derive(Getters, Setters)]
#[getset(get = "pub", set = "pub")]
pub struct MemWbRegister{
    instruction: Option<Instruction>, 
    result: i32, 
    destination: usize 
}

impl MemWbRegister{
    pub fn new() -> Self{
        MemWbRegister{instruction: None, result: 0, destination: 0}
    }
}
