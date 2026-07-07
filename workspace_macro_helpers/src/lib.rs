#[must_use]
#[derive(Debug, Clone)]
pub struct MacroTokens(pub proc_macro2::TokenStream);
impl From<proc_macro2::TokenStream> for MacroTokens {
    fn from(value: proc_macro2::TokenStream) -> Self {
        Self(value)
    }
}
impl From<MacroTokens> for proc_macro2::TokenStream {
    fn from(value: MacroTokens) -> Self {
        value.0
    }
}
impl IntoIterator for MacroTokens {
    type IntoIter = proc_macro2::token_stream::IntoIter;
    type Item = proc_macro2::TokenTree;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
impl quote::ToTokens for MacroTokens {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.extend(self.0.clone());
    }
}
impl std::fmt::Display for MacroTokens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl syn::parse::Parse for MacroTokens {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        input.step(|cursor| {
            let mut rest = *cursor;
            let mut tokens = proc_macro2::TokenStream::new();
            while let Some((token, next)) = rest.token_tree() {
                tokens.extend([token]);
                rest = next;
            }
            Ok((Self(tokens), rest))
        })
    }
}
#[must_use]
#[derive(Debug, Clone)]
pub struct TopLevelCommaParts(Vec<proc_macro2::TokenStream>);
impl std::ops::Deref for TopLevelCommaParts {
    type Target = Vec<proc_macro2::TokenStream>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::ops::DerefMut for TopLevelCommaParts {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl IntoIterator for TopLevelCommaParts {
    type IntoIter = std::vec::IntoIter<proc_macro2::TokenStream>;
    type Item = proc_macro2::TokenStream;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
impl syn::parse::Parse for TopLevelCommaParts {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let mut parts = Vec::new();
        let mut current = proc_macro2::TokenStream::new();
        let mut angle_depth = 0usize;
        while !input.is_empty() {
            if input.peek(syn::Token![,]) && angle_depth == 0 {
                let _: syn::Token![,] = input.parse()?;
                parts.push(current);
                current = proc_macro2::TokenStream::new();
                continue;
            }
            let token = input.parse::<proc_macro2::TokenTree>()?;
            match &token {
                proc_macro2::TokenTree::Punct(punct) if punct.as_char() == '<' => {
                    angle_depth = angle_depth.saturating_add(1);
                }
                proc_macro2::TokenTree::Punct(punct)
                    if punct.as_char() == '>' && angle_depth != 0 =>
                {
                    angle_depth = angle_depth.saturating_sub(1);
                }
                proc_macro2::TokenTree::Group(_)
                | proc_macro2::TokenTree::Ident(_)
                | proc_macro2::TokenTree::Punct(_)
                | proc_macro2::TokenTree::Literal(_) => {}
            }
            current.extend([token]);
        }
        parts.push(current);
        Ok(Self(parts))
    }
}
#[must_use]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FirstIdent(pub String);
impl std::fmt::Display for FirstIdent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
#[must_use]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FirstCommaStripped(pub bool);
impl std::ops::Not for FirstCommaStripped {
    type Output = bool;
    fn not(self) -> Self::Output {
        !self.0
    }
}
#[must_use]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PartIndex(pub usize);
impl From<usize> for PartIndex {
    fn from(value: usize) -> Self {
        Self(value)
    }
}
pub fn compile_error_ts<S>(msg: S) -> MacroTokens
where
    S: AsRef<str>,
{
    let compile_msg = msg.as_ref().to_owned();
    quote::quote! {compile_error!(#compile_msg);}.into()
}
pub fn split_top_level_commas<T>(input: T) -> TopLevelCommaParts
where
    T: Into<MacroTokens>,
{
    syn::parse2::<TopLevelCommaParts>(input.into().0)
        .unwrap_or_else(|_| TopLevelCommaParts(Vec::new()))
}
pub fn first_ident<I>(input: &mut I) -> Option<FirstIdent>
where
    I: Iterator<Item = proc_macro2::TokenTree>,
{
    match input.next()? {
        proc_macro2::TokenTree::Ident(ident) => Some(FirstIdent(ident.to_string())),
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
pub fn strip_first_comma<I>(input: &mut I) -> FirstCommaStripped
where
    I: Iterator<Item = proc_macro2::TokenTree>,
{
    FirstCommaStripped(
        matches!(input.next(), Some(proc_macro2::TokenTree::Punct(punct)) if punct.as_char() == ','),
    )
}
#[must_use]
pub fn part_at<I>(parts: &TopLevelCommaParts, idx: I) -> Option<MacroTokens>
where
    I: Into<PartIndex>,
{
    parts.get(idx.into().0).cloned().map(MacroTokens)
}
#[must_use]
pub fn first_ident_at<I>(parts: &TopLevelCommaParts, idx: I) -> Option<FirstIdent>
where
    I: Into<PartIndex>,
{
    first_ident(&mut part_at(parts, idx)?.into_iter())
}
#[must_use]
pub fn split_fat_arrow<T>(input: T) -> Option<(MacroTokens, MacroTokens)>
where
    T: Into<MacroTokens>,
{
    let mut before = proc_macro2::TokenStream::new();
    let mut after = proc_macro2::TokenStream::new();
    let mut iter = input.into().0.into_iter().peekable();
    while let Some(token) = iter.next() {
        if let proc_macro2::TokenTree::Punct(punct) = &token
            && punct.as_char() == '='
            && let Some(proc_macro2::TokenTree::Punct(next_punct)) = iter.peek()
            && next_punct.as_char() == '>'
        {
            let arrow = {
                let mut ts = proc_macro2::TokenStream::new();
                ts.extend([token]);
                ts.extend(iter.next());
                ts
            };
            let _: syn::Token![=>] = syn::parse2(arrow).ok()?;
            after.extend(iter);
            return Some((MacroTokens(before), MacroTokens(after)));
        }
        before.extend([token]);
    }
    None
}
#[allow(clippy::single_call_fn)] // this keeps the closure parser isolated from proc-macro expansion bodies
#[must_use]
pub fn closure_ident_and_body<T>(input: T) -> Option<(FirstIdent, MacroTokens)>
where
    T: Into<MacroTokens>,
{
    struct ClosureIdentAndBody {
        body: MacroTokens,
        ident: syn::Ident,
    }
    impl syn::parse::Parse for ClosureIdentAndBody {
        fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
            let _: syn::Token![|] = input.parse()?;
            let ident = input.parse::<syn::Ident>()?;
            let _: syn::Token![|] = input.parse()?;
            let body = input.parse::<MacroTokens>()?;
            Ok(Self { body, ident })
        }
    }
    let parsed = syn::parse2::<ClosureIdentAndBody>(input.into().0).ok()?;
    Some((FirstIdent(parsed.ident.to_string()), parsed.body))
}
