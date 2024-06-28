#[cfg(test)]
mod test{
    

    use std::collections::HashMap;

    use bpu::core::{
        enums::opcode::Opcode, instruction::Instruction, pipeline::Pipeline, predictors::{perceptron_predictor::PerceptronPredictor, predictor::BranchPredictor}
    };
    use bpu::utils::plot::{plot_series, plot_histogram};
    use bpu::utils::csv::read_csv_to_vector;
   


    const FILES: [&str; 6] = ["I04.csv", "INT03.csv", "MM03.csv", "MM05.csv", "S02.csv", "S04.csv"];
    const PARAMS: [usize; 6] = [15, 43, 40, 20, 24, 61];

    #[test]
    fn test_perceptron_all_datasets(){
            let mut plot_data:Vec<f64> = Vec::new();
            let mut series_data:HashMap<&str,Vec<u32>> = HashMap::new();

            for i in 0..6{

                let data = read_csv_to_vector("data/".to_owned()+FILES[i]).expect("Failed to parse CSV"); 

                let history_len = PARAMS[i]; 
                let threshold = (history_len as f64* 1.93 + 14.0).floor() as i32;
                let mut predictor = PerceptronPredictor::new(history_len, threshold); 
                    
                let mut mispredictions = 0; 
                for (pc, outcome) in &data{
                    
                    let instruction = Instruction::new(Opcode::BEQ, 0, 0, 0, *pc, None); 
                    let prediction = predictor.predict(&instruction);
                
                
                    if prediction != *outcome{
                        
                        mispredictions+=1; 
                    }
                    series_data.entry(FILES[i])
                    .and_modify(|v| v.push(mispredictions))
                    .or_insert_with(|| vec![mispredictions]);
                    predictor.update(&instruction, *outcome); 

                    predictor.update(&instruction, *outcome); 
                    
                }
                let prediction_rate = 100.0 - (mispredictions  as f64/ data.len() as f64)*100.0;
                plot_data.push(prediction_rate);
                println!("{} : {:.2}% prediction rate, mispredictions = {}",FILES[i], prediction_rate, mispredictions);
                    
            }
            _ = plot_histogram(plot_data);
            _ = plot_series(series_data);
          
        
     
    }
   
    #[test]
    fn test_perceptron_all_datasets_series(){
        let mut plot_data:HashMap<&str,Vec<u32>> = HashMap::new();

        for i in 0..6{
            let data = read_csv_to_vector("data/".to_owned()+FILES[i]).expect("Failed to parse CSV"); 

            let history_len = PARAMS[i]; 
            let threshold = (history_len as f64* 1.93 + 14.0).floor() as i32;
            let mut predictor = PerceptronPredictor::new(history_len, threshold); 
                
            let mut mispredictions = 0; 
            for (pc, outcome) in &data{
                
                let instruction = Instruction::new(Opcode::BEQ, 0, 0, 0, *pc, None); 
                let prediction = predictor.predict(&instruction);
            
            
                if prediction != *outcome{
                    
                    mispredictions+=1; 
                
                }
                plot_data.entry(FILES[i])
                .and_modify(|v| v.push(mispredictions))
                .or_insert_with(|| vec![mispredictions]);
                predictor.update(&instruction, *outcome); 
                
            }
            let prediction_rate = 100.0 - (mispredictions  as f64/ data.len() as f64)*100.0;
            println!("{} : {:.2}% prediction rate, mispredictions = {}",FILES[0], prediction_rate, mispredictions);
        }
        
        _ = plot_series(plot_data);
    }
    #[test]
    fn test_perceptron_predictor_parameters(){     
        for f in FILES{
            let file_path = "data/".to_owned()+f; 
            let data = read_csv_to_vector(file_path).expect("Failed to parse CSV"); 

            let mut best_i = 0; 
            let mut best_j = 0;
            let mut least_mispredictions = usize::MAX;

            for i in 12..=62{
                    let threshold = (i as f64 * 1.93 +14.0).floor() as i32; 
                    let mut predictor = PerceptronPredictor::new(i, threshold); 
                
                    let mut mispredictions = 0; 
                    for (pc, outcome) in &data{
                        
                        let instruction = Instruction::new(Opcode::BEQ, 0, 0, 0, *pc, None); 
                        let prediction = predictor.predict(&instruction);
                    
                    
                        if prediction != *outcome{
                            
                            mispredictions+=1; 
                        }
                        predictor.update(&instruction, *outcome); 
                        
                    }
                    if mispredictions < least_mispredictions{
                        least_mispredictions = mispredictions; 
                        best_i = i;
                        best_j = threshold;
                    }
            }      
            println!("history_length = {}, threshold = {} with {} mispredictions", best_i, best_j, least_mispredictions);

        }
    }


    #[test]
    fn test_perceptron_in_pipeline() {
        let instruction_memory = vec![
            Instruction::new(Opcode::BEQ, 1, 2, 0, 2, None), 
            Instruction::new(Opcode::ADDI, 3, 0, 3, 0, Some(5)), 
            Instruction::new(Opcode::BEQ, 9, 10, 0, 10, None),
            Instruction::new(Opcode::ADDI, 17, 0, 17, 0, Some(13)), 
        ];
        let predictor = PerceptronPredictor::new(28, 56);
       

        let mut pipeline = Pipeline::new(instruction_memory.clone(), 512, 32, Box::new(predictor));   
   
        pipeline.register_file[1] = 10;
        pipeline.register_file[2] = 11;
        pipeline.register_file[9] = 1;  
        pipeline.register_file[10] = 1;

        let result = pipeline.start();
        println!("{}", result.mispredictions);
   
        assert!(pipeline.register_file[3] == 5);
        assert!(pipeline.register_file[17] == 0)


    }
}