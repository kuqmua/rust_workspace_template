#[allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the table-model owner reads this private syntax input wrapper"
)]
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    newtype::DerefInner,
    newtype::FromInner,
    newtype::IntoInnerFrom,
)]
pub(crate) struct SynGeneratePgTableModelInput(syn::DeriveInput);
