#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
#[error("{message}", message = constants_str::METRICS_RESPONSE_BODY_EXCEEDS_MAXIMUM_LENGTH)]
pub struct MetricsResponseBodyError;
