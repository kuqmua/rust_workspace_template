#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, generate_constructor::New, serde::Serialize,
)]
#[constructor(pub(crate))]
pub(super) struct RequestTimeoutBody {
    error: crate::std_request_timeout_message::StdRequestTimeoutMessage,
}
