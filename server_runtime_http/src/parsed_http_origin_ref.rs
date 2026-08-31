#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug)]
pub(super) struct ParsedHttpOriginRef<'text> {
    authority: crate::http_origin_text_ref::HttpOriginTextRef<'text>,
    scheme: crate::http_origin_text_ref::HttpOriginTextRef<'text>,
}

impl<'text> ParsedHttpOriginRef<'text> {
    pub(crate) const fn authority(self) -> crate::http_origin_text_ref::HttpOriginTextRef<'text> {
        self.authority
    }

    pub(crate) const fn scheme(self) -> crate::http_origin_text_ref::HttpOriginTextRef<'text> {
        self.scheme
    }
}

impl<'text>
    From<(
        crate::http_origin_text_ref::HttpOriginTextRef<'text>,
        crate::http_origin_text_ref::HttpOriginTextRef<'text>,
    )> for ParsedHttpOriginRef<'text>
{
    fn from(
        (authority, scheme): (
            crate::http_origin_text_ref::HttpOriginTextRef<'text>,
            crate::http_origin_text_ref::HttpOriginTextRef<'text>,
        ),
    ) -> Self {
        Self { authority, scheme }
    }
}
