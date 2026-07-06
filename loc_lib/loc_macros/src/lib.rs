use proc_macro::TokenStream;
use quote::quote;
#[proc_macro]
pub fn loc(input: TokenStream) -> TokenStream {
    drop(input);
    quote! {
        loc_lib::loc::Loc::new(file!().to_owned(), line!(), column!(), None)
    }
    .into()
}
