use crate::core::instruction::Instruction;
use crate::core::registers::{if_id_register::*, id_ex_register::*, ex_mem_register::*, mem_wb_register::*}; 
use crate::core::enums::opcode::Opcode;
use crate::core::result::PipelineResult;

use super::predictors::predictor::BranchPredictor;

pub struct Pipeline{
    pub pc: usize, 
    flush_flag: bool,  
    instruction_memory: Vec<Instruction>, 

    pub register_file: Vec<i32>, 
    pub memory: Vec<i32>, 

    predictor: Box<dyn BranchPredictor>
    
}
impl Pipeline{
    
    fn instruction_fetch(&mut self, if_id: &mut IfIdRegister){
        if self.pc < self.instruction_memory.len(){
            let instruction = self.instruction_memory[self.pc].clone(); 
            if_id.set_instruction(Some(instruction));
            if_id.set_old_pc(self.pc);
            if *(instruction.opcode()) == Opcode::BEQ && self.predictor.predict(&instruction){
                self.pc =  *instruction.address() as usize;        
            }else{
                self.pc = self.pc +1;
            }
            if_id.set_pc(self.pc);
        }else{
            if_id.set_instruction(None);
        }

    }

    fn instruction_decode(&mut self, if_id: &IfIdRegister, id_ex: &mut IdExRegister){

        if let Some(instruction) = if_id.instruction(){
            id_ex.set_instruction(Some(*instruction));
            id_ex.set_operand1_value(self.register_file[*instruction.operand1()]);
            id_ex.set_operand2_value(self.register_file[*instruction.operand2()]); 
            id_ex.set_destination(*(instruction.destination()));   
            id_ex.set_pc(*(if_id.pc()));
            id_ex.set_old_pc(*(if_id.old_pc()));
        }else{
            id_ex.set_instruction(None);
        }

    }


    fn execute(&mut self,id_ex: &IdExRegister, ex_mem: &mut ExMemRegister){

        if let Some(instruction) = id_ex.instruction(){
            ex_mem.set_instruction(Some(*instruction)); 
            ex_mem.set_result(match *(instruction.opcode()){
                Opcode::ADD => {
                    id_ex.operand1_value() + id_ex.operand2_value()
                }
                Opcode::ADDI => id_ex.operand1_value() + instruction.constant().unwrap_or(0), 
                Opcode::SUB => id_ex. operand1_value() - id_ex.operand2_value(), 
                Opcode::BEQ => 
                {
                    let branch_taken = id_ex.operand1_value() == id_ex.operand2_value();
                    self.predictor.update(instruction, branch_taken);
                    if branch_taken && *id_ex.pc() != *instruction.address(){
                        self.pc = *instruction.address();
                        self.flush_flag = true;   
                    }else if !branch_taken && *id_ex.pc() == *instruction.address(){
                        self.pc = *(id_ex.old_pc());
                        self.flush_flag = true; 
                    }

                    branch_taken as i32
                    
                }, 
                _ => 0, 
            }); 
            ex_mem.set_destination(*(id_ex.destination()));
        }else{
            ex_mem.set_instruction(None);
        }
    }


    fn memory_access(&mut self, ex_mem: &ExMemRegister, mem_wb: &mut MemWbRegister){
        if let Some(instruction) = ex_mem.instruction(){
            mem_wb.set_instruction(Some(*instruction)); 
            mem_wb.set_result(match *(instruction.opcode()){
                Opcode::LOAD => self.memory[*(ex_mem.result()) as usize], 
                Opcode::STORE => {
                        self.memory[*instruction.address() as usize] = self.register_file[*(ex_mem.destination())];
                        0
                }, 
                _ => *(ex_mem.result()), 
            });
            mem_wb.set_destination(*(ex_mem.destination()));
        }else{
            mem_wb.set_instruction(None);
        }
    }

    fn write_back(&mut self, mem_wb: &mut MemWbRegister){
       
        if let Some(instruction) = mem_wb.instruction(){
        
            if *(instruction.opcode()) != Opcode::STORE && *(instruction.opcode()) != Opcode::BEQ && *(instruction.opcode()) != Opcode::LOAD{
                self.register_file[*(mem_wb.destination())] = *(mem_wb.result());
            }else if *(instruction.opcode()) == Opcode::LOAD{
                self.register_file[*(mem_wb.destination())] = self.memory[*instruction.address() as usize];
            }
        }else{
            mem_wb.set_instruction(None);
        }

    }

    pub fn new(instruction_memory: Vec<Instruction>, memory_size: usize, register_count: usize, predictor: Box<dyn BranchPredictor>) -> Self{
        Pipeline{
            pc: 0,
            flush_flag: false, 
            instruction_memory, 
            register_file: vec![0; register_count], 
            memory: vec![0; memory_size], 
            predictor
        }
    }

    pub fn start(&mut self) -> PipelineResult{
        let mut if_id = IfIdRegister::new(); 
        let mut id_ex = IdExRegister::new(); 
        let mut ex_mem =  ExMemRegister::new(); 
        let mut mem_wb = MemWbRegister::new();

        let mut cycle = 0; 
        let mut mispredictions = 0; 

        if self.instruction_memory.is_empty(){
            return PipelineResult{cycles: 0, mispredictions: 0}; 
        }

        loop{

            if if_id.instruction().is_none() && id_ex.instruction().is_none() && ex_mem.instruction().is_none() && mem_wb.instruction().is_none() && cycle!=0{
                break;
            }
                  
            Self::write_back(self,&mut mem_wb);
            Self::memory_access(self,&ex_mem, &mut mem_wb); 

            if self.flush_flag{
              
                if_id = IfIdRegister::new(); 
                id_ex = IdExRegister::new(); 
                ex_mem =  ExMemRegister::new(); 
                self.flush_flag = false; 
                mispredictions+=1;
            }

            Self::execute(self,&id_ex, &mut ex_mem); 
            Self::instruction_decode(self,&if_id, &mut id_ex); 
            Self::instruction_fetch(self,&mut if_id); 
    
            cycle+=1; 
        }
        PipelineResult{cycles:cycle, mispredictions}
        

    }
}