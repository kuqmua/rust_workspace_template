const IS_NL_PREFIX_STR_MAX_LEN: usize = 1_048_576;
#[derive(Debug, Clone, optimal_memory_layout::OptimalMemoryLayout)]
pub enum DeriveOrImpl {
    Derive,
    Impl(macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream),
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Default, newtype::FromInner)]
pub struct ProcMacro2GeneratedRustTokenStreamVec(
    Vec<macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream>,
);
impl quote::ToTokens for ProcMacro2GeneratedRustTokenStreamVec {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.0
            .iter()
            .for_each(|element| quote::ToTokens::to_tokens(element, tokens));
    }
}
impl
    FromIterator<
        macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream,
    > for ProcMacro2GeneratedRustTokenStreamVec
{
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<
            Item = macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream,
        >,
    {
        Self::from(iter.into_iter().collect::<Vec<_>>())
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    newtype::Display,
    newtype::FromInner,
)]
pub struct NonNullOrNullableStr(&'static str);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    newtype::BoundedString,
    newtype::Display,
)]
#[bounded_string(max = IS_NL_PREFIX_STR_MAX_LEN, description = "is nullable prefix string" )]
pub struct IsNullablePrefixStr(String);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    newtype::AsRefStr,
    newtype::Display,
    newtype::FromInner,
)]
pub struct ImportSnakeCaseStr(&'static str);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    newtype::Display,
    newtype::FromInner,
)]
pub struct ImportPathStr(&'static str);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    newtype::FromInner,
    newtype::GetInner,
)]
pub struct DimensionNumber(usize);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    newtype::FromInner,
    newtype::GetInner,
)]
pub struct StructElsLen(usize);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    newtype::FromInner,
    newtype::GetInner,
)]
pub struct DeLen(usize);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    newtype::FromInner,
    newtype::IntoInnerFrom,
)]
pub struct WrapIntoBraces(bool);
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, newtype::FromInner)]
pub struct ParseTokenStreamStrings(Vec<String>);
impl ParseTokenStreamStrings {
    #[must_use]
    pub fn into_generated_vec(
        self,
        uuid: ParseErrorIdRef<'_>,
    ) -> ProcMacro2GeneratedRustTokenStreamVec {
        self.0
            .into_iter()
            .map(
                |element| match element.as_str().parse::<proc_macro2::TokenStream>() {
                    Ok(parsed_token_stream) => parsed_token_stream.into(),
                    Err(error) => {
                        let message = format!("{}: {error}", uuid.as_ref());
                        quote::quote! {compile_error!(#message);}.into()
                    }
                },
            )
            .collect::<ProcMacro2GeneratedRustTokenStreamVec>()
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    newtype::AsRefInner,
    newtype::FromInner,
)]
pub struct ParseErrorIdRef<'lt>(&'lt str);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    newtype::AsRefInner,
    newtype::FromInner,
)]
pub struct PanicUuidRef<'lt>(&'lt str);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    newtype::FromInner,
    newtype::IntoInnerFrom,
)]
pub struct SynIdentifierTypeRefs<'lt>(&'lt [(&'lt syn::Ident, &'lt syn::Type)]);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    newtype::FromInner,
    newtype::IntoInnerFrom,
)]
pub struct SynFieldRefs<'lt>(&'lt [macro_helpers::domain_types::field_data::SynField]);
#[derive(Debug, Clone, Copy, optimal_memory_layout::OptimalMemoryLayout)]
pub enum IsStandardNonNull {
    False,
    True,
}
#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    strum_macros::Display,
    strum_macros::EnumIter,
    optimal_memory_layout::OptimalMemoryLayout,
)]
pub enum IsNullable {
    #[default]
    False,
    True,
}
impl IsNullable {
    #[must_use]
    pub fn maybe_optional_wrap(
        &self,
        ts: macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream,
    ) -> macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream{
        match &self {
            Self::False => ts,
            Self::True => quote::quote! {Option<#ts>}.into(),
        }
    }
    #[must_use]
    pub fn maybe_some_wrap(
        &self,
        ts: macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream,
    ) -> macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream{
        match &self {
            Self::False => ts,
            Self::True => quote::quote! {Some(#ts)}.into(),
        }
    }
    #[must_use]
    pub fn non_null_or_nullable_str(&self) -> NonNullOrNullableStr {
        match &self {
            Self::False => NonNullOrNullableStr::from(constants_str::NONNULL),
            Self::True => NonNullOrNullableStr::from(constants_str::NULLABLE),
        }
    }
    #[must_use]
    pub fn prefix_str(&self) -> IsNullablePrefixStr {
        match &self {
            Self::False => IsNullablePrefixStr::try_from(String::default())
                .unwrap_or_else(IsNullablePrefixStr::from),
            Self::True => {
                IsNullablePrefixStr::try_from(String::from(constants_str::STDOPTIONALOPTIONAL))
                    .unwrap_or_else(IsNullablePrefixStr::from)
            }
        }
    }
    #[must_use]
    pub fn rust(&self) -> &'static dyn std::fmt::Display {
        match &self {
            Self::False => &constants_str::PG_CRUD_EMPTY_SQL_SUFFIX,
            Self::True => &naming::domain_types::OptionalUpperCamelCase,
        }
    }
}
#[derive(Debug, Clone, Copy, optimal_memory_layout::OptimalMemoryLayout)]
pub enum Import {
    Crate,
    PgCrudCommon,
}
impl Import {
    pub(crate) fn all_enum_variants(
        self,
    ) -> macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream{
        macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream::from(
            match self {
                Self::Crate => quote::quote! { crate::AllEnumVariants },
                Self::PgCrudCommon => {
                    quote::quote! { pg_crud_common::domain_types::AllEnumVariants }
                }
            },
        )
    }
    pub(crate) fn all_variants_default_some_one_element(&self) -> &dyn quote::ToTokens {
        match &self {
            Self::Crate => &token_patterns::CrateAllEnumVariantsArrayDefaultSomeOneElement,
            Self::PgCrudCommon => {
                &token_patterns::PgCrudCommonAllEnumVariantsArrayDefaultSomeOneElement
            }
        }
    }
    pub(crate) fn all_variants_default_some_one_element_max_page_size(
        &self,
    ) -> &dyn quote::ToTokens {
        match &self {
            Self::Crate => {
                &token_patterns::CrateAllEnumVariantsArrayDefaultSomeOneElementMaxPageSize
            }
            Self::PgCrudCommon => {
                &token_patterns::PgCrudCommonAllEnumVariantsArrayDefaultSomeOneElementMaxPageSize
            }
        }
    }
    pub(crate) fn default_some_one_element(&self) -> &dyn quote::ToTokens {
        match &self {
            Self::Crate => &token_patterns::CrateDefaultSomeOneElement,
            Self::PgCrudCommon => &token_patterns::PgCrudCommonDefaultSomeOneElement,
        }
    }
    pub(crate) fn default_some_one_element_max_page_size(&self) -> &dyn quote::ToTokens {
        match &self {
            Self::Crate => &token_patterns::CrateDefaultSomeOneElementMaxPageSize,
            Self::PgCrudCommon => &token_patterns::PgCrudCommonDefaultSomeOneElementMaxPageSize,
        }
    }
    #[must_use]
    pub fn sc_str(&self) -> ImportSnakeCaseStr {
        match &self {
            Self::Crate => ImportSnakeCaseStr::from(constants_str::CRATE),
            Self::PgCrudCommon => {
                ImportSnakeCaseStr::from(constants_str::PG_CRUD_COMMON_DOMAIN_TYPES)
            }
        }
    }
    #[must_use]
    pub fn to_path(&self) -> ImportPathStr {
        match &self {
            Self::Crate => ImportPathStr::from(constants_str::CRATE),
            Self::PgCrudCommon => ImportPathStr::from(constants_str::PG_CRUD_COMMON_DOMAIN_TYPES),
        }
    }
}
impl quote::ToTokens for Import {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match &self {
            Self::Crate => quote::quote! { crate },
            Self::PgCrudCommon => quote::quote! { pg_crud_common::domain_types },
        }
        .to_tokens(tokens);
    }
}
pg_crud_macro_common_macros::bool_enum_to_tokens!(AddOperatorUndrscr, false => naming::domain_types::AddOperatorSnakeCase, true => quote::quote! {_});
pg_crud_macro_common_macros::bool_enum_to_tokens!(ColumnParameterUndrscr, false => naming::domain_types::ColumnSnakeCase, true => quote::quote! {_});
pg_crud_macro_common_macros::bool_enum_to_tokens!(IncrementParameterUndrscr, false => naming::domain_types::IncrementSnakeCase, true => quote::quote! {_});
pg_crud_macro_common_macros::bool_enum_to_tokens!(IsCreateQueryBindMut, false => proc_macro2::TokenStream::new(), true => naming::domain_types::MutSnakeCase);
pg_crud_macro_common_macros::bool_enum_to_tokens!(IsQueryBindMut, false => proc_macro2::TokenStream::new(), true => naming::domain_types::MutSnakeCase);
pg_crud_macro_common_macros::bool_enum_to_tokens!(IsSelectOnlyCreatedIdsQueryBindMut, false => proc_macro2::TokenStream::new(), true => naming::domain_types::MutSnakeCase);
pg_crud_macro_common_macros::bool_enum_to_tokens!(IsSelectOnlyUpdatedIdsQueryBindMut, false => proc_macro2::TokenStream::new(), true => naming::domain_types::MutSnakeCase);
pg_crud_macro_common_macros::bool_enum_to_tokens!(IsSelectQueryPartColumnFieldForErrorMessageUsed, false => quote::quote! {_}, true => naming::domain_types::ColumnFieldForErrorMessageSnakeCase);
pg_crud_macro_common_macros::bool_enum_to_tokens!(IsSelectQueryPartIsPgTypeUsed, false => quote::quote! {_}, true => quote::quote! {is_pg_type});
pg_crud_macro_common_macros::bool_enum_to_tokens!(IsSelectQueryPartSelfSelectUsed, false => quote::quote! {_}, true => naming::domain_types::VSnakeCase);
pg_crud_macro_common_macros::bool_enum_to_tokens!(IsUpdateQueryBindMut, false => proc_macro2::TokenStream::new(), true => naming::domain_types::MutSnakeCase);
pg_crud_macro_common_macros::bool_enum_to_tokens!(IsUpdateQueryPartSelfUpdateUsed, false => quote::quote! {_}, true => naming::domain_types::VSnakeCase);
pg_crud_macro_common_macros::bool_enum_to_tokens!(ShouldDSchemarsJsonSchema, false => proc_macro2::TokenStream::new(), true => quote::quote! {, schemars::JsonSchema});
pg_crud_macro_common_macros::bool_enum_to_tokens!(ShouldDeriveUtoipaToSchema, false => proc_macro2::TokenStream::new(), true => quote::quote! {, utoipa::ToSchema});
#[derive(Debug, Clone, Copy, optimal_memory_layout::OptimalMemoryLayout)]
pub enum ReadOrUpdate {
    Read,
    Update,
}
impl ReadOrUpdate {
    #[must_use]
    pub fn ucc(&self) -> &dyn naming::domain_types::DisplayPlusToTokens {
        match &self {
            Self::Read => &naming::domain_types::ReadUpperCamelCase,
            Self::Update => &naming::domain_types::UpdateUpperCamelCase,
        }
    }
}
pg_crud_macro_common_macros::bool_enum_to_tokens!(IsPrimaryKeyUndrscr, false => naming::domain_types::IsPrimaryKeySnakeCase, true => quote::quote! {_});
#[derive(Debug, Clone, Copy, optimal_memory_layout::OptimalMemoryLayout)]
pub enum DefaultSomeOneOrDefaultSomeOneWithMaxPageSize {
    DefaultSomeOne,
    DefaultSomeOneWithMaxPageSize,
}
#[derive(Debug, Clone, Copy, optimal_memory_layout::OptimalMemoryLayout)]
pub enum EqOrEqUsingFields {
    Eq,
    EqUsingFields,
}
#[derive(Debug, Clone, Copy, optimal_memory_layout::OptimalMemoryLayout)]
pub enum EqOperatorVariant {
    Eq,
    IsNull,
}
impl EqOperatorVariant {
    #[must_use]
    pub fn to_tokens_path(
        &self,
        import: &Import,
    ) -> macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream{
        let names = super::token_emission::NamesCtx::new();
        #[allow(non_snake_case)]
        let (EqOperatorUpperCamelCase,) = (&names.EqOperatorUpperCamelCase,);
        let ts = match &self {
            Self::Eq => quote::quote! {Eq},
            Self::IsNull => quote::quote! {IsNull},
        };
        quote::quote! {#import::#EqOperatorUpperCamelCase::#ts}.into()
    }
}
//todo maybe reuse with other structs
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, Clone, Copy, optimal_memory_layout::OptimalMemoryLayout)]
pub enum Dimension {
    One,
    Two,
    Three,
    Four,
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, Clone, Copy, optimal_memory_layout::OptimalMemoryLayout)]
pub enum DimensionIndexNumber {
    Zero,
    One,
    Two,
    Three,
}
impl From<&Dimension> for DimensionIndexNumber {
    fn from(v: &Dimension) -> Self {
        match &v {
            Dimension::One => Self::Zero,
            Dimension::Two => Self::One,
            Dimension::Three => Self::Two,
            Dimension::Four => Self::Three,
        }
    }
}
pg_crud_macro_common_macros::bool_enum_to_tokens!(CreateQueryBindValueUndrscr, false => naming::domain_types::VSnakeCase, true => quote::quote! {_});
pg_crud_macro_common_macros::bool_enum_to_tokens!(CreateQueryPartIncrementUndrscr, false => naming::domain_types::IncrementSnakeCase, true => quote::quote! {_});
pg_crud_macro_common_macros::bool_enum_to_tokens!(CreateQueryPartValueUndrscr, false => naming::domain_types::VSnakeCase, true => quote::quote! {_});
pg_crud_macro_common_macros::bool_enum_to_tokens!(SelectQueryPartValueUndrscr, false => naming::domain_types::VSnakeCase, true => quote::quote! {_});
pg_crud_macro_common_macros::bool_enum_to_tokens!(UpdateQueryPartAccumulatorUndrscr, false => quote::quote! {update_accumulator}, true => quote::quote! {_});
pg_crud_macro_common_macros::bool_enum_to_tokens!(UpdateQueryPartPathUndrscr, false => quote::quote! {update_path}, true => quote::quote! {_});
pg_crud_macro_common_macros::bool_enum_to_tokens!(UpdateQueryPartTargetUndrscr, false => quote::quote! {update_target}, true => quote::quote! {_});
pg_crud_macro_common_macros::bool_enum_to_tokens!(UpdateQueryPartValueUndrscr, false => naming::domain_types::VSnakeCase, true => quote::quote! {_});

#[cfg(test)]
mod tests {
    #[test]
    fn import_paths_match_their_owners() {
        assert_eq!(
            super::Import::Crate.to_path().to_string(),
            constants_str::CRATE
        );
        assert_eq!(
            super::Import::PgCrudCommon.to_path().to_string(),
            constants_str::PG_CRUD_COMMON_DOMAIN_TYPES
        );
    }
}
