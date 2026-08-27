pub fn generate_impl_sqlx_encode_sqlx_pg_for_identifier_token_stream(
    identifier_token_stream: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream{
    quote::quote! {
        impl sqlx::Encode<'_, sqlx::Postgres> for #identifier_token_stream {
            fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
                sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&#ts, buf)
            }
        }
    }.into()
}
