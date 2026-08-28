pub use crate::initialization;

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub enum ServiceTracingFormat {
    Json,
    Text,
}
