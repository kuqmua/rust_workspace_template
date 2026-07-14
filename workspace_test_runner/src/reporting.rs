#![allow(clippy::single_call_fn)] // reporting stays independent from command execution so diagnostics have one owner
pub(super) fn allocation_tool(name: &str, path: &str, available: bool) {
    println!("measurement=allocation_tool_available tool={name} path={path} available={available}");
}
pub(super) fn result_directory_failed(error: &std::io::Error) {
    eprintln!("failed to create test result directory: {error}");
}
pub(super) fn result_log_failed(path: &std::path::Path, error: &std::io::Error) {
    eprintln!(
        "failed to write test result log {}: {error}",
        path.display()
    );
}
pub(super) fn result_summary_failed(error: &std::io::Error) {
    eprintln!("failed to write test result summary: {error}");
}
