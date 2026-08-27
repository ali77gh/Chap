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
    match param {
        Param::Value(value) => expr_value(value),
        Param::Variable(name) => Ok(cname(name)),
        Param::Tag(tag, _) => Err(err(0, format!("unexpected tag @{} in value position", tag))),
    }
}

/// turn a compile time value into a C expression producing a CV
fn expr_value(value: &DataType) -> Result<String> {
    Ok(match value {
        DataType::Int(i) => format!("cv_int({}LL)", i),
        DataType::Float(f) => format!("cv_flt({:?})", f),
        DataType::Bool(b) => format!("cv_bool({})", *b as i32),
        DataType::String(s) => format!("cv_str(\"{}\")", escape_string(s)),
        DataType::List(items) => {
            let args = items
                .iter()
                .map(expr_value)
                .collect::<Result<Vec<String>>>()?;
            variadic("cv_list_lit", &args)
        }
        DataType::Map(map) => {
            // sorted for stable output, map equality does not care about order
            let mut entries: Vec<(&String, &DataType)> = map.iter().collect();
            entries.sort_by(|a, b| a.0.cmp(b.0));
            let mut call = format!("cv_map_lit({}", entries.len());
            for (k, v) in entries {
                call.push_str(&format!(", \"{}\", {}", escape_string(k), expr_value(v)?));
            }
            call.push(')');
            call
        }
    })
}

/// call a variadic C helper: name(count, arg1, arg2, ...)
fn variadic(name: &str, args: &[String]) -> String {
    format!(
        "{}({}{})",
        name,
        args.len(),
        args.iter().fold(String::new(), |acc, x| acc + ", " + x)
    )
}

