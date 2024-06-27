use std::{error::Error, fs::File, path::Path};



pub fn read_csv_to_vector<P: AsRef<Path>>(path: P) -> Result<Vec<(usize, bool)>, Box<dyn Error>> {
    let file = File::open(path)?;
    let mut rdr = csv::Reader::from_reader(file);
    let mut result = Vec::new(); 

    for record in rdr.records(){
        let record = record?;
        
        let pc: usize = record.get(0).unwrap().trim().parse()?; 
        
        let outcome_str = record.get(1).unwrap().trim(); 
        let outcome: bool = match outcome_str{
            "0" => false, 
            "1" => true,
            _ => return Err("Invalid boolean value".into())
        };
        result.push((pc,outcome));
    }
    Ok(result)
}