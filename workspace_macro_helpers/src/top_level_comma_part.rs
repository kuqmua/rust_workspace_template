#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
use crate::domain_types::ProcMacro2MacroTokens;

#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(super) struct TopLevelCommaPart(pub(super) ProcMacro2MacroTokens);
impl From<ProcMacro2MacroTokens> for TopLevelCommaPart {
    fn from(value: ProcMacro2MacroTokens) -> Self {
        Self(value)
    }
}
impl syn::parse::Parse for TopLevelCommaPart {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let type_fork = input.fork();
        if let Ok(parsed) = type_fork.parse::<syn::Type>()
            && (type_fork.is_empty() || type_fork.peek(syn::Token![,]))
        {
            syn::parse::discouraged::Speculative::advance_to(input, &type_fork);
            return Ok(Self::from(ProcMacro2MacroTokens::from(
                quote::ToTokens::to_token_stream(&parsed),
            )));
        }
        let expr_fork = input.fork();
        if let Ok(parsed) = expr_fork.parse::<syn::Expr>()
            && (expr_fork.is_empty() || expr_fork.peek(syn::Token![,]))
        {
            syn::parse::discouraged::Speculative::advance_to(input, &expr_fork);
            return Ok(Self::from(ProcMacro2MacroTokens::from(
                quote::ToTokens::to_token_stream(&parsed),
            )));
        }
        input.step(|cursor| {
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
            Ok((Self::from(ProcMacro2MacroTokens::from(tokens)), rest))
        })
    }
}
