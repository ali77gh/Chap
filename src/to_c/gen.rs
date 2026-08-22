use std::collections::{BTreeSet, HashMap, HashSet};
use std::fmt::Write;

use crate::common::data_type::DataType;
use crate::common::errors::{ChapError, Result};
use crate::common::executable::ExecutableLine;
use crate::common::param::Param;

const PRELUDE: &str = include_str!("prelude.c");

/// normalize function name the same way builtin_function::closure_gen does
fn normalize(function_name: &str) -> String {
    function_name.to_lowercase().replace([' ', '_', '?'], "")
}

fn cname(name: &str) -> String {
    let mut result = String::from("v_");
    for c in name.chars() {
        if c.is_ascii_alphanumeric() || c == '_' {
            result.push(c);
        } else {
            result.push('_');
        }
    }
    result
}

fn escape_string(s: &str) -> String {
    let mut result = String::new();
    for c in s.chars() {
        match c {
            '\\' => result.push_str("\\\\"),
            '"' => result.push_str("\\\""),
            '\n' => result.push_str("\\n"),
            '\r' => result.push_str("\\r"),
            '\t' => result.push_str("\\t"),
            _ => result.push(c),
        }
    }
    result
}

fn err(line_number: u32, msg: String) -> ChapError {
    ChapError::static_analyzer_with_msg(line_number, msg)
}

/// turn a param into a C expression producing a CV
fn expr(param: &Param) -> Result<String> {
    Ok(match param {
        Param::Value(DataType::Int(i)) => format!("cv_int({}LL)", i),
        Param::Value(DataType::Float(f)) => format!("cv_flt({:?})", f),
        Param::Value(DataType::Bool(b)) => format!("cv_bool({})", *b as i32),
        Param::Value(DataType::String(s)) => format!("cv_str(\"{}\")", escape_string(s)),
        Param::Value(value) => {
            return Err(err(
                0,
                format!(
                    "chap to C compiler does not support this type yet: {}",
                    value.type_name()
                ),
            ))
        }
        Param::Variable(name) => cname(name),
        Param::Tag(tag, _) => {
            return Err(err(0, format!("unexpected tag @{} in value position", tag)))
        }
    })
}

pub fn generate(executables: &[ExecutableLine]) -> Result<String> {
    let names: Vec<String> = executables
        .iter()
        .map(|e| normalize(&e.function_name))
        .collect();

    // collect variables
    let mut vars: BTreeSet<String> = BTreeSet::new();
    for ex in executables {
        if let Some(output) = &ex.output_var {
            vars.insert(cname(output));
        }
        for param in &ex.params {
            if let Param::Variable(v) = param {
                vars.insert(cname(v));
            }
        }
    }

    // map tag name -> executable index (labels)
    let mut tags: HashMap<&str, usize> = HashMap::new();
    for (i, ex) in executables.iter().enumerate() {
        if names[i] == "newtag" {
            match ex.params.first() {
                Some(Param::Tag(tag, _)) => {
                    tags.insert(tag.as_str(), i);
                }
                _ => {
                    return Err(err(
                        ex.line_number,
                        "error while creating a tag".to_string(),
                    ))
                }
            }
        }
    }

    let target_of = |ex: &ExecutableLine| -> Result<usize> {
        match ex.params.first() {
            Some(Param::Tag(tag, _)) => match tags.get(tag.as_str()) {
                Some(i) => Ok(*i),
                None => Err(err(ex.line_number, format!("cant find tag: {}", tag))),
            },
            _ => Err(err(
                ex.line_number,
                "function jump needs a tag for first params".to_string(),
            )),
        }
    };

    // indexes which get a C label because some jump points at them
    let mut targets: HashSet<usize> = HashSet::new();
    for (i, name) in names.iter().enumerate() {
        if matches!(name.as_str(), "jump" | "jumpifnot" | "jeq" | "jneq") {
            targets.insert(target_of(&executables[i])?);
        }
    }

    // body
    let mut body = String::from("int main(void) {\n");
    for v in &vars {
        writeln!(&mut body, "    CV {} = {{.t = T_INT}};", v).unwrap();
    }

    for (i, ex) in executables.iter().enumerate() {
        if targets.contains(&i) {
            writeln!(&mut body, "L{}:;", i).unwrap();
        }
        let statement = statement(ex, &names[i], &target_of)?;
        if !statement.is_empty() {
            writeln!(&mut body, "    {}", statement).unwrap();
        }
    }
    body.push_str("    return 0;\n}\n");

    let mut c_code = String::from(PRELUDE);
    c_code.push_str(&body);
    Ok(c_code)
}

