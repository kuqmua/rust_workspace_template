#[must_use]
pub fn pg_crud_common_query_part_error_checked_add_initialization_token_stream()
-> macro_helpers::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream {
    quote::quote! {pg_crud_common::query_part_error::QueryPartError::CheckedAdd { location: location_macros::location!() }}.into()
}