pub fn generate(executables: &[ExecutableLine]) -> Result<String> {
    let names: Vec<String> = executables
        .iter()
        .map(|e| normalize(&e.function_name))
        .collect();

    // collect variables (cname, original name)
    let mut vars: BTreeSet<(String, String)> = BTreeSet::new();
    for ex in executables {
        if let Some(output) = &ex.output_var {
            vars.insert((cname(output), output.clone()));
        }
        for param in &ex.params {
            if let Param::Variable(v) = param {
                vars.insert((cname(v), v.clone()));
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
        if matches!(
            name.as_str(),
            "jump" | "jumpif" | "jumpifnot" | "jeq" | "jneq"
        ) {
            targets.insert(target_of(&executables[i])?);
        }
    }

    // body
    let mut body = String::from("int main(void) {\n");
    for (v, _) in &vars {
        writeln!(&mut body, "    CV {v} = {{.t = T_INT}};").unwrap();
    }

    for (i, ex) in executables.iter().enumerate() {
        if targets.contains(&i) {
            writeln!(&mut body, "L{}:;", i).unwrap();
        }
        let statement = statement(ex, &names[i], &vars, &target_of)?;
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
    vars: &BTreeSet<(String, String)>,
    target_of: &dyn Fn(&ExecutableLine) -> Result<usize>,
) -> Result<String> {
    let params = &ex.params;
    // chap semantics: a function result without output variable gets printed.
    // cv_copy gives value semantics on assignment (chap clones values too)
    let assign_to = |value: String| -> String {
        match &ex.output_var {
            Some(output) => format!("{} = cv_copy({});", cname(output), value),
            None => format!("chap_print(1, {});", value),
        }
    };
    let args = |from: usize| -> Result<Vec<String>> {
        params[from..]
            .iter()
            .map(expr)
            .collect::<Result<Vec<String>>>()
    };
    // functions that mutate (insert, pop, ...) need a variable to mutate
    let var_param = |ex: &ExecutableLine, idx: usize, msg: &str| -> Result<String> {
        match params.get(idx) {
            Some(Param::Variable(v)) => Ok(cname(v)),
            _ => Err(err(ex.line_number, msg.to_string())),
        }
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
                Some(output) => format!("{} = cv_copy({});", cname(output), value),
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
        "jumpif" => {
            let tag = target_of(ex)?;
            let cond = args(1)?.into_iter().next();
            match cond {
                Some(cond) => format!("if (({}).b) goto L{};", cond, tag),
                None => {
                    return Err(err(
                        ex.line_number,
                        "jump_if function needs bool as second param".to_string(),
                    ))
                }
            }
        }
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
            format!("{};", variadic("chap_print", &a))
        }
        "concat" | "cat" => {
            let a = args(0)?;
            assign_to(variadic("cv_concat", &a))
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

        "decrease" | "dec" => match params.first() {
            Some(Param::Variable(v)) => format!("{}.i -= 1;", cname(v)),
            _ => {
                return Err(err(
                    ex.line_number,
                    "decrease function need one variable".to_string(),
                ))
            }
        },

        "power" | "pow" => {
            let a = args(0)?;
            assign_to(format!("cv_pow({}, {})", a[0], a[1]))
        }
        "addmany" | "addall" => {
            let a = args(0)?;
            assign_to(variadic("cv_add_many", &a))
        }

        // does nothing
        "pass" | "nop" | "noop" => String::new(),

        "repeat" => {
            let a = args(0)?;
            assign_to(format!("cv_repeat({}, {})", a[0], a[1]))
        }
        "length" | "len" => {
            let a = args(0)?;
            assign_to(format!("cv_length({})", a[0]))
        }
        "contains" | "has" => {
            let a = args(0)?;
            assign_to(format!("cv_contains({}, {})", a[0], a[1]))
        }
        "slice" | "substring" => {
            let a = args(0)?;
            assign_to(format!("cv_slice({}, {}, {})", a[0], a[1], a[2]))
        }
        "charat" => {
            let a = args(0)?;
            assign_to(format!("cv_char_at({}, {})", a[0], a[1]))
        }
        "toupper" | "uppercase" => {
            let a = args(0)?;
            assign_to(format!("cv_to_upper({})", a[0]))
        }
        "tolower" | "lowercase" => {
            let a = args(0)?;
            assign_to(format!("cv_to_lower({})", a[0]))
        }
        "trim" => {
            let a = args(0)?;
            assign_to(format!("cv_trim({})", a[0]))
        }

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
        "tostring" | "tostr" => {
            let a = args(0)?;
            assign_to(format!("cv_torepr({})", a[0]))
        }
        "tofloat" => {
            let a = args(0)?;
            assign_to(format!("cv_tofloat({})", a[0]))
        }
        "typeof" | "type" => {
            let a = args(0)?;
            assign_to(format!("cv_typeof({})", a[0]))
        }
        "dump" | "dumpmemory" => {
            // interpreter iterates a hash map so its order is random, we print
            // variables in declaration order instead
            let mut block = format!(
                "printf(\"------- Memory dump line: {} -------\\n\");",
                ex.line_number
            );
            for (v, orig) in vars {
                block.push_str(&format!(
                    " {{ char* _s = cv_to_string({}); printf(\"%s -> $%s\\n\", _s, \"{}\"); free(_s); }}",
                    v,
                    escape_string(orig)
                ));
            }
            if vars.is_empty() {
                block.push_str(" putchar('\\n');");
            }
            block.push_str(" puts(\"------- Memory dump ends -------\");");
            block
        }
        "lt" | "lessthan" => {
            let a = args(0)?;
            assign_to(format!("cv_bool(cv_num({}) < cv_num({}))", a[0], a[1]))
        }
        "lte" | "lessthanequal" => {
            let a = args(0)?;
            assign_to(format!("cv_bool(cv_num({}) <= cv_num({}))", a[0], a[1]))
        }
        "gt" | "greaterthan" => {
            let a = args(0)?;
            assign_to(format!("cv_bool(cv_num({}) > cv_num({}))", a[0], a[1]))
        }
        "gte" | "greaterthanequal" => {
            let a = args(0)?;
            assign_to(format!("cv_bool(cv_num({}) >= cv_num({}))", a[0], a[1]))
        }

        "add" => {
            let a = args(0)?;
            assign_to(format!("cv_add({}, {})", a[0], a[1]))
        }
        "minus" => {
            let a = args(0)?;
            assign_to(format!("cv_minus({}, {})", a[0], a[1]))
        }
        "multiply" => {
            let a = args(0)?;
            assign_to(format!("cv_multiply({}, {})", a[0], a[1]))
        }
        "divide" => {
            let a = args(0)?;
            assign_to(format!("cv_divide({}, {})", a[0], a[1]))
        }

        "equal" | "eq" => {
            let a = args(0)?;
            assign_to(format!("cv_bool(cv_eq({}, {}))", a[0], a[1]))
        }
        "notequal" | "neq" => {
            let a = args(0)?;
            assign_to(format!("cv_bool(!cv_eq({}, {}))", a[0], a[1]))
        }

        // and/or/xor take any number of bool params (folded like chap does)
        "and" => {
            let a = args(0)?;
            assign_to(variadic("cv_and", &a))
        }
        "or" => {
            let a = args(0)?;
            assign_to(variadic("cv_or", &a))
        }
        "xor" => {
            let a = args(0)?;
            assign_to(variadic("cv_xor", &a))
        }
        "not" => {
            let a = args(0)?;
            assign_to(format!("cv_bool(!({}).b)", a[0]))
        }

        "input" | "stdin" => assign_to("cv_input()".to_string()),

        "now" | "nowsec" | "unixtime" => assign_to("cv_now()".to_string()),
        "waitmil" | "waitmillis" => format!(
            "chap_wait_millis({});",
            args(0)?.into_iter().next().ok_or_else(|| err(
                ex.line_number,
                "wait_millis needs one int param".to_string()
            ))?
        ),
        "waitsec" | "waitseconds" => format!(
            "chap_wait_seconds({});",
            args(0)?.into_iter().next().ok_or_else(|| err(
                ex.line_number,
                "wait_seconds needs one int param".to_string()
            ))?
        ),
        "waitmin" | "waitminutes" => format!(
            "chap_wait_minutes({});",
            args(0)?.into_iter().next().ok_or_else(|| err(
                ex.line_number,
                "wait_minutes needs one int param".to_string()
            ))?
        ),
        "waithour" => format!(
            "chap_wait_hours({});",
            args(0)?
                .into_iter()
                .next()
                .ok_or_else(|| err(ex.line_number, "wait_hour needs one int param".to_string()))?
        ),

        "randomnumber" | "randnum" => {
            let a = args(0)?;
            if a.len() != 2 {
                return Err(err(
                    ex.line_number,
                    "random_number needs two params (min, max)".to_string(),
                ));
            }
            assign_to(format!("cv_random_number({}, {})", a[0], a[1]))
        }
        "randomstring" | "randstr" => {
            let a = args(0)?;
            if a.len() != 2 {
                return Err(err(
                    ex.line_number,
                    "random_string needs two params (alphabet, length)".to_string(),
                ));
            }
            assign_to(format!("cv_random_string({}, {})", a[0], a[1]))
        }
        "randombool" | "randbool" => assign_to("cv_random_bool()".to_string()),
        "randomchoice" | "randchoice" => {
            let a = args(0)?;
            assign_to(variadic("cv_random_choice", &a))
        }

        // ---- collection functions ----
        // mutating ones need a variable as first param (like the interpreter)
        "insert" | "push" => {
            let target = var_param(
                ex,
                0,
                "insert function needs a variable holding a list or map",
            )?;
            let a = args(1)?;
            if a.len() != 1 {
                return Err(err(
                    ex.line_number,
                    "insert function needs exactly two params".to_string(),
                ));
            }
            format!("cv_insert(&{}, {});", target, a[0])
        }
        "get" | "at" => {
            let a = args(0)?;
            if a.len() != 2 {
                return Err(err(
                    ex.line_number,
                    "correct form of 'get' function: <list | map>, <index | key> -> get -> $item"
                        .to_string(),
                ));
            }
            assign_to(format!("cv_get({}, {})", a[0], a[1]))
        }
        "includes" | "in" => {
            let a = args(0)?;
            assign_to(format!("cv_has({}, {})", a[0], a[1]))
        }
        "indexof" => {
            let a = args(0)?;
            assign_to(format!("cv_index_of({}, {})", a[0], a[1]))
        }
        "pop" => {
            let target = var_param(ex, 0, "pop function first param should be a list variable")?;
            assign_to(format!("cv_pop(&{})", target))
        }
        "last" => {
            let a = args(0)?;
            assign_to(format!("cv_last({})", a[0]))
        }
        "removeat" | "rmat" => {
            let target = var_param(ex, 0, "remove_at needs a list variable as first param")?;
            let a = args(1)?;
            if a.len() != 1 {
                return Err(err(
                    ex.line_number,
                    "correct form of remove_at function: <list>, <index> -> remove_at".to_string(),
                ));
            }
            assign_to(format!("cv_remove_at(&{}, {})", target, a[0]))
        }
        // chap remove_item has no output
        "removeitem" | "rmit" => {
            let target = var_param(
                ex,
                0,
                "remove_item needs a list or map variable as first param",
            )?;
            let a = args(1)?;
            if a.len() != 1 {
                return Err(err(
                    ex.line_number,
                    "correct form of remove_item function: <list | map>, <item | key> -> remove_item"
                        .to_string(),
                ));
            }
            format!("cv_remove_item(&{}, {});", target, a[0])
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
        assert!(c_code.contains("v_a = cv_copy(cv_int(10LL));"));
        assert!(c_code.contains("chap_print(1, v_a);"));
        assert!(c_code.ends_with("    return 0;\n}\n"));
    }

    #[test]
    fn list_and_map_literals_compile() {
        let c_code = super::super::compile_to_c("[1 2 3] -> $l\n{\"a\":1} -> $m").unwrap();
        assert!(c_code.contains("cv_list_lit(3, cv_int(1LL), cv_int(2LL), cv_int(3LL))"));
        assert!(c_code.contains("cv_map_lit(1, \"a\", cv_int(1LL))"));
    }

    #[test]
    fn mutating_functions_need_variables() {
        assert!(super::super::compile_to_c("5 -> insert").is_err());
        assert!(super::super::compile_to_c("[1] -> $l\n$l -> pop -> $x").is_ok());
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
    fn skipped_functions_are_error() {
        assert!(super::super::compile_to_c("\"url\" -> http_get").is_err());
        assert!(super::super::compile_to_c("{\"a\":1} -> to_json").is_err());
        assert!(super::super::compile_to_c("1 -> from_json -> $x").is_err());
    }
}
