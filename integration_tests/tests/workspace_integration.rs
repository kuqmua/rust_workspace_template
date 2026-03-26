#![cfg(test)]
use integration_tests as _;
use std::path::PathBuf;
use std::process::Command;
fn cargo_bin(name: &str) -> Command {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let _popped = path.pop();
    path.push("target");
    path.push("debug");
    path.push(name);
    Command::new(path)
}
#[test]
fn server_binary_runs_and_exits_successfully() {
    let output = cargo_bin("server")
        .output()
        .expect("a1b2c3d4: failed to execute server binary");
    assert!(
        output.status.success(),
        "server exited with non-zero status: {:?}",
        output.status
    );
}
#[test]
fn server_binary_prints_hello_world() {
    let output = cargo_bin("server")
        .output()
        .expect("e5f6a7b8: failed to execute server binary");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(stdout.trim(), "Hello, world!");
}
#[test]
fn server_binary_produces_no_stderr() {
    let output = cargo_bin("server")
        .output()
        .expect("c9d0e1f2: failed to execute server binary");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.is_empty(),
        "server produced unexpected stderr: {stderr}"
    );
}
