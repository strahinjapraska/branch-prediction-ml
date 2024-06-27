pub mod core{
    pub mod instruction;
    pub mod pipeline;
    pub mod result;

    pub mod enums{
        pub mod opcode;
    }
    pub mod predictors{
        pub mod perceptron_predictor;
        pub mod predictor;
        pub mod static_predictor;
    }
    pub mod registers{
        pub mod if_id_register; 
        pub mod id_ex_register;
        pub mod ex_mem_register;
        pub mod mem_wb_register;
    } 
}
pub mod utils{
    pub mod plot;
    pub mod csv; 
}
