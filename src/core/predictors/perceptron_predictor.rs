use std::collections::HashMap;

use crate::core::instruction::Instruction;

use super::predictor::BranchPredictor;

pub struct PerceptronPredictor{
    perceptrons: HashMap<usize, Vec<i32>>, 
    global_history: Vec<i32>, 
    history_len : usize, 
    threshold: i32 

}
impl PerceptronPredictor{
    pub fn new(history_len: usize, threshold: i32) -> Self{
        PerceptronPredictor{
            perceptrons: HashMap::new(), 
            global_history: vec![0; history_len], 
            history_len, 
            threshold
        }
    }
    
    fn get_perceptrons(&mut self, address: usize) -> &mut Vec<i32>{
        self.perceptrons.entry(address).or_insert_with(||vec![0; self.history_len + 1])
    }
    
    fn update_global_history(&mut self, outcome: bool){
        self.global_history.pop(); 
        self.global_history.insert(0, if outcome {1} else {-1});
     
    }

    fn caculate_output(&self, perceptron: &Vec<i32>) -> i32{
        let bias = perceptron[0];

        let weighted_sum = self.global_history.iter()
        .zip(&perceptron[1..])
        .map(|(h, w)| h*w)
        .sum::<i32>();

      
      
        bias+weighted_sum
    }

    fn train_perceptron(&mut self, perceptron: &mut Vec<i32>, outcome: bool){
        let t = if outcome {1} else {-1}; 
        let y  = self.caculate_output(perceptron); 
        if y*t <= self.threshold{
            perceptron[0] += t; 
            for i in 0..self.history_len{
                perceptron[i+1] += t*self.global_history[i];
            }
        }
    }

}

impl BranchPredictor for PerceptronPredictor{
    fn predict(&mut self, instruction: &Instruction) -> bool {
        let address = *instruction.address();
        let perceptron = self.get_perceptrons(address); 
        let perceptron_clone = perceptron.clone();
        let res = self.caculate_output(&perceptron_clone);
        res>=0
    }
    
    fn update(&mut self, instruction: &Instruction, outcome: bool) {
        let address = *instruction.address();
        let perceptron = self.get_perceptrons(address);
        let mut perceptron_clone = perceptron.clone();
        self.train_perceptron(&mut perceptron_clone, outcome);
        self.perceptrons.insert(address, perceptron_clone);
        self.update_global_history(outcome);
    }
}