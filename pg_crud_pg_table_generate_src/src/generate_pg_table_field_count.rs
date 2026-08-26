#[allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the table-model owner validates this private generated field count"
)]
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    newtype::DerefInner,
    newtype::FromInner,
    newtype::IntoInnerFrom,
)]
pub struct GeneratePgTableFieldCount(usize);
