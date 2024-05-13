use crate::common::data_type::DataType;
use crate::common::errors::Result;
use crate::builtin_function::utils::returns;
use crate::{common::executable::ExecutableLine, runtime::Runtime};

pub fn input(runtime: &mut Runtime, executable: &ExecutableLine) -> Result<()> {
    let inp = runtime.std_in();
    returns(runtime, executable, DataType::String(inp))
}

#[cfg(test)]
mod tests {
    use crate::{
        common::{data_type::DataType, executable::ExecutableLine},
        runtime::Runtime,
    };

    use super::input;

    #[test]
    fn input_test() {
        let mut runtime = Runtime::new(|_| {}, || "test".to_string());

        input(
            &mut runtime,
            &ExecutableLine::new(1, "".to_string(), vec![], Some("test_var".to_string())),
        )
        .unwrap();

        assert_eq!(
            runtime.variables.get("test_var").unwrap(),
            &DataType::String("test".to_string())
        )
    }
}
