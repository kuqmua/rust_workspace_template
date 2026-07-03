#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct SqlxPgTypesPgIntervalAsInterval(pg_types_cmn::PgnStartsWithOne);

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct SqlxTypesTimeTimeAsTime(pg_crud_cmn::Order);

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct SqlxTypesUuidUuidAsUuidInitByClient(wh_flts::RgxCase);

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct SqlxTypesUuidUuidAsUuidV4InitByPg(wh_flts::RgxCase);

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct StdVecVecU8AsBytea(wh_flts::EncodeFormat);

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct StringAsText(pg_crud_cmn::Order);
