#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    proc_macro_newtype_as_ref_str::AsRefStr,
    proc_macro_newtype_bounded_string_wrapper::BoundedStringWrapper,
    proc_macro_newtype_display::Display,
)]
#[bounded_string(max = constants_usize::VALUE_16_777_216)]
pub(crate) struct ScaffoldText(
    bounded_types::bounded_string::BoundedString<
        0usize,
        { constants_usize::VALUE_16_777_216 },
        false,
    >,
);
