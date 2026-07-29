const FIRST_IDENT_MAX_LEN: usize = 1_048_576;
#[derive(Debug, Clone, Copy)]
pub struct SynDeriveInputRef<'input_lt>(&'input_lt syn::DeriveInput);
impl<'input_lt> From<&'input_lt syn::DeriveInput> for SynDeriveInputRef<'input_lt> {
    fn from(value: &'input_lt syn::DeriveInput) -> Self {
        Self(value)
    }
}
impl<'input_lt> SynDeriveInputRef<'input_lt> {
    #[must_use]
    pub const fn get(self) -> &'input_lt syn::DeriveInput {
        self.0
    }
}
#[derive(Debug, Clone, Copy)]
pub enum SynStructShapeRef<'shape_lt> {
    Named(SynFieldsNamedRef<'shape_lt>),
    Tuple(SynFieldsUnnamedRef<'shape_lt>),
    Unit,
}
#[derive(Debug, Clone, Copy)]
pub struct SynFieldsNamedRef<'fields_lt>(&'fields_lt syn::FieldsNamed);
impl<'fields_lt> From<&'fields_lt syn::FieldsNamed> for SynFieldsNamedRef<'fields_lt> {
    fn from(value: &'fields_lt syn::FieldsNamed) -> Self {
        Self(value)
    }
}
impl<'fields_lt> SynFieldsNamedRef<'fields_lt> {
    #[must_use]
    pub const fn get(self) -> &'fields_lt syn::FieldsNamed {
        self.0
    }
}
impl std::ops::Deref for SynFieldsNamedRef<'_> {
    type Target = syn::FieldsNamed;
    fn deref(&self) -> &Self::Target {
        self.0
    }
}
#[derive(Debug, Clone, Copy)]
pub struct SynFieldsUnnamedRef<'fields_lt>(&'fields_lt syn::FieldsUnnamed);
impl<'fields_lt> From<&'fields_lt syn::FieldsUnnamed> for SynFieldsUnnamedRef<'fields_lt> {
    fn from(value: &'fields_lt syn::FieldsUnnamed) -> Self {
        Self(value)
    }
}
impl<'fields_lt> SynFieldsUnnamedRef<'fields_lt> {
    #[must_use]
    pub const fn get(self) -> &'fields_lt syn::FieldsUnnamed {
        self.0
    }
}
impl std::ops::Deref for SynFieldsUnnamedRef<'_> {
    type Target = syn::FieldsUnnamed;
    fn deref(&self) -> &Self::Target {
        self.0
    }
}
impl<'shape_lt> TryFrom<&'shape_lt syn::DeriveInput> for SynStructShapeRef<'shape_lt> {
    type Error = syn::Error;
    fn try_from(value: &'shape_lt syn::DeriveInput) -> Result<Self, Self::Error> {
        let syn::Data::Struct(data) = &value.data else {
            return Err(syn::Error::new_spanned(
                value,
                str_constants::EXPECTED_A_STRUCT,
            ));
        };
        Ok(match &data.fields {
            syn::Fields::Named(fields) => Self::Named(SynFieldsNamedRef(fields)),
            syn::Fields::Unnamed(fields) => Self::Tuple(SynFieldsUnnamedRef(fields)),
            syn::Fields::Unit => Self::Unit,
        })
    }
}
#[must_use]
#[derive(Debug, Clone)]
pub struct ProcMacro2MacroTokens(Vec<proc_macro2::TokenTree>);
impl From<proc_macro2::TokenStream> for ProcMacro2MacroTokens {
    fn from(value: proc_macro2::TokenStream) -> Self {
        Self(value.into_iter().collect())
    }
}
impl From<ProcMacro2MacroTokens> for proc_macro2::TokenStream {
    fn from(value: ProcMacro2MacroTokens) -> Self {
        value.0.into_iter().collect()
    }
}
impl ProcMacro2MacroTokens {
    pub fn from_into<T>(value: T) -> Self
    where
        T: Into<proc_macro2::TokenStream>,
    {
        Self::from(value.into())
    }
    #[must_use]
    pub fn into_inner(self) -> proc_macro2::TokenStream {
        self.0.into_iter().collect()
    }
}
impl std::ops::Deref for ProcMacro2MacroTokens {
    type Target = [proc_macro2::TokenTree];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl IntoIterator for ProcMacro2MacroTokens {
    type IntoIter = std::vec::IntoIter<proc_macro2::TokenTree>;
    type Item = proc_macro2::TokenTree;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
impl quote::ToTokens for ProcMacro2MacroTokens {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.extend(self.0.iter().cloned());
    }
}
impl std::fmt::Display for ProcMacro2MacroTokens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0
            .iter()
            .cloned()
            .collect::<proc_macro2::TokenStream>()
            .fmt(f)
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
impl From<Vec<proc_macro2::TokenStream>> for ProcMacro2TopLevelCommaParts {
    fn from(value: Vec<proc_macro2::TokenStream>) -> Self {
        Self(value)
    }
}
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
        let parts =
            syn::punctuated::Punctuated::<TopLevelCommaPart, syn::Token![,]>::parse_terminated(
                input,
            )?
            .into_iter()
            .map(|part| part.0.into_inner())
            .collect::<Vec<_>>();
        Ok(Self::from(parts))
    }
}
struct TopLevelCommaPart(ProcMacro2MacroTokens);
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
            let mut tokens = Vec::new();
            while let Some((token, next)) = rest.token_tree() {
                if matches!(&token, proc_macro2::TokenTree::Punct(punct) if punct.as_char() == ',')
                {
                    break;
                }
                tokens.push(token);
                rest = next;
            }
            Ok((
                Self::from(ProcMacro2MacroTokens::from(
                    tokens.into_iter().collect::<proc_macro2::TokenStream>(),
                )),
                rest,
            ))
        })
    }
}
#[must_use]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FirstIdentifier(String);
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FirstIdentifierifierTryFromStringError(usize);
impl From<usize> for FirstIdentifierifierTryFromStringError {
    fn from(value: usize) -> Self {
        Self(value)
    }
}
impl From<FirstIdentifierifierTryFromStringError> for FirstIdentifier {
    fn from(value: FirstIdentifierifierTryFromStringError) -> Self {
        Self(value.to_string())
    }
}
impl TryFrom<String> for FirstIdentifier {
    type Error = FirstIdentifierifierTryFromStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > FIRST_IDENT_MAX_LEN {
            return Err(FirstIdentifierifierTryFromStringError(value.len()));
        }
        Ok(Self(value))
    }
}
impl std::fmt::Display for FirstIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl std::fmt::Display for FirstIdentifierifierTryFromStringError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "first identifier length {} exceeds maximum {FIRST_IDENT_MAX_LEN}",
            self.0
        )
    }
}
#[derive(Debug, Clone)]
pub struct StdUniqueOptionSet<OptionValue>(std::collections::BTreeSet<OptionValue>);
impl<OptionValue> From<std::collections::BTreeSet<OptionValue>>
    for StdUniqueOptionSet<OptionValue>
{
    fn from(value: std::collections::BTreeSet<OptionValue>) -> Self {
        Self(value)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StdUniqueOptionSetContains(bool);
impl From<bool> for StdUniqueOptionSetContains {
    fn from(value: bool) -> Self {
        Self(value)
    }
}
impl StdUniqueOptionSetContains {
    #[must_use]
    pub const fn get(self) -> bool {
        self.0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StdUniqueOptionSetIsEmpty(bool);
impl From<bool> for StdUniqueOptionSetIsEmpty {
    fn from(value: bool) -> Self {
        Self(value)
    }
}
impl StdUniqueOptionSetIsEmpty {
    #[must_use]
    pub const fn get(self) -> bool {
        self.0
    }
}
impl<OptionValue> Default for StdUniqueOptionSet<OptionValue> {
    fn default() -> Self {
        Self::from(std::collections::BTreeSet::new())
    }
}
impl<OptionValue> StdUniqueOptionSet<OptionValue>
where
    OptionValue: Copy + Ord,
{
    #[must_use]
    pub fn contains(&self, value: OptionValue) -> StdUniqueOptionSetContains {
        StdUniqueOptionSetContains::from(self.0.contains(&value))
    }
    #[must_use]
    pub fn is_empty(&self) -> StdUniqueOptionSetIsEmpty {
        StdUniqueOptionSetIsEmpty::from(self.0.is_empty())
    }
    pub fn try_insert_with<DuplicateError>(
        &mut self,
        value: OptionValue,
        duplicate_error: DuplicateError,
    ) -> syn::Result<()>
    where
        DuplicateError: FnOnce() -> syn::Error,
    {
        if !self.0.insert(value) {
            return Err(duplicate_error());
        }
        Ok(())
    }
}
#[must_use]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FirstCommaStripped(bool);
impl From<bool> for FirstCommaStripped {
    fn from(value: bool) -> Self {
        Self(value)
    }
}
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
pub fn compile_error_token_stream<S>(message: S) -> ProcMacro2MacroTokens
where
    S: AsRef<str>,
{
    let compile_message = message.as_ref().to_owned();
    quote::quote! {compile_error!(#compile_message);}.into()
}
pub fn split_top_level_commas<T>(input: T) -> ProcMacro2TopLevelCommaParts
where
    T: Into<ProcMacro2MacroTokens>,
{
    syn::parse2::<ProcMacro2TopLevelCommaParts>(input.into().into_inner())
        .unwrap_or_else(|_| ProcMacro2TopLevelCommaParts::from(Vec::new()))
}
pub fn first_identifier<I>(input: &mut I) -> Option<FirstIdentifier>
where
    I: Iterator<Item = proc_macro2::TokenTree>,
{
    match input.next()? {
        proc_macro2::TokenTree::Ident(identifier) => Some(
            FirstIdentifier::try_from(identifier.to_string()).unwrap_or_else(FirstIdentifier::from),
        ),
        proc_macro2::TokenTree::Group(group)
            if group.delimiter() == proc_macro2::Delimiter::None =>
        {
            first_identifier(&mut group.stream().into_iter())
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
    FirstCommaStripped::from(
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
pub fn first_identifier_at<I>(
    parts: &ProcMacro2TopLevelCommaParts,
    idx: I,
) -> Option<FirstIdentifier>
where
    I: Into<PartIndex>,
{
    first_identifier(&mut part_at(parts, idx)?.into_iter())
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
#[must_use]
pub fn closure_identifier_and_body<T>(input: T) -> Option<(FirstIdentifier, ProcMacro2MacroTokens)>
where
    T: Into<ProcMacro2MacroTokens>,
{
    struct ClosureIdentifierAndBody {
        body: ProcMacro2MacroTokens,
        identifier: syn::Ident,
    }
    impl syn::parse::Parse for ClosureIdentifierAndBody {
        fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
            let _: syn::Token![|] = input.parse()?;
            let identifier = input.parse::<syn::Ident>()?;
            let _: syn::Token![|] = input.parse()?;
            let body = input.parse::<ProcMacro2MacroTokens>()?;
            Ok(Self { body, identifier })
        }
    }
    let parsed = syn::parse2::<ClosureIdentifierAndBody>(input.into().into_inner()).ok()?;
    Some((
        FirstIdentifier::try_from(parsed.identifier.to_string())
            .unwrap_or_else(FirstIdentifier::from),
        parsed.body,
    ))
}
#[cfg(test)]
mod tests {
    #[test]
    fn struct_shape_preserves_named_tuple_and_unit_forms() {
        let named = syn::parse_quote!(
            struct Named {
                value: u8,
            }
        );
        let tuple = syn::parse_quote!(
            struct Tuple(u8);
        );
        let unit = syn::parse_quote!(
            struct Unit;
        );
        assert!(matches!(
            super::SynStructShapeRef::try_from(&named),
            Ok(super::SynStructShapeRef::Named(_))
        ));
        assert!(matches!(
            super::SynStructShapeRef::try_from(&tuple),
            Ok(super::SynStructShapeRef::Tuple(_))
        ));
        assert!(matches!(
            super::SynStructShapeRef::try_from(&unit),
            Ok(super::SynStructShapeRef::Unit)
        ));
    }
    #[test]
    fn split_top_level_commas_keeps_generic_type_commas_inside_part() {
        let parts = super::split_top_level_commas(quote::quote! {
            Vec<Result<A, B>>,
            Option<C>
        });
        assert_eq!(parts.len(), 2);
        assert_eq!(
            parts.first().map(ToString::to_string),
            Some("Vec < Result < A , B > >".to_owned())
        );
        assert_eq!(
            parts.get(1).map(ToString::to_string),
            Some("Option < C >".to_owned())
        );
    }
    #[test]
    fn split_top_level_commas_keeps_fat_arrow_pair_as_single_part() {
        let parts = super::split_top_level_commas(quote::quote! {
            SomeType => "message",
            OtherType => format!("{}" , value)
        });
        assert_eq!(parts.len(), 2);
        assert_eq!(
            parts.first().map(ToString::to_string),
            Some("SomeType => \"message\"".to_owned())
        );
        assert_eq!(
            parts.get(1).map(ToString::to_string),
            Some("OtherType => format ! (\"{}\" , value)".to_owned())
        );
    }
    #[test]
    fn proc_macro2_macro_tokens_to_tokens_preserves_stream() {
        let tokens = super::ProcMacro2MacroTokens::from(quote::quote! {
            Result<Vec<A>, B>
        });
        assert_eq!(
            quote::quote! {#tokens}.to_string(),
            "Result < Vec < A > , B >"
        );
    }
    #[test]
    fn unique_option_set_preserves_first_span_aware_error() {
        let mut values = super::StdUniqueOptionSet::default();
        values
            .try_insert_with(1u8, || {
                syn::Error::new(proc_macro2::Span::call_site(), str_constants::FIRST_ALT)
            })
            .expect("12817d29");
        let error = values
            .try_insert_with(1u8, || {
                syn::Error::new(proc_macro2::Span::call_site(), str_constants::DUPLICATE)
            })
            .expect_err(str_constants::CE4826F4);
        assert_eq!(error.to_string(), "duplicate");
        assert!(values.contains(1u8).get());
    }
}
