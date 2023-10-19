
use crate::common::line_of_code::LineOfCode;
use crate::common::splitter::string_safe_split;


#[derive(Default)]
pub struct Preprocessor{
    current_line: u32
}

// Preprocessor converts users code to parsable code
impl Preprocessor {

    pub fn on_new_line(&mut self, actual_line: String) -> Vec<LineOfCode>{

        self.current_line+=1;

        let binding = string_safe_split(actual_line.as_str(), "//".to_string());
        let line = binding.first().unwrap();

        line.split(';')
            .map(|line|{ line.trim() })
            .filter(|x|{ !x.is_empty()})
            .map(|x|{ LineOfCode::new(self.current_line,x.to_string()) })
            .collect()
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

        all_lines.append(&mut pp.on_new_line("//line".to_string()));
        all_lines.append(&mut pp.on_new_line("    //line".to_string()));
        all_lines.append(&mut pp.on_new_line("//line    ".to_string()));
        all_lines.append(&mut pp.on_new_line("   //line    ".to_string()));
        all_lines.append(&mut pp.on_new_line("   ".to_string()));
        all_lines.append(&mut pp.on_new_line("".to_string()));

        assert_eq!(6, pp.current_line);

        assert_eq!(0, all_lines.len());
    }


    #[test]
    fn semicolon_split() {

        let mut pp = Preprocessor::default();
        
        assert_eq!(0 , pp.on_new_line("//comment".to_string()).len() );
        assert_eq!(0 , pp.on_new_line("".to_string()).len() );
        assert_eq!(1 , pp.on_new_line("   command1".to_string()).len() );
        assert_eq!(2 , pp.on_new_line("  command2  ;   command3  ".to_string()).len() );
        assert_eq!(1 , pp.on_new_line("  command3  ;   //comment  ".to_string()).len() );
        assert_eq!(1 , pp.on_new_line("  command4     //comment  ".to_string()).len() );
        assert_eq!(2 , pp.on_new_line("  command5 ; ; command6     //comment  ".to_string()).len() );

        assert_eq!(7, pp.current_line);
    }

    #[test]
    fn currect_line_numbers(){

        let mut pp = Preprocessor::default();

        assert_eq!(0 , pp.on_new_line("//comment1".to_string()).len() );
        assert_eq!(0 , pp.on_new_line("//comment2".to_string()).len() );

        let pped = pp.on_new_line("param -> func -> out".to_string());

        assert_eq!(3, pped.get(0).unwrap().line_number)
    }

    #[test]
    fn currect_actual_line(){

        let mut pp = Preprocessor::default();

        let pped = pp.on_new_line(";  command1 ;  ; command2 ;;//comment;".to_string());

        assert_eq!("command1", pped.get(0).unwrap().code);
        assert_eq!("command2", pped.get(1).unwrap().code);
    }

    #[test]
    fn string_includes_comment(){
        let mut p = Preprocessor::default();
        let result = p.on_new_line("\"hello // world\"".to_string());
        assert_eq!(
            result.get(0).unwrap().code,
            "\"hello // world\"".to_string()
        );
    }

}