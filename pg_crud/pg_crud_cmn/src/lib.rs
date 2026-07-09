pub const DEFAULT_PAGINATION_LIMIT: i64 = 5;
const PG_CRUD_STRING_WRAPPER_MAX_LEN: usize = 1_048_576;
#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    schemars::JsonSchema,
    optml::Optml,
    newtype::Newtype,
)]
#[newtype(display, from, to_err_string)]
pub struct PgnLimit(i64);
impl PgnLimit {
    #[must_use]
    pub const fn get(self) -> i64 {
        self.0
    }
}
impl From<i32> for PgnLimit {
    fn from(value: i32) -> Self {
        Self(value.into())
    }
}
#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    schemars::JsonSchema,
    optml::Optml,
    newtype::Newtype,
)]
#[newtype(display, from, to_err_string)]
pub struct PgnOffset(i64);
impl PgnOffset {
    #[must_use]
    pub const fn get(self) -> i64 {
        self.0
    }
}
impl From<i32> for PgnOffset {
    fn from(value: i32) -> Self {
        Self(value.into())
    }
}
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, optml::Optml)]
pub struct PgnStart(i64);
impl From<i64> for PgnStart {
    fn from(value: i64) -> Self {
        Self(value)
    }
}
impl PgnStart {
    #[must_use]
    pub const fn get(self) -> i64 {
        self.0
    }
}
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, optml::Optml)]
pub struct PgnEnd(i64);
impl From<i64> for PgnEnd {
    fn from(value: i64) -> Self {
        Self(value)
    }
}
impl PgnEnd {
    #[must_use]
    pub const fn get(self) -> i64 {
        self.0
    }
}
pub trait AllEnumVrtsArrDfltSomeOneEl: Sized {
    fn all_vrts_dflt_some_one_el() -> Vec<Self>;
}
pub trait AllEnumVrtsArrDfltSomeOneElMaxPageSize: Sized {
    fn all_vrts_dflt_some_one_el_max_page_size() -> Vec<Self>;
}
pub trait DfltSomeOneEl: Sized {
    fn dflt_some_one_el() -> Self;
}
pub trait DfltSomeOneElMaxPageSize: Sized {
    fn dflt_some_one_el_max_page_size() -> Self;
}
#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    serde::Serialize,
    serde::Deserialize,
    Eq,
    PartialEq,
    schemars::JsonSchema,
    optml::Optml,
)]
pub enum Oprtr {
    And,
    AndNot,
    #[default]
    Or,
    OrNot,
}
impl DfltSomeOneEl for Oprtr {
    fn dflt_some_one_el() -> Self {
        Self::default()
    }
}
impl std::fmt::Display for Oprtr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl Oprtr {
    #[must_use]
    pub fn to_qp(&self, add_oprtr: AddOprtr) -> QpFragment {
        const SPACE: &str = " ";
        let mut qp = String::with_capacity(8);
        if bool::from(add_oprtr) {
            let write_res = match *self {
                Self::And | Self::AndNot => {
                    std::fmt::Write::write_fmt(&mut qp, format_args!("{}{}", naming::AndSc, SPACE))
                }
                Self::Or | Self::OrNot => {
                    std::fmt::Write::write_fmt(&mut qp, format_args!("{}{}", naming::OrSc, SPACE))
                }
            };
            if write_res.is_err() {
                return QpFragment::try_from(String::default()).unwrap_or_else(QpFragment::from);
            }
        }
        if matches!(*self, Self::AndNot | Self::OrNot)
            && std::fmt::Write::write_fmt(&mut qp, format_args!("{}{}", naming::NotSc, SPACE))
                .is_err()
        {
            return QpFragment::try_from(String::default()).unwrap_or_else(QpFragment::from);
        }
        QpFragment::try_from(qp).unwrap_or_else(QpFragment::from)
    }
}
#[cfg(test)]
mod tests_oprtr_to_qp {
    #[test]
    fn to_qp_includes_oprtr_when_requested() {
        assert_eq!(
            super::Oprtr::And.to_qp(super::AddOprtr::from(true)).0,
            format!("{} ", naming::AndSc)
        );
        assert_eq!(
            super::Oprtr::Or.to_qp(super::AddOprtr::from(true)).0,
            format!("{} ", naming::OrSc)
        );
    }
    #[test]
    fn to_qp_includes_not_suffix_for_negative_variants() {
        assert_eq!(
            super::Oprtr::AndNot.to_qp(super::AddOprtr::from(true)).0,
            format!("{} {} ", naming::AndSc, naming::NotSc)
        );
        assert_eq!(
            super::Oprtr::OrNot.to_qp(super::AddOprtr::from(true)).0,
            format!("{} {} ", naming::OrSc, naming::NotSc)
        );
    }
    #[test]
    fn to_qp_omits_oprtr_when_disabled_and_keeps_not_only_for_negative_variants() {
        assert_eq!(super::Oprtr::And.to_qp(super::AddOprtr::from(false)).0, "");
        assert_eq!(super::Oprtr::Or.to_qp(super::AddOprtr::from(false)).0, "");
        assert_eq!(
            super::Oprtr::AndNot.to_qp(super::AddOprtr::from(false)).0,
            format!("{} ", naming::NotSc)
        );
        assert_eq!(
            super::Oprtr::OrNot.to_qp(super::AddOprtr::from(false)).0,
            format!("{} ", naming::NotSc)
        );
    }
}
impl quote::ToTokens for Oprtr {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match *self {
            Self::And => quote::quote! {And},
            Self::Or => quote::quote! {Or},
            Self::AndNot => quote::quote! {AndNot},
            Self::OrNot => quote::quote! {OrNot},
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, PartialEq, optml::Optml)]
pub enum PgTypeGreaterThanVrt {
    EqNotGreaterThan,
    GreaterThan,
    NotGreaterThan,
}
impl PgTypeGreaterThanVrt {
    #[must_use]
    pub const fn oprtr(&self) -> Oprtr {
        match *self {
            Self::GreaterThan => Oprtr::Or,
            Self::NotGreaterThan | Self::EqNotGreaterThan => Oprtr::OrNot,
        }
    }
}
impl quote::ToTokens for PgTypeGreaterThanVrt {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match *self {
            Self::EqNotGreaterThan => quote::quote! {EqNotGreaterThan},
            Self::GreaterThan => quote::quote! {GreaterThan},
            Self::NotGreaterThan => quote::quote! {NotGreaterThan},
        }
        .to_tokens(tokens);
    }
}
pg_crud_cmn_macros::trait_al!(DebugClonePartialEqAl = std::fmt::Debug + Clone + PartialEq);
pg_crud_cmn_macros::trait_al!(
    DebugClonePartialEqSerializeAl = DebugClonePartialEqAl + serde::Serialize
);
pg_crud_cmn_macros::trait_al!(DebugClonePartialEqSerdeAl = DebugClonePartialEqSerializeAl + for<'__> serde::Deserialize<'__>);
pg_crud_cmn_macros::trait_al!(
    DebugClonePartialEqSerdeDefaultSomeOneAl = DebugClonePartialEqSerdeAl + DfltSomeOneEl
);
pg_crud_cmn_macros::trait_al!(SqlxEncodePgSqlxTypePgAl = for<'__> sqlx::Encode<'__, sqlx::Postgres> + sqlx::Type<sqlx::Postgres>);
pg_crud_cmn_macros::trait_al!(
    UtoipaToSchemaAndSchemarsJsonSchemaAl = for<'__> utoipa::ToSchema<'__> + schemars::JsonSchema
);
pg_crud_cmn_macros::trait_al!(TtAl = DebugClonePartialEqSerdeDefaultSomeOneAl);
pg_crud_cmn_macros::trait_al!(CrAl = DebugClonePartialEqSerdeDefaultSomeOneAl);
pg_crud_cmn_macros::trait_al!(
    CrForQueryAl = DebugClonePartialEqSerializeAl + SqlxEncodePgSqlxTypePgAl
);
pg_crud_cmn_macros::trait_al!(SelAl = DebugClonePartialEqSerdeDefaultSomeOneAl);
pg_crud_cmn_macros::trait_al!(WhAl = DebugClonePartialEqSerdeAl + for<'__> PgTypeWhFlt<'__>);
pg_crud_cmn_macros::trait_al!(RdAl = DebugClonePartialEqSerdeAl);
pg_crud_cmn_macros::trait_al!(RdIdsAl = DebugClonePartialEqSerdeAl);
pg_crud_cmn_macros::trait_al!(RdInnAl = DebugClonePartialEqAl);
pg_crud_cmn_macros::trait_al!(UpdAl = DebugClonePartialEqSerdeDefaultSomeOneAl);
pg_crud_cmn_macros::trait_al!(UpdForQueryAl = DebugClonePartialEqSerializeAl);
#[allow(clippy::arbitrary_source_item_ordering)]
pub trait PgType {
    //difference between Cr and Tt - Cr may not contain generated by pg id
    type Tt: TtAl;
    fn cr_tbl_col_qp(col: SqlColRef<'_>, is_pk: IsPk) -> QpFragment;
    type Cr: CrAl;
    fn cr_qp(v: &Self::Cr, incr: &mut dyn QpIncrMut) -> Result<QpFragment, QpEr>;
    fn cr_qb(v: Self::Cr, query: PgQuery<'_>) -> Result<PgQuery<'_>, PgQueryBindEr>;
    type Sel: SelAl;
    fn sel_qp(v: &Self::Sel, col: SqlColRef<'_>) -> Result<QpFragment, QpEr>;
    type Wh: WhAl;
    type Rd: RdAl + for<'__> sqlx::Decode<'__, sqlx::Postgres> + sqlx::Type<sqlx::Postgres>;
    fn normalize(v: Self::Rd) -> Self::Rd;
    type RdIds: RdIdsAl;
    fn sel_only_ids_qp(col: SqlColRef<'_>) -> Result<QpFragment, QpEr>;
    type RdInn: RdInnAl;
    fn into_inn(v: Self::Rd) -> Self::RdInn;
    type Upd: UpdAl;
    type UpdForQuery: UpdForQueryAl;
    fn upd_qp(
        v: &Self::UpdForQuery,
        upd_accumulator: SqlColRef<'_>,
        upd_target: SqlColRef<'_>,
        upd_path: SqlColRef<'_>,
        incr: &mut dyn QpIncrMut,
    ) -> Result<QpFragment, QpEr>;
    fn upd_qb(v: Self::UpdForQuery, query: PgQuery<'_>) -> Result<PgQuery<'_>, PgQueryBindEr>;
    fn sel_only_updd_ids_qp(
        v: &Self::UpdForQuery,
        col: SqlColRef<'_>,
        incr: &mut dyn QpIncrMut,
    ) -> Result<QpFragment, QpEr>;
    fn sel_only_updd_ids_qb<'lt>(
        v: &'lt Self::UpdForQuery,
        query: PgQuery<'lt>,
    ) -> Result<PgQuery<'lt>, PgQueryBindEr>;
}
#[allow(clippy::arbitrary_source_item_ordering)]
pub trait PgTypePk {
    type PgType: PgType;
    type Tt: TtAl + PartialOrd;
    fn rd_ids_into_tt(v: <Self::PgType as PgType>::RdIds) -> <Self::PgType as PgType>::Tt;
    fn rd_ids_into_rd(v: <Self::PgType as PgType>::RdIds) -> <Self::PgType as PgType>::Rd;
    fn rd_ids_into_upd(v: <Self::PgType as PgType>::RdIds) -> <Self::PgType as PgType>::Upd;
    fn rd_into_tt(v: <Self::PgType as PgType>::Rd) -> <Self::PgType as PgType>::Tt;
}
#[allow(clippy::arbitrary_source_item_ordering)]
pub trait PgTypeNotPk {
    type PgType: PgType;
    type Cr: CrAl + SqlxEncodePgSqlxTypePgAl;
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[cfg(feature = "test-utils")]
pub trait PgTypeTestCases {
    type PgType: PgType;
    type Sel: SelAl + DfltSomeOneElMaxPageSize;
    #[must_use]
    fn opt_vec_cr() -> Option<Vec<<Self::PgType as PgType>::Cr>> {
        None
    }
    fn rd_ids_to_2_dims_vec_rd_inn(
        rd_ids: &<Self::PgType as PgType>::RdIds,
    ) -> Vec<Vec<<Self::PgType as PgType>::RdInn>>;
    fn rd_inn_into_rd_with_new_or_try_new_unwraped(
        v: <Self::PgType as PgType>::RdInn,
    ) -> <Self::PgType as PgType>::Rd;
    fn rd_inn_into_upd_with_new_or_try_new_unwraped(
        v: <Self::PgType as PgType>::RdInn,
    ) -> <Self::PgType as PgType>::Upd;
    fn upd_to_rd_ids(v: &<Self::PgType as PgType>::Upd) -> <Self::PgType as PgType>::RdIds;
    fn rd_ids_to_opt_v_rd_dflt_some_one_el(
        _v: &<Self::PgType as PgType>::RdIds,
    ) -> Option<V<<Self::PgType as PgType>::Rd>> {
        None
    }
    fn previous_rd_and_opt_upd_into_rd(
        rd: <Self::PgType as PgType>::Rd,
        opt_upd: Option<<Self::PgType as PgType>::Upd>,
    ) -> <Self::PgType as PgType>::Rd;
    fn rd_ids_and_cr_into_rd(
        rd_ids: <Self::PgType as PgType>::RdIds,
        cr: <Self::PgType as PgType>::Cr,
    ) -> <Self::PgType as PgType>::Rd;
    fn rd_ids_and_cr_into_opt_v_rd(
        _rd_ids: <Self::PgType as PgType>::RdIds,
        _cr: <Self::PgType as PgType>::Cr,
    ) -> Option<V<<Self::PgType as PgType>::Rd>> {
        None
    }
    fn rd_ids_and_cr_into_tt(
        rd_ids: <Self::PgType as PgType>::RdIds,
        cr: <Self::PgType as PgType>::Cr,
    ) -> <Self::PgType as PgType>::Tt;
    fn rd_ids_and_cr_into_wh_eq(
        rd_ids: <Self::PgType as PgType>::RdIds,
        cr: <Self::PgType as PgType>::Cr,
    ) -> <Self::PgType as PgType>::Wh;
    fn rd_ids_and_cr_into_vec_wh_eq_using_fields(
        rd_ids: <Self::PgType as PgType>::RdIds,
        cr: <Self::PgType as PgType>::Cr,
    ) -> NotEmptyUnqVec<<Self::PgType as PgType>::Wh>;
    fn rd_ids_and_cr_into_opt_vec_wh_eq_to_field(
        _rd_ids: <Self::PgType as PgType>::RdIds,
        _cr: <Self::PgType as PgType>::Cr,
    ) -> Option<NotEmptyUnqVec<<Self::PgType as PgType>::Wh>> {
        None
    }
    fn cr_into_pg_type_opt_vec_wh_dim_one_eq(
        _cr: <Self::PgType as PgType>::Cr,
    ) -> Option<NotEmptyUnqVec<<Self::PgType as PgType>::Wh>> {
        None
    }
    #[must_use]
    fn pg_type_opt_vec_wh_greater_than_test()
    -> Option<NotEmptyUnqVec<PgTypeGreaterThanTest<Self::PgType>>> {
        None
    }
    fn rd_ids_and_tt_into_pg_type_opt_wh_greater_than(
        _greater_than_vrt: PgTypeGreaterThanVrt,
        _rd_ids: <Self::PgType as PgType>::RdIds,
        _tt: <Self::PgType as PgType>::Tt,
    ) -> Option<<Self::PgType as PgType>::Wh> {
        None
    }
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, Clone, PartialEq, optml::Optml)]
pub struct PgTypeGreaterThanTest<T: PgType> {
    pub greater_than: <T as PgType>::Tt,
    pub cr: <T as PgType>::Cr,
    pub vrt: PgTypeGreaterThanVrt,
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, optml::Optml)]
pub struct PgTypeLenGreaterThanTest<T: PgType> {
    pub cr: <T as PgType>::Cr,
    pub vrt: PgTypeGreaterThanVrt,
    pub len_greater_than: UnsignedPartOfI32,
}
pub struct PgQuery<'query_lt>(
    sqlx::query::Query<'query_lt, sqlx::Postgres, sqlx::postgres::PgArguments>,
);
impl<'query_lt> From<sqlx::query::Query<'query_lt, sqlx::Postgres, sqlx::postgres::PgArguments>>
    for PgQuery<'query_lt>
{
    fn from(
        value: sqlx::query::Query<'query_lt, sqlx::Postgres, sqlx::postgres::PgArguments>,
    ) -> Self {
        Self(value)
    }
}
impl<'query_lt> PgQuery<'query_lt> {
    pub fn into_inner(
        self,
    ) -> sqlx::query::Query<'query_lt, sqlx::Postgres, sqlx::postgres::PgArguments> {
        self.0
    }
}
impl<'query_lt> AsMut<sqlx::query::Query<'query_lt, sqlx::Postgres, sqlx::postgres::PgArguments>>
    for PgQuery<'query_lt>
{
    fn as_mut(
        &mut self,
    ) -> &mut sqlx::query::Query<'query_lt, sqlx::Postgres, sqlx::postgres::PgArguments> {
        &mut self.0
    }
}
impl std::fmt::Debug for PgQuery<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("PgQuery").finish()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, optml::Optml, newtype::Newtype)]
#[newtype(display)]
pub struct PgQueryBindEr(String);
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, thiserror::Error,
)]
pub enum PgCrudStringWrapperTryFromStringEr {
    #[error("string wrapper length {len} exceeds maximum {max}")]
    TooLong { len: usize, max: usize },
}
impl to_err_string::ToErrString for PgCrudStringWrapperTryFromStringEr {
    fn to_err_string(&self) -> to_err_string::ToErrStringValue {
        to_err_string::ToErrStringValue::try_from(self.to_string())
            .unwrap_or_else(to_err_string::ToErrStringValue::from)
    }
}
impl From<PgCrudStringWrapperTryFromStringEr> for PgQueryBindEr {
    fn from(value: PgCrudStringWrapperTryFromStringEr) -> Self {
        Self(value.to_string())
    }
}
impl PgQueryBindEr {
    #[must_use]
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for PgQueryBindEr {
    type Error = PgCrudStringWrapperTryFromStringEr;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > PG_CRUD_STRING_WRAPPER_MAX_LEN {
            return Err(Self::Error::TooLong {
                len: value.len(),
                max: PG_CRUD_STRING_WRAPPER_MAX_LEN,
            });
        }
        Ok(Self(value))
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, optml::Optml, newtype::Newtype)]
#[newtype(display)]
pub struct QpIncr(u64);
impl From<u64> for QpIncr {
    fn from(value: u64) -> Self {
        Self(value)
    }
}
impl QpIncr {
    #[must_use]
    pub const fn get(self) -> u64 {
        self.0
    }
}
pub trait QpIncrMut {
    fn checked_add_one(&mut self) -> Option<QpIncr>;
}
impl QpIncrMut for QpIncr {
    fn checked_add_one(&mut self) -> Option<QpIncr> {
        self.0.checked_add(1).map(|v| {
            *self = Self::from(v);
            Self::from(v)
        })
    }
}
impl QpIncrMut for u64 {
    fn checked_add_one(&mut self) -> Option<QpIncr> {
        self.checked_add(1).map(|v| {
            *self = v;
            QpIncr::from(v)
        })
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, optml::Optml)]
pub struct AddOprtr(bool);
impl From<bool> for AddOprtr {
    fn from(value: bool) -> Self {
        Self(value)
    }
}
impl From<AddOprtr> for bool {
    fn from(value: AddOprtr) -> Self {
        value.0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, optml::Optml)]