fn statement(
    ex: &ExecutableLine,
    name: &str,
    target_of: &dyn Fn(&ExecutableLine) -> Result<usize>,
) -> Result<String> {
    let params = &ex.params;
    // chap semantics: a function result without output variable gets printed
    let assign_to = |value: String| -> String {
        match &ex.output_var {
            Some(output) => format!("{} = {};", cname(output), value),
            None => format!("chap_print(1, {});", value),
        }
    };
    let args = |from: usize| -> Result<Vec<String>> {
        params[from..]
            .iter()
            .map(expr)
            .collect::<Result<Vec<String>>>()
    };

    Ok(match name {
        "assign" => {
            let value = match params.first() {
                Some(p) => expr(p)?,
                None => {
                    return Err(err(
                        ex.line_number,
                        "assign function needs one input param".to_string(),
                    ))
                }
            };
            match &ex.output_var {
                Some(output) => format!("{} = {};", cname(output), value),
                None => {
                    return Err(err(
                        ex.line_number,
                        "assign function needs output".to_string(),
                    ))
                }
            }
        }
        "newtag" => String::new(), // handled by label emission

        "jump" => format!("goto L{};", target_of(ex)?),
        "jumpifnot" => {
            let tag = target_of(ex)?;
            let cond = args(1)?.into_iter().next();
            match cond {
                Some(cond) => format!("if (!({}).b) goto L{};", cond, tag),
                None => {
                    return Err(err(
                        ex.line_number,
                        "jump_if_not function needs bool as second param".to_string(),
                    ))
                }
            }
        }
        "jeq" | "jumpifequal" => {
            let tag = target_of(ex)?;
            let a = args(1)?; // [p1, p2]
            if a.len() != 2 {
                return Err(err(
                    ex.line_number,
                    "jump_if_equal needs two comparable params".to_string(),
                ));
            }
            format!("if (cv_eq({}, {})) goto L{};", a[0], a[1], tag)
        }
        "jneq" | "jumpifnotequal" => {
            let tag = target_of(ex)?;
            let a = args(1)?;
            if a.len() != 2 {
                return Err(err(
                    ex.line_number,
                    "jump_if_not_equal needs two comparable params".to_string(),
                ));
            }
            format!("if (!cv_eq({}, {})) goto L{};", a[0], a[1], tag)
        }

        "print" | "show" | "stdout" => {
            let a = args(0)?;
            format!(
                "chap_print({}{});",
                a.len(),
                a.into_iter().fold(String::new(), |acc, x| acc + ", " + &x)
            )
        }
        "concat" | "cat" => {
            let a = args(0)?;
            assign_to(format!(
                "cv_concat({}{})",
                a.len(),
                a.into_iter().fold(String::new(), |acc, x| acc + ", " + &x)
            ))
        }

        "increase" | "inc" => match params.first() {
            Some(Param::Variable(v)) => format!("{}.i += 1;", cname(v)),
            _ => {
                return Err(err(
                    ex.line_number,
                    "increase function need one variable".to_string(),
                ))
            }
        },

        "modulus" | "mod" => {
            let a = args(0)?;
            assign_to(format!("cv_mod({}, {})", a[0], a[1]))
        }
        "sqrt" | "squareroot" => {
            let a = args(0)?;
            assign_to(format!("cv_flt(sqrt(cv_num({})))", a[0]))
        }
        "toint" => {
            let a = args(0)?;
            assign_to(format!("cv_toint({})", a[0]))
        }
        "lt" | "lessthan" => {
            let a = args(0)?;
            assign_to(format!("cv_bool(cv_num({}) < cv_num({}))", a[0], a[1]))
        }

        "exit" | "quit" | "kill" | "end" => "return 0;".to_string(),

        _ => {
            return Err(err(
                ex.line_number,
                format!(
                    "chap to C compiler does not support this function yet: {}",
                    ex.function_name
                ),
            ))
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assign_and_print() {
        let c_code = super::super::compile_to_c("10 -> $a\n$a").unwrap();
        assert!(c_code.contains("v_a = cv_int(10LL);"));
        assert!(c_code.contains("chap_print(1, v_a);"));
        assert!(c_code.ends_with("    return 0;\n}\n"));
    }

    #[test]
    fn jumps_become_gotos() {
        let c_code =
            super::super::compile_to_c("@loop\n@is, $i -> jump_if_not\n@end -> jump\n@is\n@end")
                .unwrap();
        assert!(c_code.contains("L3:;")); // label for @is
        assert!(c_code.contains("L4:;")); // label for @end
        assert!(c_code.contains("if (!(v_i).b) goto L3;")); // jump_if_not to @is
        assert!(c_code.contains("goto L4;")); // jump to @end
    }

    #[test]
    fn unknown_function_is_error() {
        assert!(super::super::compile_to_c("$a -> not_a_function").is_err());
    }

    #[test]
    fn unsupported_value_types_are_error() {
        assert!(super::super::compile_to_c("[1,2] -> $l").is_err());
    }
}
