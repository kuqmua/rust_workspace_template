mod flts;
pub use flts::*;
const IS_NL_PREFIX_STR_MAX_LEN: usize = 1_048_576;
#[derive(Debug, Clone, optml::Optml)]
pub enum DeriveOrImpl {
    Derive,
    Impl(macros_helpers::GeneratedRustTs),
}
#[derive(Debug, Clone, Default)]
pub struct GeneratedRustTsVec(Vec<macros_helpers::GeneratedRustTs>);
impl From<Vec<macros_helpers::GeneratedRustTs>> for GeneratedRustTsVec {
    fn from(value: Vec<macros_helpers::GeneratedRustTs>) -> Self {
        Self(value)
    }
}
impl quote::ToTokens for GeneratedRustTsVec {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.extend(self.0.iter().map(quote::ToTokens::to_token_stream));
    }
}
impl FromIterator<macros_helpers::GeneratedRustTs> for GeneratedRustTsVec {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = macros_helpers::GeneratedRustTs>,
    {
        Self(iter.into_iter().collect())
    }
}
#[derive(Debug, Clone, Copy)]
pub struct NnOrNlStr(&'static str);
impl From<&'static str> for NnOrNlStr {
    fn from(value: &'static str) -> Self {
        Self(value)
    }
}
impl std::fmt::Display for NnOrNlStr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
#[derive(Debug, Clone)]
pub struct IsNlPrefixStr(String);
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IsNlPrefixStrTryFromStringEr {
    TooLong { len: usize, max: usize },
}
impl std::fmt::Display for IsNlPrefixStrTryFromStringEr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TooLong { len, max } => {
                write!(f, "is nl prefix string length {len} exceeds maximum {max}")
            }
        }
    }
}
impl From<IsNlPrefixStrTryFromStringEr> for IsNlPrefixStr {
    fn from(value: IsNlPrefixStrTryFromStringEr) -> Self {
        Self(value.to_string())
    }
}
impl TryFrom<String> for IsNlPrefixStr {
    type Error = IsNlPrefixStrTryFromStringEr;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > IS_NL_PREFIX_STR_MAX_LEN {
            return Err(Self::Error::TooLong {
                len: value.len(),
                max: IS_NL_PREFIX_STR_MAX_LEN,
            });
        }
        Ok(Self(value))
    }
}
impl std::fmt::Display for IsNlPrefixStr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct ImportScStr(&'static str);
impl From<&'static str> for ImportScStr {
    fn from(value: &'static str) -> Self {
        Self(value)
    }
}
impl AsRef<str> for ImportScStr {
    fn as_ref(&self) -> &str {
        self.0
    }
}
impl std::fmt::Display for ImportScStr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct ImportPathStr(&'static str);
impl From<&'static str> for ImportPathStr {
    fn from(value: &'static str) -> Self {
        Self(value)
    }
}
impl std::fmt::Display for ImportPathStr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct DimNbr(usize);
impl From<usize> for DimNbr {
    fn from(value: usize) -> Self {
        Self(value)
    }
}
impl DimNbr {
    #[must_use]
    pub const fn get(self) -> usize {
        self.0
    }
}
#[derive(Debug, Clone, Copy)]
pub struct StructElsLen(usize);
impl From<usize> for StructElsLen {
    fn from(value: usize) -> Self {
        Self(value)
    }
}
impl StructElsLen {
    #[must_use]
    pub const fn get(self) -> usize {
        self.0
    }
}
#[derive(Debug, Clone, Copy)]
pub struct DeLen(usize);
impl From<usize> for DeLen {
    fn from(value: usize) -> Self {
        Self(value)
    }
}
impl DeLen {
    #[must_use]
    pub const fn get(self) -> usize {
        self.0
    }
}
#[derive(Debug, Clone, Copy)]
pub struct WrapIntoBraces(bool);
impl From<bool> for WrapIntoBraces {
    fn from(value: bool) -> Self {
        Self(value)
    }
}
impl From<WrapIntoBraces> for bool {
    fn from(value: WrapIntoBraces) -> Self {
        value.0
    }
}
#[derive(Debug, Clone)]
pub struct ParseTsStrings(Vec<String>);
impl From<Vec<String>> for ParseTsStrings {
    fn from(value: Vec<String>) -> Self {
        Self(value)
    }
}
impl ParseTsStrings {
    #[must_use]
    pub fn into_vec(self) -> Vec<String> {
        self.0
    }
}
#[derive(Debug, Clone, Copy)]
pub struct ParseTsTextRef<'lt>(&'lt str);
impl<'lt> From<&'lt str> for ParseTsTextRef<'lt> {
    fn from(value: &'lt str) -> Self {
        Self(value)
    }
}
impl AsRef<str> for ParseTsTextRef<'_> {
    fn as_ref(&self) -> &str {
        self.0
    }
}
#[derive(Debug, Clone, Copy)]
pub struct ParseErIdRef<'lt>(&'lt str);
impl<'lt> From<&'lt str> for ParseErIdRef<'lt> {
    fn from(value: &'lt str) -> Self {
        Self(value)
    }
}
impl AsRef<str> for ParseErIdRef<'_> {
    fn as_ref(&self) -> &str {
        self.0
    }
}
#[derive(Debug, Clone, Copy)]
pub struct PanicUuidRef<'lt>(&'lt str);
impl<'lt> From<&'lt str> for PanicUuidRef<'lt> {
    fn from(value: &'lt str) -> Self {
        Self(value)
    }
}
impl AsRef<str> for PanicUuidRef<'_> {
    fn as_ref(&self) -> &str {
        self.0
    }
}
#[derive(Debug, Clone, Copy)]
pub struct SynIdentTypeRefs<'lt>(&'lt [(&'lt syn::Ident, &'lt syn::Type)]);
impl<'lt> From<&'lt [(&'lt syn::Ident, &'lt syn::Type)]> for SynIdentTypeRefs<'lt> {
    fn from(value: &'lt [(&'lt syn::Ident, &'lt syn::Type)]) -> Self {
        Self(value)
    }
}
#[derive(Debug, Clone, Copy, optml::Optml)]
pub enum IsStdrtNn {
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
    optml::Optml,
)]
pub enum IsNl {
    #[default]
    False,
    True,
}
impl IsNl {
    #[must_use]
    pub fn mb_opt_wrap(
        &self,
        ts: macros_helpers::GeneratedRustTs,
    ) -> macros_helpers::GeneratedRustTs {
        match &self {
            Self::False => ts,
            Self::True => quote::quote! {Option<#ts>}.into(),
        }
    }
    #[must_use]
    pub fn mb_some_wrap(
        &self,
        ts: macros_helpers::GeneratedRustTs,
    ) -> macros_helpers::GeneratedRustTs {
        match &self {
            Self::False => ts,
            Self::True => quote::quote! {Some(#ts)}.into(),
        }
    }
    #[must_use]
    pub fn nn_or_nl_str(&self) -> NnOrNlStr {
        match &self {
            Self::False => NnOrNlStr::from("Nn"),
            Self::True => NnOrNlStr::from("Nl"),
        }
    }
    #[must_use]
    pub fn prefix_str(&self) -> IsNlPrefixStr {
        match &self {
            Self::False => {
                IsNlPrefixStr::try_from(String::default()).unwrap_or_else(IsNlPrefixStr::from)
            }
            Self::True => IsNlPrefixStr::try_from(String::from("StdOptOpt"))
                .unwrap_or_else(IsNlPrefixStr::from),
        }
    }
    #[must_use]
    pub fn rust(&self) -> &'static dyn std::fmt::Display {
        match &self {
            Self::False => &"",
            Self::True => &naming::OptUcc,
        }
    }
}
#[derive(Debug, Clone, Copy, optml::Optml)]
pub enum Import {
    Crate,
    PgCrud,
    PgCrudCmn,
}
impl Import {
    fn all_vrts_dflt_some_one_el(&self) -> &dyn quote::ToTokens {
        match &self {
            Self::Crate => &token_patterns::CrateAllEnumVrtsArrDfltSomeOneEl,
            Self::PgCrud => &token_patterns::PgCrudAllEnumVrtsArrDfltSomeOneEl,
            Self::PgCrudCmn => &token_patterns::PgCrudCmnAllEnumVrtsArrDfltSomeOneEl,
        }
    }
    fn all_vrts_dflt_some_one_el_max_page_size(&self) -> &dyn quote::ToTokens {
        match &self {
            Self::Crate => &token_patterns::CrateAllEnumVrtsArrDfltSomeOneElMaxPageSize,
            Self::PgCrud => &token_patterns::PgCrudAllEnumVrtsArrDfltSomeOneElMaxPageSize,
            Self::PgCrudCmn => &token_patterns::PgCrudCmnAllEnumVrtsArrDfltSomeOneElMaxPageSize,
        }
    }
    fn dflt_some_one_el(&self) -> &dyn quote::ToTokens {
        match &self {
            Self::Crate => &token_patterns::CrateDfltSomeOneEl,
            Self::PgCrud => &token_patterns::PgCrudDfltSomeOneEl,
            Self::PgCrudCmn => &token_patterns::PgCrudCmnDfltSomeOneEl,
        }
    }
    fn dflt_some_one_el_max_page_size(&self) -> &dyn quote::ToTokens {
        match &self {
            Self::Crate => &token_patterns::CrateDfltSomeOneElMaxPageSize,
            Self::PgCrud => &token_patterns::PgCrudDfltSomeOneElMaxPageSize,
            Self::PgCrudCmn => &token_patterns::PgCrudCmnDfltSomeOneElMaxPageSize,
        }
    }
    #[must_use]
    pub fn sc_str(&self) -> ImportScStr {
        match &self {
            Self::Crate => ImportScStr::from("crate"),
            Self::PgCrud => ImportScStr::from("pg_crud"),
            Self::PgCrudCmn => ImportScStr::from("pg_crud_cmn"),
        }
    }
    #[must_use]
    pub fn to_path(&self) -> ImportPathStr {
        match &self {
            Self::Crate => ImportPathStr::from("crate"),
            Self::PgCrud => ImportPathStr::from("pg_crud"),
            Self::PgCrudCmn => ImportPathStr::from("pg_crud_cmn"),
        }
    }
}
impl quote::ToTokens for Import {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        parse_ts_or_compile_error(
            ParseTsTextRef::from(self.sc_str().as_ref()),
            ParseErIdRef::from("d8636ee5"),
        )
        .to_tokens(tokens);
    }
}
pg_crud_macros_cmn_macros::bool_enum_to_tokens!(AddOprtrUndrscr, false => naming::AddOprtrSc, true => quote::quote! {_});
pg_crud_macros_cmn_macros::bool_enum_to_tokens!(ColPrmUndrscr, false => naming::ColSc, true => quote::quote! {_});
pg_crud_macros_cmn_macros::bool_enum_to_tokens!(IncrPrmUndrscr, false => naming::IncrSc, true => quote::quote! {_});
pg_crud_macros_cmn_macros::bool_enum_to_tokens!(IsCrQbMut, false => proc_macro2::TokenStream::new(), true => naming::MutSc);
pg_crud_macros_cmn_macros::bool_enum_to_tokens!(IsQbMut, false => proc_macro2::TokenStream::new(), true => naming::MutSc);
pg_crud_macros_cmn_macros::bool_enum_to_tokens!(IsSelOnlyCrdIdsQbMut, false => proc_macro2::TokenStream::new(), true => naming::MutSc);
pg_crud_macros_cmn_macros::bool_enum_to_tokens!(IsSelOnlyUpddIdsQbMut, false => proc_macro2::TokenStream::new(), true => naming::MutSc);
pg_crud_macros_cmn_macros::bool_enum_to_tokens!(IsSelQpColFieldForErMsgUsed, false => quote::quote! {_}, true => naming::ColFieldForErMsgSc);
pg_crud_macros_cmn_macros::bool_enum_to_tokens!(IsSelQpIsPgTypeUsed, false => quote::quote! {_}, true => quote::quote! {is_pg_type});
pg_crud_macros_cmn_macros::bool_enum_to_tokens!(IsSelQpSelfSelUsed, false => quote::quote! {_}, true => naming::VSc);
pg_crud_macros_cmn_macros::bool_enum_to_tokens!(IsUpdQbMut, false => proc_macro2::TokenStream::new(), true => naming::MutSc);
pg_crud_macros_cmn_macros::bool_enum_to_tokens!(IsUpdQpSelfUpdUsed, false => quote::quote! {_}, true => naming::VSc);
pg_crud_macros_cmn_macros::bool_enum_to_tokens!(ShouldDSchemarsJsonSchema, false => proc_macro2::TokenStream::new(), true => quote::quote! {, schemars::JsonSchema});
pg_crud_macros_cmn_macros::bool_enum_to_tokens!(ShouldDeriveUtoipaToSchema, false => proc_macro2::TokenStream::new(), true => quote::quote! {, utoipa::ToSchema});
#[derive(Debug, Clone, Copy, optml::Optml)]
pub enum RdOrUpd {
    Rd,
    Upd,
}
impl RdOrUpd {
    #[must_use]
    pub fn ucc(&self) -> &dyn naming::DisplayPlusToTokens {
        match &self {
            Self::Rd => &naming::RdUcc,
            Self::Upd => &naming::UpdUcc,
        }
    }
}
pg_crud_macros_cmn_macros::bool_enum_to_tokens!(IsPkUndrscr, false => naming::IsPkSc, true => quote::quote! {_});
#[derive(Debug, Clone, Copy, optml::Optml)]
pub enum DefaultSomeOneOrDefaultSomeOneWithMaxPageSize {
    DefaultSomeOne,
    DefaultSomeOneWithMaxPageSize,
}
#[derive(Debug, Clone, Copy, optml::Optml)]
pub enum EqOrEqUsingFields {
    Eq,
    EqUsingFields,
}
#[derive(Debug, Clone, Copy, optml::Optml)]
pub enum EqOprtrH {
    Eq,
    IsNull,
}
impl EqOprtrH {
    #[must_use]
    pub fn to_tokens_path(&self, import: &Import) -> macros_helpers::GeneratedRustTs {
        #[allow(non_snake_case, unused_variables)]
        let (
            AddOprtrSc,
            AllVrtsDfltSomeOneElMaxPageSizeSc,
            AllVrtsDfltSomeOneElSc,
            AllowClippyArbitrarySrcItemOrdering,
            ColSc,
            CrQbSc,
            CrQpSc,
            CrSc,
            CrTblColQpSc,
            CrUcc,
            DfltSomeOneElMaxPageSizeSc,
            DfltSomeOneElSc,
            EqOprtrUcc,
            ErSc,
            IncrSc,
            NormalizeSc,
            OptUpdSc,
            OptVecCrSc,
            PgCrudCmnDfltSomeOneElCall,
            PgTypeEqOprtrUcc,
            PgTypeNotPkUcc,
            PgTypeOptVecWhGreaterThanTestSc,
            PgTypeTestCasesUcc,
            PgTypeUcc,
            PgTypeWhFltUcc,
            PreviousRdAndOptUpdIntoRdSc,
            QbSc,
            QpErUcc,
            QpSc,
            QuerySc,
            RdIdsAndCrIntoOptVRdSc,
            RdIdsAndCrIntoOptVecWhEqToFieldSc,
            RdIdsAndCrIntoRdSc,
            RdIdsAndCrIntoTtSc,
            RdIdsAndCrIntoVecWhEqUsingFieldsSc,
            RdIdsAndCrIntoWhEqSc,
            RdIdsAndTtIntoPgTypeOptWhGreaterThanSc,
            RdIdsSc,
            RdIdsTo2DimsVecRdInnSc,
            RdIdsToOptVRdDfltSomeOneElSc,
            RdIdsUcc,
            RdInnIntoRdWithNewOrTryNewUnwrapedSc,
            RdInnIntoUpdWithNewOrTryNewUnwrapedSc,
            RdInnUcc,
            RdSc,
            RdUcc,
            SelOnlyIdsQpSc,
            SelOnlyUpddIdsQbSc,
            SelOnlyUpddIdsQpSc,
            SelQpSc,
            SelUcc,
            SelfUcc,
            TtSc,
            TtUcc,
            UpdForQueryUcc,
            UpdQbSc,
            UpdQpSc,
            UpdToRdIdsSc,
            UpdUcc,
            VSc,
            VUcc,
            ValueSc,
            WhUcc,
        ) = (
            naming::AddOprtrSc,
            naming::AllVrtsDfltSomeOneElMaxPageSizeSc,
            naming::AllVrtsDfltSomeOneElSc,
            token_patterns::AllowClippyArbitrarySrcItemOrdering,
            naming::ColSc,
            naming::CrQbSc,
            naming::CrQpSc,
            naming::CrSc,
            naming::CrTblColQpSc,
            naming::CrUcc,
            naming::DfltSomeOneElMaxPageSizeSc,
            naming::DfltSomeOneElSc,
            naming::EqOprtrUcc,
            naming::ErSc,
            naming::IncrSc,
            naming::NormalizeSc,
            naming::OptUpdSc,
            naming::OptVecCrSc,
            token_patterns::PgCrudCmnDfltSomeOneElCall,
            naming::PgTypeEqOprtrUcc,
            naming::PgTypeNotPkUcc,
            naming::PgTypeOptVecWhGreaterThanTestSc,
            naming::PgTypeTestCasesUcc,
            naming::PgTypeUcc,
            naming::PgTypeWhFltUcc,
            naming::PreviousRdAndOptUpdIntoRdSc,
            naming::QbSc,
            naming::QpErUcc,
            naming::QpSc,
            naming::QuerySc,
            naming::RdIdsAndCrIntoOptVRdSc,
            naming::RdIdsAndCrIntoOptVecWhEqToFieldSc,
            naming::RdIdsAndCrIntoRdSc,
            naming::RdIdsAndCrIntoTtSc,
            naming::RdIdsAndCrIntoVecWhEqUsingFieldsSc,
            naming::RdIdsAndCrIntoWhEqSc,
            naming::RdIdsAndTtIntoPgTypeOptWhGreaterThanSc,
            naming::RdIdsSc,
            naming::RdIdsTo2DimsVecRdInnSc,
            naming::RdIdsToOptVRdDfltSomeOneElSc,
            naming::RdIdsUcc,
            naming::RdInnIntoRdWithNewOrTryNewUnwrapedSc,
            naming::RdInnIntoUpdWithNewOrTryNewUnwrapedSc,
            naming::RdInnUcc,
            naming::RdSc,
            naming::RdUcc,
            naming::SelOnlyIdsQpSc,
            naming::SelOnlyUpddIdsQbSc,
            naming::SelOnlyUpddIdsQpSc,
            naming::SelQpSc,
            naming::SelUcc,
            naming::SelfUcc,
            naming::TtSc,
            naming::TtUcc,
            naming::UpdForQueryUcc,
            naming::UpdQbSc,
            naming::UpdQpSc,
            naming::UpdToRdIdsSc,
            naming::UpdUcc,
            naming::VSc,
            naming::VUcc,
            naming::ValueSc,
            naming::WhUcc,
        );
        let ts = match &self {
            Self::Eq => quote::quote! {Eq},
            Self::IsNull => quote::quote! {IsNull},
        };
        quote::quote! {#import::#EqOprtrUcc::#ts}.into()
    }
}
//todo mb reuse with other structs
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, Clone, Copy, optml::Optml)]
pub enum Dim {
    One,
    Two,
    Three,
    Four,
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, Clone, Copy, optml::Optml)]
pub enum DimIndexNbr {
    Zero,
    One,
    Two,
    Three,
}
impl From<&Dim> for DimIndexNbr {
    fn from(v: &Dim) -> Self {
        match &v {
            Dim::One => Self::Zero,
            Dim::Two => Self::One,
            Dim::Three => Self::Two,
            Dim::Four => Self::Three,
        }
    }
}
pg_crud_macros_cmn_macros::bool_enum_to_tokens!(CrQbValueUndrscr, false => naming::VSc, true => quote::quote! {_});
pg_crud_macros_cmn_macros::bool_enum_to_tokens!(CrQpIncrUndrscr, false => naming::IncrSc, true => quote::quote! {_});
pg_crud_macros_cmn_macros::bool_enum_to_tokens!(CrQpValueUndrscr, false => naming::VSc, true => quote::quote! {_});
pg_crud_macros_cmn_macros::bool_enum_to_tokens!(SelQpValueUndrscr, false => naming::VSc, true => quote::quote! {_});
pg_crud_macros_cmn_macros::bool_enum_to_tokens!(UpdQpAccumulatorUndrscr, false => quote::quote! {upd_accumulator}, true => quote::quote! {_});
pg_crud_macros_cmn_macros::bool_enum_to_tokens!(UpdQpPathUndrscr, false => quote::quote! {upd_path}, true => quote::quote! {_});
pg_crud_macros_cmn_macros::bool_enum_to_tokens!(UpdQpTargetUndrscr, false => quote::quote! {upd_target}, true => quote::quote! {_});
pg_crud_macros_cmn_macros::bool_enum_to_tokens!(UpdQpValueUndrscr, false => naming::VSc, true => quote::quote! {_});
pub fn gen_pg_type_wh_ts(
    attrs_ts: &dyn quote::ToTokens,
    vrts: &Vec<&dyn PgFlt>,
    prefix: &dyn quote::ToTokens,
    should_derive_utoipa_to_schema: &ShouldDeriveUtoipaToSchema,
    should_derive_schemars_json_schema: &ShouldDSchemarsJsonSchema,
    is_qb_mut: &IsQbMut,
) -> macros_helpers::GeneratedRustTs {
    #[allow(non_snake_case, unused_variables)]
    let (
        AddOprtrSc,
        AllVrtsDfltSomeOneElMaxPageSizeSc,
        AllVrtsDfltSomeOneElSc,
        AllowClippyArbitrarySrcItemOrdering,
        ColSc,
        CrQbSc,
        CrQpSc,
        CrSc,
        CrTblColQpSc,
        CrUcc,
        DfltSomeOneElMaxPageSizeSc,
        DfltSomeOneElSc,
        EqOprtrUcc,
        ErSc,
        IncrSc,
        NormalizeSc,
        OptUpdSc,
        OptVecCrSc,
        PgCrudCmnDfltSomeOneElCall,
        PgTypeEqOprtrUcc,
        PgTypeNotPkUcc,
        PgTypeOptVecWhGreaterThanTestSc,
        PgTypeTestCasesUcc,
        PgTypeUcc,
        PgTypeWhFltUcc,
        PreviousRdAndOptUpdIntoRdSc,
        QbSc,
        QpErUcc,
        QpSc,
        QuerySc,
        RdIdsAndCrIntoOptVRdSc,
        RdIdsAndCrIntoOptVecWhEqToFieldSc,
        RdIdsAndCrIntoRdSc,
        RdIdsAndCrIntoTtSc,
        RdIdsAndCrIntoVecWhEqUsingFieldsSc,
        RdIdsAndCrIntoWhEqSc,
        RdIdsAndTtIntoPgTypeOptWhGreaterThanSc,
        RdIdsSc,
        RdIdsTo2DimsVecRdInnSc,
        RdIdsToOptVRdDfltSomeOneElSc,
        RdIdsUcc,
        RdInnIntoRdWithNewOrTryNewUnwrapedSc,
        RdInnIntoUpdWithNewOrTryNewUnwrapedSc,
        RdInnUcc,
        RdSc,
        RdUcc,
        SelOnlyIdsQpSc,
        SelOnlyUpddIdsQbSc,
        SelOnlyUpddIdsQpSc,
        SelQpSc,
        SelUcc,
        SelfUcc,
        TtSc,
        TtUcc,
        UpdForQueryUcc,
        UpdQbSc,
        UpdQpSc,
        UpdToRdIdsSc,
        UpdUcc,
        VSc,
        VUcc,
        ValueSc,
        WhUcc,
    ) = (
        naming::AddOprtrSc,
        naming::AllVrtsDfltSomeOneElMaxPageSizeSc,
        naming::AllVrtsDfltSomeOneElSc,
        token_patterns::AllowClippyArbitrarySrcItemOrdering,
        naming::ColSc,
        naming::CrQbSc,
        naming::CrQpSc,
        naming::CrSc,
        naming::CrTblColQpSc,
        naming::CrUcc,
        naming::DfltSomeOneElMaxPageSizeSc,
        naming::DfltSomeOneElSc,
        naming::EqOprtrUcc,
        naming::ErSc,
        naming::IncrSc,
        naming::NormalizeSc,
        naming::OptUpdSc,
        naming::OptVecCrSc,
        token_patterns::PgCrudCmnDfltSomeOneElCall,
        naming::PgTypeEqOprtrUcc,
        naming::PgTypeNotPkUcc,
        naming::PgTypeOptVecWhGreaterThanTestSc,
        naming::PgTypeTestCasesUcc,
        naming::PgTypeUcc,
        naming::PgTypeWhFltUcc,
        naming::PreviousRdAndOptUpdIntoRdSc,
        naming::QbSc,
        naming::QpErUcc,
        naming::QpSc,
        naming::QuerySc,
        naming::RdIdsAndCrIntoOptVRdSc,
        naming::RdIdsAndCrIntoOptVecWhEqToFieldSc,
        naming::RdIdsAndCrIntoRdSc,
        naming::RdIdsAndCrIntoTtSc,
        naming::RdIdsAndCrIntoVecWhEqUsingFieldsSc,
        naming::RdIdsAndCrIntoWhEqSc,
        naming::RdIdsAndTtIntoPgTypeOptWhGreaterThanSc,
        naming::RdIdsSc,
        naming::RdIdsTo2DimsVecRdInnSc,
        naming::RdIdsToOptVRdDfltSomeOneElSc,
        naming::RdIdsUcc,
        naming::RdInnIntoRdWithNewOrTryNewUnwrapedSc,
        naming::RdInnIntoUpdWithNewOrTryNewUnwrapedSc,
        naming::RdInnUcc,
        naming::RdSc,
        naming::RdUcc,
        naming::SelOnlyIdsQpSc,
        naming::SelOnlyUpddIdsQbSc,
        naming::SelOnlyUpddIdsQpSc,
        naming::SelQpSc,
        naming::SelUcc,
        naming::SelfUcc,
        naming::TtSc,
        naming::TtUcc,
        naming::UpdForQueryUcc,
        naming::UpdQbSc,
        naming::UpdQpSc,
        naming::UpdToRdIdsSc,
        naming::UpdUcc,
        naming::VSc,
        naming::VUcc,
        naming::ValueSc,
        naming::WhUcc,
    );
    let ident = naming::prm::SelfWhUcc::from_tokens(&prefix);
    let pg_type_tokens_wh_ts = {
        let vrts_ts = vrts.iter().map(|el| {
            let el_ucc = el.ucc();
            let prefix_wh_self_ucc = el.prefix_wh_self_ucc();
            let opt_type_ts: Option<macros_helpers::GeneratedRustTs> = el.mb_generic();
            let type_ts =
                opt_type_ts.map_or_else(proc_macro2::TokenStream::new, |v| quote::quote! {<#v>});
            quote::quote! {#el_ucc(wh_flts::#prefix_wh_self_ucc #type_ts)}
        });
        quote::quote! {
            #attrs_ts
            #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize #should_derive_utoipa_to_schema #should_derive_schemars_json_schema, optml::Optml)]
            pub enum #ident {
                #(#vrts_ts),*
            }
        }
    };
    let impl_pg_type_pg_type_wh_flt_for_pg_type_tokens_wh_ts = impl_pg_type_wh_flt_for_ident_ts(
        &quote::quote! {<'lt>},
        &ident,
        &proc_macro2::TokenStream::new(),
        &IncrPrmUndrscr::False,
        &ColPrmUndrscr::False,
        &AddOprtrUndrscr::False,
        &{
            let vrts_ts = vrts.iter().map(|el| {
                let el_ucc = el.ucc();
                quote::quote! {
                    Self::#el_ucc(#VSc) => pg_crud_cmn::PgTypeWhFlt::qp(
                        #VSc,
                        #IncrSc,
                        #ColSc,
                        #AddOprtrSc,
                    )
                }
            });
            quote::quote! {
                match &self {
                    #(#vrts_ts),*
                }
            }
        },
        is_qb_mut,
        &{
            let vrts_ts = vrts.iter().map(|el| {
                let el_ucc = el.ucc();
                quote::quote! {
                    Self::#el_ucc(#VSc) => pg_crud_cmn::PgTypeWhFlt::qb(
                        #VSc,
                        #QuerySc
                    )
                }
            });
            quote::quote! {
                match self {
                    #(#vrts_ts),*
                }
            }
        },
        &Import::PgCrudCmn,
    );
    let impl_loc_lib_to_err_string_for_pg_type_tokens_wh_ts =
        gen_impl_to_err_string_no_generics_ts(&ident, &quote::quote! {format!("{self:#?}")});
    let impl_all_vrts_dflt_some_one_el_for_pg_type_tokens_wh_ts =
        gen_impl_pg_crud_cmn_all_vrts_dflt_some_one_el_ts(&ident, &{
            let vrts_ts = vrts.iter().map(|el| {
                let el_ucc = el.ucc();
                quote::quote! {Self::#el_ucc(#PgCrudCmnDfltSomeOneElCall)}
            });
            quote::quote! {vec![#(#vrts_ts),*]}
        });
    quote::quote! {
        #pg_type_tokens_wh_ts
        #impl_pg_type_pg_type_wh_flt_for_pg_type_tokens_wh_ts
        #impl_loc_lib_to_err_string_for_pg_type_tokens_wh_ts
        #impl_all_vrts_dflt_some_one_el_for_pg_type_tokens_wh_ts
    }
    .into()
}
pub fn gen_impl_to_err_string_no_generics_ts(
    ident: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> macros_helpers::GeneratedRustTs {
    macros_helpers::gen_impl_to_err_string_ts(
        &proc_macro2::TokenStream::new(),
        ident,
        &proc_macro2::TokenStream::new(),
        ts,
    )
}
pub fn gen_impl_display_and_to_err_string_debug_ts(
    ident: &dyn quote::ToTokens,
) -> macros_helpers::GeneratedRustTs {
    let impl_display_ts = macros_helpers::gen_impl_display_ts(
        &proc_macro2::TokenStream::new(),
        ident,
        &proc_macro2::TokenStream::new(),
        &quote::quote! {write!(f, "{self:?}")},
    );
    let impl_to_err_string_ts =
        gen_impl_to_err_string_no_generics_ts(ident, &quote::quote! {format!("{self:#?}")});
    quote::quote! {
        #impl_display_ts
        #impl_to_err_string_ts
    }
    .into()
}
#[must_use]
pub fn pg_crud_cmn_qp_er_ts() -> macros_helpers::GeneratedRustTs {
    #[allow(non_snake_case, unused_variables)]
    let (
        AddOprtrSc,
        AllVrtsDfltSomeOneElMaxPageSizeSc,
        AllVrtsDfltSomeOneElSc,
        AllowClippyArbitrarySrcItemOrdering,
        ColSc,
        CrQbSc,
        CrQpSc,
        CrSc,
        CrTblColQpSc,
        CrUcc,
        DfltSomeOneElMaxPageSizeSc,
        DfltSomeOneElSc,
        EqOprtrUcc,
        ErSc,
        IncrSc,
        NormalizeSc,
        OptUpdSc,
        OptVecCrSc,
        PgCrudCmnDfltSomeOneElCall,
        PgTypeEqOprtrUcc,
        PgTypeNotPkUcc,
        PgTypeOptVecWhGreaterThanTestSc,
        PgTypeTestCasesUcc,
        PgTypeUcc,
        PgTypeWhFltUcc,
        PreviousRdAndOptUpdIntoRdSc,
        QbSc,
        QpErUcc,
        QpSc,
        QuerySc,
        RdIdsAndCrIntoOptVRdSc,
        RdIdsAndCrIntoOptVecWhEqToFieldSc,
        RdIdsAndCrIntoRdSc,
        RdIdsAndCrIntoTtSc,
        RdIdsAndCrIntoVecWhEqUsingFieldsSc,
        RdIdsAndCrIntoWhEqSc,
        RdIdsAndTtIntoPgTypeOptWhGreaterThanSc,
        RdIdsSc,
        RdIdsTo2DimsVecRdInnSc,
        RdIdsToOptVRdDfltSomeOneElSc,
        RdIdsUcc,
        RdInnIntoRdWithNewOrTryNewUnwrapedSc,
        RdInnIntoUpdWithNewOrTryNewUnwrapedSc,
        RdInnUcc,
        RdSc,
        RdUcc,
        SelOnlyIdsQpSc,
        SelOnlyUpddIdsQbSc,
        SelOnlyUpddIdsQpSc,
        SelQpSc,
        SelUcc,
        SelfUcc,
        TtSc,
        TtUcc,
        UpdForQueryUcc,
        UpdQbSc,
        UpdQpSc,
        UpdToRdIdsSc,
        UpdUcc,
        VSc,
        VUcc,
        ValueSc,
        WhUcc,
    ) = (
        naming::AddOprtrSc,
        naming::AllVrtsDfltSomeOneElMaxPageSizeSc,
        naming::AllVrtsDfltSomeOneElSc,
        token_patterns::AllowClippyArbitrarySrcItemOrdering,
        naming::ColSc,
        naming::CrQbSc,
        naming::CrQpSc,
        naming::CrSc,
        naming::CrTblColQpSc,
        naming::CrUcc,
        naming::DfltSomeOneElMaxPageSizeSc,
        naming::DfltSomeOneElSc,
        naming::EqOprtrUcc,
        naming::ErSc,
        naming::IncrSc,
        naming::NormalizeSc,
        naming::OptUpdSc,
        naming::OptVecCrSc,
        token_patterns::PgCrudCmnDfltSomeOneElCall,
        naming::PgTypeEqOprtrUcc,
        naming::PgTypeNotPkUcc,
        naming::PgTypeOptVecWhGreaterThanTestSc,
        naming::PgTypeTestCasesUcc,
        naming::PgTypeUcc,
        naming::PgTypeWhFltUcc,
        naming::PreviousRdAndOptUpdIntoRdSc,
        naming::QbSc,
        naming::QpErUcc,
        naming::QpSc,
        naming::QuerySc,
        naming::RdIdsAndCrIntoOptVRdSc,
        naming::RdIdsAndCrIntoOptVecWhEqToFieldSc,
        naming::RdIdsAndCrIntoRdSc,
        naming::RdIdsAndCrIntoTtSc,
        naming::RdIdsAndCrIntoVecWhEqUsingFieldsSc,
        naming::RdIdsAndCrIntoWhEqSc,
        naming::RdIdsAndTtIntoPgTypeOptWhGreaterThanSc,
        naming::RdIdsSc,
        naming::RdIdsTo2DimsVecRdInnSc,
        naming::RdIdsToOptVRdDfltSomeOneElSc,
        naming::RdIdsUcc,
        naming::RdInnIntoRdWithNewOrTryNewUnwrapedSc,
        naming::RdInnIntoUpdWithNewOrTryNewUnwrapedSc,
        naming::RdInnUcc,
        naming::RdSc,
        naming::RdUcc,
        naming::SelOnlyIdsQpSc,
        naming::SelOnlyUpddIdsQbSc,
        naming::SelOnlyUpddIdsQpSc,
        naming::SelQpSc,
        naming::SelUcc,
        naming::SelfUcc,
        naming::TtSc,
        naming::TtUcc,
        naming::UpdForQueryUcc,
        naming::UpdQbSc,
        naming::UpdQpSc,
        naming::UpdToRdIdsSc,
        naming::UpdUcc,
        naming::VSc,
        naming::VUcc,
        naming::ValueSc,
        naming::WhUcc,
    );
    quote::quote! {pg_crud_cmn::#QpErUcc}.into()
}
#[must_use]
pub fn gen_dim_nbr_pgn_ts(dim_nbr: DimNbr) -> macros_helpers::GeneratedRustTs {
    parse_ts_or_compile_error(
        ParseTsTextRef::from(format!("dim{}_pgn", dim_nbr.get()).as_str()),
        ParseErIdRef::from("7c3a91b2"),
    )
}
pub fn gen_struct_ident_dq_ts(v: &dyn std::fmt::Display) -> gen_quotes::QuotedLiteralTs {
    gen_quotes::dq_ts(&format!("struct {v}"))
}
pub fn gen_struct_ident_with_nbr_els_dq_ts(
    ident: &dyn naming::DisplayPlusToTokens,
    len: StructElsLen,
) -> gen_quotes::QuotedLiteralTs {
    gen_quotes::dq_ts(&format!("struct {ident} with {} els", len.get()))
}
pub fn gen_sqlx_types_json_type_dcl_ts(
    type_ts: &dyn quote::ToTokens,
) -> macros_helpers::GeneratedRustTs {
    quote::quote! {sqlx::types::Json<#type_ts>}.into()
}
pub fn gen_opt_type_dcl_ts(type_ts: &dyn quote::ToTokens) -> macros_helpers::GeneratedRustTs {
    quote::quote! {Option<#type_ts>}.into()
}
pub fn gen_vec_tokens_dcl_ts(type_ts: &dyn quote::ToTokens) -> macros_helpers::GeneratedRustTs {
    quote::quote! {Vec<#type_ts>}.into()
}
pub fn gen_de_dq_ts(
    ident: &dyn naming::DisplayPlusToTokens,
    len: DeLen,
) -> (
    gen_quotes::QuotedLiteralTs,
    gen_quotes::QuotedLiteralTs,
    gen_quotes::QuotedLiteralTs,
) {
    let struct_pg_type_ident_wh_tokens_dq_ts = gen_struct_ident_dq_ts(ident);
    let struct_pg_type_ident_wh_tokens_with_nbr_els_dq_ts =
        gen_struct_ident_with_nbr_els_dq_ts(ident, StructElsLen::from(len.get()));
    let pg_type_ident_wh_tokens_dq_ts = gen_quotes::dq_ts(&ident);
    (
        struct_pg_type_ident_wh_tokens_dq_ts,
        struct_pg_type_ident_wh_tokens_with_nbr_els_dq_ts,
        pg_type_ident_wh_tokens_dq_ts,
    )
}
pub fn gen_impl_dflt_some_one_el_ts(
    impl_generic_ts: &dyn quote::ToTokens,
    import: &Import,
    ident: &dyn quote::ToTokens,
    ident_generic_ts: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> macros_helpers::GeneratedRustTs {
    #[allow(non_snake_case, unused_variables)]
    let (
        AddOprtrSc,
        AllVrtsDfltSomeOneElMaxPageSizeSc,
        AllVrtsDfltSomeOneElSc,
        AllowClippyArbitrarySrcItemOrdering,
        ColSc,
        CrQbSc,
        CrQpSc,
        CrSc,
        CrTblColQpSc,
        CrUcc,
        DfltSomeOneElMaxPageSizeSc,
        DfltSomeOneElSc,
        EqOprtrUcc,
        ErSc,
        IncrSc,
        NormalizeSc,
        OptUpdSc,
        OptVecCrSc,
        PgCrudCmnDfltSomeOneElCall,
        PgTypeEqOprtrUcc,
        PgTypeNotPkUcc,
        PgTypeOptVecWhGreaterThanTestSc,
        PgTypeTestCasesUcc,
        PgTypeUcc,
        PgTypeWhFltUcc,
        PreviousRdAndOptUpdIntoRdSc,
        QbSc,
        QpErUcc,
        QpSc,
        QuerySc,
        RdIdsAndCrIntoOptVRdSc,
        RdIdsAndCrIntoOptVecWhEqToFieldSc,
        RdIdsAndCrIntoRdSc,
        RdIdsAndCrIntoTtSc,
        RdIdsAndCrIntoVecWhEqUsingFieldsSc,
        RdIdsAndCrIntoWhEqSc,
        RdIdsAndTtIntoPgTypeOptWhGreaterThanSc,
        RdIdsSc,
        RdIdsTo2DimsVecRdInnSc,
        RdIdsToOptVRdDfltSomeOneElSc,
        RdIdsUcc,
        RdInnIntoRdWithNewOrTryNewUnwrapedSc,
        RdInnIntoUpdWithNewOrTryNewUnwrapedSc,
        RdInnUcc,
        RdSc,
        RdUcc,
        SelOnlyIdsQpSc,
        SelOnlyUpddIdsQbSc,
        SelOnlyUpddIdsQpSc,
        SelQpSc,
        SelUcc,
        SelfUcc,
        TtSc,
        TtUcc,
        UpdForQueryUcc,
        UpdQbSc,
        UpdQpSc,
        UpdToRdIdsSc,
        UpdUcc,
        VSc,
        VUcc,
        ValueSc,
        WhUcc,
    ) = (
        naming::AddOprtrSc,
        naming::AllVrtsDfltSomeOneElMaxPageSizeSc,
        naming::AllVrtsDfltSomeOneElSc,
        token_patterns::AllowClippyArbitrarySrcItemOrdering,
        naming::ColSc,
        naming::CrQbSc,
        naming::CrQpSc,
        naming::CrSc,
        naming::CrTblColQpSc,
        naming::CrUcc,
        naming::DfltSomeOneElMaxPageSizeSc,
        naming::DfltSomeOneElSc,
        naming::EqOprtrUcc,
        naming::ErSc,
        naming::IncrSc,
        naming::NormalizeSc,
        naming::OptUpdSc,
        naming::OptVecCrSc,
        token_patterns::PgCrudCmnDfltSomeOneElCall,
        naming::PgTypeEqOprtrUcc,
        naming::PgTypeNotPkUcc,
        naming::PgTypeOptVecWhGreaterThanTestSc,
        naming::PgTypeTestCasesUcc,
        naming::PgTypeUcc,
        naming::PgTypeWhFltUcc,
        naming::PreviousRdAndOptUpdIntoRdSc,
        naming::QbSc,
        naming::QpErUcc,
        naming::QpSc,
        naming::QuerySc,
        naming::RdIdsAndCrIntoOptVRdSc,
        naming::RdIdsAndCrIntoOptVecWhEqToFieldSc,
        naming::RdIdsAndCrIntoRdSc,
        naming::RdIdsAndCrIntoTtSc,
        naming::RdIdsAndCrIntoVecWhEqUsingFieldsSc,
        naming::RdIdsAndCrIntoWhEqSc,
        naming::RdIdsAndTtIntoPgTypeOptWhGreaterThanSc,
        naming::RdIdsSc,
        naming::RdIdsTo2DimsVecRdInnSc,
        naming::RdIdsToOptVRdDfltSomeOneElSc,
        naming::RdIdsUcc,
        naming::RdInnIntoRdWithNewOrTryNewUnwrapedSc,
        naming::RdInnIntoUpdWithNewOrTryNewUnwrapedSc,
        naming::RdInnUcc,
        naming::RdSc,
        naming::RdUcc,
        naming::SelOnlyIdsQpSc,
        naming::SelOnlyUpddIdsQbSc,
        naming::SelOnlyUpddIdsQpSc,
        naming::SelQpSc,
        naming::SelUcc,
        naming::SelfUcc,
        naming::TtSc,
        naming::TtUcc,
        naming::UpdForQueryUcc,
        naming::UpdQbSc,
        naming::UpdQpSc,
        naming::UpdToRdIdsSc,
        naming::UpdUcc,
        naming::VSc,
        naming::VUcc,
        naming::ValueSc,
        naming::WhUcc,
    );
    let path_trait_ts = import.dflt_some_one_el();
    quote::quote! {
        impl #impl_generic_ts #path_trait_ts for #ident #ident_generic_ts {
            fn #DfltSomeOneElSc() -> Self {
                #ts
            }
        }
    }
    .into()
}
pub fn gen_impl_all_vrts_dflt_some_one_el_ts(
    import: &Import,
    ident: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> macros_helpers::GeneratedRustTs {
    #[allow(non_snake_case, unused_variables)]
    let (
        AddOprtrSc,
        AllVrtsDfltSomeOneElMaxPageSizeSc,
        AllVrtsDfltSomeOneElSc,
        AllowClippyArbitrarySrcItemOrdering,
        ColSc,
        CrQbSc,
        CrQpSc,
        CrSc,
        CrTblColQpSc,
        CrUcc,
        DfltSomeOneElMaxPageSizeSc,
        DfltSomeOneElSc,
        EqOprtrUcc,
        ErSc,
        IncrSc,
        NormalizeSc,
        OptUpdSc,
        OptVecCrSc,
        PgCrudCmnDfltSomeOneElCall,
        PgTypeEqOprtrUcc,
        PgTypeNotPkUcc,
        PgTypeOptVecWhGreaterThanTestSc,
        PgTypeTestCasesUcc,
        PgTypeUcc,
        PgTypeWhFltUcc,
        PreviousRdAndOptUpdIntoRdSc,
        QbSc,
        QpErUcc,
        QpSc,
        QuerySc,
        RdIdsAndCrIntoOptVRdSc,
        RdIdsAndCrIntoOptVecWhEqToFieldSc,
        RdIdsAndCrIntoRdSc,
        RdIdsAndCrIntoTtSc,
        RdIdsAndCrIntoVecWhEqUsingFieldsSc,
        RdIdsAndCrIntoWhEqSc,
        RdIdsAndTtIntoPgTypeOptWhGreaterThanSc,
        RdIdsSc,
        RdIdsTo2DimsVecRdInnSc,
        RdIdsToOptVRdDfltSomeOneElSc,
        RdIdsUcc,
        RdInnIntoRdWithNewOrTryNewUnwrapedSc,
        RdInnIntoUpdWithNewOrTryNewUnwrapedSc,
        RdInnUcc,
        RdSc,
        RdUcc,
        SelOnlyIdsQpSc,
        SelOnlyUpddIdsQbSc,
        SelOnlyUpddIdsQpSc,
        SelQpSc,
        SelUcc,
        SelfUcc,
        TtSc,
        TtUcc,
        UpdForQueryUcc,
        UpdQbSc,
        UpdQpSc,
        UpdToRdIdsSc,
        UpdUcc,
        VSc,
        VUcc,
        ValueSc,
        WhUcc,
    ) = (
        naming::AddOprtrSc,
        naming::AllVrtsDfltSomeOneElMaxPageSizeSc,
        naming::AllVrtsDfltSomeOneElSc,
        token_patterns::AllowClippyArbitrarySrcItemOrdering,
        naming::ColSc,
        naming::CrQbSc,
        naming::CrQpSc,
        naming::CrSc,
        naming::CrTblColQpSc,
        naming::CrUcc,
        naming::DfltSomeOneElMaxPageSizeSc,
        naming::DfltSomeOneElSc,
        naming::EqOprtrUcc,
        naming::ErSc,
        naming::IncrSc,
        naming::NormalizeSc,
        naming::OptUpdSc,
        naming::OptVecCrSc,
        token_patterns::PgCrudCmnDfltSomeOneElCall,
        naming::PgTypeEqOprtrUcc,
        naming::PgTypeNotPkUcc,
        naming::PgTypeOptVecWhGreaterThanTestSc,
        naming::PgTypeTestCasesUcc,
        naming::PgTypeUcc,
        naming::PgTypeWhFltUcc,
        naming::PreviousRdAndOptUpdIntoRdSc,
        naming::QbSc,
        naming::QpErUcc,
        naming::QpSc,
        naming::QuerySc,
        naming::RdIdsAndCrIntoOptVRdSc,
        naming::RdIdsAndCrIntoOptVecWhEqToFieldSc,
        naming::RdIdsAndCrIntoRdSc,
        naming::RdIdsAndCrIntoTtSc,
        naming::RdIdsAndCrIntoVecWhEqUsingFieldsSc,
        naming::RdIdsAndCrIntoWhEqSc,
        naming::RdIdsAndTtIntoPgTypeOptWhGreaterThanSc,
        naming::RdIdsSc,
        naming::RdIdsTo2DimsVecRdInnSc,
        naming::RdIdsToOptVRdDfltSomeOneElSc,
        naming::RdIdsUcc,
        naming::RdInnIntoRdWithNewOrTryNewUnwrapedSc,
        naming::RdInnIntoUpdWithNewOrTryNewUnwrapedSc,
        naming::RdInnUcc,
        naming::RdSc,
        naming::RdUcc,
        naming::SelOnlyIdsQpSc,
        naming::SelOnlyUpddIdsQbSc,
        naming::SelOnlyUpddIdsQpSc,
        naming::SelQpSc,
        naming::SelUcc,
        naming::SelfUcc,
        naming::TtSc,
        naming::TtUcc,
        naming::UpdForQueryUcc,
        naming::UpdQbSc,
        naming::UpdQpSc,
        naming::UpdToRdIdsSc,
        naming::UpdUcc,
        naming::VSc,
        naming::VUcc,
        naming::ValueSc,
        naming::WhUcc,
    );
    let path_trait_ts = import.all_vrts_dflt_some_one_el();
    quote::quote! {
        impl #path_trait_ts for #ident {
            fn #AllVrtsDfltSomeOneElSc() -> Vec<Self> {
                #ts
            }
        }
    }
    .into()
}
pub fn gen_impl_dflt_some_one_el_max_page_size_ts(
    impl_generic_ts: &dyn quote::ToTokens,
    import: &Import,
    ident: &dyn quote::ToTokens,
    ident_generic_ts: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> macros_helpers::GeneratedRustTs {
    #[allow(non_snake_case, unused_variables)]
    let (
        AddOprtrSc,
        AllVrtsDfltSomeOneElMaxPageSizeSc,
        AllVrtsDfltSomeOneElSc,
        AllowClippyArbitrarySrcItemOrdering,
        ColSc,
        CrQbSc,
        CrQpSc,
        CrSc,
        CrTblColQpSc,
        CrUcc,
        DfltSomeOneElMaxPageSizeSc,
        DfltSomeOneElSc,
        EqOprtrUcc,
        ErSc,
        IncrSc,
        NormalizeSc,
        OptUpdSc,
        OptVecCrSc,
        PgCrudCmnDfltSomeOneElCall,
        PgTypeEqOprtrUcc,
        PgTypeNotPkUcc,
        PgTypeOptVecWhGreaterThanTestSc,
        PgTypeTestCasesUcc,
        PgTypeUcc,
        PgTypeWhFltUcc,
        PreviousRdAndOptUpdIntoRdSc,
        QbSc,
        QpErUcc,
        QpSc,
        QuerySc,
        RdIdsAndCrIntoOptVRdSc,
        RdIdsAndCrIntoOptVecWhEqToFieldSc,
        RdIdsAndCrIntoRdSc,
        RdIdsAndCrIntoTtSc,
        RdIdsAndCrIntoVecWhEqUsingFieldsSc,
        RdIdsAndCrIntoWhEqSc,
        RdIdsAndTtIntoPgTypeOptWhGreaterThanSc,
        RdIdsSc,
        RdIdsTo2DimsVecRdInnSc,
        RdIdsToOptVRdDfltSomeOneElSc,
        RdIdsUcc,
        RdInnIntoRdWithNewOrTryNewUnwrapedSc,
        RdInnIntoUpdWithNewOrTryNewUnwrapedSc,
        RdInnUcc,
        RdSc,
        RdUcc,
        SelOnlyIdsQpSc,
        SelOnlyUpddIdsQbSc,
        SelOnlyUpddIdsQpSc,
        SelQpSc,
        SelUcc,
        SelfUcc,
        TtSc,
        TtUcc,
        UpdForQueryUcc,
        UpdQbSc,
        UpdQpSc,
        UpdToRdIdsSc,
        UpdUcc,
        VSc,
        VUcc,
        ValueSc,
        WhUcc,
    ) = (
        naming::AddOprtrSc,
        naming::AllVrtsDfltSomeOneElMaxPageSizeSc,
        naming::AllVrtsDfltSomeOneElSc,
        token_patterns::AllowClippyArbitrarySrcItemOrdering,
        naming::ColSc,
        naming::CrQbSc,
        naming::CrQpSc,
        naming::CrSc,
        naming::CrTblColQpSc,
        naming::CrUcc,
        naming::DfltSomeOneElMaxPageSizeSc,
        naming::DfltSomeOneElSc,
        naming::EqOprtrUcc,
        naming::ErSc,
        naming::IncrSc,
        naming::NormalizeSc,
        naming::OptUpdSc,
        naming::OptVecCrSc,
        token_patterns::PgCrudCmnDfltSomeOneElCall,
        naming::PgTypeEqOprtrUcc,
        naming::PgTypeNotPkUcc,
        naming::PgTypeOptVecWhGreaterThanTestSc,
        naming::PgTypeTestCasesUcc,
        naming::PgTypeUcc,
        naming::PgTypeWhFltUcc,
        naming::PreviousRdAndOptUpdIntoRdSc,
        naming::QbSc,
        naming::QpErUcc,
        naming::QpSc,
        naming::QuerySc,
        naming::RdIdsAndCrIntoOptVRdSc,
        naming::RdIdsAndCrIntoOptVecWhEqToFieldSc,
        naming::RdIdsAndCrIntoRdSc,
        naming::RdIdsAndCrIntoTtSc,
        naming::RdIdsAndCrIntoVecWhEqUsingFieldsSc,
        naming::RdIdsAndCrIntoWhEqSc,
        naming::RdIdsAndTtIntoPgTypeOptWhGreaterThanSc,
        naming::RdIdsSc,
        naming::RdIdsTo2DimsVecRdInnSc,
        naming::RdIdsToOptVRdDfltSomeOneElSc,
        naming::RdIdsUcc,
        naming::RdInnIntoRdWithNewOrTryNewUnwrapedSc,
        naming::RdInnIntoUpdWithNewOrTryNewUnwrapedSc,
        naming::RdInnUcc,
        naming::RdSc,
        naming::RdUcc,
        naming::SelOnlyIdsQpSc,
        naming::SelOnlyUpddIdsQbSc,
        naming::SelOnlyUpddIdsQpSc,
        naming::SelQpSc,
        naming::SelUcc,
        naming::SelfUcc,
        naming::TtSc,
        naming::TtUcc,
        naming::UpdForQueryUcc,
        naming::UpdQbSc,
        naming::UpdQpSc,
        naming::UpdToRdIdsSc,
        naming::UpdUcc,
        naming::VSc,
        naming::VUcc,
        naming::ValueSc,
        naming::WhUcc,
    );
    let path_trait_ts = import.dflt_some_one_el_max_page_size();
    quote::quote! {
        impl #impl_generic_ts #path_trait_ts for #ident #ident_generic_ts {
            fn #DfltSomeOneElMaxPageSizeSc() -> Self {
                #ts
            }
        }
    }
    .into()
}
pub fn gen_impl_all_vrts_dflt_some_one_el_max_page_size_ts(
    import: &Import,
    ident: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> macros_helpers::GeneratedRustTs {
    #[allow(non_snake_case, unused_variables)]
    let (
        AddOprtrSc,
        AllVrtsDfltSomeOneElMaxPageSizeSc,
        AllVrtsDfltSomeOneElSc,
        AllowClippyArbitrarySrcItemOrdering,
        ColSc,
        CrQbSc,
        CrQpSc,
        CrSc,
        CrTblColQpSc,
        CrUcc,
        DfltSomeOneElMaxPageSizeSc,
        DfltSomeOneElSc,
        EqOprtrUcc,
        ErSc,
        IncrSc,
        NormalizeSc,
        OptUpdSc,
        OptVecCrSc,
        PgCrudCmnDfltSomeOneElCall,
        PgTypeEqOprtrUcc,
        PgTypeNotPkUcc,
        PgTypeOptVecWhGreaterThanTestSc,
        PgTypeTestCasesUcc,
        PgTypeUcc,
        PgTypeWhFltUcc,
        PreviousRdAndOptUpdIntoRdSc,
        QbSc,
        QpErUcc,
        QpSc,
        QuerySc,
        RdIdsAndCrIntoOptVRdSc,
        RdIdsAndCrIntoOptVecWhEqToFieldSc,
        RdIdsAndCrIntoRdSc,
        RdIdsAndCrIntoTtSc,
        RdIdsAndCrIntoVecWhEqUsingFieldsSc,
        RdIdsAndCrIntoWhEqSc,
        RdIdsAndTtIntoPgTypeOptWhGreaterThanSc,
        RdIdsSc,
        RdIdsTo2DimsVecRdInnSc,
        RdIdsToOptVRdDfltSomeOneElSc,
        RdIdsUcc,
        RdInnIntoRdWithNewOrTryNewUnwrapedSc,
        RdInnIntoUpdWithNewOrTryNewUnwrapedSc,
        RdInnUcc,
        RdSc,
        RdUcc,
        SelOnlyIdsQpSc,
        SelOnlyUpddIdsQbSc,
        SelOnlyUpddIdsQpSc,
        SelQpSc,
        SelUcc,
        SelfUcc,
        TtSc,
        TtUcc,
        UpdForQueryUcc,
        UpdQbSc,
        UpdQpSc,
        UpdToRdIdsSc,
        UpdUcc,
        VSc,
        VUcc,
        ValueSc,
        WhUcc,
    ) = (
        naming::AddOprtrSc,
        naming::AllVrtsDfltSomeOneElMaxPageSizeSc,
        naming::AllVrtsDfltSomeOneElSc,
        token_patterns::AllowClippyArbitrarySrcItemOrdering,
        naming::ColSc,
        naming::CrQbSc,
        naming::CrQpSc,
        naming::CrSc,
        naming::CrTblColQpSc,
        naming::CrUcc,
        naming::DfltSomeOneElMaxPageSizeSc,
        naming::DfltSomeOneElSc,
        naming::EqOprtrUcc,
        naming::ErSc,
        naming::IncrSc,
        naming::NormalizeSc,
        naming::OptUpdSc,
        naming::OptVecCrSc,
        token_patterns::PgCrudCmnDfltSomeOneElCall,
        naming::PgTypeEqOprtrUcc,
        naming::PgTypeNotPkUcc,
        naming::PgTypeOptVecWhGreaterThanTestSc,
        naming::PgTypeTestCasesUcc,
        naming::PgTypeUcc,
        naming::PgTypeWhFltUcc,
        naming::PreviousRdAndOptUpdIntoRdSc,
        naming::QbSc,
        naming::QpErUcc,
        naming::QpSc,
        naming::QuerySc,
        naming::RdIdsAndCrIntoOptVRdSc,
        naming::RdIdsAndCrIntoOptVecWhEqToFieldSc,
        naming::RdIdsAndCrIntoRdSc,
        naming::RdIdsAndCrIntoTtSc,
        naming::RdIdsAndCrIntoVecWhEqUsingFieldsSc,
        naming::RdIdsAndCrIntoWhEqSc,
        naming::RdIdsAndTtIntoPgTypeOptWhGreaterThanSc,
        naming::RdIdsSc,
        naming::RdIdsTo2DimsVecRdInnSc,
        naming::RdIdsToOptVRdDfltSomeOneElSc,
        naming::RdIdsUcc,
        naming::RdInnIntoRdWithNewOrTryNewUnwrapedSc,
        naming::RdInnIntoUpdWithNewOrTryNewUnwrapedSc,
        naming::RdInnUcc,
        naming::RdSc,
        naming::RdUcc,
        naming::SelOnlyIdsQpSc,
        naming::SelOnlyUpddIdsQbSc,
        naming::SelOnlyUpddIdsQpSc,
        naming::SelQpSc,
        naming::SelUcc,
        naming::SelfUcc,
        naming::TtSc,
        naming::TtUcc,
        naming::UpdForQueryUcc,
        naming::UpdQbSc,
        naming::UpdQpSc,
        naming::UpdToRdIdsSc,
        naming::UpdUcc,
        naming::VSc,
        naming::VUcc,
        naming::ValueSc,
        naming::WhUcc,
    );
    let path_trait_ts = import.all_vrts_dflt_some_one_el_max_page_size();
    let all_vrts_dflt_some_one_el_max_page_size_sc = AllVrtsDfltSomeOneElMaxPageSizeSc;
    quote::quote! {
        impl #path_trait_ts for #ident {
            fn #all_vrts_dflt_some_one_el_max_page_size_sc() -> Vec<Self> {
                #ts
            }
        }
    }
    .into()
}
pub fn gen_impl_pg_crud_cmn_dflt_some_one_el_ts(
    ident: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> macros_helpers::GeneratedRustTs {
    gen_impl_dflt_some_one_el_ts(
        &proc_macro2::TokenStream::new(),
        &Import::PgCrudCmn,
        ident,
        &proc_macro2::TokenStream::new(),
        ts,
    )
}
pub fn gen_impl_pg_crud_dflt_some_one_el_ts(
    ident: &dyn quote::ToTokens,
    lt_ts: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> macros_helpers::GeneratedRustTs {
    gen_impl_dflt_some_one_el_ts(
        &proc_macro2::TokenStream::new(),
        &Import::PgCrud,
        ident,
        lt_ts,
        ts,
    )
}
pub fn gen_impl_pg_crud_cmn_all_vrts_dflt_some_one_el_ts(
    ident: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> macros_helpers::GeneratedRustTs {
    gen_impl_all_vrts_dflt_some_one_el_ts(&Import::PgCrudCmn, ident, ts)
}
pub fn gen_impl_pg_crud_all_vrts_dflt_some_one_el_ts(
    ident: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> macros_helpers::GeneratedRustTs {
    gen_impl_all_vrts_dflt_some_one_el_ts(&Import::PgCrud, ident, ts)
}
pub fn gen_impl_pg_crud_cmn_dflt_some_one_el_max_page_size_ts(
    ident: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> macros_helpers::GeneratedRustTs {
    gen_impl_dflt_some_one_el_max_page_size_ts(
        &proc_macro2::TokenStream::new(),
        &Import::PgCrudCmn,
        ident,
        &proc_macro2::TokenStream::new(),
        ts,
    )
}
pub fn gen_impl_pg_crud_dflt_some_one_el_max_page_size_ts(
    ident: &dyn quote::ToTokens,
    lt_ts: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> macros_helpers::GeneratedRustTs {
    gen_impl_dflt_some_one_el_max_page_size_ts(
        &proc_macro2::TokenStream::new(),
        &Import::PgCrud,
        ident,
        lt_ts,
        ts,
    )
}
pub fn gen_impl_pg_crud_all_vrts_dflt_some_one_el_max_page_size_ts(
    ident: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> macros_helpers::GeneratedRustTs {
    gen_impl_all_vrts_dflt_some_one_el_max_page_size_ts(&Import::PgCrud, ident, ts)
}
pub fn impl_pg_type_wh_flt_for_ident_ts(
    impl_generic_ts: &dyn quote::ToTokens,
    ident_ts: &dyn quote::ToTokens,
    ident_generic_ts: &dyn quote::ToTokens,
    incr_prm_undrscr: &IncrPrmUndrscr,
    col_prm_undrscr: &ColPrmUndrscr,
    add_oprtr_undrscr: &AddOprtrUndrscr,
    qp_ts: &dyn quote::ToTokens,
    is_qb_mut: &IsQbMut,
    qb_ts: &dyn quote::ToTokens,
    import: &Import,
) -> macros_helpers::GeneratedRustTs {
    #[allow(non_snake_case, unused_variables)]
    let (
        AddOprtrSc,
        AllVrtsDfltSomeOneElMaxPageSizeSc,
        AllVrtsDfltSomeOneElSc,
        AllowClippyArbitrarySrcItemOrdering,
        ColSc,
        CrQbSc,
        CrQpSc,
        CrSc,
        CrTblColQpSc,
        CrUcc,
        DfltSomeOneElMaxPageSizeSc,
        DfltSomeOneElSc,
        EqOprtrUcc,
        ErSc,
        IncrSc,
        NormalizeSc,
        OptUpdSc,
        OptVecCrSc,
        PgCrudCmnDfltSomeOneElCall,
        PgTypeEqOprtrUcc,
        PgTypeNotPkUcc,
        PgTypeOptVecWhGreaterThanTestSc,
        PgTypeTestCasesUcc,
        PgTypeUcc,
        PgTypeWhFltUcc,
        PreviousRdAndOptUpdIntoRdSc,
        QbSc,
        QpErUcc,
        QpSc,
        QuerySc,
        RdIdsAndCrIntoOptVRdSc,
        RdIdsAndCrIntoOptVecWhEqToFieldSc,
        RdIdsAndCrIntoRdSc,
        RdIdsAndCrIntoTtSc,
        RdIdsAndCrIntoVecWhEqUsingFieldsSc,
        RdIdsAndCrIntoWhEqSc,
        RdIdsAndTtIntoPgTypeOptWhGreaterThanSc,
        RdIdsSc,
        RdIdsTo2DimsVecRdInnSc,
        RdIdsToOptVRdDfltSomeOneElSc,
        RdIdsUcc,
        RdInnIntoRdWithNewOrTryNewUnwrapedSc,
        RdInnIntoUpdWithNewOrTryNewUnwrapedSc,
        RdInnUcc,
        RdSc,
        RdUcc,
        SelOnlyIdsQpSc,
        SelOnlyUpddIdsQbSc,
        SelOnlyUpddIdsQpSc,
        SelQpSc,
        SelUcc,
        SelfUcc,
        TtSc,
        TtUcc,
        UpdForQueryUcc,
        UpdQbSc,
        UpdQpSc,
        UpdToRdIdsSc,
        UpdUcc,
        VSc,
        VUcc,
        ValueSc,
        WhUcc,
    ) = (
        naming::AddOprtrSc,
        naming::AllVrtsDfltSomeOneElMaxPageSizeSc,
        naming::AllVrtsDfltSomeOneElSc,
        token_patterns::AllowClippyArbitrarySrcItemOrdering,
        naming::ColSc,
        naming::CrQbSc,
        naming::CrQpSc,
        naming::CrSc,
        naming::CrTblColQpSc,
        naming::CrUcc,
        naming::DfltSomeOneElMaxPageSizeSc,
        naming::DfltSomeOneElSc,
        naming::EqOprtrUcc,
        naming::ErSc,
        naming::IncrSc,
        naming::NormalizeSc,
        naming::OptUpdSc,
        naming::OptVecCrSc,
        token_patterns::PgCrudCmnDfltSomeOneElCall,
        naming::PgTypeEqOprtrUcc,
        naming::PgTypeNotPkUcc,
        naming::PgTypeOptVecWhGreaterThanTestSc,
        naming::PgTypeTestCasesUcc,
        naming::PgTypeUcc,
        naming::PgTypeWhFltUcc,
        naming::PreviousRdAndOptUpdIntoRdSc,
        naming::QbSc,
        naming::QpErUcc,
        naming::QpSc,
        naming::QuerySc,
        naming::RdIdsAndCrIntoOptVRdSc,
        naming::RdIdsAndCrIntoOptVecWhEqToFieldSc,
        naming::RdIdsAndCrIntoRdSc,
        naming::RdIdsAndCrIntoTtSc,
        naming::RdIdsAndCrIntoVecWhEqUsingFieldsSc,
        naming::RdIdsAndCrIntoWhEqSc,
        naming::RdIdsAndTtIntoPgTypeOptWhGreaterThanSc,
        naming::RdIdsSc,
        naming::RdIdsTo2DimsVecRdInnSc,
        naming::RdIdsToOptVRdDfltSomeOneElSc,
        naming::RdIdsUcc,
        naming::RdInnIntoRdWithNewOrTryNewUnwrapedSc,
        naming::RdInnIntoUpdWithNewOrTryNewUnwrapedSc,
        naming::RdInnUcc,
        naming::RdSc,
        naming::RdUcc,
        naming::SelOnlyIdsQpSc,
        naming::SelOnlyUpddIdsQbSc,
        naming::SelOnlyUpddIdsQpSc,
        naming::SelQpSc,
        naming::SelUcc,
        naming::SelfUcc,
        naming::TtSc,
        naming::TtUcc,
        naming::UpdForQueryUcc,
        naming::UpdQbSc,
        naming::UpdQpSc,
        naming::UpdToRdIdsSc,
        naming::UpdUcc,
        naming::VSc,
        naming::VUcc,
        naming::ValueSc,
        naming::WhUcc,
    );
    quote::quote! {
        #AllowClippyArbitrarySrcItemOrdering
        impl #impl_generic_ts #import ::#PgTypeWhFltUcc<'lt> for #ident_ts #ident_generic_ts {
            fn #QpSc(
                &self,
                #incr_prm_undrscr: &mut dyn #import::QpIncrMut,
                #col_prm_undrscr: #import::SqlColRef<'_>,
                #add_oprtr_undrscr: #import::AddOprtr
            ) -> Result<#import::QpFragment, #import::#QpErUcc> {
                #qp_ts
            }
            fn #QbSc(self, #is_qb_mut query: #import::PgQuery<'lt>) -> Result<
                #import::PgQuery<'lt>,
                #import::PgQueryBindEr
            > {
                #qb_ts
            }
        }
    }
    .into()
}
pub fn gen_impl_sqlx_encode_sqlx_pg_for_ident_ts(
    ident_ts: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> macros_helpers::GeneratedRustTs {
    quote::quote! {
        impl sqlx::Encode<'_, sqlx::Postgres> for #ident_ts {
            fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
                sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&#ts, buf)
            }
        }
    }.into()
}
pub fn gen_impl_sqlx_decode_sqlx_pg_for_ident_ts(
    ident_ts: &dyn quote::ToTokens,
    type_ts: &dyn quote::ToTokens,
    ok_v_match_ts: &dyn quote::ToTokens,
) -> macros_helpers::GeneratedRustTs {
    #[allow(non_snake_case, unused_variables)]
    let (
        AddOprtrSc,
        AllVrtsDfltSomeOneElMaxPageSizeSc,
        AllVrtsDfltSomeOneElSc,
        AllowClippyArbitrarySrcItemOrdering,
        ColSc,
        CrQbSc,
        CrQpSc,
        CrSc,
        CrTblColQpSc,
        CrUcc,
        DfltSomeOneElMaxPageSizeSc,
        DfltSomeOneElSc,
        EqOprtrUcc,
        ErSc,
        IncrSc,
        NormalizeSc,
        OptUpdSc,
        OptVecCrSc,
        PgCrudCmnDfltSomeOneElCall,
        PgTypeEqOprtrUcc,
        PgTypeNotPkUcc,
        PgTypeOptVecWhGreaterThanTestSc,
        PgTypeTestCasesUcc,
        PgTypeUcc,
        PgTypeWhFltUcc,
        PreviousRdAndOptUpdIntoRdSc,
        QbSc,
        QpErUcc,
        QpSc,
        QuerySc,
        RdIdsAndCrIntoOptVRdSc,
        RdIdsAndCrIntoOptVecWhEqToFieldSc,
        RdIdsAndCrIntoRdSc,
        RdIdsAndCrIntoTtSc,
        RdIdsAndCrIntoVecWhEqUsingFieldsSc,
        RdIdsAndCrIntoWhEqSc,
        RdIdsAndTtIntoPgTypeOptWhGreaterThanSc,
        RdIdsSc,
        RdIdsTo2DimsVecRdInnSc,
        RdIdsToOptVRdDfltSomeOneElSc,
        RdIdsUcc,
        RdInnIntoRdWithNewOrTryNewUnwrapedSc,
        RdInnIntoUpdWithNewOrTryNewUnwrapedSc,
        RdInnUcc,
        RdSc,
        RdUcc,
        SelOnlyIdsQpSc,
        SelOnlyUpddIdsQbSc,
        SelOnlyUpddIdsQpSc,
        SelQpSc,
        SelUcc,
        SelfUcc,
        TtSc,
        TtUcc,
        UpdForQueryUcc,
        UpdQbSc,
        UpdQpSc,
        UpdToRdIdsSc,
        UpdUcc,
        VSc,
        VUcc,
        ValueSc,
        WhUcc,
    ) = (
        naming::AddOprtrSc,
        naming::AllVrtsDfltSomeOneElMaxPageSizeSc,
        naming::AllVrtsDfltSomeOneElSc,
        token_patterns::AllowClippyArbitrarySrcItemOrdering,
        naming::ColSc,
        naming::CrQbSc,
        naming::CrQpSc,
        naming::CrSc,
        naming::CrTblColQpSc,
        naming::CrUcc,
        naming::DfltSomeOneElMaxPageSizeSc,
        naming::DfltSomeOneElSc,
        naming::EqOprtrUcc,
        naming::ErSc,
        naming::IncrSc,
        naming::NormalizeSc,
        naming::OptUpdSc,
        naming::OptVecCrSc,
        token_patterns::PgCrudCmnDfltSomeOneElCall,
        naming::PgTypeEqOprtrUcc,
        naming::PgTypeNotPkUcc,
        naming::PgTypeOptVecWhGreaterThanTestSc,
        naming::PgTypeTestCasesUcc,
        naming::PgTypeUcc,
        naming::PgTypeWhFltUcc,
        naming::PreviousRdAndOptUpdIntoRdSc,
        naming::QbSc,
        naming::QpErUcc,
        naming::QpSc,
        naming::QuerySc,
        naming::RdIdsAndCrIntoOptVRdSc,
        naming::RdIdsAndCrIntoOptVecWhEqToFieldSc,
        naming::RdIdsAndCrIntoRdSc,
        naming::RdIdsAndCrIntoTtSc,
        naming::RdIdsAndCrIntoVecWhEqUsingFieldsSc,
        naming::RdIdsAndCrIntoWhEqSc,
        naming::RdIdsAndTtIntoPgTypeOptWhGreaterThanSc,
        naming::RdIdsSc,
        naming::RdIdsTo2DimsVecRdInnSc,
        naming::RdIdsToOptVRdDfltSomeOneElSc,
        naming::RdIdsUcc,
        naming::RdInnIntoRdWithNewOrTryNewUnwrapedSc,
        naming::RdInnIntoUpdWithNewOrTryNewUnwrapedSc,
        naming::RdInnUcc,
        naming::RdSc,
        naming::RdUcc,
        naming::SelOnlyIdsQpSc,
        naming::SelOnlyUpddIdsQbSc,
        naming::SelOnlyUpddIdsQpSc,
        naming::SelQpSc,
        naming::SelUcc,
        naming::SelfUcc,
        naming::TtSc,
        naming::TtUcc,
        naming::UpdForQueryUcc,
        naming::UpdQbSc,
        naming::UpdQpSc,
        naming::UpdToRdIdsSc,
        naming::UpdUcc,
        naming::VSc,
        naming::VUcc,
        naming::ValueSc,
        naming::WhUcc,
    );
    quote::quote! {
        impl sqlx::Decode<'_, sqlx::Postgres> for #ident_ts {
            fn decode(#ValueSc: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
                match <#type_ts as sqlx::Decode<sqlx::Postgres>>::decode(#ValueSc) {
                    Ok(v) => #ok_v_match_ts,
                    Err(er) => Err(er),
                }
            }
        }
    }.into()
}
pub fn gen_impl_sqlx_type_for_ident_ts(
    ident_ts: &dyn quote::ToTokens,
    type_ts: &dyn quote::ToTokens,
) -> macros_helpers::GeneratedRustTs {
    quote::quote! {
        impl sqlx::Type<sqlx::Postgres> for #ident_ts {
            fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
                <#type_ts as sqlx::Type<sqlx::Postgres>>::compatible(ty)
            }
            fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
               <#type_ts as sqlx::Type<sqlx::Postgres>>::type_info()
            }
        }
    }
    .into()
}
pub fn gen_impl_sqlx_type_and_encode_for_ident_ts(
    ident_ts: &dyn quote::ToTokens,
    type_ts: &dyn quote::ToTokens,
    encode_ts: &dyn quote::ToTokens,
) -> macros_helpers::GeneratedRustTs {
    let impl_type_ts = gen_impl_sqlx_type_for_ident_ts(ident_ts, type_ts);
    let impl_encode_ts = gen_impl_sqlx_encode_sqlx_pg_for_ident_ts(ident_ts, encode_ts);
    quote::quote! {
        #impl_type_ts
        #impl_encode_ts
    }
    .into()
}
pub fn gen_impl_pg_type_ts(
    import: &Import,
    ident: &dyn quote::ToTokens,
    ident_tt_ucc: &dyn quote::ToTokens,
    is_pk_undrscr: &IsPkUndrscr,
    cr_tbl_col_qp_ts: &dyn quote::ToTokens,
    ident_cr_ucc: &dyn quote::ToTokens,
    cr_qp_v_undrscr: &CrQpValueUndrscr,
    cr_qp_incr_undrscr: &CrQpIncrUndrscr,
    cr_qp_ts: &dyn quote::ToTokens,
    cr_qb_v_undrscr: &CrQbValueUndrscr,
    is_cr_qb_mut: &IsCrQbMut,
    cr_qb_ts: &dyn quote::ToTokens,
    ident_sel_ucc: &dyn quote::ToTokens,
    sel_qp_v_undrscr: &SelQpValueUndrscr,
    sel_qp_ts: &dyn quote::ToTokens,
    ident_wh_ucc: &dyn quote::ToTokens,
    ident_rd_ucc: &dyn quote::ToTokens,
    normalize_ts: &dyn quote::ToTokens,
    rd_ids_ts: &dyn quote::ToTokens,
    sel_only_ids_qp_ts: &dyn quote::ToTokens,
    ident_rd_inn_ucc: &dyn quote::ToTokens,
    into_inn_ts: &dyn quote::ToTokens,
    ident_upd_ucc: &dyn quote::ToTokens,
    ident_upd_for_query_ucc: &dyn quote::ToTokens,
    upd_qp_v_undrscr: &UpdQpValueUndrscr,
    upd_qp_accumulator_undrscr: &UpdQpAccumulatorUndrscr,
    upd_qp_target_undrscr: &UpdQpTargetUndrscr,
    upd_qp_path_undrscr: &UpdQpPathUndrscr,
    upd_qp_ts: &dyn quote::ToTokens,
    is_upd_qb_mut: &IsUpdQbMut,
    upd_qb_ts: &dyn quote::ToTokens,
    sel_only_updd_ids_qp_ts: &dyn quote::ToTokens,
    is_sel_only_updd_ids_qb_mut: &IsSelOnlyUpddIdsQbMut,
    sel_only_updd_ids_qb_ts: &dyn quote::ToTokens,
) -> macros_helpers::GeneratedRustTs {
    #[allow(non_snake_case, unused_variables)]
    let (
        AddOprtrSc,
        AllVrtsDfltSomeOneElMaxPageSizeSc,
        AllVrtsDfltSomeOneElSc,
        AllowClippyArbitrarySrcItemOrdering,
        ColSc,
        CrQbSc,
        CrQpSc,
        CrSc,
        CrTblColQpSc,
        CrUcc,
        DfltSomeOneElMaxPageSizeSc,
        DfltSomeOneElSc,
        EqOprtrUcc,
        ErSc,
        IncrSc,
        NormalizeSc,
        OptUpdSc,
        OptVecCrSc,
        PgCrudCmnDfltSomeOneElCall,
        PgTypeEqOprtrUcc,
        PgTypeNotPkUcc,
        PgTypeOptVecWhGreaterThanTestSc,
        PgTypeTestCasesUcc,
        PgTypeUcc,
        PgTypeWhFltUcc,
        PreviousRdAndOptUpdIntoRdSc,
        QbSc,
        QpErUcc,
        QpSc,
        QuerySc,
        RdIdsAndCrIntoOptVRdSc,
        RdIdsAndCrIntoOptVecWhEqToFieldSc,
        RdIdsAndCrIntoRdSc,
        RdIdsAndCrIntoTtSc,
        RdIdsAndCrIntoVecWhEqUsingFieldsSc,
        RdIdsAndCrIntoWhEqSc,
        RdIdsAndTtIntoPgTypeOptWhGreaterThanSc,
        RdIdsSc,
        RdIdsTo2DimsVecRdInnSc,
        RdIdsToOptVRdDfltSomeOneElSc,
        RdIdsUcc,
        RdInnIntoRdWithNewOrTryNewUnwrapedSc,
        RdInnIntoUpdWithNewOrTryNewUnwrapedSc,
        RdInnUcc,
        RdSc,
        RdUcc,
        SelOnlyIdsQpSc,
        SelOnlyUpddIdsQbSc,
        SelOnlyUpddIdsQpSc,
        SelQpSc,
        SelUcc,
        SelfUcc,
        TtSc,
        TtUcc,
        UpdForQueryUcc,
        UpdQbSc,
        UpdQpSc,
        UpdToRdIdsSc,
        UpdUcc,
        VSc,
        VUcc,
        ValueSc,
        WhUcc,
    ) = (
        naming::AddOprtrSc,
        naming::AllVrtsDfltSomeOneElMaxPageSizeSc,
        naming::AllVrtsDfltSomeOneElSc,
        token_patterns::AllowClippyArbitrarySrcItemOrdering,
        naming::ColSc,
        naming::CrQbSc,
        naming::CrQpSc,
        naming::CrSc,
        naming::CrTblColQpSc,
        naming::CrUcc,
        naming::DfltSomeOneElMaxPageSizeSc,
        naming::DfltSomeOneElSc,
        naming::EqOprtrUcc,
        naming::ErSc,
        naming::IncrSc,
        naming::NormalizeSc,
        naming::OptUpdSc,
        naming::OptVecCrSc,
        token_patterns::PgCrudCmnDfltSomeOneElCall,
        naming::PgTypeEqOprtrUcc,
        naming::PgTypeNotPkUcc,
        naming::PgTypeOptVecWhGreaterThanTestSc,
        naming::PgTypeTestCasesUcc,
        naming::PgTypeUcc,
        naming::PgTypeWhFltUcc,
        naming::PreviousRdAndOptUpdIntoRdSc,
        naming::QbSc,
        naming::QpErUcc,
        naming::QpSc,
        naming::QuerySc,
        naming::RdIdsAndCrIntoOptVRdSc,
        naming::RdIdsAndCrIntoOptVecWhEqToFieldSc,
        naming::RdIdsAndCrIntoRdSc,
        naming::RdIdsAndCrIntoTtSc,
        naming::RdIdsAndCrIntoVecWhEqUsingFieldsSc,
        naming::RdIdsAndCrIntoWhEqSc,
        naming::RdIdsAndTtIntoPgTypeOptWhGreaterThanSc,
        naming::RdIdsSc,
        naming::RdIdsTo2DimsVecRdInnSc,
        naming::RdIdsToOptVRdDfltSomeOneElSc,
        naming::RdIdsUcc,
        naming::RdInnIntoRdWithNewOrTryNewUnwrapedSc,
        naming::RdInnIntoUpdWithNewOrTryNewUnwrapedSc,
        naming::RdInnUcc,
        naming::RdSc,
        naming::RdUcc,
        naming::SelOnlyIdsQpSc,
        naming::SelOnlyUpddIdsQbSc,
        naming::SelOnlyUpddIdsQpSc,
        naming::SelQpSc,
        naming::SelUcc,
        naming::SelfUcc,
        naming::TtSc,
        naming::TtUcc,
        naming::UpdForQueryUcc,
        naming::UpdQbSc,
        naming::UpdQpSc,
        naming::UpdToRdIdsSc,
        naming::UpdUcc,
        naming::VSc,
        naming::VUcc,
        naming::ValueSc,
        naming::WhUcc,
    );
    quote::quote! {
        #AllowClippyArbitrarySrcItemOrdering
        impl #import :: #PgTypeUcc for #ident {
            type #TtUcc = #ident_tt_ucc;
            fn #CrTblColQpSc(#ColSc: #import::SqlColRef<'_>, #is_pk_undrscr: #import::IsPk) -> #import::QpFragment {
                #cr_tbl_col_qp_ts
            }
            type #CrUcc = #ident_cr_ucc;
            fn #CrQpSc(
                #cr_qp_v_undrscr: &Self::#CrUcc,
                #cr_qp_incr_undrscr: &mut dyn #import::QpIncrMut
            ) -> Result<#import::QpFragment, #import ::#QpErUcc> {
                #cr_qp_ts
            }
            fn #CrQbSc(
                #cr_qb_v_undrscr: Self::#CrUcc,
                #is_cr_qb_mut #QuerySc: #import::PgQuery<'_>
            ) -> Result<#import::PgQuery<'_>, #import::PgQueryBindEr> {
                #cr_qb_ts
            }
            type #SelUcc = #ident_sel_ucc;
            fn #SelQpSc(
                #sel_qp_v_undrscr: &Self::#SelUcc,
                #ColSc: #import::SqlColRef<'_>,
            ) -> Result<#import::QpFragment, #import ::#QpErUcc> {
                #sel_qp_ts
            }
            type #WhUcc = #ident_wh_ucc;
            type #RdUcc = #ident_rd_ucc;
            fn #NormalizeSc(#VSc: Self::#RdUcc) -> Self::#RdUcc {
                #normalize_ts
            }
            type #RdIdsUcc = #rd_ids_ts;
            fn #SelOnlyIdsQpSc(
                #ColSc: #import::SqlColRef<'_>
            ) -> Result<#import::QpFragment, #import ::#QpErUcc> {
                #sel_only_ids_qp_ts
            }
            type #RdInnUcc = #ident_rd_inn_ucc;
            fn into_inn(#VSc: Self::#RdUcc) -> Self::#RdInnUcc {
                #into_inn_ts
            }
            type #UpdUcc = #ident_upd_ucc;
            type #UpdForQueryUcc = #ident_upd_for_query_ucc;
            #[allow(unused_variables)]
            fn #UpdQpSc(
                #upd_qp_v_undrscr: &Self::#UpdForQueryUcc,
                #upd_qp_accumulator_undrscr: #import::SqlColRef<'_>,
                #upd_qp_target_undrscr: #import::SqlColRef<'_>,
                #upd_qp_path_undrscr: #import::SqlColRef<'_>,
                #IncrSc: &mut dyn #import::QpIncrMut
            ) -> Result<#import::QpFragment, #import ::#QpErUcc> {
                #upd_qp_ts
            }
            fn #UpdQbSc(
                #VSc: Self::#UpdForQueryUcc,
                #is_upd_qb_mut #QuerySc: #import::PgQuery<'_>
            ) -> Result<#import::PgQuery<'_>, #import::PgQueryBindEr> {
                #upd_qb_ts
            }
            fn #SelOnlyUpddIdsQpSc(
                #VSc: &Self::#UpdForQueryUcc,
                #ColSc: #import::SqlColRef<'_>,
                #IncrSc: &mut dyn #import::QpIncrMut,
            ) -> Result<#import::QpFragment, #import ::#QpErUcc> {
                #sel_only_updd_ids_qp_ts
            }
            fn #SelOnlyUpddIdsQbSc<'lt>(
                #VSc: &'lt Self::#UpdForQueryUcc,
                #is_sel_only_updd_ids_qb_mut #QuerySc: #import::PgQuery<'lt>
            ) -> Result<#import::PgQuery<'lt>, #import::PgQueryBindEr> {
                #sel_only_updd_ids_qb_ts
            }
        }
    }.into()
}
pub fn gen_impl_pg_type_not_pk_for_ident_ts(
    import: &Import,
    ident: &dyn quote::ToTokens,
) -> macros_helpers::GeneratedRustTs {
    let ident_cr_ucc = naming::prm::SelfCrUcc::from_tokens(&ident);
    let allow_clippy_arbitrary_src_item_ordering =
        token_patterns::AllowClippyArbitrarySrcItemOrdering;
    let pg_type_not_pk_ucc = naming::PgTypeNotPkUcc;
    let pg_type_ucc = naming::PgTypeUcc;
    let cr_ucc = naming::CrUcc;
    quote::quote! {
        #allow_clippy_arbitrary_src_item_ordering
        impl #import::#pg_type_not_pk_ucc for #ident {
            type #pg_type_ucc = Self;
            type #cr_ucc = #ident_cr_ucc;
        }
    }
    .into()
}
#[expect(
    clippy::single_call_fn,
    reason = "keeps generated method snippets separated"
)]
fn gen_opt_vec_cr_ts(
    path_ts: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> macros_helpers::GeneratedRustTs {
    #[allow(non_snake_case, unused_variables)]
    let (
        AddOprtrSc,
        AllVrtsDfltSomeOneElMaxPageSizeSc,
        AllVrtsDfltSomeOneElSc,
        AllowClippyArbitrarySrcItemOrdering,
        ColSc,
        CrQbSc,
        CrQpSc,
        CrSc,
        CrTblColQpSc,
        CrUcc,
        DfltSomeOneElMaxPageSizeSc,
        DfltSomeOneElSc,
        EqOprtrUcc,
        ErSc,
        IncrSc,
        NormalizeSc,
        OptUpdSc,
        OptVecCrSc,
        PgCrudCmnDfltSomeOneElCall,
        PgTypeEqOprtrUcc,
        PgTypeNotPkUcc,
        PgTypeOptVecWhGreaterThanTestSc,
        PgTypeTestCasesUcc,
        PgTypeUcc,
        PgTypeWhFltUcc,
        PreviousRdAndOptUpdIntoRdSc,
        QbSc,
        QpErUcc,
        QpSc,
        QuerySc,
        RdIdsAndCrIntoOptVRdSc,
        RdIdsAndCrIntoOptVecWhEqToFieldSc,
        RdIdsAndCrIntoRdSc,
        RdIdsAndCrIntoTtSc,
        RdIdsAndCrIntoVecWhEqUsingFieldsSc,
        RdIdsAndCrIntoWhEqSc,
        RdIdsAndTtIntoPgTypeOptWhGreaterThanSc,
        RdIdsSc,
        RdIdsTo2DimsVecRdInnSc,
        RdIdsToOptVRdDfltSomeOneElSc,
        RdIdsUcc,
        RdInnIntoRdWithNewOrTryNewUnwrapedSc,
        RdInnIntoUpdWithNewOrTryNewUnwrapedSc,
        RdInnUcc,
        RdSc,
        RdUcc,
        SelOnlyIdsQpSc,
        SelOnlyUpddIdsQbSc,
        SelOnlyUpddIdsQpSc,
        SelQpSc,
        SelUcc,
        SelfUcc,
        TtSc,
        TtUcc,
        UpdForQueryUcc,
        UpdQbSc,
        UpdQpSc,
        UpdToRdIdsSc,
        UpdUcc,
        VSc,
        VUcc,
        ValueSc,
        WhUcc,
    ) = (
        naming::AddOprtrSc,
        naming::AllVrtsDfltSomeOneElMaxPageSizeSc,
        naming::AllVrtsDfltSomeOneElSc,
        token_patterns::AllowClippyArbitrarySrcItemOrdering,
        naming::ColSc,
        naming::CrQbSc,
        naming::CrQpSc,
        naming::CrSc,
        naming::CrTblColQpSc,
        naming::CrUcc,
        naming::DfltSomeOneElMaxPageSizeSc,
        naming::DfltSomeOneElSc,
        naming::EqOprtrUcc,
        naming::ErSc,
        naming::IncrSc,
        naming::NormalizeSc,
        naming::OptUpdSc,
        naming::OptVecCrSc,
        token_patterns::PgCrudCmnDfltSomeOneElCall,
        naming::PgTypeEqOprtrUcc,
        naming::PgTypeNotPkUcc,
        naming::PgTypeOptVecWhGreaterThanTestSc,
        naming::PgTypeTestCasesUcc,
        naming::PgTypeUcc,
        naming::PgTypeWhFltUcc,
        naming::PreviousRdAndOptUpdIntoRdSc,
        naming::QbSc,
        naming::QpErUcc,
        naming::QpSc,
        naming::QuerySc,
        naming::RdIdsAndCrIntoOptVRdSc,
        naming::RdIdsAndCrIntoOptVecWhEqToFieldSc,
        naming::RdIdsAndCrIntoRdSc,
        naming::RdIdsAndCrIntoTtSc,
        naming::RdIdsAndCrIntoVecWhEqUsingFieldsSc,
        naming::RdIdsAndCrIntoWhEqSc,
        naming::RdIdsAndTtIntoPgTypeOptWhGreaterThanSc,
        naming::RdIdsSc,
        naming::RdIdsTo2DimsVecRdInnSc,
        naming::RdIdsToOptVRdDfltSomeOneElSc,
        naming::RdIdsUcc,
        naming::RdInnIntoRdWithNewOrTryNewUnwrapedSc,
        naming::RdInnIntoUpdWithNewOrTryNewUnwrapedSc,
        naming::RdInnUcc,
        naming::RdSc,
        naming::RdUcc,
        naming::SelOnlyIdsQpSc,
        naming::SelOnlyUpddIdsQbSc,
        naming::SelOnlyUpddIdsQpSc,
        naming::SelQpSc,
        naming::SelUcc,
        naming::SelfUcc,
        naming::TtSc,
        naming::TtUcc,
        naming::UpdForQueryUcc,
        naming::UpdQbSc,
        naming::UpdQpSc,
        naming::UpdToRdIdsSc,
        naming::UpdUcc,
        naming::VSc,
        naming::VUcc,
        naming::ValueSc,
        naming::WhUcc,
    );
    quote::quote! {
        fn #OptVecCrSc() -> Option<Vec<#path_ts::#CrUcc>> {
            #ts
        }
    }
    .into()
}
#[expect(
    clippy::single_call_fn,
    reason = "keeps generated method snippets separated"
)]
fn gen_rd_ids_to_2_dims_vec_rd_inn_ts(
    path_ts: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> macros_helpers::GeneratedRustTs {
    #[allow(non_snake_case, unused_variables)]
    let (
        AddOprtrSc,
        AllVrtsDfltSomeOneElMaxPageSizeSc,
        AllVrtsDfltSomeOneElSc,
        AllowClippyArbitrarySrcItemOrdering,
        ColSc,
        CrQbSc,
        CrQpSc,
        CrSc,
        CrTblColQpSc,
        CrUcc,
        DfltSomeOneElMaxPageSizeSc,
        DfltSomeOneElSc,
        EqOprtrUcc,
        ErSc,
        IncrSc,
        NormalizeSc,
        OptUpdSc,
        OptVecCrSc,
        PgCrudCmnDfltSomeOneElCall,
        PgTypeEqOprtrUcc,
        PgTypeNotPkUcc,
        PgTypeOptVecWhGreaterThanTestSc,
        PgTypeTestCasesUcc,
        PgTypeUcc,
        PgTypeWhFltUcc,
        PreviousRdAndOptUpdIntoRdSc,
        QbSc,
        QpErUcc,
        QpSc,
        QuerySc,
        RdIdsAndCrIntoOptVRdSc,
        RdIdsAndCrIntoOptVecWhEqToFieldSc,
        RdIdsAndCrIntoRdSc,
        RdIdsAndCrIntoTtSc,
        RdIdsAndCrIntoVecWhEqUsingFieldsSc,
        RdIdsAndCrIntoWhEqSc,
        RdIdsAndTtIntoPgTypeOptWhGreaterThanSc,
        RdIdsSc,
        RdIdsTo2DimsVecRdInnSc,
        RdIdsToOptVRdDfltSomeOneElSc,
        RdIdsUcc,
        RdInnIntoRdWithNewOrTryNewUnwrapedSc,
        RdInnIntoUpdWithNewOrTryNewUnwrapedSc,
        RdInnUcc,
        RdSc,
        RdUcc,
        SelOnlyIdsQpSc,
        SelOnlyUpddIdsQbSc,
        SelOnlyUpddIdsQpSc,
        SelQpSc,
        SelUcc,
        SelfUcc,
        TtSc,
        TtUcc,
        UpdForQueryUcc,
        UpdQbSc,
        UpdQpSc,
        UpdToRdIdsSc,
        UpdUcc,
        VSc,
        VUcc,
        ValueSc,
        WhUcc,
    ) = (
        naming::AddOprtrSc,
        naming::AllVrtsDfltSomeOneElMaxPageSizeSc,
        naming::AllVrtsDfltSomeOneElSc,
        token_patterns::AllowClippyArbitrarySrcItemOrdering,
        naming::ColSc,
        naming::CrQbSc,
        naming::CrQpSc,
        naming::CrSc,
        naming::CrTblColQpSc,
        naming::CrUcc,
        naming::DfltSomeOneElMaxPageSizeSc,
        naming::DfltSomeOneElSc,
        naming::EqOprtrUcc,
        naming::ErSc,
        naming::IncrSc,
        naming::NormalizeSc,
        naming::OptUpdSc,
        naming::OptVecCrSc,
        token_patterns::PgCrudCmnDfltSomeOneElCall,
        naming::PgTypeEqOprtrUcc,
        naming::PgTypeNotPkUcc,
        naming::PgTypeOptVecWhGreaterThanTestSc,
        naming::PgTypeTestCasesUcc,
        naming::PgTypeUcc,
        naming::PgTypeWhFltUcc,
        naming::PreviousRdAndOptUpdIntoRdSc,
        naming::QbSc,
        naming::QpErUcc,
        naming::QpSc,
        naming::QuerySc,
        naming::RdIdsAndCrIntoOptVRdSc,
        naming::RdIdsAndCrIntoOptVecWhEqToFieldSc,
        naming::RdIdsAndCrIntoRdSc,
        naming::RdIdsAndCrIntoTtSc,
        naming::RdIdsAndCrIntoVecWhEqUsingFieldsSc,
        naming::RdIdsAndCrIntoWhEqSc,
        naming::RdIdsAndTtIntoPgTypeOptWhGreaterThanSc,
        naming::RdIdsSc,
        naming::RdIdsTo2DimsVecRdInnSc,
        naming::RdIdsToOptVRdDfltSomeOneElSc,
        naming::RdIdsUcc,
        naming::RdInnIntoRdWithNewOrTryNewUnwrapedSc,
        naming::RdInnIntoUpdWithNewOrTryNewUnwrapedSc,
        naming::RdInnUcc,
        naming::RdSc,
        naming::RdUcc,
        naming::SelOnlyIdsQpSc,
        naming::SelOnlyUpddIdsQbSc,
        naming::SelOnlyUpddIdsQpSc,
        naming::SelQpSc,
        naming::SelUcc,
        naming::SelfUcc,
        naming::TtSc,
        naming::TtUcc,
        naming::UpdForQueryUcc,
        naming::UpdQbSc,
        naming::UpdQpSc,
        naming::UpdToRdIdsSc,
        naming::UpdUcc,
        naming::VSc,
        naming::VUcc,
        naming::ValueSc,
        naming::WhUcc,
    );
    quote::quote! {
        fn #RdIdsTo2DimsVecRdInnSc(
            #RdIdsSc: &#path_ts::#RdIdsUcc
        ) -> Vec<Vec<#path_ts::#RdInnUcc>> {
            #ts
        }
    }
    .into()
}
fn gen_rd_inn_into_rd_or_upd_with_new_or_try_new_unwraped_ts(
    method_name_ts: &dyn quote::ToTokens,
    type_ts: &dyn quote::ToTokens,
    path_ts: &dyn quote::ToTokens,
    return_type_ts: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> macros_helpers::GeneratedRustTs {
    #[allow(non_snake_case, unused_variables)]
    let (
        AddOprtrSc,
        AllVrtsDfltSomeOneElMaxPageSizeSc,
        AllVrtsDfltSomeOneElSc,
        AllowClippyArbitrarySrcItemOrdering,
        ColSc,
        CrQbSc,
        CrQpSc,
        CrSc,
        CrTblColQpSc,
        CrUcc,
        DfltSomeOneElMaxPageSizeSc,
        DfltSomeOneElSc,
        EqOprtrUcc,
        ErSc,
        IncrSc,
        NormalizeSc,
        OptUpdSc,
        OptVecCrSc,
        PgCrudCmnDfltSomeOneElCall,
        PgTypeEqOprtrUcc,
        PgTypeNotPkUcc,
        PgTypeOptVecWhGreaterThanTestSc,
        PgTypeTestCasesUcc,
        PgTypeUcc,
        PgTypeWhFltUcc,
        PreviousRdAndOptUpdIntoRdSc,
        QbSc,
        QpErUcc,
        QpSc,
        QuerySc,
        RdIdsAndCrIntoOptVRdSc,
        RdIdsAndCrIntoOptVecWhEqToFieldSc,
        RdIdsAndCrIntoRdSc,
        RdIdsAndCrIntoTtSc,
        RdIdsAndCrIntoVecWhEqUsingFieldsSc,
        RdIdsAndCrIntoWhEqSc,
        RdIdsAndTtIntoPgTypeOptWhGreaterThanSc,
        RdIdsSc,
        RdIdsTo2DimsVecRdInnSc,
        RdIdsToOptVRdDfltSomeOneElSc,
        RdIdsUcc,
        RdInnIntoRdWithNewOrTryNewUnwrapedSc,
        RdInnIntoUpdWithNewOrTryNewUnwrapedSc,
        RdInnUcc,
        RdSc,
        RdUcc,
        SelOnlyIdsQpSc,
        SelOnlyUpddIdsQbSc,
        SelOnlyUpddIdsQpSc,
        SelQpSc,
        SelUcc,
        SelfUcc,
        TtSc,
        TtUcc,
        UpdForQueryUcc,
        UpdQbSc,
        UpdQpSc,
        UpdToRdIdsSc,
        UpdUcc,
        VSc,
        VUcc,
        ValueSc,
        WhUcc,
    ) = (
        naming::AddOprtrSc,
        naming::AllVrtsDfltSomeOneElMaxPageSizeSc,
        naming::AllVrtsDfltSomeOneElSc,
        token_patterns::AllowClippyArbitrarySrcItemOrdering,
        naming::ColSc,
        naming::CrQbSc,
        naming::CrQpSc,
        naming::CrSc,
        naming::CrTblColQpSc,
        naming::CrUcc,
        naming::DfltSomeOneElMaxPageSizeSc,
        naming::DfltSomeOneElSc,
        naming::EqOprtrUcc,
        naming::ErSc,
        naming::IncrSc,
        naming::NormalizeSc,
        naming::OptUpdSc,
        naming::OptVecCrSc,
        token_patterns::PgCrudCmnDfltSomeOneElCall,
        naming::PgTypeEqOprtrUcc,
        naming::PgTypeNotPkUcc,
        naming::PgTypeOptVecWhGreaterThanTestSc,
        naming::PgTypeTestCasesUcc,
        naming::PgTypeUcc,
        naming::PgTypeWhFltUcc,
        naming::PreviousRdAndOptUpdIntoRdSc,
        naming::QbSc,
        naming::QpErUcc,
        naming::QpSc,
        naming::QuerySc,
        naming::RdIdsAndCrIntoOptVRdSc,
        naming::RdIdsAndCrIntoOptVecWhEqToFieldSc,
        naming::RdIdsAndCrIntoRdSc,
        naming::RdIdsAndCrIntoTtSc,
        naming::RdIdsAndCrIntoVecWhEqUsingFieldsSc,
        naming::RdIdsAndCrIntoWhEqSc,
        naming::RdIdsAndTtIntoPgTypeOptWhGreaterThanSc,
        naming::RdIdsSc,
        naming::RdIdsTo2DimsVecRdInnSc,
        naming::RdIdsToOptVRdDfltSomeOneElSc,
        naming::RdIdsUcc,
        naming::RdInnIntoRdWithNewOrTryNewUnwrapedSc,
        naming::RdInnIntoUpdWithNewOrTryNewUnwrapedSc,
        naming::RdInnUcc,
        naming::RdSc,
        naming::RdUcc,
        naming::SelOnlyIdsQpSc,
        naming::SelOnlyUpddIdsQbSc,
        naming::SelOnlyUpddIdsQpSc,
        naming::SelQpSc,
        naming::SelUcc,
        naming::SelfUcc,
        naming::TtSc,
        naming::TtUcc,
        naming::UpdForQueryUcc,
        naming::UpdQbSc,
        naming::UpdQpSc,
        naming::UpdToRdIdsSc,
        naming::UpdUcc,
        naming::VSc,
        naming::VUcc,
        naming::ValueSc,
        naming::WhUcc,
    );
    quote::quote! {
        fn #method_name_ts(
            #VSc: #type_ts
        ) -> #path_ts::#return_type_ts {
            #ts
        }
    }
    .into()
}
#[expect(
    clippy::single_call_fn,
    reason = "keeps generated method snippets separated"
)]
fn gen_upd_to_rd_ids_ts(
    path_ts: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> macros_helpers::GeneratedRustTs {
    #[allow(non_snake_case, unused_variables)]
    let (
        AddOprtrSc,
        AllVrtsDfltSomeOneElMaxPageSizeSc,
        AllVrtsDfltSomeOneElSc,
        AllowClippyArbitrarySrcItemOrdering,
        ColSc,
        CrQbSc,
        CrQpSc,
        CrSc,
        CrTblColQpSc,
        CrUcc,
        DfltSomeOneElMaxPageSizeSc,
        DfltSomeOneElSc,
        EqOprtrUcc,
        ErSc,
        IncrSc,
        NormalizeSc,
        OptUpdSc,
        OptVecCrSc,
        PgCrudCmnDfltSomeOneElCall,
        PgTypeEqOprtrUcc,
        PgTypeNotPkUcc,
        PgTypeOptVecWhGreaterThanTestSc,
        PgTypeTestCasesUcc,
        PgTypeUcc,
        PgTypeWhFltUcc,
        PreviousRdAndOptUpdIntoRdSc,
        QbSc,
        QpErUcc,
        QpSc,
        QuerySc,
        RdIdsAndCrIntoOptVRdSc,
        RdIdsAndCrIntoOptVecWhEqToFieldSc,
        RdIdsAndCrIntoRdSc,
        RdIdsAndCrIntoTtSc,
        RdIdsAndCrIntoVecWhEqUsingFieldsSc,
        RdIdsAndCrIntoWhEqSc,
        RdIdsAndTtIntoPgTypeOptWhGreaterThanSc,
        RdIdsSc,
        RdIdsTo2DimsVecRdInnSc,
        RdIdsToOptVRdDfltSomeOneElSc,
        RdIdsUcc,
        RdInnIntoRdWithNewOrTryNewUnwrapedSc,
        RdInnIntoUpdWithNewOrTryNewUnwrapedSc,
        RdInnUcc,
        RdSc,
        RdUcc,
        SelOnlyIdsQpSc,
        SelOnlyUpddIdsQbSc,
        SelOnlyUpddIdsQpSc,
        SelQpSc,
        SelUcc,
        SelfUcc,
        TtSc,
        TtUcc,
        UpdForQueryUcc,
        UpdQbSc,
        UpdQpSc,
        UpdToRdIdsSc,
        UpdUcc,
        VSc,
        VUcc,
        ValueSc,
        WhUcc,
    ) = (
        naming::AddOprtrSc,
        naming::AllVrtsDfltSomeOneElMaxPageSizeSc,
        naming::AllVrtsDfltSomeOneElSc,
        token_patterns::AllowClippyArbitrarySrcItemOrdering,
        naming::ColSc,
        naming::CrQbSc,
        naming::CrQpSc,
        naming::CrSc,
        naming::CrTblColQpSc,
        naming::CrUcc,
        naming::DfltSomeOneElMaxPageSizeSc,
        naming::DfltSomeOneElSc,
        naming::EqOprtrUcc,
        naming::ErSc,
        naming::IncrSc,
        naming::NormalizeSc,
        naming::OptUpdSc,
        naming::OptVecCrSc,
        token_patterns::PgCrudCmnDfltSomeOneElCall,
        naming::PgTypeEqOprtrUcc,
        naming::PgTypeNotPkUcc,
        naming::PgTypeOptVecWhGreaterThanTestSc,
        naming::PgTypeTestCasesUcc,
        naming::PgTypeUcc,
        naming::PgTypeWhFltUcc,
        naming::PreviousRdAndOptUpdIntoRdSc,
        naming::QbSc,
        naming::QpErUcc,
        naming::QpSc,
        naming::QuerySc,
        naming::RdIdsAndCrIntoOptVRdSc,
        naming::RdIdsAndCrIntoOptVecWhEqToFieldSc,
        naming::RdIdsAndCrIntoRdSc,
        naming::RdIdsAndCrIntoTtSc,
        naming::RdIdsAndCrIntoVecWhEqUsingFieldsSc,
        naming::RdIdsAndCrIntoWhEqSc,
        naming::RdIdsAndTtIntoPgTypeOptWhGreaterThanSc,
        naming::RdIdsSc,
        naming::RdIdsTo2DimsVecRdInnSc,
        naming::RdIdsToOptVRdDfltSomeOneElSc,
        naming::RdIdsUcc,
        naming::RdInnIntoRdWithNewOrTryNewUnwrapedSc,
        naming::RdInnIntoUpdWithNewOrTryNewUnwrapedSc,
        naming::RdInnUcc,
        naming::RdSc,
        naming::RdUcc,
        naming::SelOnlyIdsQpSc,
        naming::SelOnlyUpddIdsQbSc,
        naming::SelOnlyUpddIdsQpSc,
        naming::SelQpSc,
        naming::SelUcc,
        naming::SelfUcc,
        naming::TtSc,
        naming::TtUcc,
        naming::UpdForQueryUcc,
        naming::UpdQbSc,
        naming::UpdQpSc,
        naming::UpdToRdIdsSc,
        naming::UpdUcc,
        naming::VSc,
        naming::VUcc,
        naming::ValueSc,
        naming::WhUcc,
    );
    quote::quote! {
        fn #UpdToRdIdsSc(
            #VSc: &#path_ts::#UpdUcc
        ) -> #path_ts::#RdIdsUcc {
            #ts
        }
    }
    .into()
}
#[expect(
    clippy::single_call_fn,
    reason = "keeps generated method snippets separated"
)]
fn gen_rd_ids_to_opt_v_rd_dflt_some_one_el_ts(
    import: Import,
    path_ts: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> macros_helpers::GeneratedRustTs {
    #[allow(non_snake_case, unused_variables)]
    let (
        AddOprtrSc,
        AllVrtsDfltSomeOneElMaxPageSizeSc,
        AllVrtsDfltSomeOneElSc,
        AllowClippyArbitrarySrcItemOrdering,
        ColSc,
        CrQbSc,
        CrQpSc,
        CrSc,
        CrTblColQpSc,
        CrUcc,
        DfltSomeOneElMaxPageSizeSc,
        DfltSomeOneElSc,
        EqOprtrUcc,
        ErSc,
        IncrSc,
        NormalizeSc,
        OptUpdSc,
        OptVecCrSc,
        PgCrudCmnDfltSomeOneElCall,
        PgTypeEqOprtrUcc,
        PgTypeNotPkUcc,
        PgTypeOptVecWhGreaterThanTestSc,
        PgTypeTestCasesUcc,
        PgTypeUcc,
        PgTypeWhFltUcc,
        PreviousRdAndOptUpdIntoRdSc,
        QbSc,
        QpErUcc,
        QpSc,
        QuerySc,
        RdIdsAndCrIntoOptVRdSc,
        RdIdsAndCrIntoOptVecWhEqToFieldSc,
        RdIdsAndCrIntoRdSc,
        RdIdsAndCrIntoTtSc,
        RdIdsAndCrIntoVecWhEqUsingFieldsSc,
        RdIdsAndCrIntoWhEqSc,
        RdIdsAndTtIntoPgTypeOptWhGreaterThanSc,
        RdIdsSc,
        RdIdsTo2DimsVecRdInnSc,
        RdIdsToOptVRdDfltSomeOneElSc,
        RdIdsUcc,
        RdInnIntoRdWithNewOrTryNewUnwrapedSc,
        RdInnIntoUpdWithNewOrTryNewUnwrapedSc,
        RdInnUcc,
        RdSc,
        RdUcc,
        SelOnlyIdsQpSc,
        SelOnlyUpddIdsQbSc,
        SelOnlyUpddIdsQpSc,
        SelQpSc,
        SelUcc,
        SelfUcc,
        TtSc,
        TtUcc,
        UpdForQueryUcc,
        UpdQbSc,
        UpdQpSc,
        UpdToRdIdsSc,
        UpdUcc,
        VSc,
        VUcc,
        ValueSc,
        WhUcc,
    ) = (
        naming::AddOprtrSc,
        naming::AllVrtsDfltSomeOneElMaxPageSizeSc,
        naming::AllVrtsDfltSomeOneElSc,
        token_patterns::AllowClippyArbitrarySrcItemOrdering,
        naming::ColSc,
        naming::CrQbSc,
        naming::CrQpSc,
        naming::CrSc,
        naming::CrTblColQpSc,
        naming::CrUcc,
        naming::DfltSomeOneElMaxPageSizeSc,
        naming::DfltSomeOneElSc,
        naming::EqOprtrUcc,
        naming::ErSc,
        naming::IncrSc,
        naming::NormalizeSc,
        naming::OptUpdSc,
        naming::OptVecCrSc,
        token_patterns::PgCrudCmnDfltSomeOneElCall,
        naming::PgTypeEqOprtrUcc,
        naming::PgTypeNotPkUcc,
        naming::PgTypeOptVecWhGreaterThanTestSc,
        naming::PgTypeTestCasesUcc,
        naming::PgTypeUcc,
        naming::PgTypeWhFltUcc,
        naming::PreviousRdAndOptUpdIntoRdSc,
        naming::QbSc,
        naming::QpErUcc,
        naming::QpSc,
        naming::QuerySc,
        naming::RdIdsAndCrIntoOptVRdSc,
        naming::RdIdsAndCrIntoOptVecWhEqToFieldSc,
        naming::RdIdsAndCrIntoRdSc,
        naming::RdIdsAndCrIntoTtSc,
        naming::RdIdsAndCrIntoVecWhEqUsingFieldsSc,
        naming::RdIdsAndCrIntoWhEqSc,
        naming::RdIdsAndTtIntoPgTypeOptWhGreaterThanSc,
        naming::RdIdsSc,
        naming::RdIdsTo2DimsVecRdInnSc,
        naming::RdIdsToOptVRdDfltSomeOneElSc,
        naming::RdIdsUcc,
        naming::RdInnIntoRdWithNewOrTryNewUnwrapedSc,
        naming::RdInnIntoUpdWithNewOrTryNewUnwrapedSc,
        naming::RdInnUcc,
        naming::RdSc,
        naming::RdUcc,
        naming::SelOnlyIdsQpSc,
        naming::SelOnlyUpddIdsQbSc,
        naming::SelOnlyUpddIdsQpSc,
        naming::SelQpSc,
        naming::SelUcc,
        naming::SelfUcc,
        naming::TtSc,
        naming::TtUcc,
        naming::UpdForQueryUcc,
        naming::UpdQbSc,
        naming::UpdQpSc,
        naming::UpdToRdIdsSc,
        naming::UpdUcc,
        naming::VSc,
        naming::VUcc,
        naming::ValueSc,
        naming::WhUcc,
    );
    quote::quote! {
        fn #RdIdsToOptVRdDfltSomeOneElSc(
            #VSc: &#path_ts::#RdIdsUcc
        ) -> Option<#import::#VUcc<#path_ts::#RdUcc>> {
            #ts
        }
    }
    .into()
}
#[expect(
    clippy::single_call_fn,
    reason = "keeps generated method snippets separated"
)]
fn gen_previous_rd_and_opt_upd_into_rd_ts(
    path_ts: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> macros_helpers::GeneratedRustTs {
    #[allow(non_snake_case, unused_variables)]
    let (
        AddOprtrSc,
        AllVrtsDfltSomeOneElMaxPageSizeSc,
        AllVrtsDfltSomeOneElSc,
        AllowClippyArbitrarySrcItemOrdering,
        ColSc,
        CrQbSc,
        CrQpSc,
        CrSc,
        CrTblColQpSc,
        CrUcc,
        DfltSomeOneElMaxPageSizeSc,
        DfltSomeOneElSc,
        EqOprtrUcc,
        ErSc,
        IncrSc,
        NormalizeSc,
        OptUpdSc,
        OptVecCrSc,
        PgCrudCmnDfltSomeOneElCall,
        PgTypeEqOprtrUcc,
        PgTypeNotPkUcc,
        PgTypeOptVecWhGreaterThanTestSc,
        PgTypeTestCasesUcc,
        PgTypeUcc,
        PgTypeWhFltUcc,
        PreviousRdAndOptUpdIntoRdSc,
        QbSc,
        QpErUcc,
        QpSc,
        QuerySc,
        RdIdsAndCrIntoOptVRdSc,
        RdIdsAndCrIntoOptVecWhEqToFieldSc,
        RdIdsAndCrIntoRdSc,
        RdIdsAndCrIntoTtSc,
        RdIdsAndCrIntoVecWhEqUsingFieldsSc,
        RdIdsAndCrIntoWhEqSc,
        RdIdsAndTtIntoPgTypeOptWhGreaterThanSc,
        RdIdsSc,
        RdIdsTo2DimsVecRdInnSc,
        RdIdsToOptVRdDfltSomeOneElSc,
        RdIdsUcc,
        RdInnIntoRdWithNewOrTryNewUnwrapedSc,
        RdInnIntoUpdWithNewOrTryNewUnwrapedSc,
        RdInnUcc,
        RdSc,
        RdUcc,
        SelOnlyIdsQpSc,
        SelOnlyUpddIdsQbSc,
        SelOnlyUpddIdsQpSc,
        SelQpSc,
        SelUcc,
        SelfUcc,
        TtSc,
        TtUcc,
        UpdForQueryUcc,
        UpdQbSc,
        UpdQpSc,
        UpdToRdIdsSc,
        UpdUcc,
        VSc,
        VUcc,
        ValueSc,
        WhUcc,
    ) = (
        naming::AddOprtrSc,
        naming::AllVrtsDfltSomeOneElMaxPageSizeSc,
        naming::AllVrtsDfltSomeOneElSc,
        token_patterns::AllowClippyArbitrarySrcItemOrdering,
        naming::ColSc,
        naming::CrQbSc,
        naming::CrQpSc,
        naming::CrSc,
        naming::CrTblColQpSc,
        naming::CrUcc,
        naming::DfltSomeOneElMaxPageSizeSc,
        naming::DfltSomeOneElSc,
        naming::EqOprtrUcc,
        naming::ErSc,
        naming::IncrSc,
        naming::NormalizeSc,
        naming::OptUpdSc,
        naming::OptVecCrSc,
        token_patterns::PgCrudCmnDfltSomeOneElCall,
        naming::PgTypeEqOprtrUcc,
        naming::PgTypeNotPkUcc,
        naming::PgTypeOptVecWhGreaterThanTestSc,
        naming::PgTypeTestCasesUcc,
        naming::PgTypeUcc,
        naming::PgTypeWhFltUcc,
        naming::PreviousRdAndOptUpdIntoRdSc,
        naming::QbSc,
        naming::QpErUcc,
        naming::QpSc,
        naming::QuerySc,
        naming::RdIdsAndCrIntoOptVRdSc,
        naming::RdIdsAndCrIntoOptVecWhEqToFieldSc,
        naming::RdIdsAndCrIntoRdSc,
        naming::RdIdsAndCrIntoTtSc,
        naming::RdIdsAndCrIntoVecWhEqUsingFieldsSc,
        naming::RdIdsAndCrIntoWhEqSc,
        naming::RdIdsAndTtIntoPgTypeOptWhGreaterThanSc,
        naming::RdIdsSc,
        naming::RdIdsTo2DimsVecRdInnSc,
        naming::RdIdsToOptVRdDfltSomeOneElSc,
        naming::RdIdsUcc,
        naming::RdInnIntoRdWithNewOrTryNewUnwrapedSc,
        naming::RdInnIntoUpdWithNewOrTryNewUnwrapedSc,
        naming::RdInnUcc,
        naming::RdSc,
        naming::RdUcc,
        naming::SelOnlyIdsQpSc,
        naming::SelOnlyUpddIdsQbSc,
        naming::SelOnlyUpddIdsQpSc,
        naming::SelQpSc,
        naming::SelUcc,
        naming::SelfUcc,
        naming::TtSc,
        naming::TtUcc,
        naming::UpdForQueryUcc,
        naming::UpdQbSc,
        naming::UpdQpSc,
        naming::UpdToRdIdsSc,
        naming::UpdUcc,
        naming::VSc,
        naming::VUcc,
        naming::ValueSc,
        naming::WhUcc,
    );
    quote::quote! {
        fn #PreviousRdAndOptUpdIntoRdSc(
            #RdSc: #path_ts::#RdUcc,
            #OptUpdSc: Option<#path_ts::#UpdUcc>,
        ) -> #path_ts::#RdUcc {
            #ts
        }
    }
    .into()
}
#[expect(
    clippy::single_call_fn,
    reason = "keeps generated method snippets separated"
)]
fn gen_rd_ids_and_cr_into_rd_ts(
    path_ts: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> macros_helpers::GeneratedRustTs {
    #[allow(non_snake_case, unused_variables)]
    let (
        AddOprtrSc,
        AllVrtsDfltSomeOneElMaxPageSizeSc,
        AllVrtsDfltSomeOneElSc,
        AllowClippyArbitrarySrcItemOrdering,
        ColSc,
        CrQbSc,
        CrQpSc,
        CrSc,
        CrTblColQpSc,
        CrUcc,
        DfltSomeOneElMaxPageSizeSc,
        DfltSomeOneElSc,
        EqOprtrUcc,
        ErSc,
        IncrSc,
        NormalizeSc,
        OptUpdSc,
        OptVecCrSc,
        PgCrudCmnDfltSomeOneElCall,
        PgTypeEqOprtrUcc,
        PgTypeNotPkUcc,
        PgTypeOptVecWhGreaterThanTestSc,
        PgTypeTestCasesUcc,
        PgTypeUcc,
        PgTypeWhFltUcc,
        PreviousRdAndOptUpdIntoRdSc,
        QbSc,
        QpErUcc,
        QpSc,
        QuerySc,
        RdIdsAndCrIntoOptVRdSc,
        RdIdsAndCrIntoOptVecWhEqToFieldSc,
        RdIdsAndCrIntoRdSc,
        RdIdsAndCrIntoTtSc,
        RdIdsAndCrIntoVecWhEqUsingFieldsSc,
        RdIdsAndCrIntoWhEqSc,
        RdIdsAndTtIntoPgTypeOptWhGreaterThanSc,
        RdIdsSc,
        RdIdsTo2DimsVecRdInnSc,
        RdIdsToOptVRdDfltSomeOneElSc,
        RdIdsUcc,
        RdInnIntoRdWithNewOrTryNewUnwrapedSc,
        RdInnIntoUpdWithNewOrTryNewUnwrapedSc,
        RdInnUcc,
        RdSc,
        RdUcc,
        SelOnlyIdsQpSc,
        SelOnlyUpddIdsQbSc,
        SelOnlyUpddIdsQpSc,
        SelQpSc,
        SelUcc,
        SelfUcc,
        TtSc,
        TtUcc,
        UpdForQueryUcc,
        UpdQbSc,
        UpdQpSc,
        UpdToRdIdsSc,
        UpdUcc,
        VSc,
        VUcc,
        ValueSc,
        WhUcc,
    ) = (
        naming::AddOprtrSc,
        naming::AllVrtsDfltSomeOneElMaxPageSizeSc,
        naming::AllVrtsDfltSomeOneElSc,
        token_patterns::AllowClippyArbitrarySrcItemOrdering,
        naming::ColSc,
        naming::CrQbSc,
        naming::CrQpSc,
        naming::CrSc,
        naming::CrTblColQpSc,
        naming::CrUcc,
        naming::DfltSomeOneElMaxPageSizeSc,
        naming::DfltSomeOneElSc,
        naming::EqOprtrUcc,
        naming::ErSc,
        naming::IncrSc,
        naming::NormalizeSc,
        naming::OptUpdSc,
        naming::OptVecCrSc,
        token_patterns::PgCrudCmnDfltSomeOneElCall,
        naming::PgTypeEqOprtrUcc,
        naming::PgTypeNotPkUcc,
        naming::PgTypeOptVecWhGreaterThanTestSc,
        naming::PgTypeTestCasesUcc,
        naming::PgTypeUcc,
        naming::PgTypeWhFltUcc,
        naming::PreviousRdAndOptUpdIntoRdSc,
        naming::QbSc,
        naming::QpErUcc,
        naming::QpSc,
        naming::QuerySc,
        naming::RdIdsAndCrIntoOptVRdSc,
        naming::RdIdsAndCrIntoOptVecWhEqToFieldSc,
        naming::RdIdsAndCrIntoRdSc,
        naming::RdIdsAndCrIntoTtSc,
        naming::RdIdsAndCrIntoVecWhEqUsingFieldsSc,
        naming::RdIdsAndCrIntoWhEqSc,
        naming::RdIdsAndTtIntoPgTypeOptWhGreaterThanSc,
        naming::RdIdsSc,
        naming::RdIdsTo2DimsVecRdInnSc,
        naming::RdIdsToOptVRdDfltSomeOneElSc,
        naming::RdIdsUcc,
        naming::RdInnIntoRdWithNewOrTryNewUnwrapedSc,
        naming::RdInnIntoUpdWithNewOrTryNewUnwrapedSc,
        naming::RdInnUcc,
        naming::RdSc,
        naming::RdUcc,
        naming::SelOnlyIdsQpSc,
        naming::SelOnlyUpddIdsQbSc,
        naming::SelOnlyUpddIdsQpSc,
        naming::SelQpSc,
        naming::SelUcc,
        naming::SelfUcc,
        naming::TtSc,
        naming::TtUcc,
        naming::UpdForQueryUcc,
        naming::UpdQbSc,
        naming::UpdQpSc,
        naming::UpdToRdIdsSc,
        naming::UpdUcc,
        naming::VSc,
        naming::VUcc,
        naming::ValueSc,
        naming::WhUcc,
    );
    quote::quote! {
        fn #RdIdsAndCrIntoRdSc(
            #RdIdsSc: #path_ts::#RdIdsUcc,
            #CrSc: #path_ts::#CrUcc
        ) -> #path_ts::#RdUcc {
            #ts
        }
    }
    .into()
}
#[expect(
    clippy::single_call_fn,
    reason = "keeps generated method snippets separated"
)]
fn gen_rd_ids_and_cr_into_opt_v_rd_ts(
    import: Import,
    path_ts: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> macros_helpers::GeneratedRustTs {
    #[allow(non_snake_case, unused_variables)]
    let (
        AddOprtrSc,
        AllVrtsDfltSomeOneElMaxPageSizeSc,
        AllVrtsDfltSomeOneElSc,
        AllowClippyArbitrarySrcItemOrdering,
        ColSc,
        CrQbSc,
        CrQpSc,
        CrSc,
        CrTblColQpSc,
        CrUcc,
        DfltSomeOneElMaxPageSizeSc,
        DfltSomeOneElSc,
        EqOprtrUcc,
        ErSc,
        IncrSc,
        NormalizeSc,
        OptUpdSc,
        OptVecCrSc,
        PgCrudCmnDfltSomeOneElCall,
        PgTypeEqOprtrUcc,
        PgTypeNotPkUcc,
        PgTypeOptVecWhGreaterThanTestSc,
        PgTypeTestCasesUcc,
        PgTypeUcc,
        PgTypeWhFltUcc,
        PreviousRdAndOptUpdIntoRdSc,
        QbSc,
        QpErUcc,
        QpSc,
        QuerySc,
        RdIdsAndCrIntoOptVRdSc,
        RdIdsAndCrIntoOptVecWhEqToFieldSc,
        RdIdsAndCrIntoRdSc,
        RdIdsAndCrIntoTtSc,
        RdIdsAndCrIntoVecWhEqUsingFieldsSc,
        RdIdsAndCrIntoWhEqSc,
        RdIdsAndTtIntoPgTypeOptWhGreaterThanSc,
        RdIdsSc,
        RdIdsTo2DimsVecRdInnSc,
        RdIdsToOptVRdDfltSomeOneElSc,
        RdIdsUcc,
        RdInnIntoRdWithNewOrTryNewUnwrapedSc,
        RdInnIntoUpdWithNewOrTryNewUnwrapedSc,
        RdInnUcc,
        RdSc,
        RdUcc,
        SelOnlyIdsQpSc,
        SelOnlyUpddIdsQbSc,
        SelOnlyUpddIdsQpSc,
        SelQpSc,
        SelUcc,
        SelfUcc,
        TtSc,
        TtUcc,
        UpdForQueryUcc,
        UpdQbSc,
        UpdQpSc,
        UpdToRdIdsSc,
        UpdUcc,
        VSc,
        VUcc,
        ValueSc,
        WhUcc,
    ) = (
        naming::AddOprtrSc,
        naming::AllVrtsDfltSomeOneElMaxPageSizeSc,
        naming::AllVrtsDfltSomeOneElSc,
        token_patterns::AllowClippyArbitrarySrcItemOrdering,
        naming::ColSc,
        naming::CrQbSc,
        naming::CrQpSc,
        naming::CrSc,
        naming::CrTblColQpSc,
        naming::CrUcc,
        naming::DfltSomeOneElMaxPageSizeSc,
        naming::DfltSomeOneElSc,
        naming::EqOprtrUcc,
        naming::ErSc,
        naming::IncrSc,
        naming::NormalizeSc,
        naming::OptUpdSc,
        naming::OptVecCrSc,
        token_patterns::PgCrudCmnDfltSomeOneElCall,
        naming::PgTypeEqOprtrUcc,
        naming::PgTypeNotPkUcc,
        naming::PgTypeOptVecWhGreaterThanTestSc,
        naming::PgTypeTestCasesUcc,
        naming::PgTypeUcc,
        naming::PgTypeWhFltUcc,
        naming::PreviousRdAndOptUpdIntoRdSc,
        naming::QbSc,
        naming::QpErUcc,
        naming::QpSc,
        naming::QuerySc,
        naming::RdIdsAndCrIntoOptVRdSc,
        naming::RdIdsAndCrIntoOptVecWhEqToFieldSc,
        naming::RdIdsAndCrIntoRdSc,
        naming::RdIdsAndCrIntoTtSc,
        naming::RdIdsAndCrIntoVecWhEqUsingFieldsSc,
        naming::RdIdsAndCrIntoWhEqSc,
        naming::RdIdsAndTtIntoPgTypeOptWhGreaterThanSc,
        naming::RdIdsSc,
        naming::RdIdsTo2DimsVecRdInnSc,
        naming::RdIdsToOptVRdDfltSomeOneElSc,
        naming::RdIdsUcc,
        naming::RdInnIntoRdWithNewOrTryNewUnwrapedSc,
        naming::RdInnIntoUpdWithNewOrTryNewUnwrapedSc,
        naming::RdInnUcc,
        naming::RdSc,
        naming::RdUcc,
        naming::SelOnlyIdsQpSc,
        naming::SelOnlyUpddIdsQbSc,
        naming::SelOnlyUpddIdsQpSc,
        naming::SelQpSc,
        naming::SelUcc,
        naming::SelfUcc,
        naming::TtSc,
        naming::TtUcc,
        naming::UpdForQueryUcc,
        naming::UpdQbSc,
        naming::UpdQpSc,
        naming::UpdToRdIdsSc,
        naming::UpdUcc,
        naming::VSc,
        naming::VUcc,
        naming::ValueSc,
        naming::WhUcc,
    );
    quote::quote! {
        fn #RdIdsAndCrIntoOptVRdSc(
            #RdIdsSc: #path_ts::#RdIdsUcc,
            #CrSc: #path_ts::#CrUcc
        ) -> Option<#import::#VUcc<#path_ts::#RdUcc>> {
            #ts
        }
    }
    .into()
}
#[expect(
    clippy::single_call_fn,
    reason = "keeps generated method snippets separated"
)]
fn gen_rd_ids_and_cr_into_tt_ts(
    path_ts: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> macros_helpers::GeneratedRustTs {
    #[allow(non_snake_case, unused_variables)]
    let (
        AddOprtrSc,
        AllVrtsDfltSomeOneElMaxPageSizeSc,
        AllVrtsDfltSomeOneElSc,
        AllowClippyArbitrarySrcItemOrdering,
        ColSc,
        CrQbSc,
        CrQpSc,
        CrSc,
        CrTblColQpSc,
        CrUcc,
        DfltSomeOneElMaxPageSizeSc,
        DfltSomeOneElSc,
        EqOprtrUcc,
        ErSc,
        IncrSc,
        NormalizeSc,
        OptUpdSc,
        OptVecCrSc,
        PgCrudCmnDfltSomeOneElCall,
        PgTypeEqOprtrUcc,
        PgTypeNotPkUcc,
        PgTypeOptVecWhGreaterThanTestSc,
        PgTypeTestCasesUcc,
        PgTypeUcc,
        PgTypeWhFltUcc,
        PreviousRdAndOptUpdIntoRdSc,
        QbSc,
        QpErUcc,
        QpSc,
        QuerySc,
        RdIdsAndCrIntoOptVRdSc,
        RdIdsAndCrIntoOptVecWhEqToFieldSc,
        RdIdsAndCrIntoRdSc,
        RdIdsAndCrIntoTtSc,
        RdIdsAndCrIntoVecWhEqUsingFieldsSc,
        RdIdsAndCrIntoWhEqSc,
        RdIdsAndTtIntoPgTypeOptWhGreaterThanSc,
        RdIdsSc,
        RdIdsTo2DimsVecRdInnSc,
        RdIdsToOptVRdDfltSomeOneElSc,
        RdIdsUcc,
        RdInnIntoRdWithNewOrTryNewUnwrapedSc,
        RdInnIntoUpdWithNewOrTryNewUnwrapedSc,
        RdInnUcc,
        RdSc,
        RdUcc,
        SelOnlyIdsQpSc,
        SelOnlyUpddIdsQbSc,
        SelOnlyUpddIdsQpSc,
        SelQpSc,
        SelUcc,
        SelfUcc,
        TtSc,
        TtUcc,
        UpdForQueryUcc,
        UpdQbSc,
        UpdQpSc,
        UpdToRdIdsSc,
        UpdUcc,
        VSc,
        VUcc,
        ValueSc,
        WhUcc,
    ) = (
        naming::AddOprtrSc,
        naming::AllVrtsDfltSomeOneElMaxPageSizeSc,
        naming::AllVrtsDfltSomeOneElSc,
        token_patterns::AllowClippyArbitrarySrcItemOrdering,
        naming::ColSc,
        naming::CrQbSc,
        naming::CrQpSc,
        naming::CrSc,
        naming::CrTblColQpSc,
        naming::CrUcc,
        naming::DfltSomeOneElMaxPageSizeSc,
        naming::DfltSomeOneElSc,
        naming::EqOprtrUcc,
        naming::ErSc,
        naming::IncrSc,
        naming::NormalizeSc,
        naming::OptUpdSc,
        naming::OptVecCrSc,
        token_patterns::PgCrudCmnDfltSomeOneElCall,
        naming::PgTypeEqOprtrUcc,
        naming::PgTypeNotPkUcc,
        naming::PgTypeOptVecWhGreaterThanTestSc,
        naming::PgTypeTestCasesUcc,
        naming::PgTypeUcc,
        naming::PgTypeWhFltUcc,
        naming::PreviousRdAndOptUpdIntoRdSc,
        naming::QbSc,
        naming::QpErUcc,
        naming::QpSc,
        naming::QuerySc,
        naming::RdIdsAndCrIntoOptVRdSc,
        naming::RdIdsAndCrIntoOptVecWhEqToFieldSc,
        naming::RdIdsAndCrIntoRdSc,
        naming::RdIdsAndCrIntoTtSc,
        naming::RdIdsAndCrIntoVecWhEqUsingFieldsSc,
        naming::RdIdsAndCrIntoWhEqSc,
        naming::RdIdsAndTtIntoPgTypeOptWhGreaterThanSc,
        naming::RdIdsSc,
        naming::RdIdsTo2DimsVecRdInnSc,
        naming::RdIdsToOptVRdDfltSomeOneElSc,
        naming::RdIdsUcc,
        naming::RdInnIntoRdWithNewOrTryNewUnwrapedSc,
        naming::RdInnIntoUpdWithNewOrTryNewUnwrapedSc,
        naming::RdInnUcc,
        naming::RdSc,
        naming::RdUcc,
        naming::SelOnlyIdsQpSc,
        naming::SelOnlyUpddIdsQbSc,
        naming::SelOnlyUpddIdsQpSc,
        naming::SelQpSc,
        naming::SelUcc,
        naming::SelfUcc,
        naming::TtSc,
        naming::TtUcc,
        naming::UpdForQueryUcc,
        naming::UpdQbSc,
        naming::UpdQpSc,
        naming::UpdToRdIdsSc,
        naming::UpdUcc,
        naming::VSc,
        naming::VUcc,
        naming::ValueSc,
        naming::WhUcc,
    );
    quote::quote! {
        fn #RdIdsAndCrIntoTtSc(
            #RdIdsSc: #path_ts::#RdIdsUcc,
            #CrSc: #path_ts::#CrUcc
        ) -> #path_ts::#TtUcc {
            #ts
        }
    }
    .into()
}
pub fn gen_rd_ids_and_cr_into_wh_eq_ts(
    rd_ids_ts: &dyn quote::ToTokens,
    cr_ts: &dyn quote::ToTokens,
    wh_ts: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> macros_helpers::GeneratedRustTs {
    #[allow(non_snake_case, unused_variables)]
    let (
        AddOprtrSc,
        AllVrtsDfltSomeOneElMaxPageSizeSc,
        AllVrtsDfltSomeOneElSc,
        AllowClippyArbitrarySrcItemOrdering,
        ColSc,
        CrQbSc,
        CrQpSc,
        CrSc,
        CrTblColQpSc,
        CrUcc,
        DfltSomeOneElMaxPageSizeSc,
        DfltSomeOneElSc,
        EqOprtrUcc,
        ErSc,
        IncrSc,
        NormalizeSc,
        OptUpdSc,
        OptVecCrSc,
        PgCrudCmnDfltSomeOneElCall,
        PgTypeEqOprtrUcc,
        PgTypeNotPkUcc,
        PgTypeOptVecWhGreaterThanTestSc,
        PgTypeTestCasesUcc,
        PgTypeUcc,
        PgTypeWhFltUcc,
        PreviousRdAndOptUpdIntoRdSc,
        QbSc,
        QpErUcc,
        QpSc,
        QuerySc,
        RdIdsAndCrIntoOptVRdSc,
        RdIdsAndCrIntoOptVecWhEqToFieldSc,
        RdIdsAndCrIntoRdSc,
        RdIdsAndCrIntoTtSc,
        RdIdsAndCrIntoVecWhEqUsingFieldsSc,
        RdIdsAndCrIntoWhEqSc,
        RdIdsAndTtIntoPgTypeOptWhGreaterThanSc,
        RdIdsSc,
        RdIdsTo2DimsVecRdInnSc,
        RdIdsToOptVRdDfltSomeOneElSc,
        RdIdsUcc,
        RdInnIntoRdWithNewOrTryNewUnwrapedSc,
        RdInnIntoUpdWithNewOrTryNewUnwrapedSc,
        RdInnUcc,
        RdSc,
        RdUcc,
        SelOnlyIdsQpSc,
        SelOnlyUpddIdsQbSc,
        SelOnlyUpddIdsQpSc,
        SelQpSc,
        SelUcc,
        SelfUcc,
        TtSc,
        TtUcc,
        UpdForQueryUcc,
        UpdQbSc,
        UpdQpSc,
        UpdToRdIdsSc,
        UpdUcc,
        VSc,
        VUcc,
        ValueSc,
        WhUcc,
    ) = (
        naming::AddOprtrSc,
        naming::AllVrtsDfltSomeOneElMaxPageSizeSc,
        naming::AllVrtsDfltSomeOneElSc,
        token_patterns::AllowClippyArbitrarySrcItemOrdering,
        naming::ColSc,
        naming::CrQbSc,
        naming::CrQpSc,
        naming::CrSc,
        naming::CrTblColQpSc,
        naming::CrUcc,
        naming::DfltSomeOneElMaxPageSizeSc,
        naming::DfltSomeOneElSc,
        naming::EqOprtrUcc,
        naming::ErSc,
        naming::IncrSc,
        naming::NormalizeSc,
        naming::OptUpdSc,
        naming::OptVecCrSc,
        token_patterns::PgCrudCmnDfltSomeOneElCall,
        naming::PgTypeEqOprtrUcc,
        naming::PgTypeNotPkUcc,
        naming::PgTypeOptVecWhGreaterThanTestSc,
        naming::PgTypeTestCasesUcc,
        naming::PgTypeUcc,
        naming::PgTypeWhFltUcc,
        naming::PreviousRdAndOptUpdIntoRdSc,
        naming::QbSc,
        naming::QpErUcc,
        naming::QpSc,
        naming::QuerySc,
        naming::RdIdsAndCrIntoOptVRdSc,
        naming::RdIdsAndCrIntoOptVecWhEqToFieldSc,
        naming::RdIdsAndCrIntoRdSc,
        naming::RdIdsAndCrIntoTtSc,
        naming::RdIdsAndCrIntoVecWhEqUsingFieldsSc,
        naming::RdIdsAndCrIntoWhEqSc,
        naming::RdIdsAndTtIntoPgTypeOptWhGreaterThanSc,
        naming::RdIdsSc,
        naming::RdIdsTo2DimsVecRdInnSc,
        naming::RdIdsToOptVRdDfltSomeOneElSc,
        naming::RdIdsUcc,
        naming::RdInnIntoRdWithNewOrTryNewUnwrapedSc,
        naming::RdInnIntoUpdWithNewOrTryNewUnwrapedSc,
        naming::RdInnUcc,
        naming::RdSc,
        naming::RdUcc,
        naming::SelOnlyIdsQpSc,
        naming::SelOnlyUpddIdsQbSc,
        naming::SelOnlyUpddIdsQpSc,
        naming::SelQpSc,
        naming::SelUcc,
        naming::SelfUcc,
        naming::TtSc,
        naming::TtUcc,
        naming::UpdForQueryUcc,
        naming::UpdQbSc,
        naming::UpdQpSc,
        naming::UpdToRdIdsSc,
        naming::UpdUcc,
        naming::VSc,
        naming::VUcc,
        naming::ValueSc,
        naming::WhUcc,
    );
    quote::quote! {
        fn #RdIdsAndCrIntoWhEqSc(
            #RdIdsSc: #rd_ids_ts,
            #CrSc: #cr_ts
        ) -> #wh_ts {
            #ts
        }
    }
    .into()
}
pub fn gen_rd_ids_and_cr_into_vec_wh_eq_using_fields_ts(
    import: &Import,
    rd_ids_ts: &dyn quote::ToTokens,
    cr_ts: &dyn quote::ToTokens,
    wh_ts: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> macros_helpers::GeneratedRustTs {
    #[allow(non_snake_case, unused_variables)]
    let (
        AddOprtrSc,
        AllVrtsDfltSomeOneElMaxPageSizeSc,
        AllVrtsDfltSomeOneElSc,
        AllowClippyArbitrarySrcItemOrdering,
        ColSc,
        CrQbSc,
        CrQpSc,
        CrSc,
        CrTblColQpSc,
        CrUcc,
        DfltSomeOneElMaxPageSizeSc,
        DfltSomeOneElSc,
        EqOprtrUcc,
        ErSc,
        IncrSc,
        NormalizeSc,
        OptUpdSc,
        OptVecCrSc,
        PgCrudCmnDfltSomeOneElCall,
        PgTypeEqOprtrUcc,
        PgTypeNotPkUcc,
        PgTypeOptVecWhGreaterThanTestSc,
        PgTypeTestCasesUcc,
        PgTypeUcc,
        PgTypeWhFltUcc,
        PreviousRdAndOptUpdIntoRdSc,
        QbSc,
        QpErUcc,
        QpSc,
        QuerySc,
        RdIdsAndCrIntoOptVRdSc,
        RdIdsAndCrIntoOptVecWhEqToFieldSc,
        RdIdsAndCrIntoRdSc,
        RdIdsAndCrIntoTtSc,
        RdIdsAndCrIntoVecWhEqUsingFieldsSc,
        RdIdsAndCrIntoWhEqSc,
        RdIdsAndTtIntoPgTypeOptWhGreaterThanSc,
        RdIdsSc,
        RdIdsTo2DimsVecRdInnSc,
        RdIdsToOptVRdDfltSomeOneElSc,
        RdIdsUcc,
        RdInnIntoRdWithNewOrTryNewUnwrapedSc,
        RdInnIntoUpdWithNewOrTryNewUnwrapedSc,
        RdInnUcc,
        RdSc,
        RdUcc,
        SelOnlyIdsQpSc,
        SelOnlyUpddIdsQbSc,
        SelOnlyUpddIdsQpSc,
        SelQpSc,
        SelUcc,
        SelfUcc,
        TtSc,
        TtUcc,
        UpdForQueryUcc,
        UpdQbSc,
        UpdQpSc,
        UpdToRdIdsSc,
        UpdUcc,
        VSc,
        VUcc,
        ValueSc,
        WhUcc,
    ) = (
        naming::AddOprtrSc,
        naming::AllVrtsDfltSomeOneElMaxPageSizeSc,
        naming::AllVrtsDfltSomeOneElSc,
        token_patterns::AllowClippyArbitrarySrcItemOrdering,
        naming::ColSc,
        naming::CrQbSc,
        naming::CrQpSc,
        naming::CrSc,
        naming::CrTblColQpSc,
        naming::CrUcc,
        naming::DfltSomeOneElMaxPageSizeSc,
        naming::DfltSomeOneElSc,
        naming::EqOprtrUcc,
        naming::ErSc,
        naming::IncrSc,
        naming::NormalizeSc,
        naming::OptUpdSc,
        naming::OptVecCrSc,
        token_patterns::PgCrudCmnDfltSomeOneElCall,
        naming::PgTypeEqOprtrUcc,
        naming::PgTypeNotPkUcc,
        naming::PgTypeOptVecWhGreaterThanTestSc,
        naming::PgTypeTestCasesUcc,
        naming::PgTypeUcc,
        naming::PgTypeWhFltUcc,
        naming::PreviousRdAndOptUpdIntoRdSc,
        naming::QbSc,
        naming::QpErUcc,
        naming::QpSc,
        naming::QuerySc,
        naming::RdIdsAndCrIntoOptVRdSc,
        naming::RdIdsAndCrIntoOptVecWhEqToFieldSc,
        naming::RdIdsAndCrIntoRdSc,
        naming::RdIdsAndCrIntoTtSc,
        naming::RdIdsAndCrIntoVecWhEqUsingFieldsSc,
        naming::RdIdsAndCrIntoWhEqSc,
        naming::RdIdsAndTtIntoPgTypeOptWhGreaterThanSc,
        naming::RdIdsSc,
        naming::RdIdsTo2DimsVecRdInnSc,
        naming::RdIdsToOptVRdDfltSomeOneElSc,
        naming::RdIdsUcc,
        naming::RdInnIntoRdWithNewOrTryNewUnwrapedSc,
        naming::RdInnIntoUpdWithNewOrTryNewUnwrapedSc,
        naming::RdInnUcc,
        naming::RdSc,
        naming::RdUcc,
        naming::SelOnlyIdsQpSc,
        naming::SelOnlyUpddIdsQbSc,
        naming::SelOnlyUpddIdsQpSc,
        naming::SelQpSc,
        naming::SelUcc,
        naming::SelfUcc,
        naming::TtSc,
        naming::TtUcc,
        naming::UpdForQueryUcc,
        naming::UpdQbSc,
        naming::UpdQpSc,
        naming::UpdToRdIdsSc,
        naming::UpdUcc,
        naming::VSc,
        naming::VUcc,
        naming::ValueSc,
        naming::WhUcc,
    );
    quote::quote! {
        fn #RdIdsAndCrIntoVecWhEqUsingFieldsSc(
            #RdIdsSc: #rd_ids_ts,
            #CrSc: #cr_ts
        ) -> #import::NotEmptyUnqVec<#wh_ts> {
            #ts
        }
    }
    .into()
}
#[expect(
    clippy::single_call_fn,
    reason = "keeps generated method snippets separated"
)]
fn gen_rd_ids_and_cr_into_opt_vec_wh_eq_to_field_ts(
    import: Import,
    rd_ids_ts: &dyn quote::ToTokens,
    cr_ts: &dyn quote::ToTokens,
    wh_ts: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> macros_helpers::GeneratedRustTs {
    #[allow(non_snake_case, unused_variables)]
    let (
        AddOprtrSc,
        AllVrtsDfltSomeOneElMaxPageSizeSc,
        AllVrtsDfltSomeOneElSc,
        AllowClippyArbitrarySrcItemOrdering,
        ColSc,
        CrQbSc,
        CrQpSc,
        CrSc,
        CrTblColQpSc,
        CrUcc,
        DfltSomeOneElMaxPageSizeSc,
        DfltSomeOneElSc,
        EqOprtrUcc,
        ErSc,
        IncrSc,
        NormalizeSc,
        OptUpdSc,
        OptVecCrSc,
        PgCrudCmnDfltSomeOneElCall,
        PgTypeEqOprtrUcc,
        PgTypeNotPkUcc,
        PgTypeOptVecWhGreaterThanTestSc,
        PgTypeTestCasesUcc,
        PgTypeUcc,
        PgTypeWhFltUcc,
        PreviousRdAndOptUpdIntoRdSc,
        QbSc,
        QpErUcc,
        QpSc,
        QuerySc,
        RdIdsAndCrIntoOptVRdSc,
        RdIdsAndCrIntoOptVecWhEqToFieldSc,
        RdIdsAndCrIntoRdSc,
        RdIdsAndCrIntoTtSc,
        RdIdsAndCrIntoVecWhEqUsingFieldsSc,
        RdIdsAndCrIntoWhEqSc,
        RdIdsAndTtIntoPgTypeOptWhGreaterThanSc,
        RdIdsSc,
        RdIdsTo2DimsVecRdInnSc,
        RdIdsToOptVRdDfltSomeOneElSc,
        RdIdsUcc,
        RdInnIntoRdWithNewOrTryNewUnwrapedSc,
        RdInnIntoUpdWithNewOrTryNewUnwrapedSc,
        RdInnUcc,
        RdSc,
        RdUcc,
        SelOnlyIdsQpSc,
        SelOnlyUpddIdsQbSc,
        SelOnlyUpddIdsQpSc,
        SelQpSc,
        SelUcc,
        SelfUcc,
        TtSc,
        TtUcc,
        UpdForQueryUcc,
        UpdQbSc,
        UpdQpSc,
        UpdToRdIdsSc,
        UpdUcc,
        VSc,
        VUcc,
        ValueSc,
        WhUcc,
    ) = (
        naming::AddOprtrSc,
        naming::AllVrtsDfltSomeOneElMaxPageSizeSc,
        naming::AllVrtsDfltSomeOneElSc,
        token_patterns::AllowClippyArbitrarySrcItemOrdering,
        naming::ColSc,
        naming::CrQbSc,
        naming::CrQpSc,
        naming::CrSc,
        naming::CrTblColQpSc,
        naming::CrUcc,
        naming::DfltSomeOneElMaxPageSizeSc,
        naming::DfltSomeOneElSc,
        naming::EqOprtrUcc,
        naming::ErSc,
        naming::IncrSc,
        naming::NormalizeSc,
        naming::OptUpdSc,
        naming::OptVecCrSc,
        token_patterns::PgCrudCmnDfltSomeOneElCall,
        naming::PgTypeEqOprtrUcc,
        naming::PgTypeNotPkUcc,
        naming::PgTypeOptVecWhGreaterThanTestSc,
        naming::PgTypeTestCasesUcc,
        naming::PgTypeUcc,
        naming::PgTypeWhFltUcc,
        naming::PreviousRdAndOptUpdIntoRdSc,
        naming::QbSc,
        naming::QpErUcc,
        naming::QpSc,
        naming::QuerySc,
        naming::RdIdsAndCrIntoOptVRdSc,
        naming::RdIdsAndCrIntoOptVecWhEqToFieldSc,
        naming::RdIdsAndCrIntoRdSc,
        naming::RdIdsAndCrIntoTtSc,
        naming::RdIdsAndCrIntoVecWhEqUsingFieldsSc,
        naming::RdIdsAndCrIntoWhEqSc,
        naming::RdIdsAndTtIntoPgTypeOptWhGreaterThanSc,
        naming::RdIdsSc,
        naming::RdIdsTo2DimsVecRdInnSc,
        naming::RdIdsToOptVRdDfltSomeOneElSc,
        naming::RdIdsUcc,
        naming::RdInnIntoRdWithNewOrTryNewUnwrapedSc,
        naming::RdInnIntoUpdWithNewOrTryNewUnwrapedSc,
        naming::RdInnUcc,
        naming::RdSc,
        naming::RdUcc,
        naming::SelOnlyIdsQpSc,
        naming::SelOnlyUpddIdsQbSc,
        naming::SelOnlyUpddIdsQpSc,
        naming::SelQpSc,
        naming::SelUcc,
        naming::SelfUcc,
        naming::TtSc,
        naming::TtUcc,
        naming::UpdForQueryUcc,
        naming::UpdQbSc,
        naming::UpdQpSc,
        naming::UpdToRdIdsSc,
        naming::UpdUcc,
        naming::VSc,
        naming::VUcc,
        naming::ValueSc,
        naming::WhUcc,
    );
    let return_type_ts = gen_opt_type_dcl_ts(&quote::quote! {#import::NotEmptyUnqVec<#wh_ts>});
    quote::quote! {
        fn #RdIdsAndCrIntoOptVecWhEqToFieldSc(
            #RdIdsSc: #rd_ids_ts,
            #CrSc: #cr_ts
        ) -> #return_type_ts {
            #ts
        }
    }
    .into()
}
pub fn gen_impl_pg_type_test_cases_for_ident_ts(
    cfg_ts: &dyn quote::ToTokens,
    import: &Import,
    type_ts: &dyn quote::ToTokens,
    ident: &dyn quote::ToTokens,
    opt_vec_cr_ts: Option<&macros_helpers::GeneratedRustTs>,
    rd_ids_to_2_dims_vec_rd_inn_ts: &dyn quote::ToTokens,
    rd_inn_into_rd_with_new_or_try_new_unwraped_ts: &dyn quote::ToTokens,
    rd_inn_into_upd_with_new_or_try_new_unwraped_ts: &dyn quote::ToTokens,
    upd_to_rd_ids_ts: &dyn quote::ToTokens,
    rd_ids_to_opt_v_rd_dflt_some_one_el_ts: &dyn quote::ToTokens,
    previous_rd_and_opt_upd_into_rd_ts: &dyn quote::ToTokens,
    rd_ids_and_cr_into_rd_ts: &dyn quote::ToTokens,
    rd_ids_and_cr_into_opt_v_rd_ts: &dyn quote::ToTokens,
    rd_ids_and_cr_into_tt_ts: &dyn quote::ToTokens,
    rd_ids_and_cr_into_wh_eq_ts: &dyn quote::ToTokens,
    rd_ids_and_cr_into_vec_wh_eq_using_fields_ts: &dyn quote::ToTokens,
    rd_ids_and_cr_into_opt_vec_wh_eq_to_field_ts: Option<&macros_helpers::GeneratedRustTs>,
    pg_type_opt_vec_wh_greater_than_test_ts: Option<&macros_helpers::GeneratedRustTs>,
    rd_ids_and_tt_into_pg_type_opt_wh_greater_than_ts: Option<&macros_helpers::GeneratedRustTs>,
) -> macros_helpers::GeneratedRustTs {
    #[allow(non_snake_case, unused_variables)]
    let (
        AddOprtrSc,
        AllVrtsDfltSomeOneElMaxPageSizeSc,
        AllVrtsDfltSomeOneElSc,
        AllowClippyArbitrarySrcItemOrdering,
        ColSc,
        CrQbSc,
        CrQpSc,
        CrSc,
        CrTblColQpSc,
        CrUcc,
        DfltSomeOneElMaxPageSizeSc,
        DfltSomeOneElSc,
        EqOprtrUcc,
        ErSc,
        IncrSc,
        NormalizeSc,
        OptUpdSc,
        OptVecCrSc,
        PgCrudCmnDfltSomeOneElCall,
        PgTypeEqOprtrUcc,
        PgTypeNotPkUcc,
        PgTypeOptVecWhGreaterThanTestSc,
        PgTypeTestCasesUcc,
        PgTypeUcc,
        PgTypeWhFltUcc,
        PreviousRdAndOptUpdIntoRdSc,
        QbSc,
        QpErUcc,
        QpSc,
        QuerySc,
        RdIdsAndCrIntoOptVRdSc,
        RdIdsAndCrIntoOptVecWhEqToFieldSc,
        RdIdsAndCrIntoRdSc,
        RdIdsAndCrIntoTtSc,
        RdIdsAndCrIntoVecWhEqUsingFieldsSc,
        RdIdsAndCrIntoWhEqSc,
        RdIdsAndTtIntoPgTypeOptWhGreaterThanSc,
        RdIdsSc,
        RdIdsTo2DimsVecRdInnSc,
        RdIdsToOptVRdDfltSomeOneElSc,
        RdIdsUcc,
        RdInnIntoRdWithNewOrTryNewUnwrapedSc,
        RdInnIntoUpdWithNewOrTryNewUnwrapedSc,
        RdInnUcc,
        RdSc,
        RdUcc,
        SelOnlyIdsQpSc,
        SelOnlyUpddIdsQbSc,
        SelOnlyUpddIdsQpSc,
        SelQpSc,
        SelUcc,
        SelfUcc,
        TtSc,
        TtUcc,
        UpdForQueryUcc,
        UpdQbSc,
        UpdQpSc,
        UpdToRdIdsSc,
        UpdUcc,
        VSc,
        VUcc,
        ValueSc,
        WhUcc,
    ) = (
        naming::AddOprtrSc,
        naming::AllVrtsDfltSomeOneElMaxPageSizeSc,
        naming::AllVrtsDfltSomeOneElSc,
        token_patterns::AllowClippyArbitrarySrcItemOrdering,
        naming::ColSc,
        naming::CrQbSc,
        naming::CrQpSc,
        naming::CrSc,
        naming::CrTblColQpSc,
        naming::CrUcc,
        naming::DfltSomeOneElMaxPageSizeSc,
        naming::DfltSomeOneElSc,
        naming::EqOprtrUcc,
        naming::ErSc,
        naming::IncrSc,
        naming::NormalizeSc,
        naming::OptUpdSc,
        naming::OptVecCrSc,
        token_patterns::PgCrudCmnDfltSomeOneElCall,
        naming::PgTypeEqOprtrUcc,
        naming::PgTypeNotPkUcc,
        naming::PgTypeOptVecWhGreaterThanTestSc,
        naming::PgTypeTestCasesUcc,
        naming::PgTypeUcc,
        naming::PgTypeWhFltUcc,
        naming::PreviousRdAndOptUpdIntoRdSc,
        naming::QbSc,
        naming::QpErUcc,
        naming::QpSc,
        naming::QuerySc,
        naming::RdIdsAndCrIntoOptVRdSc,
        naming::RdIdsAndCrIntoOptVecWhEqToFieldSc,
        naming::RdIdsAndCrIntoRdSc,
        naming::RdIdsAndCrIntoTtSc,
        naming::RdIdsAndCrIntoVecWhEqUsingFieldsSc,
        naming::RdIdsAndCrIntoWhEqSc,
        naming::RdIdsAndTtIntoPgTypeOptWhGreaterThanSc,
        naming::RdIdsSc,
        naming::RdIdsTo2DimsVecRdInnSc,
        naming::RdIdsToOptVRdDfltSomeOneElSc,
        naming::RdIdsUcc,
        naming::RdInnIntoRdWithNewOrTryNewUnwrapedSc,
        naming::RdInnIntoUpdWithNewOrTryNewUnwrapedSc,
        naming::RdInnUcc,
        naming::RdSc,
        naming::RdUcc,
        naming::SelOnlyIdsQpSc,
        naming::SelOnlyUpddIdsQbSc,
        naming::SelOnlyUpddIdsQpSc,
        naming::SelQpSc,
        naming::SelUcc,
        naming::SelfUcc,
        naming::TtSc,
        naming::TtUcc,
        naming::UpdForQueryUcc,
        naming::UpdQbSc,
        naming::UpdQpSc,
        naming::UpdToRdIdsSc,
        naming::UpdUcc,
        naming::VSc,
        naming::VUcc,
        naming::ValueSc,
        naming::WhUcc,
    );
    let self_pg_type_as_pg_type_ts = quote::quote! {<#SelfUcc::#PgTypeUcc as #import::#PgTypeUcc>};
    let self_pg_type_as_pg_type_rd_ids_ts = quote::quote! {#self_pg_type_as_pg_type_ts::#RdIdsUcc};
    let self_pg_type_as_pg_type_cr_ts = quote::quote! {#self_pg_type_as_pg_type_ts::#CrUcc};
    let self_pg_type_as_pg_type_wh_ts = quote::quote! {#self_pg_type_as_pg_type_ts::#WhUcc};
    let ident_sel_ucc = naming::prm::SelfSelUcc::from_tokens(&ident);
    let opt_vec_cr_ts_gnrtd =
        opt_vec_cr_ts.map(|ts| gen_opt_vec_cr_ts(&self_pg_type_as_pg_type_ts, ts));
    let rd_ids_to_2_dims_vec_rd_inn_ts_gnrtd = gen_rd_ids_to_2_dims_vec_rd_inn_ts(
        &self_pg_type_as_pg_type_ts,
        &rd_ids_to_2_dims_vec_rd_inn_ts,
    );
    let rd_inn_into_rd_with_new_or_try_new_unwraped_ts_gnrtd =
        gen_rd_inn_into_rd_or_upd_with_new_or_try_new_unwraped_ts(
            &RdInnIntoRdWithNewOrTryNewUnwrapedSc,
            &type_ts,
            &self_pg_type_as_pg_type_ts,
            &naming::RdUcc,
            &rd_inn_into_rd_with_new_or_try_new_unwraped_ts,
        );
    let rd_inn_into_upd_with_new_or_try_new_unwraped_ts_gnrtd =
        gen_rd_inn_into_rd_or_upd_with_new_or_try_new_unwraped_ts(
            &RdInnIntoUpdWithNewOrTryNewUnwrapedSc,
            &type_ts,
            &self_pg_type_as_pg_type_ts,
            &naming::UpdUcc,
            &rd_inn_into_upd_with_new_or_try_new_unwraped_ts,
        );
    let upd_to_rd_ids_ts_gnrtd =
        gen_upd_to_rd_ids_ts(&self_pg_type_as_pg_type_ts, &upd_to_rd_ids_ts);
    let rd_ids_to_opt_v_rd_dflt_some_one_el_ts_gnrtd = gen_rd_ids_to_opt_v_rd_dflt_some_one_el_ts(
        *import,
        &self_pg_type_as_pg_type_ts,
        &rd_ids_to_opt_v_rd_dflt_some_one_el_ts,
    );
    let previous_rd_and_opt_upd_into_rd_ts_gnrtd = gen_previous_rd_and_opt_upd_into_rd_ts(
        &self_pg_type_as_pg_type_ts,
        &previous_rd_and_opt_upd_into_rd_ts,
    );
    let rd_ids_and_cr_into_rd_ts_gnrtd =
        gen_rd_ids_and_cr_into_rd_ts(&self_pg_type_as_pg_type_ts, &rd_ids_and_cr_into_rd_ts);
    let rd_ids_and_cr_into_opt_v_rd_ts_gnrtd = gen_rd_ids_and_cr_into_opt_v_rd_ts(
        *import,
        &self_pg_type_as_pg_type_ts,
        &rd_ids_and_cr_into_opt_v_rd_ts,
    );
    let rd_ids_and_cr_into_tt_ts_gnrtd =
        gen_rd_ids_and_cr_into_tt_ts(&self_pg_type_as_pg_type_ts, &rd_ids_and_cr_into_tt_ts);
    let rd_ids_and_cr_into_wh_eq_ts_gnrtd = gen_rd_ids_and_cr_into_wh_eq_ts(
        &self_pg_type_as_pg_type_rd_ids_ts,
        &self_pg_type_as_pg_type_cr_ts,
        &self_pg_type_as_pg_type_wh_ts,
        &rd_ids_and_cr_into_wh_eq_ts,
    );
    let rd_ids_and_cr_into_vec_wh_eq_using_fields_ts_gnrtd =
        gen_rd_ids_and_cr_into_vec_wh_eq_using_fields_ts(
            import,
            &self_pg_type_as_pg_type_rd_ids_ts,
            &self_pg_type_as_pg_type_cr_ts,
            &self_pg_type_as_pg_type_wh_ts,
            &rd_ids_and_cr_into_vec_wh_eq_using_fields_ts,
        );
    let rd_ids_and_cr_into_opt_vec_wh_eq_to_field_ts_gnrtd =
        rd_ids_and_cr_into_opt_vec_wh_eq_to_field_ts.map(|ts| {
            gen_rd_ids_and_cr_into_opt_vec_wh_eq_to_field_ts(
                *import,
                &self_pg_type_as_pg_type_rd_ids_ts,
                &self_pg_type_as_pg_type_cr_ts,
                &self_pg_type_as_pg_type_wh_ts,
                ts,
            )
        });
    let pg_type_opt_vec_wh_greater_than_test_ts_gnrtd = pg_type_opt_vec_wh_greater_than_test_ts
        .map(|ts| {
            quote::quote! {
                fn #PgTypeOptVecWhGreaterThanTestSc() -> Option<
                    #import::NotEmptyUnqVec<
                        #import::PgTypeGreaterThanTest<
                            #SelfUcc::#PgTypeUcc
                        >
                    >
                > {
                    #ts
                }
            }
        });
    let rd_ids_and_tt_into_pg_type_opt_wh_greater_than_ts_gnrtd =
        rd_ids_and_tt_into_pg_type_opt_wh_greater_than_ts.map(|ts| {
            let rd_ids_and_tt_into_pg_type_opt_wh_greater_than_sc =
                RdIdsAndTtIntoPgTypeOptWhGreaterThanSc;
            quote::quote! {
                fn #rd_ids_and_tt_into_pg_type_opt_wh_greater_than_sc(
                    greater_than_vrt: #import::PgTypeGreaterThanVrt,
                    #RdIdsSc: #self_pg_type_as_pg_type_ts::#RdIdsUcc,
                    #TtSc: #self_pg_type_as_pg_type_ts::#TtUcc,
                ) -> Option<#self_pg_type_as_pg_type_ts::#WhUcc> {
                    #ts
                }
            }
        });
    quote::quote! {
        #[allow(unused_qualifications)]
        #[allow(clippy::absolute_paths)]
        #AllowClippyArbitrarySrcItemOrdering
        #cfg_ts
        #[allow(clippy::float_arithmetic)]
        impl #import::#PgTypeTestCasesUcc for #ident {
            type #PgTypeUcc = #SelfUcc;
            type #SelUcc = #ident_sel_ucc;
            #opt_vec_cr_ts_gnrtd
            #rd_ids_to_2_dims_vec_rd_inn_ts_gnrtd
            #rd_inn_into_rd_with_new_or_try_new_unwraped_ts_gnrtd
            #rd_inn_into_upd_with_new_or_try_new_unwraped_ts_gnrtd
            #upd_to_rd_ids_ts_gnrtd
            #rd_ids_to_opt_v_rd_dflt_some_one_el_ts_gnrtd
            #previous_rd_and_opt_upd_into_rd_ts_gnrtd
            #rd_ids_and_cr_into_rd_ts_gnrtd
            #rd_ids_and_cr_into_opt_v_rd_ts_gnrtd
            #rd_ids_and_cr_into_tt_ts_gnrtd
            #rd_ids_and_cr_into_wh_eq_ts_gnrtd
            #rd_ids_and_cr_into_vec_wh_eq_using_fields_ts_gnrtd
            #rd_ids_and_cr_into_opt_vec_wh_eq_to_field_ts_gnrtd
            #pg_type_opt_vec_wh_greater_than_test_ts_gnrtd
            #rd_ids_and_tt_into_pg_type_opt_wh_greater_than_ts_gnrtd
        }
    }
    .into()
}
#[must_use]
pub fn pg_crud_cmn_qp_er_checked_add_init_ts() -> macros_helpers::GeneratedRustTs {
    quote::quote! {pg_crud_cmn::QpEr::CheckedAdd { loc: loc_lib::loc!() }}.into()
}
pub fn gen_impl_crate_is_string_empty_for_ident_ts(
    ident: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> macros_helpers::GeneratedRustTs {
    quote::quote! {
        impl pg_crud_cmn::IsStringEmpty for #ident {
            fn is_string_empty(&self) -> pg_crud_cmn::IsStringEmptyRes {
                pg_crud_cmn::IsStringEmptyRes::from(#ts)
            }
        }
    }
    .into()
}
pub fn gen_match_try_new_in_de_ts(
    ident: &dyn quote::ToTokens,
    init_ts: &dyn quote::ToTokens,
) -> macros_helpers::GeneratedRustTs {
    quote::quote! {
        match #ident::try_new(#init_ts) {
            Ok(v) => Ok(v),
            Err(er) => Err(serde::de::Error::custom(format!("{er:?}")))
        }
    }
    .into()
}
pub fn gen_impl_de_for_struct_ts(
    ident: &dyn naming::DisplayPlusToTokens,
    vec_ident_type: SynIdentTypeRefs<'_>,
    _len: DeLen,
    gen_type_ts: &dyn Fn(&syn::Ident, &syn::Type) -> macros_helpers::GeneratedRustTs,
) -> macros_helpers::GeneratedRustTs {
    let allow_clippy_arbitrary_src_item_ordering =
        token_patterns::AllowClippyArbitrarySrcItemOrdering;
    let raw_ident_ts = parse_ts_or_compile_error(
        ParseTsTextRef::from(format!("{ident}Raw").as_str()),
        ParseErIdRef::from("a1b2c3d4"),
    );
    let raw_fields_ts = vec_ident_type.0.iter().map(|(fi, ty)| {
        let type_ts = gen_type_ts(fi, ty);
        quote::quote! { #fi: #type_ts, }
    });
    let try_from_fields_ts = vec_ident_type.0.iter().map(|(fi, _)| {
        quote::quote! { raw.#fi }
    });
    quote::quote! {
        #[derive(serde::Deserialize)]
        #[allow(clippy::arbitrary_source_item_ordering)]
        struct #raw_ident_ts {
            #(#raw_fields_ts)*
        }
        #[allow(unused_qualifications)]
        #[allow(clippy::absolute_paths)]
        #allow_clippy_arbitrary_src_item_ordering
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for #ident {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    let raw = <#raw_ident_ts as _serde::Deserialize>::deserialize(__deserializer)?;
                    Self::try_new(#(#try_from_fields_ts),*).map_err(|er| _serde::de::Error::custom(format!("{er:?}")))
                }
            }
        };
    }.into()
}
pub fn wrap_into_scopes_ts(ts: &dyn quote::ToTokens) -> macros_helpers::GeneratedRustTs {
    quote::quote! {(#ts)}.into()
}
pub fn mb_wrap_into_braces_ts(
    ts: &dyn quote::ToTokens,
    wrap: WrapIntoBraces,
) -> macros_helpers::GeneratedRustTs {
    if bool::from(wrap) {
        wrap_into_scopes_ts(&ts)
    } else {
        quote::quote! {#ts}.into()
    }
}
pub fn gen_v_dcl_ts(import: &Import, ts: &dyn quote::ToTokens) -> macros_helpers::GeneratedRustTs {
    #[allow(non_snake_case, unused_variables)]
    let (
        AddOprtrSc,
        AllVrtsDfltSomeOneElMaxPageSizeSc,
        AllVrtsDfltSomeOneElSc,
        AllowClippyArbitrarySrcItemOrdering,
        ColSc,
        CrQbSc,
        CrQpSc,
        CrSc,
        CrTblColQpSc,
        CrUcc,
        DfltSomeOneElMaxPageSizeSc,
        DfltSomeOneElSc,
        EqOprtrUcc,
        ErSc,
        IncrSc,
        NormalizeSc,
        OptUpdSc,
        OptVecCrSc,
        PgCrudCmnDfltSomeOneElCall,
        PgTypeEqOprtrUcc,
        PgTypeNotPkUcc,
        PgTypeOptVecWhGreaterThanTestSc,
        PgTypeTestCasesUcc,
        PgTypeUcc,
        PgTypeWhFltUcc,
        PreviousRdAndOptUpdIntoRdSc,
        QbSc,
        QpErUcc,
        QpSc,
        QuerySc,
        RdIdsAndCrIntoOptVRdSc,
        RdIdsAndCrIntoOptVecWhEqToFieldSc,
        RdIdsAndCrIntoRdSc,
        RdIdsAndCrIntoTtSc,
        RdIdsAndCrIntoVecWhEqUsingFieldsSc,
        RdIdsAndCrIntoWhEqSc,
        RdIdsAndTtIntoPgTypeOptWhGreaterThanSc,
        RdIdsSc,
        RdIdsTo2DimsVecRdInnSc,
        RdIdsToOptVRdDfltSomeOneElSc,
        RdIdsUcc,
        RdInnIntoRdWithNewOrTryNewUnwrapedSc,
        RdInnIntoUpdWithNewOrTryNewUnwrapedSc,
        RdInnUcc,
        RdSc,
        RdUcc,
        SelOnlyIdsQpSc,
        SelOnlyUpddIdsQbSc,
        SelOnlyUpddIdsQpSc,
        SelQpSc,
        SelUcc,
        SelfUcc,
        TtSc,
        TtUcc,
        UpdForQueryUcc,
        UpdQbSc,
        UpdQpSc,
        UpdToRdIdsSc,
        UpdUcc,
        VSc,
        VUcc,
        ValueSc,
        WhUcc,
    ) = (
        naming::AddOprtrSc,
        naming::AllVrtsDfltSomeOneElMaxPageSizeSc,
        naming::AllVrtsDfltSomeOneElSc,
        token_patterns::AllowClippyArbitrarySrcItemOrdering,
        naming::ColSc,
        naming::CrQbSc,
        naming::CrQpSc,
        naming::CrSc,
        naming::CrTblColQpSc,
        naming::CrUcc,
        naming::DfltSomeOneElMaxPageSizeSc,
        naming::DfltSomeOneElSc,
        naming::EqOprtrUcc,
        naming::ErSc,
        naming::IncrSc,
        naming::NormalizeSc,
        naming::OptUpdSc,
        naming::OptVecCrSc,
        token_patterns::PgCrudCmnDfltSomeOneElCall,
        naming::PgTypeEqOprtrUcc,
        naming::PgTypeNotPkUcc,
        naming::PgTypeOptVecWhGreaterThanTestSc,
        naming::PgTypeTestCasesUcc,
        naming::PgTypeUcc,
        naming::PgTypeWhFltUcc,
        naming::PreviousRdAndOptUpdIntoRdSc,
        naming::QbSc,
        naming::QpErUcc,
        naming::QpSc,
        naming::QuerySc,
        naming::RdIdsAndCrIntoOptVRdSc,
        naming::RdIdsAndCrIntoOptVecWhEqToFieldSc,
        naming::RdIdsAndCrIntoRdSc,
        naming::RdIdsAndCrIntoTtSc,
        naming::RdIdsAndCrIntoVecWhEqUsingFieldsSc,
        naming::RdIdsAndCrIntoWhEqSc,
        naming::RdIdsAndTtIntoPgTypeOptWhGreaterThanSc,
        naming::RdIdsSc,
        naming::RdIdsTo2DimsVecRdInnSc,
        naming::RdIdsToOptVRdDfltSomeOneElSc,
        naming::RdIdsUcc,
        naming::RdInnIntoRdWithNewOrTryNewUnwrapedSc,
        naming::RdInnIntoUpdWithNewOrTryNewUnwrapedSc,
        naming::RdInnUcc,
        naming::RdSc,
        naming::RdUcc,
        naming::SelOnlyIdsQpSc,
        naming::SelOnlyUpddIdsQbSc,
        naming::SelOnlyUpddIdsQpSc,
        naming::SelQpSc,
        naming::SelUcc,
        naming::SelfUcc,
        naming::TtSc,
        naming::TtUcc,
        naming::UpdForQueryUcc,
        naming::UpdQbSc,
        naming::UpdQpSc,
        naming::UpdToRdIdsSc,
        naming::UpdUcc,
        naming::VSc,
        naming::VUcc,
        naming::ValueSc,
        naming::WhUcc,
    );
    quote::quote! {#import::V<#ts>}.into()
}
pub fn gen_v_init_ts(import: &Import, ts: &dyn quote::ToTokens) -> macros_helpers::GeneratedRustTs {
    #[allow(non_snake_case, unused_variables)]
    let (
        AddOprtrSc,
        AllVrtsDfltSomeOneElMaxPageSizeSc,
        AllVrtsDfltSomeOneElSc,
        AllowClippyArbitrarySrcItemOrdering,
        ColSc,
        CrQbSc,
        CrQpSc,
        CrSc,
        CrTblColQpSc,
        CrUcc,
        DfltSomeOneElMaxPageSizeSc,
        DfltSomeOneElSc,
        EqOprtrUcc,
        ErSc,
        IncrSc,
        NormalizeSc,
        OptUpdSc,
        OptVecCrSc,
        PgCrudCmnDfltSomeOneElCall,
        PgTypeEqOprtrUcc,
        PgTypeNotPkUcc,
        PgTypeOptVecWhGreaterThanTestSc,
        PgTypeTestCasesUcc,
        PgTypeUcc,
        PgTypeWhFltUcc,
        PreviousRdAndOptUpdIntoRdSc,
        QbSc,
        QpErUcc,
        QpSc,
        QuerySc,
        RdIdsAndCrIntoOptVRdSc,
        RdIdsAndCrIntoOptVecWhEqToFieldSc,
        RdIdsAndCrIntoRdSc,
        RdIdsAndCrIntoTtSc,
        RdIdsAndCrIntoVecWhEqUsingFieldsSc,
        RdIdsAndCrIntoWhEqSc,
        RdIdsAndTtIntoPgTypeOptWhGreaterThanSc,
        RdIdsSc,
        RdIdsTo2DimsVecRdInnSc,
        RdIdsToOptVRdDfltSomeOneElSc,
        RdIdsUcc,
        RdInnIntoRdWithNewOrTryNewUnwrapedSc,
        RdInnIntoUpdWithNewOrTryNewUnwrapedSc,
        RdInnUcc,
        RdSc,
        RdUcc,
        SelOnlyIdsQpSc,
        SelOnlyUpddIdsQbSc,
        SelOnlyUpddIdsQpSc,
        SelQpSc,
        SelUcc,
        SelfUcc,
        TtSc,
        TtUcc,
        UpdForQueryUcc,
        UpdQbSc,
        UpdQpSc,
        UpdToRdIdsSc,
        UpdUcc,
        VSc,
        VUcc,
        ValueSc,
        WhUcc,
    ) = (
        naming::AddOprtrSc,
        naming::AllVrtsDfltSomeOneElMaxPageSizeSc,
        naming::AllVrtsDfltSomeOneElSc,
        token_patterns::AllowClippyArbitrarySrcItemOrdering,
        naming::ColSc,
        naming::CrQbSc,
        naming::CrQpSc,
        naming::CrSc,
        naming::CrTblColQpSc,
        naming::CrUcc,
        naming::DfltSomeOneElMaxPageSizeSc,
        naming::DfltSomeOneElSc,
        naming::EqOprtrUcc,
        naming::ErSc,
        naming::IncrSc,
        naming::NormalizeSc,
        naming::OptUpdSc,
        naming::OptVecCrSc,
        token_patterns::PgCrudCmnDfltSomeOneElCall,
        naming::PgTypeEqOprtrUcc,
        naming::PgTypeNotPkUcc,
        naming::PgTypeOptVecWhGreaterThanTestSc,
        naming::PgTypeTestCasesUcc,
        naming::PgTypeUcc,
        naming::PgTypeWhFltUcc,
        naming::PreviousRdAndOptUpdIntoRdSc,
        naming::QbSc,
        naming::QpErUcc,
        naming::QpSc,
        naming::QuerySc,
        naming::RdIdsAndCrIntoOptVRdSc,
        naming::RdIdsAndCrIntoOptVecWhEqToFieldSc,
        naming::RdIdsAndCrIntoRdSc,
        naming::RdIdsAndCrIntoTtSc,
        naming::RdIdsAndCrIntoVecWhEqUsingFieldsSc,
        naming::RdIdsAndCrIntoWhEqSc,
        naming::RdIdsAndTtIntoPgTypeOptWhGreaterThanSc,
        naming::RdIdsSc,
        naming::RdIdsTo2DimsVecRdInnSc,
        naming::RdIdsToOptVRdDfltSomeOneElSc,
        naming::RdIdsUcc,
        naming::RdInnIntoRdWithNewOrTryNewUnwrapedSc,
        naming::RdInnIntoUpdWithNewOrTryNewUnwrapedSc,
        naming::RdInnUcc,
        naming::RdSc,
        naming::RdUcc,
        naming::SelOnlyIdsQpSc,
        naming::SelOnlyUpddIdsQbSc,
        naming::SelOnlyUpddIdsQpSc,
        naming::SelQpSc,
        naming::SelUcc,
        naming::SelfUcc,
        naming::TtSc,
        naming::TtUcc,
        naming::UpdForQueryUcc,
        naming::UpdQbSc,
        naming::UpdQpSc,
        naming::UpdToRdIdsSc,
        naming::UpdUcc,
        naming::VSc,
        naming::VUcc,
        naming::ValueSc,
        naming::WhUcc,
    );
    quote::quote! {#import::V { #VSc: #ts }}.into()
}
pub fn impl_pg_type_eq_oprtr_for_ident_ts(
    import: &Import,
    ident: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> macros_helpers::GeneratedRustTs {
    #[allow(non_snake_case, unused_variables)]
    let (
        AddOprtrSc,
        AllVrtsDfltSomeOneElMaxPageSizeSc,
        AllVrtsDfltSomeOneElSc,
        AllowClippyArbitrarySrcItemOrdering,
        ColSc,
        CrQbSc,
        CrQpSc,
        CrSc,
        CrTblColQpSc,
        CrUcc,
        DfltSomeOneElMaxPageSizeSc,
        DfltSomeOneElSc,
        EqOprtrUcc,
        ErSc,
        IncrSc,
        NormalizeSc,
        OptUpdSc,
        OptVecCrSc,
        PgCrudCmnDfltSomeOneElCall,
        PgTypeEqOprtrUcc,
        PgTypeNotPkUcc,
        PgTypeOptVecWhGreaterThanTestSc,
        PgTypeTestCasesUcc,
        PgTypeUcc,
        PgTypeWhFltUcc,
        PreviousRdAndOptUpdIntoRdSc,
        QbSc,
        QpErUcc,
        QpSc,
        QuerySc,
        RdIdsAndCrIntoOptVRdSc,
        RdIdsAndCrIntoOptVecWhEqToFieldSc,
        RdIdsAndCrIntoRdSc,
        RdIdsAndCrIntoTtSc,
        RdIdsAndCrIntoVecWhEqUsingFieldsSc,
        RdIdsAndCrIntoWhEqSc,
        RdIdsAndTtIntoPgTypeOptWhGreaterThanSc,
        RdIdsSc,
        RdIdsTo2DimsVecRdInnSc,
        RdIdsToOptVRdDfltSomeOneElSc,
        RdIdsUcc,
        RdInnIntoRdWithNewOrTryNewUnwrapedSc,
        RdInnIntoUpdWithNewOrTryNewUnwrapedSc,
        RdInnUcc,
        RdSc,
        RdUcc,
        SelOnlyIdsQpSc,
        SelOnlyUpddIdsQbSc,
        SelOnlyUpddIdsQpSc,
        SelQpSc,
        SelUcc,
        SelfUcc,
        TtSc,
        TtUcc,
        UpdForQueryUcc,
        UpdQbSc,
        UpdQpSc,
        UpdToRdIdsSc,
        UpdUcc,
        VSc,
        VUcc,
        ValueSc,
        WhUcc,
    ) = (
        naming::AddOprtrSc,
        naming::AllVrtsDfltSomeOneElMaxPageSizeSc,
        naming::AllVrtsDfltSomeOneElSc,
        token_patterns::AllowClippyArbitrarySrcItemOrdering,
        naming::ColSc,
        naming::CrQbSc,
        naming::CrQpSc,
        naming::CrSc,
        naming::CrTblColQpSc,
        naming::CrUcc,
        naming::DfltSomeOneElMaxPageSizeSc,
        naming::DfltSomeOneElSc,
        naming::EqOprtrUcc,
        naming::ErSc,
        naming::IncrSc,
        naming::NormalizeSc,
        naming::OptUpdSc,
        naming::OptVecCrSc,
        token_patterns::PgCrudCmnDfltSomeOneElCall,
        naming::PgTypeEqOprtrUcc,
        naming::PgTypeNotPkUcc,
        naming::PgTypeOptVecWhGreaterThanTestSc,
        naming::PgTypeTestCasesUcc,
        naming::PgTypeUcc,
        naming::PgTypeWhFltUcc,
        naming::PreviousRdAndOptUpdIntoRdSc,
        naming::QbSc,
        naming::QpErUcc,
        naming::QpSc,
        naming::QuerySc,
        naming::RdIdsAndCrIntoOptVRdSc,
        naming::RdIdsAndCrIntoOptVecWhEqToFieldSc,
        naming::RdIdsAndCrIntoRdSc,
        naming::RdIdsAndCrIntoTtSc,
        naming::RdIdsAndCrIntoVecWhEqUsingFieldsSc,
        naming::RdIdsAndCrIntoWhEqSc,
        naming::RdIdsAndTtIntoPgTypeOptWhGreaterThanSc,
        naming::RdIdsSc,
        naming::RdIdsTo2DimsVecRdInnSc,
        naming::RdIdsToOptVRdDfltSomeOneElSc,
        naming::RdIdsUcc,
        naming::RdInnIntoRdWithNewOrTryNewUnwrapedSc,
        naming::RdInnIntoUpdWithNewOrTryNewUnwrapedSc,
        naming::RdInnUcc,
        naming::RdSc,
        naming::RdUcc,
        naming::SelOnlyIdsQpSc,
        naming::SelOnlyUpddIdsQbSc,
        naming::SelOnlyUpddIdsQpSc,
        naming::SelQpSc,
        naming::SelUcc,
        naming::SelfUcc,
        naming::TtSc,
        naming::TtUcc,
        naming::UpdForQueryUcc,
        naming::UpdQbSc,
        naming::UpdQpSc,
        naming::UpdToRdIdsSc,
        naming::UpdUcc,
        naming::VSc,
        naming::VUcc,
        naming::ValueSc,
        naming::WhUcc,
    );
    quote::quote! {
        impl #import::#PgTypeEqOprtrUcc for #ident {
            fn oprtr(&self) -> #import::#EqOprtrUcc {
                #ts
            }
        }
    }
    .into()
}
#[must_use]
pub fn gen_qp_er_write_into_buffer_ts(import: Import) -> macros_helpers::GeneratedRustTs {
    #[allow(non_snake_case, unused_variables)]
    let (
        AddOprtrSc,
        AllVrtsDfltSomeOneElMaxPageSizeSc,
        AllVrtsDfltSomeOneElSc,
        AllowClippyArbitrarySrcItemOrdering,
        ColSc,
        CrQbSc,
        CrQpSc,
        CrSc,
        CrTblColQpSc,
        CrUcc,
        DfltSomeOneElMaxPageSizeSc,
        DfltSomeOneElSc,
        EqOprtrUcc,
        ErSc,
        IncrSc,
        NormalizeSc,
        OptUpdSc,
        OptVecCrSc,
        PgCrudCmnDfltSomeOneElCall,
        PgTypeEqOprtrUcc,
        PgTypeNotPkUcc,
        PgTypeOptVecWhGreaterThanTestSc,
        PgTypeTestCasesUcc,
        PgTypeUcc,
        PgTypeWhFltUcc,
        PreviousRdAndOptUpdIntoRdSc,
        QbSc,
        QpErUcc,
        QpSc,
        QuerySc,
        RdIdsAndCrIntoOptVRdSc,
        RdIdsAndCrIntoOptVecWhEqToFieldSc,
        RdIdsAndCrIntoRdSc,
        RdIdsAndCrIntoTtSc,
        RdIdsAndCrIntoVecWhEqUsingFieldsSc,
        RdIdsAndCrIntoWhEqSc,
        RdIdsAndTtIntoPgTypeOptWhGreaterThanSc,
        RdIdsSc,
        RdIdsTo2DimsVecRdInnSc,
        RdIdsToOptVRdDfltSomeOneElSc,
        RdIdsUcc,
        RdInnIntoRdWithNewOrTryNewUnwrapedSc,
        RdInnIntoUpdWithNewOrTryNewUnwrapedSc,
        RdInnUcc,
        RdSc,
        RdUcc,
        SelOnlyIdsQpSc,
        SelOnlyUpddIdsQbSc,
        SelOnlyUpddIdsQpSc,
        SelQpSc,
        SelUcc,
        SelfUcc,
        TtSc,
        TtUcc,
        UpdForQueryUcc,
        UpdQbSc,
        UpdQpSc,
        UpdToRdIdsSc,
        UpdUcc,
        VSc,
        VUcc,
        ValueSc,
        WhUcc,
    ) = (
        naming::AddOprtrSc,
        naming::AllVrtsDfltSomeOneElMaxPageSizeSc,
        naming::AllVrtsDfltSomeOneElSc,
        token_patterns::AllowClippyArbitrarySrcItemOrdering,
        naming::ColSc,
        naming::CrQbSc,
        naming::CrQpSc,
        naming::CrSc,
        naming::CrTblColQpSc,
        naming::CrUcc,
        naming::DfltSomeOneElMaxPageSizeSc,
        naming::DfltSomeOneElSc,
        naming::EqOprtrUcc,
        naming::ErSc,
        naming::IncrSc,
        naming::NormalizeSc,
        naming::OptUpdSc,
        naming::OptVecCrSc,
        token_patterns::PgCrudCmnDfltSomeOneElCall,
        naming::PgTypeEqOprtrUcc,
        naming::PgTypeNotPkUcc,
        naming::PgTypeOptVecWhGreaterThanTestSc,
        naming::PgTypeTestCasesUcc,
        naming::PgTypeUcc,
        naming::PgTypeWhFltUcc,
        naming::PreviousRdAndOptUpdIntoRdSc,
        naming::QbSc,
        naming::QpErUcc,
        naming::QpSc,
        naming::QuerySc,
        naming::RdIdsAndCrIntoOptVRdSc,
        naming::RdIdsAndCrIntoOptVecWhEqToFieldSc,
        naming::RdIdsAndCrIntoRdSc,
        naming::RdIdsAndCrIntoTtSc,
        naming::RdIdsAndCrIntoVecWhEqUsingFieldsSc,
        naming::RdIdsAndCrIntoWhEqSc,
        naming::RdIdsAndTtIntoPgTypeOptWhGreaterThanSc,
        naming::RdIdsSc,
        naming::RdIdsTo2DimsVecRdInnSc,
        naming::RdIdsToOptVRdDfltSomeOneElSc,
        naming::RdIdsUcc,
        naming::RdInnIntoRdWithNewOrTryNewUnwrapedSc,
        naming::RdInnIntoUpdWithNewOrTryNewUnwrapedSc,
        naming::RdInnUcc,
        naming::RdSc,
        naming::RdUcc,
        naming::SelOnlyIdsQpSc,
        naming::SelOnlyUpddIdsQbSc,
        naming::SelOnlyUpddIdsQpSc,
        naming::SelQpSc,
        naming::SelUcc,
        naming::SelfUcc,
        naming::TtSc,
        naming::TtUcc,
        naming::UpdForQueryUcc,
        naming::UpdQbSc,
        naming::UpdQpSc,
        naming::UpdToRdIdsSc,
        naming::UpdUcc,
        naming::VSc,
        naming::VUcc,
        naming::ValueSc,
        naming::WhUcc,
    );
    quote::quote! {
        #import::QpEr::WriteIntoBuffer {
            loc: loc_lib::loc!()
        }
    }
    .into()
}
#[must_use]
pub fn gen_return_err_qp_er_write_into_buffer_ts(
    import: Import,
) -> macros_helpers::GeneratedRustTs {
    #[allow(non_snake_case, unused_variables)]
    let (
        AddOprtrSc,
        AllVrtsDfltSomeOneElMaxPageSizeSc,
        AllVrtsDfltSomeOneElSc,
        AllowClippyArbitrarySrcItemOrdering,
        ColSc,
        CrQbSc,
        CrQpSc,
        CrSc,
        CrTblColQpSc,
        CrUcc,
        DfltSomeOneElMaxPageSizeSc,
        DfltSomeOneElSc,
        EqOprtrUcc,
        ErSc,
        IncrSc,
        NormalizeSc,
        OptUpdSc,
        OptVecCrSc,
        PgCrudCmnDfltSomeOneElCall,
        PgTypeEqOprtrUcc,
        PgTypeNotPkUcc,
        PgTypeOptVecWhGreaterThanTestSc,
        PgTypeTestCasesUcc,
        PgTypeUcc,
        PgTypeWhFltUcc,
        PreviousRdAndOptUpdIntoRdSc,
        QbSc,
        QpErUcc,
        QpSc,
        QuerySc,
        RdIdsAndCrIntoOptVRdSc,
        RdIdsAndCrIntoOptVecWhEqToFieldSc,
        RdIdsAndCrIntoRdSc,
        RdIdsAndCrIntoTtSc,
        RdIdsAndCrIntoVecWhEqUsingFieldsSc,
        RdIdsAndCrIntoWhEqSc,
        RdIdsAndTtIntoPgTypeOptWhGreaterThanSc,
        RdIdsSc,
        RdIdsTo2DimsVecRdInnSc,
        RdIdsToOptVRdDfltSomeOneElSc,
        RdIdsUcc,
        RdInnIntoRdWithNewOrTryNewUnwrapedSc,
        RdInnIntoUpdWithNewOrTryNewUnwrapedSc,
        RdInnUcc,
        RdSc,
        RdUcc,
        SelOnlyIdsQpSc,
        SelOnlyUpddIdsQbSc,
        SelOnlyUpddIdsQpSc,
        SelQpSc,
        SelUcc,
        SelfUcc,
        TtSc,
        TtUcc,
        UpdForQueryUcc,
        UpdQbSc,
        UpdQpSc,
        UpdToRdIdsSc,
        UpdUcc,
        VSc,
        VUcc,
        ValueSc,
        WhUcc,
    ) = (
        naming::AddOprtrSc,
        naming::AllVrtsDfltSomeOneElMaxPageSizeSc,
        naming::AllVrtsDfltSomeOneElSc,
        token_patterns::AllowClippyArbitrarySrcItemOrdering,
        naming::ColSc,
        naming::CrQbSc,
        naming::CrQpSc,
        naming::CrSc,
        naming::CrTblColQpSc,
        naming::CrUcc,
        naming::DfltSomeOneElMaxPageSizeSc,
        naming::DfltSomeOneElSc,
        naming::EqOprtrUcc,
        naming::ErSc,
        naming::IncrSc,
        naming::NormalizeSc,
        naming::OptUpdSc,
        naming::OptVecCrSc,
        token_patterns::PgCrudCmnDfltSomeOneElCall,
        naming::PgTypeEqOprtrUcc,
        naming::PgTypeNotPkUcc,
        naming::PgTypeOptVecWhGreaterThanTestSc,
        naming::PgTypeTestCasesUcc,
        naming::PgTypeUcc,
        naming::PgTypeWhFltUcc,
        naming::PreviousRdAndOptUpdIntoRdSc,
        naming::QbSc,
        naming::QpErUcc,
        naming::QpSc,
        naming::QuerySc,
        naming::RdIdsAndCrIntoOptVRdSc,
        naming::RdIdsAndCrIntoOptVecWhEqToFieldSc,
        naming::RdIdsAndCrIntoRdSc,
        naming::RdIdsAndCrIntoTtSc,
        naming::RdIdsAndCrIntoVecWhEqUsingFieldsSc,
        naming::RdIdsAndCrIntoWhEqSc,
        naming::RdIdsAndTtIntoPgTypeOptWhGreaterThanSc,
        naming::RdIdsSc,
        naming::RdIdsTo2DimsVecRdInnSc,
        naming::RdIdsToOptVRdDfltSomeOneElSc,
        naming::RdIdsUcc,
        naming::RdInnIntoRdWithNewOrTryNewUnwrapedSc,
        naming::RdInnIntoUpdWithNewOrTryNewUnwrapedSc,
        naming::RdInnUcc,
        naming::RdSc,
        naming::RdUcc,
        naming::SelOnlyIdsQpSc,
        naming::SelOnlyUpddIdsQbSc,
        naming::SelOnlyUpddIdsQpSc,
        naming::SelQpSc,
        naming::SelUcc,
        naming::SelfUcc,
        naming::TtSc,
        naming::TtUcc,
        naming::UpdForQueryUcc,
        naming::UpdQbSc,
        naming::UpdQpSc,
        naming::UpdToRdIdsSc,
        naming::UpdUcc,
        naming::VSc,
        naming::VUcc,
        naming::ValueSc,
        naming::WhUcc,
    );
    let ts = gen_qp_er_write_into_buffer_ts(import);
    quote::quote! {return Err(#ts);}.into()
}
#[must_use]
pub fn parse_strs_to_ts2_vec(v: ParseTsStrings, uuid: ParseErIdRef<'_>) -> GeneratedRustTsVec {
    v.into_vec()
        .into_iter()
        .map(|el| parse_ts_or_compile_error(ParseTsTextRef::from(el.as_str()), uuid))
        .collect::<GeneratedRustTsVec>()
}
#[must_use]
pub fn gen_mod_with_pub_use_ts(
    mod_name: &dyn quote::ToTokens,
    content_ts: &GeneratedRustTsVec,
) -> macros_helpers::GeneratedRustTs {
    quote::quote! {
        #[allow(unused_qualifications)]
        #[allow(unused_variables)]
        #[allow(clippy::absolute_paths)]
        #[allow(clippy::arbitrary_source_item_ordering)]
        mod #mod_name {
            #content_ts
        }
        pub use #mod_name::*;
    }
    .into()
}
#[must_use]
pub fn cmn_d_ts_builder() -> macros_helpers::DTsBuilder {
    macros_helpers::DTsBuilder::new()
        .make_pub()
        .d_debug()
        .d_clone()
        .d_partial_eq()
        .d_serde_serialize()
        .d_serde_deserialize()
}
#[must_use]
pub fn serde_er_enum_d_ts_builder() -> macros_helpers::DTsBuilder {
    macros_helpers::DTsBuilder::new()
        .make_pub()
        .d_debug()
        .d_serde_serialize()
        .d_serde_deserialize()
        .d_thiserror_error()
        .d_loc_lib_location()
}
#[must_use]
pub fn er_enum_d_ts_builder() -> macros_helpers::DTsBuilder {
    macros_helpers::DTsBuilder::new()
        .make_pub()
        .d_debug()
        .d_thiserror_error()
        .d_loc_lib_location()
}
#[must_use]
pub fn gen_match_ok_assign_or_return_err_ts(
    expr_ts: &dyn quote::ToTokens,
    assign_target_ts: &dyn quote::ToTokens,
    ok_v_ts: &dyn quote::ToTokens,
) -> macros_helpers::GeneratedRustTs {
    #[allow(non_snake_case, unused_variables)]
    let (
        AddOprtrSc,
        AllVrtsDfltSomeOneElMaxPageSizeSc,
        AllVrtsDfltSomeOneElSc,
        AllowClippyArbitrarySrcItemOrdering,
        ColSc,
        CrQbSc,
        CrQpSc,
        CrSc,
        CrTblColQpSc,
        CrUcc,
        DfltSomeOneElMaxPageSizeSc,
        DfltSomeOneElSc,
        EqOprtrUcc,
        ErSc,
        IncrSc,
        NormalizeSc,
        OptUpdSc,
        OptVecCrSc,
        PgCrudCmnDfltSomeOneElCall,
        PgTypeEqOprtrUcc,
        PgTypeNotPkUcc,
        PgTypeOptVecWhGreaterThanTestSc,
        PgTypeTestCasesUcc,
        PgTypeUcc,
        PgTypeWhFltUcc,
        PreviousRdAndOptUpdIntoRdSc,
        QbSc,
        QpErUcc,
        QpSc,
        QuerySc,
        RdIdsAndCrIntoOptVRdSc,
        RdIdsAndCrIntoOptVecWhEqToFieldSc,
        RdIdsAndCrIntoRdSc,
        RdIdsAndCrIntoTtSc,
        RdIdsAndCrIntoVecWhEqUsingFieldsSc,
        RdIdsAndCrIntoWhEqSc,
        RdIdsAndTtIntoPgTypeOptWhGreaterThanSc,
        RdIdsSc,
        RdIdsTo2DimsVecRdInnSc,
        RdIdsToOptVRdDfltSomeOneElSc,
        RdIdsUcc,
        RdInnIntoRdWithNewOrTryNewUnwrapedSc,
        RdInnIntoUpdWithNewOrTryNewUnwrapedSc,
        RdInnUcc,
        RdSc,
        RdUcc,
        SelOnlyIdsQpSc,
        SelOnlyUpddIdsQbSc,
        SelOnlyUpddIdsQpSc,
        SelQpSc,
        SelUcc,
        SelfUcc,
        TtSc,
        TtUcc,
        UpdForQueryUcc,
        UpdQbSc,
        UpdQpSc,
        UpdToRdIdsSc,
        UpdUcc,
        VSc,
        VUcc,
        ValueSc,
        WhUcc,
    ) = (
        naming::AddOprtrSc,
        naming::AllVrtsDfltSomeOneElMaxPageSizeSc,
        naming::AllVrtsDfltSomeOneElSc,
        token_patterns::AllowClippyArbitrarySrcItemOrdering,
        naming::ColSc,
        naming::CrQbSc,
        naming::CrQpSc,
        naming::CrSc,
        naming::CrTblColQpSc,
        naming::CrUcc,
        naming::DfltSomeOneElMaxPageSizeSc,
        naming::DfltSomeOneElSc,
        naming::EqOprtrUcc,
        naming::ErSc,
        naming::IncrSc,
        naming::NormalizeSc,
        naming::OptUpdSc,
        naming::OptVecCrSc,
        token_patterns::PgCrudCmnDfltSomeOneElCall,
        naming::PgTypeEqOprtrUcc,
        naming::PgTypeNotPkUcc,
        naming::PgTypeOptVecWhGreaterThanTestSc,
        naming::PgTypeTestCasesUcc,
        naming::PgTypeUcc,
        naming::PgTypeWhFltUcc,
        naming::PreviousRdAndOptUpdIntoRdSc,
        naming::QbSc,
        naming::QpErUcc,
        naming::QpSc,
        naming::QuerySc,
        naming::RdIdsAndCrIntoOptVRdSc,
        naming::RdIdsAndCrIntoOptVecWhEqToFieldSc,
        naming::RdIdsAndCrIntoRdSc,
        naming::RdIdsAndCrIntoTtSc,
        naming::RdIdsAndCrIntoVecWhEqUsingFieldsSc,
        naming::RdIdsAndCrIntoWhEqSc,
        naming::RdIdsAndTtIntoPgTypeOptWhGreaterThanSc,
        naming::RdIdsSc,
        naming::RdIdsTo2DimsVecRdInnSc,
        naming::RdIdsToOptVRdDfltSomeOneElSc,
        naming::RdIdsUcc,
        naming::RdInnIntoRdWithNewOrTryNewUnwrapedSc,
        naming::RdInnIntoUpdWithNewOrTryNewUnwrapedSc,
        naming::RdInnUcc,
        naming::RdSc,
        naming::RdUcc,
        naming::SelOnlyIdsQpSc,
        naming::SelOnlyUpddIdsQbSc,
        naming::SelOnlyUpddIdsQpSc,
        naming::SelQpSc,
        naming::SelUcc,
        naming::SelfUcc,
        naming::TtSc,
        naming::TtUcc,
        naming::UpdForQueryUcc,
        naming::UpdQbSc,
        naming::UpdQpSc,
        naming::UpdToRdIdsSc,
        naming::UpdUcc,
        naming::VSc,
        naming::VUcc,
        naming::ValueSc,
        naming::WhUcc,
    );
    quote::quote! {
        match #expr_ts {
            Ok(#ok_v_ts) => {
                #assign_target_ts = #ok_v_ts;
            }
            Err(#ErSc) => {
                return Err(#ErSc);
            }
        }
    }
    .into()
}
#[must_use]
pub fn gen_match_ok_or_return_err_ts(
    expr_ts: &dyn quote::ToTokens,
    ok_v_ts: &dyn quote::ToTokens,
) -> macros_helpers::GeneratedRustTs {
    #[allow(non_snake_case, unused_variables)]
    let (
        AddOprtrSc,
        AllVrtsDfltSomeOneElMaxPageSizeSc,
        AllVrtsDfltSomeOneElSc,
        AllowClippyArbitrarySrcItemOrdering,
        ColSc,
        CrQbSc,
        CrQpSc,
        CrSc,
        CrTblColQpSc,
        CrUcc,
        DfltSomeOneElMaxPageSizeSc,
        DfltSomeOneElSc,
        EqOprtrUcc,
        ErSc,
        IncrSc,
        NormalizeSc,
        OptUpdSc,
        OptVecCrSc,
        PgCrudCmnDfltSomeOneElCall,
        PgTypeEqOprtrUcc,
        PgTypeNotPkUcc,
        PgTypeOptVecWhGreaterThanTestSc,
        PgTypeTestCasesUcc,
        PgTypeUcc,
        PgTypeWhFltUcc,
        PreviousRdAndOptUpdIntoRdSc,
        QbSc,
        QpErUcc,
        QpSc,
        QuerySc,
        RdIdsAndCrIntoOptVRdSc,
        RdIdsAndCrIntoOptVecWhEqToFieldSc,
        RdIdsAndCrIntoRdSc,
        RdIdsAndCrIntoTtSc,
        RdIdsAndCrIntoVecWhEqUsingFieldsSc,
        RdIdsAndCrIntoWhEqSc,
        RdIdsAndTtIntoPgTypeOptWhGreaterThanSc,
        RdIdsSc,
        RdIdsTo2DimsVecRdInnSc,
        RdIdsToOptVRdDfltSomeOneElSc,
        RdIdsUcc,
        RdInnIntoRdWithNewOrTryNewUnwrapedSc,
        RdInnIntoUpdWithNewOrTryNewUnwrapedSc,
        RdInnUcc,
        RdSc,
        RdUcc,
        SelOnlyIdsQpSc,
        SelOnlyUpddIdsQbSc,
        SelOnlyUpddIdsQpSc,
        SelQpSc,
        SelUcc,
        SelfUcc,
        TtSc,
        TtUcc,
        UpdForQueryUcc,
        UpdQbSc,
        UpdQpSc,
        UpdToRdIdsSc,
        UpdUcc,
        VSc,
        VUcc,
        ValueSc,
        WhUcc,
    ) = (
        naming::AddOprtrSc,
        naming::AllVrtsDfltSomeOneElMaxPageSizeSc,
        naming::AllVrtsDfltSomeOneElSc,
        token_patterns::AllowClippyArbitrarySrcItemOrdering,
        naming::ColSc,
        naming::CrQbSc,
        naming::CrQpSc,
        naming::CrSc,
        naming::CrTblColQpSc,
        naming::CrUcc,
        naming::DfltSomeOneElMaxPageSizeSc,
        naming::DfltSomeOneElSc,
        naming::EqOprtrUcc,
        naming::ErSc,
        naming::IncrSc,
        naming::NormalizeSc,
        naming::OptUpdSc,
        naming::OptVecCrSc,
        token_patterns::PgCrudCmnDfltSomeOneElCall,
        naming::PgTypeEqOprtrUcc,
        naming::PgTypeNotPkUcc,
        naming::PgTypeOptVecWhGreaterThanTestSc,
        naming::PgTypeTestCasesUcc,
        naming::PgTypeUcc,
        naming::PgTypeWhFltUcc,
        naming::PreviousRdAndOptUpdIntoRdSc,
        naming::QbSc,
        naming::QpErUcc,
        naming::QpSc,
        naming::QuerySc,
        naming::RdIdsAndCrIntoOptVRdSc,
        naming::RdIdsAndCrIntoOptVecWhEqToFieldSc,
        naming::RdIdsAndCrIntoRdSc,
        naming::RdIdsAndCrIntoTtSc,
        naming::RdIdsAndCrIntoVecWhEqUsingFieldsSc,
        naming::RdIdsAndCrIntoWhEqSc,
        naming::RdIdsAndTtIntoPgTypeOptWhGreaterThanSc,
        naming::RdIdsSc,
        naming::RdIdsTo2DimsVecRdInnSc,
        naming::RdIdsToOptVRdDfltSomeOneElSc,
        naming::RdIdsUcc,
        naming::RdInnIntoRdWithNewOrTryNewUnwrapedSc,
        naming::RdInnIntoUpdWithNewOrTryNewUnwrapedSc,
        naming::RdInnUcc,
        naming::RdSc,
        naming::RdUcc,
        naming::SelOnlyIdsQpSc,
        naming::SelOnlyUpddIdsQbSc,
        naming::SelOnlyUpddIdsQpSc,
        naming::SelQpSc,
        naming::SelUcc,
        naming::SelfUcc,
        naming::TtSc,
        naming::TtUcc,
        naming::UpdForQueryUcc,
        naming::UpdQbSc,
        naming::UpdQpSc,
        naming::UpdToRdIdsSc,
        naming::UpdUcc,
        naming::VSc,
        naming::VUcc,
        naming::ValueSc,
        naming::WhUcc,
    );
    quote::quote! {
        match #expr_ts {
            Ok(#ok_v_ts) => #ok_v_ts,
            Err(#ErSc) => {
                return Err(#ErSc);
            }
        }
    }
    .into()
}
#[must_use]
pub fn gen_match_not_empty_unq_vec_try_new_some_or_none_ts(
    import: &Import,
    expr_ts: &dyn quote::ToTokens,
    ok_v_ts: &dyn quote::ToTokens,
    panic_uuid: PanicUuidRef<'_>,
) -> macros_helpers::GeneratedRustTs {
    let panic_uuid_ts = gen_quotes::dq_ts(panic_uuid.as_ref());
    quote::quote! {
        match #expr_ts {
            Ok(#ok_v_ts) => Some(#ok_v_ts),
            Err(er) => match er {
                #import::NotEmptyUnqVecTryNewEr::IsEmpty {..} => None,
                #import::NotEmptyUnqVecTryNewEr::NotUnq {..} => panic!(#panic_uuid_ts)
            }
        }
    }
    .into()
}
#[must_use]
pub fn gen_if_let_some_match_ok_assign_query_or_return_err_ts(
    expr_ts: &dyn quote::ToTokens,
    some_v_ts: &dyn quote::ToTokens,
    ok_v_ts: &dyn quote::ToTokens,
) -> macros_helpers::GeneratedRustTs {
    #[allow(non_snake_case, unused_variables)]
    let (
        AddOprtrSc,
        AllVrtsDfltSomeOneElMaxPageSizeSc,
        AllVrtsDfltSomeOneElSc,
        AllowClippyArbitrarySrcItemOrdering,
        ColSc,
        CrQbSc,
        CrQpSc,
        CrSc,
        CrTblColQpSc,
        CrUcc,
        DfltSomeOneElMaxPageSizeSc,
        DfltSomeOneElSc,
        EqOprtrUcc,
        ErSc,
        IncrSc,
        NormalizeSc,
        OptUpdSc,
        OptVecCrSc,
        PgCrudCmnDfltSomeOneElCall,
        PgTypeEqOprtrUcc,
        PgTypeNotPkUcc,
        PgTypeOptVecWhGreaterThanTestSc,
        PgTypeTestCasesUcc,
        PgTypeUcc,
        PgTypeWhFltUcc,
        PreviousRdAndOptUpdIntoRdSc,
        QbSc,
        QpErUcc,
        QpSc,
        QuerySc,
        RdIdsAndCrIntoOptVRdSc,
        RdIdsAndCrIntoOptVecWhEqToFieldSc,
        RdIdsAndCrIntoRdSc,
        RdIdsAndCrIntoTtSc,
        RdIdsAndCrIntoVecWhEqUsingFieldsSc,
        RdIdsAndCrIntoWhEqSc,
        RdIdsAndTtIntoPgTypeOptWhGreaterThanSc,
        RdIdsSc,
        RdIdsTo2DimsVecRdInnSc,
        RdIdsToOptVRdDfltSomeOneElSc,
        RdIdsUcc,
        RdInnIntoRdWithNewOrTryNewUnwrapedSc,
        RdInnIntoUpdWithNewOrTryNewUnwrapedSc,
        RdInnUcc,
        RdSc,
        RdUcc,
        SelOnlyIdsQpSc,
        SelOnlyUpddIdsQbSc,
        SelOnlyUpddIdsQpSc,
        SelQpSc,
        SelUcc,
        SelfUcc,
        TtSc,
        TtUcc,
        UpdForQueryUcc,
        UpdQbSc,
        UpdQpSc,
        UpdToRdIdsSc,
        UpdUcc,
        VSc,
        VUcc,
        ValueSc,
        WhUcc,
    ) = (
        naming::AddOprtrSc,
        naming::AllVrtsDfltSomeOneElMaxPageSizeSc,
        naming::AllVrtsDfltSomeOneElSc,
        token_patterns::AllowClippyArbitrarySrcItemOrdering,
        naming::ColSc,
        naming::CrQbSc,
        naming::CrQpSc,
        naming::CrSc,
        naming::CrTblColQpSc,
        naming::CrUcc,
        naming::DfltSomeOneElMaxPageSizeSc,
        naming::DfltSomeOneElSc,
        naming::EqOprtrUcc,
        naming::ErSc,
        naming::IncrSc,
        naming::NormalizeSc,
        naming::OptUpdSc,
        naming::OptVecCrSc,
        token_patterns::PgCrudCmnDfltSomeOneElCall,
        naming::PgTypeEqOprtrUcc,
        naming::PgTypeNotPkUcc,
        naming::PgTypeOptVecWhGreaterThanTestSc,
        naming::PgTypeTestCasesUcc,
        naming::PgTypeUcc,
        naming::PgTypeWhFltUcc,
        naming::PreviousRdAndOptUpdIntoRdSc,
        naming::QbSc,
        naming::QpErUcc,
        naming::QpSc,
        naming::QuerySc,
        naming::RdIdsAndCrIntoOptVRdSc,
        naming::RdIdsAndCrIntoOptVecWhEqToFieldSc,
        naming::RdIdsAndCrIntoRdSc,
        naming::RdIdsAndCrIntoTtSc,
        naming::RdIdsAndCrIntoVecWhEqUsingFieldsSc,
        naming::RdIdsAndCrIntoWhEqSc,
        naming::RdIdsAndTtIntoPgTypeOptWhGreaterThanSc,
        naming::RdIdsSc,
        naming::RdIdsTo2DimsVecRdInnSc,
        naming::RdIdsToOptVRdDfltSomeOneElSc,
        naming::RdIdsUcc,
        naming::RdInnIntoRdWithNewOrTryNewUnwrapedSc,
        naming::RdInnIntoUpdWithNewOrTryNewUnwrapedSc,
        naming::RdInnUcc,
        naming::RdSc,
        naming::RdUcc,
        naming::SelOnlyIdsQpSc,
        naming::SelOnlyUpddIdsQbSc,
        naming::SelOnlyUpddIdsQpSc,
        naming::SelQpSc,
        naming::SelUcc,
        naming::SelfUcc,
        naming::TtSc,
        naming::TtUcc,
        naming::UpdForQueryUcc,
        naming::UpdQbSc,
        naming::UpdQpSc,
        naming::UpdToRdIdsSc,
        naming::UpdUcc,
        naming::VSc,
        naming::VUcc,
        naming::ValueSc,
        naming::WhUcc,
    );
    let match_ts = gen_match_ok_assign_or_return_err_ts(expr_ts, &QuerySc, ok_v_ts);
    quote::quote! {
        if let Some(#some_v_ts) = &#VSc.0 {
            #match_ts
        }
        Ok(#QuerySc)
    }
    .into()
}
fn parse_ts_or_compile_error(
    v: ParseTsTextRef<'_>,
    er_id: ParseErIdRef<'_>,
) -> macros_helpers::GeneratedRustTs {
    match v.as_ref().parse::<proc_macro2::TokenStream>() {
        Ok(parsed_ts) => parsed_ts.into(),
        Err(er) => {
            let msg = format!("{}: {er}", er_id.as_ref());
            quote::quote! {compile_error!(#msg);}.into()
        }
    }
}
