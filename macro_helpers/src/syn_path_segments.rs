#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    newtype::FromInner,
    newtype::IntoInnerFrom,
    newtype::ToTokens,
)]
pub struct SynPathSegments(syn::punctuated::Punctuated<syn::PathSegment, syn::token::PathSep>);

impl SynPathSegments {
    #[cfg(test)]
    pub(crate) fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
