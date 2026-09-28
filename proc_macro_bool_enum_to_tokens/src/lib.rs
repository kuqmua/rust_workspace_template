#[proc_macro]
pub fn bool_enum_to_tokens(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    workspace_macro_helpers::generate_bool_enum_to_tokens::generate_bool_enum_to_tokens(
        workspace_macro_helpers::proc_macro2_macro_tokens::ProcMacro2MacroTokens::from_into(
            token_stream,
        ),
    )
    .into_inner()
    .into()
}
