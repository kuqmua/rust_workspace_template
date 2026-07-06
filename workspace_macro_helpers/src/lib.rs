use proc_macro2::{Delimiter, TokenStream as Ts2, TokenTree};
use quote::quote;
#[must_use]
pub fn compile_error_ts(msg: &str) -> Ts2 {
    let compile_msg = msg.to_owned();
    quote! {compile_error!(#compile_msg);}
}
#[must_use]
pub fn split_top_level_commas(input: Ts2) -> Vec<Ts2> {
    let mut parts = Vec::new();
    let mut current = Ts2::new();
    let mut angle_depth = 0usize;
    for token in input {
        match &token {
            TokenTree::Punct(punct) if punct.as_char() == '<' => {
                angle_depth = angle_depth.saturating_add(1);
                current.extend([token]);
            }
            TokenTree::Punct(punct) if punct.as_char() == '>' && angle_depth != 0 => {
                angle_depth = angle_depth.saturating_sub(1);
                current.extend([token]);
            }
            TokenTree::Punct(punct) if punct.as_char() == ',' && angle_depth == 0 => {
                parts.push(current);
                current = Ts2::new();
            }
            TokenTree::Group(_)
            | TokenTree::Ident(_)
            | TokenTree::Punct(_)
            | TokenTree::Literal(_) => {
                current.extend([token]);
            }
        }
    }
    parts.push(current);
    parts
}
pub fn first_ident<I>(input: &mut I) -> Option<String>
where
    I: Iterator<Item = TokenTree>,
{
    match input.next()? {
        TokenTree::Ident(ident) => Some(ident.to_string()),
        TokenTree::Group(group) if group.delimiter() == Delimiter::None => {
            first_ident(&mut group.stream().into_iter())
        }
        TokenTree::Group(_) | TokenTree::Punct(_) | TokenTree::Literal(_) => None,
    }
}
#[must_use]
pub fn strip_first_comma<I>(input: &mut I) -> bool
where
    I: Iterator<Item = TokenTree>,
{
    matches!(input.next(), Some(TokenTree::Punct(punct)) if punct.as_char() == ',')
}
#[must_use]
pub fn part_at(parts: &[Ts2], idx: usize) -> Option<Ts2> {
    parts.get(idx).cloned()
}
#[must_use]
pub fn first_ident_at(parts: &[Ts2], idx: usize) -> Option<String> {
    first_ident(&mut part_at(parts, idx)?.into_iter())
}
#[must_use]
pub fn split_fat_arrow(input: Ts2) -> Option<(Ts2, Ts2)> {
    let mut before = Ts2::new();
    let mut after = Ts2::new();
    let mut iter = input.into_iter().peekable();
    while let Some(token) = iter.next() {
        if let TokenTree::Punct(punct) = &token
            && punct.as_char() == '='
            && let Some(TokenTree::Punct(next_punct)) = iter.peek()
            && next_punct.as_char() == '>'
        {
            drop(iter.next());
            after.extend(iter);
            return Some((before, after));
        }
        before.extend([token]);
    }
    None
}
#[allow(clippy::single_call_fn)] // this keeps the closure parser isolated from proc-macro expansion bodies
#[must_use]
pub fn closure_ident_and_body(input: Ts2) -> Option<(String, Ts2)> {
    let mut iter = input.into_iter();
    let Some(TokenTree::Punct(open_pipe)) = iter.next() else {
        return None;
    };
    if open_pipe.as_char() != '|' {
        return None;
    }
    let Some(TokenTree::Ident(ident)) = iter.next() else {
        return None;
    };
    let Some(TokenTree::Punct(close_pipe)) = iter.next() else {
        return None;
    };
    if close_pipe.as_char() != '|' {
        return None;
    }
    Some((ident.to_string(), iter.collect::<Ts2>()))
}
