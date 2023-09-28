// use crate::{runtime::runtime::Runtime, common::{data_type::DataType, errors::ChapError, param::Param}};
// use crate::common::errors::Result;


// pub fn get_var(runtime: &Runtime, name: &str) -> Result<DataType>{

//     match runtime.variables.get(name){
//         Some(x) => Ok(x.clone()),
//         None => return Err(
//             ChapError::runtime_with_msg(0, "".to_string())
//         ),
//     }
// }

// pub fn param_to_datatype<'a>(runtime: &Runtime, param: &'a Param) -> Result<&'a DataType>{

//     match param {
//         Param::Tag(name) => return Err(ChapError::runtime_with_msg(0, "cant convert a tag to datatype".to_string())),
//         Param::Value(value) => return Ok(value),
//         Param::Variable(name) => match runtime.variables.get(name){
//             Some(x) => return Ok(&x.clone()),
//             None => return Err(
//                 ChapError::runtime_with_msg(0, "".to_string())
//             ),
//         },
//     }
    
// }
