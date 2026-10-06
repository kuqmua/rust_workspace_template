#![allow(
    clippy::must_use_candidate,
    clippy::useless_conversion,
    reason = "shared proc-macro implementations preserve original entrypoint conversion points while returning proc_macro2 streams to one-entrypoint facade crates; every result is consumed immediately by its facade"
)]

pub(crate) mod proc_macro2_generate_token_pattern_input;
pub(crate) mod proc_macro2_generate_token_pattern_output;
#[cfg(test)]
mod test_token_pattern_boundaries;

fn generate_token_pattern(
    proc_macro2_generate_token_pattern_input: proc_macro2_generate_token_pattern_input::ProcMacro2GenerateTokenPatternInput,
) -> proc_macro2_generate_token_pattern_output::ProcMacro2GenerateTokenPatternOutput {
    let mut iter =
        proc_macro2::TokenStream::from(proc_macro2_generate_token_pattern_input).into_iter();
    let Some(name) =
        workspace_macro_helpers::parse_first_identifier::parse_first_identifier(&mut iter)
    else {
        return proc_macro2_generate_token_pattern_output::ProcMacro2GenerateTokenPatternOutput::from(
            workspace_macro_helpers::compile_error_token_stream::compile_error_token_stream(
                constants_str::COMPILE_ERROR_CE_076,
            )
            .into_inner(),
        );
    };
    if !workspace_macro_helpers::strip_first_comma::strip_first_comma(&mut iter) {
        return proc_macro2_generate_token_pattern_output::ProcMacro2GenerateTokenPatternOutput::from(
            workspace_macro_helpers::compile_error_token_stream::compile_error_token_stream(
                constants_str::COMPILE_ERROR_CE_075,
            )
            .into_inner(),
        );
    }
    let body = iter.collect::<proc_macro2::TokenStream>();
    let name_identifier = quote::format_ident!("{name}");
    proc_macro2_generate_token_pattern_output::ProcMacro2GenerateTokenPatternOutput::from(
        quote::quote! {
            #[derive(Debug, Clone, Copy, proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
            pub struct #name_identifier;
            impl quote::ToTokens for #name_identifier {
                fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
                    crate::proc_macro2_tokens_mut::ProcMacro2TokensMut::from(&mut *tokens)
                        .append(quote::quote! {#body});
                }
            }
        },
    )
}
pub fn token_pattern(token_stream: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    proc_macro2::TokenStream::from(generate_token_pattern(
        proc_macro2_generate_token_pattern_input::ProcMacro2GenerateTokenPatternInput::from(
            proc_macro2::TokenStream::from(token_stream),
        ),
    ))
    .into()
}
pub fn token_pattern_parts(token_stream: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let parts = workspace_macro_helpers::split_top_level_commas::split_top_level_commas(
        workspace_macro_helpers::proc_macro2_macro_tokens::ProcMacro2MacroTokens::from_into(
            token_stream,
        ),
    );
    if parts.len() < 2 {
        return workspace_macro_helpers::compile_error_token_stream::compile_error_token_stream(
            constants_str::COMPILE_ERROR_CE_078,
        )
        .into_inner()
        .into();
    }
    let mut part_iterator = parts.into_iter();
    let mut name_iter = part_iterator.next().into_iter().flatten();
    let Some(name) =
        workspace_macro_helpers::parse_first_identifier::parse_first_identifier(&mut name_iter)
    else {
        return workspace_macro_helpers::compile_error_token_stream::compile_error_token_stream(
            constants_str::COMPILE_ERROR_CE_077,
        )
        .into_inner()
        .into();
    };
    let name_identifier = quote::format_ident!("{name}");
    let part_streams = part_iterator.collect::<Vec<proc_macro2::TokenStream>>();
    quote::quote! {
        #[derive(Debug, Clone, Copy, proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
        pub struct #name_identifier;
        impl quote::ToTokens for #name_identifier {
            fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
                #(crate::proc_macro2_tokens_mut::ProcMacro2TokensMut::from(&mut *tokens).append(#part_streams);)*
            }
        }
    }
    .into()
}
pub fn token_stream_path_function(
    token_stream: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let mut iter = proc_macro2::TokenStream::from(token_stream).into_iter();
    let Some(name) =
        workspace_macro_helpers::parse_first_identifier::parse_first_identifier(&mut iter)
    else {
        return workspace_macro_helpers::compile_error_token_stream::compile_error_token_stream(
            constants_str::COMPILE_ERROR_CE_082,
        )
        .into_inner()
        .into();
    };
    if !workspace_macro_helpers::strip_first_comma::strip_first_comma(&mut iter) {
        return workspace_macro_helpers::compile_error_token_stream::compile_error_token_stream(
            constants_str::COMPILE_ERROR_CE_081,
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
pub fn token_pattern_batch(token_stream: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let output = proc_macro2::TokenStream::from(token_stream)
        .into_iter()
        .filter_map(|token| match token {
            proc_macro2::TokenTree::Group(group)
                if group.delimiter() == proc_macro2::Delimiter::Parenthesis =>
            {
                Some(proc_macro2::TokenStream::from(generate_token_pattern(
                    proc_macro2_generate_token_pattern_input::ProcMacro2GenerateTokenPatternInput::from(group.stream()),
                )))
            }
            proc_macro2::TokenTree::Group(_)
            | proc_macro2::TokenTree::Ident(_)
            | proc_macro2::TokenTree::Punct(_)
            | proc_macro2::TokenTree::Literal(_) => None,
        })
        .collect::<proc_macro2::TokenStream>();
    output.into()
}
