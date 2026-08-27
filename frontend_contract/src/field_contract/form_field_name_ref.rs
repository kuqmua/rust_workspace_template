#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    newtype::AsRefStr,
    newtype::FromInner,
)]
pub struct FormFieldNameRef<'field_lt>(&'field_lt str);
