pub(super) fn struct_shape(
    syn_derive_input_ref: workspace_macro_helpers::syn_derive_input_ref::SynDeriveInputRef<'_>,
) -> syn::Result<workspace_macro_helpers::syn_struct_shape_ref::SynStructShapeRef<'_>> {
    workspace_macro_helpers::syn_struct_shape_ref::SynStructShapeRef::try_from(
        syn_derive_input_ref.get(),
    )
}
