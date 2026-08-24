#![allow(
    clippy::single_call_fn,
    reason = "table parsing has a private physical boundary from descriptor and token emitters"
)]
pub(super) fn struct_shape(
    input: workspace_macro_helpers::SynDeriveInputRef<'_>,
) -> syn::Result<workspace_macro_helpers::SynStructShapeRef<'_>> {
    workspace_macro_helpers::SynStructShapeRef::try_from(input.get())
}
