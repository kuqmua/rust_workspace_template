#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConfigExampleValidity {
    Invalid,
    Valid,
}
