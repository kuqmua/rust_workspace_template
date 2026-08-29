#![allow(
    clippy::wildcard_imports,
    reason = "split owner modules import the private facade vocabulary used by the moved generator"
)]

pub fn generate_impl_sqlx_type_and_encode_for_identifier_token_stream(
    identifier_token_stream: &dyn quote::ToTokens,
    type_token_stream: &dyn quote::ToTokens,
    encode_token_stream: &dyn quote::ToTokens,
) -> macro_helpers::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream {
    let impl_type_token_stream = crate::generate_impl_sqlx_type_for_identifier_token_stream::generate_impl_sqlx_type_for_identifier_token_stream(
        identifier_token_stream,
        type_token_stream,
    );
    let impl_encode_token_stream = crate::generate_impl_sqlx_encode_sqlx_pg_for_identifier_token_stream::generate_impl_sqlx_encode_sqlx_pg_for_identifier_token_stream(
        identifier_token_stream,
        encode_token_stream,
    );
    quote::quote! {
        #impl_type_token_stream
        #impl_encode_token_stream
    }
    .into()
}
