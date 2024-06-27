#[cfg(test)]
mod test{
    use bpu::core::pipeline::Pipeline;
    use bpu::core::instruction::Instruction;
    use bpu::core::enums::opcode::Opcode;
    use bpu::core::predictors::static_predictor::StaticPredictor;

    #[test]
    fn test_addi(){

        let instruction_memory = vec![
            Instruction::new(Opcode::ADDI, 1, 0, 1, 0, Some(10)),
        ]; 

        let mut pipeline = Pipeline::new(instruction_memory, 512, 32, Box::new(StaticPredictor));
        pipeline.start();

        assert!(pipeline.register_file[1] == 10)
    }
    #[test]
    fn test_add(){

        let instruction_memory = vec![
            Instruction::new(Opcode::ADD, 1, 2, 3, 0, None)
        ]; 

        let mut pipeline = Pipeline::new(instruction_memory, 512, 32, Box::new(StaticPredictor));
        pipeline.register_file[1] = 10; 
        pipeline.register_file[2] = 20; 
        pipeline.start();

        assert!(pipeline.register_file[3] == 30)

    }
    #[test]
    fn test_sub(){

        let instruction_memory = vec![
            Instruction::new(Opcode::SUB, 1, 2, 3, 0, None)
        ];

        let mut pipeline = Pipeline::new(instruction_memory, 512, 32, Box::new(StaticPredictor));
        pipeline.register_file[1] = 31; 
        pipeline.register_file[2] = 10; 
        pipeline.start();

        assert!(pipeline.register_file[3] == 21)

    }

    #[test]
    fn test_load(){

        let instruction_memory = vec![
            Instruction::new(Opcode::LOAD, 0, 0, 1, 9, None)
        ];

        let mut pipeline = Pipeline::new(instruction_memory, 512, 32, Box::new(StaticPredictor)); 
        pipeline.memory[9] = 42; 
        pipeline.start(); 

        assert!(pipeline.register_file[1] == 42)
    }

    #[test]
    fn test_store(){
        
        let instruction_memory = vec![
            Instruction::new(Opcode::STORE, 0, 0, 1, 10, None)
        ];

        let mut pipeline = Pipeline::new(instruction_memory, 512, 32, Box::new(StaticPredictor)); 
        pipeline.register_file[1] = 53; 
        pipeline.start(); 

        assert!(pipeline.memory[10] == 53)
    }

    #[test]
    fn test_beq(){

        let instruction_memory = vec![
            Instruction::new(Opcode::BEQ, 1, 2, 0, 2, None), 
            Instruction::new(Opcode::ADDI, 3, 0, 3, 0, Some(5)), 
            Instruction::new(Opcode::ADD, 0, 0, 0 , 0, None)
        ];
        
        let mut pipeline = Pipeline::new(instruction_memory.clone(), 512, 32, Box::new(StaticPredictor));   
        pipeline.register_file[1] = 10; 
        pipeline.register_file[2] = 10; 
        let result = pipeline.start();

        assert!(result.mispredictions == 1);
        assert!(pipeline.register_file[3] == 0);

        pipeline = Pipeline::new(instruction_memory, 512, 32, Box::new(StaticPredictor)); 
        pipeline.register_file[1] = 10; 
        pipeline.register_file[2] = 2; 
        let result = pipeline.start();
       
        assert!(result.mispredictions == 0);
        assert!(pipeline.register_file[3] == 5);

    }

}