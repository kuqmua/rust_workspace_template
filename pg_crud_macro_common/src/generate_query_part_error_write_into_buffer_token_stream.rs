#![allow(
    clippy::wildcard_imports,
    reason = "split owner modules import the private facade vocabulary used by the moved generator"
)]

#[must_use]
pub fn generate_query_part_error_write_into_buffer_token_stream(
    import: crate::import::Import,
) -> macro_helpers::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream {
    quote::quote! {
        #import::query_part_error::QueryPartError::WriteIntoBuffer {
            location: proc_macro_location_bang::location!()
        }
    }
    .into()
}
