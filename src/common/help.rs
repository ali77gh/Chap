pub fn show_help() {
    println!("┌───────────────────────────────────────────────┐");
    println!("│                 Chap Language                 │");
    println!("│                                               │");
    println!("│  https://github.com/ali77gh/Chap              │");
    println!("│                                               │");
    println!("│  Run script file:                             │");
    println!("│      $ chap <file_name>                       │");
    println!("│                                               │");
    println!("│  Compile to C:                                │");
    println!("│      $ chap --to-c <file_name> [output.c]     │");
    println!("│      note: compile the C code with -lm        │");
    println!("│          $ gcc output.c -o output -lm         │");
    println!("│                                               │");
    println!("│  Compile to executable (needs gcc):           │");
    println!("│      $ chap compile <file_name> [output]      │");
    println!("│                                               │");
    println!("│  Run REPL mode:                               │");
    println!("│      $ chap                                   │");
    println!("│                                               │");
    println!("│  Options:                                     │");
    println!("│   -h, --help                                  │");
    println!("│   -v, --version                               │");
    println!("│                                               │");
    println!("└───────────────────────────────────────────────┘");
}
