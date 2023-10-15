
pub fn string_safe_split(inp: &str, pattern: String) -> Vec<&str>{
    let mut result: Vec<&str> = vec![];

    let mut quotations_on_left=0;
    let mut last_seen = 0;
    for (i,ch) in inp.chars().enumerate(){
        if ch == '\"'{
            quotations_on_left+=1;
        }
        let token = &inp[i..];
        if token.starts_with(&pattern) && quotations_on_left%2==0{
            result.push(&inp[last_seen..i]);
            last_seen=i+1
        }
    }
    result.push(&inp[last_seen..]);

    result
}


#[cfg(test)]
mod tests{
    use crate::common::splitter::string_safe_split;

    #[test]
    fn comma_spliter_test(){
        assert_eq!(
            string_safe_split("ali,hasan,majid", ",".to_string()),
            vec!["ali","hasan","majid"]
        );
        assert_eq!(
            string_safe_split("\"ali,hasan\",majid", ",".to_string()),
            vec!["\"ali,hasan\"","majid"]
        );
    }
}