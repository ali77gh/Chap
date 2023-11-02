
mod chunk_detector;
mod get_single_var;

use crate::common::param::Param;
use crate::parser::get_single_var::get_single_var;
use crate::parser::chunk_detector::chunk_detector;
use crate::common::chunk::Chunk;
use crate::common::{line_of_code::LineOfCode, executable::ExecutableLine};
use crate::common::errors::{Result, ChapError};
use crate::common::splitter::string_safe_split;

#[derive(Default)]
pub struct Parser{
    tmp_var_counter: usize
}
    
impl Parser {

    pub fn on_new_line(&mut self, new_line: LineOfCode) -> Result<Vec<ExecutableLine>>{

        let ln = new_line.line_number;

        let chunks = string_safe_split(&new_line.code, "->".to_string());
        let chunks: Vec<Result<Chunk>>= chunks.iter().map(|x|{chunk_detector(x.to_string(), ln)}).collect();

        let mut result = Vec::<ExecutableLine>::new();
        let mut pointer = 0usize;

        loop{

            let pc= if pointer==0 { None } else{ chunks.get(pointer - 1) };
            let cc= chunks.get(pointer).unwrap().clone()?;
            let nc= chunks.get(pointer + 1);
            pointer+=1;

            if pointer==1{ // first
                match cc{
                    Chunk::Params(cc) => {
                        match nc {
                            None => {   // nothing -> param -> nothing
                                match cc.get(0) {
                                    None => return Err(ChapError::syntax_with_msg(ln,"nothing to execute".to_string())),
                                    Some(cc1) => {
                                        match cc1 {
                                            Param::Tag(_) => { 
                                                if cc.len() > 1{
                                                    return Err(ChapError::syntax_with_msg(ln, "<tag>,<param>... does not mean anything".to_string()));
                                                }
                                                result.push(ExecutableLine::new(ln, "new_tag".to_string(), cc, None)) 
                                            },
                                            _ => { 
                                                result.push(ExecutableLine::new(ln, "print".to_string(), cc, None)) 
                                            },
                                        }
                                    },
                                }
                                break;
                            },
                            Some(nc) => { 
                                let nc = nc.clone()?;
                                match nc {
                                    Chunk::Params(nc) => {// nothing -> param -> param
                                        let nc = get_single_var(nc,ln)?;
                                        result.push(ExecutableLine::new(ln, "assign".to_string(), cc, Some(nc)));
                                        break;
                                    },
                                    Chunk::Function(_) => continue, // nothing -> param -> function
                                }
                            }, 
                        }
                    }, // will use in next itration
                    Chunk::Function(cc) => {
                        match nc {
                            None => { // nothing -> function -> nothing
                                result.push(ExecutableLine::new(ln, cc, vec![], None));
                                break;
                            },
                            Some(nc) => {
                                match nc.clone()?{
                                    Chunk::Params(nc) => {
                                        let nc = get_single_var(nc,ln)?;
                                        result.push(ExecutableLine::new(ln, cc, vec![], Some(nc.clone())));
                                        break;
                                    },
                                    Chunk::Function(_) => {
                                        result.push(ExecutableLine::new(ln, cc, vec![], Some(self.get_next_tmp())))
                                    },
                                }
                            },
                        }
                    },
                }
            }else{
                let pc = pc.unwrap().clone()?;
                match cc {
                    Chunk::Params(_) => return Err(ChapError::syntax_with_msg(ln, "this will not happen".to_string())),
                    Chunk::Function(cc) => {
                        match pc {
                            Chunk::Params(pc) => {
                                match nc {
                                    None => { // param -> function -> nothing
                                        result.push(ExecutableLine::new(ln, cc, pc, None));
                                        break;
                                    },
                                    Some(nc) => {
                                        let nc = nc.clone()?;
                                        match nc {
                                            Chunk::Params(nc) => { // param -> function -> param
                                                let nc = get_single_var(nc,ln)?;
                                                result.push(ExecutableLine::new(ln, cc, pc, Some(nc.clone())));
                                                break;
                                            },
                                            Chunk::Function(_nc) => { // param -> function -> function
                                                result.push(ExecutableLine::new(ln, cc, pc, Some(self.get_next_tmp())));
                                            },
                                        }
                                    },
                                }
                            },
                            Chunk::Function(_pc) => {
                                match nc {
                                    None => { // function -> function -> nothing
                                        let tmp = Param::Variable(self.get_current_tmp());
                                        result.push(ExecutableLine::new(ln, cc, vec![tmp], None));
                                        break;
                                    },
                                    Some(nc) => {
                                        let nc = nc.clone()?;
                                        match nc {
                                            Chunk::Params(nc) => { //function -> function -> param
                                                let tmp = Param::Variable(self.get_current_tmp());
                                                let nc = get_single_var(nc,ln)?;
                                                result.push(ExecutableLine::new(ln, cc, vec![tmp], Some(nc.clone())));
                                                break;
                                                
                                            },
                                            Chunk::Function(_nc) => { // function -> function -> function
                                                let tmp = Param::Variable(self.get_current_tmp());
                                                result.push(ExecutableLine::new(ln, cc, vec![tmp], Some(self.get_next_tmp())));
                                            },
                                        }
                                    },
                                }
                            },
                        }
                    },
                }
            }

        };

        Ok(result)

    }

