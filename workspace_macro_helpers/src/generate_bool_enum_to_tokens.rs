pub fn generate_bool_enum_to_tokens(
    proc_macro2_macro_tokens: crate::proc_macro2_macro_tokens::ProcMacro2MacroTokens,
) -> crate::proc_macro2_macro_tokens::ProcMacro2MacroTokens {
    #[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
    struct BoolEnumTokenBranches {
        false_expression: crate::proc_macro2_macro_tokens::ProcMacro2MacroTokens,
        name_identifier: crate::proc_macro2_macro_tokens::ProcMacro2MacroTokens,
        true_expression: crate::proc_macro2_macro_tokens::ProcMacro2MacroTokens,
    }
    impl syn::parse::Parse for BoolEnumTokenBranches {
        fn parse(parse_stream: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
            let name_identifier = parse_stream.parse::<syn::Ident>()?;
            let _: syn::Token![,] = parse_stream.parse()?;
            let false_keyword = parse_stream.parse::<syn::LitBool>()?;
            if false_keyword.value {
                return Err(syn::Error::new_spanned(
                    false_keyword,
                    constants_str::COMPILE_ERROR_CE_046,
                ));
            }
            let _: syn::Token![=>] = parse_stream.parse()?;
            let false_expression = parse_stream.parse::<syn::Expr>()?;
            let _: syn::Token![,] = parse_stream.parse()?;
            let true_keyword = parse_stream.parse::<syn::LitBool>()?;
            if !true_keyword.value {
                return Err(syn::Error::new_spanned(
                    true_keyword,
                    constants_str::COMPILE_ERROR_CE_047,
                ));
            }
            let _: syn::Token![=>] = parse_stream.parse()?;
            let true_expression = parse_stream.parse::<syn::Expr>()?;
            Ok(Self {
                false_expression: quote::quote! { #false_expression }.into(),
                name_identifier: quote::quote! { #name_identifier }.into(),
                true_expression: quote::quote! { #true_expression }.into(),
            })
        }
    }
    let parsed = match syn::parse2::<BoolEnumTokenBranches>(proc_macro2_macro_tokens.into_inner()) {
        Ok(value) => value,
        Err(error) => return error.into_compile_error().into(),
    };
    let false_expression = &parsed.false_expression;
    let name_identifier = &parsed.name_identifier;
    let true_expression = &parsed.true_expression;
    quote::quote! {
        #[derive(Debug, Clone, Copy, proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
        pub enum #name_identifier {
            False,
            True,
        }
        impl quote::ToTokens for #name_identifier {
            fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
                match &self {
                    Self::False => (#false_expression).to_tokens(tokens),
                    Self::True => (#true_expression).to_tokens(tokens),
                }
            }
        }
    }
    .into()
}
