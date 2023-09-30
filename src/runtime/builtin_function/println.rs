use crate::{runtime::runtime::Runtime, common::executable::ExecutableLine};
use crate::runtime::builtin_function::utils::param_to_datatype;
use crate::common::errors::Result;


pub fn println(runtime: &mut Runtime, executable: &ExecutableLine)-> Result<()>{

    let mut result: Vec<String> = Vec::new();
    for param in &executable.params {
        let dt = param_to_datatype(runtime, Some(param), executable.line_number)?;
        result.push(dt.to_string());
    }
    runtime.std_out(result.join(", ").as_str());
    
    Ok(())
}


#[cfg(test)]
mod tests{

    use crate::common::data_type::DataType;
    use crate::common::param::Param;
    use super::*;

    #[test]
    fn print_string(){
        let mut runtime = Runtime::new(|x|{
            assert_eq!(x, "test");
        }, ||{ "".to_string() });


        println(
            &mut runtime,
            &ExecutableLine::new(1,
                "".to_string(),
                vec![Param::Value(DataType::String("test".to_string()))],
                None
            )
        ).unwrap();
    }

    #[test]
    fn print_int(){
        let mut runtime = Runtime::new(|x|{
            assert_eq!(x, "2");
        }, ||{ "".to_string() });

        println(
            &mut runtime,
            &ExecutableLine::new(1,
                "".to_string(),
                vec![Param::Value(DataType::Int(2))],
                None
            )
        ).unwrap();
    }

    #[test]
    fn print_many(){
        let mut runtime = Runtime::new(|x|{
            assert_eq!(x, "2, false");
        }, ||{ "".to_string() });

        println(
            &mut runtime,
            &ExecutableLine::new(1,
                "".to_string(),
                vec![Param::Value(DataType::Int(2)),Param::Value(DataType::Bool(false))],
                None
            )
        ).unwrap();
    }
}