#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype_foundation_foundation_from_inner::FromInner,
)]
pub(super) struct TopLevelCommaPart(crate::proc_macro2_macro_tokens::ProcMacro2MacroTokens);
impl TopLevelCommaPart {
    pub(super) fn into_inner(self) -> crate::proc_macro2_macro_tokens::ProcMacro2MacroTokens {
        self.0
    }
}
impl syn::parse::Parse for TopLevelCommaPart {
    fn parse(parse_stream: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let type_fork = parse_stream.fork();
        if let Ok(parsed) = type_fork.parse::<syn::Type>()
            && (type_fork.is_empty() || type_fork.peek(syn::Token![,]))
        {
            syn::parse::discouraged::Speculative::advance_to(parse_stream, &type_fork);
            return Ok(Self::from(
                crate::proc_macro2_macro_tokens::ProcMacro2MacroTokens::from(
                    quote::ToTokens::to_token_stream(&parsed),
                ),
            ));
        }
        let expr_fork = parse_stream.fork();
        if let Ok(parsed) = expr_fork.parse::<syn::Expr>()
            && (expr_fork.is_empty() || expr_fork.peek(syn::Token![,]))
        {
            syn::parse::discouraged::Speculative::advance_to(parse_stream, &expr_fork);
            return Ok(Self::from(
                crate::proc_macro2_macro_tokens::ProcMacro2MacroTokens::from(
                    quote::ToTokens::to_token_stream(&parsed),
                ),
            ));
        }
        parse_stream.step(|cursor| {
            let mut rest = *cursor;
            let mut tokens = proc_macro2::TokenStream::new();
            while let Some((token, next)) = rest.token_tree() {
                if matches!(&token, proc_macro2::TokenTree::Punct(punct) if punct.as_char() == ',')
                {
                    break;
                }
                tokens.extend(std::iter::once(token));
                rest = next;
            }
            Ok((
                Self::from(crate::proc_macro2_macro_tokens::ProcMacro2MacroTokens::from(tokens)),
                rest,
            ))
        })
    }
}
