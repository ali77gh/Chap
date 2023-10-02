use crate::common::data_type::DataType;
use crate::runtime::builtin_function::utils::returns;
use crate::{runtime::Runtime, common::executable::ExecutableLine};
use crate::common::errors::Result;


pub fn input(runtime: &mut Runtime, executable: &ExecutableLine)-> Result<()>{

    let inp = runtime.std_in();
    returns(runtime, executable, DataType::String(inp))
}


#[cfg(test)]
mod tests{
    use crate::{runtime::Runtime, common::{executable::ExecutableLine, data_type::DataType}};

    use super::input;


    #[test]
    fn input_test(){

        let mut runtime = Runtime::new(|_|{}, ||{ "test".to_string() });

        input(
            &mut runtime,
            &ExecutableLine::new(
                1,
                "".to_string(),
                vec![],
                Some("test_var".to_string())
            )
        ).unwrap();

        assert_eq!(
            runtime.variables.get("test_var").unwrap(),
            &DataType::String("test".to_string())
        )
    }
}