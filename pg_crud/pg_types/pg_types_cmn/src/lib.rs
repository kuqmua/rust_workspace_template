#[derive(Debug, serde::Deserialize, schemars::JsonSchema, optml::Optml)]
struct PgnStartsWithOneRaw {
    limit: PgnStartsWithOneValue,
    offset: PgnStartsWithOneValue,
}
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    schemars::JsonSchema,
    optml::Optml,
    newtype::Newtype,
)]
#[newtype(display, from, to_err_string)]
pub struct PgnStartsWithOneValue(i64);
impl PgnStartsWithOneValue {
    #[must_use]
    pub const fn get(self) -> i64 {
        self.0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, optml::Optml, newtype::Newtype)]
#[newtype(from)]
pub struct IsPrimaryKey(bool);
impl From<IsPrimaryKey> for bool {
    fn from(value: IsPrimaryKey) -> Self {
        value.0
    }
}
impl From<pg_crud_cmn::IsPk> for IsPrimaryKey {
    fn from(value: pg_crud_cmn::IsPk) -> Self {
        Self::from(bool::from(value))
    }
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
#[serde(try_from = "PgnStartsWithOneRaw")]
pub struct PgnStartsWithOne(pg_crud_cmn::PgnBase);
#[derive(
    Debug, serde::Serialize, serde::Deserialize, thiserror::Error, location::Location, optml::Optml,
)]
pub enum PgnStartsWithOneTryNewEr {
    LimitIsLessThanOrEqToZero {
        #[eo_to_err_string_serde]
        limit: PgnStartsWithOneValue,
        loc: loc_lib::loc::Loc,
    },
    OffsetIsLessThanOne {
        #[eo_to_err_string_serde]
        offset: PgnStartsWithOneValue,
        loc: loc_lib::loc::Loc,
    },
    OffsetPlusLimitIsIntOverflow {
        #[eo_to_err_string_serde]
        limit: PgnStartsWithOneValue,
        #[eo_to_err_string_serde]
        offset: PgnStartsWithOneValue,
        loc: loc_lib::loc::Loc,
    },
}
impl PgnStartsWithOne {
    #[must_use]
    pub fn end(&self) -> PgnStartsWithOneValue {
        PgnStartsWithOneValue::from(self.0.end().get())
    }
    #[must_use]
    pub fn start(&self) -> PgnStartsWithOneValue {
        PgnStartsWithOneValue::from(self.0.start().get())
    }
    pub fn try_new<L, O>(limit: L, offset: O) -> Result<Self, PgnStartsWithOneTryNewEr>
    where
        L: Into<PgnStartsWithOneValue>,
        O: Into<PgnStartsWithOneValue>,
    {
        let limit_value = limit.into();
        let offset_value = offset.into();
        if limit_value.get() <= 0 || offset_value.get() < 1 {
            if limit_value.get() <= 0 {
                Err(PgnStartsWithOneTryNewEr::LimitIsLessThanOrEqToZero {
                    limit: limit_value,
                    loc: loc_macros::loc!(),
                })
            } else {
                Err(PgnStartsWithOneTryNewEr::OffsetIsLessThanOne {
                    offset: offset_value,
                    loc: loc_macros::loc!(),
                })
            }
        } else if offset_value.get().checked_add(limit_value.get()).is_some() {
            Ok(Self(pg_crud_cmn::PgnBase::new_unchecked(
                limit_value.get(),
                offset_value.get(),
            )))
        } else {
            Err(PgnStartsWithOneTryNewEr::OffsetPlusLimitIsIntOverflow {
                limit: limit_value,
                offset: offset_value,
                loc: loc_macros::loc!(),
            })
        }
    }
}
impl TryFrom<PgnStartsWithOneRaw> for PgnStartsWithOne {
    type Error = PgnStartsWithOneTryNewEr;
    fn try_from(v: PgnStartsWithOneRaw) -> Result<Self, Self::Error> {
        Self::try_new(v.limit, v.offset)
    }
}
impl<'lt> pg_crud_cmn::PgTypeWhFlt<'lt> for PgnStartsWithOne {
    fn qb(
        self,
        query: pg_crud_cmn::PgQuery<'lt>,
    ) -> Result<pg_crud_cmn::PgQuery<'lt>, pg_crud_cmn::PgQueryBindEr> {
        self.0.qb(query)
    }
    fn qp(
        &self,
        incr: &mut dyn pg_crud_cmn::QpIncrMut,
        col: pg_crud_cmn::SqlColRef<'_>,
        add_oprtr: pg_crud_cmn::AddOprtr,
    ) -> Result<pg_crud_cmn::QpFragment, pg_crud_cmn::QpEr> {
        self.0.qp(incr, col, add_oprtr)
    }
}
impl pg_crud_cmn::DfltSomeOneEl for PgnStartsWithOne {
    #[inline]
    fn dflt_some_one_el() -> Self {
        Self(pg_crud_cmn::PgnBase::new_unchecked(
            pg_crud_cmn::DEFAULT_PAGINATION_LIMIT,
            1,
        ))
    }
}
impl pg_crud_cmn::DfltSomeOneElMaxPageSize for PgnStartsWithOne {
    #[inline]
    fn dflt_some_one_el_max_page_size() -> Self {
        let one: i32 = 1;
        Self(pg_crud_cmn::PgnBase::new_unchecked(i32::MAX - one, one))
    }
}
#[must_use]
pub fn mb_pk<V>(v: V) -> impl std::fmt::Display
where
    V: Into<IsPrimaryKey>,
{
    if bool::from(v.into()) {
        "primary key"
    } else {
        ""
    }
}
