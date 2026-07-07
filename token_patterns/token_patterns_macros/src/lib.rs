struct GenTpInput(proc_macro2::TokenStream);
struct GenTpOutput(proc_macro2::TokenStream);
fn gen_tp(input: GenTpInput) -> GenTpOutput {
    let mut iter = input.0.into_iter();
    let Some(name) = workspace_macro_helpers::first_ident(&mut iter) else {
        return GenTpOutput(workspace_macro_helpers::compile_error_ts("tp expects type name").0);
    };
    if !workspace_macro_helpers::strip_first_comma(&mut iter) {
        return GenTpOutput(
            workspace_macro_helpers::compile_error_ts("tp expects comma after type name").0,
        );
    }
    let body = iter.collect::<proc_macro2::TokenStream>();
    let name_ident = quote::format_ident!("{name}");
    GenTpOutput(quote::quote! {
        #[derive(Debug, Clone, Copy, optml::Optml)]
        pub struct #name_ident;
        impl quote::ToTokens for #name_ident {
            fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
                append_tokens(&mut TokensMut(tokens), quote::quote! {#body});
            }
        }
    })
}
#[proc_macro]
pub fn tp(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    gen_tp(GenTpInput(input.into())).0.into()
}
#[proc_macro]
pub fn tp_parts(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut parts = workspace_macro_helpers::split_top_level_commas(
        workspace_macro_helpers::MacroTokens(input.into()),
    );
    if parts.len() < 2 {
        return workspace_macro_helpers::compile_error_ts(
            "tp_parts expects type name and at least one part",
        )
        .0
        .into();
    }
    let mut name_iter = parts.remove(0).into_iter();
    let Some(name) = workspace_macro_helpers::first_ident(&mut name_iter) else {
        return workspace_macro_helpers::compile_error_ts("tp_parts expects type name")
            .0
            .into();
    };
    let name_ident = quote::format_ident!("{name}");
    let part_streams = parts.into_iter().collect::<Vec<proc_macro2::TokenStream>>();
    quote::quote! {
        #[derive(Debug, Clone, Copy, optml::Optml)]
        pub struct #name_ident;
        impl quote::ToTokens for #name_ident {
            fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
                #(append_tokens(&mut TokensMut(tokens), #part_streams);)*
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
            .0
            .into();
    };
    if !workspace_macro_helpers::strip_first_comma(&mut iter) {
        return workspace_macro_helpers::compile_error_ts(
            "ts_path_fn expects comma after function name",
        )
        .0
        .into();
    }
    let body = iter.collect::<proc_macro2::TokenStream>();
    let name_ident = quote::format_ident!("{name}");
    quote::quote! {
        fn #name_ident() -> proc_macro2::TokenStream {
            quote::quote! {#body}
        }
    }
    .into()
}
#[proc_macro]
pub fn tp_batch(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let output = proc_macro2::TokenStream::from(input)
        .into_iter()
        .filter_map(|token| match token {
            proc_macro2::TokenTree::Group(group)
                if group.delimiter() == proc_macro2::Delimiter::Parenthesis =>
            {
                Some(gen_tp(GenTpInput(group.stream())).0)
            }
            proc_macro2::TokenTree::Group(_)
            | proc_macro2::TokenTree::Ident(_)
            | proc_macro2::TokenTree::Punct(_)
            | proc_macro2::TokenTree::Literal(_) => None,
        })
        .collect::<proc_macro2::TokenStream>();
    output.into()
}
