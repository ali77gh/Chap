use crate::common::data_type::DataType;
use crate::common::errors::Result;
use crate::common::executable::ExecutableLine;
use rustc_hash::FxHashMap;

pub struct Runtime<'a> {
    pub executables: Vec<ExecutableLine>,
    pub variables: FxHashMap<String, DataType>, // <variable name, variable value>
    pub tags: FxHashMap<String, usize>,         // <tag name, index in executables vector>
    pub current_line: usize,
    pub std_out: Box<dyn FnMut(&str) + 'a>,
    pub std_in: Box<dyn FnMut() -> String + 'a>,
}

impl<'a> Runtime<'a> {
    pub fn new(
        std_out: Box<dyn FnMut(&str) + 'a>,
        std_in: Box<dyn FnMut() -> String + 'a>,
    ) -> Self {
        Self {
            executables: vec![],
            variables: FxHashMap::default(),
            tags: FxHashMap::default(),
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
    ///
    /// # Invariant
    /// builtin functions must never push/remove from `executables` or take `&mut`
    /// into it while executing (`on_new_line` only runs at load time), so the
    /// pointer below stays valid for the whole call and no clone is needed.
    pub unsafe fn execution_cycle(&mut self) -> Result<()> {
        let ptr: *const ExecutableLine = self.executables.get_unchecked(self.current_line);
        let closure = (*ptr).closure;
        self.current_line += 1;
        (closure)(self, &*ptr)
    }

    pub fn std_out(&mut self, msg: &str) {
        (self.std_out)(msg);
    }

    pub fn std_in(&mut self) -> String {
        (self.std_in)()
    }
}

#[cfg(test)]
mod tests {

    use crate::common::{data_type::DataType, executable::ExecutableLine, param::Param};

    use super::Runtime;

    #[test]
    fn simple_execution_test() {
        let mut rt = Runtime::new(Box::new(|_| {}), Box::new(|| "".to_string()));

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
            Box::new(|x| {
                assert_eq!(x, "the text");
            }),
            Box::new(|| "the text".to_string()),
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
