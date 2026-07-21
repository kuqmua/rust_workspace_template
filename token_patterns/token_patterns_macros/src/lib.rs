struct ProcMacro2GenerateTpInput(proc_macro2::TokenStream);
impl From<proc_macro2::TokenStream> for ProcMacro2GenerateTpInput {
    fn from(value: proc_macro2::TokenStream) -> Self {
        Self(value)
    }
}
struct ProcMacro2GenerateTpOutput(proc_macro2::TokenStream);
impl From<proc_macro2::TokenStream> for ProcMacro2GenerateTpOutput {
    fn from(value: proc_macro2::TokenStream) -> Self {
        Self(value)
    }
}
fn generate_tp(input: ProcMacro2GenerateTpInput) -> ProcMacro2GenerateTpOutput {
    let mut iter = input.0.into_iter();
    let Some(name) = workspace_macro_helpers::first_identifier(&mut iter) else {
        return ProcMacro2GenerateTpOutput::from(
            workspace_macro_helpers::compile_error_token_stream(
                str_constants::COMPILE_ERROR_CE_076,
            )
            .into_inner(),
        );
    };
    if !workspace_macro_helpers::strip_first_comma(&mut iter) {
        return ProcMacro2GenerateTpOutput::from(
            workspace_macro_helpers::compile_error_token_stream(
                str_constants::COMPILE_ERROR_CE_075,
            )
            .into_inner(),
        );
    }
    let body = iter.collect::<proc_macro2::TokenStream>();
    let name_identifier = quote::format_ident!("{name}");
    ProcMacro2GenerateTpOutput::from(quote::quote! {
        #[derive(Debug, Clone, Copy, optml::Optml)]
        pub struct #name_identifier;
        impl quote::ToTokens for #name_identifier {
            fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
                append_tokens(&mut ProcMacro2TokensMut(tokens), quote::quote! {#body});
            }
        }
    })
}
#[proc_macro]
pub fn tp(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    generate_tp(ProcMacro2GenerateTpInput::from(
        proc_macro2::TokenStream::from(input),
    ))
    .0
    .into()
}
#[proc_macro]
pub fn tp_parts(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut parts = workspace_macro_helpers::split_top_level_commas(
        workspace_macro_helpers::ProcMacro2MacroTokens::from_into(input),
    );
    if parts.len() < 2 {
        return workspace_macro_helpers::compile_error_token_stream(
            str_constants::COMPILE_ERROR_CE_078,
        )
        .into_inner()
        .into();
    }
    let mut name_iter = parts.remove(0).into_iter();
    let Some(name) = workspace_macro_helpers::first_identifier(&mut name_iter) else {
        return workspace_macro_helpers::compile_error_token_stream(
            str_constants::COMPILE_ERROR_CE_077,
        )
        .into_inner()
        .into();
    };
    let name_identifier = quote::format_ident!("{name}");
    let part_streams = parts.into_iter().collect::<Vec<proc_macro2::TokenStream>>();
    quote::quote! {
        #[derive(Debug, Clone, Copy, optml::Optml)]
        pub struct #name_identifier;
        impl quote::ToTokens for #name_identifier {
            fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
                #(append_tokens(&mut ProcMacro2TokensMut(tokens), #part_streams);)*
            }
        }
    }
    .into()
}
#[proc_macro]
pub fn ts_path_fn(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut iter = proc_macro2::TokenStream::from(input).into_iter();
    let Some(name) = workspace_macro_helpers::first_identifier(&mut iter) else {
        return workspace_macro_helpers::compile_error_token_stream(
            str_constants::COMPILE_ERROR_CE_082,
        )
        .into_inner()
        .into();
    };
    if !workspace_macro_helpers::strip_first_comma(&mut iter) {
        return workspace_macro_helpers::compile_error_token_stream(
            str_constants::COMPILE_ERROR_CE_081,
        )
        .into_inner()
        .into();
    }
    let body = iter.collect::<proc_macro2::TokenStream>();
    let name_identifier = quote::format_ident!("{name}");
    quote::quote! {
        fn #name_identifier() -> proc_macro2::TokenStream {
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
                Some(generate_tp(ProcMacro2GenerateTpInput::from(group.stream())).0)
            }
            proc_macro2::TokenTree::Group(_)
            | proc_macro2::TokenTree::Ident(_)
            | proc_macro2::TokenTree::Punct(_)
            | proc_macro2::TokenTree::Literal(_) => None,
        })
        .collect::<proc_macro2::TokenStream>();
    output.into()
}