pub struct IsPk(bool);
impl From<bool> for IsPk {
    fn from(value: bool) -> Self {
        Self(value)
    }
}
impl From<IsPk> for bool {
    fn from(value: IsPk) -> Self {
        value.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq, optml::Optml, newtype::Newtype)]
#[newtype(display, deref)]
pub struct QpFragment(String);
impl QpFragment {
    #[must_use]
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl From<PgCrudStringWrapperTryFromStringEr> for QpFragment {
    fn from(value: PgCrudStringWrapperTryFromStringEr) -> Self {
        Self(value.to_string())
    }
}
impl TryFrom<String> for QpFragment {
    type Error = PgCrudStringWrapperTryFromStringEr;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > PG_CRUD_STRING_WRAPPER_MAX_LEN {
            return Err(Self::Error::TooLong {
                len: value.len(),
                max: PG_CRUD_STRING_WRAPPER_MAX_LEN,
            });
        }
        Ok(Self(value))
    }
}
impl AsRef<str> for QpFragment {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
impl std::fmt::Write for QpFragment {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.0.push_str(s);
        Ok(())
    }
}
#[derive(Clone, Copy)]
pub struct SqlColRef<'col_lt>(&'col_lt dyn std::fmt::Display);
impl<'col_lt, T> From<&'col_lt T> for SqlColRef<'col_lt>
where
    T: std::fmt::Display,
{
    fn from(value: &'col_lt T) -> Self {
        Self(value)
    }
}
impl std::fmt::Debug for SqlColRef<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("SqlColRef").finish()
    }
}
impl std::fmt::Display for SqlColRef<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
pub trait PgTypeWhFlt<'query_lt> {
    fn qb(self, query: PgQuery<'query_lt>) -> Result<PgQuery<'query_lt>, PgQueryBindEr>;
    fn qp(
        &self,
        incr: &mut dyn QpIncrMut,
        col: SqlColRef<'_>,
        add_oprtr: AddOprtr,
    ) -> Result<QpFragment, QpEr>;
}
//todo custom deserialization - must not contain more than one el
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    schemars::JsonSchema,
    optml::Optml,
)]
pub struct NlJsonObjPgTypeWhFlt<
    T: std::fmt::Debug + PartialEq + Clone + for<'lt> PgTypeWhFlt<'lt> + AllEnumVrtsArrDfltSomeOneEl,
