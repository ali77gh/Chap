use std::collections::HashMap;

use crate::common::{data_type::DataType, executable::ExecutableLine};
use crate::common::errors::Result;

use super::builtin_function::execute;



pub struct Runtime{
    pub executables: Vec<ExecutableLine>, 
    pub variables: HashMap<String,DataType>, // <variable name, variable value>
    pub tags: HashMap<String, u32>, // <tag name, index in executables vector>
    pub current_line: u32,
}

impl Runtime {
    
    fn new() -> Self{
        Self {
            executables: vec![],
            variables: HashMap::new(),
            tags: HashMap::new(),
            current_line: 0u32,
        }
    }

    fn on_new_line(&mut self, line: ExecutableLine){
        self.executables.push(line);
    }

    fn execution_cycle(&mut self) -> Result<()>{

        // let cr: u32 = (&self).current_line;
        let l = self.executables.get(0).unwrap().clone();

        execute(self, &l)?;
        
        self.current_line+=1;
        Ok(())
    }

}