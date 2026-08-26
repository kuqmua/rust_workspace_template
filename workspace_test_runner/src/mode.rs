#![allow(clippy::single_call_fn)] // discovery remains a separate responsibility even when a mode has one orchestration caller
pub(crate) fn mode() -> Option<crate::domain_types::RunnerMode> {
    std::env::args().nth(1).map(|value| {
        crate::domain_types::RunnerMode::try_from(value)
            .unwrap_or_else(crate::domain_types::RunnerMode::from)
    })
}