>(Option<NotEmptyUnqVec<T>>);
impl<T> NlJsonObjPgTypeWhFlt<T>
where
    T: std::fmt::Debug
        + PartialEq
        + Clone
        + for<'t_lt> PgTypeWhFlt<'t_lt>
        + AllEnumVrtsArrDfltSomeOneEl,
{
    #[must_use]
    pub const fn as_ref(&self) -> Option<&NotEmptyUnqVec<T>> {
        self.0.as_ref()
    }
    #[must_use]
    pub fn into_option(self) -> Option<NotEmptyUnqVec<T>> {
        self.0
    }
}
impl<T> From<Option<NotEmptyUnqVec<T>>> for NlJsonObjPgTypeWhFlt<T>
where
    T: std::fmt::Debug
        + PartialEq
        + Clone
        + for<'t_lt> PgTypeWhFlt<'t_lt>
        + AllEnumVrtsArrDfltSomeOneEl,
{
    fn from(value: Option<NotEmptyUnqVec<T>>) -> Self {
        Self(value)
    }
}
impl<'query_lt, T> PgTypeWhFlt<'query_lt> for NlJsonObjPgTypeWhFlt<T>
where
    T: std::fmt::Debug
        + PartialEq
        + Clone
        + for<'t_lt> PgTypeWhFlt<'t_lt>
        + AllEnumVrtsArrDfltSomeOneEl,
{
    fn qb(self, query: PgQuery<'query_lt>) -> Result<PgQuery<'query_lt>, PgQueryBindEr> {
        match self.into_option() {
            Some(v) => v.qb(query),
            None => Ok(query), //todo mb wrong
        }
    }
    fn qp(
        &self,
        incr: &mut dyn QpIncrMut,
        col: SqlColRef<'_>,
        add_oprtr: AddOprtr,
    ) -> Result<QpFragment, QpEr> {
        self.as_ref().map_or_else(
            || Ok(QpFragment::try_from(format!("{col} = 'null'")).unwrap_or_else(QpFragment::from)),
            |v| v.qp(incr, col, add_oprtr),
        )
    }
}
impl<T> to_err_string::ToErrString for NlJsonObjPgTypeWhFlt<T>
where
    T: std::fmt::Debug
        + PartialEq
        + Clone
        + for<'t_lt> PgTypeWhFlt<'t_lt>
        + AllEnumVrtsArrDfltSomeOneEl,
{
    fn to_err_string(&self) -> to_err_string::ToErrStringValue {
        to_err_string::ToErrStringValue::try_from(format!("{self:#?}"))
            .unwrap_or_else(to_err_string::ToErrStringValue::from)
    }
}
impl<T> AllEnumVrtsArrDfltSomeOneEl for NlJsonObjPgTypeWhFlt<T>
where
    T: std::fmt::Debug
        + PartialEq
        + Clone
        + for<'t_lt> PgTypeWhFlt<'t_lt>
        + AllEnumVrtsArrDfltSomeOneEl,
{
    fn all_vrts_dflt_some_one_el() -> Vec<Self> {
        vec![Self(Some(DfltSomeOneEl::dflt_some_one_el()))]
    }
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    thiserror::Error,
    location::Location,
    optml::Optml,
)]
pub enum QpEr {
    CheckedAdd {
        loc: loc_lib::loc::Loc,
    },
    StringWrapperTryFromString {
        loc: loc_lib::loc::Loc,
        #[eo_to_err_string_serde]
        er: PgCrudStringWrapperTryFromStringEr,
    },
    WriteIntoBuffer {
        loc: loc_lib::loc::Loc,
    },
}
impl From<PgCrudStringWrapperTryFromStringEr> for QpEr {
    fn from(er: PgCrudStringWrapperTryFromStringEr) -> Self {
        Self::StringWrapperTryFromString {
            loc: loc_macros::loc!(),
            er,
        }
    }
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    utoipa::ToSchema,
    schemars::JsonSchema,
    optml::Optml,
)]
pub struct PgTypeWh<T> {
    v: NotEmptyUnqVec<T>,
    oprtr: Oprtr,
}
impl<T: PartialEq + Clone> PgTypeWh<T> {
    #[must_use]
    pub const fn get_oprtr(&self) -> &Oprtr {
        &self.oprtr
    }
    #[must_use]
    pub const fn new(oprtr: Oprtr, v: NotEmptyUnqVec<T>) -> Self {
        Self { v, oprtr }
    }
    pub fn try_new(oprtr: Oprtr, v: Vec<T>) -> Result<Self, NotEmptyUnqVecTryNewEr<T>> {
        match NotEmptyUnqVec::try_new(v) {
            Ok(v0) => Ok(Self { oprtr, v: v0 }),
            Err(er) => Err(er),
        }
    }
}
#[allow(unused_qualifications)]
#[allow(clippy::absolute_paths)]
#[allow(clippy::arbitrary_source_item_ordering)]
const _: () = {
    #[expect(clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de, T: std::fmt::Debug + PartialEq + Clone + serde::Deserialize<'de>>
        serde::Deserialize<'de> for PgTypeWh<T>
    {
        fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>
        where
            __D: serde::Deserializer<'de>,
        {
            #[expect(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                f0,
                f1,
                __ignore,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            impl _serde::de::Visitor<'_> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __f: &mut std::fmt::Formatter<'_>,
                ) -> _serde::__private228::fmt::Result {
                    _serde::__private228::Formatter::write_str(__f, "field identifier")
                }
                fn visit_u64<__E>(self, v: u64) -> Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match v {
                        0u64 => Ok(__Field::f0),
                        1u64 => Ok(__Field::f1),
                        _ => Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(self, v: &str) -> Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match v {
                        "oprtr" => Ok(__Field::f0),
                        "v" => Ok(__Field::f1),
                        _ => Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(self, v: &[u8]) -> Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match v {
                        b"oprtr" => Ok(__Field::f0),
                        b"v" => Ok(__Field::f1),
                        _ => Ok(__Field::__ignore),
                    }
                }
            }
            impl<'de> serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>
                where
                    __D: serde::Deserializer<'de>,
                {
                    serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de, PgTypeWh> {
                marker: _serde::__private228::PhantomData<PgTypeWh>,
                lt: _serde::__private228::PhantomData<&'de ()>,
            }
            impl<'de, T: std::fmt::Debug + PartialEq + Clone + serde::Deserialize<'de>>
                _serde::de::Visitor<'de> for __Visitor<'de, T>
            {
                type Value = PgTypeWh<T>;
                fn expecting(
                    &self,
                    __f: &mut std::fmt::Formatter<'_>,
                ) -> _serde::__private228::fmt::Result {
                    std::fmt::Formatter::write_str(__f, "struct PgTypeWh")
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let Some(f0) = _serde::de::SeqAccess::next_element::<Oprtr>(&mut __seq)? else {
                        return Err(_serde::de::Error::invalid_length(
                            0usize,
                            &"struct PgTypeWh with 2 els",
                        ));
                    };
                    let Some(f1) = _serde::de::SeqAccess::next_element::<Vec<T>>(&mut __seq)?
                    else {
                        return Err(_serde::de::Error::invalid_length(
                            1usize,
                            &"struct PgTypeWh with 2 els",
                        ));
                    };
                    match PgTypeWh::try_new(f0, f1) {
                        Ok(v) => Ok(v),
                        Err(er) => Err(serde::de::Error::custom(format!("{er:?}"))),
                    }
                }
                #[inline]
                fn visit_map<__A>(self, mut __map: __A) -> Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut f0: Option<Oprtr> = None;
                    let mut f1: Option<Vec<T>> = None;
                    while let Some(__k) = _serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                        match __k {
                            __Field::f0 => {
                                if Option::is_some(&f0) {
                                    return Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("oprtr"),
                                    );
                                }
                                f0 = Some(_serde::de::MapAccess::next_value::<Oprtr>(&mut __map)?);
                            }
                            __Field::f1 => {
                                if Option::is_some(&f1) {
                                    return Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("v"),
                                    );
                                }
                                f1 = Some(_serde::de::MapAccess::next_value::<Vec<T>>(&mut __map)?);
                            }
                            __Field::__ignore => {
                                let _: serde::de::IgnoredAny =
                                    _serde::de::MapAccess::next_value::<_serde::de::IgnoredAny>(
                                        &mut __map,
                                    )?;
                            }
                        }
                    }
                    let f0_v = match f0 {
                        Some(v) => v,
                        None => _serde::__private228::de::missing_field("oprtr")?,
                    };
                    let f1_v = match f1 {
                        Some(v) => v,
                        None => _serde::__private228::de::missing_field("v")?,
                    };
                    match PgTypeWh::try_new(f0_v, f1_v) {
                        Ok(v) => Ok(v),
                        Err(er) => Err(serde::de::Error::custom(format!("{er:?}"))),
                    }
                }
            }
            #[doc(hidden)]
            const FIELDS: &[&str] = &["oprtr", "v"];
            serde::Deserializer::deserialize_struct(
                __deserializer,
                "PgTypeWh",
                FIELDS,
                __Visitor {
                    marker: _serde::__private228::PhantomData::<T>,
                    lt: _serde::__private228::PhantomData,
                },
            )
        }
    }
};
impl<'query_lt, T: PgTypeWhFlt<'query_lt>> PgTypeWhFlt<'query_lt> for PgTypeWh<T> {
    fn qb(self, query: PgQuery<'query_lt>) -> Result<PgQuery<'query_lt>, PgQueryBindEr> {
        self.v
            .0
            .into_iter()
            .try_fold(query, |acc_query, el| PgTypeWhFlt::qb(el, acc_query))
    }
    fn qp(
        &self,
        incr: &mut dyn QpIncrMut,
        col: SqlColRef<'_>,
        add_oprtr: AddOprtr,
    ) -> Result<QpFragment, QpEr> {
        let mut acc = String::default();
        let mut add_oprtr_inn_h = AddOprtr::from(false);
        self.v.0.iter().try_for_each(|el| {
            let v = PgTypeWhFlt::qp(el, incr, col, add_oprtr_inn_h)?;
            if std::fmt::Write::write_fmt(&mut acc, format_args!("{v} ")).is_err() {
                return Err(QpEr::WriteIntoBuffer {
                    loc: loc_macros::loc!(),
                });
            }
            add_oprtr_inn_h = AddOprtr::from(true);
            Ok(())
        })?;
        let _: Option<char> = acc.pop();
        Ok(
            QpFragment::try_from(format!("{}({acc})", self.oprtr.to_qp(add_oprtr)))
                .unwrap_or_else(QpFragment::from),
        )
    }
}
impl<T: std::fmt::Debug + PartialEq + Clone + AllEnumVrtsArrDfltSomeOneEl> DfltSomeOneEl
    for PgTypeWh<T>
{
    fn dflt_some_one_el() -> Self {
        Self {
            oprtr: DfltSomeOneEl::dflt_some_one_el(),
            v: DfltSomeOneEl::dflt_some_one_el(),
        }
    }
}
#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    Eq,
    strum_macros::EnumString,
    optml::Optml,
)]
#[strum(serialize_all = "snake_case")]
pub enum Order {
    #[serde(rename(serialize = "asc", deserialize = "asc"))]
    #[default]
    Asc,
    #[serde(rename(serialize = "desc", deserialize = "desc"))]
    Desc,
}
impl std::fmt::Display for Order {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Asc => write!(f, "{}", naming::AscUcc),
            Self::Desc => write!(f, "{}", naming::DescUcc),
        }
    }
}
impl DfltSomeOneEl for Order {
    fn dflt_some_one_el() -> Self {
        Self::default()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, optml::Optml)]
