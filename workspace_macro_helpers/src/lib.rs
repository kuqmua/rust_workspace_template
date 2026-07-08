const FIRST_IDENT_MAX_LEN: usize = 1_048_576;
#[must_use]
#[derive(Debug, Clone)]
pub struct ProcMacro2MacroTokens(proc_macro2::TokenStream);
impl From<proc_macro2::TokenStream> for ProcMacro2MacroTokens {
    fn from(value: proc_macro2::TokenStream) -> Self {
        Self(value)
    }
}
impl From<ProcMacro2MacroTokens> for proc_macro2::TokenStream {
    fn from(value: ProcMacro2MacroTokens) -> Self {
        value.0
    }
}
impl ProcMacro2MacroTokens {
    pub fn from_into<T>(value: T) -> Self
    where
        T: Into<proc_macro2::TokenStream>,
    {
        Self(value.into())
    }
    #[must_use]
    pub fn into_inner(self) -> proc_macro2::TokenStream {
        self.0
    }
}
impl IntoIterator for ProcMacro2MacroTokens {
    type IntoIter = proc_macro2::token_stream::IntoIter;
    type Item = proc_macro2::TokenTree;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
impl quote::ToTokens for ProcMacro2MacroTokens {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.extend(self.0.clone());
    }
}
impl std::fmt::Display for ProcMacro2MacroTokens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl syn::parse::Parse for ProcMacro2MacroTokens {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        input.step(|cursor| {
            let mut rest = *cursor;
            let mut tokens = proc_macro2::TokenStream::new();
            while let Some((token, next)) = rest.token_tree() {
                tokens.extend([token]);
                rest = next;
            }
            Ok((Self::from(tokens), rest))
        })
    }
}
#[must_use]
#[derive(Debug, Clone)]
pub struct ProcMacro2TopLevelCommaParts(Vec<proc_macro2::TokenStream>);
impl std::ops::Deref for ProcMacro2TopLevelCommaParts {
    type Target = Vec<proc_macro2::TokenStream>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::ops::DerefMut for ProcMacro2TopLevelCommaParts {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl IntoIterator for ProcMacro2TopLevelCommaParts {
    type IntoIter = std::vec::IntoIter<proc_macro2::TokenStream>;
    type Item = proc_macro2::TokenStream;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
impl syn::parse::Parse for ProcMacro2TopLevelCommaParts {
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
pub struct FirstIdent(String);
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FirstIdentTryFromStringEr {
    TooLong { len: usize, max: usize },
}
impl std::fmt::Display for FirstIdentTryFromStringEr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TooLong { len, max } => {
                write!(f, "first ident length {len} exceeds maximum {max}")
            }
        }
    }
}
impl From<FirstIdentTryFromStringEr> for FirstIdent {
    fn from(value: FirstIdentTryFromStringEr) -> Self {
        Self(value.to_string())
    }
}
impl TryFrom<String> for FirstIdent {
    type Error = FirstIdentTryFromStringEr;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > FIRST_IDENT_MAX_LEN {
            return Err(Self::Error::TooLong {
                len: value.len(),
                max: FIRST_IDENT_MAX_LEN,
            });
        }
        Ok(Self(value))
    }
}
impl std::fmt::Display for FirstIdent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
#[must_use]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FirstCommaStripped(bool);
impl std::ops::Not for FirstCommaStripped {
    type Output = bool;
    fn not(self) -> Self::Output {
        !self.0
    }
}
#[must_use]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PartIndex(usize);
impl From<usize> for PartIndex {
    fn from(value: usize) -> Self {
        Self(value)
    }
}
pub fn compile_error_ts<S>(msg: S) -> ProcMacro2MacroTokens
where
    S: AsRef<str>,
{
    let compile_msg = msg.as_ref().to_owned();
    quote::quote! {compile_error!(#compile_msg);}.into()
}
pub fn split_top_level_commas<T>(input: T) -> ProcMacro2TopLevelCommaParts
where
    T: Into<ProcMacro2MacroTokens>,
{
    syn::parse2::<ProcMacro2TopLevelCommaParts>(input.into().0)
        .unwrap_or_else(|_| ProcMacro2TopLevelCommaParts(Vec::new()))
}
pub fn first_ident<I>(input: &mut I) -> Option<FirstIdent>
where
    I: Iterator<Item = proc_macro2::TokenTree>,
{
    match input.next()? {
        proc_macro2::TokenTree::Ident(ident) => {
            Some(FirstIdent::try_from(ident.to_string()).unwrap_or_else(FirstIdent::from))
        }
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
pub fn part_at<I>(parts: &ProcMacro2TopLevelCommaParts, idx: I) -> Option<ProcMacro2MacroTokens>
where
    I: Into<PartIndex>,
{
    parts
        .get(idx.into().0)
        .cloned()
        .map(ProcMacro2MacroTokens::from)
}
#[must_use]
pub fn first_ident_at<I>(parts: &ProcMacro2TopLevelCommaParts, idx: I) -> Option<FirstIdent>
where
    I: Into<PartIndex>,
{
    first_ident(&mut part_at(parts, idx)?.into_iter())
}
#[must_use]
pub fn split_fat_arrow<T>(input: T) -> Option<(ProcMacro2MacroTokens, ProcMacro2MacroTokens)>
where
    T: Into<ProcMacro2MacroTokens>,
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
            return Some((
                ProcMacro2MacroTokens::from(before),
                ProcMacro2MacroTokens::from(after),
            ));
        }
        before.extend([token]);
    }
    None
}
#[allow(clippy::single_call_fn)] // this keeps the closure parser isolated from proc-macro expansion bodies
#[must_use]
pub fn closure_ident_and_body<T>(input: T) -> Option<(FirstIdent, ProcMacro2MacroTokens)>
where
    T: Into<ProcMacro2MacroTokens>,
{
    struct ClosureIdentAndBody {
        body: ProcMacro2MacroTokens,
        ident: syn::Ident,
    }
    impl syn::parse::Parse for ClosureIdentAndBody {
        fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
            let _: syn::Token![|] = input.parse()?;
            let ident = input.parse::<syn::Ident>()?;
            let _: syn::Token![|] = input.parse()?;
            let body = input.parse::<ProcMacro2MacroTokens>()?;
            Ok(Self { body, ident })
        }
    }
    let parsed = syn::parse2::<ClosureIdentAndBody>(input.into().0).ok()?;
    Some((
        FirstIdent::try_from(parsed.ident.to_string()).unwrap_or_else(FirstIdent::from),
        parsed.body,
    ))
}
