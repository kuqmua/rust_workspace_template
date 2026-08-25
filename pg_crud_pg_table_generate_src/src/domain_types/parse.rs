#![allow(
    clippy::single_call_fn,
    reason = "table parsing has a private physical boundary from descriptor and token emitters"
)]
pub(super) fn struct_shape(
    input: workspace_macro_helpers::domain_types::SynDeriveInputRef<'_>,
) -> syn::Result<workspace_macro_helpers::domain_types::SynStructShapeRef<'_>> {
    workspace_macro_helpers::domain_types::SynStructShapeRef::try_from(input.get())
}
