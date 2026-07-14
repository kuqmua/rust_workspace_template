#![allow(clippy::single_call_fn)] // discovery remains a separate responsibility even when a mode has one orchestration caller
pub(super) fn mode() -> Option<String> {
    std::env::args().nth(1)
}
pub(super) fn tool_available(path: &str) -> bool {
    std::path::Path::new(path).exists()
}
