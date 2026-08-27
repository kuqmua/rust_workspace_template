use super::OpenApiContractTextTryFromStringError;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    newtype::Display,
    newtype::FromInner,
)]
pub struct OpenApiContractTextError(OpenApiContractTextTryFromStringError);
