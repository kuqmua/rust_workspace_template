#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner)]
pub(super) struct FormatterRefMut<'fmt_ref_lt, 'fmt_lt>(
    pub(super) &'fmt_ref_lt mut std::fmt::Formatter<'fmt_lt>,
);
