#[cfg(test)]
#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
#[allow(
    dead_code,
    reason = "the marker makes the integration-test-only derive dependency explicit"
)]
struct GenerateAccessorTestDependencyMarker;

#[proc_macro_derive(Getters, attributes(getters))]
pub fn getters(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    workspace_macro_helpers::generate_private_field_getters::generate_private_field_getters(
        workspace_macro_helpers::proc_macro2_macro_tokens::ProcMacro2MacroTokens::from_into(
            token_stream,
        ),
    )
    .into_inner()
    .into()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_getter_shared_owner_preserves_non_struct_diagnostic() {
        let input = quote::quote! {
            #[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
            enum GetterRejectedInput { Value }
        };
        let generated =
            workspace_macro_helpers::generate_private_field_getters::generate_private_field_getters(
                workspace_macro_helpers::proc_macro2_macro_tokens::ProcMacro2MacroTokens::from(
                    input,
                ),
            );
        let output_text = generated.to_string();
        let syntax = syn::parse2::<syn::File>(proc_macro2::TokenStream::from(generated));
        assert!(matches!(syntax, Ok(file) if file.items.len() == constants_usize::ONE));
        assert!(output_text.contains(constants_str::GETTERS_REQUIRES_STRUCT));
    }
}
