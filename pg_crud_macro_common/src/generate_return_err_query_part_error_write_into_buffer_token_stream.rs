#![allow(
    clippy::wildcard_imports,
    reason = "split owner modules import the private facade vocabulary used by the moved generator"
)]
use super::domain_types::*;

#[must_use]
pub fn generate_return_err_query_part_error_write_into_buffer_token_stream(
    import: Import,
) -> macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream{
    let ts = generate_query_part_error_write_into_buffer_token_stream(import);
    quote::quote! {return Err(#ts);}.into()
}
