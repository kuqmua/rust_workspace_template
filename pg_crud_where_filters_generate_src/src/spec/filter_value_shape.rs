#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy)]
pub(super) enum FilterValueShape {
    Scalar,
    Text,
}
