#[proc_macro]
pub fn trait_alias(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    workspace_macro_helpers::generate_trait_alias::generate_trait_alias(
        workspace_macro_helpers::proc_macro2_macro_tokens::ProcMacro2MacroTokens::from_into(
            token_stream,
        ),
    )
    .into_inner()
    .into()
}
