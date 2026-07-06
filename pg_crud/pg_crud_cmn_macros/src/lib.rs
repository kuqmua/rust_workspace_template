#[proc_macro]
pub fn trait_al(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let text = input.to_string();
    let Some((name, bounds)) = text.split_once('=') else {
        return workspace_macro_helpers::compile_error_ts("trait_al expects Name = Bounds").into();
    };
    let name_ident = quote::format_ident!("{}", name.trim());
    let Ok(bounds_ts) = bounds.parse::<proc_macro2::TokenStream>() else {
        return workspace_macro_helpers::compile_error_ts("trait_al failed to parse bounds").into();
    };
    quote::quote! {
        pub trait #name_ident: #bounds_ts {}
        impl<T: #bounds_ts> #name_ident for T {}
    }
    .into()
}
