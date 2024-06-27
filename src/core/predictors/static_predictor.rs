use crate::core::instruction::Instruction;
use super::predictor::BranchPredictor;

pub struct StaticPredictor;

impl BranchPredictor for StaticPredictor{
    fn predict(&mut self, _instruction: &Instruction) -> bool{
        false 
    }
    
    fn update(&mut self, _: &Instruction, _: bool) {
        
    }
}