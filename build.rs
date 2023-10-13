use std::io::Write;

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();

    gen_insn_tests(&out_dir);
}

fn gen_insn_tests(out_dir: &str) {
    let destination = std::path::Path::new(&out_dir).join("tests_insn.rs");
    let mut f = std::fs::File::create(&destination).unwrap();

    let tests = std::fs::read_dir("tests/instructions")
        .expect("Could not list files in tests/instructions")
        .filter_map(|r| {
            r.ok()
                .filter(|f| f.path().is_file() && f.path().extension().is_some_and(|e| e == "S"))
        });

    for test in tests {
        let testcase = match test.path().canonicalize() {
            Ok(testcase) => testcase,
            Err(e) => {
                println!(
                    "cargo:warning=Could not get test file for test {:?}: {e}",
                    test.path()
                );
                continue;
            }
        };

        let binary = match test.path().with_extension("bin").canonicalize() {
            Ok(testcase) => testcase,
            Err(e) => {
                println!(
                    "cargo:warning=Could not get test binary for test {:?}: {e}",
                    test.path()
                );
                continue;
            }
        };

        let testcase = testcase
            .to_str()
            .expect("Could not convert path to string!")
            .escape_default();
        let binary = binary
            .to_str()
            .expect("Could not convert path to string!")
            .escape_default();

        let name = test.path().with_extension("");
        let name = name
            .file_name()
            .and_then(std::ffi::OsStr::to_str)
            .expect("Could not get test name!");

        println!("cargo:rerun-if-changed={testcase}");
        println!("cargo:rerun-if-changed={binary}");

        write!(
            f,
            "
#[test]
/// autogenerated test for instruction {name}
fn test_insn_{name}() {{
    execute_insn_test(\"{name}\", include_str!(\"{testcase}\"), include_bytes!(\"{binary}\"));
}}"
        )
        .unwrap();
    }
}
