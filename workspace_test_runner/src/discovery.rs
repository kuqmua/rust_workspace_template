#![allow(clippy::single_call_fn)] // discovery remains a separate responsibility even when a mode has one orchestration caller
pub(super) fn mode() -> Option<super::RunnerMode> {
    std::env::args()
        .nth(1)
        .map(|value| super::RunnerMode::try_from(value).unwrap_or_else(super::RunnerMode::from))
}
pub(super) fn tool_available(path: super::ToolPath) -> super::ToolAvailable {
    super::ToolAvailable::from(std::path::Path::new(path.get()).exists())
}
