#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
#[error(transparent)]
#[derive(proc_macro_newtype_from_inner::FromInner)]
pub struct ChildProcessIoError(std::io::Error);
