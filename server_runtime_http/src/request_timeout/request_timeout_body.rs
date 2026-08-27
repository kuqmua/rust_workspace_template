#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, serde::Serialize)]
pub(super) struct RequestTimeoutBody {
    pub(super) error: super::StdRequestTimeoutMessage,
}
