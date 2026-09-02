#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    proc_macro_newtype::FromInner,
)]
pub struct SynVariantRef<'variant_lt>(&'variant_lt syn::Variant);

impl<'variant_lt> SynVariantRef<'variant_lt> {
    pub(crate) const fn variant(self) -> &'variant_lt syn::Variant {
        self.0
    }
}
