pub fn generate_impl_try_from_token_stream(
    from_type_token_stream: &dyn quote::ToTokens,
    for_type_token_stream: &dyn quote::ToTokens,
    error_type_token_stream: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> crate::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream {
    let v_snake_case = naming::VSnakeCase;
    quote::quote! {
        impl TryFrom<#from_type_token_stream> for #for_type_token_stream {
            type Error = #error_type_token_stream;
            fn try_from(#v_snake_case: #from_type_token_stream) -> Result<Self, Self::Error> {
                #ts
            }
        }
    }
    .into()
}
