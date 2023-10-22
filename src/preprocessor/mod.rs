
use crate::common::errors::Result;
use crate::common::line_of_code::LineOfCode;
use crate::common::splitter::string_safe_split;


#[derive(Default)]
pub struct Preprocessor{
    current_line: u32,
    buffer: String,
}

// Preprocessor converts users code to parsable code
impl Preprocessor {

    pub fn on_new_line(&mut self, actual_line: String) -> Result<Vec<LineOfCode>>{

        let actual_line = format!("{}{}",self.buffer,actual_line);
        self.buffer = String::new();

        self.current_line+=1;

        let binding = string_safe_split(actual_line.as_str(), "//".to_string());
        let line = binding.first().unwrap();

        let lines = line.split(';')
            .map(|line|{ line.trim() })
            .filter(|x|{ !x.is_empty()});

        let mut lines: Vec<&str> = lines.collect();

        let line = self.current_line;

        lines = self.multiline_expresion(lines);
        // let lines = self.parentheses(lines)?;

        Ok(lines.iter().map(|x|{ LineOfCode::new(line,x.to_string()) })
            .collect())
    }

    fn multiline_expresion<'a>(&'a mut self, mut inp: Vec<&'a str>) -> Vec<&str>{
        if let Some(x) = inp.last(){
            if x.ends_with("->"){
                let halfline = inp.pop().unwrap();
                self.buffer.push_str(halfline);
                inp
            }else{
                inp
            }
        }else{
            vec![]
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::common::line_of_code::LineOfCode;

    use super::Preprocessor;

    #[test]
    fn remove_comments() {
    
        let mut pp = Preprocessor::default();

        let mut all_lines: Vec<LineOfCode> = Vec::new();

        all_lines.append(&mut pp.on_new_line("//line".to_string()).unwrap());
        all_lines.append(&mut pp.on_new_line("    //line".to_string()).unwrap());
        all_lines.append(&mut pp.on_new_line("//line    ".to_string()).unwrap());
        all_lines.append(&mut pp.on_new_line("   //line    ".to_string()).unwrap());
        all_lines.append(&mut pp.on_new_line("   ".to_string()).unwrap());
        all_lines.append(&mut pp.on_new_line("".to_string()).unwrap());

        assert_eq!(6, pp.current_line);

        assert_eq!(0, all_lines.len());
    }


    #[test]
    fn semicolon_split() {

        let mut pp = Preprocessor::default();
        
        assert_eq!(0 , pp.on_new_line("//comment".to_string()).unwrap().len() );
        assert_eq!(0 , pp.on_new_line("".to_string()).unwrap().len() );
        assert_eq!(1 , pp.on_new_line("   command1".to_string()).unwrap().len() );
        assert_eq!(2 , pp.on_new_line("  command2  ;   command3  ".to_string()).unwrap().len() );
        assert_eq!(1 , pp.on_new_line("  command3  ;   //comment  ".to_string()).unwrap().len() );
        assert_eq!(1 , pp.on_new_line("  command4     //comment  ".to_string()).unwrap().len() );
        assert_eq!(2 , pp.on_new_line("  command5 ; ; command6     //comment  ".to_string()).unwrap().len() );

        assert_eq!(7, pp.current_line);
    }

    #[test]
    fn currect_line_numbers(){

        let mut pp = Preprocessor::default();

        assert_eq!(0 , pp.on_new_line("//comment1".to_string()).unwrap().len() );
        assert_eq!(0 , pp.on_new_line("//comment2".to_string()).unwrap().len() );

        let pped = pp.on_new_line("param -> func -> out".to_string()).unwrap();

        assert_eq!(3, pped.get(0).unwrap().line_number)
    }

    #[test]
    fn currect_actual_line(){

        let mut pp = Preprocessor::default();

        let pped = pp.on_new_line(";  command1 ;  ; command2 ;;//comment;".to_string()).unwrap();

        assert_eq!("command1", pped.get(0).unwrap().code);
        assert_eq!("command2", pped.get(1).unwrap().code);
    }

    #[test]
    fn string_includes_comment(){
        let mut p = Preprocessor::default();
        let result = p.on_new_line("\"hello // world\"".to_string()).unwrap();
        assert_eq!(
            result.get(0).unwrap().code,
            "\"hello // world\"".to_string()
        );
    }

    #[test]
    fn multiline_expresion(){
        let mut p = Preprocessor::default();
        let result1 = p.on_new_line("1,2->".to_string()).unwrap();
        let result2 = p.on_new_line("add".to_string()).unwrap();

        assert_eq!(result1.len(), 0);
        assert_eq!(result2.get(0).unwrap().code, "1,2->add".to_string());
    }

}