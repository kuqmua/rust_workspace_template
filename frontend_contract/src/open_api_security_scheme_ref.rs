#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    newtype::FromInner,
    newtype::GetInner,
)]
pub struct OpenApiSecuritySchemeRef<'value_lt>(&'value_lt str);
