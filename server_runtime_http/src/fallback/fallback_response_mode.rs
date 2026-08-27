#[path = "fallback_response_mode/fallback_response_mode.rs"]
mod fallback_response_mode;

pub use fallback_response_mode::fallback_response_mode;

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub enum FallbackResponseMode {
    HumanReadable,
    MachineReadable,
}
