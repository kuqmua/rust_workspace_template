#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(super) struct SynVisibility(pub(super) syn::Visibility);

impl From<syn::Visibility> for SynVisibility {
    fn from(value: syn::Visibility) -> Self {
        Self(value)
    }
}

impl syn::parse::Parse for SynVisibility {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        input.parse().map(Self)
    }
}
