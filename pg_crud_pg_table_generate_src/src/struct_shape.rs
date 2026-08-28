pub(super) fn struct_shape(
    input: workspace_macro_helpers::domain_types::SynDeriveInputRef<'_>,
) -> syn::Result<workspace_macro_helpers::domain_types::SynStructShapeRef<'_>> {
    workspace_macro_helpers::domain_types::SynStructShapeRef::try_from(input.get())
}
