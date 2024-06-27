use crate::core::instruction::Instruction;

pub trait BranchPredictor{
    fn predict(&mut self, instruction: &Instruction) -> bool;
    fn update(&mut self, instruction: &Instruction, outcome: bool);
}