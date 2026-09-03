#[proc_macro_derive(GenerateAccessorTraitsForStructFields)]
pub fn generate_accessor_traits_for_struct_fields(
    token_stream: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    proc_macro_generate_accessor_traits_for_struct_fields_shared::generate_accessor_traits_for_struct_fields(token_stream.into()).into()
}
