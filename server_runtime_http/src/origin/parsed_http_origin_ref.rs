#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug)]
pub(super) struct ParsedHttpOriginRef<'text> {
    pub(super) authority: super::HttpOriginTextRef<'text>,
    pub(super) scheme: super::HttpOriginTextRef<'text>,
}
