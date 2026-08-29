pub fn generate_match_try_new_in_de_token_stream(
    identifier: &dyn quote::ToTokens,
    initialization_token_stream: &dyn quote::ToTokens,
) -> macro_helpers::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream {
    quote::quote! {
        match #identifier::try_new(#initialization_token_stream) {
            Ok(v) => Ok(v),
            Err(error) => Err(serde::de::Error::custom(format!("{error:?}")))
        }
    }
    .into()
}
