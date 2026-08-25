pub(crate) const SNAKE_IDENT_MAX_LEN: usize = 1_048_576;
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Default)]
#[allow(clippy::arbitrary_source_item_ordering)] // alignment order required by optimal_memory_layout takes precedence over alphabetical field order
#[allow(clippy::field_scoped_visibility_modifiers)] // the proc-macro entry module parses and consumes this domain model
pub(crate) struct NewtypeAttrs {
    pub(crate) options: workspace_macro_helpers::domain_types::UniqueOptionBTreeSet<NewtypeOption>,
    pub(crate) try_from: Option<NewtypeTryFromAttrs>,
    pub(crate) to_err_string_mode: Option<ToErrStringMode>,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug)]
#[allow(clippy::field_scoped_visibility_modifiers)] // the proc-macro entry module consumes this parsed domain model
pub(crate) struct NewtypeTryFromAttrs {
    pub(crate) error: Option<SynType>,
    pub(crate) validator: SynExpr,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
#[allow(clippy::field_scoped_visibility_modifiers)] // the proc-macro entry module incrementally builds this parsed domain model
pub(crate) struct BoundedStringAttrs {
    pub(crate) description: Option<SynExpr>,
    pub(crate) max: Option<SynExpr>,
    pub(crate) min: Option<SynExpr>,
    pub(crate) options:
        workspace_macro_helpers::domain_types::UniqueOptionBTreeSet<BoundedStringOption>,
    pub(crate) validator: Option<SynExpr>,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
#[allow(clippy::field_scoped_visibility_modifiers)] // the proc-macro entry module consumes this parsed domain model
pub(crate) struct WireEnumAttrs {
    pub(crate) error_message: SynExpr,
    pub(crate) ref_type: SynIdentifier,
}
impl syn::parse::Parse for WireEnumAttrs {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let mut error_message = None;
        let mut ref_type = None;
        while !input.is_empty() {
            let name = input.parse::<syn::Ident>()?;
            let _equals = input.parse::<syn::Token![=]>()?;
            if name == constants_str::WIRE_ENUM_REF_TYPE {
                ref_type = Some(SynIdentifier::from(input.parse::<syn::Ident>()?));
            } else if name == constants_str::WIRE_ENUM_ERROR_MESSAGE {
                error_message = Some(SynExpr::from(input.parse::<syn::Expr>()?));
            } else {
                return Err(syn::Error::new_spanned(
                    name,
                    constants_str::WIRE_ENUM_REQUIRES_ATTRIBUTE,
                ));
            }
            if !input.is_empty() {
                let _comma = input.parse::<syn::Token![,]>()?;
            }
        }
        Ok(Self {
            error_message: error_message
                .ok_or_else(|| input.error(constants_str::WIRE_ENUM_REQUIRES_ATTRIBUTE))?,
            ref_type: ref_type
                .ok_or_else(|| input.error(constants_str::WIRE_ENUM_REQUIRES_ATTRIBUTE))?,
        })
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord,
)]
pub(crate) enum BoundedStringOption {
    Chars,
    NulFree,
    Serde,
    Trim,
    Utoipa,
    WriteOnly,
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord,
)]
pub(crate) enum NewtypeOption {
    Accessor,
    AsMut,
    AsRef,
    AsRefInner,
    AsRefOwned,
    AsRefStr,
    AsRefTarget,
    AsSlice,
    BorrowInner,
    BorrowOwned,
    BorrowPath,
    BorrowStr,
    CloneInner,
    DebugRedacted,
    DebugTransparent,
    DefaultInner,
    DerefInner,
    DerefMutInner,
    DerefMutTarget,
    DerefTarget,
    Display,
    From,
    GetInner,
    IntoInner,
    IntoInnerFrom,
    IntoIterator,
    IntoVec,
    NotInner,
    PartialEqInner,
    Secret,
    ToTokens,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ToErrStringMode {
    AsRefStr,
    Debug,
    Display,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(crate) struct ProcMacro2GeneratedTokenStream(proc_macro2::TokenStream);
impl From<proc_macro2::TokenStream> for ProcMacro2GeneratedTokenStream {
    fn from(value: proc_macro2::TokenStream) -> Self {
        Self(value)
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(crate) struct ProcMacroInputTokenStream(proc_macro::TokenStream);
impl From<proc_macro::TokenStream> for ProcMacroInputTokenStream {
    fn from(value: proc_macro::TokenStream) -> Self {
        Self(value)
    }
}
impl ProcMacroInputTokenStream {
    pub(crate) fn into_inner(self) -> proc_macro::TokenStream {
        self.0
    }
}
impl From<ProcMacro2GeneratedTokenStream> for proc_macro2::TokenStream {
    fn from(value: ProcMacro2GeneratedTokenStream) -> Self {
        value.0
    }
}
impl From<ProcMacro2GeneratedTokenStream> for proc_macro::TokenStream {
    fn from(value: ProcMacro2GeneratedTokenStream) -> Self {
        proc_macro2::TokenStream::from(value).into()
    }
}
impl quote::ToTokens for ProcMacro2GeneratedTokenStream {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.0.to_tokens(tokens);
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(crate) struct NewtypeBool(bool);
impl From<bool> for NewtypeBool {
    fn from(value: bool) -> Self {
        Self(value)
    }
}
impl NewtypeBool {
    pub(crate) const fn get(&self) -> bool {
        self.0
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(crate) struct SnakeIdentifier(String);
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug)]
pub(crate) struct SnakeIdentifierifierLen(usize);
impl From<usize> for SnakeIdentifierifierLen {
    fn from(value: usize) -> Self {
        Self(value)
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug)]
pub(crate) struct SnakeIdentifierifierTryFromStringError(SnakeIdentifierifierLen);
impl From<SnakeIdentifierifierLen> for SnakeIdentifierifierTryFromStringError {
    fn from(value: SnakeIdentifierifierLen) -> Self {
        Self(value)
    }
}
impl std::fmt::Display for SnakeIdentifierifierTryFromStringError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "snake identifier length {} exceeds maximum {SNAKE_IDENT_MAX_LEN}",
            self.0.0
        )
    }
}
impl TryFrom<String> for SnakeIdentifier {
    type Error = SnakeIdentifierifierTryFromStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > SNAKE_IDENT_MAX_LEN {
            return Err(SnakeIdentifierifierTryFromStringError(
                SnakeIdentifierifierLen::from(value.len()),
            ));
        }
        Ok(Self(value))
    }
}
impl AsRef<str> for SnakeIdentifier {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
impl std::fmt::Display for SnakeIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl quote::ToTokens for SnakeIdentifier {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.0.to_tokens(tokens);
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy)]
pub(crate) struct SynDeriveInputRef<'syn_lt>(&'syn_lt syn::DeriveInput);
impl<'syn_lt> From<&'syn_lt syn::DeriveInput> for SynDeriveInputRef<'syn_lt> {
    fn from(value: &'syn_lt syn::DeriveInput) -> Self {
        Self(value)
    }
}
impl AsRef<syn::DeriveInput> for SynDeriveInputRef<'_> {
    fn as_ref(&self) -> &syn::DeriveInput {
        self.0
    }
}
impl<'syn_lt> SynDeriveInputRef<'syn_lt> {
    pub(crate) const fn get(self) -> &'syn_lt syn::DeriveInput {
        self.0
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy)]
pub(crate) struct SynIdentifierRef<'syn_lt>(&'syn_lt syn::Ident);
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(crate) struct SynIdentifier(syn::Ident);
impl From<syn::Ident> for SynIdentifier {
    fn from(value: syn::Ident) -> Self {
        Self(value)
    }
}
impl SynIdentifier {
    pub(crate) fn into_inner(self) -> syn::Ident {
        self.0
    }
}
impl<'syn_lt> From<&'syn_lt syn::Ident> for SynIdentifierRef<'syn_lt> {
    fn from(value: &'syn_lt syn::Ident) -> Self {
        Self(value)
    }
}
impl AsRef<syn::Ident> for SynIdentifierRef<'_> {
    fn as_ref(&self) -> &syn::Ident {
        self.0
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy)]
pub(crate) struct SynTypeRef<'syn_lt>(&'syn_lt syn::Type);
impl<'syn_lt> From<&'syn_lt syn::Type> for SynTypeRef<'syn_lt> {
    fn from(value: &'syn_lt syn::Type) -> Self {
        Self(value)
    }
}
impl AsRef<syn::Type> for SynTypeRef<'_> {
    fn as_ref(&self) -> &syn::Type {
        self.0
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug)]
pub(crate) struct SynType(syn::Type);
impl From<syn::Type> for SynType {
    fn from(value: syn::Type) -> Self {
        Self(value)
    }
}
impl AsRef<syn::Type> for SynType {
    fn as_ref(&self) -> &syn::Type {
        &self.0
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug)]
pub(crate) struct SynExpr(syn::Expr);
impl From<syn::Expr> for SynExpr {
    fn from(value: syn::Expr) -> Self {
        Self(value)
    }
}
impl SynExpr {
    pub(crate) fn into_inner(self) -> syn::Expr {
        self.0
    }
}
impl AsRef<syn::Expr> for SynExpr {
    fn as_ref(&self) -> &syn::Expr {
        &self.0
    }
}
impl quote::ToTokens for SynExpr {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.0.to_tokens(tokens);
    }
}
impl NewtypeAttrs {
    pub(crate) fn contains(&self, option: NewtypeOption) -> NewtypeBool {
        NewtypeBool::from(self.options.contains(option).get())
    }
}
