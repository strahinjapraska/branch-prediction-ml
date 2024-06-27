use getset::{Getters, Setters};

use crate::core::instruction::Instruction;


#[derive(Getters, Setters)]
#[getset(get = "pub", set = "pub")]
pub struct IfIdRegister{
    instruction: Option<Instruction>,  
    pc: usize,
    old_pc: usize, 
}
impl IfIdRegister{
    pub fn new() -> Self{
        IfIdRegister{instruction: None, pc: 0, old_pc: 0}
    }
}