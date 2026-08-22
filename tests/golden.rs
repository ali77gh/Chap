// Golden tests: every .chap file in tests/data is executed by both the
// interpreter and a C-compiled binary, their outputs must be identical.
// An optional <name>.stdin file is fed to both runs.
// Skipped when no C compiler is available (override with CHAP_CC).

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

fn cc() -> Option<String> {
    let cc = std::env::var("CHAP_CC").unwrap_or_else(|_| "cc".to_string());
    match Command::new(&cc)
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .output()
    {
        Ok(_) => Some(cc),
        Err(_) => None,
    }
}

fn stdin_file(source: &Path) -> Option<PathBuf> {
    let path = source.with_extension("stdin");
    path.is_file().then_some(path)
}

fn run(mut command: Command) -> Vec<u8> {
    let output = command.output().expect("failed to run process");
    assert!(
        output.status.success(),
        "process failed:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
    output.stdout
}

#[test]
fn golden() {
    let Some(cc) = cc() else {
        eprintln!("skipping golden tests: no C compiler found");
        return;
    };

    let data_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/data");
    let mut sources: Vec<PathBuf> = fs::read_dir(&data_dir)
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| p.extension().is_some_and(|ext| ext == "chap"))
        .collect();
    sources.sort();

    assert!(!sources.is_empty(), "no .chap files found in tests/data");

    let build_dir = std::env::temp_dir().join("chap_golden");
    fs::create_dir_all(&build_dir).unwrap();

    for source in sources {
        let name = source.file_stem().unwrap().to_string_lossy().to_string();
        let stdin_path = stdin_file(&source);

        // reference: interpreter output
        let mut interpreter = Command::new(env!("CARGO_BIN_EXE_chap"));
        interpreter.arg(&source);
        if let Some(path) = &stdin_path {
            interpreter.stdin(fs::File::open(path).unwrap());
        }
        let expected = run(interpreter);

        // generate and compile C code
        let c_code = chap::to_c::compile_to_c(&fs::read_to_string(&source).unwrap())
            .unwrap_or_else(|e| panic!("{}: {}", name, e.error_message()));
        let c_file = build_dir.join(format!("{}.c", name));
        fs::write(&c_file, c_code).unwrap();

        let binary = build_dir.join(&name);
        let compile = Command::new(&cc)
            .arg("-O2")
            .arg(&c_file)
            .arg("-o")
            .arg(&binary)
            .arg("-lm")
            .output()
            .expect("failed to run C compiler");
        assert!(
            compile.status.success(),
            "C compilation failed for {}:\n{}",
            name,
            String::from_utf8_lossy(&compile.stderr)
        );

        // compiled binary output
        let mut compiled = Command::new(&binary);
        if let Some(path) = &stdin_path {
            compiled.stdin(fs::File::open(path).unwrap());
        }
        let actual = run(compiled);

        assert!(
            expected == actual,
            "{}: outputs differ\nexpected:\n{}\nactual:\n{}",
            name,
            String::from_utf8_lossy(&expected),
            String::from_utf8_lossy(&actual)
        );
    }
}
