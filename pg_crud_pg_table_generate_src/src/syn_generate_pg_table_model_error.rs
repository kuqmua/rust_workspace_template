#[allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the table-model owner constructs this private syntax error wrapper"
)]
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner, newtype::IntoInnerFrom,
)]
pub(crate) struct SynGeneratePgTableModelError(syn::Error);
