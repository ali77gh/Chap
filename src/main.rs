
mod common; // language common defenitions

//phases
mod preprocessor;// phase 1
mod parser;      // phase 2
mod runtime;     // phase 3

// cli 
mod repl;
mod file_executor; 
mod arg_paresr;

use crate::common::{
    help::show_help,
    version::show_version,
    errors::Result
};
use crate::file_executor::file_executor;
use crate::repl::start_rpel;
use crate::arg_paresr::{arg_parser,InputType};

fn main() -> Result<()> {

    match arg_parser() {
        InputType::ExecuteFile(file_name) => {
            file_executor(&file_name)?;
        },
        InputType::Help => show_help(),
        InputType::Version => show_version(),
        InputType::Repl => start_rpel(),
    }

    Ok(())
}
