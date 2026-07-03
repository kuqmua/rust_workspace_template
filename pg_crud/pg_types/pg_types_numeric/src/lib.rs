#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct BoolAsBool(pg_crud_cmn::EqOprtr);

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct F32AsFloat4(pg_crud_cmn::PgTypeGreaterThanVrt);

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct F64AsFloat8(pg_crud_cmn::PgTypeGreaterThanVrt);

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct I16AsInt2(pg_crud_cmn::PgTypeGreaterThanVrt);

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct I16AsSmallSerialInitByPg(pg_types_cmn::PgnStartsWithOne);

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct I32AsInt4(pg_crud_cmn::PgTypeGreaterThanVrt);

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct I32AsSerialInitByPg(pg_types_cmn::PgnStartsWithOne);

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct I64AsBigSerialInitByPg(pg_types_cmn::PgnStartsWithOne);

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct I64AsInt8(pg_crud_cmn::PgTypeGreaterThanVrt);

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct SqlxPgTypesPgMoneyAsMoney(wh_flts::EncodeFormat);

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct SqlxPgTypesPgRangeI32AsInt4Range(pg_crud_cmn::PgTypeGreaterThanVrt);

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct SqlxPgTypesPgRangeI64AsInt8Range(pg_crud_cmn::PgTypeGreaterThanVrt);
