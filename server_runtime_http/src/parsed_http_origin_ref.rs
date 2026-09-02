#[derive(proc_macro_getters::Getters)]
#[getters(bare)]
#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug)]
pub(super) struct ParsedHttpOriginRef<'text> {
    #[getters(copy)]
    authority: crate::http_origin_text_ref::HttpOriginTextRef<'text>,
    #[getters(copy)]
    scheme: crate::http_origin_text_ref::HttpOriginTextRef<'text>,
}

impl<'text>
    From<(
        crate::http_origin_text_ref::HttpOriginTextRef<'text>,
        crate::http_origin_text_ref::HttpOriginTextRef<'text>,
    )> for ParsedHttpOriginRef<'text>
{
    fn from(
        value: (
            crate::http_origin_text_ref::HttpOriginTextRef<'text>,
            crate::http_origin_text_ref::HttpOriginTextRef<'text>,
        ),
    ) -> Self {
        let (authority, scheme) = value;
        Self { authority, scheme }
    }
}
