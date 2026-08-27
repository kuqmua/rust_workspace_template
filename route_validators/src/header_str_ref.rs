#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    newtype::AsRefInner,
    newtype::DerefTarget,
    newtype::FromInner,
)]
pub(crate) struct HeaderStrRef<'header_str_lt>(pub(super) &'header_str_lt str);

#[cfg(test)]
impl<'header_str_lt> HeaderStrRef<'header_str_lt> {
    #[allow(clippy::single_call_fn)] // typed accessor keeps test assertions from exposing the tuple field
    pub(crate) const fn get(self) -> &'header_str_lt str {
        self.0
    }
}
