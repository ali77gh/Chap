use crate::{runtime::runtime::Runtime, common::{executable::ExecutableLine, errors::ChapError}};
use crate::common::errors::Result;
use crate::common::param::Param;

pub fn new_tag(runtime: &mut Runtime, executable: ExecutableLine)-> Result<()>{

    if let Some(Param::Tag(tag)) = executable.params.get(0){
        runtime.tags.insert(tag.clone(), runtime.current_line);
    } else {
        return Err(ChapError::runtime_with_msg(executable.line_number, "error while creating a tag".to_string()));
    }
    Ok(())
}


#[cfg(test)]
mod tests{
    use crate::{runtime::runtime::Runtime, common::{executable::ExecutableLine, param::Param}};

    use super::new_tag;

    #[test]
    fn new_tag_test(){
        let mut runtime = Runtime::new(|_|{}, ||{ "".to_string() });

        new_tag(
            &mut runtime,
            ExecutableLine::new(
                6,
                "".to_string(),
                vec![Param::Tag("new_tag".to_string())],
                None
            )
        ).unwrap();

        assert_eq!(
            runtime.tags.get("new_tag").unwrap(),
            &0usize
        );

    }
}