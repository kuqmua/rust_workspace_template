#[path = "capture.rs"]
pub mod capture;
#[path = "initialization.rs"]
pub mod initialization;

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub enum ServiceTracingFormat {
    Json,
    Text,
}
