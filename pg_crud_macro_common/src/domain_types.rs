pub mod filters;
pub mod pg_type_test_cases;
pub mod token_stream_helpers;
const IS_NL_PREFIX_STR_MAX_LEN: usize = 1_048_576;
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
#[allow(dead_code, non_snake_case)]
struct NamesCtx {
    AddOperatorSnakeCase: naming::domain_types::AddOperatorSnakeCase,
    AllVariantsDefaultSomeOneElementMaxPageSizeSnakeCase:
        naming::domain_types::AllVariantsDefaultSomeOneElementMaxPageSizeSnakeCase,
    AllVariantsDefaultSomeOneElementSnakeCase:
        naming::domain_types::AllVariantsDefaultSomeOneElementSnakeCase,
    AllowClippyArbitrarySrcItemOrdering: token_patterns::AllowClippyArbitrarySrcItemOrdering,
    ColumnSnakeCase: naming::domain_types::ColumnSnakeCase,
    CreateQueryBindSnakeCase: naming::domain_types::CreateQueryBindSnakeCase,
    CreateQueryPartSnakeCase: naming::domain_types::CreateQueryPartSnakeCase,
    CreateSnakeCase: naming::domain_types::CreateSnakeCase,
    CreateTableColumnQueryPartSnakeCase: naming::domain_types::CreateTableColumnQueryPartSnakeCase,
    CreateUpperCamelCase: naming::domain_types::CreateUpperCamelCase,
    DefaultSomeOneElementMaxPageSizeSnakeCase:
        naming::domain_types::DefaultSomeOneElementMaxPageSizeSnakeCase,
    DefaultSomeOneElementSnakeCase: naming::domain_types::DefaultSomeOneElementSnakeCase,
    EqOperatorUpperCamelCase: naming::domain_types::EqOperatorUpperCamelCase,
    ErrorSnakeCase: naming::domain_types::ErrorSnakeCase,
    IncrementSnakeCase: naming::domain_types::IncrementSnakeCase,
    NormalizeSnakeCase: naming::domain_types::NormalizeSnakeCase,
    OptionalUpdateSnakeCase: naming::domain_types::OptionalUpdateSnakeCase,
    OptionalVecCreateSnakeCase: naming::domain_types::OptionalVecCreateSnakeCase,
    PgCrudCommonDefaultSomeOneElementCall: token_patterns::PgCrudCommonDefaultSomeOneElementCall,
    PgTypeEqOperatorUpperCamelCase: naming::domain_types::PgTypeEqOperatorUpperCamelCase,
    PgTypeNotPrimaryKeyUpperCamelCase: naming::domain_types::PgTypeNotPrimaryKeyUpperCamelCase,
    PgTypeOptionalVecWhereGreaterThanTestSnakeCase:
        naming::domain_types::PgTypeOptionalVecWhereGreaterThanTestSnakeCase,
    PgTypeTestCasesUpperCamelCase: naming::domain_types::PgTypeTestCasesUpperCamelCase,
    PgTypeUpperCamelCase: naming::domain_types::PgTypeUpperCamelCase,
    PgTypeWhereFilterUpperCamelCase: naming::domain_types::PgTypeWhereFilterUpperCamelCase,
    PreviousReadAndOptionalUpdateIntoReadSnakeCase:
        naming::domain_types::PreviousReadAndOptionalUpdateIntoReadSnakeCase,
    QueryBindSnakeCase: naming::domain_types::QueryBindSnakeCase,
    QueryPartErrorUpperCamelCase: naming::domain_types::QueryPartErrorUpperCamelCase,
    QueryPartSnakeCase: naming::domain_types::QueryPartSnakeCase,
    QuerySnakeCase: naming::domain_types::QuerySnakeCase,
    ReadIdsAndCreateIntoOptionalVReadSnakeCase:
        naming::domain_types::ReadIdsAndCreateIntoOptionalVReadSnakeCase,
    ReadIdsAndCreateIntoOptionalVecWhereEqToFieldSnakeCase:
        naming::domain_types::ReadIdsAndCreateIntoOptionalVecWhereEqToFieldSnakeCase,
    ReadIdsAndCreateIntoReadSnakeCase: naming::domain_types::ReadIdsAndCreateIntoReadSnakeCase,
    ReadIdsAndCreateIntoTableTypeSnakeCase:
        naming::domain_types::ReadIdsAndCreateIntoTableTypeSnakeCase,
    ReadIdsAndCreateIntoVecWhereEqUsingFieldsSnakeCase:
        naming::domain_types::ReadIdsAndCreateIntoVecWhereEqUsingFieldsSnakeCase,
    ReadIdsAndCreateIntoWhereEqSnakeCase:
        naming::domain_types::ReadIdsAndCreateIntoWhereEqSnakeCase,
    ReadIdsAndTableTypeIntoPgTypeOptionalWhereGreaterThanSnakeCase:
        naming::domain_types::ReadIdsAndTableTypeIntoPgTypeOptionalWhereGreaterThanSnakeCase,
    ReadIdsSnakeCase: naming::domain_types::ReadIdsSnakeCase,
    ReadIdsTo2DimensionsVecReadInnerSnakeCase:
        naming::domain_types::ReadIdsTo2DimensionsVecReadInnerSnakeCase,
    ReadIdsToOptionalVReadDefaultSomeOneElementSnakeCase:
        naming::domain_types::ReadIdsToOptionalVReadDefaultSomeOneElementSnakeCase,
    ReadIdsUpperCamelCase: naming::domain_types::ReadIdsUpperCamelCase,
    ReadInnerIntoReadWithNewOrTryNewUnwrapedSnakeCase:
        naming::domain_types::ReadInnerIntoReadWithNewOrTryNewUnwrapedSnakeCase,
    ReadInnerIntoUpdateWithNewOrTryNewUnwrapedSnakeCase:
        naming::domain_types::ReadInnerIntoUpdateWithNewOrTryNewUnwrapedSnakeCase,
    ReadInnerUpperCamelCase: naming::domain_types::ReadInnerUpperCamelCase,
    ReadSnakeCase: naming::domain_types::ReadSnakeCase,
    ReadUpperCamelCase: naming::domain_types::ReadUpperCamelCase,
    SelectOnlyIdsQueryPartSnakeCase: naming::domain_types::SelectOnlyIdsQueryPartSnakeCase,
    SelectOnlyUpdatedIdsQueryBindSnakeCase:
        naming::domain_types::SelectOnlyUpdatedIdsQueryBindSnakeCase,
    SelectOnlyUpdatedIdsQueryPartSnakeCase:
        naming::domain_types::SelectOnlyUpdatedIdsQueryPartSnakeCase,
    SelectQueryPartSnakeCase: naming::domain_types::SelectQueryPartSnakeCase,
    SelectUpperCamelCase: naming::domain_types::SelectUpperCamelCase,
    SelfUpperCamelCase: naming::domain_types::SelfUpperCamelCase,
    TableTypeSnakeCase: naming::domain_types::TableTypeSnakeCase,
    TableTypeUpperCamelCase: naming::domain_types::TableTypeUpperCamelCase,
    UpdateForQueryUpperCamelCase: naming::domain_types::UpdateForQueryUpperCamelCase,
    UpdateQueryBindSnakeCase: naming::domain_types::UpdateQueryBindSnakeCase,
    UpdateQueryPartSnakeCase: naming::domain_types::UpdateQueryPartSnakeCase,
    UpdateToReadIdsSnakeCase: naming::domain_types::UpdateToReadIdsSnakeCase,
    UpdateUpperCamelCase: naming::domain_types::UpdateUpperCamelCase,
    VSnakeCase: naming::domain_types::VSnakeCase,
    VUpperCamelCase: naming::domain_types::VUpperCamelCase,
    ValueSnakeCase: naming::domain_types::ValueSnakeCase,
    WhereUpperCamelCase: naming::domain_types::WhereUpperCamelCase,
}
impl NamesCtx {
    const fn new() -> Self {
        Self {
            AddOperatorSnakeCase: naming::domain_types::AddOperatorSnakeCase,
            AllVariantsDefaultSomeOneElementMaxPageSizeSnakeCase:
                naming::domain_types::AllVariantsDefaultSomeOneElementMaxPageSizeSnakeCase,
            AllVariantsDefaultSomeOneElementSnakeCase:
                naming::domain_types::AllVariantsDefaultSomeOneElementSnakeCase,
            AllowClippyArbitrarySrcItemOrdering:
                token_patterns::AllowClippyArbitrarySrcItemOrdering,
            ColumnSnakeCase: naming::domain_types::ColumnSnakeCase,
            CreateQueryBindSnakeCase: naming::domain_types::CreateQueryBindSnakeCase,
            CreateQueryPartSnakeCase: naming::domain_types::CreateQueryPartSnakeCase,
            CreateSnakeCase: naming::domain_types::CreateSnakeCase,
            CreateTableColumnQueryPartSnakeCase:
                naming::domain_types::CreateTableColumnQueryPartSnakeCase,
            CreateUpperCamelCase: naming::domain_types::CreateUpperCamelCase,
            DefaultSomeOneElementMaxPageSizeSnakeCase:
                naming::domain_types::DefaultSomeOneElementMaxPageSizeSnakeCase,
            DefaultSomeOneElementSnakeCase: naming::domain_types::DefaultSomeOneElementSnakeCase,
            EqOperatorUpperCamelCase: naming::domain_types::EqOperatorUpperCamelCase,
            ErrorSnakeCase: naming::domain_types::ErrorSnakeCase,
            IncrementSnakeCase: naming::domain_types::IncrementSnakeCase,
            NormalizeSnakeCase: naming::domain_types::NormalizeSnakeCase,
            OptionalUpdateSnakeCase: naming::domain_types::OptionalUpdateSnakeCase,
            OptionalVecCreateSnakeCase: naming::domain_types::OptionalVecCreateSnakeCase,
            PgCrudCommonDefaultSomeOneElementCall:
                token_patterns::PgCrudCommonDefaultSomeOneElementCall,
            PgTypeEqOperatorUpperCamelCase: naming::domain_types::PgTypeEqOperatorUpperCamelCase,
            PgTypeNotPrimaryKeyUpperCamelCase:
                naming::domain_types::PgTypeNotPrimaryKeyUpperCamelCase,
            PgTypeOptionalVecWhereGreaterThanTestSnakeCase:
                naming::domain_types::PgTypeOptionalVecWhereGreaterThanTestSnakeCase,
            PgTypeTestCasesUpperCamelCase: naming::domain_types::PgTypeTestCasesUpperCamelCase,
            PgTypeUpperCamelCase: naming::domain_types::PgTypeUpperCamelCase,
            PgTypeWhereFilterUpperCamelCase: naming::domain_types::PgTypeWhereFilterUpperCamelCase,
            PreviousReadAndOptionalUpdateIntoReadSnakeCase:
                naming::domain_types::PreviousReadAndOptionalUpdateIntoReadSnakeCase,
            QueryBindSnakeCase: naming::domain_types::QueryBindSnakeCase,
            QueryPartErrorUpperCamelCase: naming::domain_types::QueryPartErrorUpperCamelCase,
            QueryPartSnakeCase: naming::domain_types::QueryPartSnakeCase,
            QuerySnakeCase: naming::domain_types::QuerySnakeCase,
            ReadIdsAndCreateIntoOptionalVReadSnakeCase:
                naming::domain_types::ReadIdsAndCreateIntoOptionalVReadSnakeCase,
            ReadIdsAndCreateIntoOptionalVecWhereEqToFieldSnakeCase:
                naming::domain_types::ReadIdsAndCreateIntoOptionalVecWhereEqToFieldSnakeCase,
            ReadIdsAndCreateIntoReadSnakeCase:
                naming::domain_types::ReadIdsAndCreateIntoReadSnakeCase,
            ReadIdsAndCreateIntoTableTypeSnakeCase:
                naming::domain_types::ReadIdsAndCreateIntoTableTypeSnakeCase,
            ReadIdsAndCreateIntoVecWhereEqUsingFieldsSnakeCase:
                naming::domain_types::ReadIdsAndCreateIntoVecWhereEqUsingFieldsSnakeCase,
            ReadIdsAndCreateIntoWhereEqSnakeCase:
                naming::domain_types::ReadIdsAndCreateIntoWhereEqSnakeCase,
            ReadIdsAndTableTypeIntoPgTypeOptionalWhereGreaterThanSnakeCase:
                naming::domain_types::ReadIdsAndTableTypeIntoPgTypeOptionalWhereGreaterThanSnakeCase,
            ReadIdsSnakeCase: naming::domain_types::ReadIdsSnakeCase,
            ReadIdsTo2DimensionsVecReadInnerSnakeCase:
                naming::domain_types::ReadIdsTo2DimensionsVecReadInnerSnakeCase,
            ReadIdsToOptionalVReadDefaultSomeOneElementSnakeCase:
                naming::domain_types::ReadIdsToOptionalVReadDefaultSomeOneElementSnakeCase,
            ReadIdsUpperCamelCase: naming::domain_types::ReadIdsUpperCamelCase,
            ReadInnerIntoReadWithNewOrTryNewUnwrapedSnakeCase:
                naming::domain_types::ReadInnerIntoReadWithNewOrTryNewUnwrapedSnakeCase,
            ReadInnerIntoUpdateWithNewOrTryNewUnwrapedSnakeCase:
                naming::domain_types::ReadInnerIntoUpdateWithNewOrTryNewUnwrapedSnakeCase,
            ReadInnerUpperCamelCase: naming::domain_types::ReadInnerUpperCamelCase,
            ReadSnakeCase: naming::domain_types::ReadSnakeCase,
            ReadUpperCamelCase: naming::domain_types::ReadUpperCamelCase,
            SelectOnlyIdsQueryPartSnakeCase: naming::domain_types::SelectOnlyIdsQueryPartSnakeCase,
            SelectOnlyUpdatedIdsQueryBindSnakeCase:
                naming::domain_types::SelectOnlyUpdatedIdsQueryBindSnakeCase,
            SelectOnlyUpdatedIdsQueryPartSnakeCase:
                naming::domain_types::SelectOnlyUpdatedIdsQueryPartSnakeCase,
            SelectQueryPartSnakeCase: naming::domain_types::SelectQueryPartSnakeCase,
            SelectUpperCamelCase: naming::domain_types::SelectUpperCamelCase,
            SelfUpperCamelCase: naming::domain_types::SelfUpperCamelCase,
            TableTypeSnakeCase: naming::domain_types::TableTypeSnakeCase,
            TableTypeUpperCamelCase: naming::domain_types::TableTypeUpperCamelCase,
            UpdateForQueryUpperCamelCase: naming::domain_types::UpdateForQueryUpperCamelCase,
            UpdateQueryBindSnakeCase: naming::domain_types::UpdateQueryBindSnakeCase,
            UpdateQueryPartSnakeCase: naming::domain_types::UpdateQueryPartSnakeCase,
            UpdateToReadIdsSnakeCase: naming::domain_types::UpdateToReadIdsSnakeCase,
            UpdateUpperCamelCase: naming::domain_types::UpdateUpperCamelCase,
            VSnakeCase: naming::domain_types::VSnakeCase,
            VUpperCamelCase: naming::domain_types::VUpperCamelCase,
            ValueSnakeCase: naming::domain_types::ValueSnakeCase,
            WhereUpperCamelCase: naming::domain_types::WhereUpperCamelCase,
        }
    }
}
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
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, newtype::FromInner)]
pub struct SynIdentifierTypeRefs<'lt>(&'lt [(&'lt syn::Ident, &'lt syn::Type)]);
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, newtype::FromInner)]
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
    fn all_enum_variants(
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
    fn all_variants_default_some_one_element(&self) -> &dyn quote::ToTokens {
        match &self {
            Self::Crate => &token_patterns::CrateAllEnumVariantsArrayDefaultSomeOneElement,
            Self::PgCrudCommon => {
                &token_patterns::PgCrudCommonAllEnumVariantsArrayDefaultSomeOneElement
            }
        }
    }
    fn all_variants_default_some_one_element_max_page_size(&self) -> &dyn quote::ToTokens {
        match &self {
            Self::Crate => {
                &token_patterns::CrateAllEnumVariantsArrayDefaultSomeOneElementMaxPageSize
            }
            Self::PgCrudCommon => {
                &token_patterns::PgCrudCommonAllEnumVariantsArrayDefaultSomeOneElementMaxPageSize
            }
        }
    }
    fn default_some_one_element(&self) -> &dyn quote::ToTokens {
        match &self {
            Self::Crate => &token_patterns::CrateDefaultSomeOneElement,
            Self::PgCrudCommon => &token_patterns::PgCrudCommonDefaultSomeOneElement,
        }
    }
    fn default_some_one_element_max_page_size(&self) -> &dyn quote::ToTokens {
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
        let names = NamesCtx::new();
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
pub fn generate_pg_type_where_token_stream<T>(
    attrs_token_stream: &dyn quote::ToTokens,
    variants: &[T],
    prefix: &dyn quote::ToTokens,
    should_derive_utoipa_to_schema: &ShouldDeriveUtoipaToSchema,
    should_derive_schemars_json_schema: &ShouldDSchemarsJsonSchema,
    is_query_bind_mut: &IsQueryBindMut,
) -> macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream
where
    T: filters::PgFilter,
{
    let names = NamesCtx::new();
    #[allow(non_snake_case)]
    let (
        AddOperatorSnakeCase,
        ColumnSnakeCase,
        IncrementSnakeCase,
        PgCrudCommonDefaultSomeOneElementCall,
        QuerySnakeCase,
        VSnakeCase,
    ) = (
        &names.AddOperatorSnakeCase,
        &names.ColumnSnakeCase,
        &names.IncrementSnakeCase,
        &names.PgCrudCommonDefaultSomeOneElementCall,
        &names.QuerySnakeCase,
        &names.VSnakeCase,
    );
    let identifier = naming::domain_types::parameter::SelfWhereUpperCamelCase::from_tokens(&prefix);
    let pg_type_tokens_where_token_stream = {
        let vrts_token_stream = variants.iter().map(|element| {
            let element_upper_camel_case = element.ucc();
            let prefix_where_self_upper_camel_case = element.prefix_where_self_upper_camel_case();
            let optional_type_token_stream: Option<macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream> =
                element.maybe_generic();
            let type_token_stream =
                optional_type_token_stream.map_or_else(proc_macro2::TokenStream::new, |v| quote::quote! {<#v>});
            quote::quote! {#element_upper_camel_case(where_filters::domain_types::#prefix_where_self_upper_camel_case #type_token_stream)}
        });
        let utoipa_schema_token_stream = match should_derive_utoipa_to_schema {
            ShouldDeriveUtoipaToSchema::False => proc_macro2::TokenStream::new(),
            ShouldDeriveUtoipaToSchema::True => {
                let schema_items_token_stream = variants.iter().map(|element| {
                    let element_upper_camel_case = element.ucc();
                    let prefix_where_self_upper_camel_case =
                        element.prefix_where_self_upper_camel_case();
                    let optional_type_token_stream = element.maybe_generic();
                    let type_token_stream = optional_type_token_stream.map_or_else(
                        proc_macro2::TokenStream::new,
                        |value| quote::quote! {<#value>},
                    );
                    quote::quote! {
                        .item(
                            utoipa::openapi::ObjectBuilder::new()
                                .property(
                                    stringify!(#element_upper_camel_case),
                                    <where_filters::domain_types::#prefix_where_self_upper_camel_case #type_token_stream as utoipa::PartialSchema>::schema(),
                                )
                                .required(stringify!(#element_upper_camel_case)),
                        )
                    }
                });
                let schema_name = identifier.to_string();
                quote::quote! {
                    impl utoipa::PartialSchema for #identifier {
                        fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
                            utoipa::openapi::schema::Schema::from(
                                utoipa::openapi::OneOfBuilder::new()
                                    #(#schema_items_token_stream)*
                                    .build(),
                            )
                            .into()
                        }
                    }
                    impl utoipa::ToSchema for #identifier {
                        fn name() -> std::borrow::Cow<'static, str> {
                            std::borrow::Cow::Borrowed(#schema_name)
                        }
                    }
                }
            }
        };
        quote::quote! {
            #attrs_token_stream
            #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize #should_derive_schemars_json_schema, optimal_memory_layout::OptimalMemoryLayout)]
            pub enum #identifier {
                #(#vrts_token_stream),*
            }
            #utoipa_schema_token_stream
        }
    };
    let impl_pg_type_pg_type_where_filter_for_pg_type_tokens_where_token_stream =
        impl_pg_type_where_filter_for_identifier_token_stream(
            &quote::quote! {<'lt>},
            &identifier,
            &proc_macro2::TokenStream::new(),
            &IncrementParameterUndrscr::False,
            &ColumnParameterUndrscr::False,
            &AddOperatorUndrscr::False,
            &{
                let vrts_token_stream = variants.iter().map(|element| {
                let element_upper_camel_case = element.ucc();
                quote::quote! {
                    Self::#element_upper_camel_case(#VSnakeCase) => pg_crud_common::domain_types::PgTypeWhereFilter::query_part(
                        #VSnakeCase,
                        #IncrementSnakeCase,
                        #ColumnSnakeCase,
                        #AddOperatorSnakeCase,
                    )
                }
            });
                quote::quote! {
                    match &self {
                        #(#vrts_token_stream),*
                    }
                }
            },
            is_query_bind_mut,
            &{
                let vrts_token_stream = variants.iter().map(|element| {
                let element_upper_camel_case = element.ucc();
                quote::quote! {
                    Self::#element_upper_camel_case(#VSnakeCase) => pg_crud_common::domain_types::PgTypeWhereFilter::query_bind(
                        #VSnakeCase,
                        #QuerySnakeCase
                    )
                }
            });
                quote::quote! {
                    match self {
                        #(#vrts_token_stream),*
                    }
                }
            },
            &Import::PgCrudCommon,
        );
    let impl_location_lib_to_err_string_for_pg_type_tokens_where_token_stream =
        generate_impl_to_err_string_no_generics_token_stream(
            &identifier,
            &quote::quote! {format!("{self:#?}")},
        );
    let impl_all_variants_default_some_one_element_for_pg_type_tokens_where_token_stream =
        generate_impl_pg_crud_common_all_variants_default_some_one_element_token_stream(
            &identifier,
            &{
                let vrts_token_stream = variants.iter().map(|element| {
                let element_upper_camel_case = element.ucc();
                quote::quote! {Self::#element_upper_camel_case(#PgCrudCommonDefaultSomeOneElementCall)}
            });
                quote::quote! {vec![#(#vrts_token_stream),*]}
            },
        );
    quote::quote! {
        #pg_type_tokens_where_token_stream
        #impl_pg_type_pg_type_where_filter_for_pg_type_tokens_where_token_stream
        #impl_location_lib_to_err_string_for_pg_type_tokens_where_token_stream
        #impl_all_variants_default_some_one_element_for_pg_type_tokens_where_token_stream
    }
    .into()
}
pub fn generate_impl_to_err_string_no_generics_token_stream(
    identifier: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream{
    macro_helpers::domain_types::generate_impl_to_err_string_token_stream::generate_impl_to_err_string_token_stream(
        &proc_macro2::TokenStream::new(),
        identifier,
        &proc_macro2::TokenStream::new(),
        ts,
    )
}
pub fn generate_impl_display_and_to_err_string_debug_token_stream(
    identifier: &dyn quote::ToTokens,
) -> macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream{
    let impl_display_token_stream =
        macro_helpers::domain_types::generate_impl_display_token_stream::generate_impl_display_token_stream(
            &proc_macro2::TokenStream::new(),
            identifier,
            &proc_macro2::TokenStream::new(),
            &quote::quote! {write!(f, "{self:?}")},
        );
    let impl_to_err_string_token_stream = generate_impl_to_err_string_no_generics_token_stream(
        identifier,
        &quote::quote! {format!("{self:#?}")},
    );
    quote::quote! {
        #impl_display_token_stream
        #impl_to_err_string_token_stream
    }
    .into()
}
#[must_use]
pub fn pg_crud_common_query_part_error_token_stream()
-> macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream{
    let names = NamesCtx::new();
    #[allow(non_snake_case)]
    let (QueryPartErrorUpperCamelCase,) = (&names.QueryPartErrorUpperCamelCase,);
    quote::quote! {pg_crud_common::domain_types::#QueryPartErrorUpperCamelCase}.into()
}
#[must_use]
pub fn generate_dimension_number_pagination_token_stream(
    dimension_number: DimensionNumber,
) -> macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream{
    let identifier = quote::format_ident!("dimension{}_pagination", dimension_number.get());
    quote::quote! {#identifier}.into()
}
pub fn generate_struct_identifier_double_quoted_token_stream(
    v: &dyn std::fmt::Display,
) -> generate_quotes::domain_types::ProcMacro2QuotedLiteralTokenStream {
    generate_quotes::domain_types::dq_token_stream(&format!("struct {v}"))
}
pub fn generate_struct_identifier_with_number_els_double_quoted_token_stream(
    identifier: &dyn naming::domain_types::DisplayPlusToTokens,
    len: StructElsLen,
) -> generate_quotes::domain_types::ProcMacro2QuotedLiteralTokenStream {
    generate_quotes::domain_types::dq_token_stream(&format!(
        "struct {identifier} with {} els",
        len.get()
    ))
}
pub fn generate_sqlx_types_json_type_declaration_token_stream(
    type_token_stream: &dyn quote::ToTokens,
) -> macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream{
    quote::quote! {sqlx::types::Json<#type_token_stream>}.into()
}
pub fn generate_optional_type_declaration_token_stream(
    type_token_stream: &dyn quote::ToTokens,
) -> macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream{
    quote::quote! {Option<#type_token_stream>}.into()
}
pub fn generate_vec_tokens_declaration_token_stream(
    type_token_stream: &dyn quote::ToTokens,
) -> macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream{
    quote::quote! {Vec<#type_token_stream>}.into()
}
pub fn generate_de_double_quoted_token_stream(
    identifier: &dyn naming::domain_types::DisplayPlusToTokens,
    len: DeLen,
) -> (
    generate_quotes::domain_types::ProcMacro2QuotedLiteralTokenStream,
    generate_quotes::domain_types::ProcMacro2QuotedLiteralTokenStream,
    generate_quotes::domain_types::ProcMacro2QuotedLiteralTokenStream,
) {
    let struct_pg_type_identifier_where_tokens_double_quoted_token_stream =
        generate_struct_identifier_double_quoted_token_stream(identifier);
    let struct_pg_type_identifier_where_tokens_with_number_els_double_quoted_token_stream =
        generate_struct_identifier_with_number_els_double_quoted_token_stream(
            identifier,
            StructElsLen::from(len.get()),
        );
    let pg_type_identifier_where_tokens_double_quoted_token_stream =
        generate_quotes::domain_types::dq_token_stream(&identifier);
    (
        struct_pg_type_identifier_where_tokens_double_quoted_token_stream,
        struct_pg_type_identifier_where_tokens_with_number_els_double_quoted_token_stream,
        pg_type_identifier_where_tokens_double_quoted_token_stream,
    )
}
pub fn generate_impl_default_some_one_element_token_stream(
    impl_generic_token_stream: &dyn quote::ToTokens,
    import: &Import,
    identifier: &dyn quote::ToTokens,
    identifier_generic_token_stream: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream{
    let names = NamesCtx::new();
    #[allow(non_snake_case)]
    let (DefaultSomeOneElementSnakeCase,) = (&names.DefaultSomeOneElementSnakeCase,);
    let path_trait_token_stream = import.default_some_one_element();
    quote::quote! {
        impl #impl_generic_token_stream #path_trait_token_stream for #identifier #identifier_generic_token_stream {
            fn #DefaultSomeOneElementSnakeCase() -> Self {
                #ts
            }
        }
    }
    .into()
}
pub fn generate_impl_all_variants_default_some_one_element_token_stream(
    import: &Import,
    identifier: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream{
    let names = NamesCtx::new();
    #[allow(non_snake_case)]
    let (AllVariantsDefaultSomeOneElementSnakeCase,) =
        (&names.AllVariantsDefaultSomeOneElementSnakeCase,);
    let path_trait_token_stream = import.all_variants_default_some_one_element();
    let all_enum_variants = import.all_enum_variants();
    quote::quote! {
        impl #path_trait_token_stream for #identifier {
            fn #AllVariantsDefaultSomeOneElementSnakeCase() -> #all_enum_variants<Self> {
                (#ts).into()
            }
        }
    }
    .into()
}
pub fn generate_impl_default_some_one_element_max_page_size_token_stream(
    impl_generic_token_stream: &dyn quote::ToTokens,
    import: &Import,
    identifier: &dyn quote::ToTokens,
    identifier_generic_token_stream: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream{
    let names = NamesCtx::new();
    #[allow(non_snake_case)]
    let (DefaultSomeOneElementMaxPageSizeSnakeCase,) =
        (&names.DefaultSomeOneElementMaxPageSizeSnakeCase,);
    let path_trait_token_stream = import.default_some_one_element_max_page_size();
    quote::quote! {
        impl #impl_generic_token_stream #path_trait_token_stream for #identifier #identifier_generic_token_stream {
            fn #DefaultSomeOneElementMaxPageSizeSnakeCase() -> Self {
                #ts
            }
        }
    }
    .into()
}
pub fn generate_impl_all_variants_default_some_one_element_max_page_size_token_stream(
    import: &Import,
    identifier: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream{
    let names = NamesCtx::new();
    #[allow(non_snake_case)]
    let (AllVariantsDefaultSomeOneElementMaxPageSizeSnakeCase,) =
        (&names.AllVariantsDefaultSomeOneElementMaxPageSizeSnakeCase,);
    let path_trait_token_stream = import.all_variants_default_some_one_element_max_page_size();
    let all_enum_variants = import.all_enum_variants();
    let all_variants_default_some_one_element_max_page_size_snake_case =
        AllVariantsDefaultSomeOneElementMaxPageSizeSnakeCase;
    quote::quote! {
        impl #path_trait_token_stream for #identifier {
            fn #all_variants_default_some_one_element_max_page_size_snake_case() -> #all_enum_variants<Self> {
                (#ts).into()
            }
        }
    }
    .into()
}
pub fn generate_impl_pg_crud_common_default_some_one_element_token_stream(
    identifier: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream{
    generate_impl_default_some_one_element_token_stream(
        &proc_macro2::TokenStream::new(),
        &Import::PgCrudCommon,
        identifier,
        &proc_macro2::TokenStream::new(),
        ts,
    )
}
pub fn generate_impl_pg_crud_default_some_one_element_token_stream(
    identifier: &dyn quote::ToTokens,
    lt_token_stream: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream{
    generate_impl_default_some_one_element_token_stream(
        &proc_macro2::TokenStream::new(),
        &Import::PgCrudCommon,
        identifier,
        lt_token_stream,
        ts,
    )
}
pub fn generate_impl_pg_crud_common_all_variants_default_some_one_element_token_stream(
    identifier: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream{
    generate_impl_all_variants_default_some_one_element_token_stream(
        &Import::PgCrudCommon,
        identifier,
        ts,
    )
}
pub fn generate_impl_pg_crud_all_variants_default_some_one_element_token_stream(
    identifier: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream{
    generate_impl_all_variants_default_some_one_element_token_stream(
        &Import::PgCrudCommon,
        identifier,
        ts,
    )
}
pub fn generate_impl_pg_crud_common_default_some_one_element_max_page_size_token_stream(
    identifier: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream{
    generate_impl_default_some_one_element_max_page_size_token_stream(
        &proc_macro2::TokenStream::new(),
        &Import::PgCrudCommon,
        identifier,
        &proc_macro2::TokenStream::new(),
        ts,
    )
}
pub fn generate_impl_pg_crud_default_some_one_element_max_page_size_token_stream(
    identifier: &dyn quote::ToTokens,
    lt_token_stream: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream{
    generate_impl_default_some_one_element_max_page_size_token_stream(
        &proc_macro2::TokenStream::new(),
        &Import::PgCrudCommon,
        identifier,
        lt_token_stream,
        ts,
    )
}
pub fn generate_impl_pg_crud_all_variants_default_some_one_element_max_page_size_token_stream(
    identifier: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream{
    generate_impl_all_variants_default_some_one_element_max_page_size_token_stream(
        &Import::PgCrudCommon,
        identifier,
        ts,
    )
}
pub fn impl_pg_type_where_filter_for_identifier_token_stream(
    impl_generic_token_stream: &dyn quote::ToTokens,
    identifier_token_stream: &dyn quote::ToTokens,
    identifier_generic_token_stream: &dyn quote::ToTokens,
    increment_parameter_undrscr: &IncrementParameterUndrscr,
    column_parameter_undrscr: &ColumnParameterUndrscr,
    add_operator_undrscr: &AddOperatorUndrscr,
    query_part_token_stream: &dyn quote::ToTokens,
    is_query_bind_mut: &IsQueryBindMut,
    query_bind_token_stream: &dyn quote::ToTokens,
    import: &Import,
) -> macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream{
    let names = NamesCtx::new();
    #[allow(non_snake_case)]
    let (
        AllowClippyArbitrarySrcItemOrdering,
        PgTypeWhereFilterUpperCamelCase,
        QueryBindSnakeCase,
        QueryPartErrorUpperCamelCase,
        QueryPartSnakeCase,
    ) = (
        &names.AllowClippyArbitrarySrcItemOrdering,
        &names.PgTypeWhereFilterUpperCamelCase,
        &names.QueryBindSnakeCase,
        &names.QueryPartErrorUpperCamelCase,
        &names.QueryPartSnakeCase,
    );
    quote::quote! {
        #AllowClippyArbitrarySrcItemOrdering
        impl #impl_generic_token_stream #import ::#PgTypeWhereFilterUpperCamelCase<'lt> for #identifier_token_stream #identifier_generic_token_stream {
            fn #QueryPartSnakeCase(
                &self,
                #increment_parameter_undrscr: &mut dyn #import::QueryPartIncrementMut,
                #column_parameter_undrscr: #import::SqlColumnRef<'_>,
                #add_operator_undrscr: #import::AddOperator
            ) -> Result<#import::QueryPartFragment, #import::#QueryPartErrorUpperCamelCase> {
                #query_part_token_stream
            }
            fn #QueryBindSnakeCase(self, #is_query_bind_mut query: #import::SqlxPostgresQuery<'lt>) -> Result<
                #import::SqlxPostgresQuery<'lt>,
                #import::SqlxPostgresQueryBindError
            > {
                #query_bind_token_stream
            }
        }
    }
    .into()
}
pub fn generate_impl_sqlx_encode_sqlx_pg_for_identifier_token_stream(
    identifier_token_stream: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream{
    quote::quote! {
        impl sqlx::Encode<'_, sqlx::Postgres> for #identifier_token_stream {
            fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
                sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&#ts, buf)
            }
        }
    }.into()
}
pub fn generate_impl_sqlx_decode_sqlx_pg_for_identifier_token_stream(
    identifier_token_stream: &dyn quote::ToTokens,
    type_token_stream: &dyn quote::ToTokens,
    ok_v_match_token_stream: &dyn quote::ToTokens,
) -> macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream{
    let names = NamesCtx::new();
    #[allow(non_snake_case)]
    let (ValueSnakeCase,) = (&names.ValueSnakeCase,);
    quote::quote! {
        impl sqlx::Decode<'_, sqlx::Postgres> for #identifier_token_stream {
            fn decode(#ValueSnakeCase: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
                match <#type_token_stream as sqlx::Decode<sqlx::Postgres>>::decode(#ValueSnakeCase) {
                    Ok(v) => #ok_v_match_token_stream,
                    Err(error) => Err(error),
                }
            }
        }
    }.into()
}
pub fn generate_impl_sqlx_type_for_identifier_token_stream(
    identifier_token_stream: &dyn quote::ToTokens,
    type_token_stream: &dyn quote::ToTokens,
) -> macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream{
    quote::quote! {
        impl sqlx::Type<sqlx::Postgres> for #identifier_token_stream {
            fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
                <#type_token_stream as sqlx::Type<sqlx::Postgres>>::compatible(ty)
            }
            fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
               <#type_token_stream as sqlx::Type<sqlx::Postgres>>::type_info()
            }
        }
    }
    .into()
}
pub fn generate_impl_sqlx_type_and_encode_for_identifier_token_stream(
    identifier_token_stream: &dyn quote::ToTokens,
    type_token_stream: &dyn quote::ToTokens,
    encode_token_stream: &dyn quote::ToTokens,
) -> macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream{
    let impl_type_token_stream = generate_impl_sqlx_type_for_identifier_token_stream(
        identifier_token_stream,
        type_token_stream,
    );
    let impl_encode_token_stream = generate_impl_sqlx_encode_sqlx_pg_for_identifier_token_stream(
        identifier_token_stream,
        encode_token_stream,
    );
    quote::quote! {
        #impl_type_token_stream
        #impl_encode_token_stream
    }
    .into()
}
pub fn generate_impl_pg_type_token_stream(
    import: &Import,
    identifier: &dyn quote::ToTokens,
    identifier_table_type_upper_camel_case: &dyn quote::ToTokens,
    is_primary_key_undrscr: &IsPrimaryKeyUndrscr,
    create_table_column_query_part_token_stream: &dyn quote::ToTokens,
    identifier_create_upper_camel_case: &dyn quote::ToTokens,
    create_query_part_v_undrscr: &CreateQueryPartValueUndrscr,
    create_query_part_increment_undrscr: &CreateQueryPartIncrementUndrscr,
    create_query_part_token_stream: &dyn quote::ToTokens,
    create_query_bind_v_undrscr: &CreateQueryBindValueUndrscr,
    is_create_query_bind_mut: &IsCreateQueryBindMut,
    create_query_bind_token_stream: &dyn quote::ToTokens,
    identifier_select_upper_camel_case: &dyn quote::ToTokens,
    select_query_part_v_undrscr: &SelectQueryPartValueUndrscr,
    select_query_part_token_stream: &dyn quote::ToTokens,
    identifier_where_upper_camel_case: &dyn quote::ToTokens,
    identifier_read_upper_camel_case: &dyn quote::ToTokens,
    normalize_token_stream: &dyn quote::ToTokens,
    read_ids_token_stream: &dyn quote::ToTokens,
    select_only_ids_query_part_token_stream: &dyn quote::ToTokens,
    identifier_read_inner_upper_camel_case: &dyn quote::ToTokens,
    into_inner_token_stream: &dyn quote::ToTokens,
    identifier_update_upper_camel_case: &dyn quote::ToTokens,
    identifier_update_for_query_upper_camel_case: &dyn quote::ToTokens,
    update_query_part_v_undrscr: &UpdateQueryPartValueUndrscr,
    update_query_part_accumulator_undrscr: &UpdateQueryPartAccumulatorUndrscr,
    update_query_part_target_undrscr: &UpdateQueryPartTargetUndrscr,
    update_query_part_path_undrscr: &UpdateQueryPartPathUndrscr,
    update_query_part_token_stream: &dyn quote::ToTokens,
    is_update_query_bind_mut: &IsUpdateQueryBindMut,
    update_query_bind_token_stream: &dyn quote::ToTokens,
    select_only_updated_ids_query_part_token_stream: &dyn quote::ToTokens,
    is_select_only_updated_ids_query_bind_mut: &IsSelectOnlyUpdatedIdsQueryBindMut,
    select_only_updated_ids_query_bind_token_stream: &dyn quote::ToTokens,
) -> macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream{
    let names = NamesCtx::new();
    #[allow(non_snake_case)]
    let (
        AllowClippyArbitrarySrcItemOrdering,
        ColumnSnakeCase,
        CreateQueryBindSnakeCase,
        CreateQueryPartSnakeCase,
        CreateTableColumnQueryPartSnakeCase,
        CreateUpperCamelCase,
        IncrementSnakeCase,
        NormalizeSnakeCase,
        PgTypeUpperCamelCase,
        QueryPartErrorUpperCamelCase,
        QuerySnakeCase,
        ReadIdsUpperCamelCase,
        ReadInnerUpperCamelCase,
        ReadUpperCamelCase,
        SelectOnlyIdsQueryPartSnakeCase,
        SelectOnlyUpdatedIdsQueryBindSnakeCase,
        SelectOnlyUpdatedIdsQueryPartSnakeCase,
        SelectQueryPartSnakeCase,
        SelectUpperCamelCase,
        TableTypeUpperCamelCase,
        UpdateForQueryUpperCamelCase,
        UpdateQueryBindSnakeCase,
        UpdateQueryPartSnakeCase,
        UpdateUpperCamelCase,
        VSnakeCase,
        WhereUpperCamelCase,
    ) = (
        &names.AllowClippyArbitrarySrcItemOrdering,
        &names.ColumnSnakeCase,
        &names.CreateQueryBindSnakeCase,
        &names.CreateQueryPartSnakeCase,
        &names.CreateTableColumnQueryPartSnakeCase,
        &names.CreateUpperCamelCase,
        &names.IncrementSnakeCase,
        &names.NormalizeSnakeCase,
        &names.PgTypeUpperCamelCase,
        &names.QueryPartErrorUpperCamelCase,
        &names.QuerySnakeCase,
        &names.ReadIdsUpperCamelCase,
        &names.ReadInnerUpperCamelCase,
        &names.ReadUpperCamelCase,
        &names.SelectOnlyIdsQueryPartSnakeCase,
        &names.SelectOnlyUpdatedIdsQueryBindSnakeCase,
        &names.SelectOnlyUpdatedIdsQueryPartSnakeCase,
        &names.SelectQueryPartSnakeCase,
        &names.SelectUpperCamelCase,
        &names.TableTypeUpperCamelCase,
        &names.UpdateForQueryUpperCamelCase,
        &names.UpdateQueryBindSnakeCase,
        &names.UpdateQueryPartSnakeCase,
        &names.UpdateUpperCamelCase,
        &names.VSnakeCase,
        &names.WhereUpperCamelCase,
    );
    quote::quote! {
        #AllowClippyArbitrarySrcItemOrdering
        impl #import :: #PgTypeUpperCamelCase for #identifier {
            type #TableTypeUpperCamelCase = #identifier_table_type_upper_camel_case;
            fn #CreateTableColumnQueryPartSnakeCase(#ColumnSnakeCase: #import::SqlColumnRef<'_>, #is_primary_key_undrscr: #import::IsPrimaryKey) -> #import::QueryPartFragment {
                #create_table_column_query_part_token_stream
            }
            type #CreateUpperCamelCase = #identifier_create_upper_camel_case;
            fn #CreateQueryPartSnakeCase(
                #create_query_part_v_undrscr: &Self::#CreateUpperCamelCase,
                #create_query_part_increment_undrscr: &mut dyn #import::QueryPartIncrementMut
            ) -> Result<#import::QueryPartFragment, #import ::#QueryPartErrorUpperCamelCase> {
                #create_query_part_token_stream
            }
            fn #CreateQueryBindSnakeCase(
                #create_query_bind_v_undrscr: Self::#CreateUpperCamelCase,
                #is_create_query_bind_mut #QuerySnakeCase: #import::SqlxPostgresQuery<'_>
            ) -> Result<#import::SqlxPostgresQuery<'_>, #import::SqlxPostgresQueryBindError> {
                #create_query_bind_token_stream
            }
            type #SelectUpperCamelCase = #identifier_select_upper_camel_case;
            fn #SelectQueryPartSnakeCase(
                #select_query_part_v_undrscr: &Self::#SelectUpperCamelCase,
                #ColumnSnakeCase: #import::SqlColumnRef<'_>,
            ) -> Result<#import::QueryPartFragment, #import ::#QueryPartErrorUpperCamelCase> {
                #select_query_part_token_stream
            }
            type #WhereUpperCamelCase = #identifier_where_upper_camel_case;
            type #ReadUpperCamelCase = #identifier_read_upper_camel_case;
            fn #NormalizeSnakeCase(#VSnakeCase: Self::#ReadUpperCamelCase) -> Self::#ReadUpperCamelCase {
                #normalize_token_stream
            }
            type #ReadIdsUpperCamelCase = #read_ids_token_stream;
            fn #SelectOnlyIdsQueryPartSnakeCase(
                #ColumnSnakeCase: #import::SqlColumnRef<'_>
            ) -> Result<#import::QueryPartFragment, #import ::#QueryPartErrorUpperCamelCase> {
                #select_only_ids_query_part_token_stream
            }
            type #ReadInnerUpperCamelCase = #identifier_read_inner_upper_camel_case;
            fn into_inner(#VSnakeCase: Self::#ReadUpperCamelCase) -> Self::#ReadInnerUpperCamelCase {
                #into_inner_token_stream
            }
            type #UpdateUpperCamelCase = #identifier_update_upper_camel_case;
            type #UpdateForQueryUpperCamelCase = #identifier_update_for_query_upper_camel_case;
            #[allow(unused_variables)]
            fn #UpdateQueryPartSnakeCase(
                #update_query_part_v_undrscr: &Self::#UpdateForQueryUpperCamelCase,
                #update_query_part_accumulator_undrscr: #import::SqlColumnRef<'_>,
                #update_query_part_target_undrscr: #import::SqlColumnRef<'_>,
                #update_query_part_path_undrscr: #import::SqlColumnRef<'_>,
                #IncrementSnakeCase: &mut dyn #import::QueryPartIncrementMut
            ) -> Result<#import::QueryPartFragment, #import ::#QueryPartErrorUpperCamelCase> {
                #update_query_part_token_stream
            }
            fn #UpdateQueryBindSnakeCase(
                #VSnakeCase: Self::#UpdateForQueryUpperCamelCase,
                #is_update_query_bind_mut #QuerySnakeCase: #import::SqlxPostgresQuery<'_>
            ) -> Result<#import::SqlxPostgresQuery<'_>, #import::SqlxPostgresQueryBindError> {
                #update_query_bind_token_stream
            }
            fn #SelectOnlyUpdatedIdsQueryPartSnakeCase(
                #VSnakeCase: &Self::#UpdateForQueryUpperCamelCase,
                #ColumnSnakeCase: #import::SqlColumnRef<'_>,
                #IncrementSnakeCase: &mut dyn #import::QueryPartIncrementMut,
            ) -> Result<#import::QueryPartFragment, #import ::#QueryPartErrorUpperCamelCase> {
                #select_only_updated_ids_query_part_token_stream
            }
            fn #SelectOnlyUpdatedIdsQueryBindSnakeCase<'lt>(
                #VSnakeCase: &'lt Self::#UpdateForQueryUpperCamelCase,
                #is_select_only_updated_ids_query_bind_mut #QuerySnakeCase: #import::SqlxPostgresQuery<'lt>
            ) -> Result<#import::SqlxPostgresQuery<'lt>, #import::SqlxPostgresQueryBindError> {
                #select_only_updated_ids_query_bind_token_stream
            }
        }
    }.into()
}
pub fn generate_impl_pg_type_not_primary_key_for_identifier_token_stream(
    import: &Import,
    identifier: &dyn quote::ToTokens,
) -> macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream{
    let identifier_create_upper_camel_case =
        naming::domain_types::parameter::SelfCreateUpperCamelCase::from_tokens(&identifier);
    let allow_clippy_arbitrary_src_item_ordering =
        token_patterns::AllowClippyArbitrarySrcItemOrdering;
    let pg_type_not_primary_key_upper_camel_case =
        naming::domain_types::PgTypeNotPrimaryKeyUpperCamelCase;
    let pg_type_upper_camel_case = naming::domain_types::PgTypeUpperCamelCase;
    let create_upper_camel_case = naming::domain_types::CreateUpperCamelCase;
    quote::quote! {
        #allow_clippy_arbitrary_src_item_ordering
        impl #import::#pg_type_not_primary_key_upper_camel_case for #identifier {
            type #pg_type_upper_camel_case = Self;
            type #create_upper_camel_case = #identifier_create_upper_camel_case;
        }
    }
    .into()
}
#[must_use]
pub fn pg_crud_common_query_part_error_checked_add_initialization_token_stream()
-> macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream{
    quote::quote! {pg_crud_common::domain_types::QueryPartError::CheckedAdd { location: location_macros::location!() }}.into()
}
pub fn generate_impl_crate_is_string_empty_for_identifier_token_stream(
    identifier: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream{
    quote::quote! {
        impl pg_crud_common::domain_types::IsStringEmpty for #identifier {
            fn is_string_empty(&self) -> pg_crud_common::domain_types::IsStringEmptyRes {
                pg_crud_common::domain_types::IsStringEmptyRes::from(#ts)
            }
        }
    }
    .into()
}
pub fn generate_match_try_new_in_de_token_stream(
    identifier: &dyn quote::ToTokens,
    initialization_token_stream: &dyn quote::ToTokens,
) -> macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream{
    quote::quote! {
        match #identifier::try_new(#initialization_token_stream) {
            Ok(v) => Ok(v),
            Err(error) => Err(serde::de::Error::custom(format!("{error:?}")))
        }
    }
    .into()
}
pub fn generate_impl_de_for_struct_token_stream(
    identifier: &dyn naming::domain_types::DisplayPlusToTokens,
    vec_identifier_type: SynIdentifierTypeRefs<'_>,
    _len: DeLen,
    generate_type_token_stream: &dyn Fn(
        &syn::Ident,
        &syn::Type,
    ) -> macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream,
) -> macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream{
    let allow_clippy_arbitrary_src_item_ordering =
        token_patterns::AllowClippyArbitrarySrcItemOrdering;
    let raw_identifier_token_stream = quote::format_ident!("{}Raw", identifier.to_string());
    let raw_fields_token_stream = vec_identifier_type.0.iter().map(|(field, ty)| {
        let type_token_stream = generate_type_token_stream(field, ty);
        quote::quote! { #field: #type_token_stream, }
    });
    let try_from_fields_token_stream = vec_identifier_type.0.iter().map(|(field, _)| {
        quote::quote! { raw.#field }
    });
    quote::quote! {
        #[derive(serde::Deserialize)]
        #[allow(clippy::arbitrary_source_item_ordering)]
        struct #raw_identifier_token_stream {
            #(#raw_fields_token_stream)*
        }
        #[allow(unused_qualifications)]
        #[allow(clippy::absolute_paths)]
        #allow_clippy_arbitrary_src_item_ordering
        const _: () = {
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for #identifier {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    let raw = <#raw_identifier_token_stream as _serde::Deserialize>::deserialize(__deserializer)?;
                    Self::try_new(#(#try_from_fields_token_stream),*).map_err(|error| _serde::de::Error::custom(format!("{error:?}")))
                }
            }
        };
    }.into()
}
pub fn generate_impl_de_for_struct_by_fields_token_stream(
    identifier: &dyn naming::domain_types::DisplayPlusToTokens,
    fields: SynFieldRefs<'_>,
    _len: DeLen,
    generate_type_token_stream: &dyn Fn(
        &syn::Ident,
        &syn::Type,
    ) -> macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream,
) -> macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream{
    let allow_clippy_arbitrary_src_item_ordering =
        token_patterns::AllowClippyArbitrarySrcItemOrdering;
    let raw_identifier_token_stream = quote::format_ident!("{}Raw", identifier.to_string());
    let raw_fields_token_stream = fields.0.iter().map(|field| {
        let ty = field.type0.as_ref();
        let field_identifier = field.identifier.as_ref();
        let type_token_stream = generate_type_token_stream(field_identifier, ty);
        quote::quote! { #field_identifier: #type_token_stream, }
    });
    let try_from_fields_token_stream = fields.0.iter().map(|field| {
        let field_identifier = field.identifier.as_ref();
        quote::quote! { raw.#field_identifier }
    });
    quote::quote! {
        #[derive(serde::Deserialize)]
        #[allow(clippy::arbitrary_source_item_ordering)]
        struct #raw_identifier_token_stream {
            #(#raw_fields_token_stream)*
        }
        #[allow(unused_qualifications)]
        #[allow(clippy::absolute_paths)]
        #allow_clippy_arbitrary_src_item_ordering
        const _: () = {
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for #identifier {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    let raw = <#raw_identifier_token_stream as _serde::Deserialize>::deserialize(__deserializer)?;
                    Self::try_new(#(#try_from_fields_token_stream),*).map_err(|error| _serde::de::Error::custom(format!("{error:?}")))
                }
            }
        };
    }.into()
}
pub fn wrap_into_scopes_token_stream(
    ts: &dyn quote::ToTokens,
) -> macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream{
    quote::quote! {(#ts)}.into()
}
pub fn maybe_wrap_into_braces_token_stream(
    ts: &dyn quote::ToTokens,
    wrap: WrapIntoBraces,
) -> macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream{
    if bool::from(wrap) {
        wrap_into_scopes_token_stream(&ts)
    } else {
        quote::quote! {#ts}.into()
    }
}
pub fn generate_v_declaration_token_stream(
    import: &Import,
    ts: &dyn quote::ToTokens,
) -> macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream{
    quote::quote! {#import::V<#ts>}.into()
}
pub fn generate_v_initialization_token_stream(
    import: &Import,
    ts: &dyn quote::ToTokens,
) -> macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream{
    let names = NamesCtx::new();
    #[allow(non_snake_case)]
    let (VSnakeCase,) = (&names.VSnakeCase,);
    quote::quote! {#import::V { #VSnakeCase: #ts }}.into()
}
pub fn impl_pg_type_eq_operator_for_identifier_token_stream(
    import: &Import,
    identifier: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream{
    let names = NamesCtx::new();
    #[allow(non_snake_case)]
    let (EqOperatorUpperCamelCase, PgTypeEqOperatorUpperCamelCase) = (
        &names.EqOperatorUpperCamelCase,
        &names.PgTypeEqOperatorUpperCamelCase,
    );
    quote::quote! {
        impl #import::#PgTypeEqOperatorUpperCamelCase for #identifier {
            fn operator(&self) -> #import::#EqOperatorUpperCamelCase {
                #ts
            }
        }
    }
    .into()
}
#[must_use]
pub fn generate_query_part_error_write_into_buffer_token_stream(
    import: Import,
) -> macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream{
    quote::quote! {
        #import::QueryPartError::WriteIntoBuffer {
            location: location_macros::location!()
        }
    }
    .into()
}
#[must_use]
pub fn generate_return_err_query_part_error_write_into_buffer_token_stream(
    import: Import,
) -> macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream{
    let ts = generate_query_part_error_write_into_buffer_token_stream(import);
    quote::quote! {return Err(#ts);}.into()
}
