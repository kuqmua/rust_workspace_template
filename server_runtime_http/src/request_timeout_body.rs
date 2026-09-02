#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    proc_macro_new::New,
    serde::Serialize,
)]
#[constructor(pub(crate))]
pub(super) struct RequestTimeoutBody {
    error: crate::std_request_timeout_message::StdRequestTimeoutMessage,
}
