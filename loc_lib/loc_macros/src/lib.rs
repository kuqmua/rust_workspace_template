#[proc_macro]
pub fn loc(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    drop(input);
    quote::quote! {
        loc_lib::loc::Loc::new(file!().to_owned(), line!(), column!(), None)
    }
    .into()
}