pub struct OrderScStr(String);
impl From<PgCrudStringWrapperTryFromStringEr> for OrderScStr {
    fn from(value: PgCrudStringWrapperTryFromStringEr) -> Self {
        Self(value.to_string())
    }
}
impl TryFrom<String> for OrderScStr {
    type Error = PgCrudStringWrapperTryFromStringEr;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > PG_CRUD_STRING_WRAPPER_MAX_LEN {
            return Err(Self::Error::TooLong {
                len: value.len(),
                max: PG_CRUD_STRING_WRAPPER_MAX_LEN,
            });
        }
        Ok(Self(value))
    }
}
impl std::fmt::Display for OrderScStr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, optml::Optml)]
pub struct OrderUccStr(String);
impl From<PgCrudStringWrapperTryFromStringEr> for OrderUccStr {
    fn from(value: PgCrudStringWrapperTryFromStringEr) -> Self {
        Self(value.to_string())
    }
}
impl TryFrom<String> for OrderUccStr {
    type Error = PgCrudStringWrapperTryFromStringEr;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > PG_CRUD_STRING_WRAPPER_MAX_LEN {
            return Err(Self::Error::TooLong {
                len: value.len(),
                max: PG_CRUD_STRING_WRAPPER_MAX_LEN,
            });
        }
        Ok(Self(value))
    }
}
impl std::fmt::Display for OrderUccStr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}
impl Order {
    #[must_use]
    pub fn to_sc_str(&self) -> OrderScStr {
        OrderScStr::try_from(naming_cmn::DisplayToScStr::case(self))
            .unwrap_or_else(OrderScStr::from)
    }
    #[must_use]
    pub fn to_ucc_str(&self) -> OrderUccStr {
        OrderUccStr::try_from(naming_cmn::DisplayToUccStr::case(self))
            .unwrap_or_else(OrderUccStr::from)
    }
}
#[derive(Debug, serde::Serialize, serde::Deserialize, optml::Optml)]
pub struct OrderBy<ColGeneric> {
    pub col: ColGeneric,
    pub order: Option<Order>,
}
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    schemars::JsonSchema,
    optml::Optml,
)]
pub struct PgnBase {
    limit: PgnLimit,
    offset: PgnOffset,
}
impl PgnBase {
    #[must_use]
    pub fn end(&self) -> PgnEnd {
        PgnEnd::from(self.offset.get().saturating_add(self.limit.get()))
    }
    #[must_use]
    pub fn new_unchecked<LimitTy, OffsetTy>(limit: LimitTy, offset: OffsetTy) -> Self
    where
        LimitTy: Into<PgnLimit>,
        OffsetTy: Into<PgnOffset>,
    {
        Self {
            limit: limit.into(),
            offset: offset.into(),
        }
    }
    #[must_use]
    pub fn start(&self) -> PgnStart {
        PgnStart::from(self.offset.get())
    }
}
impl<'query_lt> PgTypeWhFlt<'query_lt> for PgnBase {
    fn qb(self, mut query: PgQuery<'query_lt>) -> Result<PgQuery<'query_lt>, PgQueryBindEr> {
        if let Err(er) = query.as_mut().try_bind(self.limit.get()) {
            return Err(PgQueryBindEr::try_from(er.to_string()).unwrap_or_else(PgQueryBindEr::from));
        }
        if let Err(er) = query.as_mut().try_bind(self.offset.get()) {
            return Err(PgQueryBindEr::try_from(er.to_string()).unwrap_or_else(PgQueryBindEr::from));
        }
        Ok(query)
    }
    fn qp(
        &self,
        incr: &mut dyn QpIncrMut,
        _: SqlColRef<'_>,
        _: AddOprtr,
    ) -> Result<QpFragment, QpEr> {
        let limit_incr = match incr_checked_add_one_returning_incr(incr) {
            Ok(v) => v,
            Err(er) => {
                return Err(er);
            }
        };
        let offset_incr = match incr_checked_add_one_returning_incr(incr) {
            Ok(v) => v,
            Err(er) => {
                return Err(er);
            }
        };
        Ok(
            QpFragment::try_from(format!("limit ${limit_incr} offset ${offset_incr}"))
                .unwrap_or_else(QpFragment::from),
        )
    }
}
impl Default for PgnBase {
    fn default() -> Self {
        Self::new_unchecked(DEFAULT_PAGINATION_LIMIT, 0)
    }
}
#[derive(Debug, serde::Deserialize, schemars::JsonSchema, optml::Optml)]
struct PgnStartsWithZeroRaw {
    limit: PgnLimit,
    offset: PgnOffset,
}
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Default,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    schemars::JsonSchema,
    optml::Optml,
)]
#[serde(try_from = "PgnStartsWithZeroRaw")]
pub struct PgnStartsWithZero(PgnBase);
#[derive(
    Debug, serde::Serialize, serde::Deserialize, thiserror::Error, location::Location, optml::Optml,
)]
pub enum PgnStartsWithZeroTryNewEr {
    LimitIsLessThanOrEqToZero {
        #[eo_to_err_string_serde]
        limit: PgnLimit,
        loc: loc_lib::loc::Loc,
    },
    OffsetIsLessThanZero {
        #[eo_to_err_string_serde]
        offset: PgnOffset,
        loc: loc_lib::loc::Loc,
    },
    OffsetPlusLimitIsIntOverflow {
        #[eo_to_err_string_serde]
        limit: PgnLimit,
        #[eo_to_err_string_serde]
        offset: PgnOffset,
        loc: loc_lib::loc::Loc,
    },
}
impl PgnStartsWithZero {
    #[must_use]
    pub fn end(&self) -> PgnEnd {
        self.0.end()
    }
    #[must_use]
    pub fn start(&self) -> PgnStart {
        self.0.start()
    }
    pub fn try_new<LimitTy, OffsetTy>(
        limit: LimitTy,
        offset: OffsetTy,
    ) -> Result<Self, PgnStartsWithZeroTryNewEr>
    where
        LimitTy: Into<PgnLimit>,
        OffsetTy: Into<PgnOffset>,
    {
        let limit_value = limit.into();
        let offset_value = offset.into();
        if limit_value.get() <= 0 || offset_value.get() < 0 {
            if limit_value.get() <= 0 {
                Err(PgnStartsWithZeroTryNewEr::LimitIsLessThanOrEqToZero {
                    limit: limit_value,
                    loc: loc_macros::loc!(),
                })
            } else {
                Err(PgnStartsWithZeroTryNewEr::OffsetIsLessThanZero {
                    offset: offset_value,
                    loc: loc_macros::loc!(),
                })
            }
        } else if offset_value.get().checked_add(limit_value.get()).is_some() {
            Ok(Self(PgnBase::new_unchecked(limit_value, offset_value)))
        } else {
            Err(PgnStartsWithZeroTryNewEr::OffsetPlusLimitIsIntOverflow {
                limit: limit_value,
                offset: offset_value,
                loc: loc_macros::loc!(),
            })
        }
    }
}
impl TryFrom<PgnStartsWithZeroRaw> for PgnStartsWithZero {
    type Error = PgnStartsWithZeroTryNewEr;
    fn try_from(v: PgnStartsWithZeroRaw) -> Result<Self, Self::Error> {
        Self::try_new(v.limit, v.offset)
    }
}
impl<'query_lt> PgTypeWhFlt<'query_lt> for PgnStartsWithZero {
    fn qb(self, query: PgQuery<'query_lt>) -> Result<PgQuery<'query_lt>, PgQueryBindEr> {
        self.0.qb(query)
    }
    fn qp(
        &self,
        incr: &mut dyn QpIncrMut,
        col: SqlColRef<'_>,
        add_oprtr: AddOprtr,
    ) -> Result<QpFragment, QpEr> {
        self.0.qp(incr, col, add_oprtr)
    }
}
impl DfltSomeOneEl for PgnStartsWithZero {
    #[inline]
    fn dflt_some_one_el() -> Self {
        Self(PgnBase::new_unchecked(DEFAULT_PAGINATION_LIMIT, 0))
    }
}
impl DfltSomeOneElMaxPageSize for PgnStartsWithZero {
    #[inline]
    fn dflt_some_one_el_max_page_size() -> Self {
        Self(PgnBase::new_unchecked(i32::MAX, 0))
    }
}
//this needed coz serde Option<Opt<T>> #[serde(skip_serializing_if = "Option::is_none")] - if both opts: inn and parent is null then it skip - its not correct
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    schemars::JsonSchema,
    optml::Optml,
)]
pub struct V<T> {
    pub v: T,
}
//todo ExactSizeIterator now is not a solution. er[E0658]: use of unstable library feature `exact_size_is_empty`. mb rewrite it later
#[derive(Debug, Clone, Copy, PartialEq, Eq, optml::Optml)]
pub struct IsStringEmptyRes(bool);
impl From<bool> for IsStringEmptyRes {
    fn from(value: bool) -> Self {
        Self(value)
    }
}
impl From<IsStringEmptyRes> for bool {
    fn from(value: IsStringEmptyRes) -> Self {
        value.0
    }
}
pub trait IsStringEmpty {
    fn is_string_empty(&self) -> IsStringEmptyRes;
}
#[derive(
    Debug, serde::Serialize, serde::Deserialize, thiserror::Error, location::Location, optml::Optml,
)]
pub enum NotEmptyUnqVecTryNewEr<T> {
    IsEmpty {
        loc: loc_lib::loc::Loc,
    },
    NotUnq {
        #[eo_to_err_string_serde]
        v: T,
        loc: loc_lib::loc::Loc,
    },
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    utoipa::ToSchema,
    schemars::JsonSchema,
    optml::Optml,
    newtype::Newtype,
)]
#[newtype(into_vec)]
pub struct NotEmptyUnqVec<T>(Vec<T>);
impl<T> NotEmptyUnqVec<T> {
    #[must_use]
    pub const fn as_slice(&self) -> &[T] {
        self.0.as_slice()
    }
    #[must_use]
    pub const fn to_vec(&self) -> &Vec<T> {
        &self.0
    }
}
impl<T: PartialEq> NotEmptyUnqVec<T> {
    pub fn try_new(mut values: Vec<T>) -> Result<Self, NotEmptyUnqVecTryNewEr<T>> {
        if values.is_empty() {
            return Err(NotEmptyUnqVecTryNewEr::IsEmpty {
                loc: loc_macros::loc!(),
            });
        }
        if let Some(duplicate) = take_fst_dup(&mut values) {
            return Err(NotEmptyUnqVecTryNewEr::NotUnq {
                v: duplicate,
                loc: loc_macros::loc!(),
            });
        }
        Ok(Self(values))
    }
}
#[allow(unused_qualifications)]
#[allow(clippy::absolute_paths)]
#[allow(clippy::arbitrary_source_item_ordering)]
const _: () = {
    #[expect(clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de, T: std::fmt::Debug + PartialEq + serde::Deserialize<'de>> serde::Deserialize<'de>
        for NotEmptyUnqVec<T>
    {
        fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>
        where
            __D: serde::Deserializer<'de>,
        {
            #[doc(hidden)]
            struct __Visitor<'de, T>
            where
                T: serde::Deserialize<'de>,
            {
                marker: _serde::__private228::PhantomData<NotEmptyUnqVec<T>>,
                lt: _serde::__private228::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de, T: std::fmt::Debug + PartialEq + serde::Deserialize<'de>>
                _serde::de::Visitor<'de> for __Visitor<'de, T>
            {
                type Value = NotEmptyUnqVec<T>;
                fn expecting(
                    &self,
                    __f: &mut std::fmt::Formatter<'_>,
                ) -> _serde::__private228::fmt::Result {
                    std::fmt::Formatter::write_str(__f, "tuple struct NotEmptyUnqVec")
                }
                #[inline]
                fn visit_newtype_struct<__E>(self, __e: __E) -> Result<Self::Value, __E::Error>
                where
                    __E: serde::Deserializer<'de>,
                {
                    let f0: Vec<T> = <Vec<T> as serde::Deserialize>::deserialize(__e)?;
                    Ok(NotEmptyUnqVec(f0))
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let Some(f0) = _serde::de::SeqAccess::next_element::<Vec<T>>(&mut __seq)?
                    else {
                        return Err(_serde::de::Error::invalid_length(
                            0usize,
                            &"tuple struct NotEmptyUnqVec with 1 el",
                        ));
                    };
                    match NotEmptyUnqVec::try_new(f0) {
                        Ok(v) => Ok(v),
                        Err(er) => Err(_serde::de::Error::custom(format!("{er:?}"))),
                    }
                }
            }
            serde::Deserializer::deserialize_newtype_struct(
                __deserializer,
                "NotEmptyUnqVec",
                __Visitor {
                    marker: _serde::__private228::PhantomData::<Self>,
                    lt: _serde::__private228::PhantomData,
                },
            )
        }
    }
};
impl<T: AllEnumVrtsArrDfltSomeOneEl> DfltSomeOneEl for NotEmptyUnqVec<T> {
    fn dflt_some_one_el() -> Self {
        Self(AllEnumVrtsArrDfltSomeOneEl::all_vrts_dflt_some_one_el())
    }
}
impl<T: AllEnumVrtsArrDfltSomeOneElMaxPageSize> DfltSomeOneElMaxPageSize for NotEmptyUnqVec<T> {
    fn dflt_some_one_el_max_page_size() -> Self {
        Self(AllEnumVrtsArrDfltSomeOneElMaxPageSize::all_vrts_dflt_some_one_el_max_page_size())
    }
}
impl<T> Default for NotEmptyUnqVec<T> {
    fn default() -> Self {
        Self(Vec::default())
    }
}
impl<T> From<NotEmptyUnqVec<T>> for Vec<T> {
    fn from(v: NotEmptyUnqVec<T>) -> Self {
        v.0
    }
}
impl<T1> NotEmptyUnqVec<T1> {
    pub fn from_t1_impl_from_t2<T2>(v: Self) -> NotEmptyUnqVec<T2>
    where
        T2: From<T1>,
    {
        NotEmptyUnqVec(v.0.into_iter().map(T2::from).collect::<Vec<T2>>())
    }
}
#[cfg(test)]
mod tests_not_empty_unq_vec {
    #[derive(Debug, PartialEq, Eq)]
    struct NonClone(u8);
    #[test]
    fn not_empty_unq_vec_try_new_supports_non_clone_values() {
        let er = super::NotEmptyUnqVec::try_new(vec![NonClone(1), NonClone(2), NonClone(1)])
            .expect_err("adf2b8c1");
        match er {
            super::NotEmptyUnqVecTryNewEr::NotUnq { v, .. } => assert_eq!(v, NonClone(1)),
            super::NotEmptyUnqVecTryNewEr::IsEmpty { .. } => panic!("9f5e2a34"),
        }
    }
    #[test]
    fn not_empty_unq_vec_try_new_returns_is_empty_for_empty_vec() {
        let er = super::NotEmptyUnqVec::<u8>::try_new(Vec::new()).expect_err("3b41de7f");
        assert!(matches!(er, super::NotEmptyUnqVecTryNewEr::IsEmpty { .. }));
    }
    #[test]
    fn first_duplicate_idx_returns_none_for_unq_input() {
        let values = vec![1u8, 2u8, 3u8];
        assert!(super::first_duplicate_idx(&values).is_none());
    }
    #[test]
    fn first_duplicate_idx_returns_none_for_empty_and_single_input() {
        assert!(super::first_duplicate_idx::<u8>(&[]).is_none());
        assert!(super::first_duplicate_idx(&[1u8]).is_none());
    }
    #[test]
    fn first_duplicate_idx_returns_first_repeated_value_index() {
        let values = vec![7u8, 8u8, 8u8, 7u8];
        assert_eq!(
            super::first_duplicate_idx(&values),
            Some(super::DuplicateIdx::from(2))
        );
    }
    #[test]
    fn take_fst_dup_returns_none_for_unq_input() {
        let mut values = vec![1u8, 2u8, 3u8];
        let actual = super::take_fst_dup(&mut values);
        assert!(actual.is_none());
        assert_eq!(values, vec![1u8, 2u8, 3u8]);
    }
    #[test]
    fn take_fst_dup_returns_first_duplicate_value() {
        let mut values = vec![7u8, 8u8, 8u8, 7u8];
        let actual = super::take_fst_dup(&mut values);
        assert_eq!(actual, Some(8u8));
        assert_eq!(values.len(), 3usize);
    }
    #[test]
    fn as_slice_matches_to_vec_view() {
        let values = super::NotEmptyUnqVec::try_new(vec![1u8, 2u8, 3u8]).expect("3f6e8a12");
        assert_eq!(values.as_slice(), &[1u8, 2u8, 3u8]);
        assert_eq!(values.as_slice(), values.to_vec().as_slice());
    }
}
impl<'query_lt, T> PgTypeWhFlt<'query_lt> for NotEmptyUnqVec<T>
where
    T: std::fmt::Debug
        + PartialEq
        + Clone
        + for<'t_lt> PgTypeWhFlt<'t_lt>
        + AllEnumVrtsArrDfltSomeOneEl,
{
    fn qb(self, query: PgQuery<'query_lt>) -> Result<PgQuery<'query_lt>, PgQueryBindEr> {
        self.0
            .into_iter()
            .try_fold(query, |acc_query, el| el.qb(acc_query))
    }
    fn qp(
        &self,
        incr: &mut dyn QpIncrMut,
        col: SqlColRef<'_>,
        add_oprtr: AddOprtr,
    ) -> Result<QpFragment, QpEr> {
        self.0
            .iter()
            .enumerate()
            .try_fold(String::default(), |mut acc, (i, el)| {
                let v = el.qp(
                    incr,
                    col,
                    if i == 0 {
                        add_oprtr
                    } else {
                        AddOprtr::from(true)
                    },
                )?;
                acc.push_str(&v.0);
                Ok(acc)
            })
            .map(QpFragment)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, optml::Optml)]
pub struct NonPkPgTypeRdIds(V<Option<()>>);
impl From<V<Option<()>>> for NonPkPgTypeRdIds {
    fn from(value: V<Option<()>>) -> Self {
        Self(value)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for NonPkPgTypeRdIds {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        <sqlx::types::Json<Self> as sqlx::Decode<sqlx::Postgres>>::decode(value).map(|v0| v0.0)
    }
}
impl sqlx::Type<sqlx::Postgres> for NonPkPgTypeRdIds {
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
impl Default for NonPkPgTypeRdIds {
    fn default() -> Self {
        Self(V { v: None })
    }
}
#[derive(Debug, Clone, Copy, optml::Optml)]
pub enum EqOprtr {
    Eq,
    IsNull,
}
impl EqOprtr {
    #[must_use]
    pub fn to_query_str(&self) -> EqOprtrQueryStr {
        match &self {
            Self::Eq => EqOprtrQueryStr::from("="),
            Self::IsNull => EqOprtrQueryStr::from("is null"),
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, optml::Optml)]
pub struct EqOprtrQueryStr(&'static str);
impl From<&'static str> for EqOprtrQueryStr {
    fn from(value: &'static str) -> Self {
        Self(value)
    }
}
impl AsRef<str> for EqOprtrQueryStr {
    fn as_ref(&self) -> &str {
        self.0
    }
}
impl std::fmt::Display for EqOprtrQueryStr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0)
    }
}
pub trait PgTypeEqOprtr {
    fn oprtr(&self) -> EqOprtr;
}
#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    optml::Optml,
)]
#[serde(try_from = "i32")]
pub struct UnsignedPartOfI32(i32);
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    thiserror::Error,
    location::Location,
    schemars::JsonSchema,
    optml::Optml,
)]
pub enum UnsignedPartOfI32TryFromI32Er {
    LessThanZero {
        loc: loc_lib::loc::Loc,
        #[eo_to_err_string_serde]
        v: UnsignedPartOfI32Raw,
    },
}
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    optml::Optml,
)]
pub struct UnsignedPartOfI32Raw(i32);
impl From<i32> for UnsignedPartOfI32Raw {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
impl UnsignedPartOfI32Raw {
    #[must_use]
    pub const fn get(self) -> i32 {
        self.0
    }
}
impl std::fmt::Display for UnsignedPartOfI32Raw {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl to_err_string::ToErrString for UnsignedPartOfI32Raw {
    fn to_err_string(&self) -> to_err_string::ToErrStringValue {
        to_err_string::ToErrStringValue::try_from(self.to_string())
            .unwrap_or_else(to_err_string::ToErrStringValue::from)
    }
}
impl TryFrom<i32> for UnsignedPartOfI32 {
    type Error = UnsignedPartOfI32TryFromI32Er;
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        if v >= 0 {
            Ok(Self(v))
        } else {
            Err(Self::Error::LessThanZero {
                v: UnsignedPartOfI32Raw::from(v),
                loc: loc_macros::loc!(),
            })
        }
    }
}
impl to_err_string::ToErrString for UnsignedPartOfI32 {
    fn to_err_string(&self) -> to_err_string::ToErrStringValue {
        to_err_string::ToErrStringValue::try_from(self.0.to_string())
            .unwrap_or_else(to_err_string::ToErrStringValue::from)
    }
}
impl sqlx::Type<sqlx::Postgres> for UnsignedPartOfI32 {
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <i32 as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <i32 as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for UnsignedPartOfI32 {
    fn encode_by_ref(
        &self,
        buf: &mut sqlx::postgres::PgArgumentBuffer,
    ) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        <i32 as sqlx::Encode<sqlx::Postgres>>::encode_by_ref(&self.0, buf)
    }
}
impl UnsignedPartOfI32 {
    #[must_use]
    pub const fn get(&self) -> Self {
        *self
    }
}
impl DfltSomeOneEl for UnsignedPartOfI32 {
    fn dflt_some_one_el() -> Self {
        Self(0)
    }
}
#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    optml::Optml,
)]
#[serde(try_from = "i32")]
pub struct NotZeroUnsignedPartOfI32(UnsignedPartOfI32);
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    thiserror::Error,
    location::Location,
    schemars::JsonSchema,
    optml::Optml,
)]
pub enum NotZeroUnsignedPartOfI32TryFromI32Er {
    IsZero {
        loc: loc_lib::loc::Loc,
    },
    UnsignedPartOfI32TryFromI32Er {
        #[eo_loc]
        v: UnsignedPartOfI32TryFromI32Er,
        loc: loc_lib::loc::Loc,
    },
}
impl TryFrom<i32> for NotZeroUnsignedPartOfI32 {
    type Error = NotZeroUnsignedPartOfI32TryFromI32Er;
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        let v0 = UnsignedPartOfI32::try_from(v).map_err(|er| {
            Self::Error::UnsignedPartOfI32TryFromI32Er {
                v: er,
                loc: loc_macros::loc!(),
            }
        })?;
        if v0.0 == 0 {
            Err(Self::Error::IsZero {
                loc: loc_macros::loc!(),
            })
        } else {
            Ok(Self(v0))
        }
    }
}
impl to_err_string::ToErrString for NotZeroUnsignedPartOfI32 {
    fn to_err_string(&self) -> to_err_string::ToErrStringValue {
        self.0.to_err_string()
    }
}
impl sqlx::Type<sqlx::Postgres> for NotZeroUnsignedPartOfI32 {
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <UnsignedPartOfI32 as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <UnsignedPartOfI32 as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for NotZeroUnsignedPartOfI32 {
    fn encode_by_ref(
        &self,
        buf: &mut sqlx::postgres::PgArgumentBuffer,
    ) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        <UnsignedPartOfI32 as sqlx::Encode<sqlx::Postgres>>::encode_by_ref(&self.0, buf)
    }
}
impl NotZeroUnsignedPartOfI32 {
    #[must_use]
    pub const fn get(&self) -> UnsignedPartOfI32 {
        self.0.get()
    }
}
impl DfltSomeOneEl for NotZeroUnsignedPartOfI32 {
    fn dflt_some_one_el() -> Self {
        Self(DfltSomeOneEl::dflt_some_one_el())
    }
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    schemars::JsonSchema,
    optml::Optml,
)]
pub enum SingleOrMultiple<T: std::fmt::Debug + PartialEq + Clone> {
    Multiple(NotEmptyUnqVec<T>),
    Single(T),
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, optml::Optml)]
pub struct DuplicateIdx(usize);
impl From<usize> for DuplicateIdx {
    fn from(value: usize) -> Self {
        Self(value)
    }
}
impl DuplicateIdx {
    #[must_use]
    pub const fn get(self) -> usize {
        self.0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, optml::Optml)]
