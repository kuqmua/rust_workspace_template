#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddOprtrUndrscr {
    False,
    True,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColPrmUndrscr {
    False,
    True,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DefaultSomeOneOrDefaultSomeOneWithMaxPageSize {
    DefaultSomeOne,
    DefaultSomeOneWithMaxPageSize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeriveOrImpl {
    Derive,
    Impl,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dim {
    Four,
    One,
    Three,
    Two,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EqOprtrH {
    Eq,
    IsNull,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EqOrEqUsingFields {
    Eq,
    EqUsingFields,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Import {
    Crate,
    PgCrud,
    PgCrudCmn,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IncrPrmUndrscr {
    False,
    True,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IsCrQbMut {
    False,
    True,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IsNl {
    False,
    True,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IsPkUndrscr {
    False,
    True,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IsQbMut {
    False,
    True,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IsSelOnlyCrdIdsQbMut {
    False,
    True,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IsSelOnlyUpddIdsQbMut {
    False,
    True,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IsSelQpColFieldForErMsgUsed {
    False,
    True,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IsSelQpIsPgTypeUsed {
    False,
    True,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IsSelQpSelfSelUsed {
    False,
    True,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IsStdrtNn {
    False,
    True,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IsUpdQbMut {
    False,
    True,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IsUpdQpJsonbSetTargetUsed {
    False,
    True,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IsUpdQpSelfUpdUsed {
    False,
    True,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PgTypeOrPgJson {
    PgJson,
    PgType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RdOrUpd {
    Rd,
    Upd,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShouldDeriveUtoipaToSchema {
    False,
    True,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShouldDSchemarsJsonSchema {
    False,
    True,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct JsonbNullLiteral;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParsedTokenStreams;

impl AsRef<str> for JsonbNullLiteral {
    fn as_ref(&self) -> &str {
        naming_constants::POSTGRES_JSONB_NULL_LITERAL
    }
}

impl Import {
    #[must_use]
    pub const fn module_scope(self) -> PostgresModuleScope {
        match self {
            Self::Crate => PostgresModuleScope::Crate,
            Self::PgCrud => PostgresModuleScope::PgCrud,
            Self::PgCrudCmn => PostgresModuleScope::PgCrudCmn,
        }
    }
}

impl IsNl {
    #[must_use]
    pub fn maybe_option_wrap<TokenStreamValue>(
        self,
        token_stream: &TokenStreamValue,
    ) -> proc_macro2::TokenStream
    where
        TokenStreamValue: quote::ToTokens + ?Sized,
    {
        match self {
            Self::False => quote::quote! { #token_stream },
            Self::True => quote::quote! { Option<#token_stream> },
        }
    }

    #[must_use]
    pub fn maybe_some_wrap<TokenStreamValue>(
        self,
        token_stream: &TokenStreamValue,
    ) -> proc_macro2::TokenStream
    where
        TokenStreamValue: quote::ToTokens + ?Sized,
    {
        match self {
            Self::False => quote::quote! { #token_stream },
            Self::True => quote::quote! { Some(#token_stream) },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PostgresModuleScope {
    Crate,
    PgCrud,
    PgCrudCmn,
}

impl AsRef<str> for PostgresModuleScope {
    fn as_ref(&self) -> &str {
        match *self {
            Self::Crate => naming_constants::POSTGRES_MODULE_SCOPE_CRATE,
            Self::PgCrud => naming_constants::POSTGRES_MODULE_SCOPE_PG_CRUD,
            Self::PgCrudCmn => naming_constants::POSTGRES_MODULE_SCOPE_PG_CRUD_CMN,
        }
    }
}

#[must_use]
pub const fn cmn_d_ts_builder() -> macros_helpers::DTsBuilder {
    macros_helpers::DTsBuilder::new()
}

#[must_use]
pub fn gen_jsonb_build_obj<FieldsTokenStream>(
    fields: &FieldsTokenStream,
) -> proc_macro2::TokenStream
where
    FieldsTokenStream: quote::ToTokens + ?Sized,
{
    quote::quote! { sqlx::types::JsonValue::Object(#fields) }
}

#[must_use]
pub fn gen_jsonb_build_obj_v<ValueTokenStream>(value: &ValueTokenStream) -> proc_macro2::TokenStream
where
    ValueTokenStream: quote::ToTokens + ?Sized,
{
    quote::quote! { sqlx::types::JsonValue::from(#value) }
}

#[must_use]
pub fn gen_case_jsonb_typeof_null<ValueTokenStream>(
    value: &ValueTokenStream,
) -> proc_macro2::TokenStream
where
    ValueTokenStream: quote::ToTokens + ?Sized,
{
    quote::quote! {
        if #value == #value {
            #value
        }
    }
}

#[must_use]
pub fn gen_mod_with_pub_use_ts<ModuleIdentifierTokenStream, ContentTokenStream>(
    module_identifier: &ModuleIdentifierTokenStream,
    content: &ContentTokenStream,
) -> proc_macro2::TokenStream
where
    ModuleIdentifierTokenStream: quote::ToTokens + ?Sized,
    ContentTokenStream: quote::ToTokens + ?Sized,
{
    quote::quote! {
        pub mod #module_identifier {
            #content
        }
    }
}

#[must_use]
pub const fn parse_strs_to_ts2_vec<Value>(_value: &Value) -> ParsedTokenStreams
where
    Value: ?Sized,
{
    ParsedTokenStreams
}
