
pub mod common; // language common defenitions

//phases
pub mod preprocessor;    // phase 1
pub mod parser;          // phase 2
pub mod static_analyzer; // phase 3
pub mod optimizer;       // phase 4
pub mod runtime;         // phase 5

//repl and file_executor and param parser should not be in library
