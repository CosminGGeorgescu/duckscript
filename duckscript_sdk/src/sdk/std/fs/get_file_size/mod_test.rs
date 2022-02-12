use super::*;
use crate::test;
use crate::test::CommandValidation;

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_path_provided() {
    test::run_script_and_error(
        vec![create("")],
        "out = get_file_size",
        "out"
    );
}

#[test]
fn run_file() {
    test::run_script_and_validate(
        vec![create("")],
        "out = get_file_size ./Cargo.toml",
        CommandValidation::PositiveNumber("out".to_string()),
    );
}

#[test]
fn run_directory() {
    test::run_script_and_error(
        vec![create("")],
        "out = get_file_size ./src",
        "out",
    );
}

#[test]
fn run_not_found() {
    test::run_script_and_error(
        vec![create("")],
        "out = get_file_size ./badpath",
        "out",
    );
}
