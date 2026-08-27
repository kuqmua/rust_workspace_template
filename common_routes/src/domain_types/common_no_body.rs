#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    serde::Deserialize,
    serde::Serialize,
    utoipa::ToSchema,
)]
pub struct CommonNoBody;
