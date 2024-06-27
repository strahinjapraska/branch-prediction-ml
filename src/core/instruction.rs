use getset::{Getters, Setters};

use crate::core::enums::opcode::Opcode;


#[derive(Clone,Copy, Getters, Setters)]
#[getset(get = "pub", set = "pub")]
pub struct Instruction{
    opcode: Opcode, 
    operand1: usize, 
    operand2: usize, 
    destination: usize, 
    address: usize, 
    constant: Option<i32>, 
}

impl Instruction{
    pub fn new(opcode:Opcode, operand1:usize, operand2:usize, destination:usize, address:usize, constant: Option<i32>) -> Self{
        Instruction{opcode, operand1, operand2, destination, address, constant}
    }
}