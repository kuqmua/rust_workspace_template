#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(super) struct SynLitStr(pub(super) syn::LitStr);

impl From<syn::LitStr> for SynLitStr {
    fn from(value: syn::LitStr) -> Self {
        Self(value)
    }
}

impl syn::parse::Parse for SynLitStr {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        input.parse().map(Self)
    }
}
