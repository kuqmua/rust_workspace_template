#[proc_macro]
pub fn location(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    drop(input);
    quote::quote! {
        location_lib::location::Location::new(file!(), line!(), column!(), None)
    }
    .into()
}