    fn get_current_tmp(&mut self) -> String{
        format!("LTMP_{}",self.tmp_var_counter) 
    }

    fn get_next_tmp(&mut self) -> String{
        self.tmp_var_counter+=1;
        self.get_current_tmp()
    }
    

}


#[cfg(test)]
mod tests{

    use crate::common::{param::Param, data_type::DataType};

    use super::*;


    // ---- single chunk ---- 
    #[test]
    fn tag_parser(){
        let mut p = Parser::default();
        let result = p.on_new_line(LineOfCode::new(1, " @myTag ".to_string())).unwrap();
        
        assert_eq!(
            result.get(0).unwrap(),
            &ExecutableLine::new(1,"new_tag".to_string(),vec![Param::Tag("myTag".to_string())],None)
        );
    }

    #[test]
    fn print_detector_parser(){
        let mut p = Parser::default();
        let result = p.on_new_line(LineOfCode::new(1, " $myVar ".to_string())).unwrap();
        assert_eq!(
            result.get(0).unwrap(),
            &ExecutableLine::new(1,"print".to_string(), vec![Param::Variable("myVar".to_string())], None)
        );

        let result = p.on_new_line(LineOfCode::new(1, " \"hello\" ".to_string())).unwrap();
        assert_eq!(
            result.get(0).unwrap(),
            &ExecutableLine::new(1,"print".to_string(),vec![Param::Value(DataType::String("hello".to_string()))],None)
        );

    }

    #[test]
    fn function_call_paramless(){
        let mut p = Parser::default();
        let result = p.on_new_line(LineOfCode::new(1, " exit ".to_string())).unwrap();
        assert_eq!(
            result.get(0).unwrap(),
            &ExecutableLine::new(1,"exit".to_string(), vec![], None)
        );
    }


    // ---- double chunk ---- 
    #[test]
    fn param_param_test(){
        let mut p = Parser::default();
        let result = p.on_new_line(LineOfCode::new(1, " 1 -> $var ".to_string())).unwrap();
        assert_eq!(
            result.get(0).unwrap(),
            &ExecutableLine::new(1,"assign".to_string(), vec![Param::Value(DataType::Int(1))], Some("var".to_string()))
        );
    }

    #[test]
    fn param_function_test(){
        let mut p = Parser::default();
        let result = p.on_new_line(LineOfCode::new(1, " $var -> function ".to_string())).unwrap();
        assert_eq!(
            result.get(0).unwrap(),
            &ExecutableLine::new(1, "function".to_string(), vec![Param::Variable("var".to_string())], None)
        );
    }

    #[test]
    fn function_param_test(){
        let mut p = Parser::default();
        let result = p.on_new_line(LineOfCode::new(1, " input -> $var ".to_string())).unwrap();
        assert_eq!(
            result.get(0).unwrap(),
            &ExecutableLine::new(1, "input".to_string(), vec![], Some("var".to_string()))
        );
    }


    // ---- triplet chunk ----
    #[test]
    fn valid_expression(){
        let mut p = Parser::default();
        let result = p.on_new_line(LineOfCode::new(1, " 2 -> sum -> $var ".to_string())).unwrap();
        assert_eq!(
            result.get(0).unwrap(),
            &ExecutableLine::new(
                1,
                "sum".to_string(),
                vec![Param::Value(DataType::Int(2))],
                Some("var".to_string())
            )
        );
    }


    #[test]
    fn piping_test(){
        let mut p = Parser::default();

        let result = p.on_new_line(LineOfCode::new(1, "1, 2 -> add -> to_string -> $a".to_string())).unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(
            result.get(0).unwrap(),
            &ExecutableLine::new(
                1,
                "add".to_string(),
                vec![Param::Value(DataType::Int(1)), Param::Value(DataType::Int(2))], 
                Some("LTMP_1".to_string())
            ) 
        );
        assert_eq!(
            result.get(1).unwrap(),
            &ExecutableLine::new(
                1,
                "to_string".to_string(),
                vec![Param::Variable("LTMP_1".to_string())], 
                Some("a".to_string())
            ) 
        );
    }

    #[test]
    fn string_includes_arrow(){
        let mut p = Parser::default();
        assert_eq!(
            p.on_new_line(LineOfCode::new(1, "\"hello -> world\"".to_string())),
            Ok(vec![ExecutableLine::new(
                1,
                "print".to_string(),
                vec![Param::Value(DataType::String("hello -> world".to_string()))],
                None
            )])
        );
    }

    #[test]
    fn string_includes_comment(){
        let mut p = Parser::default();
        assert_eq!(
            p.on_new_line(LineOfCode::new(1, "\"hello // world\"".to_string())),
            Ok(vec![ExecutableLine::new(
                1,
                "print".to_string(),
                vec![Param::Value(DataType::String("hello // world".to_string()))],
                None
            )])
        );
    }

}