#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug)]
pub(super) struct ParsedHttpOriginRef<'text> {
    pub(super) authority: super::HttpOriginTextRef<'text>,
    pub(super) scheme: super::HttpOriginTextRef<'text>,
}
