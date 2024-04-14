mod common; // language common defenitions

//phases
mod parser; // phase 2
mod preprocessor; // phase 1
mod runtime; // phase 3

// cli
mod arg_paresr;
mod file_executor;
mod repl;

use crate::arg_paresr::{arg_parser, InputType};
use crate::common::{errors::Result, help::show_help, version::show_version};
use crate::file_executor::file_executor;
use crate::repl::start_rpel;

fn main() -> Result<()> {
    match arg_parser() {
        InputType::ExecuteFile(file_name) => {
            file_executor(&file_name)?;
        }
        InputType::Help => show_help(),
        InputType::Version => show_version(),
        InputType::Repl => start_rpel(),
    }

    Ok(())
}
