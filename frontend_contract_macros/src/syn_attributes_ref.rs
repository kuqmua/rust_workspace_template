#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, newtype::FromInner, newtype::GetInner,
)]
pub(crate) struct SynAttributesRef<'attributes_lt>(&'attributes_lt [syn::Attribute]);
