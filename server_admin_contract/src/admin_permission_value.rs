#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    PartialEq,
    Eq,
    proc_macro_newtype_bounded_string_wrapper::BoundedStringWrapper,
    proc_macro_newtype_as_ref_owned::AsRefOwned,
    proc_macro_newtype_display::Display,
    proc_macro_newtype_into_inner::IntoInner,
)]
#[bounded_string(
    max = 128,
    chars,
    serde,
    utoipa,
    description = "administrator permission"
)]
pub struct AdminPermissionValue(bounded_types::bounded_string::BoundedString<0usize, 128, true>);
