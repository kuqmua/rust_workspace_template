pub fn generate_impl_sqlx_type_for_identifier_token_stream(
    identifier_token_stream: &dyn quote::ToTokens,
    type_token_stream: &dyn quote::ToTokens,
) -> macro_helpers::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream {
    quote::quote! {
        impl sqlx::Type<sqlx::Postgres> for #identifier_token_stream {
            fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
                <#type_token_stream as sqlx::Type<sqlx::Postgres>>::compatible(ty)
            }
            fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
               <#type_token_stream as sqlx::Type<sqlx::Postgres>>::type_info()
            }
        }
    }
    .into()
}
