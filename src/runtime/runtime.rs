use std::collections::HashMap;
use crate::common::{data_type::DataType, executable::ExecutableLine};
use crate::common::errors::Result;

use super::builtin_function::execute;


pub struct Runtime{
    pub executables: Vec<ExecutableLine>, 
    pub variables: HashMap<String,DataType>, // <variable name, variable value>
    pub tags: HashMap<String, u32>, // <tag name, index in executables vector>
    pub current_line: usize,
    pub std_out: fn(&str),
    pub std_in: fn() -> String,
}

impl Runtime{
    
    fn new(std_out: fn(&str), std_in: fn() -> String) -> Self{
        Self {
            executables: vec![],
            variables: HashMap::new(),
            tags: HashMap::new(),
            current_line: 0,
            std_out,
            std_in,
        }
    }

    fn on_new_line(&mut self, line: ExecutableLine){
        self.executables.push(line);
    }

    fn execution_cycle(&mut self) -> Result<()>{

        match self.executables.get((&self).current_line) {
            Some(l) => {
                let l = l.clone();
                execute(self, &l)?;
                self.current_line+=1;
            },
            None => {
                //do nothing and wait for more executables
            },
        }
        Ok(())
    }

    pub fn std_out(&self,msg:&str){
        (self.std_out)(msg);
    }

    pub fn std_in(&self) -> String{
         (self.std_in)()
    }
}


#[cfg(test)]
mod tests{

    use crate::common::{executable::ExecutableLine, param::Param, data_type::DataType};

    use super::Runtime;

    #[test]
    fn simple_executaion_test(){
        
        let mut rt = Runtime::new(|_|{},||{"".to_string()});

        assert_eq!(rt.current_line, 0);
        rt.on_new_line(ExecutableLine::new(
            1,
            "assign".to_string(),
            vec![Param::Value(DataType::Int(2))],
            Some("my_variable".to_string())
        ));
        assert_eq!(rt.current_line, 0);

        rt.execution_cycle().unwrap();
        assert_eq!(rt.current_line, 1);

        assert_eq!(
            rt.variables.get("my_variable"),
            Some(&DataType::Int(2))
        )
    }

    #[test]
    fn runtime_std_test(){

        let mut rt = Runtime::new(|x|{
            assert_eq!(x, "the text");
        },||{
            "the text".to_string()
        });

        rt.on_new_line(ExecutableLine { 
            line_number: 1,
            function_name: "input".to_string(),
            params: vec![],
            output_var: Some("name".to_string()) 
        });
        rt.execution_cycle().unwrap();

        rt.on_new_line(ExecutableLine { 
            line_number: 2,
            function_name: "println".to_string(),
            params: vec![Param::Variable("name".to_string())],
            output_var: None
        });
        rt.execution_cycle().unwrap();

    }
}