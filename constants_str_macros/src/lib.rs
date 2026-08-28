#[path = "domain_types.rs"]
mod domain_types;

#[proc_macro]
pub fn define_str_constants(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    match syn::parse::<domain_types::DefineStrConstantsInput>(input) {
        Ok(parsed) => proc_macro::TokenStream::from(proc_macro2::TokenStream::from(parsed)),
        Err(error) => proc_macro::TokenStream::from(error.into_compile_error()),
    }
}
