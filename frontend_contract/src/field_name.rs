#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    proc_macro_newtype_as_ref_str::AsRefStr,
    proc_macro_newtype_display::Display,
    proc_macro_newtype_from_inner::FromInner,
    proc_macro_newtype_into_inner_from::IntoInnerFrom,
)]
pub struct FieldName(crate::contract_str::ContractStr);
