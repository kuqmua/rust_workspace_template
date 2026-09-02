#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
#[error(transparent)]
#[derive(proc_macro_newtype::FromInner)]
pub struct TokioChildProcessJoinError(tokio::task::JoinError);
