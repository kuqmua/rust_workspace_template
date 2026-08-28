#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    thiserror::Error,
    newtype::FromInner,
    newtype::IntoInnerFrom,
)]
#[error(transparent)]
pub struct SynGeneratePgTablePipelineError(syn::Error);
