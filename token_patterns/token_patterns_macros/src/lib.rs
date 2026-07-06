fn gen_tp(input: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let mut iter = input.into_iter();
    let Some(name) = workspace_macro_helpers::first_ident(&mut iter) else {
        return workspace_macro_helpers::compile_error_ts("tp expects type name");
    };
    if !workspace_macro_helpers::strip_first_comma(&mut iter) {
        return workspace_macro_helpers::compile_error_ts("tp expects comma after type name");
    }
    let body = iter.collect::<proc_macro2::TokenStream>();
    let name_ident = quote::format_ident!("{name}");
    quote::quote! {
        #[derive(Debug, Clone, Copy, Optml)]
        pub struct #name_ident;
        impl ToTokens for #name_ident {
            fn to_tokens(&self, tokens: &mut Ts2) {
                append_tokens(tokens, quote! {#body});
            }
        }
    }
}
#[proc_macro]
pub fn tp(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    gen_tp(input.into()).into()
}
#[proc_macro]
pub fn tp_parts(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut parts = workspace_macro_helpers::split_top_level_commas(input.into());
    if parts.len() < 2 {
        return workspace_macro_helpers::compile_error_ts(
            "tp_parts expects type name and at least one part",
        )
        .into();
    }
    let mut name_iter = parts.remove(0).into_iter();
    let Some(name) = workspace_macro_helpers::first_ident(&mut name_iter) else {
        return workspace_macro_helpers::compile_error_ts("tp_parts expects type name").into();
    };
    let name_ident = quote::format_ident!("{name}");
    let part_streams = parts.into_iter().collect::<Vec<proc_macro2::TokenStream>>();
    quote::quote! {
        #[derive(Debug, Clone, Copy, Optml)]
        pub struct #name_ident;
        impl ToTokens for #name_ident {
            fn to_tokens(&self, tokens: &mut Ts2) {
                #(append_tokens(tokens, #part_streams);)*
            }
        }
    }
    .into()
}
#[proc_macro]
pub fn ts_path_fn(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut iter = proc_macro2::TokenStream::from(input).into_iter();
    let Some(name) = workspace_macro_helpers::first_ident(&mut iter) else {
        return workspace_macro_helpers::compile_error_ts("ts_path_fn expects function name")
            .into();
    };
    if !workspace_macro_helpers::strip_first_comma(&mut iter) {
        return workspace_macro_helpers::compile_error_ts(
            "ts_path_fn expects comma after function name",
        )
        .into();
    }
    let body = iter.collect::<proc_macro2::TokenStream>();
    let name_ident = quote::format_ident!("{name}");
    quote::quote! {
        fn #name_ident() -> Ts2 {
            quote! {#body}
        }
    }
    .into()
}
#[proc_macro]
pub fn tp_batch(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut output = proc_macro2::TokenStream::new();
    for token in proc_macro2::TokenStream::from(input) {
        if let proc_macro2::TokenTree::Group(group) = token
            && group.delimiter() == proc_macro2::Delimiter::Parenthesis
        {
            output.extend(gen_tp(group.stream()));
        }
    }
    output.into()
}
