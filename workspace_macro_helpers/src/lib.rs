#[must_use]
pub fn compile_error_ts(msg: &str) -> proc_macro2::TokenStream {
    let compile_msg = msg.to_owned();
    quote::quote! {compile_error!(#compile_msg);}
}
#[must_use]
pub fn split_top_level_commas(input: proc_macro2::TokenStream) -> Vec<proc_macro2::TokenStream> {
    let mut parts = Vec::new();
    let mut current = proc_macro2::TokenStream::new();
    let mut angle_depth = 0usize;
    for token in input {
        match &token {
            proc_macro2::TokenTree::Punct(punct) if punct.as_char() == '<' => {
                angle_depth = angle_depth.saturating_add(1);
                current.extend([token]);
            }
            proc_macro2::TokenTree::Punct(punct) if punct.as_char() == '>' && angle_depth != 0 => {
                angle_depth = angle_depth.saturating_sub(1);
                current.extend([token]);
            }
            proc_macro2::TokenTree::Punct(punct) if punct.as_char() == ',' && angle_depth == 0 => {
                parts.push(current);
                current = proc_macro2::TokenStream::new();
            }
            proc_macro2::TokenTree::Group(_)
            | proc_macro2::TokenTree::Ident(_)
            | proc_macro2::TokenTree::Punct(_)
            | proc_macro2::TokenTree::Literal(_) => {
                current.extend([token]);
            }
        }
    }
    parts.push(current);
    parts
}
pub fn first_ident<I>(input: &mut I) -> Option<String>
where
    I: Iterator<Item = proc_macro2::TokenTree>,
{
    match input.next()? {
        proc_macro2::TokenTree::Ident(ident) => Some(ident.to_string()),
        proc_macro2::TokenTree::Group(group)
            if group.delimiter() == proc_macro2::Delimiter::None =>
        {
            first_ident(&mut group.stream().into_iter())
        }
        proc_macro2::TokenTree::Group(_)
        | proc_macro2::TokenTree::Punct(_)
        | proc_macro2::TokenTree::Literal(_) => None,
    }
}
#[must_use]
pub fn strip_first_comma<I>(input: &mut I) -> bool
where
    I: Iterator<Item = proc_macro2::TokenTree>,
{
    matches!(input.next(), Some(proc_macro2::TokenTree::Punct(punct)) if punct.as_char() == ',')
}
#[must_use]
pub fn part_at(parts: &[proc_macro2::TokenStream], idx: usize) -> Option<proc_macro2::TokenStream> {
    parts.get(idx).cloned()
}
#[must_use]
pub fn first_ident_at(parts: &[proc_macro2::TokenStream], idx: usize) -> Option<String> {
    first_ident(&mut part_at(parts, idx)?.into_iter())
}
#[must_use]
pub fn split_fat_arrow(
    input: proc_macro2::TokenStream,
) -> Option<(proc_macro2::TokenStream, proc_macro2::TokenStream)> {
    let mut before = proc_macro2::TokenStream::new();
    let mut after = proc_macro2::TokenStream::new();
    let mut iter = input.into_iter().peekable();
    while let Some(token) = iter.next() {
        if let proc_macro2::TokenTree::Punct(punct) = &token
            && punct.as_char() == '='
            && let Some(proc_macro2::TokenTree::Punct(next_punct)) = iter.peek()
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
pub fn closure_ident_and_body(
    input: proc_macro2::TokenStream,
) -> Option<(String, proc_macro2::TokenStream)> {
    let mut iter = input.into_iter();
    let Some(proc_macro2::TokenTree::Punct(open_pipe)) = iter.next() else {
        return None;
    };
    if open_pipe.as_char() != '|' {
        return None;
    }
    let Some(proc_macro2::TokenTree::Ident(ident)) = iter.next() else {
        return None;
    };
    let Some(proc_macro2::TokenTree::Punct(close_pipe)) = iter.next() else {
        return None;
    };
    if close_pipe.as_char() != '|' {
        return None;
    }
    Some((
        ident.to_string(),
        iter.collect::<proc_macro2::TokenStream>(),
    ))
}
