mod common;

use crate::common::{TestEnv, TestEnvBuilder};
use assert_cmd::prelude::*;

#[test]
fn test_cli_format_rust_basic() {
    let env = TestEnv::new_fixture("./tests/fixtures/cli_format_rust/basic/");

    env.foro(&["format", "./main.rs"]);
    env.assert_eq("main.rs", "expected.rs");
}

#[test]
fn test_cli_format_rust_ignore() {
    let env = TestEnv::new_fixture("./tests/fixtures/cli_format_rust/ignore/");

    env.foro(&["format", "./main.rs"]);
    env.assert_eq("main.rs", "expected.rs");
}

#[test]
fn test_cli_format_rust_with_config() {
    let env = TestEnv::new_fixture("./tests/fixtures/cli_format_rust/with_config/");

    env.foro(&["format", "./main.rs"]);
    env.assert_eq("main.rs", "expected.rs");
}

#[test]
fn test_cli_format_rust_overwrite_config() {
    // rustfmt overwrites the config file.
    // In other words, it uses the rustfmt.toml file in the closest ancestor directory and
    // ignores rustfmt.toml files further away.

    let env = TestEnv::new_fixture("./tests/fixtures/cli_format_rust/overwrite_config/");

    env.foro(&["format", "./src/main.rs"]);
    env.assert_eq("src/main.rs", "src/expected.rs");
}

#[test]
fn test_cli_format_rust_nested_config() {
    let env = TestEnv::new_fixture("./tests/fixtures/cli_format_rust/nested_config/");

    env.foro(&["format", "./nest/src/main.rs"]);
    env.assert_eq("nest/src/main.rs", "nest/src/expected.rs");
}

#[test]
fn test_cli_format_cpp() {
    let env = TestEnv::new_fixture("./tests/fixtures/cli_format_cpp/basic/");

    env.foro(&["format", "./main.cpp"]);
    env.assert_eq("main.cpp", "expected.cpp");
}

#[test]
fn test_cli_format_cpp_ignore() {
    let env = TestEnv::new_fixture("./tests/fixtures/cli_format_cpp/ignore/");

    env.foro(&["format", "./main.cpp"]);
    env.assert_eq("main.cpp", "expected.cpp");
}

#[test]
fn test_cli_format_cpp_with_config() {
    let env = TestEnv::new_fixture("./tests/fixtures/cli_format_cpp/with_config/");

    env.foro(&["format", "./main.cpp"]);
    env.assert_eq("main.cpp", "expected.cpp");
}

#[test]
fn test_cli_format_cpp_disable() {
    let env = TestEnv::new_fixture("./tests/fixtures/cli_format_cpp/disable/");

    env.foro(&["format", "./main.cpp"]);
    env.assert_eq("main.cpp", "expected.cpp");
}

#[test]
fn test_cli_format_cpp_nested_config() {
    let env = TestEnv::new_fixture("./tests/fixtures/cli_format_cpp/nested_config/");

    env.foro(&["format", "./nest/src/main.cpp"]);
    env.assert_eq("nest/src/main.cpp", "nest/src/expected.cpp");
}

#[test]
fn test_cli_format_cpp_overwrite_config() {
    let env = TestEnv::new_fixture("./tests/fixtures/cli_format_cpp/overwrite_config/");

    env.foro(&["format", "./src/main.cpp"]);
    env.assert_eq("src/main.cpp", "src/expected.cpp");
}

#[test]
#[ignore]
fn test_cli_format_go() {
    let env = TestEnv::new_fixture("./tests/fixtures/cli_format_go/basic/");

    env.foro(&["format", "./main.go"]);
    env.assert_eq("main.go", "expected.go");
}

#[test]
fn test_cli_format_ts_basic() {
    let env = TestEnv::new_fixture("./tests/fixtures/cli_format_ts/basic/");

    env.foro(&["format", "./main.ts"]);
    env.assert_eq("main.ts", "expected.ts");
}

#[test]
fn test_cli_format_ts_ignore() {
    let env = TestEnv::new_fixture("./tests/fixtures/cli_format_ts/ignore/");

    env.foro(&["format", "./main.ts"]);
    env.assert_eq("main.ts", "expected.ts");
}

