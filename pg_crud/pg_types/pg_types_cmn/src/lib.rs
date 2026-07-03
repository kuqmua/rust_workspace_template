#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct PgnStartsWithOne(pg_crud_cmn::PgnBase);

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum PgnStartsWithOneTryNewEr {
    LimitIsLessThanOrEqToZero,
    OffsetIsLessThanOne,
    OffsetPlusLimitIsIntOverflow,
}

impl pg_crud_cmn::DfltSomeOneEl for PgnStartsWithOne {
    fn dflt_some_one_el() -> Self {
        Self(pg_crud_cmn::PgnBase::default())
    }
}

impl pg_crud_cmn::DfltSomeOneElMaxPageSize for PgnStartsWithOne {
    fn dflt_some_one_el_max_page_size() -> Self {
        Self(pg_crud_cmn::PgnBase::default())
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum MbPkValue {
    False,
    True,
}

impl From<bool> for MbPkValue {
    fn from(value: bool) -> Self {
        if value {
            return Self::True;
        }
        Self::False
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum MbPkSql {
    Empty,
    PrimaryKey,
}

impl core::fmt::Display for MbPkSql {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match *self {
            Self::Empty => core::write!(f, ""),
            Self::PrimaryKey => core::write!(f, "primary key"),
        }
    }
}

#[must_use]
pub const fn mb_pk(value: MbPkValue) -> MbPkSql {
    match value {
        MbPkValue::False => MbPkSql::Empty,
        MbPkValue::True => MbPkSql::PrimaryKey,
    }
}
