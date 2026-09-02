#[proc_macro]
pub fn trait_alias(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let text = token_stream.to_string();
    let Some((name, bounds)) = text.split_once('=') else {
        return workspace_macro_helpers::compile_error_token_stream::compile_error_token_stream(
            constants_str::COMPILE_ERROR_CE_079,
        )
        .into_inner()
        .into();
    };
    let name_identifier = quote::format_ident!("{}", name.trim());
    let Ok(bounds_token_stream) = bounds.parse::<proc_macro2::TokenStream>() else {
        return workspace_macro_helpers::compile_error_token_stream::compile_error_token_stream(
            constants_str::COMPILE_ERROR_CE_080,
        )
        .into_inner()
        .into();
    };
    quote::quote! {
        pub trait #name_identifier: #bounds_token_stream {}
        impl<T: #bounds_token_stream> #name_identifier for T {}
    }
    .into()
}
