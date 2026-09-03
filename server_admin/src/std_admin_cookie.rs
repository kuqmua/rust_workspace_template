#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    PartialEq,
    Eq,
    proc_macro_newtype_bounded_string_wrapper::BoundedStringWrapper,
    proc_macro_newtype_as_ref_owned::AsRefOwned,
    proc_macro_newtype_into_inner::IntoInner,
)]
#[bounded_string(max = 8192, description = "administrator cookie")]
pub struct StdAdminCookie(bounded_types::bounded_string::BoundedString<0usize, 8192, false>);
