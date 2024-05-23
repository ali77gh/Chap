use crate::common::data_type::DataType;
use crate::common::errors::Result;
use crate::common::executable::ExecutableLine;
use std::collections::HashMap;

pub struct Runtime {
    pub executables: Vec<ExecutableLine>,
    pub variables: HashMap<String, DataType>, // <variable name, variable value>
    pub tags: HashMap<String, usize>,         // <tag name, index in executables vector>
    pub current_line: usize,
    pub std_out: fn(&str),
    pub std_in: fn() -> String,
}

impl Runtime {
    pub fn new(std_out: fn(&str), std_in: fn() -> String) -> Self {
        Self {
            executables: vec![],
            variables: HashMap::new(),
            tags: HashMap::new(),
            current_line: 0,
            std_out,
            std_in,
        }
    }

    pub fn on_new_line(&mut self, mut line: ExecutableLine) -> Result<()> {
        line.bind_closure()?;
        self.executables.push(line);
        Ok(())
    }

    /// # Safety
    /// this function is not checking if there is more lines or not (for better performance)
    /// caller should check to avoid segmentation fault
    /// eval function adds an exit function call to end of source code to avoid checking next line exist at every line execution
    /// repl is checking next line exist itself
    pub unsafe fn execution_cycle(&mut self) -> Result<()> {
        let l = self.executables.get(self.current_line).unwrap_unchecked(); // unsafe
        let l = l.clone();
        self.current_line += 1;
        (l.closure)(self, &l)?;
        Ok(())
    }

    pub fn std_out(&self, msg: &str) {
        (self.std_out)(msg);
    }

    pub fn std_in(&self) -> String {
        (self.std_in)()
    }
}

#[cfg(test)]
mod tests {

    use crate::common::{data_type::DataType, executable::ExecutableLine, param::Param};

    use super::Runtime;

    #[test]
    fn simple_execution_test() {
        let mut rt = Runtime::new(|_| {}, || "".to_string());

        assert_eq!(rt.current_line, 0);
        rt.on_new_line(ExecutableLine::new(
            1,
            "assign".to_string(),
            vec![Param::Value(DataType::Int(2))],
            Some("my_variable".to_string()),
        ))
        .unwrap();
        assert_eq!(rt.current_line, 0);

        unsafe { rt.execution_cycle().unwrap() };
        assert_eq!(rt.current_line, 1);

        assert_eq!(rt.variables.get("my_variable"), Some(&DataType::Int(2)))
    }

    #[test]
    fn runtime_std_test() {
        let mut rt = Runtime::new(
            |x| {
                assert_eq!(x, "the text");
            },
            || "the text".to_string(),
        );

        rt.on_new_line(ExecutableLine::new(
            1,
            "input".to_string(),
            vec![],
            Some("name".to_string()),
        ))
        .unwrap();
        unsafe { rt.execution_cycle().unwrap() };

        rt.on_new_line(ExecutableLine::new(
            2,
            "print".to_string(),
            vec![Param::Variable("name".to_string())],
            None,
        ))
        .unwrap();
        unsafe { rt.execution_cycle().unwrap() };
    }
}
