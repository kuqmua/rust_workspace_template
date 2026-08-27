#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, newtype::FromInner)]
pub struct SynStatusCodeVariantRef<'variant_lt>(pub(super) &'variant_lt syn::Variant);
