#[derive(Debug, serde::Deserialize, schemars::JsonSchema, optml::Optml)]
struct PgnStartsWithOneRaw {
    limit: i64,
    offset: i64,
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
    Debug, serde::Serialize, serde::Deserialize, thiserror::Error, loc_lib::Location, optml::Optml,
)]
pub enum PgnStartsWithOneTryNewEr {
    LimitIsLessThanOrEqToZero {
        #[eo_to_err_string_serde]
        limit: i64,
        loc: loc_lib::loc::Loc,
    },
    OffsetIsLessThanOne {
        #[eo_to_err_string_serde]
        offset: i64,
        loc: loc_lib::loc::Loc,
    },
    OffsetPlusLimitIsIntOverflow {
        #[eo_to_err_string_serde]
        limit: i64,
        #[eo_to_err_string_serde]
        offset: i64,
        loc: loc_lib::loc::Loc,
    },
}
impl PgnStartsWithOne {
    #[must_use]
    pub const fn end(&self) -> i64 {
        self.0.end()
    }
    #[must_use]
    pub const fn start(&self) -> i64 {
        self.0.start()
    }
    pub fn try_new(limit: i64, offset: i64) -> Result<Self, PgnStartsWithOneTryNewEr> {
        if limit <= 0 || offset < 1 {
            if limit <= 0 {
                Err(PgnStartsWithOneTryNewEr::LimitIsLessThanOrEqToZero {
                    limit,
                    loc: loc_lib::loc!(),
                })
            } else {
                Err(PgnStartsWithOneTryNewEr::OffsetIsLessThanOne {
                    offset,
                    loc: loc_lib::loc!(),
                })
            }
        } else if offset.checked_add(limit).is_some() {
            Ok(Self(pg_crud_cmn::PgnBase::new_unchecked(limit, offset)))
        } else {
            Err(PgnStartsWithOneTryNewEr::OffsetPlusLimitIsIntOverflow {
                limit,
                offset,
                loc: loc_lib::loc!(),
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
        query: sqlx::query::Query<'lt, sqlx::Postgres, sqlx::postgres::PgArguments>,
    ) -> Result<sqlx::query::Query<'lt, sqlx::Postgres, sqlx::postgres::PgArguments>, String> {
        self.0.qb(query)
    }
    fn qp(
        &self,
        incr: &mut u64,
        col: &dyn std::fmt::Display,
        add_oprtr: bool,
    ) -> Result<String, pg_crud_cmn::QpEr> {
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
        Self(pg_crud_cmn::PgnBase::new_unchecked(
            (i32::MAX - one).into(),
            one.into(),
        ))
    }
}
#[must_use]
pub fn mb_pk(v: bool) -> impl std::fmt::Display {
    if v { "primary key" } else { "" }
}
