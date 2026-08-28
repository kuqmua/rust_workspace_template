#[must_use]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub enum FallbackResponseMode {
    HumanReadable,
    MachineReadable,
}
