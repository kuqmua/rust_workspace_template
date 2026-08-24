#![allow(clippy::single_call_fn)] // reporting stays independent from command execution so diagnostics have one owner
pub(super) fn allocation_tool(
    name: super::ToolName,
    path: super::ToolPath,
    available: super::ToolAvailable,
) {
    println!(
        "measurement=allocation_tool_available tool={} path={} available={}",
        name.get(),
        path.get(),
        available.get()
    );
}
pub(super) fn result_directory_failed(error: super::RunnerIoErrorRef<'_>) {
    eprintln!("failed to create test result directory: {}", error.get());
}
pub(super) fn result_log_failed(
    path: super::RunnerPathRef<'_>,
    error: super::RunnerIoErrorRef<'_>,
) {
    eprintln!(
        "failed to write test result log {}: {}",
        path.get().display(),
        error.get()
    );
}
pub(super) fn result_summary_failed(error: super::RunnerIoErrorRef<'_>) {
    eprintln!("failed to write test result summary: {}", error.get());
}
