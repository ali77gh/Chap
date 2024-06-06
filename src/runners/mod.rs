// evel should be in bin and lib (file executor is using eval BTW)
pub mod eval;

// file executor should not include in wasm (we can't read/write files on browsers)
#[cfg(not(target_family = "wasm"))]
pub mod file_executor;

// repl should not include in wasm (rustyline can not compile to wasm)
#[cfg(not(target_family = "wasm"))]
pub mod repl;
