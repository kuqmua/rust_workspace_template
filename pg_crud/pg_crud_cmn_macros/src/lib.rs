use proc_macro::TokenStream;
use proc_macro2::TokenStream as Ts2;
use quote::{format_ident, quote};
use workspace_macro_helpers::compile_error_ts;
#[proc_macro]
pub fn trait_al(input: TokenStream) -> TokenStream {
    let text = input.to_string();
    let Some((name, bounds)) = text.split_once('=') else {
        return compile_error_ts("trait_al expects Name = Bounds").into();
    };
    let name_ident = format_ident!("{}", name.trim());
    let Ok(bounds_ts) = bounds.parse::<Ts2>() else {
        return compile_error_ts("trait_al failed to parse bounds").into();
    };
    quote! {
        pub trait #name_ident: #bounds_ts {}
        impl<T: #bounds_ts> #name_ident for T {}
    }
    .into()
}
