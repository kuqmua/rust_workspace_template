#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, newtype::FromInner)]
pub(super) struct PgTableCompileErrorMessage<'message_lt>(pub(super) &'message_lt str);

impl<'message_lt> From<&'message_lt String> for PgTableCompileErrorMessage<'message_lt> {
    fn from(value: &'message_lt String) -> Self {
        Self(value.as_str())
    }
}
