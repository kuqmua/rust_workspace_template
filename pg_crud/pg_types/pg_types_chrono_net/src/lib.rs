#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange(
    pg_types_cmn::PgnStartsWithOne,
);

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange(pg_types_cmn::PgnStartsWithOne);

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange(
    pg_types_cmn::PgnStartsWithOne,
);

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz(pg_crud_cmn::Order);

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct SqlxTypesChronoNaiveDateAsDate(pg_crud_cmn::Order);

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct SqlxTypesChronoNaiveDateTimeAsTimestamp(pg_crud_cmn::Order);

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct SqlxTypesChronoNaiveTimeAsTime(pg_crud_cmn::Order);

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct SqlxTypesIpnetworkIpNetworkAsInet(wh_flts::RgxCase);

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct SqlxTypesMacAddressMacAddressAsMacAddr(wh_flts::RgxCase);
