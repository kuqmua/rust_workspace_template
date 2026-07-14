#![allow(clippy::single_call_fn)] // reporting stays independent from command execution so diagnostics have one owner
pub(super) fn allocation_tool(name: &str, path: &str, available: bool) {
    println!("measurement=allocation_tool_available tool={name} path={path} available={available}");
}
pub(super) fn command_failed(program: &str, args: &[&str], status: std::process::ExitStatus) {
    eprintln!("command failed: {program} {args:?}: {status}");
}
pub(super) fn command_spawn_failed(program: &str, args: &[&str], error: &std::io::Error) {
    eprintln!("failed to spawn command: {program} {args:?}: {error}");
}