pub struct UuidUuidTestCases([uuid::Uuid; 1]);
impl From<[uuid::Uuid; 1]> for UuidUuidTestCases {
    fn from(value: [uuid::Uuid; 1]) -> Self {
        Self(value)
    }
}
impl IntoIterator for UuidUuidTestCases {
    type IntoIter = std::array::IntoIter<uuid::Uuid, 1>;
    type Item = uuid::Uuid;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
pub fn incr_checked_add_one_returning_incr<IncrTy>(incr: &mut IncrTy) -> Result<QpIncr, QpEr>
where
    IncrTy: QpIncrMut + ?Sized,
{
    incr.checked_add_one().map_or_else(
        || {
            Err(QpEr::CheckedAdd {
                loc: loc_macros::loc!(),
            })
        },
        Ok,
    )
}
#[cfg(feature = "test-utils")]
#[must_use]
pub const fn i8_test_cases_vec() -> [i8; 3] {
    [i8::MIN, 0, i8::MAX]
}
#[cfg(feature = "test-utils")]
#[must_use]
pub const fn i16_test_cases_vec() -> [i16; 3] {
    [i16::MIN, 0, i16::MAX]
}
#[cfg(feature = "test-utils")]
#[must_use]
pub const fn i32_test_cases_vec() -> [i32; 3] {
    [i32::MIN, 0, i32::MAX]
}
#[cfg(feature = "test-utils")]
#[must_use]
pub const fn i64_test_cases_vec() -> [i64; 3] {
    [i64::MIN, 0, i64::MAX]
}
#[cfg(feature = "test-utils")]
#[must_use]
pub const fn u8_test_cases_vec() -> [u8; 3] {
    [u8::MIN, 0, u8::MAX]
}
#[cfg(feature = "test-utils")]
#[must_use]
pub const fn u16_test_cases_vec() -> [u16; 3] {
    [u16::MIN, 0, u16::MAX]
}
#[cfg(feature = "test-utils")]
#[must_use]
pub const fn u32_test_cases_vec() -> [u32; 3] {
    [u32::MIN, 0, u32::MAX]
}
#[cfg(feature = "test-utils")]
#[must_use]
pub const fn u64_test_cases_vec() -> [u64; 3] {
    [u64::MIN, 0, u64::MAX]
}
#[cfg(feature = "test-utils")]
#[must_use]
pub const fn f32_test_cases_vec() -> [f32; 18] {
    [
        f32::EPSILON,
        f32::MAX,
        f32::MIN,
        f32::MIN_POSITIVE,
        -1e30,
        -1e-30,
        -16_777_214.0,
        -100.0,
        -10.0,
        -1.0,
        -0.0,
        0.0,
        1.0,
        10.0,
        100.0,
        16_777_214.0,
        1e-30,
        1e30,
    ]
}
#[cfg(feature = "test-utils")]
#[must_use]
pub const fn f64_test_cases_vec() -> [f64; 18] {
    [
        f64::EPSILON,
        f64::MAX,
        f64::MIN,
        f64::MIN_POSITIVE,
        -1e300,
        -1e-300,
        -9_007_199_254_740_990.0,
        -100.0,
        -10.0,
        -1.0,
        -0.0,
        0.0,
        1.0,
        10.0,
        100.0,
        9_007_199_254_740_990.0,
        1e-300,
        1e300,
    ]
}
#[cfg(feature = "test-utils")]
#[must_use]
pub const fn bool_test_cases_vec() -> [bool; 2] {
    [true, false]
}
#[cfg(feature = "test-utils")]
#[must_use]
pub fn string_test_cases_vec() -> [String; 12] {
    [
        String::new(),
        "a".to_owned(),
        "Hello, world!".to_owned(),
        "   ".to_owned(),
        "\n\r\t".to_owned(),
        "1234567890".to_owned(),
        "\u{1F600}".to_owned(),
        "\u{3053}\u{3093}\u{306B}\u{3061}\u{306F}".to_owned(),
        "\u{1F30D}\u{1F680}\u{2728} Rust \u{1F496}\u{1F980}".to_owned(),
        "a".repeat(1024),
        "line1\nline2\nline3".to_owned(),
        "\u{1F496}".to_owned(),
    ]
}
#[must_use]
pub fn uuid_uuid_test_cases_vec() -> UuidUuidTestCases {
    UuidUuidTestCases::from([uuid::Uuid::new_v4()])
}
#[must_use]
pub fn first_duplicate_idx<T>(values: &[T]) -> Option<DuplicateIdx>
where
    T: PartialEq,
{
    values
        .iter()
        .enumerate()
        .find(|(idx, current)| values.iter().take(*idx).any(|prev| prev == *current))
        .map(|(idx, _)| DuplicateIdx::from(idx))
}
#[must_use]
pub fn take_fst_dup<T>(values: &mut Vec<T>) -> Option<T>
where
    T: PartialEq,
{
    let duplicate_idx = first_duplicate_idx(values.as_slice())?;
    Some(values.swap_remove(duplicate_idx.get()))
}
