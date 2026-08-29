#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug)]
pub(super) struct ParsedHttpOriginRef<'text> {
    pub(super) authority: crate::http_origin_text_ref::HttpOriginTextRef<'text>,
    pub(super) scheme: crate::http_origin_text_ref::HttpOriginTextRef<'text>,
}
