#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    newtype::FromInner,
    generate_accessor::Getters,
)]
pub(crate) struct AdminSessionPath(crate::admin_session_id::AdminSessionId);
