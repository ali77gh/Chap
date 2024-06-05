use crate::common::{errors::Result, param::Param};

use crate::{
    common::{errors::ChapError, executable::ExecutableLine},
    runtime::Runtime,
};

pub fn new_tag(runtime: &mut Runtime, executable: &ExecutableLine) -> Result<()> {
    if let Some(Param::Tag(tag_name, _)) = executable.params.first() {
        runtime.tags.insert(tag_name.clone(), runtime.current_line);
    } else {
        return Err(ChapError::runtime_with_msg(
            executable.line_number,
            "error while creating a tag".to_string(),
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{
        common::{executable::ExecutableLine, param::Param},
        runtime::Runtime,
    };

    use super::new_tag;

    #[test]
    fn new_tag_test() {
        let mut runtime = Runtime::new(|_| {}, || "".to_string());

        new_tag(
            &mut runtime,
            &ExecutableLine::new(
                6,
                "".to_string(),
                vec![Param::Tag("new_tag".to_string(), None)],
                None,
            ),
        )
        .unwrap();

        assert_eq!(runtime.tags.get("new_tag").unwrap(), &0usize);
    }
}