#[test]
fn test_cli_format_ts_with_config() {
    let env = TestEnv::new_fixture("./tests/fixtures/cli_format_ts/with_config/");

    env.foro(&["format", "./main.ts"]);
    env.assert_eq("main.ts", "expected.ts");
}

#[test]
fn test_cli_format_ts_nested_config() {
    // biome does not read biome.json located deeper than the current directory.
    // biome plugin for foro follows this specification.

    let env = TestEnv::new_fixture("./tests/fixtures/cli_format_ts/nested_config/");

    env.foro(&["format", "./nest/src/main.ts"]);
    env.assert_eq("nest/src/main.ts", "nest/src/expected.ts");
}

#[test]
fn test_cli_format_ts_overwrite_config() {
    let env = TestEnvBuilder::new()
        .fixture_path("./tests/fixtures/cli_format_ts/overwrite_config/")
        .work_dir("./root/")
        .build();

    env.foro(&["format", "./main.ts"]);
    env.assert_eq("root/main.ts", "root/expected.ts");
}

#[test]
fn test_cli_format_ts_extend_config() {
    let env = TestEnvBuilder::new()
        .fixture_path("./tests/fixtures/cli_format_ts/extend_config/")
        .work_dir("./root/")
        .build();

    env.foro(&["format", "./main.ts"]);
    env.assert_eq("root/main.ts", "root/expected.ts");
}

#[test]
fn test_cli_format_python() {
    let env = TestEnv::new_fixture("./tests/fixtures/cli_format_python/basic/");

    env.foro(&["format", "./main.py"]);
    env.assert_eq("main.py", "expected.py");
}

#[test]
fn test_cli_format_python_ignore() {
    let env = TestEnv::new_fixture("./tests/fixtures/cli_format_python/ignore/");
    env.foro(&["format", "./main.py"]);
    env.assert_eq("main.py", "expected.py");
}

#[test]
fn test_cli_format_python_with_config() {
    let env = TestEnv::new_fixture("./tests/fixtures/cli_format_python/with_config/");
    env.foro(&["format", "./main.py"]);
    env.assert_eq("main.py", "expected.py");
}

#[test]
fn test_cli_format_python_nested_config() {
    let env = TestEnv::new_fixture("./tests/fixtures/cli_format_python/nested_config/");
    env.foro(&["format", "./nest/src/main.py"]);
    env.assert_eq("nest/src/main.py", "nest/src/expected.py");
}

#[test]
fn test_cli_format_python_overwrite_config() {
    let env = TestEnv::new_fixture("./tests/fixtures/cli_format_python/overwrite_config/");
    env.foro(&["format", "./src/main.py"]);
    env.assert_eq("src/main.py", "src/expected.py");
}

#[test]
fn test_cli_format_python_extend_config() {
    let env = TestEnvBuilder::new()
        .fixture_path("./tests/fixtures/cli_format_python/extend_config/")
        .work_dir("./root/")
        .build();

    env.foro(&["format", "./main.py"]);
    env.assert_eq("root/main.py", "root/expected.py");
}

#[test]
fn test_cli_format_rules() {
    let env = TestEnv::new_fixture("./tests/fixtures/cli_format_rules/");

    let output = env.foro_cmd(&["format", "./main.rs"]).unwrap();

    assert!(output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("File ignored. reason: No rule matched"))
}

#[test]
#[ignore]
fn test_cli_format_no_cache() {
    // This test is ignored because it's really slow! (~30s)

    let env = TestEnvBuilder::new()
        .fixture_path("./tests/fixtures/cli_format_rust/basic/")
        .cache_dir("./cache/")
        .build();

    env.foro(&["format", "./main.rs"]);
    env.assert_eq("main.rs", "expected.rs");
}

#[test]
#[ignore]
fn test_cli_format_parallel() {
    // When two foro formats are running simultaneously and dllpack is downloaded at the same time,
    // cache conflicts may occur (in improper implementations), resulting in errors.

    let env = TestEnvBuilder::new()
        .fixture_path("./tests/fixtures/cli_format_rust/basic/")
        .cache_dir("./cache/")
        .build();

    let mut proc_0 = env.foro_cmd(&["format", "./main.rs"]).spawn().unwrap();
    let mut proc_1 = env.foro_cmd(&["format", "./main.rs"]).spawn().unwrap();

    assert!(proc_0.wait().unwrap().success());
    assert!(proc_1.wait().unwrap().success());
}
