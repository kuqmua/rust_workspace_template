#![allow(
    clippy::wildcard_imports,
    reason = "split owner modules import the private facade vocabulary used by the moved generator"
)]

pub fn generate_impl_sqlx_decode_sqlx_pg_for_identifier_token_stream(
    identifier_token_stream: &dyn quote::ToTokens,
    type_token_stream: &dyn quote::ToTokens,
    ok_v_match_token_stream: &dyn quote::ToTokens,
) -> macro_helpers::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream {
    let names = crate::names_context::NamesContext::new();

    #[allow(
        non_snake_case,
        reason = "generate impl sqlx decode sqlx pg for identifier token stream requires this localized allowance for generated or framework-constrained code verified by focused tests"
    )]
    let (ValueSnakeCase,) = (names.get_value_snake_case(),);
    quote::quote! {
        impl sqlx::Decode<'_, sqlx::Postgres> for #identifier_token_stream {
            fn decode(#ValueSnakeCase: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
                match <#type_token_stream as sqlx::Decode<sqlx::Postgres>>::decode(#ValueSnakeCase) {
                    Ok(v) => #ok_v_match_token_stream,
                    Err(error) => Err(error),
                }
            }
        }
    }.into()
}
