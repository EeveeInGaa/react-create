use std::{fs, path::Path};

use assert_cmd::Command;
use predicates::str::contains;
use tempfile::tempdir;

fn run_interface_generator(name: &str, target_path: &Path) {
    Command::cargo_bin("rea")
        .expect("binary should exist")
        .args([
            "g",
            "i",
            name,
            "--path",
            target_path.to_str().expect("path should be valid UTF-8"),
        ])
        .assert()
        .success();
}

#[test]
fn should_generate_interface_files_for_different_names() {
    let test_cases = [
        ("user", "User", "User.ts"),
        ("user-profile", "UserProfile", "UserProfile.ts"),
        ("user_profile", "UserProfile", "UserProfile.ts"),
        ("UserProfile", "UserProfile", "UserProfile.ts"),
        ("USER_PROFILE", "UserProfile", "UserProfile.ts"),
    ];

    for (input_name, expected_interface_name, expected_file_name) in test_cases {
        let temp_dir = tempdir().expect("temp dir should be created");
        let target_path = temp_dir.path().join("src/interfaces");

        run_interface_generator(input_name, &target_path);

        let interface_path = target_path.join(expected_file_name);

        assert!(
            interface_path.exists(),
            "Input '{input_name}' should create file '{}' at '{}'",
            expected_file_name,
            target_path.display()
        );

        let content = fs::read_to_string(&interface_path).expect("file should be readable");

        assert_eq!(
            content,
            format!("export interface {expected_interface_name} {{\n}}\n"),
            "Input '{input_name}' should create the expected interface content"
        );
    }
}

#[test]
fn should_fail_if_interface_file_already_exists() {
    let temp_dir = tempdir().expect("temp dir should be created");
    let target_path = temp_dir.path().join("src/interfaces");

    run_interface_generator("user", &target_path);

    Command::cargo_bin("rea")
        .expect("binary should exist")
        .args([
            "g",
            "i",
            "user",
            "--path",
            target_path.to_str().expect("path should be valid UTF-8"),
        ])
        .assert()
        .failure()
        .stderr(contains("File already exists"));
}

#[test]
fn should_reject_component_only_options_for_interfaces() {
    let temp_dir = tempdir().expect("temp dir should be created");
    let target_path = temp_dir.path().join("src/interfaces");

    Command::cargo_bin("rea")
        .expect("binary should exist")
        .args([
            "g",
            "i",
            "user",
            "--css",
            "--path",
            target_path.to_str().expect("path should be valid UTF-8"),
        ])
        .assert()
        .failure()
        .stderr(contains("--css"));
}