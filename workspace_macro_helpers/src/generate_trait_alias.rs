pub fn generate_trait_alias(
    proc_macro2_macro_tokens: crate::proc_macro2_macro_tokens::ProcMacro2MacroTokens,
) -> crate::proc_macro2_macro_tokens::ProcMacro2MacroTokens {
    let mut tokens = proc_macro2_macro_tokens.into_iter();
    let Some(name_identifier) = tokens
        .next()
        .and_then(|token| syn::parse2::<syn::Ident>(token.into()).ok())
    else {
        return crate::compile_error_token_stream::compile_error_token_stream(
            constants_str::COMPILE_ERROR_CE_079,
        );
    };
    if !matches!(tokens.next(), Some(proc_macro2::TokenTree::Punct(punctuation)) if punctuation.as_char() == '=')
    {
        return crate::compile_error_token_stream::compile_error_token_stream(
            constants_str::COMPILE_ERROR_CE_079,
        );
    }
    let Ok(bounds_token_stream) = tokens
        .collect::<proc_macro2::TokenStream>()
        .to_string()
        .parse::<proc_macro2::TokenStream>()
    else {
        return crate::compile_error_token_stream::compile_error_token_stream(
            constants_str::COMPILE_ERROR_CE_080,
        );
    };
    quote::quote! {
        pub trait #name_identifier: #bounds_token_stream {}
        impl<T: #bounds_token_stream> #name_identifier for T {}
    }
    .into()
}
