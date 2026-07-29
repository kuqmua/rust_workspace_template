#![allow(clippy::unreachable, clippy::wildcard_enum_match_arm)] // schema branches are guarded by the PostgreSQL type category selected immediately before each match
const GENERATE_PG_TYPES_MAX_LEN: usize = 128usize;

#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, strum_macros::Display, optml::Optml)]
enum RustTypeName {
    I16,
    I32,
    I64,
    F32,
    F64,
    SqlxPgTypesPgMoney,
    Bool,
    String,
    StdVecVecU8,
    SqlxTypesChronoNaiveTime,
    SqlxTypesTimeTime,
    SqlxPgTypesPgInterval,
    SqlxTypesChronoNaiveDate,
    SqlxTypesChronoNaiveDateTime,
    SqlxTypesChronoDateTimeSqlxTypesChronoUtc,
    SqlxTypesUuidUuid,
    SqlxTypesIpnetworkIpNetwork,
    SqlxTypesMacAddressMacAddress,
    SqlxPgTypesPgRangeI32,
    SqlxPgTypesPgRangeI64,
    SqlxPgTypesPgRangeSqlxTypesChronoNaiveDate,
    SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTime,
    SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc,
}
impl From<&PgType> for RustTypeName {
    fn from(v: &PgType) -> Self {
        match &v {
                PgType::F32AsFloat4 => Self::F32,
                PgType::F64AsFloat8 => Self::F64,
                PgType::I16AsInt2 | PgType::I16AsSmallSerialInitializationByPg => Self::I16,
                PgType::I32AsInt4 | PgType::I32AsSerialInitializationByPg => Self::I32,
                PgType::I64AsInt8 | PgType::I64AsBigSerialInitializationByPg => Self::I64,
                PgType::SqlxPgTypesPgMoneyAsMoney => Self::SqlxPgTypesPgMoney,
                PgType::BoolAsBool => Self::Bool,
                PgType::StringAsText => Self::String,
                PgType::StdVecVecU8AsBytea => Self::StdVecVecU8,
                PgType::SqlxTypesChronoNaiveTimeAsTime => Self::SqlxTypesChronoNaiveTime,
                PgType::SqlxTypesTimeTimeAsTime => Self::SqlxTypesTimeTime,
                PgType::SqlxPgTypesPgIntervalAsInterval => Self::SqlxPgTypesPgInterval,
                PgType::SqlxTypesChronoNaiveDateAsDate => Self::SqlxTypesChronoNaiveDate,
                PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp => Self::SqlxTypesChronoNaiveDateTime,
                PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtc,
                PgType::SqlxTypesUuidUuidAsUuidV4InitializationByPg | PgType::SqlxTypesUuidUuidAsUuidInitializationByClient => Self::SqlxTypesUuidUuid,
                PgType::SqlxTypesIpnetworkIpNetworkAsInet => Self::SqlxTypesIpnetworkIpNetwork,
                PgType::SqlxTypesMacAddressMacAddressAsMacAddr => Self::SqlxTypesMacAddressMacAddress,
                PgType::SqlxPgTypesPgRangeI32AsInt4Range => Self::SqlxPgTypesPgRangeI32,
                PgType::SqlxPgTypesPgRangeI64AsInt8Range => Self::SqlxPgTypesPgRangeI64,
                PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => Self::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDate,
                PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => Self::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTime,
                PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => Self::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc,
            }
    }
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, strum_macros::Display, optml::Optml)]
enum PgTypeName {
    Int2,
    Int4,
    Int8,
    Float4,
    Float8,
    SmallSerialInitializationByPg,
    SerialInitializationByPg,
    BigSerialInitializationByPg,
    Money,
    Bool,
    Text,
    Bytea,
    Time,
    Interval,
    Date,
    Timestamp,
    TimestampTz,
    UuidV4InitializationByPg,
    UuidInitializationByClient,
    Inet,
    MacAddr,
    Int4Range,
    Int8Range,
    DateRange,
    TimestampRange,
    TimestampTzRange,
}
impl From<&PgType> for PgTypeName {
    fn from(v: &PgType) -> Self {
        match &v {
                PgType::I16AsInt2 => Self::Int2,
                PgType::I32AsInt4 => Self::Int4,
                PgType::I64AsInt8 => Self::Int8,
                PgType::F32AsFloat4 => Self::Float4,
                PgType::F64AsFloat8 => Self::Float8,
                PgType::I16AsSmallSerialInitializationByPg => Self::SmallSerialInitializationByPg,
                PgType::I32AsSerialInitializationByPg => Self::SerialInitializationByPg,
                PgType::I64AsBigSerialInitializationByPg => Self::BigSerialInitializationByPg,
                PgType::SqlxPgTypesPgMoneyAsMoney => Self::Money,
                PgType::BoolAsBool => Self::Bool,
                PgType::StringAsText => Self::Text,
                PgType::StdVecVecU8AsBytea => Self::Bytea,
                PgType::SqlxTypesChronoNaiveTimeAsTime | PgType::SqlxTypesTimeTimeAsTime => Self::Time,
                PgType::SqlxPgTypesPgIntervalAsInterval => Self::Interval,
                PgType::SqlxTypesChronoNaiveDateAsDate => Self::Date,
                PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp => Self::Timestamp,
                PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => Self::TimestampTz,
                PgType::SqlxTypesUuidUuidAsUuidV4InitializationByPg => Self::UuidV4InitializationByPg,
                PgType::SqlxTypesUuidUuidAsUuidInitializationByClient => Self::UuidInitializationByClient,
                PgType::SqlxTypesIpnetworkIpNetworkAsInet => Self::Inet,
                PgType::SqlxTypesMacAddressMacAddressAsMacAddr => Self::MacAddr,
                PgType::SqlxPgTypesPgRangeI32AsInt4Range => Self::Int4Range,
                PgType::SqlxPgTypesPgRangeI64AsInt8Range => Self::Int8Range,
                PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => Self::DateRange,
                PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => Self::TimestampRange,
                PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => Self::TimestampTzRange,
            }
    }
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    std::hash::Hash,
    serde::Serialize,
    serde::Deserialize,
    strum_macros::Display,
    strum_macros::EnumIter,
    optml::Optml,
)]
enum PgType {
    I16AsInt2,
    I32AsInt4,
    I64AsInt8,
    F32AsFloat4,
    F64AsFloat8,
    I16AsSmallSerialInitializationByPg,
    I32AsSerialInitializationByPg,
    I64AsBigSerialInitializationByPg,
    SqlxPgTypesPgMoneyAsMoney,
    // SqlxTypesBigDecimalAsNumeric, remove coz dont know how to deserialize with scale i64
    BoolAsBool,
    StringAsText,
    StdVecVecU8AsBytea,
    SqlxTypesChronoNaiveTimeAsTime,
    SqlxTypesTimeTimeAsTime,
    SqlxPgTypesPgIntervalAsInterval,
    SqlxTypesChronoNaiveDateAsDate,
    SqlxTypesChronoNaiveDateTimeAsTimestamp,
    SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz,
    SqlxTypesUuidUuidAsUuidV4InitializationByPg,
    SqlxTypesUuidUuidAsUuidInitializationByClient,
    SqlxTypesIpnetworkIpNetworkAsInet,
    SqlxTypesMacAddressMacAddressAsMacAddr,
    SqlxPgTypesPgRangeI32AsInt4Range,
    SqlxPgTypesPgRangeI64AsInt8Range,
    SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange,
    SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange,
    SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange,
}
#[derive(Clone, Copy)]
enum WireKind {
    Bool,
    Bytes,
    Date,
    Float32,
    Float64,
    Inet,
    Int16,
    Int32,
    Int64,
    Interval,
    Mac,
    RangeDate,
    RangeInt32,
    RangeInt64,
    RangeTimestamp,
    RangeTimestampTz,
    String,
    TimeChrono,
    TimeTime,
    Timestamp,
    TimestampTz,
    Uuid,
}
#[derive(Clone, Copy)]
enum FilterKind {
    Bool,
    Bytes,
    Date,
    IntervalOrInet,
    Mac,
    Money,
    Number,
    Range,
    String,
    Time,
    Timestamp,
    TimestampTz,
    Uuid,
}
#[derive(Clone, Copy)]
enum CanBePrimaryKey {
    False,
    True,
}
#[derive(Clone, Copy, newtype::AsRefInner, newtype::FromInner, newtype::ToTokens)]
struct PgSqlName(&'static str);
impl PgType {
    fn can_be_nullable(self) -> CanBeNullable {
        crate::sqlx::can_be_nullable(self.spec())
    }
    fn spec(
        self,
    ) -> crate::model::PgTypeSpec<CanBeNullable, CanBePrimaryKey, FilterKind, PgSqlName, WireKind>
    {
        match self {
            Self::I16AsInt2 => crate::model::PgTypeSpec {
                can_be_primary_key: CanBePrimaryKey::False,
                can_be_nullable: CanBeNullable::True,
                filter_kind: FilterKind::Number,
                pg_name: PgSqlName::from(str_constants::PG_CRUD_PG_INT2),
                wire_kind: WireKind::Int16,
            },
            Self::I32AsInt4 => crate::model::PgTypeSpec {
                can_be_primary_key: CanBePrimaryKey::False,
                can_be_nullable: CanBeNullable::True,
                filter_kind: FilterKind::Number,
                pg_name: PgSqlName::from(str_constants::PG_CRUD_PG_INT4),
                wire_kind: WireKind::Int32,
            },
            Self::I64AsInt8 => crate::model::PgTypeSpec {
                can_be_primary_key: CanBePrimaryKey::False,
                can_be_nullable: CanBeNullable::True,
                filter_kind: FilterKind::Number,
                pg_name: PgSqlName::from(str_constants::PG_CRUD_PG_INT8),
                wire_kind: WireKind::Int64,
            },
            Self::F32AsFloat4 => crate::model::PgTypeSpec {
                can_be_primary_key: CanBePrimaryKey::False,
                can_be_nullable: CanBeNullable::True,
                filter_kind: FilterKind::Number,
                pg_name: PgSqlName::from(str_constants::PG_CRUD_PG_FLOAT4),
                wire_kind: WireKind::Float32,
            },
            Self::F64AsFloat8 => crate::model::PgTypeSpec {
                can_be_primary_key: CanBePrimaryKey::False,
                can_be_nullable: CanBeNullable::True,
                filter_kind: FilterKind::Number,
                pg_name: PgSqlName::from(str_constants::PG_CRUD_PG_FLOAT8),
                wire_kind: WireKind::Float64,
            },
            Self::I16AsSmallSerialInitializationByPg => crate::model::PgTypeSpec {
                can_be_primary_key: CanBePrimaryKey::True,
                can_be_nullable: CanBeNullable::False,
                filter_kind: FilterKind::Number,
                pg_name: PgSqlName::from(str_constants::PG_CRUD_PG_SMALLSERIAL),
                wire_kind: WireKind::Int16,
            },
            Self::I32AsSerialInitializationByPg => crate::model::PgTypeSpec {
                can_be_primary_key: CanBePrimaryKey::True,
                can_be_nullable: CanBeNullable::False,
                filter_kind: FilterKind::Number,
                pg_name: PgSqlName::from(str_constants::PG_CRUD_PG_SERIAL),
                wire_kind: WireKind::Int32,
            },
            Self::I64AsBigSerialInitializationByPg => crate::model::PgTypeSpec {
                can_be_primary_key: CanBePrimaryKey::True,
                can_be_nullable: CanBeNullable::False,
                filter_kind: FilterKind::Number,
                pg_name: PgSqlName::from(str_constants::PG_CRUD_PG_BIGSERIAL),
                wire_kind: WireKind::Int64,
            },
            Self::SqlxPgTypesPgMoneyAsMoney => crate::model::PgTypeSpec {
                can_be_primary_key: CanBePrimaryKey::False,
                can_be_nullable: CanBeNullable::True,
                filter_kind: FilterKind::Money,
                pg_name: PgSqlName::from(str_constants::PG_CRUD_PG_MONEY),
                wire_kind: WireKind::Int64,
            },
            Self::BoolAsBool => crate::model::PgTypeSpec {
                can_be_primary_key: CanBePrimaryKey::False,
                can_be_nullable: CanBeNullable::True,
                filter_kind: FilterKind::Bool,
                pg_name: PgSqlName::from(str_constants::PG_CRUD_PG_BOOL),
                wire_kind: WireKind::Bool,
            },
            Self::StringAsText => crate::model::PgTypeSpec {
                can_be_primary_key: CanBePrimaryKey::False,
                can_be_nullable: CanBeNullable::True,
                filter_kind: FilterKind::String,
                pg_name: PgSqlName::from(str_constants::PG_CRUD_PG_TEXT),
                wire_kind: WireKind::String,
            },
            Self::StdVecVecU8AsBytea => crate::model::PgTypeSpec {
                can_be_primary_key: CanBePrimaryKey::False,
                can_be_nullable: CanBeNullable::True,
                filter_kind: FilterKind::Bytes,
                pg_name: PgSqlName::from(str_constants::PG_CRUD_PG_BYTEA),
                wire_kind: WireKind::Bytes,
            },
            Self::SqlxTypesChronoNaiveTimeAsTime => crate::model::PgTypeSpec {
                can_be_primary_key: CanBePrimaryKey::False,
                can_be_nullable: CanBeNullable::True,
                filter_kind: FilterKind::Time,
                pg_name: PgSqlName::from(str_constants::PG_CRUD_PG_TIME),
                wire_kind: WireKind::TimeChrono,
            },
            Self::SqlxTypesTimeTimeAsTime => crate::model::PgTypeSpec {
                can_be_primary_key: CanBePrimaryKey::False,
                can_be_nullable: CanBeNullable::True,
                filter_kind: FilterKind::Time,
                pg_name: PgSqlName::from(str_constants::PG_CRUD_PG_TIME),
                wire_kind: WireKind::TimeTime,
            },
            Self::SqlxPgTypesPgIntervalAsInterval => crate::model::PgTypeSpec {
                can_be_primary_key: CanBePrimaryKey::False,
                can_be_nullable: CanBeNullable::True,
                filter_kind: FilterKind::IntervalOrInet,
                pg_name: PgSqlName::from(str_constants::PG_CRUD_PG_INTERVAL),
                wire_kind: WireKind::Interval,
            },
            Self::SqlxTypesChronoNaiveDateAsDate => crate::model::PgTypeSpec {
                can_be_primary_key: CanBePrimaryKey::False,
                can_be_nullable: CanBeNullable::True,
                filter_kind: FilterKind::Date,
                pg_name: PgSqlName::from(str_constants::PG_CRUD_PG_DATE),
                wire_kind: WireKind::Date,
            },
            Self::SqlxTypesChronoNaiveDateTimeAsTimestamp => crate::model::PgTypeSpec {
                can_be_primary_key: CanBePrimaryKey::False,
                can_be_nullable: CanBeNullable::True,
                filter_kind: FilterKind::Timestamp,
                pg_name: PgSqlName::from(str_constants::PG_CRUD_PG_TIMESTAMP),
                wire_kind: WireKind::Timestamp,
            },
            Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => {
                crate::model::PgTypeSpec {
                    can_be_primary_key: CanBePrimaryKey::False,
                    can_be_nullable: CanBeNullable::True,
                    filter_kind: FilterKind::TimestampTz,
                    pg_name: PgSqlName::from(str_constants::PG_CRUD_PG_TIMESTAMPTZ),
                    wire_kind: WireKind::TimestampTz,
                }
            }
            Self::SqlxTypesUuidUuidAsUuidV4InitializationByPg => crate::model::PgTypeSpec {
                can_be_primary_key: CanBePrimaryKey::True,
                can_be_nullable: CanBeNullable::False,
                filter_kind: FilterKind::Uuid,
                pg_name: PgSqlName::from(str_constants::PG_CRUD_PG_UUID),
                wire_kind: WireKind::Uuid,
            },
            Self::SqlxTypesUuidUuidAsUuidInitializationByClient => crate::model::PgTypeSpec {
                can_be_primary_key: CanBePrimaryKey::False,
                can_be_nullable: CanBeNullable::True,
                filter_kind: FilterKind::Uuid,
                pg_name: PgSqlName::from(str_constants::PG_CRUD_PG_UUID),
                wire_kind: WireKind::Uuid,
            },
            Self::SqlxTypesIpnetworkIpNetworkAsInet => crate::model::PgTypeSpec {
                can_be_primary_key: CanBePrimaryKey::False,
                can_be_nullable: CanBeNullable::True,
                filter_kind: FilterKind::IntervalOrInet,
                pg_name: PgSqlName::from(str_constants::PG_CRUD_PG_INET),
                wire_kind: WireKind::Inet,
            },
            Self::SqlxTypesMacAddressMacAddressAsMacAddr => crate::model::PgTypeSpec {
                can_be_primary_key: CanBePrimaryKey::False,
                can_be_nullable: CanBeNullable::True,
                filter_kind: FilterKind::Mac,
                pg_name: PgSqlName::from(str_constants::PG_CRUD_PG_MACADDR),
                wire_kind: WireKind::Mac,
            },
            Self::SqlxPgTypesPgRangeI32AsInt4Range => crate::model::PgTypeSpec {
                can_be_primary_key: CanBePrimaryKey::False,
                can_be_nullable: CanBeNullable::True,
                filter_kind: FilterKind::Range,
                pg_name: PgSqlName::from(str_constants::PG_CRUD_PG_INT4RANGE),
                wire_kind: WireKind::RangeInt32,
            },
            Self::SqlxPgTypesPgRangeI64AsInt8Range => crate::model::PgTypeSpec {
                can_be_primary_key: CanBePrimaryKey::False,
                can_be_nullable: CanBeNullable::True,
                filter_kind: FilterKind::Range,
                pg_name: PgSqlName::from(str_constants::PG_CRUD_PG_INT8RANGE),
                wire_kind: WireKind::RangeInt64,
            },
            Self::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => {
                crate::model::PgTypeSpec {
                    can_be_primary_key: CanBePrimaryKey::False,
                    can_be_nullable: CanBeNullable::True,
                    filter_kind: FilterKind::Range,
                    pg_name: PgSqlName::from(str_constants::PG_CRUD_PG_DATERANGE),
                    wire_kind: WireKind::RangeDate,
                }
            }
            Self::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => {
                crate::model::PgTypeSpec {
                    can_be_primary_key: CanBePrimaryKey::False,
                    can_be_nullable: CanBeNullable::True,
                    filter_kind: FilterKind::Range,
                    pg_name: PgSqlName::from(str_constants::PG_CRUD_PG_TSRANGE),
                    wire_kind: WireKind::RangeTimestamp,
                }
            }
            Self::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => {
                crate::model::PgTypeSpec {
                    can_be_primary_key: CanBePrimaryKey::False,
                    can_be_nullable: CanBeNullable::True,
                    filter_kind: FilterKind::Range,
                    pg_name: PgSqlName::from(str_constants::PG_CRUD_PG_TSTZRANGE),
                    wire_kind: WireKind::RangeTimestampTz,
                }
            }
        }
    }
}
#[derive(Clone, Copy)]
enum CanBeNullable {
    False,
    True,
}
impl quote::ToTokens for PgType {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        quote::format_ident!("{}", self.to_string()).to_tokens(tokens);
    }
}
impl From<&Range> for PgType {
    fn from(v: &Range) -> Self {
        match v {
            Range::I32AsInt4 => Self::I32AsInt4,
            Range::I64AsInt8 => Self::I64AsInt8,
            Range::SqlxTypesChronoNaiveDateAsDate => Self::SqlxTypesChronoNaiveDateAsDate,
            Range::SqlxTypesChronoNaiveDateTimeAsTimestamp => {
                Self::SqlxTypesChronoNaiveDateTimeAsTimestamp
            }
            Range::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => {
                Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz
            }
        }
    }
}
#[allow(clippy::arbitrary_source_item_ordering)]
enum Range {
    I32AsInt4,
    I64AsInt8,
    SqlxTypesChronoNaiveDateAsDate,
    SqlxTypesChronoNaiveDateTimeAsTimestamp,
    SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz,
}
impl TryFrom<&PgType> for Range {
    type Error = ();
    fn try_from(v: &PgType) -> Result<Self, Self::Error> {
        match &v {
                PgType::I16AsInt2
                | PgType::I32AsInt4
                | PgType::I64AsInt8
                | PgType::F32AsFloat4
                | PgType::F64AsFloat8
                | PgType::I16AsSmallSerialInitializationByPg
                | PgType::I32AsSerialInitializationByPg
                | PgType::I64AsBigSerialInitializationByPg
                | PgType::SqlxPgTypesPgMoneyAsMoney
                | PgType::BoolAsBool
                | PgType::StringAsText
                | PgType::StdVecVecU8AsBytea
                | PgType::SqlxTypesChronoNaiveTimeAsTime
                | PgType::SqlxTypesTimeTimeAsTime
                | PgType::SqlxPgTypesPgIntervalAsInterval
                | PgType::SqlxTypesChronoNaiveDateAsDate
                | PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp
                | PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz
                | PgType::SqlxTypesUuidUuidAsUuidV4InitializationByPg
                | PgType::SqlxTypesUuidUuidAsUuidInitializationByClient
                | PgType::SqlxTypesIpnetworkIpNetworkAsInet
                | PgType::SqlxTypesMacAddressMacAddressAsMacAddr => Err(()),
                PgType::SqlxPgTypesPgRangeI32AsInt4Range => Ok(Self::I32AsInt4),
                PgType::SqlxPgTypesPgRangeI64AsInt8Range => Ok(Self::I64AsInt8),
                PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => Ok(Self::SqlxTypesChronoNaiveDateAsDate),
                PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => Ok(Self::SqlxTypesChronoNaiveDateTimeAsTimestamp),
                PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => Ok(Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz),
            }
    }
}
impl std::fmt::Display for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            naming::parameter::SelfNonNullUpperCamelCase::from_display(&PgType::from(self))
        )
    }
}
impl quote::ToTokens for Range {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        quote::format_ident!("{}", self.to_string()).to_tokens(tokens);
    }
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    std::hash::Hash,
    serde::Serialize,
    serde::Deserialize,
    strum_macros::Display,
    strum_macros::EnumIter,
    optml::Optml,
)]
enum PgTypePattern {
    Standard,
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    std::hash::Hash,
    serde::Serialize,
    serde::Deserialize,
    optml::Optml,
)]
#[serde(try_from = "PgTypeRecordRaw")]
struct PgTypeRecord {
    pg_type: PgType,
    is_nullable: pg_crud_macros_common::IsNullable,
    pg_type_pattern: PgTypePattern,
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, serde::Deserialize, optml::Optml)]
struct PgTypeRecordRaw {
    pg_type: PgType,
    is_nullable: pg_crud_macros_common::IsNullable,
    pg_type_pattern: PgTypePattern,
}
impl TryFrom<PgTypeRecordRaw> for PgTypeRecord {
    type Error = String;
    fn try_from(v: PgTypeRecordRaw) -> Result<Self, Self::Error> {
        let cant_supp_nullable_variants_message = str_constants::CANT_SUPPORT_NULLABLE_VARIANTS;
        match &v.pg_type.can_be_nullable() {
            CanBeNullable::False => {
                if matches!(&v.is_nullable, pg_crud_macros_common::IsNullable::True) {
                    return Err(format!("{cant_supp_nullable_variants_message}{v:#?}"));
                }
                Ok(Self {
                    pg_type: v.pg_type,
                    is_nullable: v.is_nullable,
                    pg_type_pattern: v.pg_type_pattern,
                })
            }
            CanBeNullable::True => Ok(Self {
                pg_type: v.pg_type,
                is_nullable: v.is_nullable,
                pg_type_pattern: v.pg_type_pattern,
            }),
        }
    }
}
#[derive(Debug, newtype::DerefTarget, newtype::IntoInnerFrom, serde::Deserialize)]
#[serde(try_from = "Vec<PgTypeRecord>")]
struct GeneratePgTypeRecords(Vec<PgTypeRecord>);
#[derive(Debug, newtype::DerefTarget, serde::Deserialize)]
#[serde(try_from = "Vec<PgType>")]
struct GeneratePgTypes(Vec<PgType>);
#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
#[error("{self:?}")]
struct GeneratePgTypesLengthError;
impl TryFrom<Vec<PgTypeRecord>> for GeneratePgTypeRecords {
    type Error = GeneratePgTypesLengthError;

    fn try_from(value: Vec<PgTypeRecord>) -> Result<Self, Self::Error> {
        if value.len() > GENERATE_PG_TYPES_MAX_LEN {
            Err(GeneratePgTypesLengthError)
        } else {
            Ok(Self(value))
        }
    }
}
impl TryFrom<Vec<PgType>> for GeneratePgTypes {
    type Error = GeneratePgTypesLengthError;

    fn try_from(value: Vec<PgType>) -> Result<Self, Self::Error> {
        if value.len() > GENERATE_PG_TYPES_MAX_LEN {
            Err(GeneratePgTypesLengthError)
        } else {
            Ok(Self(value))
        }
    }
}
#[derive(Debug, serde::Deserialize, optml::Optml)]
enum GeneratePgTypesConfigVariant {
    All,
    Concrete(GeneratePgTypeRecords),
    Subset(GeneratePgTypes),
}
#[derive(Clone, Copy, Debug, Default, serde::Deserialize)]
#[serde(from = "bool")]
#[derive(newtype::FromInner)]
struct GenerateSecretText(bool);

#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, serde::Deserialize, optml::Optml)]
struct GeneratePgTypesConfig {
    variant: GeneratePgTypesConfigVariant,
    pg_table_cols_write_into_file: macros_helpers::ts_writer::ShouldWriteTokenStreamIntoFile,
    whole_write_into_file: macros_helpers::ts_writer::ShouldWriteTokenStreamIntoFile,
    #[serde(default)]
    generate_secret_text: GenerateSecretText,
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, optml::Optml)]
enum PgTypeInitializationTryNew {
    F64AsFloat8,
    StringAsText,
    SqlxTypesChronoNaiveTimeAsTime,
    SqlxTypesTimeTimeAsTime,
    SqlxTypesChronoNaiveDateAsDate,
    SqlxTypesChronoNaiveDateTimeAsTimestamp,
    SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz,
    SqlxPgTypesPgRangeI32AsInt4Range,
    SqlxPgTypesPgRangeI64AsInt8Range,
    SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange,
    SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange,
    SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange,
}
impl TryFrom<&PgType> for PgTypeInitializationTryNew {
    type Error = ();
    fn try_from(v: &PgType) -> Result<Self, Self::Error> {
        match v {
                PgType::I16AsInt2
                | PgType::I32AsInt4
                | PgType::I64AsInt8
                | PgType::F32AsFloat4
                | PgType::I16AsSmallSerialInitializationByPg
                | PgType::I32AsSerialInitializationByPg
                | PgType::I64AsBigSerialInitializationByPg
                | PgType::SqlxPgTypesPgMoneyAsMoney
                | PgType::BoolAsBool
                | PgType::StdVecVecU8AsBytea
                | PgType::SqlxPgTypesPgIntervalAsInterval
                | PgType::SqlxTypesUuidUuidAsUuidV4InitializationByPg
                | PgType::SqlxTypesUuidUuidAsUuidInitializationByClient
                | PgType::SqlxTypesIpnetworkIpNetworkAsInet
                | PgType::SqlxTypesMacAddressMacAddressAsMacAddr => Err(()),
                PgType::F64AsFloat8 => Ok(Self::F64AsFloat8),
                PgType::StringAsText => Ok(Self::StringAsText),
                PgType::SqlxTypesChronoNaiveTimeAsTime => Ok(Self::SqlxTypesChronoNaiveTimeAsTime),
                PgType::SqlxTypesTimeTimeAsTime => Ok(Self::SqlxTypesTimeTimeAsTime),
                PgType::SqlxTypesChronoNaiveDateAsDate => Ok(Self::SqlxTypesChronoNaiveDateAsDate),
                PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp => Ok(Self::SqlxTypesChronoNaiveDateTimeAsTimestamp),
                PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => Ok(Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz),
                PgType::SqlxPgTypesPgRangeI32AsInt4Range => Ok(Self::SqlxPgTypesPgRangeI32AsInt4Range),
                PgType::SqlxPgTypesPgRangeI64AsInt8Range => Ok(Self::SqlxPgTypesPgRangeI64AsInt8Range),
                PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => Ok(Self::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange),
                PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => Ok(Self::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange),
                PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => Ok(Self::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange),
            }
    }
}
impl From<&PgTypeInitializationTryNew> for PgType {
    fn from(v: &PgTypeInitializationTryNew) -> Self {
        match v {
                PgTypeInitializationTryNew::F64AsFloat8 => Self::F64AsFloat8,
                PgTypeInitializationTryNew::StringAsText => Self::StringAsText,
                PgTypeInitializationTryNew::SqlxTypesChronoNaiveTimeAsTime => Self::SqlxTypesChronoNaiveTimeAsTime,
                PgTypeInitializationTryNew::SqlxTypesTimeTimeAsTime => Self::SqlxTypesTimeTimeAsTime,
                PgTypeInitializationTryNew::SqlxTypesChronoNaiveDateAsDate => Self::SqlxTypesChronoNaiveDateAsDate,
                PgTypeInitializationTryNew::SqlxTypesChronoNaiveDateTimeAsTimestamp => Self::SqlxTypesChronoNaiveDateTimeAsTimestamp,
                PgTypeInitializationTryNew::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz,
                PgTypeInitializationTryNew::SqlxPgTypesPgRangeI32AsInt4Range => Self::SqlxPgTypesPgRangeI32AsInt4Range,
                PgTypeInitializationTryNew::SqlxPgTypesPgRangeI64AsInt8Range => Self::SqlxPgTypesPgRangeI64AsInt8Range,
                PgTypeInitializationTryNew::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => Self::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange,
                PgTypeInitializationTryNew::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => Self::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange,
                PgTypeInitializationTryNew::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => Self::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange,
            }
    }
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, optml::Optml)]
enum PgTypeImplTryNewForDe {
    StringAsText,
    SqlxTypesChronoNaiveTimeAsTime,
    SqlxTypesTimeTimeAsTime,
    SqlxTypesChronoNaiveDateAsDate,
    SqlxPgTypesPgRangeI32AsInt4Range,
    SqlxPgTypesPgRangeI64AsInt8Range,
    SqlxTypesUuidUuidAsUuidV4InitializationByPg,
    SqlxTypesUuidUuidAsUuidInitializationByClient,
}
#[derive(Debug, optml::Optml)]
enum PgTypeImplNewForDeserializeOrTryNewForDe {
    NewForDeserialize,
    TryNewForDe(PgTypeImplTryNewForDe),
}
#[derive(Debug, optml::Optml)]
enum PgTypeDeserialize {
    Derive,
    ImplNewForDeserializeOrTryNewForDe(PgTypeImplNewForDeserializeOrTryNewForDe),
}
impl From<&PgType> for PgTypeDeserialize {
    fn from(v: &PgType) -> Self {
        match v {
                PgType::I16AsInt2
                | PgType::I32AsInt4
                | PgType::I64AsInt8
                | PgType::F32AsFloat4
                | PgType::I16AsSmallSerialInitializationByPg
                | PgType::I32AsSerialInitializationByPg
                | PgType::I64AsBigSerialInitializationByPg
                | PgType::SqlxPgTypesPgMoneyAsMoney
                | PgType::BoolAsBool
                | PgType::StdVecVecU8AsBytea
                | PgType::SqlxTypesUuidUuidAsUuidV4InitializationByPg
                | PgType::SqlxTypesUuidUuidAsUuidInitializationByClient
                | PgType::SqlxTypesIpnetworkIpNetworkAsInet
                | PgType::SqlxTypesMacAddressMacAddressAsMacAddr
                | PgType::F64AsFloat8 => Self::Derive,
                PgType::StringAsText => Self::ImplNewForDeserializeOrTryNewForDe(PgTypeImplNewForDeserializeOrTryNewForDe::TryNewForDe(PgTypeImplTryNewForDe::StringAsText)),
                PgType::SqlxTypesChronoNaiveTimeAsTime => Self::ImplNewForDeserializeOrTryNewForDe(PgTypeImplNewForDeserializeOrTryNewForDe::TryNewForDe(PgTypeImplTryNewForDe::SqlxTypesChronoNaiveTimeAsTime)),
                PgType::SqlxTypesTimeTimeAsTime => Self::ImplNewForDeserializeOrTryNewForDe(PgTypeImplNewForDeserializeOrTryNewForDe::TryNewForDe(PgTypeImplTryNewForDe::SqlxTypesTimeTimeAsTime)),
                PgType::SqlxTypesChronoNaiveDateAsDate => Self::ImplNewForDeserializeOrTryNewForDe(PgTypeImplNewForDeserializeOrTryNewForDe::TryNewForDe(PgTypeImplTryNewForDe::SqlxTypesChronoNaiveDateAsDate)),
                PgType::SqlxPgTypesPgRangeI32AsInt4Range => Self::ImplNewForDeserializeOrTryNewForDe(PgTypeImplNewForDeserializeOrTryNewForDe::TryNewForDe(PgTypeImplTryNewForDe::SqlxPgTypesPgRangeI32AsInt4Range)),
                PgType::SqlxPgTypesPgRangeI64AsInt8Range => Self::ImplNewForDeserializeOrTryNewForDe(PgTypeImplNewForDeserializeOrTryNewForDe::TryNewForDe(PgTypeImplTryNewForDe::SqlxPgTypesPgRangeI64AsInt8Range)),
                PgType::SqlxPgTypesPgIntervalAsInterval |
                PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp |
                PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz |
                PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange |
                PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange |
                PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => Self::ImplNewForDeserializeOrTryNewForDe(PgTypeImplNewForDeserializeOrTryNewForDe::NewForDeserialize),
            }
    }
}
#[derive(Debug, newtype::FromInner)]
pub struct ParsedGeneratePgTypesConfig(GeneratePgTypesConfig);

#[derive(Debug)]
pub struct BuiltGeneratePgTypesModel {
    config: GeneratePgTypesConfig,
    entry_count: PgTypesModelEntryCount,
}
#[derive(Debug)]
pub struct ValidatedGeneratePgTypesConfig {
    config: GeneratePgTypesConfig,
    entry_count: PgTypesModelEntryCount,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, newtype::FromInner, newtype::IntoInnerFrom)]
pub struct PgTypesModelEntryCount(usize);
impl ValidatedGeneratePgTypesConfig {
    #[must_use]
    pub const fn entry_count(&self) -> PgTypesModelEntryCount {
        self.entry_count
    }
}
#[derive(Debug, thiserror::Error, newtype::FromInner)]
#[error(transparent)]
pub struct SerdeJsonGeneratePgTypesError(serde_json::Error);

#[derive(Debug, thiserror::Error)]
pub enum GeneratePgTypesPipelineError {
    #[error("{0}")]
    Parse(SerdeJsonGeneratePgTypesError),
}
pub fn parse_generate_pg_types(
    input: macros_helpers::ts_writer::ProcMacro2TokenStreamRef<'_>,
) -> Result<ParsedGeneratePgTypesConfig, GeneratePgTypesPipelineError> {
    serde_json::from_str::<GeneratePgTypesConfig>(&input.as_ref().to_string())
        .map(ParsedGeneratePgTypesConfig)
        .map_err(|error| {
            GeneratePgTypesPipelineError::Parse(SerdeJsonGeneratePgTypesError::from(error))
        })
}
pub fn validate_generate_pg_types(
    built: BuiltGeneratePgTypesModel,
) -> Result<ValidatedGeneratePgTypesConfig, GeneratePgTypesPipelineError> {
    Ok(ValidatedGeneratePgTypesConfig {
        config: built.config,
        entry_count: built.entry_count,
    })
}
pub fn build_generate_pg_types(
    parsed: ParsedGeneratePgTypesConfig,
) -> Result<BuiltGeneratePgTypesModel, GeneratePgTypesPipelineError> {
    let entry_count = PgTypesModelEntryCount::from(match &parsed.0.variant {
        GeneratePgTypesConfigVariant::All => <PgType as strum::IntoEnumIterator>::iter().count(),
        GeneratePgTypesConfigVariant::Concrete(records) => records.len(),
        GeneratePgTypesConfigVariant::Subset(types) => types.len(),
    });
    Ok(BuiltGeneratePgTypesModel {
        config: parsed.0,
        entry_count,
    })
}
#[must_use]
pub fn generate_pg_types(
    input: macros_helpers::ts_writer::ProcMacro2TokenStreamRef<'_>,
) -> macros_helpers::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream {
    match parse_generate_pg_types(input)
        .and_then(build_generate_pg_types)
        .and_then(validate_generate_pg_types)
    {
        Ok(validated) => emit_generate_pg_types(validated),
        Err(error) => {
            let message = format!("failed to parse GeneratePgTypesConfig: {error}");
            macros_helpers::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream::from(
                quote::quote! { compile_error!(#message); },
            )
        }
    }
}
#[must_use]
pub fn emit_generate_pg_types(
    validated: ValidatedGeneratePgTypesConfig,
) -> macros_helpers::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream {
    panic_location::panic_location();
    let generate_pg_types_config = validated.config;
    let allow_clippy_arbitrary_src_item_ordering =
        token_patterns::AllowClippyArbitrarySrcItemOrdering;
    let as_upper_camel_case = naming::AsUpperCamelCase;
    let column_snake_case = naming::ColumnSnakeCase;
    let contains_null_byte_upper_camel_case = naming::ContainsNullByteUpperCamelCase;
    let core_default = token_patterns::CoreDefault;
    let create_snake_case = naming::CreateSnakeCase;
    let date_naive_snake_case = naming::DateNaiveSnakeCase;
    let date_naive_upper_camel_case = naming::DateNaiveUpperCamelCase;
    let date_snake_case = naming::DateSnakeCase;
    let date_upper_camel_case = naming::DateUpperCamelCase;
    let days_snake_case = naming::DaysSnakeCase;
    let earlier_date_not_supported_upper_camel_case = naming::EarlierDateNotSupportedUpperCamelCase;
    let earliest_supported_date_snake_case = naming::EarliestSupportedDateSnakeCase;
    let end_snake_case = naming::EndSnakeCase;
    let end_upper_camel_case = naming::EndUpperCamelCase;
    let eq_upper_camel_case = naming::EqUpperCamelCase;
    let error_snake_case = naming::ErrorSnakeCase;
    let excluded_start_greater_than_excluded_end_upper_camel_case =
        naming::ExcludedStartGreaterThanExcludedEndUpperCamelCase;
    let excluded_start_greater_than_included_end_upper_camel_case =
        naming::ExcludedStartGreaterThanIncludedEndUpperCamelCase;
    let excluded_upper_camel_case = naming::ExcludedUpperCamelCase;
    let f32_token_stream = token_patterns::F32;
    let generate_pg_types_mod_snake_case = naming::GeneratePgTypesModSnakeCase;
    let hour_snake_case = naming::HourSnakeCase;
    let i16_token_stream = token_patterns::I16;
    let i32_token_stream = token_patterns::I32;
    let i64_token_stream = token_patterns::I64;
    let included_end_cannot_be_max_upper_camel_case = naming::IncludedEndCannotBeMaxUpperCamelCase;
    let included_start_greater_than_excluded_end_upper_camel_case =
        naming::IncludedStartGreaterThanExcludedEndUpperCamelCase;
    let included_start_greater_than_included_end_upper_camel_case =
        naming::IncludedStartGreaterThanIncludedEndUpperCamelCase;
    let included_upper_camel_case = naming::IncludedUpperCamelCase;
    let increment_snake_case = naming::IncrementSnakeCase;
    let invalid_hour_or_minute_or_second_or_microsecond_upper_camel_case =
        naming::InvalidHourOrMinuteOrSecondOrMicrosecondUpperCamelCase;
    let max_snake_case = naming::MaxSnakeCase;
    let micro_snake_case = naming::MicroSnakeCase;
    let microsecond_snake_case = naming::MicrosecondSnakeCase;
    let microseconds_snake_case = naming::MicrosecondsSnakeCase;
    let min_snake_case = naming::MinSnakeCase;
    let minute_snake_case = naming::MinuteSnakeCase;
    let months_snake_case = naming::MonthsSnakeCase;
    let must_use = token_patterns::MustUse;
    let nanosecond_precision_is_not_supported_upper_camel_case =
        naming::NanosecondPrecisionIsNotSupportedUpperCamelCase;
    let nanosecond_snake_case = naming::NanosecondSnakeCase;
    let near_zero_snake_case = naming::NearZeroSnakeCase;
    let negative_less_typical_snake_case = naming::NegativeLessTypicalSnakeCase;
    let negative_more_typical_snake_case = naming::NegativeMoreTypicalSnakeCase;
    let new_snake_case = naming::NewSnakeCase;
    let not_uuid_upper_camel_case = naming::NotUuidUpperCamelCase;
    let optional_update_snake_case = naming::OptionalUpdateSnakeCase;
    let optional_vec_create_snake_case = naming::OptionalVecCreateSnakeCase;
    let pg_crud_common_default_some_one_element_call =
        token_patterns::PgCrudCommonDefaultSomeOneElementCall;
    let pg_type_primary_key_upper_camel_case = naming::PgTypePrimaryKeyUpperCamelCase;
    let pg_type_upper_camel_case = naming::PgTypeUpperCamelCase;
    let positive_less_typical_snake_case = naming::PositiveLessTypicalSnakeCase;
    let positive_more_typical_snake_case = naming::PositiveMoreTypicalSnakeCase;
    let query_snake_case = naming::QuerySnakeCase;
    let read_ids_and_create_into_read_snake_case = naming::ReadIdsAndCreateIntoReadSnakeCase;
    let read_ids_into_read_snake_case = naming::ReadIdsIntoReadSnakeCase;
    let read_ids_into_table_type_snake_case = naming::ReadIdsIntoTableTypeSnakeCase;
    let read_ids_into_update_snake_case = naming::ReadIdsIntoUpdateSnakeCase;
    let read_ids_snake_case = naming::ReadIdsSnakeCase;
    let read_ids_to_2_dimensions_vec_read_inner_snake_case =
        naming::ReadIdsTo2DimensionsVecReadInnerSnakeCase;
    let read_ids_upper_camel_case = naming::ReadIdsUpperCamelCase;
    let read_into_table_type_snake_case = naming::ReadIntoTableTypeSnakeCase;
    let read_snake_case = naming::ReadSnakeCase;
    let read_upper_camel_case = naming::ReadUpperCamelCase;
    let sec_snake_case = naming::SecSnakeCase;
    let second_snake_case = naming::SecondSnakeCase;
    let self_snake_case = naming::SelfSnakeCase;
    let self_upper_camel_case = naming::SelfUpperCamelCase;
    let start_snake_case = naming::StartSnakeCase;
    let start_upper_camel_case = naming::StartUpperCamelCase;
    let string_token_stream = token_patterns::StringTokenStream;
    let time_snake_case = naming::TimeSnakeCase;
    let time_upper_camel_case = naming::TimeUpperCamelCase;
    let to_err_string_snake_case = naming::ToErrStringSnakeCase;
    let try_new_snake_case = naming::TryNewSnakeCase;
    let table_type_snake_case = naming::TableTypeSnakeCase;
    let table_type_upper_camel_case = naming::TableTypeUpperCamelCase;
    let u8_token_stream = token_patterns::U8;
    let u32_token_stream = token_patterns::U32;
    let unbounded_upper_camel_case = naming::UnboundedUpperCamelCase;
    let update_upper_camel_case = naming::UpdateUpperCamelCase;
    let v_snake_case = naming::VSnakeCase;
    let (cols_token_stream, mut pg_type_array) = {
        let generate_variants = |should_include: &dyn Fn(&PgType) -> bool| {
            let pg_type_iter =
                <PgType as strum::IntoEnumIterator>::iter().filter(|element| should_include(element));
            let capacity = pg_type_iter.size_hint().1.unwrap_or_default().saturating_mul(2);
            pg_type_iter.fold(Vec::with_capacity(capacity), |mut acc0, element| {
                match &crate::sqlx::can_be_nullable(element.spec()) {
                    CanBeNullable::False => {
                        acc0.push(PgTypeRecord {
                            pg_type: element,
                            is_nullable: pg_crud_macros_common::IsNullable::False,
                            pg_type_pattern: PgTypePattern::Standard,
                        });
                    },
                    CanBeNullable::True => {
                        <pg_crud_macros_common::IsNullable as strum::IntoEnumIterator>::iter().for_each(|el1| {
                            acc0.push(PgTypeRecord {
                                pg_type: element,
                                is_nullable: el1,
                                pg_type_pattern: PgTypePattern::Standard,
                            });
                        });
                    },
                }
                acc0
            })
        };
        let pg_type_records = match generate_pg_types_config.variant {
            GeneratePgTypesConfigVariant::All => generate_variants(&|_| true),
            GeneratePgTypesConfigVariant::Subset(types) => {
                let type_set = types
                    .iter()
                    .copied()
                    .collect::<std::collections::HashSet<PgType>>();
                generate_variants(&|pg_type| type_set.contains(pg_type))
            }
            GeneratePgTypesConfigVariant::Concrete(v) => Vec::from(v),
        };
        {
            let mut check_accumulator = std::collections::HashSet::with_capacity(pg_type_records.len());
            let duplicate_found = pg_type_records.iter().any(|element| !check_accumulator.insert(*element));
            if duplicate_found {
                let message_value = str_constants::DUPLICATE_PG_TYPE_CONFIG_ENTRY;
                return macros_helpers::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream::from(
                    quote::quote! {compile_error!(#message_value);},
                );
            }
        }
        let pg_type_records_len = pg_type_records.len();
        pg_type_records
            .into_iter()
            .fold(
                (
                    Vec::with_capacity(pg_type_records_len.saturating_mul(2)),
                    std::collections::HashSet::with_capacity(pg_type_records_len.saturating_mul(2)),
                ),
                |(mut records_accumulator, mut seen), element| {
                    let mut add_record = |is_nullable, pg_type_pattern| {
                        let pg_type_record = PgTypeRecord {
                            pg_type: element.pg_type,
                            is_nullable,
                            pg_type_pattern,
                        };
                        if seen.insert(pg_type_record) {
                            records_accumulator.push(pg_type_record);
                        }
                    };
                    match &element.is_nullable {
                        pg_crud_macros_common::IsNullable::False => {
                            add_record(element.is_nullable, element.pg_type_pattern);
                        }
                        pg_crud_macros_common::IsNullable::True => {
                            add_record(pg_crud_macros_common::IsNullable::False, PgTypePattern::Standard);
                            add_record(element.is_nullable, element.pg_type_pattern);
                        }
                    }
                    (records_accumulator, seen)
                },
            )
            .0
    }
    .into_iter()
    .enumerate()
    .map(|(i, element)| {
        enum PgTypeOrPgTypeTestCases {
            PgType,
            PgTypeTestCases,
        }
        enum IsNonNullStandardCanBePrimaryKey {
            False,
            True,
        }
        enum StartOrEnd {
            End,
            Start,
        }
        enum ShouldImplFrom {
            False,
            True,
        }
        enum IntRangeType {
            SqlxPgTypesPgRangeI32AsInt4Range,
            SqlxPgTypesPgRangeI64AsInt8Range,
        }
        fn generate_pg_range_conversion_token_stream(match_token_stream: &dyn quote::ToTokens, input_token_stream: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
            let arms_token_stream = quote::quote! {
                std::ops::Bound::Included(v_af65ccce) => std::ops::Bound::Included(#input_token_stream),
                std::ops::Bound::Excluded(v_af65ccce) => std::ops::Bound::Excluded(#input_token_stream),
                std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
            };
            quote::quote! {
                sqlx::postgres::types::PgRange {
                    start: match #match_token_stream.start { #arms_token_stream },
                    end: match #match_token_stream.end { #arms_token_stream },
                }
            }
        }
        let pg_type = &element.pg_type;
        let is_nullable = &element.is_nullable;
        let pg_type_pattern = &element.pg_type_pattern;
        let pg_type_initialization_try_new_try_from_pg_type = PgTypeInitializationTryNew::try_from(pg_type);
        let pg_type_deserialize = PgTypeDeserialize::from(pg_type);
        let range_try_from_pg_type = Range::try_from(pg_type);
        let range_try_from_pg_type_is_ok = range_try_from_pg_type.is_ok();
        let import = pg_crud_macros_common::Import::PgCrudCommon;
        let import_non_primary_key_pg_type_read_ids_token_stream = quote::quote! {#import::NonPrimaryKeyPgTypeReadIds};
        let none_token_stream = quote::quote! {None};
        let empty_token_stream = proc_macro2::TokenStream::new();
        let dot_clone_token_stream = quote::quote! {.clone()};
        let maybe_dot_clone_token_stream: &dyn quote::ToTokens = if matches!(&pg_type_pattern, PgTypePattern::Standard) &&
            matches!(&is_nullable, pg_crud_macros_common::IsNullable::False) && !matches!(
                pg_type,
                PgType::StdVecVecU8AsBytea | PgType::StringAsText
            )
        {
            &empty_token_stream
        } else {
            &dot_clone_token_stream
        };
        let generate_v_initialization_ts0 = |ts: &dyn quote::ToTokens| pg_crud_macros_common::generate_v_initialization_token_stream(&import, &ts);
        let generate_identifier_str = |
            pg_type_parameter: &PgType,
            is_nullable_parameter: &pg_crud_macros_common::IsNullable,
            _pg_type_pattern_parameter: &PgTypePattern
        | {
            let rust_type_name = RustTypeName::from(pg_type_parameter);
            let pg_type_name = PgTypeName::from(pg_type_parameter);
            let is_nullable_rust = is_nullable_parameter.rust();
            let non_null_or_nullable_str = is_nullable_parameter.non_null_or_nullable_str();
            format!("{is_nullable_rust}{rust_type_name}{as_upper_camel_case}{non_null_or_nullable_str}{pg_type_name}")
        };
        let generate_identifier_token_stream = |
            pg_type_parameter: &PgType,
            is_nullable_parameter: &pg_crud_macros_common::IsNullable,
            pg_type_pattern_parameter: &PgTypePattern
        | {
            let identifier_str = generate_identifier_str(
                pg_type_parameter,
                is_nullable_parameter,
                pg_type_pattern_parameter
            );
            let identifier = quote::format_ident!("{}", identifier_str);
            quote::quote! {#identifier}
        };
        let identifier = &generate_identifier_token_stream(pg_type, is_nullable, pg_type_pattern);
        let generate_identifier_standard_non_null_token_stream = |v: &PgType| generate_identifier_token_stream(v, &pg_crud_macros_common::IsNullable::False, &PgTypePattern::Standard);
        let identifier_standard_non_null_upper_camel_case = generate_identifier_standard_non_null_token_stream(pg_type);
        let generate_as_trait_token_stream = |ts: &dyn quote::ToTokens, pg_type_or_pg_type_test_cases: &PgTypeOrPgTypeTestCases| {
            let trait_token_stream = match &pg_type_or_pg_type_test_cases {
                PgTypeOrPgTypeTestCases::PgType => quote::quote! {PgType},
                PgTypeOrPgTypeTestCases::PgTypeTestCases => quote::quote! {PgTypeTestCases},
            };
            quote::quote! {<#ts as #import::#trait_token_stream>}
        };
        let generate_as_pg_type_token_stream = |ts: &dyn quote::ToTokens| generate_as_trait_token_stream(&ts, &PgTypeOrPgTypeTestCases::PgType);
        let generate_as_pg_type_test_cases_token_stream = |ts: &dyn quote::ToTokens| generate_as_trait_token_stream(&ts, &PgTypeOrPgTypeTestCases::PgTypeTestCases);
        let self_as_pg_type_token_stream = generate_as_pg_type_token_stream(&self_upper_camel_case);
        let identifier_standard_non_null_as_pg_type_token_stream = generate_as_pg_type_token_stream(&identifier_standard_non_null_upper_camel_case);
        let self_pg_type_as_pg_type_token_stream = generate_as_pg_type_token_stream(&quote::quote! {Self::#pg_type_upper_camel_case});
        let identifier_standard_non_null_as_pg_type_test_cases_token_stream = generate_as_pg_type_test_cases_token_stream(&identifier_standard_non_null_upper_camel_case);
        let generate_identifier_standard_non_null_origin_token_stream = |pg_type_parameter: &PgType| naming::parameter::SelfOriginUpperCamelCase::from_tokens(
            &generate_identifier_standard_non_null_token_stream(pg_type_parameter)
        );
        let identifier_standard_non_null_origin_upper_camel_case = generate_identifier_standard_non_null_origin_token_stream(pg_type);
        let identifier_origin_upper_camel_case = naming::parameter::SelfOriginUpperCamelCase::from_tokens(&identifier);
        let identifier_origin_wire_upper_camel_case = quote::format_ident!("{}Wire", identifier_origin_upper_camel_case.to_string());
        let generate_impl_wrapper_traits_token_stream = |identifier_token_stream: &dyn quote::ToTokens,
                                          target_token_stream: &dyn quote::ToTokens,
                                          should_impl_from: ShouldImplFrom| {
            let impl_from_token_stream = match should_impl_from {
                ShouldImplFrom::False => proc_macro2::TokenStream::new(),
                ShouldImplFrom::True => quote::quote! {
                    impl From<#target_token_stream> for #identifier_token_stream {
                        fn from(value: #target_token_stream) -> Self { Self(value) }
                    }
                },
            };
            quote::quote! {
                #impl_from_token_stream
                impl AsRef<#target_token_stream> for #identifier_token_stream {
                    fn as_ref(&self) -> &#target_token_stream { &self.0 }
                }
                impl std::borrow::Borrow<#target_token_stream> for #identifier_token_stream {
                    fn borrow(&self) -> &#target_token_stream { &self.0 }
                }
            }
        };
        let sqlx_types_chrono_naive_date_as_non_null_date_origin_upper_camel_case = generate_identifier_standard_non_null_origin_token_stream(&PgType::SqlxTypesChronoNaiveDateAsDate);
        let sqlx_types_chrono_naive_time_as_non_null_time_origin_upper_camel_case = generate_identifier_standard_non_null_origin_token_stream(&PgType::SqlxTypesChronoNaiveTimeAsTime);
        let sqlx_types_chrono_naive_date_time_as_non_null_timestamp_origin_upper_camel_case = generate_identifier_standard_non_null_origin_token_stream(&PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp);
        let sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_non_null_timestamptz_origin_upper_camel_case = generate_identifier_standard_non_null_origin_token_stream(&PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz);
        let generate_identifier_standard_non_null_origin_try_new_error_token_stream = |pg_type_parameter: &PgType| naming::parameter::SelfOriginTryNewErrorUpperCamelCase::from_tokens(
            &generate_identifier_standard_non_null_token_stream(pg_type_parameter)
        );
        let sqlx_types_chrono_naive_date_as_non_null_date_origin_try_new_error_upper_camel_case = generate_identifier_standard_non_null_origin_try_new_error_token_stream(&PgType::SqlxTypesChronoNaiveDateAsDate);
        let sqlx_types_chrono_naive_time_as_non_null_time_origin_try_new_error_upper_camel_case = generate_identifier_standard_non_null_origin_try_new_error_token_stream(&PgType::SqlxTypesChronoNaiveTimeAsTime);
        let sqlx_types_chrono_naive_date_time_as_non_null_timestamp_origin_try_new_error_upper_camel_case = generate_identifier_standard_non_null_origin_try_new_error_token_stream(&PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp);
        let sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_non_null_timestamptz_origin_try_new_error_upper_camel_case = generate_identifier_standard_non_null_origin_try_new_error_token_stream(&PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz);
        let inner_type_standard_non_null_token_stream = match &pg_type {
            PgType::F32AsFloat4 => quote::quote! {f32},
            PgType::F64AsFloat8 => quote::quote! {f64},
            PgType::I16AsInt2 | PgType::I16AsSmallSerialInitializationByPg => quote::quote! {i16},
            PgType::I32AsInt4 | PgType::I32AsSerialInitializationByPg => quote::quote! {i32},
            PgType::I64AsInt8 | PgType::I64AsBigSerialInitializationByPg => quote::quote! {i64},
            PgType::SqlxPgTypesPgMoneyAsMoney => quote::quote! {sqlx::postgres::types::PgMoney},
            PgType::BoolAsBool => quote::quote! {bool},
            PgType::StringAsText => quote::quote! {String},
            PgType::StdVecVecU8AsBytea => quote::quote! {Vec<u8>},
            PgType::SqlxTypesChronoNaiveTimeAsTime => quote::quote! {sqlx::types::chrono::NaiveTime},
            PgType::SqlxTypesTimeTimeAsTime => quote::quote! {sqlx::types::time::Time},
            PgType::SqlxPgTypesPgIntervalAsInterval => quote::quote! {sqlx::postgres::types::PgInterval},
            PgType::SqlxTypesChronoNaiveDateAsDate => quote::quote! {sqlx::types::chrono::NaiveDate},
            PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp => quote::quote! {sqlx::types::chrono::NaiveDateTime},
            PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => quote::quote! {sqlx::types::chrono::DateTime::<sqlx::types::chrono::Utc>},
            PgType::SqlxTypesUuidUuidAsUuidV4InitializationByPg | PgType::SqlxTypesUuidUuidAsUuidInitializationByClient => quote::quote! {uuid::Uuid},
            PgType::SqlxTypesIpnetworkIpNetworkAsInet => quote::quote! {sqlx::types::ipnetwork::IpNetwork},
            PgType::SqlxTypesMacAddressMacAddressAsMacAddr => quote::quote! {sqlx::types::mac_address::MacAddress},
            PgType::SqlxPgTypesPgRangeI32AsInt4Range => quote::quote! {sqlx::postgres::types::PgRange<i32>},
            PgType::SqlxPgTypesPgRangeI64AsInt8Range => quote::quote! {sqlx::postgres::types::PgRange<i64>},
            PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => quote::quote! {sqlx::postgres::types::PgRange<sqlx::types::chrono::NaiveDate>},
            PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => quote::quote! {sqlx::postgres::types::PgRange<sqlx::types::chrono::NaiveDateTime>},
            PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => quote::quote! {sqlx::postgres::types::PgRange<sqlx::types::chrono::DateTime::<sqlx::types::chrono::Utc>>},
        };
        let pg_type_dsc = pg_type.spec();
        let pg_name = crate::catalog::pg_name(pg_type_dsc);
        let open_api_object_builder_token_stream = |schema_type: &dyn quote::ToTokens,
                                          extra: &dyn quote::ToTokens| {
            quote::quote! {
                utoipa::openapi::ObjectBuilder::new()
                    .schema_type(utoipa::openapi::schema::Type::#schema_type)
                    .description(Some(concat!("PostgreSQL ", #pg_name)))
                    #extra
                    .build()
            }
        };
        let non_null_open_api_schema_token_stream = match crate::schema::wire_kind(pg_type_dsc) {
            WireKind::Int16 => open_api_object_builder_token_stream(
                &quote::quote! {Integer},
                &quote::quote! {
                    .format(Some(utoipa::openapi::SchemaFormat::KnownFormat(utoipa::openapi::KnownFormat::Int32)))
                    .minimum(Some(-32768.0))
                    .maximum(Some(32767.0))
                    .examples([serde_json::json!(42)])
                },
            ),
            WireKind::Int32 => open_api_object_builder_token_stream(
                &quote::quote! {Integer},
                &quote::quote! {
                    .format(Some(utoipa::openapi::SchemaFormat::KnownFormat(utoipa::openapi::KnownFormat::Int32)))
                    .minimum(Some(-2147483648.0))
                    .maximum(Some(2147483647.0))
                    .examples([serde_json::json!(42)])
                },
            ),
            WireKind::Int64 => open_api_object_builder_token_stream(
                &quote::quote! {Integer},
                &quote::quote! {
                    .format(Some(utoipa::openapi::SchemaFormat::KnownFormat(utoipa::openapi::KnownFormat::Int64)))
                    .examples([serde_json::json!(42)])
                },
            ),
            WireKind::Float32 => open_api_object_builder_token_stream(
                &quote::quote! {Number},
                &quote::quote! {
                    .format(Some(utoipa::openapi::SchemaFormat::KnownFormat(utoipa::openapi::KnownFormat::Float)))
                    .examples([serde_json::json!(42.0)])
                },
            ),
            WireKind::Float64 => open_api_object_builder_token_stream(
                &quote::quote! {Number},
                &quote::quote! {
                    .format(Some(utoipa::openapi::SchemaFormat::KnownFormat(utoipa::openapi::KnownFormat::Double)))
                    .examples([serde_json::json!(42.0)])
                },
            ),
            WireKind::Bool => open_api_object_builder_token_stream(
                &quote::quote! {Boolean},
                &quote::quote! {.examples([serde_json::json!(true)])},
            ),
            WireKind::String => open_api_object_builder_token_stream(
                &quote::quote! {String},
                &quote::quote! {.examples([serde_json::json!("example")])},
            ),
            WireKind::Date => open_api_object_builder_token_stream(
                &quote::quote! {String},
                &quote::quote! {
                    .format(Some(utoipa::openapi::SchemaFormat::KnownFormat(utoipa::openapi::KnownFormat::Date)))
                    .examples([serde_json::json!("2024-01-01")])
                },
            ),
            WireKind::Uuid => open_api_object_builder_token_stream(
                &quote::quote! {String},
                &quote::quote! {
                    .format(Some(utoipa::openapi::SchemaFormat::Custom("uuid".to_owned())))
                    .examples([serde_json::json!("00000000-0000-4000-8000-000000000000")])
                },
            ),
            WireKind::Inet => open_api_object_builder_token_stream(
                &quote::quote! {String},
                &quote::quote! {
                    .format(Some(utoipa::openapi::SchemaFormat::Custom("ip-network".to_owned())))
                    .examples([serde_json::json!("192.0.2.1/32")])
                },
            ),
            WireKind::Bytes | WireKind::Mac => {
                let limits_token_stream = match crate::schema::wire_kind(pg_type_dsc) {
                    WireKind::Mac => quote::quote! {
                        .min_items(Some(6))
                        .max_items(Some(6))
                    },
                    _ => proc_macro2::TokenStream::new(),
                };
                let example_token_stream = if matches!(crate::schema::wire_kind(pg_type_dsc), WireKind::Mac) {
                    quote::quote! {[0, 17, 34, 51, 68, 85]}
                } else {
                    quote::quote! {[1, 2, 3]}
                };
                quote::quote! {
                    utoipa::openapi::ArrayBuilder::new()
                        .items(utoipa::openapi::ObjectBuilder::new()
                            .schema_type(utoipa::openapi::schema::Type::Integer)
                            .minimum(Some(0.0))
                            .maximum(Some(255.0)))
                        #limits_token_stream
                        .examples([serde_json::json!(#example_token_stream)])
                        .build()
                }
            }
            WireKind::TimeChrono | WireKind::TimeTime => {
                let (minute_name, second_name, microsecond_name) = match crate::schema::wire_kind(pg_type_dsc) {
                    WireKind::TimeChrono => (str_constants::MIN, str_constants::SEC, str_constants::MICRO),
                    WireKind::TimeTime => (str_constants::MINUTE, str_constants::SECOND_ALT, str_constants::MICROSECOND),
                    _ => unreachable!(),
                };
                quote::quote! {
                    utoipa::openapi::ObjectBuilder::new()
                        .schema_type(utoipa::openapi::schema::Type::Object)
                        .property("hour", utoipa::openapi::ObjectBuilder::new().schema_type(utoipa::openapi::schema::Type::Integer).minimum(Some(0.0)).maximum(Some(23.0)))
                        .property(#minute_name, utoipa::openapi::ObjectBuilder::new().schema_type(utoipa::openapi::schema::Type::Integer).minimum(Some(0.0)).maximum(Some(59.0)))
                        .property(#second_name, utoipa::openapi::ObjectBuilder::new().schema_type(utoipa::openapi::schema::Type::Integer).minimum(Some(0.0)).maximum(Some(59.0)))
                        .property(#microsecond_name, utoipa::openapi::ObjectBuilder::new().schema_type(utoipa::openapi::schema::Type::Integer).minimum(Some(0.0)).maximum(Some(999999.0)))
                        .required("hour")
                        .required(#minute_name)
                        .required(#second_name)
                        .required(#microsecond_name)
                        .examples([serde_json::json!({
                            "hour": 12,
                            #minute_name: 34,
                            #second_name: 56,
                            #microsecond_name: 789000
                        })])
                        .build()
                }
            }
            WireKind::Interval => quote::quote! {
                utoipa::openapi::ObjectBuilder::new()
                    .schema_type(utoipa::openapi::schema::Type::Object)
                    .property("months", utoipa::openapi::ObjectBuilder::new().schema_type(utoipa::openapi::schema::Type::Integer).format(Some(utoipa::openapi::SchemaFormat::KnownFormat(utoipa::openapi::KnownFormat::Int32))))
                    .property("days", utoipa::openapi::ObjectBuilder::new().schema_type(utoipa::openapi::schema::Type::Integer).format(Some(utoipa::openapi::SchemaFormat::KnownFormat(utoipa::openapi::KnownFormat::Int32))))
                    .property("microseconds", utoipa::openapi::ObjectBuilder::new().schema_type(utoipa::openapi::schema::Type::Integer).format(Some(utoipa::openapi::SchemaFormat::KnownFormat(utoipa::openapi::KnownFormat::Int64))))
                    .required("months")
                    .required("days")
                    .required("microseconds")
                    .examples([serde_json::json!({"months": 1, "days": 2, "microseconds": 3000000})])
                    .build()
            },
            WireKind::Timestamp | WireKind::TimestampTz => {
                let date_name = match crate::schema::wire_kind(pg_type_dsc) {
                    WireKind::Timestamp => str_constants::PG_CRUD_PG_DATE,
                    WireKind::TimestampTz => str_constants::DATE_NAIVE,
                    _ => unreachable!(),
                };
                quote::quote! {
                    utoipa::openapi::ObjectBuilder::new()
                        .schema_type(utoipa::openapi::schema::Type::Object)
                        .property(#date_name, utoipa::openapi::ObjectBuilder::new().schema_type(utoipa::openapi::schema::Type::String).format(Some(utoipa::openapi::SchemaFormat::KnownFormat(utoipa::openapi::KnownFormat::Date))))
                        .property("time", utoipa::openapi::ObjectBuilder::new()
                            .schema_type(utoipa::openapi::schema::Type::Object)
                            .property("hour", utoipa::openapi::ObjectBuilder::new().schema_type(utoipa::openapi::schema::Type::Integer).minimum(Some(0.0)).maximum(Some(23.0)))
                            .property("min", utoipa::openapi::ObjectBuilder::new().schema_type(utoipa::openapi::schema::Type::Integer).minimum(Some(0.0)).maximum(Some(59.0)))
                            .property("sec", utoipa::openapi::ObjectBuilder::new().schema_type(utoipa::openapi::schema::Type::Integer).minimum(Some(0.0)).maximum(Some(59.0)))
                            .property("micro", utoipa::openapi::ObjectBuilder::new().schema_type(utoipa::openapi::schema::Type::Integer).minimum(Some(0.0)).maximum(Some(999999.0)))
                            .required("hour").required("min").required("sec").required("micro"))
                        .required(#date_name)
                        .required("time")
                        .examples([serde_json::json!({
                            #date_name: "2024-01-01",
                            "time": {"hour": 12, "min": 34, "sec": 56, "micro": 789000}
                        })])
                        .build()
                }
            }
            WireKind::RangeInt32 | WireKind::RangeInt64 | WireKind::RangeDate | WireKind::RangeTimestamp | WireKind::RangeTimestampTz => {
                let range_value_schema_token_stream = match crate::schema::wire_kind(pg_type_dsc) {
                    WireKind::RangeInt32 => quote::quote! {utoipa::openapi::ObjectBuilder::new().schema_type(utoipa::openapi::schema::Type::Integer).format(Some(utoipa::openapi::SchemaFormat::KnownFormat(utoipa::openapi::KnownFormat::Int32)))},
                    WireKind::RangeInt64 => quote::quote! {utoipa::openapi::ObjectBuilder::new().schema_type(utoipa::openapi::schema::Type::Integer).format(Some(utoipa::openapi::SchemaFormat::KnownFormat(utoipa::openapi::KnownFormat::Int64)))},
                    WireKind::RangeDate => quote::quote! {utoipa::openapi::ObjectBuilder::new().schema_type(utoipa::openapi::schema::Type::String).format(Some(utoipa::openapi::SchemaFormat::KnownFormat(utoipa::openapi::KnownFormat::Date)))},
                    WireKind::RangeTimestamp | WireKind::RangeTimestampTz => {
                        let date_name = match crate::schema::wire_kind(pg_type_dsc) {
                            WireKind::RangeTimestamp => str_constants::PG_CRUD_PG_DATE,
                            WireKind::RangeTimestampTz => str_constants::DATE_NAIVE,
                            _ => unreachable!(),
                        };
                        quote::quote! {
                            utoipa::openapi::ObjectBuilder::new()
                                .schema_type(utoipa::openapi::schema::Type::Object)
                                .property(#date_name, utoipa::openapi::ObjectBuilder::new().schema_type(utoipa::openapi::schema::Type::String).format(Some(utoipa::openapi::SchemaFormat::KnownFormat(utoipa::openapi::KnownFormat::Date))))
                                .property("time", utoipa::openapi::ObjectBuilder::new()
                                    .schema_type(utoipa::openapi::schema::Type::Object)
                                    .property("hour", utoipa::openapi::ObjectBuilder::new().schema_type(utoipa::openapi::schema::Type::Integer).minimum(Some(0.0)).maximum(Some(23.0)))
                                    .property("min", utoipa::openapi::ObjectBuilder::new().schema_type(utoipa::openapi::schema::Type::Integer).minimum(Some(0.0)).maximum(Some(59.0)))
                                    .property("sec", utoipa::openapi::ObjectBuilder::new().schema_type(utoipa::openapi::schema::Type::Integer).minimum(Some(0.0)).maximum(Some(59.0)))
                                    .property("micro", utoipa::openapi::ObjectBuilder::new().schema_type(utoipa::openapi::schema::Type::Integer).minimum(Some(0.0)).maximum(Some(999999.0)))
                                    .required("hour").required("min").required("sec").required("micro"))
                                .required(#date_name)
                                .required("time")
                        }
                    }
                    _ => unreachable!(),
                };
                let range_bound_schema_token_stream = quote::quote! {
                    utoipa::openapi::schema::Schema::from(
                        utoipa::openapi::OneOfBuilder::new()
                            .item(utoipa::openapi::ObjectBuilder::new().schema_type(utoipa::openapi::schema::Type::Object).property("Included", #range_value_schema_token_stream).required("Included"))
                            .item(utoipa::openapi::ObjectBuilder::new().schema_type(utoipa::openapi::schema::Type::Object).property("Excluded", #range_value_schema_token_stream).required("Excluded"))
                            .item(utoipa::openapi::ObjectBuilder::new().schema_type(utoipa::openapi::schema::Type::String).enum_values(Some(["Unbounded"])))
                            .build()
                    )
                };
                quote::quote! {
                    utoipa::openapi::ObjectBuilder::new()
                        .schema_type(utoipa::openapi::schema::Type::Object)
                        .property("start", #range_bound_schema_token_stream)
                        .property("end", #range_bound_schema_token_stream)
                        .required("start")
                        .required("end")
                        .examples([serde_json::json!({"start": "Unbounded", "end": "Unbounded"})])
                        .build()
                }
            }
        };
        let open_api_schema_token_stream = match &is_nullable {
            pg_crud_macros_common::IsNullable::False => non_null_open_api_schema_token_stream,
            pg_crud_macros_common::IsNullable::True => quote::quote! {
                utoipa::openapi::Schema::OneOf(
                    utoipa::openapi::OneOfBuilder::new()
                        .item(
                            utoipa::openapi::ObjectBuilder::new()
                                .schema_type(utoipa::openapi::schema::Type::Null),
                        )
                        .item(#non_null_open_api_schema_token_stream)
                        .build()
                )
            },
        };
        let field_type_handle_optional_token_stream = pg_crud_macros_common::generate_optional_type_declaration_token_stream(&identifier_standard_non_null_origin_upper_camel_case);
        let field_type_handle: &dyn quote::ToTokens = match &pg_type_pattern {
            PgTypePattern::Standard => match &is_nullable {
                pg_crud_macros_common::IsNullable::False => &inner_type_standard_non_null_token_stream,
                pg_crud_macros_common::IsNullable::True => &field_type_handle_optional_token_stream,
            },
        };
        let generate_typical_pg_query_query_bind_token_stream = |ts: &dyn quote::ToTokens| match &is_nullable {
            pg_crud_macros_common::IsNullable::False => quote::quote! {
                if let Err(error) = #query_snake_case.as_mut().try_bind(#ts) {
                    return Err(#import::SqlxPostgresQueryBindError::from(error));
                }
                Ok(#query_snake_case)
            },
            pg_crud_macros_common::IsNullable::True => quote::quote! {
                if let Err(error) = #query_snake_case.as_mut().try_bind(#ts.0.0) {
                    return Err(#import::SqlxPostgresQueryBindError::from(error));
                }
                Ok(#query_snake_case)
            },
        };
        let typical_query_bind_token_stream = generate_typical_pg_query_query_bind_token_stream(&v_snake_case);
        let identifier_inner_type_optional_token_stream = pg_crud_macros_common::generate_optional_type_declaration_token_stream(&inner_type_standard_non_null_token_stream);
        let identifier_inner_type_token_stream: &dyn quote::ToTokens = match &element.pg_type_pattern {
            PgTypePattern::Standard => match &is_nullable {
                pg_crud_macros_common::IsNullable::False => &inner_type_standard_non_null_token_stream,
                pg_crud_macros_common::IsNullable::True => &identifier_inner_type_optional_token_stream,
            },
        };
        let can_be_primary_key = crate::sqlx::can_be_primary_key(pg_type.spec());
        let is_standard_non_null = if matches!((&pg_type_pattern, &is_nullable), (PgTypePattern::Standard, pg_crud_macros_common::IsNullable::False)) {
            pg_crud_macros_common::IsStandardNonNull::True
        } else {
            pg_crud_macros_common::IsStandardNonNull::False
        };
        let d_partial_ord = match &is_standard_non_null {
            pg_crud_macros_common::IsStandardNonNull::False => macros_helpers::derive_token_stream_builder::DPartialOrd::False,
            pg_crud_macros_common::IsStandardNonNull::True => match &pg_type {
                PgType::I16AsInt2
                | PgType::I32AsInt4
                | PgType::I64AsInt8
                | PgType::F32AsFloat4
                | PgType::F64AsFloat8
                | PgType::I16AsSmallSerialInitializationByPg
                | PgType::I32AsSerialInitializationByPg
                | PgType::I64AsBigSerialInitializationByPg
                | PgType::BoolAsBool
                | PgType::StringAsText
                | PgType::StdVecVecU8AsBytea
                | PgType::SqlxTypesChronoNaiveTimeAsTime
                | PgType::SqlxTypesTimeTimeAsTime
                | PgType::SqlxTypesChronoNaiveDateAsDate
                | PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp
                | PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz
                | PgType::SqlxTypesUuidUuidAsUuidV4InitializationByPg => macros_helpers::derive_token_stream_builder::DPartialOrd::True,
                PgType::SqlxPgTypesPgMoneyAsMoney
                | PgType::SqlxPgTypesPgIntervalAsInterval
                | PgType::SqlxTypesUuidUuidAsUuidInitializationByClient
                | PgType::SqlxTypesIpnetworkIpNetworkAsInet
                | PgType::SqlxTypesMacAddressMacAddressAsMacAddr
                | PgType::SqlxPgTypesPgRangeI32AsInt4Range
                | PgType::SqlxPgTypesPgRangeI64AsInt8Range
                | PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange
                | PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange
                | PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => macros_helpers::derive_token_stream_builder::DPartialOrd::False,
            },
        };
        let is_non_null_standard_can_be_primary_key = if matches!((&is_nullable, &pg_type_pattern, &can_be_primary_key), (pg_crud_macros_common::IsNullable::False, PgTypePattern::Standard, CanBePrimaryKey::True)) {
            IsNonNullStandardCanBePrimaryKey::True
        } else {
            IsNonNullStandardCanBePrimaryKey::False
        };
        let generate_start_or_end_upper_camel_case = |start_or_end: &StartOrEnd| -> &dyn naming::DisplayPlusToTokens {
            match &start_or_end {
                StartOrEnd::End => &end_upper_camel_case,
                StartOrEnd::Start => &start_upper_camel_case,
            }
        };
        let generate_start_or_end_snake_case = |start_or_end: &StartOrEnd| -> &dyn naming::DisplayPlusToTokens {
            match &start_or_end {
                StartOrEnd::End => &end_snake_case,
                StartOrEnd::Start => &start_snake_case,
            }
        };
        let (ser_derive_or_impl, de_derive_or_impl) = if matches!(&is_standard_non_null, pg_crud_macros_common::IsStandardNonNull::True) {
            #[allow(clippy::arbitrary_source_item_ordering)]
            enum ParameterNumber {
                Two,
                Three,
                Four,
            }
            impl ParameterNumber {
                const fn get_i(&self) -> usize {
                    match &self {
                        Self::Two => 1,
                        Self::Three => 2,
                        Self::Four => 3,
                    }
                }
            }
            let self_dot_zero_token_stream = quote::quote! {#self_snake_case.0};
            let parameter_number_two = ParameterNumber::Two;
            let parameter_number_three = ParameterNumber::Three;
            let parameter_number_four = ParameterNumber::Four;
            let identifier_standard_non_null_origin_double_quoted_token_stream = generate_quotes::dq_token_stream(&identifier_standard_non_null_origin_upper_camel_case);
            (
                {
                    let generate_impl_ser_for_identifier_standard_non_null_origin_tokens = |ts: &dyn quote::ToTokens| {
                        quote::quote! {
                            #[allow(unused_qualifications)]
                            #[allow(clippy::absolute_paths)]
                            #allow_clippy_arbitrary_src_item_ordering
                            const _: () = {
                                extern crate serde as _serde;
                                #[automatically_derived]
                                impl _serde::Serialize for #identifier_standard_non_null_origin_upper_camel_case {
                                    fn serialize<__S>(&self, __serializer: __S) -> Result<__S::Ok, __S::Error>
                                    where
                                        __S: _serde::Serializer,
                                    {
                                        #ts
                                    }
                                }
                            };
                        }
                    };
                    let generate_ser_cnt = |ts: &dyn quote::ToTokens| {
                        quote::quote! {_serde::Serializer::serialize_newtype_struct(__serializer, #identifier_standard_non_null_origin_double_quoted_token_stream, &#self_dot_zero_token_stream #ts)}
                    };
                    let generate_serde_state_initialization_token_stream = |parameter_number: &ParameterNumber| {
                        let parameter_number_token_stream = {
                            let ts = std::iter::repeat_with(|| quote::quote! {+ 1})
                                .take(parameter_number.get_i().saturating_add(1));
                            quote::quote! {#(#ts)*}
                        };
                        quote::quote! {
                            let mut __serde_state = _serde::Serializer::serialize_struct(__serializer, #identifier_standard_non_null_origin_double_quoted_token_stream, usize::from(false) #parameter_number_token_stream)?;
                        }
                    };
                    let serde_state_initialization_two_fields_token_stream = generate_serde_state_initialization_token_stream(&parameter_number_two);
                    let serde_state_initialization_three_fields_token_stream = generate_serde_state_initialization_token_stream(&parameter_number_three);
                    let serde_state_initialization_four_fields_token_stream = generate_serde_state_initialization_token_stream(&parameter_number_four);
                    let generate_ser_field_token_stream = |field_name: &dyn std::fmt::Display, third_parameter_token_stream: &dyn quote::ToTokens| {
                        let field_name_double_quoted_token_stream = generate_quotes::dq_token_stream(&field_name);
                        quote::quote! {_serde::ser::SerializeStruct::serialize_field(&mut __serde_state, #field_name_double_quoted_token_stream, #third_parameter_token_stream)?;}
                    };
                    let serde_ser_ser_struct_end_token_stream = quote::quote! {_serde::ser::SerializeStruct::end(__serde_state)};
                    let ser_cnt_start_end_token_stream = {
                        let generate_self_zero_tokens_token_stream = |ts: &dyn quote::ToTokens| {
                            quote::quote! {&#self_dot_zero_token_stream.#ts}
                        };
                        let start_ser_field_token_stream = generate_ser_field_token_stream(&start_snake_case, &generate_self_zero_tokens_token_stream(&start_snake_case));
                        let end_ser_field_token_stream = generate_ser_field_token_stream(&end_snake_case, &generate_self_zero_tokens_token_stream(&end_snake_case));
                        quote::quote! {
                            #serde_state_initialization_two_fields_token_stream
                            #start_ser_field_token_stream
                            #end_ser_field_token_stream
                            #serde_ser_ser_struct_end_token_stream
                        }
                    };
                    let impl_ser_for_non_null_origin_start_end_token_stream = generate_impl_ser_for_identifier_standard_non_null_origin_tokens(&ser_cnt_start_end_token_stream);
                    let impl_ser_for_uuid_uuid_token_stream = generate_impl_ser_for_identifier_standard_non_null_origin_tokens(&generate_ser_cnt(&proc_macro2::TokenStream::new()));
                    let generate_impl_ser_for_identifier_standard_non_null_origin_start_end_range_tokens = |ts: &dyn quote::ToTokens| {
                        let generate_ser_field_match_std_ops_bound_token_stream = |start_or_end: &StartOrEnd| {
                            let start_or_end_token_stream = generate_start_or_end_snake_case(start_or_end);
                            generate_ser_field_token_stream(
                                &start_or_end_token_stream,
                                &quote::quote! {
                                    &match self.0.#start_or_end_token_stream {
                                        std::ops::Bound::Included(v_7d755c7c) => std::ops::Bound::Included(#ts::#try_new_snake_case(v_7d755c7c).map_err(_serde::ser::Error::custom)?),
                                        std::ops::Bound::Excluded(v_cfbe64e9) => std::ops::Bound::Excluded(#ts::#try_new_snake_case(v_cfbe64e9).map_err(_serde::ser::Error::custom)?),
                                        std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
                                    }
                                },
                            )
                        };
                        let start_ser_field_token_stream = generate_ser_field_match_std_ops_bound_token_stream(&StartOrEnd::Start);
                        let end_ser_field_token_stream = generate_ser_field_match_std_ops_bound_token_stream(&StartOrEnd::End);
                        generate_impl_ser_for_identifier_standard_non_null_origin_tokens(&quote::quote! {
                            #serde_state_initialization_two_fields_token_stream
                            #start_ser_field_token_stream
                            #end_ser_field_token_stream
                            #serde_ser_ser_struct_end_token_stream
                        })
                    };
                    let generate_impl_ser_wrapping_self_zero_token_stream = |ts: &dyn quote::ToTokens|{
                        pg_crud_macros_common::DeriveOrImpl::Impl(macros_helpers::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream::from(generate_impl_ser_for_identifier_standard_non_null_origin_tokens(
                            &generate_ser_cnt(&ts)
                        )))
                    };
                    let generate_four_field_time_ser_token_stream = |f1: &dyn quote::ToTokens, f2: &dyn quote::ToTokens, f3: &dyn quote::ToTokens, f4: &dyn quote::ToTokens| quote::quote! {
                        #serde_state_initialization_four_fields_token_stream
                        #f1
                        #f2
                        #f3
                        #f4
                        #serde_ser_ser_struct_end_token_stream
                    };
                    match &pg_type {
                        PgType::I16AsInt2
                        | PgType::I32AsInt4
                        | PgType::I64AsInt8
                        | PgType::F32AsFloat4
                        | PgType::F64AsFloat8
                        | PgType::I16AsSmallSerialInitializationByPg
                        | PgType::I32AsSerialInitializationByPg
                        | PgType::I64AsBigSerialInitializationByPg
                        | PgType::BoolAsBool
                        | PgType::StringAsText
                        | PgType::StdVecVecU8AsBytea
                        | PgType::SqlxTypesChronoNaiveDateAsDate => pg_crud_macros_common::DeriveOrImpl::Derive,
                        PgType::SqlxTypesIpnetworkIpNetworkAsInet => {
                            pg_crud_macros_common::DeriveOrImpl::Impl(
                                macros_helpers::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream::from(
                                    generate_impl_ser_for_identifier_standard_non_null_origin_tokens(
                                        &quote::quote! {
                                            _serde::Serializer::collect_str(__serializer, &self.0)
                                        },
                                    ),
                                ),
                            )
                        },
                        PgType::SqlxPgTypesPgMoneyAsMoney => generate_impl_ser_wrapping_self_zero_token_stream(&quote::quote! {.0}),
                        PgType::SqlxTypesMacAddressMacAddressAsMacAddr => generate_impl_ser_wrapping_self_zero_token_stream(&quote::quote! {.bytes()}),
                        PgType::SqlxTypesChronoNaiveTimeAsTime => pg_crud_macros_common::DeriveOrImpl::Impl(macros_helpers::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream::from(generate_impl_ser_for_identifier_standard_non_null_origin_tokens(&{
                            let generate_field_inner_type_standard_non_null_token_stream_as_chrono_timelike_token_stream = |ts: &dyn quote::ToTokens| {
                                quote::quote! {&(<#inner_type_standard_non_null_token_stream as chrono::Timelike>::#ts)}
                            };
                            let hour_ser_field_token_stream = generate_ser_field_token_stream(&hour_snake_case, &generate_field_inner_type_standard_non_null_token_stream_as_chrono_timelike_token_stream(&quote::quote! {hour(&self.0)}));
                            let min_ser_field_token_stream = generate_ser_field_token_stream(&min_snake_case, &generate_field_inner_type_standard_non_null_token_stream_as_chrono_timelike_token_stream(&quote::quote! {minute(&self.0)}));
                            let sec_ser_field_token_stream = generate_ser_field_token_stream(&sec_snake_case, &generate_field_inner_type_standard_non_null_token_stream_as_chrono_timelike_token_stream(&quote::quote! {second(&self.0)}));
                            let micro_ser_field_token_stream = generate_ser_field_token_stream(
                                &micro_snake_case,
                                &generate_field_inner_type_standard_non_null_token_stream_as_chrono_timelike_token_stream(&quote::quote! {
                                    #nanosecond_snake_case(&self.0).checked_div(1000).expect("aea037b7")
                                }),
                            );
                            generate_four_field_time_ser_token_stream(&hour_ser_field_token_stream, &min_ser_field_token_stream, &sec_ser_field_token_stream, &micro_ser_field_token_stream)
                        }))),
                        PgType::SqlxTypesTimeTimeAsTime => pg_crud_macros_common::DeriveOrImpl::Impl(macros_helpers::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream::from(generate_impl_ser_for_identifier_standard_non_null_origin_tokens(&{
                            let generate_ser_field_self_zero_token_stream = |v: &dyn naming::DisplayPlusToTokens| generate_ser_field_token_stream(&v, &quote::quote! {&self.0.#v()});
                            let hour_ser_field_token_stream = generate_ser_field_self_zero_token_stream(&hour_snake_case);
                            let minute_ser_field_token_stream = generate_ser_field_self_zero_token_stream(&minute_snake_case);
                            let second_ser_field_token_stream = generate_ser_field_self_zero_token_stream(&second_snake_case);
                            let microsecond_ser_field_token_stream = generate_ser_field_self_zero_token_stream(&microsecond_snake_case);
                            generate_four_field_time_ser_token_stream(&hour_ser_field_token_stream, &minute_ser_field_token_stream, &second_ser_field_token_stream, &microsecond_ser_field_token_stream)
                        }))),
                        PgType::SqlxPgTypesPgIntervalAsInterval => pg_crud_macros_common::DeriveOrImpl::Impl(macros_helpers::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream::from(generate_impl_ser_for_identifier_standard_non_null_origin_tokens(&{
                            let generate_ser_field_handle_token_stream = |v: &dyn naming::DisplayPlusToTokens| generate_ser_field_token_stream(&v, &quote::quote! {&#self_dot_zero_token_stream.#v});
                            let months_ser_field_token_stream = generate_ser_field_handle_token_stream(&months_snake_case);
                            let days_ser_field_token_stream = generate_ser_field_handle_token_stream(&days_snake_case);
                            let microseconds_ser_field_token_stream = generate_ser_field_handle_token_stream(&microseconds_snake_case);
                            quote::quote! {
                                #serde_state_initialization_three_fields_token_stream
                                #months_ser_field_token_stream
                                #days_ser_field_token_stream
                                #microseconds_ser_field_token_stream
                                #serde_ser_ser_struct_end_token_stream
                            }
                        }))),
                        PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp => pg_crud_macros_common::DeriveOrImpl::Impl(macros_helpers::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream::from(generate_impl_ser_for_identifier_standard_non_null_origin_tokens(&{
                            enum DateOrTime {
                                Date,
                                Time,
                            }
                            let generate_ser_field_try_new_unwrap_token_stream = |date_or_time: &DateOrTime| {
                                let date_or_time_token_stream: &dyn naming::DisplayPlusToTokens = match &date_or_time {
                                    DateOrTime::Date => &date_snake_case,
                                    DateOrTime::Time => &time_snake_case,
                                };
                                generate_ser_field_token_stream(&date_or_time_token_stream, &{
                                    let identifier_token_stream_date: &dyn quote::ToTokens = match &date_or_time {
                                        DateOrTime::Date => &sqlx_types_chrono_naive_date_as_non_null_date_origin_upper_camel_case,
                                        DateOrTime::Time => &sqlx_types_chrono_naive_time_as_non_null_time_origin_upper_camel_case,
                                    };
                                    quote::quote! {
                                        &match #identifier_token_stream_date::#try_new_snake_case(self.0.#date_or_time_token_stream()) {
                                            Ok(v_b2ac0c33) => v_b2ac0c33,
                                            Err(error) => {
                                                return Err(_serde::ser::Error::custom(error));
                                            },
                                        }
                                    }
                                })
                            };
                            let date_ser_field_token_stream = generate_ser_field_try_new_unwrap_token_stream(&DateOrTime::Date);
                            let time_ser_field_token_stream = generate_ser_field_try_new_unwrap_token_stream(&DateOrTime::Time);
                            quote::quote! {
                                #serde_state_initialization_two_fields_token_stream
                                #date_ser_field_token_stream
                                #time_ser_field_token_stream
                                #serde_ser_ser_struct_end_token_stream
                            }
                        }))),
                        PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => pg_crud_macros_common::DeriveOrImpl::Impl(macros_helpers::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream::from(generate_impl_ser_for_identifier_standard_non_null_origin_tokens(&{
                            enum DateNaiveOrTime {
                                Date,
                                Time,
                            }
                            let generate_ser_field_try_new_unwrap_token_stream = |date_naive_or_time: &DateNaiveOrTime| {
                                let date_naive_or_time_token_stream: &dyn naming::DisplayPlusToTokens = match &date_naive_or_time {
                                    DateNaiveOrTime::Date => &date_naive_snake_case,
                                    DateNaiveOrTime::Time => &time_snake_case,
                                };
                                generate_ser_field_token_stream(&date_naive_or_time_token_stream, &{
                                    let identifier_token_stream_time: &dyn quote::ToTokens = match &date_naive_or_time {
                                        DateNaiveOrTime::Date => &sqlx_types_chrono_naive_date_as_non_null_date_origin_upper_camel_case,
                                        DateNaiveOrTime::Time => &sqlx_types_chrono_naive_time_as_non_null_time_origin_upper_camel_case,
                                    };
                                    quote::quote! {&#identifier_token_stream_time::#try_new_snake_case(self.0.#date_naive_or_time_token_stream()).map_err(_serde::ser::Error::custom)?}
                                })
                            };
                            let date_naive_ser_field_token_stream = generate_ser_field_try_new_unwrap_token_stream(&DateNaiveOrTime::Date);
                            let time_ser_field_token_stream = generate_ser_field_try_new_unwrap_token_stream(&DateNaiveOrTime::Time);
                            quote::quote! {
                                #serde_state_initialization_two_fields_token_stream
                                #date_naive_ser_field_token_stream
                                #time_ser_field_token_stream
                                #serde_ser_ser_struct_end_token_stream
                            }
                        }))),
                        PgType::SqlxTypesUuidUuidAsUuidV4InitializationByPg | PgType::SqlxTypesUuidUuidAsUuidInitializationByClient => pg_crud_macros_common::DeriveOrImpl::Impl(macros_helpers::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream::from(impl_ser_for_uuid_uuid_token_stream)),
                        PgType::SqlxPgTypesPgRangeI32AsInt4Range | PgType::SqlxPgTypesPgRangeI64AsInt8Range => pg_crud_macros_common::DeriveOrImpl::Impl(macros_helpers::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream::from(impl_ser_for_non_null_origin_start_end_token_stream)),
                        PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => pg_crud_macros_common::DeriveOrImpl::Impl(macros_helpers::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream::from(generate_impl_ser_for_identifier_standard_non_null_origin_start_end_range_tokens(&sqlx_types_chrono_naive_date_as_non_null_date_origin_upper_camel_case))),
                        PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => pg_crud_macros_common::DeriveOrImpl::Impl(macros_helpers::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream::from(generate_impl_ser_for_identifier_standard_non_null_origin_start_end_range_tokens(&sqlx_types_chrono_naive_date_time_as_non_null_timestamp_origin_upper_camel_case))),
                        PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => pg_crud_macros_common::DeriveOrImpl::Impl(macros_helpers::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream::from(generate_impl_ser_for_identifier_standard_non_null_origin_start_end_range_tokens(&sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_non_null_timestamptz_origin_upper_camel_case))),
                    }
                },
                match &pg_type {
                    PgType::SqlxTypesIpnetworkIpNetworkAsInet => {
                        pg_crud_macros_common::DeriveOrImpl::Impl(
                            macros_helpers::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream::from(
                                quote::quote! {
                                    #[allow(unused_qualifications)]
                                    #[allow(clippy::absolute_paths)]
                                    #allow_clippy_arbitrary_src_item_ordering
                                    const _: () = {
                                        extern crate serde as _serde;
                                        #[automatically_derived]
                                        impl<'de> _serde::Deserialize<'de> for #identifier_standard_non_null_origin_upper_camel_case {
                                            fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>
                                            where
                                                __D: _serde::Deserializer<'de>,
                                            {
                                                let value = <String as _serde::Deserialize>::deserialize(__deserializer)?;
                                                <sqlx::types::ipnetwork::IpNetwork as std::str::FromStr>::from_str(&value)
                                                    .map(Self)
                                                    .map_err(_serde::de::Error::custom)
                                            }
                                        }
                                    };
                                },
                            ),
                        )
                    },
                    PgType::I16AsInt2
                    | PgType::I32AsInt4
                    | PgType::I64AsInt8
                    | PgType::F32AsFloat4
                    | PgType::F64AsFloat8
                    | PgType::I16AsSmallSerialInitializationByPg
                    | PgType::I32AsSerialInitializationByPg
                    | PgType::I64AsBigSerialInitializationByPg
                    | PgType::SqlxPgTypesPgMoneyAsMoney
                    | PgType::BoolAsBool
                    | PgType::StringAsText
                    | PgType::StdVecVecU8AsBytea
                    | PgType::SqlxTypesChronoNaiveTimeAsTime
                    | PgType::SqlxTypesTimeTimeAsTime
                    | PgType::SqlxPgTypesPgIntervalAsInterval
                    | PgType::SqlxTypesChronoNaiveDateAsDate
                    | PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp
                    | PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz
                    | PgType::SqlxTypesUuidUuidAsUuidV4InitializationByPg
                    | PgType::SqlxTypesUuidUuidAsUuidInitializationByClient
                    | PgType::SqlxTypesMacAddressMacAddressAsMacAddr
                    | PgType::SqlxPgTypesPgRangeI32AsInt4Range
                    | PgType::SqlxPgTypesPgRangeI64AsInt8Range
                    | PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange
                    | PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange
                    | PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => {
                        pg_crud_macros_common::DeriveOrImpl::Derive
                    },
                }
            )
        } else {
            (pg_crud_macros_common::DeriveOrImpl::Derive, pg_crud_macros_common::DeriveOrImpl::Derive)
        };
        let v_identifier_inner_type_token_stream = quote::quote! {#v_snake_case: #identifier_inner_type_token_stream};
        let identifier_standard_non_null_read_upper_camel_case = naming::parameter::SelfReadUpperCamelCase::from_tokens(&identifier_standard_non_null_upper_camel_case);
        let identifier_standard_non_null_origin_try_new_error_upper_camel_case = naming::parameter::SelfOriginTryNewErrorUpperCamelCase::from_display(&identifier_standard_non_null_upper_camel_case);
        let identifier_standard_non_null_origin_try_new_for_de_error_upper_camel_case = naming::parameter::SelfOriginTryNewForDeErrorUpperCamelCase::from_display(&identifier_standard_non_null_upper_camel_case);
        let int_range_type_to_range_inner_type_token_stream = |int_range_type: &IntRangeType| -> proc_macro2::TokenStream {
            match &int_range_type {
                IntRangeType::SqlxPgTypesPgRangeI32AsInt4Range => quote::quote! {#i32_token_stream},
                IntRangeType::SqlxPgTypesPgRangeI64AsInt8Range => quote::quote! {#i64_token_stream},
            }
        };
        let generate_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_from_naive_utc_and_offset_token_stream = |ts: &dyn quote::ToTokens| {
            quote::quote! {sqlx::types::chrono::DateTime::<sqlx::types::chrono::Utc>::from_naive_utc_and_offset(
                #ts,
                sqlx::types::chrono::Utc
            )}
        };
        let generate_sqlx_types_chrono_naive_date_time_new_token_stream = |ts: &dyn quote::ToTokens| {
            quote::quote! {sqlx::types::chrono::NaiveDateTime::#new_snake_case(#ts)}
        };
        let generate_sqlx_types_time_time_from_hms_micro_unwrap_token_stream = |ts: &dyn quote::ToTokens| {
            quote::quote! {sqlx::types::time::Time::from_hms_micro(#ts).expect("7a1a18fa")}
        };
        let generate_pub_const_new_or_pub_try_new_token_stream = |ts: &dyn quote::ToTokens| {
            let pub_fn_new_or_try_new_token_stream = if pg_type_initialization_try_new_try_from_pg_type.is_ok() {
                &macros_helpers::generate_new_or_try_new::generate_pub_try_new_token_stream(
                    &proc_macro2::TokenStream::new(),
                    &v_identifier_inner_type_token_stream,
                    &identifier_standard_non_null_origin_try_new_error_upper_camel_case,
                    &quote::quote! {
                        match #identifier_origin_upper_camel_case::#try_new_snake_case(#v_snake_case) {
                            Ok(v_0f9f1a61) => Ok(Self(v_0f9f1a61)),
                            Err(error) => Err(error)
                        }
                    },
                )
            } else {
                &{
                    let self_identifier_origin_new_v_token_stream = quote::quote! {Self(#identifier_origin_upper_camel_case::#new_snake_case(#v_snake_case))};
                    if matches!(&pg_type_pattern, PgTypePattern::Standard)
                        && matches!(&is_nullable, pg_crud_macros_common::IsNullable::False)
                    {
                        macros_helpers::generate_new_or_try_new::generate_pub_const_new_token_stream(
                            &must_use,
                            &v_identifier_inner_type_token_stream,
                            &self_identifier_origin_new_v_token_stream
                        )
                    } else {
                        macros_helpers::generate_new_or_try_new::generate_pub_new_token_stream(
                            &must_use,
                            &v_identifier_inner_type_token_stream,
                            &self_identifier_origin_new_v_token_stream
                        )
                    }
                }
            };
            quote::quote! {
                impl #ts {
                    #pub_fn_new_or_try_new_token_stream
                }
            }
        };
        let derive_copy = match &pg_type {
            PgType::I16AsInt2 |
            PgType::I32AsInt4 |
            PgType::I64AsInt8 |
            PgType::F32AsFloat4 |
            PgType::F64AsFloat8 |
            PgType::I16AsSmallSerialInitializationByPg |
            PgType::I32AsSerialInitializationByPg |
            PgType::I64AsBigSerialInitializationByPg |
            PgType::SqlxPgTypesPgMoneyAsMoney |
            PgType::BoolAsBool |
            PgType::SqlxTypesChronoNaiveTimeAsTime |
            PgType::SqlxTypesTimeTimeAsTime |
            PgType::SqlxPgTypesPgIntervalAsInterval |
            PgType::SqlxTypesChronoNaiveDateAsDate |
            PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp |
            PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz |
            PgType::SqlxTypesUuidUuidAsUuidV4InitializationByPg |
            PgType::SqlxTypesUuidUuidAsUuidInitializationByClient |
            PgType::SqlxTypesIpnetworkIpNetworkAsInet |
            PgType::SqlxTypesMacAddressMacAddressAsMacAddr |
            PgType::SqlxPgTypesPgRangeI32AsInt4Range |
            PgType::SqlxPgTypesPgRangeI64AsInt8Range |
            PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange |
            PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange |
            PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => macros_helpers::derive_token_stream_builder::DCopy::True,
            PgType::StringAsText |
            PgType::StdVecVecU8AsBytea => macros_helpers::derive_token_stream_builder::DCopy::False,
        };
        let sqlx_types_chrono_naive_time_min_fn_token_stream = quote::quote! {sqlx_types_chrono_naive_time_min};
        let sqlx_types_chrono_naive_time_ten_fn_token_stream = quote::quote! {sqlx_types_chrono_naive_time_ten};
        let sqlx_types_chrono_naive_time_twenty_fn_token_stream = quote::quote! {sqlx_types_chrono_naive_time_twenty};
        let sqlx_types_chrono_naive_time_max_fn_token_stream = quote::quote! {sqlx_types_chrono_naive_time_max};
        let sqlx_types_chrono_naive_date_min_fn_token_stream = quote::quote! {sqlx_types_chrono_naive_date_min};
        let sqlx_types_chrono_naive_date_negative_less_typical_fn_token_stream = quote::quote! {sqlx_types_chrono_naive_date_negative_less_typical};
        let sqlx_types_chrono_naive_date_negative_more_typical_fn_token_stream = quote::quote! {sqlx_types_chrono_naive_date_negative_more_typical};
        let sqlx_types_chrono_naive_date_near_zero_fn_token_stream = quote::quote! {sqlx_types_chrono_naive_date_near_zero};
        let sqlx_types_chrono_naive_date_positive_less_typical_fn_token_stream = quote::quote! {sqlx_types_chrono_naive_date_positive_less_typical};
        let sqlx_types_chrono_naive_date_positive_more_typical_fn_token_stream = quote::quote! {sqlx_types_chrono_naive_date_positive_more_typical};
        let sqlx_types_chrono_naive_date_max_fn_token_stream = quote::quote! {sqlx_types_chrono_naive_date_max};
        let sqlx_types_chrono_naive_date_max_pred_opt_expect_fn_token_stream = quote::quote! {sqlx_types_chrono_naive_date_max_pred_opt_expect};
        let identifier_token_stream = {
            let identifier_token_stream = macros_helpers::derive_token_stream_builder::DTokenStreamBuilder::new()
                .make_pub()
                .d_debug()
                .d_clone()
                .d_copy()
                .d_partial_eq()
                .build_struct(
                    &proc_macro2::TokenStream::new(),
                    &identifier,
                    &proc_macro2::TokenStream::new(),
                    &quote::quote! {;},
                );
            let maybe_impl_identifier_token_stream = if matches!(&pg_type_pattern, PgTypePattern::Standard) &&
                matches!(&is_nullable, pg_crud_macros_common::IsNullable::False)
            {
                enum IsConst {
                    False,
                    True,
                }
                let generate_inner_type_token_stream = |
                    is_const: IsConst,
                    name_token_stream: &dyn quote::ToTokens,
                    ts: &dyn quote::ToTokens
                |{
                    let maybe_const_token_stream = match is_const {
                        IsConst::False => proc_macro2::TokenStream::new(),
                        IsConst::True => quote::quote! {const},
                    };
                    quote::quote! {
                        #maybe_const_token_stream fn #name_token_stream() -> #identifier_inner_type_token_stream {
                            #ts
                        }
                    }
                };
                let maybe_min_inner_type_token_stream = {
                    let generate_min_inner_type_token_stream = |is_const: IsConst, ts: &dyn quote::ToTokens| generate_inner_type_token_stream(is_const, &quote::quote! {min_inner_type}, ts);
                    match &pg_type {
                        PgType::SqlxTypesChronoNaiveTimeAsTime => Some(
                            generate_min_inner_type_token_stream(
                                IsConst::True,
                                &quote::quote! {
                                    sqlx::types::chrono::NaiveTime::from_hms_micro_opt(0, 0, 0, 0).expect("000ddcc2")
                                }
                            )
                        ),
                        PgType::SqlxTypesTimeTimeAsTime => Some(
                            generate_min_inner_type_token_stream(
                                IsConst::False,
                                &quote::quote! {
                                    sqlx::types::time::Time::from_hms_micro(0, 0, 0, 0).expect("f065e2b1")
                                }
                            )
                        ),
                        PgType::I16AsInt2 |
                        PgType::I32AsInt4 |
                        PgType::I64AsInt8 |
                        PgType::F32AsFloat4 |
                        PgType::F64AsFloat8 |
                        PgType::I16AsSmallSerialInitializationByPg |
                        PgType::I32AsSerialInitializationByPg |
                        PgType::I64AsBigSerialInitializationByPg |
                        PgType::SqlxPgTypesPgMoneyAsMoney |
                        PgType::BoolAsBool |
                        PgType::StringAsText |
                        PgType::StdVecVecU8AsBytea |
                        PgType::SqlxPgTypesPgIntervalAsInterval |
                        PgType::SqlxTypesChronoNaiveDateAsDate |
                        PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp |
                        PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz |
                        PgType::SqlxTypesUuidUuidAsUuidV4InitializationByPg |
                        PgType::SqlxTypesUuidUuidAsUuidInitializationByClient |
                        PgType::SqlxTypesIpnetworkIpNetworkAsInet |
                        PgType::SqlxTypesMacAddressMacAddressAsMacAddr |
                        PgType::SqlxPgTypesPgRangeI32AsInt4Range |
                        PgType::SqlxPgTypesPgRangeI64AsInt8Range |
                        PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange |
                        PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange |
                        PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => None,
                    }
                };
                let maybe_slightly_more_than_min_inner_type_token_stream = {
                    let generate_slightly_more_than_min_inner_type_token_stream = |is_const: IsConst, ts: &dyn quote::ToTokens| generate_inner_type_token_stream(is_const, &quote::quote! {slightly_more_than_min_inner_type}, ts);
                    match &pg_type {
                        PgType::SqlxTypesChronoNaiveTimeAsTime => Some(
                            generate_slightly_more_than_min_inner_type_token_stream(
                                IsConst::True,
                                &quote::quote! {
                                    sqlx::types::chrono::NaiveTime::from_hms_micro_opt(0, 0, 0, 1).expect("9545a47c")
                                }
                            )
                        ),
                        PgType::SqlxTypesTimeTimeAsTime => Some(
                            generate_slightly_more_than_min_inner_type_token_stream(
                                IsConst::False,
                                &quote::quote! {
                                    sqlx::types::time::Time::from_hms_micro(0, 0, 0, 1).expect("03f9561a")
                                }
                            )
                        ),
                        PgType::I16AsInt2 |
                        PgType::I32AsInt4 |
                        PgType::I64AsInt8 |
                        PgType::F32AsFloat4 |
                        PgType::F64AsFloat8 |
                        PgType::I16AsSmallSerialInitializationByPg |
                        PgType::I32AsSerialInitializationByPg |
                        PgType::I64AsBigSerialInitializationByPg |
                        PgType::SqlxPgTypesPgMoneyAsMoney |
                        PgType::BoolAsBool |
                        PgType::StringAsText |
                        PgType::StdVecVecU8AsBytea |
                        PgType::SqlxPgTypesPgIntervalAsInterval |
                        PgType::SqlxTypesChronoNaiveDateAsDate |
                        PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp |
                        PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz |
                        PgType::SqlxTypesUuidUuidAsUuidV4InitializationByPg |
                        PgType::SqlxTypesUuidUuidAsUuidInitializationByClient |
                        PgType::SqlxTypesIpnetworkIpNetworkAsInet |
                        PgType::SqlxTypesMacAddressMacAddressAsMacAddr |
                        PgType::SqlxPgTypesPgRangeI32AsInt4Range |
                        PgType::SqlxPgTypesPgRangeI64AsInt8Range |
                        PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange |
                        PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange |
                        PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => None,
                    }
                };
                let maybe_middle_inner_type_token_stream = {
                    let generate_middle_inner_type_token_stream = |is_const: IsConst, ts: &dyn quote::ToTokens| generate_inner_type_token_stream(is_const, &quote::quote! {middle_inner_type}, ts);
                    match &pg_type {
                        PgType::SqlxTypesChronoNaiveTimeAsTime => Some(
                            generate_middle_inner_type_token_stream(
                                IsConst::True,
                                &quote::quote! {
                                    sqlx::types::chrono::NaiveTime::from_hms_micro_opt(0, 0, 0, 0).expect("0dafc3fc")
                                }
                            )
                        ),
                        PgType::SqlxTypesTimeTimeAsTime => Some(
                            generate_middle_inner_type_token_stream(
                                IsConst::False,
                                &quote::quote! {
                                    sqlx::types::time::Time::from_hms_micro(0, 0, 0, 0).expect("d2ec329f")
                                }
                            )
                        ),
                        PgType::SqlxTypesChronoNaiveDateAsDate => Some(
                            generate_middle_inner_type_token_stream(
                                IsConst::True,
                                &quote::quote! {
                                    sqlx::types::chrono::NaiveDate::from_ymd_opt(0, 1, 1).expect("a2f306ea")
                                }
                            )
                        ),
                        PgType::I16AsInt2 |
                        PgType::I32AsInt4 |
                        PgType::I64AsInt8 |
                        PgType::F32AsFloat4 |
                        PgType::F64AsFloat8 |
                        PgType::I16AsSmallSerialInitializationByPg |
                        PgType::I32AsSerialInitializationByPg |
                        PgType::I64AsBigSerialInitializationByPg |
                        PgType::SqlxPgTypesPgMoneyAsMoney |
                        PgType::BoolAsBool |
                        PgType::StringAsText |
                        PgType::StdVecVecU8AsBytea |
                        PgType::SqlxPgTypesPgIntervalAsInterval |
                        PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp |
                        PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz |
                        PgType::SqlxTypesUuidUuidAsUuidV4InitializationByPg |
                        PgType::SqlxTypesUuidUuidAsUuidInitializationByClient |
                        PgType::SqlxTypesIpnetworkIpNetworkAsInet |
                        PgType::SqlxTypesMacAddressMacAddressAsMacAddr |
                        PgType::SqlxPgTypesPgRangeI32AsInt4Range |
                        PgType::SqlxPgTypesPgRangeI64AsInt8Range |
                        PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange |
                        PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange |
                        PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => None,
                    }
                };
                let maybe_slightly_more_than_middle_inner_type_token_stream = {
                    let generate_slightly_more_than_middle_inner_type_token_stream = |is_const: IsConst, ts: &dyn quote::ToTokens| generate_inner_type_token_stream(is_const, &quote::quote! {slightly_more_than_middle_inner_type}, ts);
                    match &pg_type {
                        PgType::SqlxTypesChronoNaiveTimeAsTime => Some(
                            generate_slightly_more_than_middle_inner_type_token_stream(
                                IsConst::True,
                                &quote::quote! {
                                    sqlx::types::chrono::NaiveTime::from_hms_micro_opt(0, 0, 0, 1).expect("235276a7")
                                }
                            )
                        ),
                        PgType::SqlxTypesTimeTimeAsTime => Some(
                            generate_slightly_more_than_middle_inner_type_token_stream(
                                IsConst::False,
                                &quote::quote! {
                                    sqlx::types::time::Time::from_hms_micro(0, 0, 0, 1).expect("6a3dbcaa")
                                }
                            )
                        ),
                        PgType::I16AsInt2 |
                        PgType::I32AsInt4 |
                        PgType::I64AsInt8 |
                        PgType::F32AsFloat4 |
                        PgType::F64AsFloat8 |
                        PgType::I16AsSmallSerialInitializationByPg |
                        PgType::I32AsSerialInitializationByPg |
                        PgType::I64AsBigSerialInitializationByPg |
                        PgType::SqlxPgTypesPgMoneyAsMoney |
                        PgType::BoolAsBool |
                        PgType::StringAsText |
                        PgType::StdVecVecU8AsBytea |
                        PgType::SqlxPgTypesPgIntervalAsInterval |
                        PgType::SqlxTypesChronoNaiveDateAsDate |
                        PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp |
                        PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz |
                        PgType::SqlxTypesUuidUuidAsUuidV4InitializationByPg |
                        PgType::SqlxTypesUuidUuidAsUuidInitializationByClient |
                        PgType::SqlxTypesIpnetworkIpNetworkAsInet |
                        PgType::SqlxTypesMacAddressMacAddressAsMacAddr |
                        PgType::SqlxPgTypesPgRangeI32AsInt4Range |
                        PgType::SqlxPgTypesPgRangeI64AsInt8Range |
                        PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange |
                        PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange |
                        PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => None,
                    }
                };
                let maybe_max_inner_type_token_stream = {
                    let generate_max_inner_type_token_stream = |is_const: IsConst, ts: &dyn quote::ToTokens| generate_inner_type_token_stream(is_const, &quote::quote! {max_inner_type}, ts);
                    match &pg_type {
                        PgType::SqlxTypesChronoNaiveTimeAsTime => Some(
                            generate_max_inner_type_token_stream(
                                IsConst::True,
                                &quote::quote! {
                                    sqlx::types::chrono::NaiveTime::from_hms_micro_opt(23, 59, 59, 999_999).expect("b217e3bf")
                                }
                            )
                        ),
                        PgType::SqlxTypesChronoNaiveDateAsDate => Some(
                            generate_max_inner_type_token_stream(
                                IsConst::True,
                                &quote::quote! {
                                    sqlx::types::chrono::NaiveDate::MAX
                                }
                            )
                        ),
                        PgType::I16AsInt2 |
                        PgType::I32AsInt4 |
                        PgType::I64AsInt8 |
                        PgType::F32AsFloat4 |
                        PgType::F64AsFloat8 |
                        PgType::I16AsSmallSerialInitializationByPg |
                        PgType::I32AsSerialInitializationByPg |
                        PgType::I64AsBigSerialInitializationByPg |
                        PgType::SqlxPgTypesPgMoneyAsMoney |
                        PgType::BoolAsBool |
                        PgType::StringAsText |
                        PgType::StdVecVecU8AsBytea |
                        PgType::SqlxTypesTimeTimeAsTime |
                        PgType::SqlxPgTypesPgIntervalAsInterval |
                        PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp |
                        PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz |
                        PgType::SqlxTypesUuidUuidAsUuidV4InitializationByPg |
                        PgType::SqlxTypesUuidUuidAsUuidInitializationByClient |
                        PgType::SqlxTypesIpnetworkIpNetworkAsInet |
                        PgType::SqlxTypesMacAddressMacAddressAsMacAddr |
                        PgType::SqlxPgTypesPgRangeI32AsInt4Range |
                        PgType::SqlxPgTypesPgRangeI64AsInt8Range |
                        PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange |
                        PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange |
                        PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => None,
                    }
                };
                let maybe_slightly_less_than_max_inner_type_token_stream = {
                    let generate_slightly_less_than_max_inner_type_token_stream = |is_const: IsConst, ts: &dyn quote::ToTokens| generate_inner_type_token_stream(is_const, &quote::quote! {slightly_less_than_max_inner_type}, ts);
                    match &pg_type {
                        PgType::SqlxTypesChronoNaiveTimeAsTime => Some(
                            generate_slightly_less_than_max_inner_type_token_stream(
                                IsConst::True,
                                &quote::quote! {
                                    sqlx::types::chrono::NaiveTime::from_hms_micro_opt(23, 59, 59, 999_998).expect("5d6cf475")
                                }
                            )
                        ),
                        PgType::I16AsInt2 |
                        PgType::I32AsInt4 |
                        PgType::I64AsInt8 |
                        PgType::F32AsFloat4 |
                        PgType::F64AsFloat8 |
                        PgType::I16AsSmallSerialInitializationByPg |
                        PgType::I32AsSerialInitializationByPg |
                        PgType::I64AsBigSerialInitializationByPg |
                        PgType::SqlxPgTypesPgMoneyAsMoney |
                        PgType::BoolAsBool |
                        PgType::StringAsText |
                        PgType::StdVecVecU8AsBytea |
                        PgType::SqlxTypesTimeTimeAsTime |
                        PgType::SqlxPgTypesPgIntervalAsInterval |
                        PgType::SqlxTypesChronoNaiveDateAsDate |
                        PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp |
                        PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz |
                        PgType::SqlxTypesUuidUuidAsUuidV4InitializationByPg |
                        PgType::SqlxTypesUuidUuidAsUuidInitializationByClient |
                        PgType::SqlxTypesIpnetworkIpNetworkAsInet |
                        PgType::SqlxTypesMacAddressMacAddressAsMacAddr |
                        PgType::SqlxPgTypesPgRangeI32AsInt4Range |
                        PgType::SqlxPgTypesPgRangeI64AsInt8Range |
                        PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange |
                        PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange |
                        PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => None,
                    }
                };
                let maybe_read_inner_inits_token_stream = {
                    let generate_fn_token_stream: &dyn Fn(
                        &dyn quote::ToTokens,
                        &dyn quote::ToTokens,
                    ) -> proc_macro2::TokenStream = &|name_token_stream, ts_parameter| quote::quote! {
                        const fn #name_token_stream() -> #identifier_inner_type_token_stream {
                            #ts_parameter
                        }
                    };
                    match &pg_type {
                        PgType::I16AsInt2 |
                        PgType::I32AsInt4 |
                        PgType::I64AsInt8 |
                        PgType::F32AsFloat4 |
                        PgType::F64AsFloat8 |
                        PgType::I16AsSmallSerialInitializationByPg |
                        PgType::I32AsSerialInitializationByPg |
                        PgType::I64AsBigSerialInitializationByPg |
                        PgType::SqlxPgTypesPgMoneyAsMoney |
                        PgType::BoolAsBool |
                        PgType::StringAsText |
                        PgType::StdVecVecU8AsBytea |
                        PgType::SqlxTypesTimeTimeAsTime |
                        PgType::SqlxPgTypesPgIntervalAsInterval |
                        PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp |
                        PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz |
                        PgType::SqlxTypesUuidUuidAsUuidV4InitializationByPg |
                        PgType::SqlxTypesUuidUuidAsUuidInitializationByClient |
                        PgType::SqlxTypesIpnetworkIpNetworkAsInet |
                        PgType::SqlxTypesMacAddressMacAddressAsMacAddr |
                        PgType::SqlxPgTypesPgRangeI32AsInt4Range |
                        PgType::SqlxPgTypesPgRangeI64AsInt8Range |
                        PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange |
                        PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange |
                        PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => None,
                        PgType::SqlxTypesChronoNaiveTimeAsTime => Some({
                            let generate_fn_identifier_inner_type_token_stream = |name_token_stream: &dyn quote::ToTokens, parameters_token_stream: &dyn quote::ToTokens| quote::quote! {
                                const fn #name_token_stream() -> #identifier_inner_type_token_stream {
                                    #identifier_inner_type_token_stream::from_hms_micro_opt(#parameters_token_stream).expect("149e01cc")
                                }
                            };
                            let ser_de_array_token_stream = [
                                generate_fn_identifier_inner_type_token_stream(&sqlx_types_chrono_naive_time_min_fn_token_stream, &quote::quote! {0,0,0,0}),
                                generate_fn_identifier_inner_type_token_stream(&sqlx_types_chrono_naive_time_ten_fn_token_stream, &quote::quote! {10,10,10,10}),
                                generate_fn_identifier_inner_type_token_stream(&sqlx_types_chrono_naive_time_twenty_fn_token_stream, &quote::quote! {20,20,20,20}),
                                generate_fn_identifier_inner_type_token_stream(&sqlx_types_chrono_naive_time_max_fn_token_stream, &quote::quote! {23,59,59,999_999}),
                            ];
                            quote::quote! {#(#ser_de_array_token_stream)*}
                        }),
                        PgType::SqlxTypesChronoNaiveDateAsDate => Some({
                            let ser_de_array_token_stream = {
                                let generate_fn_identifier_inner_type_token_stream: &dyn Fn(
                                    &dyn quote::ToTokens,
                                    &dyn quote::ToTokens,
                                ) -> proc_macro2::TokenStream = &|name_token_stream, ts_parameter| {
                                    generate_fn_token_stream(
                                        name_token_stream,
                                        &quote::quote! {#identifier_inner_type_token_stream::#ts_parameter},
                                    )
                                };
                                [
                                    generate_fn_identifier_inner_type_token_stream(
                                        &sqlx_types_chrono_naive_date_max_fn_token_stream,
                                        &quote::quote! { MAX }
                                    ),
                                    generate_fn_token_stream(
                                        &sqlx_types_chrono_naive_date_max_pred_opt_expect_fn_token_stream,
                                        &quote::quote! {Self::#sqlx_types_chrono_naive_date_max_fn_token_stream().pred_opt().expect("b7e16bf1")}
                                    ),
                                    generate_fn_identifier_inner_type_token_stream(
                                        &sqlx_types_chrono_naive_date_min_fn_token_stream,
                                        &quote::quote! {
                                            from_ymd_opt(-4713, 12, 31)
                                                .expect("d074927c")
                                        },
                                    ),
                                    generate_fn_identifier_inner_type_token_stream(
                                        &sqlx_types_chrono_naive_date_negative_less_typical_fn_token_stream,
                                        &quote::quote! {
                                            from_ymd_opt(-2000, 1, 1)
                                                .expect("c4e31c47")
                                        },
                                    ),
                                    generate_fn_identifier_inner_type_token_stream(
                                        &sqlx_types_chrono_naive_date_negative_more_typical_fn_token_stream,
                                        &quote::quote! {
                                            from_ymd_opt(-1000, 1, 1)
                                                .expect("22400727")
                                        },
                                    ),
                                    generate_fn_identifier_inner_type_token_stream(
                                        &sqlx_types_chrono_naive_date_near_zero_fn_token_stream,
                                        &quote::quote! {
                                            from_ymd_opt(0, 1, 1)
                                                .expect("05c3dc8c")
                                        },
                                    ),
                                    generate_fn_identifier_inner_type_token_stream(
                                        &sqlx_types_chrono_naive_date_positive_less_typical_fn_token_stream,
                                        &quote::quote! {
                                            from_ymd_opt(1000, 1, 1)
                                                .expect("56140676")
                                        },
                                    ),
                                    generate_fn_identifier_inner_type_token_stream(
                                        &sqlx_types_chrono_naive_date_positive_more_typical_fn_token_stream,
                                        &quote::quote! {
                                            from_ymd_opt(2000, 1, 1)
                                                .expect("739e0bc9")
                                        },
                                    ),
                                ]
                            };
                            quote::quote! {#(#ser_de_array_token_stream)*}
                        }),
                    }
                };
                if maybe_min_inner_type_token_stream.is_some() ||
                    maybe_slightly_more_than_min_inner_type_token_stream.is_some() ||
                    maybe_middle_inner_type_token_stream.is_some() ||
                    maybe_slightly_more_than_middle_inner_type_token_stream.is_some() ||
                    maybe_max_inner_type_token_stream.is_some() ||
                    maybe_slightly_less_than_max_inner_type_token_stream.is_some() ||
                    maybe_read_inner_inits_token_stream.is_some()
                {
                    quote::quote! {
                        #allow_clippy_arbitrary_src_item_ordering
                        impl #identifier {
                            #maybe_min_inner_type_token_stream
                            #maybe_slightly_more_than_min_inner_type_token_stream
                            #maybe_middle_inner_type_token_stream
                            #maybe_slightly_more_than_middle_inner_type_token_stream
                            #maybe_max_inner_type_token_stream
                            #maybe_slightly_less_than_max_inner_type_token_stream
                            #maybe_read_inner_inits_token_stream
                        }
                    }
                }
                else {
                    proc_macro2::TokenStream::new()
                }
            }
            else {
                proc_macro2::TokenStream::new()
            };
            quote::quote! {
                #identifier_token_stream
                #maybe_impl_identifier_token_stream
            }
        };
        let sqlx_types_chrono_naive_date_as_date_standard_non_null_orig_token_stream = naming::parameter::SelfOriginUpperCamelCase::from_tokens(&generate_identifier_standard_non_null_token_stream(&PgType::SqlxTypesChronoNaiveDateAsDate));
        let identifier_update_upper_camel_case = naming::parameter::SelfUpdateUpperCamelCase::from_tokens(&identifier);
        let sqlx_encode_self_dot_zero_token_stream = quote::quote! {#self_snake_case.0};
        let identifier_origin_token_stream = {
            let identifier_origin_wire_token_stream = if matches!(&is_standard_non_null, pg_crud_macros_common::IsStandardNonNull::True) {
                match crate::serde::wire_kind(pg_type_dsc) {
                    WireKind::TimeChrono => quote::quote! {
                        #[derive(serde::Deserialize)]
                        struct #identifier_origin_wire_upper_camel_case {
                            hour: u32,
                            min: u32,
                            sec: u32,
                            micro: u32,
                        }
                    },
                    WireKind::TimeTime => quote::quote! {
                        #[derive(serde::Deserialize)]
                        struct #identifier_origin_wire_upper_camel_case {
                            hour: u8,
                            minute: u8,
                            second: u8,
                            microsecond: u32,
                        }
                    },
                    WireKind::Interval => quote::quote! {
                        #[derive(serde::Deserialize)]
                        struct #identifier_origin_wire_upper_camel_case {
                            months: i32,
                            days: i32,
                            microseconds: i64,
                        }
                    },
                    WireKind::Timestamp => quote::quote! {
                        #[derive(serde::Deserialize)]
                        struct #identifier_origin_wire_upper_camel_case {
                            date: #sqlx_types_chrono_naive_date_as_date_standard_non_null_orig_token_stream,
                            time: SqlxTypesChronoNaiveTimeAsNonNullTimeOrigin,
                        }
                    },
                    WireKind::TimestampTz => quote::quote! {
                        #[derive(serde::Deserialize)]
                        struct #identifier_origin_wire_upper_camel_case {
                            date_naive: #sqlx_types_chrono_naive_date_as_date_standard_non_null_orig_token_stream,
                            time: SqlxTypesChronoNaiveTimeAsNonNullTimeOrigin,
                        }
                    },
                    WireKind::RangeInt32 => quote::quote! {
                        #[derive(serde::Deserialize)]
                        struct #identifier_origin_wire_upper_camel_case { start: std::ops::Bound<i32>, end: std::ops::Bound<i32> }
                    },
                    WireKind::RangeInt64 => quote::quote! {
                        #[derive(serde::Deserialize)]
                        struct #identifier_origin_wire_upper_camel_case { start: std::ops::Bound<i64>, end: std::ops::Bound<i64> }
                    },
                    WireKind::RangeDate => quote::quote! {
                        #[derive(serde::Deserialize)]
                        struct #identifier_origin_wire_upper_camel_case { start: std::ops::Bound<#sqlx_types_chrono_naive_date_as_date_standard_non_null_orig_token_stream>, end: std::ops::Bound<#sqlx_types_chrono_naive_date_as_date_standard_non_null_orig_token_stream> }
                    },
                    WireKind::RangeTimestamp => quote::quote! {
                        #[derive(serde::Deserialize)]
                        struct #identifier_origin_wire_upper_camel_case { start: std::ops::Bound<SqlxTypesChronoNaiveDateTimeAsNonNullTimestampOrigin>, end: std::ops::Bound<SqlxTypesChronoNaiveDateTimeAsNonNullTimestampOrigin> }
                    },
                    WireKind::RangeTimestampTz => quote::quote! {
                        #[derive(serde::Deserialize)]
                        struct #identifier_origin_wire_upper_camel_case { start: std::ops::Bound<SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNonNullTimestampTzOrigin>, end: std::ops::Bound<SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNonNullTimestampTzOrigin> }
                    },
                    _ => proc_macro2::TokenStream::new(),
                }
            } else {
                proc_macro2::TokenStream::new()
            };
            let identifier_origin_token_stream = macros_helpers::derive_token_stream_builder::DTokenStreamBuilder::new()
                .make_pub()
                .d_debug()
                .d_clone()
                .d_copy_if(derive_copy)
                .d_partial_eq()
                .d_eq_if(match &is_non_null_standard_can_be_primary_key {
                    IsNonNullStandardCanBePrimaryKey::False => macros_helpers::derive_token_stream_builder::DEq::False,
                    IsNonNullStandardCanBePrimaryKey::True => macros_helpers::derive_token_stream_builder::DEq::True,
                })
                .d_std_hash_hash_if(match &is_non_null_standard_can_be_primary_key {
                    IsNonNullStandardCanBePrimaryKey::False => {
                        macros_helpers::derive_token_stream_builder::DStdHashHash::False
                    }
                    IsNonNullStandardCanBePrimaryKey::True => {
                        macros_helpers::derive_token_stream_builder::DStdHashHash::True
                    }
                })
                .d_partial_ord_if(d_partial_ord)
                .d_ord_if(match &is_non_null_standard_can_be_primary_key {
                    IsNonNullStandardCanBePrimaryKey::False => macros_helpers::derive_token_stream_builder::DOrd::False,
                    IsNonNullStandardCanBePrimaryKey::True => macros_helpers::derive_token_stream_builder::DOrd::True,
                })
                .d_serde_serialize_if(match &ser_derive_or_impl {
                    pg_crud_macros_common::DeriveOrImpl::Derive => macros_helpers::derive_token_stream_builder::DSerdeSerialize::True,
                    pg_crud_macros_common::DeriveOrImpl::Impl(_) => macros_helpers::derive_token_stream_builder::DSerdeSerialize::False,
                })
                .d_serde_deserialize_if(match &de_derive_or_impl {
                    pg_crud_macros_common::DeriveOrImpl::Derive => macros_helpers::derive_token_stream_builder::DSerdeDeserialize::True,
                    pg_crud_macros_common::DeriveOrImpl::Impl(_) => macros_helpers::derive_token_stream_builder::DSerdeDeserialize::False,
                })
                .build_struct(
                    &{
                        if matches!(&is_standard_non_null, pg_crud_macros_common::IsStandardNonNull::True) {
                            let generate_serde_from_token_stream = |ts: &dyn quote::ToTokens|quote::quote! {#[serde(from = #ts)]};
                            let generate_serde_try_from_token_stream = |ts: &dyn quote::ToTokens|quote::quote! {#[serde(try_from = #ts)]};
                            match &pg_type {
                            PgType::I16AsInt2 |
                            PgType::I32AsInt4 |
                            PgType::I64AsInt8 |
                            PgType::F32AsFloat4 |
                            PgType::I16AsSmallSerialInitializationByPg |
                            PgType::I32AsSerialInitializationByPg |
                            PgType::I64AsBigSerialInitializationByPg |
                            PgType::BoolAsBool |
                            PgType::StdVecVecU8AsBytea |
                            PgType::SqlxTypesIpnetworkIpNetworkAsInet => proc_macro2::TokenStream::new(),
                            PgType::F64AsFloat8 => generate_serde_try_from_token_stream(&quote::quote! {"f64"}),
                            PgType::SqlxPgTypesPgMoneyAsMoney => generate_serde_from_token_stream(&quote::quote! {"i64"}),
                            PgType::SqlxTypesChronoNaiveTimeAsTime |
                            PgType::SqlxTypesTimeTimeAsTime |
                            PgType::SqlxPgTypesPgRangeI32AsInt4Range |
                            PgType::SqlxPgTypesPgRangeI64AsInt8Range => generate_serde_try_from_token_stream(&generate_quotes::dq_token_stream(&identifier_origin_wire_upper_camel_case)),
                            PgType::SqlxPgTypesPgIntervalAsInterval |
                            PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp |
                            PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz |
                            PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange |
                            PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange |
                            PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => generate_serde_from_token_stream(&generate_quotes::dq_token_stream(&identifier_origin_wire_upper_camel_case)),
                            PgType::SqlxTypesChronoNaiveDateAsDate => generate_serde_try_from_token_stream(&quote::quote! {"sqlx::types::chrono::NaiveDate"}),
                            PgType::StringAsText |
                            PgType::SqlxTypesUuidUuidAsUuidV4InitializationByPg |
                            PgType::SqlxTypesUuidUuidAsUuidInitializationByClient => quote::quote! {#[serde(try_from = "String")]},
                            PgType::SqlxTypesMacAddressMacAddressAsMacAddr => quote::quote! {#[serde(from = "[u8; 6]")]},
                            }
                        }
                        else {
                            proc_macro2::TokenStream::new()
                        }
                    },
                    &identifier_origin_upper_camel_case,
                    &proc_macro2::TokenStream::new(),
                    &quote::quote! {(#field_type_handle);},
                );
            let generate_location_var_token_stream = |name_token_stream: &dyn quote::ToTokens, ts: &dyn quote::ToTokens|quote::quote! {
                #name_token_stream {
                    location: location_lib::location::Location,
                    #ts
                }
            };
            let generate_int_range_type_error_variants_token_stream = |int_range_type: &IntRangeType| {
                let range_inner_type_token_stream = int_range_type_to_range_inner_type_token_stream(int_range_type);
                let (
                    included_start_greater_than_included_end_token_stream,
                    included_start_greater_than_excluded_end_token_stream,
                    excluded_start_greater_than_included_end_token_stream,
                    excluded_start_greater_than_excluded_end_token_stream
                ) = {
                    let generate_token_stream = |ts: &dyn quote::ToTokens|generate_location_var_token_stream(
                        &ts,
                        &quote::quote! {
                            #[eo_to_err_string_serde]
                            #start_snake_case: #range_inner_type_token_stream,
                            #[eo_to_err_string_serde]
                            #end_snake_case: #range_inner_type_token_stream,
                        }
                    );
                    (
                        generate_token_stream(&included_start_greater_than_included_end_upper_camel_case),
                        generate_token_stream(&included_start_greater_than_excluded_end_upper_camel_case),
                        generate_token_stream(&excluded_start_greater_than_included_end_upper_camel_case),
                        generate_token_stream(&excluded_start_greater_than_excluded_end_upper_camel_case)
                    )
                };
                let included_end_cannot_be_max_upper_camel_case_token_stream = generate_location_var_token_stream(
                    &included_end_cannot_be_max_upper_camel_case,
                    &quote::quote! {
                        #[eo_to_err_string_serde]
                        #end_snake_case: #range_inner_type_token_stream,
                    }
                );
                quote::quote! {
                    #included_start_greater_than_included_end_token_stream,
                    #included_start_greater_than_excluded_end_token_stream,
                    #excluded_start_greater_than_included_end_token_stream,
                    #excluded_start_greater_than_excluded_end_token_stream,
                    #included_end_cannot_be_max_upper_camel_case_token_stream,
                }
            };
            let nanosecond_precision_is_not_supported_variant_try_new_token_stream = generate_location_var_token_stream(
                &nanosecond_precision_is_not_supported_upper_camel_case,
                &quote::quote! {
                    #[eo_to_err_string_serde]
                    #v_snake_case: #string_token_stream,
                }
            );
            let sqlx_types_chrono_naive_date_as_date_try_new_error_variants_token_stream = generate_location_var_token_stream(
                &earlier_date_not_supported_upper_camel_case,
                &quote::quote! {
                    #[eo_to_err_string_serde]
                    value: #string_token_stream,
                    #[eo_to_err_string_serde]
                    #earliest_supported_date_snake_case: #string_token_stream,
                }
            );
            let string_as_text_try_new_error_variants_token_stream = generate_location_var_token_stream(
                &contains_null_byte_upper_camel_case,
                &quote::quote! {
                    #[eo_to_err_string_serde]
                    #v_snake_case: #identifier_inner_type_token_stream,
                }
            );
            let not_finite_upper_camel_case = quote::format_ident!("NotFinite");
            let f64_as_float8_try_new_error_variants_token_stream =
                generate_location_var_token_stream(
                    &not_finite_upper_camel_case,
                    &proc_macro2::TokenStream::new(),
                );
            let uuid_as_uuid_v4_as_string_try_new_error_variants_token_stream = generate_location_var_token_stream(
                &not_uuid_upper_camel_case,
                &quote::quote! {
                    #[eo_to_err_string_serde]
                    #v_snake_case: String,
                }
            );
            let maybe_pub_enum_identifier_standard_non_null_origin_try_new_error_token_stream = if matches!(&is_standard_non_null, pg_crud_macros_common::IsStandardNonNull::True)
                && let Ok(pg_type_initialization_try_new) = &pg_type_initialization_try_new_try_from_pg_type
            {
                let serde_error_enum_token_stream = pg_crud_macros_common::token_stream_helpers::serde_error_enum_d_token_stream_builder()
                    .build_enum(
                        &proc_macro2::TokenStream::new(),
                        &identifier_standard_non_null_origin_try_new_error_upper_camel_case,
                        &proc_macro2::TokenStream::new(),
                        &{
                            let generate_token_stream = |ts: &dyn quote::ToTokens| {
                                let (start_variant_token_stream, end_variant_token_stream) = {
                                    let generate_variant_token_stream = |start_or_end: &StartOrEnd| generate_location_var_token_stream(
                                        &generate_start_or_end_upper_camel_case(start_or_end),
                                        &quote::quote! {
                                            #[eo_location]
                                            #error_snake_case: #ts,
                                        }
                                    );
                                    (generate_variant_token_stream(&StartOrEnd::Start), generate_variant_token_stream(&StartOrEnd::End))
                                };
                                quote::quote! {
                                    #start_variant_token_stream,
                                    #end_variant_token_stream,
                                }
                            };
                            let time_var_token_stream = generate_location_var_token_stream(
                                &time_upper_camel_case,
                                &quote::quote! {
                                    #[eo_location]
                                    #error_snake_case: #sqlx_types_chrono_naive_time_as_non_null_time_origin_try_new_error_upper_camel_case,
                                }
                            );
                            let ts: &dyn quote::ToTokens = match &pg_type_initialization_try_new {
                                PgTypeInitializationTryNew::F64AsFloat8 =>
                                    &f64_as_float8_try_new_error_variants_token_stream,
                                PgTypeInitializationTryNew::StringAsText => &string_as_text_try_new_error_variants_token_stream,
                                PgTypeInitializationTryNew::SqlxTypesChronoNaiveTimeAsTime | PgTypeInitializationTryNew::SqlxTypesTimeTimeAsTime => &nanosecond_precision_is_not_supported_variant_try_new_token_stream,
                                PgTypeInitializationTryNew::SqlxTypesChronoNaiveDateAsDate => &sqlx_types_chrono_naive_date_as_date_try_new_error_variants_token_stream,
                                PgTypeInitializationTryNew::SqlxTypesChronoNaiveDateTimeAsTimestamp | PgTypeInitializationTryNew::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => &{
                                    let date_name_upper_camel_case: &dyn naming::DisplayPlusToTokens = if matches!(&pg_type_initialization_try_new, PgTypeInitializationTryNew::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz) {
                                        &date_naive_upper_camel_case
                                    } else {
                                        &date_upper_camel_case
                                    };
                                    let date_var_token_stream = generate_location_var_token_stream(
                                        date_name_upper_camel_case,
                                        &quote::quote! {
                                            #[eo_location]
                                            #error_snake_case: #sqlx_types_chrono_naive_date_as_non_null_date_origin_try_new_error_upper_camel_case,
                                        }
                                    );
                                    quote::quote! {
                                        #date_var_token_stream,
                                        #time_var_token_stream,
                                    }
                                },
                                PgTypeInitializationTryNew::SqlxPgTypesPgRangeI32AsInt4Range => &generate_int_range_type_error_variants_token_stream(&IntRangeType::SqlxPgTypesPgRangeI32AsInt4Range),
                                PgTypeInitializationTryNew::SqlxPgTypesPgRangeI64AsInt8Range => &generate_int_range_type_error_variants_token_stream(&IntRangeType::SqlxPgTypesPgRangeI64AsInt8Range),
                                PgTypeInitializationTryNew::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => &generate_token_stream(
                                    &sqlx_types_chrono_naive_date_as_non_null_date_origin_try_new_error_upper_camel_case
                                ),
                                PgTypeInitializationTryNew::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => &generate_token_stream(
                                    &sqlx_types_chrono_naive_date_time_as_non_null_timestamp_origin_try_new_error_upper_camel_case
                                ),
                                PgTypeInitializationTryNew::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => &generate_token_stream(
                                    &sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_non_null_timestamptz_origin_try_new_error_upper_camel_case
                                ),
                            };
                            quote::quote! {{#ts}}
                        }
                    );
                quote::quote! {
                    #allow_clippy_arbitrary_src_item_ordering
                    #serde_error_enum_token_stream
                }
            } else {
                proc_macro2::TokenStream::new()
            };
            let maybe_pub_enum_identifier_standard_non_null_origin_try_new_for_de_error_token_stream = if matches!(&is_standard_non_null, pg_crud_macros_common::IsStandardNonNull::True) {
                //todo this is bad design. refactor later
                let generate_error_token_stream = |pg_type_impl_try_new_for_deserialize: &PgTypeImplTryNewForDe|{
                    let serde_error_enum_token_stream = pg_crud_macros_common::token_stream_helpers::serde_error_enum_d_token_stream_builder()
                    .build_enum(
                        &proc_macro2::TokenStream::new(),
                        &identifier_standard_non_null_origin_try_new_for_de_error_upper_camel_case,
                        &proc_macro2::TokenStream::new(),
                        &{
                            let ts: &dyn quote::ToTokens = match &pg_type_impl_try_new_for_deserialize {
                                PgTypeImplTryNewForDe::StringAsText => &string_as_text_try_new_error_variants_token_stream,
                                PgTypeImplTryNewForDe::SqlxTypesChronoNaiveTimeAsTime => &{
                                    let invalid_hour_or_minute_or_second_or_microsecond_var_token_stream = generate_location_var_token_stream(
                                        &invalid_hour_or_minute_or_second_or_microsecond_upper_camel_case,
                                        &quote::quote! {
                                            #[eo_to_err_string_serde]
                                            #hour_snake_case: #u32_token_stream,
                                            #[eo_to_err_string_serde]
                                            #min_snake_case: #u32_token_stream,
                                            #[eo_to_err_string_serde]
                                            #sec_snake_case: #u32_token_stream,
                                            #[eo_to_err_string_serde]
                                            #micro_snake_case: #u32_token_stream,
                                        }
                                    );
                                    quote::quote! {
                                        #invalid_hour_or_minute_or_second_or_microsecond_var_token_stream,
                                        #nanosecond_precision_is_not_supported_variant_try_new_token_stream
                                    }
                                },
                                PgTypeImplTryNewForDe::SqlxTypesTimeTimeAsTime => &{
                                    let invalid_hour_or_minute_or_second_or_microsecond_var_token_stream = generate_location_var_token_stream(
                                        &invalid_hour_or_minute_or_second_or_microsecond_upper_camel_case,
                                        &quote::quote! {
                                            #[eo_to_err_string_serde]
                                            #error_snake_case: #string_token_stream,
                                            #[eo_to_err_string_serde]
                                            #microsecond_snake_case: #u32_token_stream,
                                            #[eo_to_err_string_serde]
                                            #hour_snake_case: #u8_token_stream,
                                            #[eo_to_err_string_serde]
                                            #minute_snake_case: #u8_token_stream,
                                            #[eo_to_err_string_serde]
                                            #second_snake_case: #u8_token_stream,
                                        }
                                    );
                                    quote::quote! {
                                        #invalid_hour_or_minute_or_second_or_microsecond_var_token_stream,
                                        #nanosecond_precision_is_not_supported_variant_try_new_token_stream
                                    }
                                },
                                PgTypeImplTryNewForDe::SqlxTypesChronoNaiveDateAsDate => &sqlx_types_chrono_naive_date_as_date_try_new_error_variants_token_stream,
                                PgTypeImplTryNewForDe::SqlxPgTypesPgRangeI32AsInt4Range => &generate_int_range_type_error_variants_token_stream(&IntRangeType::SqlxPgTypesPgRangeI32AsInt4Range),
                                PgTypeImplTryNewForDe::SqlxPgTypesPgRangeI64AsInt8Range => &generate_int_range_type_error_variants_token_stream(&IntRangeType::SqlxPgTypesPgRangeI64AsInt8Range),
                                PgTypeImplTryNewForDe::SqlxTypesUuidUuidAsUuidV4InitializationByPg |
                                PgTypeImplTryNewForDe::SqlxTypesUuidUuidAsUuidInitializationByClient => &uuid_as_uuid_v4_as_string_try_new_error_variants_token_stream,
                            };
                            quote::quote! {{#ts}}
                        }
                    );
                    quote::quote! {
                        #allow_clippy_arbitrary_src_item_ordering
                        #serde_error_enum_token_stream
                    }
                };
                match &de_derive_or_impl {
                    pg_crud_macros_common::DeriveOrImpl::Derive => if matches!(&is_standard_non_null, pg_crud_macros_common::IsStandardNonNull::True) {
                        match &pg_type {
                            PgType::I16AsInt2 |
                            PgType::I32AsInt4 |
                            PgType::I64AsInt8 |
                            PgType::F32AsFloat4 |
                            PgType::F64AsFloat8 |
                            PgType::I16AsSmallSerialInitializationByPg |
                            PgType::I32AsSerialInitializationByPg |
                            PgType::I64AsBigSerialInitializationByPg |
                            PgType::BoolAsBool |
                            PgType::StdVecVecU8AsBytea |
                            PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp |
                            PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz |
                            PgType::SqlxTypesIpnetworkIpNetworkAsInet |
                            PgType::SqlxTypesMacAddressMacAddressAsMacAddr |
                            PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange |
                            PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange |
                            PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange |
                            PgType::SqlxPgTypesPgMoneyAsMoney |
                            PgType::SqlxPgTypesPgIntervalAsInterval => proc_macro2::TokenStream::new(),
                            PgType::StringAsText => generate_error_token_stream(&PgTypeImplTryNewForDe::StringAsText),
                            PgType::SqlxTypesChronoNaiveTimeAsTime => generate_error_token_stream(&PgTypeImplTryNewForDe::SqlxTypesChronoNaiveTimeAsTime),
                            PgType::SqlxTypesTimeTimeAsTime => generate_error_token_stream(&PgTypeImplTryNewForDe::SqlxTypesTimeTimeAsTime),
                            PgType::SqlxTypesChronoNaiveDateAsDate => generate_error_token_stream(&PgTypeImplTryNewForDe::SqlxTypesChronoNaiveDateAsDate),
                            PgType::SqlxTypesUuidUuidAsUuidV4InitializationByPg => generate_error_token_stream(&PgTypeImplTryNewForDe::SqlxTypesUuidUuidAsUuidV4InitializationByPg),
                            PgType::SqlxTypesUuidUuidAsUuidInitializationByClient => generate_error_token_stream(&PgTypeImplTryNewForDe::SqlxTypesUuidUuidAsUuidInitializationByClient),
                            PgType::SqlxPgTypesPgRangeI32AsInt4Range => generate_error_token_stream(&PgTypeImplTryNewForDe::SqlxPgTypesPgRangeI32AsInt4Range),
                            PgType::SqlxPgTypesPgRangeI64AsInt8Range => generate_error_token_stream(&PgTypeImplTryNewForDe::SqlxPgTypesPgRangeI64AsInt8Range),
                        }
                    }
                    else {
                        proc_macro2::TokenStream::new()
                    },
                    pg_crud_macros_common::DeriveOrImpl::Impl(_) => match &pg_type_deserialize {
                        PgTypeDeserialize::Derive => proc_macro2::TokenStream::new(),
                        PgTypeDeserialize::ImplNewForDeserializeOrTryNewForDe(pg_type_impl_new_for_de_or_try_new_for_deserialize) => match &pg_type_impl_new_for_de_or_try_new_for_deserialize {
                            PgTypeImplNewForDeserializeOrTryNewForDe::NewForDeserialize => proc_macro2::TokenStream::new(),
                            PgTypeImplNewForDeserializeOrTryNewForDe::TryNewForDe(pg_type_impl_try_new_for_deserialize) => generate_error_token_stream(pg_type_impl_try_new_for_deserialize)
                        },
                    }
                }
            } else {
                proc_macro2::TokenStream::new()
            };
            let impl_identifier_origin_token_stream = {
                let fn_new_or_try_new_token_stream = pg_type_initialization_try_new_try_from_pg_type.as_ref().map_or_else(
                |()| {
                    let ts = {
                        let ts = {
                            let generate_match_optional_token_stream = |ts: &dyn quote::ToTokens| {
                                quote::quote! {#v_snake_case.map(#ts::#new_snake_case)}
                            };
                            match &pg_type_pattern {
                                PgTypePattern::Standard => match &is_nullable {
                                    pg_crud_macros_common::IsNullable::False => {
                                        range_try_from_pg_type.as_ref().map_or_else(
                                            |()| quote::quote! {#v_snake_case},
                                            |range_try_from| generate_pg_range_conversion_token_stream(
                                                &v_snake_case,
                                                &{
                                                    let range_pg_type_identifier_origin = naming::parameter::SelfOriginUpperCamelCase::from_display(&generate_identifier_str(&PgType::from(range_try_from), is_nullable, pg_type_pattern));
                                                    quote::quote! {#range_pg_type_identifier_origin::#new_snake_case(v_af65ccce)}
                                                }
                                            )
                                        )
                                    }
                                    pg_crud_macros_common::IsNullable::True => generate_match_optional_token_stream(&identifier_standard_non_null_origin_upper_camel_case),
                                },
                            }
                        };
                        quote::quote! {Self(#ts)}
                    };
                    match &pg_type_pattern {
                        PgTypePattern::Standard => match &is_nullable {
                            pg_crud_macros_common::IsNullable::False => macros_helpers::generate_new_or_try_new::generate_const_new_token_stream(
                                &must_use,
                                &v_identifier_inner_type_token_stream,
                                &ts
                            ),
                            pg_crud_macros_common::IsNullable::True => macros_helpers::generate_new_or_try_new::generate_new_token_stream(
                                &must_use,
                                &v_identifier_inner_type_token_stream,
                                &ts
                            ),
                        },
                    }
                },
                |pg_type_initialization_try_new| {
                    let ts = {
                        let generate_match_optional_token_stream = |ts: &dyn quote::ToTokens| {
                            quote::quote! {Ok(Self(match #v_snake_case {
                                Some(v_989d943e) => Some(match #ts::#try_new_snake_case(v_989d943e) {
                                    Ok(v_ea2a4a8c) => v_ea2a4a8c,
                                    Err(error) => {
                                        return Err(error);
                                    },
                                }),
                                None => None
                            }))}
                        };
                        match &pg_type_pattern {
                            PgTypePattern::Standard => match &is_nullable {
                                pg_crud_macros_common::IsNullable::False => {
                                    let generate_int_range_check_token_stream = |int_range_type: &IntRangeType| {
                                        let max_v_token_stream = {
                                            let type_token_stream = int_range_type_to_range_inner_type_token_stream(int_range_type);
                                            quote::quote! {#type_token_stream::MAX}
                                        };
                                        quote::quote! {
                                            let max = #max_v_token_stream;
                                            let (#start_snake_case, #end_snake_case) = match (#v_snake_case.#start_snake_case, #v_snake_case.#end_snake_case) {
                                                (std::ops::Bound::Included(#start_snake_case), std::ops::Bound::Included(#end_snake_case)) => {
                                                    if #start_snake_case > #end_snake_case {
                                                        return Err(#identifier_standard_non_null_origin_try_new_error_upper_camel_case::#included_start_greater_than_included_end_upper_camel_case {
                                                            #start_snake_case,
                                                            #end_snake_case,
                                                            location: location_macros::location!(),
                                                        });
                                                    }
                                                    if #end_snake_case == max {
                                                        return Err(#identifier_standard_non_null_origin_try_new_error_upper_camel_case::#included_end_cannot_be_max_upper_camel_case {
                                                            #end_snake_case,
                                                            location: location_macros::location!(),
                                                        });
                                                    }
                                                    (std::ops::Bound::Included(#start_snake_case), std::ops::Bound::Included(#end_snake_case))
                                                }
                                                (std::ops::Bound::Included(#start_snake_case), std::ops::Bound::Excluded(#end_snake_case)) => {
                                                    if #start_snake_case > #end_snake_case {
                                                        return Err(#identifier_standard_non_null_origin_try_new_error_upper_camel_case::#included_start_greater_than_excluded_end_upper_camel_case {
                                                            #start_snake_case,
                                                            #end_snake_case,
                                                            location: location_macros::location!(),
                                                        });
                                                    }
                                                    (std::ops::Bound::Included(#start_snake_case), std::ops::Bound::Excluded(#end_snake_case))
                                                }
                                                (std::ops::Bound::Included(#start_snake_case), std::ops::Bound::Unbounded) => (std::ops::Bound::Included(#start_snake_case), std::ops::Bound::Unbounded),
                                                (std::ops::Bound::Excluded(#start_snake_case), std::ops::Bound::Included(#end_snake_case)) => {
                                                    if #start_snake_case > #end_snake_case {
                                                        return Err(#identifier_standard_non_null_origin_try_new_error_upper_camel_case::#excluded_start_greater_than_included_end_upper_camel_case {
                                                            #start_snake_case,
                                                            #end_snake_case,
                                                            location: location_macros::location!(),
                                                        });
                                                    }
                                                    if #end_snake_case == max {
                                                        return Err(#identifier_standard_non_null_origin_try_new_error_upper_camel_case::#included_end_cannot_be_max_upper_camel_case {
                                                            #end_snake_case,
                                                            location: location_macros::location!(),
                                                        });
                                                    }
                                                    (std::ops::Bound::Excluded(#start_snake_case), std::ops::Bound::Included(#end_snake_case))
                                                }
                                                (std::ops::Bound::Excluded(#start_snake_case), std::ops::Bound::Excluded(#end_snake_case)) => {
                                                    if #start_snake_case > #end_snake_case {
                                                        return Err(#identifier_standard_non_null_origin_try_new_error_upper_camel_case::#excluded_start_greater_than_excluded_end_upper_camel_case {
                                                            #start_snake_case,
                                                            #end_snake_case,
                                                            location: location_macros::location!(),
                                                        });
                                                    }
                                                    (std::ops::Bound::Excluded(#start_snake_case), std::ops::Bound::Excluded(#end_snake_case))
                                                }
                                                (std::ops::Bound::Excluded(#start_snake_case), std::ops::Bound::Unbounded) => (std::ops::Bound::Excluded(#start_snake_case), std::ops::Bound::Unbounded),
                                                (std::ops::Bound::Unbounded, std::ops::Bound::Included(#end_snake_case)) => {
                                                    if #end_snake_case == max {
                                                        return Err(#identifier_standard_non_null_origin_try_new_error_upper_camel_case::#included_end_cannot_be_max_upper_camel_case {
                                                            #end_snake_case,
                                                            location: location_macros::location!(),
                                                        });
                                                    }
                                                    (std::ops::Bound::Unbounded, std::ops::Bound::Included(#end_snake_case))
                                                }
                                                (std::ops::Bound::Unbounded, std::ops::Bound::Excluded(#end_snake_case)) => (std::ops::Bound::Unbounded, std::ops::Bound::Excluded(#end_snake_case)),
                                                (std::ops::Bound::Unbounded, std::ops::Bound::Unbounded) => (std::ops::Bound::Unbounded, std::ops::Bound::Unbounded),
                                            };
                                            Ok(Self(sqlx::postgres::types::PgRange { #start_snake_case, #end_snake_case }))
                                        }
                                    };
                                    let generate_ok_self_sqlx_pg_types_pg_range_token_stream = |ts: &dyn quote::ToTokens| {
                                        let generate_bound_arms_token_stream = |variant_token_stream: &dyn quote::ToTokens| quote::quote! {
                                            std::ops::Bound::Included(v_bound_incl) => match #ts::#try_new_snake_case(v_bound_incl) {
                                                Ok(v_bound_ok) => std::ops::Bound::Included(v_bound_ok.0),
                                                Err(error) => {
                                                    return Err(#identifier_standard_non_null_origin_try_new_error_upper_camel_case::#variant_token_stream {
                                                        #error_snake_case,
                                                        location: location_macros::location!(),
                                                    });
                                                }
                                            },
                                            std::ops::Bound::Excluded(v_bound_excl) => match #ts::#try_new_snake_case(v_bound_excl) {
                                                Ok(v_bound_ok) => std::ops::Bound::Excluded(v_bound_ok.0),
                                                Err(error) => {
                                                    return Err(#identifier_standard_non_null_origin_try_new_error_upper_camel_case::#variant_token_stream {
                                                        #error_snake_case,
                                                        location: location_macros::location!(),
                                                    });
                                                }
                                            },
                                            std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
                                        };
                                        let start_arms_token_stream = generate_bound_arms_token_stream(&quote::quote! {#start_upper_camel_case});
                                        let end_arms_token_stream = generate_bound_arms_token_stream(&quote::quote! {#end_upper_camel_case});
                                        quote::quote! {
                                            Ok(Self(sqlx::postgres::types::PgRange {
                                                #start_snake_case: match #v_snake_case.#start_snake_case { #start_arms_token_stream },
                                                #end_snake_case: match #v_snake_case.#end_snake_case { #end_arms_token_stream },
                                            }))
                                        }
                                    };
                                    match &pg_type_initialization_try_new {
                                        PgTypeInitializationTryNew::F64AsFloat8 => quote::quote! {
                                            if #v_snake_case.is_finite() {
                                                Ok(Self(#v_snake_case))
                                            } else {
                                                Err(#identifier_standard_non_null_origin_try_new_error_upper_camel_case::#not_finite_upper_camel_case {
                                                    location: location_macros::location!(),
                                                })
                                            }
                                        },
                                        PgTypeInitializationTryNew::StringAsText => quote::quote! {
                                            if #v_snake_case.find('\0').is_some() {
                                                Err(#identifier_standard_non_null_origin_try_new_error_upper_camel_case::#contains_null_byte_upper_camel_case {
                                                    #v_snake_case,
                                                    location: location_macros::location!(),
                                                })
                                            } else {
                                                Ok(Self(#v_snake_case))
                                            }
                                        },
                                        PgTypeInitializationTryNew::SqlxTypesChronoNaiveTimeAsTime => quote::quote! {
                                            if <#inner_type_standard_non_null_token_stream as chrono::Timelike>::nanosecond(&#v_snake_case).checked_rem(1000).expect("7c8b4e12") != 0 {
                                                return Err(#identifier_standard_non_null_origin_try_new_error_upper_camel_case::#nanosecond_precision_is_not_supported_upper_camel_case {
                                                    #v_snake_case: #v_snake_case.to_string(),
                                                    location: location_macros::location!(),
                                                });
                                            }
                                            Ok(Self(#v_snake_case))
                                        },
                                        PgTypeInitializationTryNew::SqlxTypesTimeTimeAsTime => quote::quote! {
                                            if #v_snake_case.nanosecond().checked_rem(1000).expect("ce47524f") != 0 {
                                                return Err(#identifier_standard_non_null_origin_try_new_error_upper_camel_case::#nanosecond_precision_is_not_supported_upper_camel_case {
                                                    #v_snake_case: #v_snake_case.to_string(),
                                                    location: location_macros::location!(),
                                                });
                                            }
                                            Ok(Self(#v_snake_case))
                                        },
                                        PgTypeInitializationTryNew::SqlxTypesChronoNaiveDateAsDate => quote::quote! {
                                            let #earliest_supported_date_snake_case = #inner_type_standard_non_null_token_stream::from_ymd_opt(-4713, 12, 31).expect("9f6241e5");
                                            if #v_snake_case >= #earliest_supported_date_snake_case {
                                                Ok(Self(#v_snake_case))
                                            }
                                            else {
                                                Err(#identifier_standard_non_null_origin_try_new_error_upper_camel_case::#earlier_date_not_supported_upper_camel_case {
                                                    value: #v_snake_case.to_string(),
                                                    #earliest_supported_date_snake_case: #earliest_supported_date_snake_case.to_string(),
                                                    location: location_macros::location!(),
                                                })
                                            }
                                        },
                                        PgTypeInitializationTryNew::SqlxTypesChronoNaiveDateTimeAsTimestamp => quote::quote! {
                                            let #date_snake_case = match #sqlx_types_chrono_naive_date_as_non_null_date_origin_upper_camel_case::#try_new_snake_case(
                                                #v_snake_case.#date_snake_case()
                                            ) {
                                                Ok(v_9be8eddb) => v_9be8eddb,
                                                Err(error) => {
                                                    return Err(#identifier_standard_non_null_origin_try_new_error_upper_camel_case::#date_upper_camel_case {
                                                        #error_snake_case,
                                                        location: location_macros::location!(),
                                                    });
                                                }
                                            };
                                            let #time_snake_case = match #sqlx_types_chrono_naive_time_as_non_null_time_origin_upper_camel_case::#try_new_snake_case(
                                                #v_snake_case.#time_snake_case()
                                            ) {
                                                Ok(v_993484ce) => v_993484ce,
                                                Err(error) => {
                                                    return Err(#identifier_standard_non_null_origin_try_new_error_upper_camel_case::#time_upper_camel_case {
                                                        #error_snake_case,
                                                        location: location_macros::location!(),
                                                    });
                                                }
                                            };
                                            Ok(Self(#inner_type_standard_non_null_token_stream::#new_snake_case(#date_snake_case.0, #time_snake_case.0)))
                                        },
                                        PgTypeInitializationTryNew::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => {
                                            let sqlx_types_chrono_date_time_sqlx_types_chrono_utc_from_naive_utc_and_offset_token_stream = generate_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_from_naive_utc_and_offset_token_stream(&generate_sqlx_types_chrono_naive_date_time_new_token_stream(&quote::quote! {
                                                #date_naive_snake_case.0,
                                                #time_snake_case.0
                                            }));
                                            quote::quote! {
                                                let #date_naive_snake_case = match #sqlx_types_chrono_naive_date_as_non_null_date_origin_upper_camel_case::#try_new_snake_case(#v_snake_case.date_naive()) {
                                                    Ok(v_158945ad) => v_158945ad,
                                                    Err(error) => {
                                                        return Err(#identifier_standard_non_null_origin_try_new_error_upper_camel_case::#date_naive_upper_camel_case {
                                                            #error_snake_case,
                                                            location: location_macros::location!(),
                                                        });
                                                    }
                                                };
                                                let #time_snake_case = match #sqlx_types_chrono_naive_time_as_non_null_time_origin_upper_camel_case::#try_new_snake_case(#v_snake_case.time()) {
                                                    Ok(v_c5af739c) => v_c5af739c,
                                                    Err(error) => {
                                                        return Err(#identifier_standard_non_null_origin_try_new_error_upper_camel_case::#time_upper_camel_case {
                                                            #error_snake_case,
                                                            location: location_macros::location!(),
                                                        });
                                                    }
                                                };
                                                Ok(Self(#sqlx_types_chrono_date_time_sqlx_types_chrono_utc_from_naive_utc_and_offset_token_stream))
                                            }
                                        }
                                        PgTypeInitializationTryNew::SqlxPgTypesPgRangeI32AsInt4Range => generate_int_range_check_token_stream(&IntRangeType::SqlxPgTypesPgRangeI32AsInt4Range),
                                        PgTypeInitializationTryNew::SqlxPgTypesPgRangeI64AsInt8Range => generate_int_range_check_token_stream(&IntRangeType::SqlxPgTypesPgRangeI64AsInt8Range),
                                        PgTypeInitializationTryNew::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => generate_ok_self_sqlx_pg_types_pg_range_token_stream(&sqlx_types_chrono_naive_date_as_non_null_date_origin_upper_camel_case),
                                        PgTypeInitializationTryNew::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => generate_ok_self_sqlx_pg_types_pg_range_token_stream(&sqlx_types_chrono_naive_date_time_as_non_null_timestamp_origin_upper_camel_case),
                                        PgTypeInitializationTryNew::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => generate_ok_self_sqlx_pg_types_pg_range_token_stream(&sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_non_null_timestamptz_origin_upper_camel_case),
                                    }
                                }
                                pg_crud_macros_common::IsNullable::True => generate_match_optional_token_stream(&identifier_standard_non_null_origin_upper_camel_case),
                            },
                        }
                    };
                    quote::quote! {
                        pub fn #try_new_snake_case(#v_identifier_inner_type_token_stream) -> Result<Self, #identifier_standard_non_null_origin_try_new_error_upper_camel_case> {
                            #ts
                        }
                    }
                    .into()
                });
                let maybe_fn_new_or_try_new_for_de_token = {
                    let generate_v_pg_range_int_type_token_stream = |int_range_type: &IntRangeType| {
                        let type_token_stream = {
                            let ts = int_range_type_to_range_inner_type_token_stream(int_range_type);
                            quote::quote! {std::ops::Bound<#ts>}
                        };
                        quote::quote! {
                            start_9a8ef454: #type_token_stream,
                            end_a14eb2b9: #type_token_stream
                        }
                    };
                    match &pg_type_pattern {
                        PgTypePattern::Standard => match &is_nullable {
                            pg_crud_macros_common::IsNullable::False => match &pg_type_deserialize {
                                PgTypeDeserialize::Derive => if matches!(&is_standard_non_null, pg_crud_macros_common::IsStandardNonNull::True) {
                                    match &pg_type {
                                        PgType::I16AsInt2 |
                                        PgType::I32AsInt4 |
                                        PgType::I64AsInt8 |
                                        PgType::F32AsFloat4 |
                                        PgType::F64AsFloat8 |
                                        PgType::I16AsSmallSerialInitializationByPg |
                                        PgType::I32AsSerialInitializationByPg |
                                        PgType::I64AsBigSerialInitializationByPg |
                                        PgType::BoolAsBool |
                                        PgType::StdVecVecU8AsBytea |
                                        PgType::SqlxTypesIpnetworkIpNetworkAsInet |
                                        PgType::SqlxTypesMacAddressMacAddressAsMacAddr |
                                        PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange |
                                        PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange |
                                        PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange |
                                        PgType::SqlxPgTypesPgMoneyAsMoney |
                                        PgType::SqlxTypesChronoNaiveTimeAsTime |
                                        PgType::SqlxTypesTimeTimeAsTime |
                                        PgType::SqlxPgTypesPgIntervalAsInterval |
                                        PgType::SqlxTypesChronoNaiveDateAsDate |
                                        PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp |
                                        PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz |
                                        PgType::StringAsText |
                                        PgType::SqlxTypesUuidUuidAsUuidInitializationByClient |
                                        PgType::SqlxTypesUuidUuidAsUuidV4InitializationByPg => proc_macro2::TokenStream::new(),
                                        PgType::SqlxPgTypesPgRangeI32AsInt4Range => generate_v_pg_range_int_type_token_stream(&IntRangeType::SqlxPgTypesPgRangeI32AsInt4Range),
                                        PgType::SqlxPgTypesPgRangeI64AsInt8Range => generate_v_pg_range_int_type_token_stream(&IntRangeType::SqlxPgTypesPgRangeI64AsInt8Range),
                                    }
                                }
                                else {
                                    proc_macro2::TokenStream::new()
                                },
                                PgTypeDeserialize::ImplNewForDeserializeOrTryNewForDe(_) => proc_macro2::TokenStream::new()
                            },
                            pg_crud_macros_common::IsNullable::True => proc_macro2::TokenStream::new(),
                        },
                    }
                };
                quote::quote! {
                    #allow_clippy_arbitrary_src_item_ordering
                    impl #identifier_origin_upper_camel_case {
                        #fn_new_or_try_new_token_stream
                        #maybe_fn_new_or_try_new_for_de_token
                    }
                }
            };
            let impl_from_identifier_origin_for_identifier_inner_type_token_stream = macros_helpers::generate_impl_from_token_stream::generate_impl_from_token_stream(
                &identifier_origin_upper_camel_case,
                &identifier_inner_type_token_stream,
                &{
                    let v_dot_zero = quote::quote! {#v_snake_case.0};
                    let generate_match_token_stream = |
                        match_token_stream: &dyn quote::ToTokens,
                        some_token_stream: &dyn quote::ToTokens,
                        some_v_token_stream: &dyn quote::ToTokens,
                    | quote::quote! {
                        #match_token_stream.map(|#some_v_token_stream|#some_v_token_stream.0#some_token_stream)
                    };
                    match &pg_type_pattern {
                        PgTypePattern::Standard => match &is_nullable {
                            pg_crud_macros_common::IsNullable::False => v_dot_zero,
                            pg_crud_macros_common::IsNullable::True => generate_match_token_stream(
                                &v_dot_zero,
                                &proc_macro2::TokenStream::new(),
                                &quote::quote! {v_6bfd70fa}
                            ),
                        },
                    }
                }
            );
            let maybe_impl_is_string_empty_for_identifier_origin_token_stream = if matches!(&is_standard_non_null, pg_crud_macros_common::IsStandardNonNull::True) {
                match &is_nullable {
                    pg_crud_macros_common::IsNullable::False => match &pg_type {
                        PgType::I16AsInt2
                        | PgType::I32AsInt4
                        | PgType::I64AsInt8
                        | PgType::F32AsFloat4
                        | PgType::F64AsFloat8
                        | PgType::I16AsSmallSerialInitializationByPg
                        | PgType::I32AsSerialInitializationByPg
                        | PgType::I64AsBigSerialInitializationByPg
                        | PgType::SqlxPgTypesPgMoneyAsMoney
                        | PgType::BoolAsBool
                        | PgType::StdVecVecU8AsBytea
                        | PgType::SqlxTypesChronoNaiveTimeAsTime
                        | PgType::SqlxTypesTimeTimeAsTime
                        | PgType::SqlxPgTypesPgIntervalAsInterval
                        | PgType::SqlxTypesChronoNaiveDateAsDate
                        | PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp
                        | PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz
                        | PgType::SqlxTypesIpnetworkIpNetworkAsInet
                        | PgType::SqlxPgTypesPgRangeI32AsInt4Range
                        | PgType::SqlxPgTypesPgRangeI64AsInt8Range
                        | PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange
                        | PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange
                        | PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => macros_helpers::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream::from(proc_macro2::TokenStream::new()),
                        PgType::StringAsText => pg_crud_macros_common::generate_impl_crate_is_string_empty_for_identifier_token_stream(
                            &identifier_origin_upper_camel_case,
                            &quote::quote! {self.0.is_empty()},
                        ),
                        PgType::SqlxTypesUuidUuidAsUuidV4InitializationByPg |
                        PgType::SqlxTypesUuidUuidAsUuidInitializationByClient |
                        PgType::SqlxTypesMacAddressMacAddressAsMacAddr => pg_crud_macros_common::generate_impl_crate_is_string_empty_for_identifier_token_stream(
                            &identifier_origin_upper_camel_case,
                            &quote::quote! {false},
                        ),
                    },
                    pg_crud_macros_common::IsNullable::True => macros_helpers::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream::from(proc_macro2::TokenStream::new()),
                }
            } else {
                macros_helpers::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream::from(proc_macro2::TokenStream::new())
            };
            let empty_generated_token_stream = macros_helpers::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream::from(proc_macro2::TokenStream::new());
            let maybe_impl_ser_for_identifier_standard_non_null_origin_token_stream = match &ser_derive_or_impl {
                pg_crud_macros_common::DeriveOrImpl::Derive => &empty_generated_token_stream,
                pg_crud_macros_common::DeriveOrImpl::Impl(v) => v,
            };
            let maybe_impl_de_for_identifier_standard_non_null_origin_token_stream = match &de_derive_or_impl {
                pg_crud_macros_common::DeriveOrImpl::Derive => &empty_generated_token_stream,
                pg_crud_macros_common::DeriveOrImpl::Impl(v) => v,
            };
            let md_de_from_for_identifier_stndrt_non_null_origin_token_stream = if matches!(&is_standard_non_null, pg_crud_macros_common::IsStandardNonNull::True) {
                let self_sqlx_pg_types_pg_range_token_stream = {
                    let (start_token_stream, end_token_stream) = {
                        let generate_token_stream = |start_or_end: StartOrEnd|{
                            let name_token_stream = match start_or_end {
                                StartOrEnd::End => quote::quote! {end},
                                StartOrEnd::Start => quote::quote! {start},
                            };
                            let ts0 = match start_or_end {
                                StartOrEnd::End => quote::quote! {v.end},
                                StartOrEnd::Start => quote::quote! {v.start},
                            };
                            quote::quote! {
                                #name_token_stream: match #ts0 {
                                    std::ops::Bound::Included(v0) => std::ops::Bound::Included(v0.0),
                                    std::ops::Bound::Excluded(v0) => std::ops::Bound::Excluded(v0.0),
                                    std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
                                },
                            }
                        };
                        (generate_token_stream(StartOrEnd::Start), generate_token_stream(StartOrEnd::End))
                    };
                    quote::quote! {Self(sqlx::postgres::types::PgRange {
                        #start_token_stream
                        #end_token_stream
                    })}
                };
                let generate_impl_from_origin_token_stream = |
                    from_type_token_stream: &dyn quote::ToTokens,
                    ts: &dyn quote::ToTokens,
                |macros_helpers::generate_impl_from_token_stream::generate_impl_from_token_stream(
                    from_type_token_stream,
                    &identifier_origin_upper_camel_case,
                    ts,
                ).into();
                match &pg_type {
                    PgType::I16AsInt2 |
                    PgType::I32AsInt4 |
                    PgType::I64AsInt8 |
                    PgType::F32AsFloat4 |
                    PgType::F64AsFloat8 |
                    PgType::I16AsSmallSerialInitializationByPg |
                    PgType::I32AsSerialInitializationByPg |
                    PgType::I64AsBigSerialInitializationByPg |
                    PgType::BoolAsBool |
                    PgType::StringAsText |
                    PgType::StdVecVecU8AsBytea |
                    PgType::SqlxTypesChronoNaiveTimeAsTime |
                    PgType::SqlxTypesTimeTimeAsTime |
                    PgType::SqlxTypesChronoNaiveDateAsDate |
                    PgType::SqlxTypesUuidUuidAsUuidV4InitializationByPg |
                    PgType::SqlxTypesUuidUuidAsUuidInitializationByClient |
                    PgType::SqlxTypesIpnetworkIpNetworkAsInet |
                    PgType::SqlxPgTypesPgRangeI32AsInt4Range |
                    PgType::SqlxPgTypesPgRangeI64AsInt8Range => proc_macro2::TokenStream::new(),
                    PgType::SqlxPgTypesPgMoneyAsMoney => generate_impl_from_origin_token_stream(
                        &quote::quote! {i64},
                        &quote::quote! {Self::new(#inner_type_standard_non_null_token_stream(v))}
                    ),
                    PgType::SqlxPgTypesPgIntervalAsInterval => generate_impl_from_origin_token_stream(
                        &identifier_origin_wire_upper_camel_case,
                        &quote::quote! {
                            Self(sqlx::postgres::types::PgInterval {
                                #months_snake_case: v.months,
                                #days_snake_case: v.days,
                                #microseconds_snake_case: v.microseconds,
                            })
                        }
                    ),
                    //todo reuse naming
                    PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp => generate_impl_from_origin_token_stream(
                        &identifier_origin_wire_upper_camel_case,
                        &quote::quote! {Self(#inner_type_standard_non_null_token_stream::#new_snake_case(v.date.0, v.time.0))}
                    ),
                    PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => generate_impl_from_origin_token_stream(
                        &identifier_origin_wire_upper_camel_case,
                        &{
                            let ts = generate_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_from_naive_utc_and_offset_token_stream(&generate_sqlx_types_chrono_naive_date_time_new_token_stream(&quote::quote! {
                                v.date_naive.0,
                                v.time.0
                            }));
                            quote::quote! {Self(#ts)}
                        }
                    ),
                    PgType::SqlxTypesMacAddressMacAddressAsMacAddr => generate_impl_from_origin_token_stream(
                        &quote::quote! {[u8; 6]},
                        &quote::quote! {Self(#inner_type_standard_non_null_token_stream::new(v))}
                    ),
                    PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange |
                    PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange |
                    PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => generate_impl_from_origin_token_stream(
                        &identifier_origin_wire_upper_camel_case,
                        &self_sqlx_pg_types_pg_range_token_stream
                    ),
                }
            }
            else {
                proc_macro2::TokenStream::new()
            };
            let md_de_try_from_for_identifier_stndrt_non_null_origin_token_stream = if matches!(&is_standard_non_null, pg_crud_macros_common::IsStandardNonNull::True) {
                let generate_self_match_try_new_token_stream = |parameters_token_stream: &dyn quote::ToTokens, match_error_variants_token_stream: &dyn quote::ToTokens| {
                    quote::quote! {
                        match Self::#try_new_snake_case(#parameters_token_stream) {
                            Ok(v_b318fc86) => Ok(v_b318fc86),
                            Err(error) => match error {
                                #match_error_variants_token_stream
                            }
                        }
                    }
                };
                let generate_impl_try_from_origin_token_stream = |
                    from_type_token_stream: &dyn quote::ToTokens,
                    error_type_token_stream: &dyn quote::ToTokens,
                    ts: &dyn quote::ToTokens
                |macros_helpers::generate_impl_try_from_token_stream::generate_impl_try_from_token_stream(
                    from_type_token_stream,
                    &identifier_origin_upper_camel_case,
                    error_type_token_stream,
                    ts
                ).into();
                let generate_impl_try_from_de_error_token_stream = |
                    from_type_token_stream: &dyn quote::ToTokens,
                    ts: &dyn quote::ToTokens
                |generate_impl_try_from_origin_token_stream(
                    from_type_token_stream,
                    &identifier_standard_non_null_origin_try_new_for_de_error_upper_camel_case,
                    ts
                );
                let generate_impl_try_from_int_range_token_stream = |
                    _int_range_type: IntRangeType,
                |generate_impl_try_from_de_error_token_stream(
                    &identifier_origin_wire_upper_camel_case,
                    &generate_self_match_try_new_token_stream(
                        &quote::quote! {sqlx::postgres::types::PgRange { #start_snake_case: v.start, #end_snake_case: v.end }},
                        &{
                            let generate_match_token_stream = |name_token_stream: &dyn quote::ToTokens, ts: &dyn quote::ToTokens|quote::quote! {
                                #identifier_standard_non_null_origin_try_new_error_upper_camel_case::#name_token_stream {
                                    location,
                                    #ts
                                } => Err(#identifier_standard_non_null_origin_try_new_for_de_error_upper_camel_case::#name_token_stream {
                                    location,
                                    #ts
                                }),
                            };
                            let (
                                included_start_greater_than_included_end_token_stream,
                                included_start_greater_than_excluded_end_token_stream,
                                excluded_start_greater_than_included_end_token_stream,
                                excluded_start_greater_than_excluded_end_token_stream,
                            ) = {
                                let generate_token_stream = |ts: &dyn quote::ToTokens|generate_match_token_stream(
                                    &ts,
                                    &quote::quote! {
                                        #start_snake_case,
                                        #end_snake_case,
                                    }
                                );
                                (
                                    generate_token_stream(&included_start_greater_than_included_end_upper_camel_case),
                                    generate_token_stream(&included_start_greater_than_excluded_end_upper_camel_case),
                                    generate_token_stream(&excluded_start_greater_than_included_end_upper_camel_case),
                                    generate_token_stream(&excluded_start_greater_than_excluded_end_upper_camel_case),
                                )
                            };
                            let included_end_cannot_be_max_token_stream = generate_match_token_stream(
                                &included_end_cannot_be_max_upper_camel_case,
                                &quote::quote! {#end_snake_case,}
                            );
                            quote::quote! {
                                #included_start_greater_than_included_end_token_stream
                                #included_start_greater_than_excluded_end_token_stream
                                #excluded_start_greater_than_included_end_token_stream
                                #excluded_start_greater_than_excluded_end_token_stream
                                #included_end_cannot_be_max_token_stream
                            }
                        },
                    )
                );
                match &pg_type {
                    PgType::I16AsInt2 |
                    PgType::I32AsInt4 |
                    PgType::I64AsInt8 |
                    PgType::F32AsFloat4 |
                    PgType::I16AsSmallSerialInitializationByPg |
                    PgType::I32AsSerialInitializationByPg |
                    PgType::I64AsBigSerialInitializationByPg |
                    PgType::SqlxPgTypesPgMoneyAsMoney |
                    PgType::BoolAsBool |
                    PgType::StdVecVecU8AsBytea |
                    PgType::SqlxPgTypesPgIntervalAsInterval |
                    PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp |
                    PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz |
                    PgType::SqlxTypesIpnetworkIpNetworkAsInet |
                    PgType::SqlxTypesMacAddressMacAddressAsMacAddr |
                    PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange |
                    PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange |
                    PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => proc_macro2::TokenStream::new(),
                    PgType::F64AsFloat8 | PgType::StringAsText => generate_impl_try_from_origin_token_stream(
                        &inner_type_standard_non_null_token_stream,
                        &identifier_standard_non_null_origin_try_new_error_upper_camel_case,
                        &quote::quote! {Self::try_new(v)}//todo use try_from instead of try_new ?
                    ),
                    PgType::SqlxTypesChronoNaiveTimeAsTime => generate_impl_try_from_de_error_token_stream(
                        &identifier_origin_wire_upper_camel_case,
                        &quote::quote! {
                            match #inner_type_standard_non_null_token_stream::from_hms_micro_opt(
                                v.hour,
                                v.min,
                                v.sec,
                                v.micro,
                            ) {
                                Some(v_b143b9e1) => {
                                    if <#inner_type_standard_non_null_token_stream as chrono::Timelike>::nanosecond(&v_b143b9e1).checked_rem(1000).expect("c0514180") != 0 {
                                        return Err(#identifier_standard_non_null_origin_try_new_for_de_error_upper_camel_case::#nanosecond_precision_is_not_supported_upper_camel_case {
                                            #v_snake_case: v_b143b9e1.to_string(),
                                            location: location_macros::location!(),
                                        });
                                    }
                                    Ok(Self(v_b143b9e1))
                                },
                                None => Err(#identifier_standard_non_null_origin_try_new_for_de_error_upper_camel_case::#invalid_hour_or_minute_or_second_or_microsecond_upper_camel_case {
                                    #hour_snake_case: v.hour,
                                    #min_snake_case: v.min,
                                    #sec_snake_case: v.sec,
                                    #micro_snake_case: v.micro,
                                    location: location_macros::location!(),
                                })
                            }
                        }
                    ),
                    PgType::SqlxTypesTimeTimeAsTime => generate_impl_try_from_de_error_token_stream(
                        &identifier_origin_wire_upper_camel_case,
                        &quote::quote! {
                            match #inner_type_standard_non_null_token_stream::from_hms_micro(
                                v.hour,
                                v.minute,
                                v.second,
                                v.microsecond,
                            ) {
                                Ok(v_9932d535) => {
                                    if v_9932d535.nanosecond().checked_rem(1000).expect("0def33ce") != 0 {
                                        return Err(#identifier_standard_non_null_origin_try_new_for_de_error_upper_camel_case::#nanosecond_precision_is_not_supported_upper_camel_case {
                                            #v_snake_case: v_9932d535.to_string(),
                                            location: location_macros::location!(),
                                        });
                                    }
                                    Ok(Self(v_9932d535))
                                },
                                Err(error) => Err(#identifier_standard_non_null_origin_try_new_for_de_error_upper_camel_case::#invalid_hour_or_minute_or_second_or_microsecond_upper_camel_case {
                                    #hour_snake_case: v.hour,
                                    #minute_snake_case: v.minute,
                                    #second_snake_case: v.second,
                                    #microsecond_snake_case: v.microsecond,
                                    #error_snake_case: error.to_string(),
                                    location: location_macros::location!(),
                                })
                            }
                        }
                    ),
                    PgType::SqlxTypesChronoNaiveDateAsDate => generate_impl_try_from_de_error_token_stream(
                        &quote::quote! {sqlx::types::chrono::NaiveDate},
                        &generate_self_match_try_new_token_stream(
                            &v_snake_case,
                            &quote::quote! {
                                #identifier_standard_non_null_origin_try_new_error_upper_camel_case::#earlier_date_not_supported_upper_camel_case {
                                    value,
                                    #earliest_supported_date_snake_case,
                                    location,
                                } => Err(#identifier_standard_non_null_origin_try_new_for_de_error_upper_camel_case::#earlier_date_not_supported_upper_camel_case {
                                    value,
                                    #earliest_supported_date_snake_case,
                                    location,
                                }),
                            }
                        )
                    ),
                    PgType::SqlxTypesUuidUuidAsUuidInitializationByClient | PgType::SqlxTypesUuidUuidAsUuidV4InitializationByPg => generate_impl_try_from_de_error_token_stream(
                        &quote::quote! {String},
                        &quote::quote! {
                            match uuid::Uuid::try_parse(&v) {
                                Ok(v0) => Ok(Self(v0)),
                                Err(error) => Err(#identifier_standard_non_null_origin_try_new_for_de_error_upper_camel_case::#not_uuid_upper_camel_case {
                                    #v_snake_case: error.to_string(),
                                    location: location_macros::location!(),
                                })
                            }
                        }
                    ),
                    PgType::SqlxPgTypesPgRangeI32AsInt4Range => generate_impl_try_from_int_range_token_stream(
                        IntRangeType::SqlxPgTypesPgRangeI32AsInt4Range
                    ),
                    PgType::SqlxPgTypesPgRangeI64AsInt8Range => generate_impl_try_from_int_range_token_stream(
                        IntRangeType::SqlxPgTypesPgRangeI64AsInt8Range
                    ),
                }
            }
            else {
                proc_macro2::TokenStream::new()
            };
            let impl_display_for_identifier_origin_token_stream = macros_helpers::generate_impl_display_token_stream::generate_impl_display_token_stream(&proc_macro2::TokenStream::new(), &identifier_origin_upper_camel_case, &proc_macro2::TokenStream::new(), &quote::quote! {write!(f, "{self:?}")});
            let impl_location_lib_to_err_string_for_identifier_origin_token_stream = pg_crud_macros_common::generate_impl_to_err_string_no_generics_token_stream(&identifier_origin_upper_camel_case, &quote::quote! {self.to_string()});
            let some_default_some_one_element_call_token_stream = quote::quote! {Some(#pg_crud_common_default_some_one_element_call)};
            let impl_default_some_one_element_for_identifier_origin_token_stream = pg_crud_macros_common::generate_impl_pg_crud_common_default_some_one_element_token_stream(&identifier_origin_upper_camel_case, &{
                let ts = match &pg_type_pattern {
                    PgTypePattern::Standard => match &is_nullable {
                        pg_crud_macros_common::IsNullable::False => {
                            let pg_range_int_default_initialization_token_stream = quote::quote! {
                                sqlx::postgres::types::PgRange {
                                    start: std::ops::Bound::Included(#core_default),
                                    end: std::ops::Bound::Excluded(#core_default),
                                }
                            };
                            let generate_as_default_some_one_element_call_token_stream = |ts: &dyn quote::ToTokens| {
                                quote::quote! {<#ts as #import::DefaultSomeOneElement>::default_some_one_element()}
                            };
                            let generate_sqlx_pg_types_pg_range_default_some_one_element_token_stream = |ts: &dyn quote::ToTokens| {
                                let ts0 = generate_as_default_some_one_element_call_token_stream(&ts);
                                quote::quote! {sqlx::postgres::types::PgRange {
                                    #start_snake_case: std::ops::Bound::Included(#ts0.0),
                                    #end_snake_case: std::ops::Bound::Excluded(#ts0.0),
                                }}
                            };
                            let sqlx_types_chrono_naive_date_as_non_null_date_origin_as_default_some_one_element_call_token_stream = generate_as_default_some_one_element_call_token_stream(&sqlx_types_chrono_naive_date_as_non_null_date_origin_upper_camel_case);
                            let sqlx_types_chrono_naive_time_as_non_null_time_origin_as_default_some_one_element_call_token_stream = generate_as_default_some_one_element_call_token_stream(&sqlx_types_chrono_naive_time_as_non_null_time_origin_upper_camel_case);
                            let initialization_token_stream: &dyn quote::ToTokens = match &pg_type {
                                PgType::I16AsInt2
                                | PgType::I32AsInt4
                                | PgType::I64AsInt8
                                | PgType::F32AsFloat4
                                | PgType::F64AsFloat8
                                | PgType::I16AsSmallSerialInitializationByPg
                                | PgType::I32AsSerialInitializationByPg
                                | PgType::I64AsBigSerialInitializationByPg
                                | PgType::BoolAsBool
                                | PgType::StringAsText
                                | PgType::SqlxTypesChronoNaiveDateAsDate
                                | PgType::SqlxTypesChronoNaiveTimeAsTime
                                | PgType::SqlxTypesMacAddressMacAddressAsMacAddr
                                | PgType::SqlxTypesUuidUuidAsUuidV4InitializationByPg => &quote::quote! {#field_type_handle::default()},
                                PgType::SqlxTypesUuidUuidAsUuidInitializationByClient => &quote::quote! {#identifier_inner_type_token_stream::default()},
                                PgType::SqlxPgTypesPgMoneyAsMoney => &quote::quote! {#inner_type_standard_non_null_token_stream(#core_default)},
                                PgType::StdVecVecU8AsBytea => &quote::quote! {vec![#core_default]},
                                PgType::SqlxTypesTimeTimeAsTime => &generate_sqlx_types_time_time_from_hms_micro_unwrap_token_stream(&quote::quote! {0,0,0,0}),
                                PgType::SqlxPgTypesPgIntervalAsInterval => &quote::quote! {#inner_type_standard_non_null_token_stream {
                                    #months_snake_case: #core_default,
                                    #days_snake_case: #core_default,
                                    #microseconds_snake_case: #core_default
                                }},
                                PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp => &generate_sqlx_types_chrono_naive_date_time_new_token_stream(&quote::quote! {
                                    #sqlx_types_chrono_naive_date_as_non_null_date_origin_as_default_some_one_element_call_token_stream.0,
                                    #sqlx_types_chrono_naive_time_as_non_null_time_origin_as_default_some_one_element_call_token_stream.0,
                                }),
                                PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => &generate_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_from_naive_utc_and_offset_token_stream(&generate_sqlx_types_chrono_naive_date_time_new_token_stream(&quote::quote! {
                                    #sqlx_types_chrono_naive_date_as_non_null_date_origin_as_default_some_one_element_call_token_stream.0,
                                    #sqlx_types_chrono_naive_time_as_non_null_time_origin_as_default_some_one_element_call_token_stream.0,
                                })),
                                PgType::SqlxTypesIpnetworkIpNetworkAsInet => &quote::quote! {
                                    sqlx::types::ipnetwork::IpNetwork::V4(sqlx::types::ipnetwork::Ipv4Network::#new_snake_case(core::net::Ipv4Addr::UNSPECIFIED, #core_default).expect("9e9c9b57"))
                                },
                                PgType::SqlxPgTypesPgRangeI32AsInt4Range | PgType::SqlxPgTypesPgRangeI64AsInt8Range => &pg_range_int_default_initialization_token_stream,
                                PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => &generate_sqlx_pg_types_pg_range_default_some_one_element_token_stream(&sqlx_types_chrono_naive_date_as_non_null_date_origin_upper_camel_case),
                                PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => &generate_sqlx_pg_types_pg_range_default_some_one_element_token_stream(&sqlx_types_chrono_naive_date_time_as_non_null_timestamp_origin_upper_camel_case),
                                PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => &generate_sqlx_pg_types_pg_range_default_some_one_element_token_stream(&sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_non_null_timestamptz_origin_upper_camel_case),
                            };
                            quote::quote! {#initialization_token_stream}
                        }
                        pg_crud_macros_common::IsNullable::True => some_default_some_one_element_call_token_stream,
                    },
                };
                quote::quote! {Self(#ts)}
            });
            let impl_sqlx_type_and_encode_for_identifier_origin_token_stream = pg_crud_macros_common::generate_impl_sqlx_type_and_encode_for_identifier_token_stream(&identifier_origin_upper_camel_case, &field_type_handle, &sqlx_encode_self_dot_zero_token_stream);
            let impl_sqlx_decode_sqlx_pg_for_identifier_origin_token_stream = pg_crud_macros_common::generate_impl_sqlx_decode_sqlx_pg_for_identifier_token_stream(&identifier_origin_upper_camel_case, &field_type_handle, &{
                let scopes_v_token_stream = quote::quote! {(v)};
                let ok_self_scopes_v_token_stream = quote::quote! {Ok(Self #scopes_v_token_stream)};
                match &pg_type_pattern {
                    PgTypePattern::Standard => match &is_nullable {
                        pg_crud_macros_common::IsNullable::False => match &pg_type {
                            PgType::I16AsInt2
                            | PgType::I32AsInt4
                            | PgType::I64AsInt8
                            | PgType::F32AsFloat4
                            | PgType::I16AsSmallSerialInitializationByPg
                            | PgType::I32AsSerialInitializationByPg
                            | PgType::I64AsBigSerialInitializationByPg
                            | PgType::SqlxPgTypesPgMoneyAsMoney
                            | PgType::BoolAsBool
                            | PgType::StringAsText
                            | PgType::StdVecVecU8AsBytea
                            | PgType::SqlxTypesChronoNaiveTimeAsTime
                            | PgType::SqlxTypesTimeTimeAsTime
                            | PgType::SqlxPgTypesPgIntervalAsInterval
                            | PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp
                            | PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz
                            | PgType::SqlxTypesUuidUuidAsUuidV4InitializationByPg
                            | PgType::SqlxTypesUuidUuidAsUuidInitializationByClient
                            | PgType::SqlxTypesIpnetworkIpNetworkAsInet
                            | PgType::SqlxTypesMacAddressMacAddressAsMacAddr
                            | PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange
                            | PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange
                            | PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => ok_self_scopes_v_token_stream,
                            PgType::F64AsFloat8 | PgType::SqlxTypesChronoNaiveDateAsDate | PgType::SqlxPgTypesPgRangeI32AsInt4Range | PgType::SqlxPgTypesPgRangeI64AsInt8Range => quote::quote! {
                                match Self::#try_new_snake_case #scopes_v_token_stream {
                                    Ok(v_93eb5329) => Ok(v_93eb5329),
                                    Err(error) => Err(Box::#new_snake_case(error)),
                                }
                            },
                        },
                        pg_crud_macros_common::IsNullable::True => ok_self_scopes_v_token_stream,
                    },
                }
            });
            let maybe_impl_from_identifier_read_for_identifier_origin_token_stream = match &is_non_null_standard_can_be_primary_key {
                IsNonNullStandardCanBePrimaryKey::False => proc_macro2::TokenStream::new(),
                IsNonNullStandardCanBePrimaryKey::True => macros_helpers::generate_impl_from_token_stream::generate_impl_from_token_stream(&identifier_standard_non_null_read_upper_camel_case, &identifier_origin_upper_camel_case, &{
                    let identifier_standard_non_null_as_crate_pg_type_token_stream = generate_as_pg_type_token_stream(&identifier_standard_non_null_upper_camel_case);
                    quote::quote! {Self::#new_snake_case(#identifier_standard_non_null_as_crate_pg_type_token_stream::into_inner(#v_snake_case))}
                }).into(),
            };
            let impl_as_ref_and_borrow_for_identifier_origin_token_stream =
                generate_impl_wrapper_traits_token_stream(&identifier_origin_upper_camel_case, &field_type_handle, ShouldImplFrom::False);
            quote::quote! {
                #identifier_origin_wire_token_stream
                #identifier_origin_token_stream
                impl utoipa::PartialSchema for #identifier_origin_upper_camel_case {
                    fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
                        (#open_api_schema_token_stream).into()
                    }
                }
                impl utoipa::ToSchema for #identifier_origin_upper_camel_case {}
                #maybe_pub_enum_identifier_standard_non_null_origin_try_new_error_token_stream
                #maybe_pub_enum_identifier_standard_non_null_origin_try_new_for_de_error_token_stream
                #impl_identifier_origin_token_stream
                #impl_from_identifier_origin_for_identifier_inner_type_token_stream
                #maybe_impl_is_string_empty_for_identifier_origin_token_stream
                #maybe_impl_ser_for_identifier_standard_non_null_origin_token_stream
                #maybe_impl_de_for_identifier_standard_non_null_origin_token_stream
                #md_de_from_for_identifier_stndrt_non_null_origin_token_stream
                #md_de_try_from_for_identifier_stndrt_non_null_origin_token_stream
                #impl_display_for_identifier_origin_token_stream
                #impl_location_lib_to_err_string_for_identifier_origin_token_stream
                #impl_default_some_one_element_for_identifier_origin_token_stream
                #impl_sqlx_type_and_encode_for_identifier_origin_token_stream
                #impl_sqlx_decode_sqlx_pg_for_identifier_origin_token_stream
                #maybe_impl_from_identifier_read_for_identifier_origin_token_stream
                #impl_as_ref_and_borrow_for_identifier_origin_token_stream
            }
        };
        let generate_pub_struct_tokens_token_stream = |identifier_token_stream_parameter: &dyn quote::ToTokens, ts: &dyn quote::ToTokens, derive_default| {
            macros_helpers::derive_token_stream_builder::DTokenStreamBuilder::new()
                .make_pub()
                .d_debug()
                .d_default_if(derive_default)
                .d_clone()
                .d_copy()
                .d_partial_eq()
                .d_eq()
                .d_std_hash_hash()
                .d_serde_serialize()
                .d_serde_deserialize()
                .d_utoipa_to_schema()
                .build_struct(
                    &proc_macro2::TokenStream::new(),
                    &identifier_token_stream_parameter,
                    &proc_macro2::TokenStream::new(),
                    &ts
                )
        };
        let identifier_origin_struct_token_stream = quote::quote! {(#identifier_origin_upper_camel_case);};
        let self_default_some_one_element_call_token_stream = quote::quote! {Self(#pg_crud_common_default_some_one_element_call)};
        let ok_self_v_token_stream = quote::quote! {Ok(Self(v))};
        let identifier_table_type_upper_camel_case = naming::parameter::SelfTableTypeUpperCamelCase::from_tokens(&identifier);
        let identifier_table_type_token_stream = {
            let identifier_table_type_token_stream = macros_helpers::derive_token_stream_builder::DTokenStreamBuilder::new()
                .make_pub()
                .d_debug()
                .d_clone()
                .d_copy_if(derive_copy)
                .d_partial_eq()
                .d_partial_ord_if(d_partial_ord)
                .d_serde_serialize()
                .d_serde_deserialize()
                .d_utoipa_to_schema()
                .build_struct(
                    &proc_macro2::TokenStream::new(),
                    &identifier_table_type_upper_camel_case,
                    &proc_macro2::TokenStream::new(),
                    &identifier_origin_struct_token_stream
                );
            let impl_identifier_table_type_token_stream = generate_pub_const_new_or_pub_try_new_token_stream(&identifier_table_type_upper_camel_case);
            let impl_default_some_one_element_for_identifier_table_type_token_stream =
                pg_crud_macros_common::generate_impl_pg_crud_common_default_some_one_element_token_stream(&identifier_table_type_upper_camel_case, &self_default_some_one_element_call_token_stream);
            let impl_sqlx_type_and_encode_for_identifier_table_type_token_stream = pg_crud_macros_common::generate_impl_sqlx_type_and_encode_for_identifier_token_stream(&identifier_table_type_upper_camel_case, &identifier_origin_upper_camel_case, &sqlx_encode_self_dot_zero_token_stream);
            let impl_sqlx_decode_sqlx_pg_for_identifier_table_type_token_stream = pg_crud_macros_common::generate_impl_sqlx_decode_sqlx_pg_for_identifier_token_stream(&identifier_table_type_upper_camel_case, &identifier_origin_upper_camel_case, &ok_self_v_token_stream);
            //todo rewrite as dependency of PgType trait?
            let impl_pg_type_eq_operator_for_identifier_table_type_token_stream = pg_crud_macros_common::impl_pg_type_eq_operator_for_identifier_token_stream(
                &import,
                &identifier_table_type_upper_camel_case,
                //todo
                &{
                    let eq_token_stream = pg_crud_macros_common::EqOperatorHandle::Eq.to_tokens_path(&import);
                    let is_null_token_stream = pg_crud_macros_common::EqOperatorHandle::IsNull.to_tokens_path(&import);
                    let nullable_eq_operator_token_stream = quote::quote! {
                        if self.0.0.is_some() {
                            #eq_token_stream
                        }
                        else {
                            #is_null_token_stream
                        }
                    };
                    match &pg_type_pattern {
                        PgTypePattern::Standard => match &is_nullable {
                            pg_crud_macros_common::IsNullable::False => eq_token_stream,
                            pg_crud_macros_common::IsNullable::True => macros_helpers::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream::from(nullable_eq_operator_token_stream),
                        },
                    }
                },
            );
            let impl_as_ref_and_borrow_for_identifier_table_type_token_stream =
                generate_impl_wrapper_traits_token_stream(&identifier_table_type_upper_camel_case, &identifier_origin_upper_camel_case, ShouldImplFrom::True);
            quote::quote! {
                #identifier_table_type_token_stream
                #impl_identifier_table_type_token_stream
                #impl_default_some_one_element_for_identifier_table_type_token_stream
                #impl_sqlx_type_and_encode_for_identifier_table_type_token_stream
                #impl_sqlx_decode_sqlx_pg_for_identifier_table_type_token_stream
                #impl_pg_type_eq_operator_for_identifier_table_type_token_stream
                #impl_as_ref_and_borrow_for_identifier_table_type_token_stream
            }
        };
        let identifier_standard_non_null_table_type_upper_camel_case = naming::parameter::SelfTableTypeUpperCamelCase::from_tokens(&identifier_standard_non_null_upper_camel_case);
        let common_d_token_stream_builder = pg_crud_macros_common::token_stream_helpers::common_d_token_stream_builder()
            .d_copy_if(derive_copy);
        let identifier_create_upper_camel_case = naming::parameter::SelfCreateUpperCamelCase::from_tokens(&identifier);
        let identifier_create_token_stream = {
            let identifier_create_token_stream = match &can_be_primary_key {
                CanBePrimaryKey::False => common_d_token_stream_builder.d_utoipa_to_schema().build_struct(
                        &proc_macro2::TokenStream::new(),
                        &identifier_create_upper_camel_case,
                        &proc_macro2::TokenStream::new(),
                        &identifier_origin_struct_token_stream
                    ),
                CanBePrimaryKey::True => generate_pub_struct_tokens_token_stream(&identifier_create_upper_camel_case, &quote::quote! {(());}, macros_helpers::derive_token_stream_builder::DDefault::False),
            };
            let maybe_impl_identifier_create_token_stream = match &can_be_primary_key {
                CanBePrimaryKey::False => generate_pub_const_new_or_pub_try_new_token_stream(&identifier_create_upper_camel_case),
                CanBePrimaryKey::True => proc_macro2::TokenStream::new(),
            };
            let impl_default_some_one_element_for_identifier_create_token_stream = pg_crud_macros_common::generate_impl_pg_crud_common_default_some_one_element_token_stream(&identifier_create_upper_camel_case, &{
                let ts: &dyn quote::ToTokens = match &can_be_primary_key {
                    CanBePrimaryKey::False => &pg_crud_common_default_some_one_element_call,
                    CanBePrimaryKey::True => &quote::quote! {()},
                };
                quote::quote! {Self(#ts)}
            });
            let maybe_impl_sqlx_type_and_encode_for_identifier_create_token_stream = match &can_be_primary_key {
                CanBePrimaryKey::False => pg_crud_macros_common::generate_impl_sqlx_type_and_encode_for_identifier_token_stream(&identifier_create_upper_camel_case, &identifier_origin_upper_camel_case, &sqlx_encode_self_dot_zero_token_stream),
                CanBePrimaryKey::True => macros_helpers::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream::from(proc_macro2::TokenStream::new()),
            };
            let maybe_impl_as_ref_and_borrow_for_identifier_create_token_stream = match &can_be_primary_key {
                CanBePrimaryKey::False => generate_impl_wrapper_traits_token_stream(&identifier_create_upper_camel_case, &identifier_origin_upper_camel_case, ShouldImplFrom::True),
                CanBePrimaryKey::True => proc_macro2::TokenStream::new(),
            };
            quote::quote! {
                #identifier_create_token_stream
                #maybe_impl_identifier_create_token_stream
                #impl_default_some_one_element_for_identifier_create_token_stream
                #maybe_impl_sqlx_type_and_encode_for_identifier_create_token_stream
                #maybe_impl_as_ref_and_borrow_for_identifier_create_token_stream
            }
        };
        let identifier_select_upper_camel_case = naming::parameter::SelfSelectUpperCamelCase::from_tokens(&identifier);
        let identifier_select_token_stream = {
            let pub_struct_identifier_select_token_stream = generate_pub_struct_tokens_token_stream(
                &identifier_select_upper_camel_case,
                &quote::quote! {;},
                macros_helpers::derive_token_stream_builder::DDefault::True,
            );
            let (impl_default_some_one_element_for_identifier_select_token_stream, impl_default_some_one_element_max_page_size_for_identifier_select_token_stream) = {
                (
                    pg_crud_macros_common::generate_impl_pg_crud_common_default_some_one_element_token_stream(&identifier_select_upper_camel_case, &quote::quote! {Self}),
                    pg_crud_macros_common::generate_impl_pg_crud_common_default_some_one_element_max_page_size_token_stream(&identifier_select_upper_camel_case, &quote::quote! {Self}),
                )
            };
            quote::quote! {
                #pub_struct_identifier_select_token_stream
                #impl_default_some_one_element_for_identifier_select_token_stream
                #impl_default_some_one_element_max_page_size_for_identifier_select_token_stream
            }
        };
        let identifier_read_upper_camel_case = naming::parameter::SelfReadUpperCamelCase::from_tokens(&identifier);
        let identifier_where_upper_camel_case = naming::parameter::SelfWhereUpperCamelCase::from_tokens(&identifier);
        let (identifier_where_token_stream, frontend_filter_contracts_token_stream) = {
            let pg_type_filters = {
                fn generate_flts_with<T>(
                    base: Vec<pg_crud_macros_common::filters::PgTypeFilter>,
                    extra: T,
                ) -> Vec<pg_crud_macros_common::filters::PgTypeFilter>
                where
                    T: IntoIterator<Item = pg_crud_macros_common::filters::PgTypeFilter>,
                {
                    let mut vec = base;
                    vec.extend(extra);
                    vec
                }
                let generate_common_pg_type_filters = || {
                    vec![pg_crud_macros_common::filters::PgTypeFilter::Eq {
                        identifier: macros_helpers::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream::from(quote::quote! {#identifier_table_type_upper_camel_case}),
                    }]
                };
                let generate_greater_than_filter = || pg_crud_macros_common::filters::PgTypeFilter::GreaterThan {
                    identifier: macros_helpers::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream::from(quote::quote! {#identifier_standard_non_null_table_type_upper_camel_case}),
                };
                let generate_between_filter = || pg_crud_macros_common::filters::PgTypeFilter::Between {
                    identifier: macros_helpers::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream::from(quote::quote! {#identifier_standard_non_null_table_type_upper_camel_case}),
                };
                let generate_in_filter = || pg_crud_macros_common::filters::PgTypeFilter::In {
                    identifier: macros_helpers::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream::from(quote::quote! {#identifier_table_type_upper_camel_case}),
                };
                let generate_before_filter = || pg_crud_macros_common::filters::PgTypeFilter::Before {
                    identifier: macros_helpers::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream::from(quote::quote! {#identifier_standard_non_null_table_type_upper_camel_case}),
                };
                match &pg_type_pattern {
                    PgTypePattern::Standard => {
                        let generate_common_standard_pg_type_number_filters = || generate_flts_with(
                            generate_common_pg_type_filters(),
                            [generate_greater_than_filter(), generate_between_filter(), generate_in_filter()],
                        );
                        let generate_ranges_common_filter_vec = || {
                            let generate_range_identifier_token_stream = || {
                                macros_helpers::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream::from(
                                    quote::quote! {#identifier_standard_non_null_table_type_upper_camel_case},
                                )
                            };
                            generate_flts_with(generate_common_pg_type_filters(), [
                                pg_crud_macros_common::filters::PgTypeFilter::FindRangesWithinGivenRange { identifier: generate_range_identifier_token_stream() },
                                pg_crud_macros_common::filters::PgTypeFilter::FindRangesThatFullyContainTheGivenRange { identifier: generate_range_identifier_token_stream() },
                                pg_crud_macros_common::filters::PgTypeFilter::StrictlyToLeftOfRange { identifier: generate_range_identifier_token_stream() },
                                pg_crud_macros_common::filters::PgTypeFilter::StrictlyToRightOfRange { identifier: generate_range_identifier_token_stream() },
                                pg_crud_macros_common::filters::PgTypeFilter::IncludedLowerBound { identifier: generate_range_identifier_token_stream() },
                                pg_crud_macros_common::filters::PgTypeFilter::ExcludedUpperBound { identifier: generate_range_identifier_token_stream() },
                                pg_crud_macros_common::filters::PgTypeFilter::GreaterThanIncludedLowerBound { identifier: generate_range_identifier_token_stream() },
                                pg_crud_macros_common::filters::PgTypeFilter::GreaterThanExcludedUpperBound { identifier: generate_range_identifier_token_stream() },
                                pg_crud_macros_common::filters::PgTypeFilter::OverlapWithRange { identifier: generate_range_identifier_token_stream() },
                                pg_crud_macros_common::filters::PgTypeFilter::AdjacentWithRange { identifier: generate_range_identifier_token_stream() },
                                pg_crud_macros_common::filters::PgTypeFilter::RangeLen,
                            ])
                        };
                        match crate::filter::filter_kind(pg_type.spec()) {
                            FilterKind::Number => generate_common_standard_pg_type_number_filters(),
                            FilterKind::Money | FilterKind::Uuid | FilterKind::Bool => generate_flts_with(generate_common_pg_type_filters(), [generate_in_filter()]),
                            FilterKind::Bytes => generate_flts_with(generate_common_pg_type_filters(), [pg_crud_macros_common::filters::PgTypeFilter::EqToEncodedStringRepresentation]),
                            FilterKind::Time => generate_flts_with(generate_common_pg_type_filters(), [generate_greater_than_filter(), generate_between_filter(), pg_crud_macros_common::filters::PgTypeFilter::CurrentTime, pg_crud_macros_common::filters::PgTypeFilter::GreaterThanCurrentTime]),
                            FilterKind::Date => generate_flts_with(generate_common_pg_type_filters(), [generate_greater_than_filter(), generate_between_filter(), pg_crud_macros_common::filters::PgTypeFilter::CurrentDate, pg_crud_macros_common::filters::PgTypeFilter::GreaterThanCurrentDate]),
                            FilterKind::Timestamp => generate_flts_with(generate_common_pg_type_filters(), [generate_greater_than_filter(), generate_between_filter(), pg_crud_macros_common::filters::PgTypeFilter::CurrentTimestamp, pg_crud_macros_common::filters::PgTypeFilter::GreaterThanCurrentTimestamp]),
                            FilterKind::TimestampTz => generate_flts_with(generate_common_pg_type_filters(), [generate_before_filter(), generate_between_filter()]),
                            FilterKind::String => generate_flts_with(generate_common_pg_type_filters(), [pg_crud_macros_common::filters::PgTypeFilter::Regex]),
                            FilterKind::IntervalOrInet => generate_common_pg_type_filters(),
                            FilterKind::Mac => generate_flts_with(generate_common_pg_type_filters(), [generate_greater_than_filter(), pg_crud_macros_common::filters::PgTypeFilter::Regex]),
                            FilterKind::Range => generate_ranges_common_filter_vec(),
                        }
                    }
                }
            };
            let frontend_filter_contracts_token_stream = pg_type_filters.iter().map(|filter| {
                let operation = match filter {
                    pg_crud_macros_common::filters::PgTypeFilter::Eq { .. } => quote::quote! {Eq},
                    pg_crud_macros_common::filters::PgTypeFilter::GreaterThan { .. } => quote::quote! {GreaterThan},
                    pg_crud_macros_common::filters::PgTypeFilter::Between { .. } => quote::quote! {Between},
                    pg_crud_macros_common::filters::PgTypeFilter::In { .. } => quote::quote! {In},
                    pg_crud_macros_common::filters::PgTypeFilter::Regex => quote::quote! {Regex},
                    pg_crud_macros_common::filters::PgTypeFilter::Before { .. } => quote::quote! {Before},
                    pg_crud_macros_common::filters::PgTypeFilter::CurrentDate => quote::quote! {CurrentDate},
                    pg_crud_macros_common::filters::PgTypeFilter::GreaterThanCurrentDate => quote::quote! {GreaterThanCurrentDate},
                    pg_crud_macros_common::filters::PgTypeFilter::CurrentTimestamp => quote::quote! {CurrentTimestamp},
                    pg_crud_macros_common::filters::PgTypeFilter::GreaterThanCurrentTimestamp => quote::quote! {GreaterThanCurrentTimestamp},
                    pg_crud_macros_common::filters::PgTypeFilter::CurrentTime => quote::quote! {CurrentTime},
                    pg_crud_macros_common::filters::PgTypeFilter::GreaterThanCurrentTime => quote::quote! {GreaterThanCurrentTime},
                    pg_crud_macros_common::filters::PgTypeFilter::EqToEncodedStringRepresentation => quote::quote! {EqToEncodedStringRepresentation},
                    pg_crud_macros_common::filters::PgTypeFilter::FindRangesWithinGivenRange { .. } => quote::quote! {FindRangesWithinGivenRange},
                    pg_crud_macros_common::filters::PgTypeFilter::FindRangesThatFullyContainTheGivenRange { .. } => quote::quote! {FindRangesThatFullyContainTheGivenRange},
                    pg_crud_macros_common::filters::PgTypeFilter::StrictlyToLeftOfRange { .. } => quote::quote! {StrictlyToLeftOfRange},
                    pg_crud_macros_common::filters::PgTypeFilter::StrictlyToRightOfRange { .. } => quote::quote! {StrictlyToRightOfRange},
                    pg_crud_macros_common::filters::PgTypeFilter::IncludedLowerBound { .. } => quote::quote! {IncludedLowerBound},
                    pg_crud_macros_common::filters::PgTypeFilter::ExcludedUpperBound { .. } => quote::quote! {ExcludedUpperBound},
                    pg_crud_macros_common::filters::PgTypeFilter::GreaterThanIncludedLowerBound { .. } => quote::quote! {GreaterThanIncludedLowerBound},
                    pg_crud_macros_common::filters::PgTypeFilter::GreaterThanExcludedUpperBound { .. } => quote::quote! {GreaterThanExcludedUpperBound},
                    pg_crud_macros_common::filters::PgTypeFilter::OverlapWithRange { .. } => quote::quote! {OverlapWithRange},
                    pg_crud_macros_common::filters::PgTypeFilter::AdjacentWithRange { .. } => quote::quote! {AdjacentWithRange},
                    pg_crud_macros_common::filters::PgTypeFilter::RangeLen => quote::quote! {RangeLen},
                };
                quote::quote! {frontend_contract::FilterOperation::#operation}
            });
            (
            pg_crud_macros_common::generate_pg_type_where_token_stream(
                &allow_clippy_arbitrary_src_item_ordering,
                pg_type_filters.as_slice(),
                &identifier,
                &pg_crud_macros_common::ShouldDeriveUtoipaToSchema::True,
                &pg_crud_macros_common::ShouldDSchemarsJsonSchema::False,
                &pg_crud_macros_common::IsQueryBindMut::False,
            ),
            quote::quote! {#(#frontend_filter_contracts_token_stream),*},
            )
        };
        let identifier_read_token_stream = {
            let identifier_read_token_stream = {
                let (
                    derive_eq,
                    derive_partial_ord,
                    derive_ord
                ) = match &is_non_null_standard_can_be_primary_key {
                    IsNonNullStandardCanBePrimaryKey::False => (
                        macros_helpers::derive_token_stream_builder::DEq::False,
                        macros_helpers::derive_token_stream_builder::DPartialOrd::False,
                        macros_helpers::derive_token_stream_builder::DOrd::False
                    ),
                    IsNonNullStandardCanBePrimaryKey::True => (
                        macros_helpers::derive_token_stream_builder::DEq::True,
                        macros_helpers::derive_token_stream_builder::DPartialOrd::True,
                        macros_helpers::derive_token_stream_builder::DOrd::True
                    ),
                };
                macros_helpers::derive_token_stream_builder::DTokenStreamBuilder::new()
                    .make_pub()
                    .d_debug()
                    .d_clone()
                    .d_copy_if(derive_copy)
                    .d_partial_eq()
                    .d_eq_if(derive_eq)
                    .d_partial_ord_if(derive_partial_ord)
                    .d_ord_if(derive_ord)
                    .d_serde_serialize()
                    .d_serde_deserialize()
                    .d_utoipa_to_schema()
                    .build_struct(
                        &proc_macro2::TokenStream::new(),
                        &identifier_read_upper_camel_case,
                        &proc_macro2::TokenStream::new(),
                        &identifier_origin_struct_token_stream
                    )
            };
            let impl_identifier_read_token_stream = generate_pub_const_new_or_pub_try_new_token_stream(&identifier_read_upper_camel_case);
            let impl_location_lib_to_err_string_for_identifier_read_token_stream = pg_crud_macros_common::generate_impl_to_err_string_no_generics_token_stream(&identifier_read_upper_camel_case, &quote::quote! {self.0.to_string()});
            let impl_crate_default_some_one_element_for_identifier_read_token_stream =
                pg_crud_macros_common::generate_impl_pg_crud_common_default_some_one_element_token_stream(&identifier_read_upper_camel_case, &self_default_some_one_element_call_token_stream);
            let impl_sqlx_type_and_encode_for_identifier_read_token_stream = pg_crud_macros_common::generate_impl_sqlx_type_and_encode_for_identifier_token_stream(&identifier_read_upper_camel_case, &identifier_origin_upper_camel_case, &sqlx_encode_self_dot_zero_token_stream);
            let impl_sqlx_decode_sqlx_pg_for_identifier_read_token_stream = pg_crud_macros_common::generate_impl_sqlx_decode_sqlx_pg_for_identifier_token_stream(
                &identifier_read_upper_camel_case,
                &identifier_origin_upper_camel_case,
                &ok_self_v_token_stream
            );
            let maybe_impl_pg_type_where_filter_for_identifier_read_if_can_be_primary_key_token_stream = if matches!(&is_non_null_standard_can_be_primary_key, IsNonNullStandardCanBePrimaryKey::True) {
                pg_crud_macros_common::impl_pg_type_where_filter_for_identifier_token_stream(
                    &quote::quote! {<'lt>},
                    &identifier_standard_non_null_read_upper_camel_case,
                    &proc_macro2::TokenStream::new(),
                    &pg_crud_macros_common::IncrementParameterUndrscr::False,
                    &pg_crud_macros_common::ColumnParameterUndrscr::False,
                    &pg_crud_macros_common::AddOperatorUndrscr::True,
                    &quote::quote! {
                        match #import::increment_checked_add_one_returning_increment(#increment_snake_case) {
                            Ok(v_8da76391) => {
                                let mut query_part_94ddf524 = String::with_capacity(32);
                                if std::fmt::Write::write_fmt(
                                    &mut query_part_94ddf524,
                                    format_args!("({column} = ${v_8da76391})"),
                                )
                                .is_err()
                                {
                                    return Err(#import::QueryPartError::WriteIntoBuffer { location: location_macros::location!() });
                                }
                                Ok(#import::QueryPartFragment::try_from(query_part_94ddf524).unwrap_or_else(#import::QueryPartFragment::from))
                            },
                            Err(error) => Err(error)
                        }
                    },
                    &pg_crud_macros_common::IsQueryBindMut::True,
                    &generate_typical_pg_query_query_bind_token_stream(&self_snake_case),
                    &import,
                )
            } else {
                macros_helpers::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream::from(proc_macro2::TokenStream::new())
            };
            let impl_as_ref_and_borrow_for_identifier_read_token_stream =
                generate_impl_wrapper_traits_token_stream(&identifier_read_upper_camel_case, &identifier_origin_upper_camel_case, ShouldImplFrom::True);
            quote::quote! {
                #identifier_read_token_stream
                #impl_identifier_read_token_stream
                #impl_location_lib_to_err_string_for_identifier_read_token_stream
                #impl_crate_default_some_one_element_for_identifier_read_token_stream
                #impl_sqlx_type_and_encode_for_identifier_read_token_stream
                #impl_sqlx_decode_sqlx_pg_for_identifier_read_token_stream
                #maybe_impl_pg_type_where_filter_for_identifier_read_if_can_be_primary_key_token_stream
                #impl_as_ref_and_borrow_for_identifier_read_token_stream
            }
        };
        let identifier_read_ids_upper_camel_case = naming::parameter::SelfReadIdsUpperCamelCase::from_tokens(&identifier);
        let identifier_read_ids_token_stream = if matches!(&is_non_null_standard_can_be_primary_key, IsNonNullStandardCanBePrimaryKey::True) {
            let identifier_read_ids_token_stream = common_d_token_stream_builder.d_utoipa_to_schema().build_struct(
                    &proc_macro2::TokenStream::new(),
                    &identifier_read_ids_upper_camel_case,
                    &proc_macro2::TokenStream::new(),
                    &quote::quote! {(#identifier_read_upper_camel_case);},
                );
            let impl_sqlx_decode_sqlx_pg_for_identifier_read_ids_token_stream = pg_crud_macros_common::generate_impl_sqlx_decode_sqlx_pg_for_identifier_token_stream(
                &identifier_read_ids_upper_camel_case,
                &identifier_read_upper_camel_case,
                &ok_self_v_token_stream
            );
            let impl_sqlx_type_for_identifier_read_ids_token_stream = pg_crud_macros_common::generate_impl_sqlx_type_for_identifier_token_stream(&identifier_read_ids_upper_camel_case, &identifier_read_upper_camel_case);
            let impl_as_ref_and_borrow_for_identifier_read_ids_token_stream =
                generate_impl_wrapper_traits_token_stream(&identifier_read_ids_upper_camel_case, &identifier_read_upper_camel_case, ShouldImplFrom::True);
            quote::quote! {
                #identifier_read_ids_token_stream
                #impl_sqlx_decode_sqlx_pg_for_identifier_read_ids_token_stream
                #impl_sqlx_type_for_identifier_read_ids_token_stream
                #impl_as_ref_and_borrow_for_identifier_read_ids_token_stream
            }
        } else {
            proc_macro2::TokenStream::new()
        };
        let identifier_read_inner_upper_camel_case = naming::parameter::SelfReadInnerUpperCamelCase::from_tokens(&identifier);
        let identifier_read_inner_token_stream = quote::quote! {
            pub type #identifier_read_inner_upper_camel_case = #identifier_inner_type_token_stream;
        };
        let identifier_update_token_stream = {
            let identifier_update_token_stream = common_d_token_stream_builder
                .d_utoipa_to_schema()
                .d_eq_if(match &is_non_null_standard_can_be_primary_key {
                    IsNonNullStandardCanBePrimaryKey::False => macros_helpers::derive_token_stream_builder::DEq::False,
                    IsNonNullStandardCanBePrimaryKey::True => macros_helpers::derive_token_stream_builder::DEq::True,
                })
                .d_std_hash_hash_if(match &is_non_null_standard_can_be_primary_key {
                    IsNonNullStandardCanBePrimaryKey::False => {
                        macros_helpers::derive_token_stream_builder::DStdHashHash::False
                    }
                    IsNonNullStandardCanBePrimaryKey::True => {
                        macros_helpers::derive_token_stream_builder::DStdHashHash::True
                    }
                })
                .build_struct(
                        &proc_macro2::TokenStream::new(),
                        &identifier_update_upper_camel_case,
                        &proc_macro2::TokenStream::new(),
                        &identifier_origin_struct_token_stream
                    );
            let impl_identifier_update_token_stream = generate_pub_const_new_or_pub_try_new_token_stream(&identifier_update_upper_camel_case);
            let impl_default_some_one_element_for_identifier_update_token_stream =
                pg_crud_macros_common::generate_impl_pg_crud_common_default_some_one_element_token_stream(&identifier_update_upper_camel_case, &self_default_some_one_element_call_token_stream);
            let impl_location_lib_to_err_string_for_identifier_update_token_stream = pg_crud_macros_common::generate_impl_to_err_string_no_generics_token_stream(&identifier_update_upper_camel_case, &quote::quote! {self.0.#to_err_string_snake_case().into_inner()});
            let impl_as_ref_and_borrow_for_identifier_update_token_stream =
                generate_impl_wrapper_traits_token_stream(&identifier_update_upper_camel_case, &identifier_origin_upper_camel_case, ShouldImplFrom::True);
            quote::quote! {
                #identifier_update_token_stream
                #impl_identifier_update_token_stream
                #impl_default_some_one_element_for_identifier_update_token_stream
                #impl_location_lib_to_err_string_for_identifier_update_token_stream
                #impl_as_ref_and_borrow_for_identifier_update_token_stream
            }
        };
        let identifier_update_for_query_upper_camel_case = naming::parameter::SelfUpdateForQueryUpperCamelCase::from_tokens(&identifier);
        let identifier_update_for_query_token_stream = {
            let identifier_update_for_query_token_stream = common_d_token_stream_builder.d_utoipa_to_schema().build_struct(
                    &proc_macro2::TokenStream::new(),
                    &identifier_update_for_query_upper_camel_case,
                    &proc_macro2::TokenStream::new(),
                    &identifier_origin_struct_token_stream
                );
            let impl_sqlx_type_and_encode_for_identifier_update_for_query_token_stream = pg_crud_macros_common::generate_impl_sqlx_type_and_encode_for_identifier_token_stream(&identifier_update_for_query_upper_camel_case, &identifier_origin_upper_camel_case, &sqlx_encode_self_dot_zero_token_stream);
            let impl_from_identifier_update_for_identifier_update_for_query_token_stream = macros_helpers::generate_impl_from_token_stream::generate_impl_from_token_stream(&identifier_update_upper_camel_case, &identifier_update_for_query_upper_camel_case, &quote::quote! {Self(#v_snake_case.0)});
            let impl_as_ref_and_borrow_for_identifier_update_for_query_token_stream =
                generate_impl_wrapper_traits_token_stream(&identifier_update_for_query_upper_camel_case, &identifier_origin_upper_camel_case, ShouldImplFrom::True);
            quote::quote! {
                #identifier_update_for_query_token_stream
                #impl_sqlx_type_and_encode_for_identifier_update_for_query_token_stream
                #impl_from_identifier_update_for_identifier_update_for_query_token_stream
                #impl_as_ref_and_borrow_for_identifier_update_for_query_token_stream
            }
        };
        let impl_pg_type_for_identifier_token_stream = {
            let generate_ok_string_from_tokens_token_stream = |ts: &dyn quote::ToTokens| {
                quote::quote! {Ok(#import::QueryPartFragment::try_from(#string_token_stream::from(#ts)).unwrap_or_else(#import::QueryPartFragment::from))}
            };
            let ok_string_from_default_token_stream = generate_ok_string_from_tokens_token_stream(&quote::quote! {"default"});
            let ok_string_from_uuid_generate_v4_token_stream = generate_ok_string_from_tokens_token_stream(&quote::quote! {"uuid_generate_v4()"});
            let typical_query_part_token_stream = {
                let if_write_is_err_token_stream = macros_helpers::generate_if_write_is_err_token_stream::generate_if_write_is_err_token_stream(
                    &quote::quote! {accumulator_c7df00f5, "${v_ba581e0f}"},
                    &pg_crud_macros_common::generate_return_err_query_part_error_write_into_buffer_token_stream(import)
                );
                quote::quote! {
                    let mut accumulator_c7df00f5 = String::with_capacity(8);
                    match #import::increment_checked_add_one_returning_increment(#increment_snake_case) {
                        Ok(v_ba581e0f) => {
                            #if_write_is_err_token_stream
                        },
                        Err(error) => {
                            return Err(error);
                        }
                    }
                    Ok(#import::QueryPartFragment::try_from(accumulator_c7df00f5).unwrap_or_else(#import::QueryPartFragment::from))
                }
            };
            let ok_query_token_stream = quote::quote! {Ok(#query_snake_case)};
            let (query_part_create_token_stream, bind_v_to_query_create_token_stream): (&dyn quote::ToTokens, &dyn quote::ToTokens) = {
                let typical: (&dyn quote::ToTokens, &dyn quote::ToTokens) = { (&typical_query_part_token_stream, &typical_query_bind_token_stream) };
                let default_initialization_by_pg: (&dyn quote::ToTokens, &dyn quote::ToTokens) = (&ok_string_from_default_token_stream, &ok_query_token_stream);
                match &pg_type {
                    PgType::I16AsInt2
                    | PgType::I32AsInt4
                    | PgType::I64AsInt8
                    | PgType::F32AsFloat4
                    | PgType::F64AsFloat8
                    | PgType::SqlxPgTypesPgMoneyAsMoney
                    | PgType::BoolAsBool
                    | PgType::StringAsText
                    | PgType::StdVecVecU8AsBytea
                    | PgType::SqlxTypesChronoNaiveTimeAsTime
                    | PgType::SqlxTypesTimeTimeAsTime
                    | PgType::SqlxPgTypesPgIntervalAsInterval
                    | PgType::SqlxTypesChronoNaiveDateAsDate
                    | PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp
                    | PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz
                    | PgType::SqlxTypesUuidUuidAsUuidInitializationByClient
                    | PgType::SqlxTypesIpnetworkIpNetworkAsInet
                    | PgType::SqlxTypesMacAddressMacAddressAsMacAddr
                    | PgType::SqlxPgTypesPgRangeI32AsInt4Range
                    | PgType::SqlxPgTypesPgRangeI64AsInt8Range
                    | PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange
                    | PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange
                    | PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => typical,
                    PgType::I16AsSmallSerialInitializationByPg | PgType::I32AsSerialInitializationByPg | PgType::I64AsBigSerialInitializationByPg => default_initialization_by_pg,
                    PgType::SqlxTypesUuidUuidAsUuidV4InitializationByPg => (&ok_string_from_uuid_generate_v4_token_stream, &ok_query_token_stream),
                }
            };
            let select_only_ids_and_select_only_updated_ids_query_common_token_stream = {
                let format_token_stream = generate_quotes::dq_token_stream(&{
                    let column_comma = str_constants::COLUMN_ALT;
                    column_comma.to_owned()
                });
                quote::quote! {
                    let mut query_part_98c19394 = String::with_capacity(32);
                    if std::fmt::Write::write_fmt(&mut query_part_98c19394, format_args!(#format_token_stream)).is_err() {
                        return Err(#import::QueryPartError::WriteIntoBuffer { location: location_macros::location!() });
                    }
                    Ok(#import::QueryPartFragment::try_from(query_part_98c19394).unwrap_or_else(#import::QueryPartFragment::from))
                }
            };
            pg_crud_macros_common::generate_impl_pg_type_token_stream(
                &import,
                &identifier,
                &identifier_table_type_upper_camel_case,
                &match &can_be_primary_key {
                    CanBePrimaryKey::False => pg_crud_macros_common::IsPrimaryKeyUndrscr::True,
                    CanBePrimaryKey::True => pg_crud_macros_common::IsPrimaryKeyUndrscr::False,
                },
                &{
                    let pg_query_type = match &pg_type {
                        PgType::I16AsInt2 => str_constants::PG_CRUD_PG_INT2,
                        PgType::I32AsInt4 => str_constants::PG_CRUD_PG_INT4,
                        PgType::I64AsInt8 => str_constants::PG_CRUD_PG_INT8,
                        PgType::F32AsFloat4 => str_constants::PG_CRUD_PG_FLOAT4,
                        PgType::F64AsFloat8 => str_constants::PG_CRUD_PG_FLOAT8,
                        PgType::I16AsSmallSerialInitializationByPg => str_constants::PG_CRUD_PG_SMALLSERIAL,
                        PgType::I32AsSerialInitializationByPg => str_constants::PG_CRUD_PG_SERIAL,
                        PgType::I64AsBigSerialInitializationByPg => str_constants::PG_CRUD_PG_BIGSERIAL,
                        PgType::SqlxPgTypesPgMoneyAsMoney => str_constants::PG_CRUD_PG_MONEY,
                        PgType::BoolAsBool => str_constants::PG_CRUD_PG_BOOL,
                        PgType::StringAsText => str_constants::PG_CRUD_PG_TEXT,
                        PgType::StdVecVecU8AsBytea => str_constants::PG_CRUD_PG_BYTEA,
                        PgType::SqlxTypesChronoNaiveTimeAsTime | PgType::SqlxTypesTimeTimeAsTime => str_constants::PG_CRUD_PG_TIME,
                        PgType::SqlxPgTypesPgIntervalAsInterval => str_constants::PG_CRUD_PG_INTERVAL,
                        PgType::SqlxTypesChronoNaiveDateAsDate => str_constants::PG_CRUD_PG_DATE,
                        PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp => str_constants::PG_CRUD_PG_TIMESTAMP,
                        PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => str_constants::PG_CRUD_PG_TIMESTAMPTZ,
                        PgType::SqlxTypesUuidUuidAsUuidV4InitializationByPg | PgType::SqlxTypesUuidUuidAsUuidInitializationByClient => str_constants::PG_CRUD_PG_UUID,
                        PgType::SqlxTypesIpnetworkIpNetworkAsInet => str_constants::PG_CRUD_PG_INET,
                        PgType::SqlxTypesMacAddressMacAddressAsMacAddr => str_constants::PG_CRUD_PG_MACADDR,
                        PgType::SqlxPgTypesPgRangeI32AsInt4Range => str_constants::PG_CRUD_PG_INT4RANGE,
                        PgType::SqlxPgTypesPgRangeI64AsInt8Range => str_constants::PG_CRUD_PG_INT8RANGE,
                        PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => str_constants::PG_CRUD_PG_DATERANGE,
                        PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => str_constants::PG_CRUD_PG_TSRANGE,
                        PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => str_constants::PG_CRUD_PG_TSTZRANGE,
                    };
                    let maybe_primary_key_is_primary_key_token_stream = quote::quote! {pg_types_common::maybe_primary_key(is_primary_key)};
                    let column_pg_query_type = format!("{{column}} {pg_query_type}");
                    let column_pg_query_type_non_null = format!("{{column}} {pg_query_type} not null");
                    let space_extra_parameter = str_constants::TEXT_ALT_3;
                    match (&is_nullable, &can_be_primary_key) {
                        (pg_crud_macros_common::IsNullable::False, CanBePrimaryKey::False) => {
                            let format_token_stream = generate_quotes::dq_token_stream(&column_pg_query_type_non_null);
                            quote::quote! {
                                let mut query_part_f8ad7c79 = String::with_capacity(32);
                                if std::fmt::Write::write_fmt(&mut query_part_f8ad7c79, format_args!(#format_token_stream)).is_err() {
                                    return #import::QueryPartFragment::try_from(String::default()).unwrap_or_else(#import::QueryPartFragment::from);
                                }
                                #import::QueryPartFragment::try_from(query_part_f8ad7c79).unwrap_or_else(#import::QueryPartFragment::from)
                            }
                        }
                        (pg_crud_macros_common::IsNullable::False, CanBePrimaryKey::True) => {
                            let format_token_stream = generate_quotes::dq_token_stream(&format!("{column_pg_query_type_non_null}{space_extra_parameter}"));
                            quote::quote! {
                                let mut query_part_06cdb263 = String::with_capacity(48);
                                if std::fmt::Write::write_fmt(
                                    &mut query_part_06cdb263,
                                    format_args!(#format_token_stream, #maybe_primary_key_is_primary_key_token_stream),
                                )
                                .is_err()
                                {
                                    return #import::QueryPartFragment::try_from(String::default()).unwrap_or_else(#import::QueryPartFragment::from);
                                }
                                #import::QueryPartFragment::try_from(query_part_06cdb263).unwrap_or_else(#import::QueryPartFragment::from)
                            }
                        }
                        (pg_crud_macros_common::IsNullable::True, CanBePrimaryKey::False) => {
                            let format_token_stream = generate_quotes::dq_token_stream(&column_pg_query_type);
                            quote::quote! {
                                let mut query_part_277407be = String::with_capacity(32);
                                if std::fmt::Write::write_fmt(&mut query_part_277407be, format_args!(#format_token_stream)).is_err() {
                                    return #import::QueryPartFragment::try_from(String::default()).unwrap_or_else(#import::QueryPartFragment::from);
                                }
                                #import::QueryPartFragment::try_from(query_part_277407be).unwrap_or_else(#import::QueryPartFragment::from)
                            }
                        }
                        (pg_crud_macros_common::IsNullable::True, CanBePrimaryKey::True) => {
                            let format_token_stream = generate_quotes::dq_token_stream(&format!("{column_pg_query_type}{space_extra_parameter}"));
                            quote::quote! {
                                let mut query_part_3265d12f = String::with_capacity(48);
                                if std::fmt::Write::write_fmt(
                                    &mut query_part_3265d12f,
                                    format_args!(#format_token_stream, #maybe_primary_key_is_primary_key_token_stream),
                                )
                                .is_err()
                                {
                                    return #import::QueryPartFragment::try_from(String::default()).unwrap_or_else(#import::QueryPartFragment::from);
                                }
                                #import::QueryPartFragment::try_from(query_part_3265d12f).unwrap_or_else(#import::QueryPartFragment::from)
                            }
                        }
                    }
                },
                &identifier_create_upper_camel_case,
                &pg_crud_macros_common::CreateQueryPartValueUndrscr::True,
                &match &can_be_primary_key {
                    CanBePrimaryKey::False => pg_crud_macros_common::CreateQueryPartIncrementUndrscr::False,
                    CanBePrimaryKey::True => pg_crud_macros_common::CreateQueryPartIncrementUndrscr::True,
                },
                &query_part_create_token_stream,
                &match &can_be_primary_key {
                    CanBePrimaryKey::False => pg_crud_macros_common::CreateQueryBindValueUndrscr::False,
                    CanBePrimaryKey::True => pg_crud_macros_common::CreateQueryBindValueUndrscr::True,
                },
                &match &can_be_primary_key {
                    CanBePrimaryKey::False => pg_crud_macros_common::IsCreateQueryBindMut::True,
                    CanBePrimaryKey::True => pg_crud_macros_common::IsCreateQueryBindMut::False,
                },
                &bind_v_to_query_create_token_stream,
                &identifier_select_upper_camel_case,
                &pg_crud_macros_common::SelectQueryPartValueUndrscr::True,
                &{
                    let ts = quote::quote! {#import::QueryPartFragment::try_from(#column_snake_case.to_string()).unwrap_or_else(#import::QueryPartFragment::from)};
                    quote::quote! {Ok(#ts)}
                },
                &identifier_where_upper_camel_case,
                &identifier_read_upper_camel_case,
                &{
                    let generate_identifier_read_identifier_origin_token_stream = |ts: &dyn quote::ToTokens| {
                        quote::quote! {#identifier_read_upper_camel_case(#identifier_origin_upper_camel_case(#ts))}
                    };
                    match &pg_type_pattern {
                        PgTypePattern::Standard => match &is_nullable {
                            pg_crud_macros_common::IsNullable::False => {
                                Range::try_from(pg_type).as_ref().map_or_else(
                                    |()| quote::quote! {#v_snake_case},
                                    |range| {
                                        let generate_sqlx_pg_types_pg_range_token_stream = |start_token_stream: &dyn quote::ToTokens, end_token_stream: &dyn quote::ToTokens| {
                                            quote::quote! {
                                                sqlx::postgres::types::PgRange{
                                                    #start_snake_case: std::ops::Bound::#start_token_stream,
                                                    #end_snake_case: std::ops::Bound::#end_token_stream
                                                }
                                            }
                                        };
                                        let included_start_token_stream = quote::quote! {#included_upper_camel_case(#start_snake_case)};
                                        let excluded_end_token_stream = quote::quote! {#excluded_upper_camel_case(#end_snake_case)};
                                        let included_end_token_stream = quote::quote! {#included_upper_camel_case(#end_snake_case)};
                                        let excluded_start_token_stream = quote::quote! {#excluded_upper_camel_case(#start_snake_case)};
                                        let sqlx_pg_types_pg_range_excluded_excluded_token_stream = generate_sqlx_pg_types_pg_range_token_stream(&excluded_start_token_stream, &excluded_end_token_stream);
                                        let sqlx_pg_types_pg_range_excluded_included_token_stream = generate_sqlx_pg_types_pg_range_token_stream(&excluded_start_token_stream, &included_end_token_stream);
                                        let sqlx_pg_types_pg_range_included_unbounded_token_stream = generate_sqlx_pg_types_pg_range_token_stream(&included_start_token_stream, &unbounded_upper_camel_case);
                                        let sqlx_pg_types_pg_range_unbounded_excluded_token_stream = generate_sqlx_pg_types_pg_range_token_stream(&unbounded_upper_camel_case, &excluded_end_token_stream);
                                        let sqlx_pg_types_pg_range_included_excluded_token_stream = generate_sqlx_pg_types_pg_range_token_stream(&included_start_token_stream, &excluded_end_token_stream);
                                        let sqlx_pg_types_pg_range_unbounded_unbounded_token_stream = generate_sqlx_pg_types_pg_range_token_stream(&unbounded_upper_camel_case, &unbounded_upper_camel_case);
                                        let generate_range_match_token_stream = |
                                            included_included_token_stream: &dyn quote::ToTokens,
                                            included_excluded_token_stream: &dyn quote::ToTokens,
                                            included_unbounded_token_stream: &dyn quote::ToTokens,
                                            excluded_included_token_stream: &dyn quote::ToTokens,
                                            excluded_excluded_token_stream: &dyn quote::ToTokens,
                                            excluded_unbounded_token_stream: &dyn quote::ToTokens,
                                            unbounded_included_token_stream: &dyn quote::ToTokens,
                                            unbounded_excluded_token_stream: &dyn quote::ToTokens
                                        | {
                                            quote::quote! {
                                                #identifier_standard_non_null_read_upper_camel_case(#identifier_standard_non_null_origin_upper_camel_case(match (
                                                    #v_snake_case.0.0.#start_snake_case,
                                                    #v_snake_case.0.0.#end_snake_case
                                                ) {
                                                    (std::ops::Bound::#included_upper_camel_case(#start_snake_case), std::ops::Bound::#included_upper_camel_case(#end_snake_case)) => {
                                                        #included_included_token_stream
                                                    },
                                                    (std::ops::Bound::#included_upper_camel_case(#start_snake_case), std::ops::Bound::#excluded_upper_camel_case(#end_snake_case)) => {
                                                        #included_excluded_token_stream
                                                    },
                                                    (std::ops::Bound::#included_upper_camel_case(#start_snake_case), std::ops::Bound::#unbounded_upper_camel_case) => {
                                                        #included_unbounded_token_stream
                                                    },
                                                    (std::ops::Bound::#excluded_upper_camel_case(#start_snake_case), std::ops::Bound::#included_upper_camel_case(#end_snake_case)) => {
                                                        #excluded_included_token_stream
                                                    },
                                                    (std::ops::Bound::#excluded_upper_camel_case(#start_snake_case), std::ops::Bound::#excluded_upper_camel_case(#end_snake_case)) => {
                                                        #excluded_excluded_token_stream
                                                    },
                                                    (std::ops::Bound::#excluded_upper_camel_case(#start_snake_case), std::ops::Bound::#unbounded_upper_camel_case) => {
                                                        #excluded_unbounded_token_stream
                                                    },
                                                    (std::ops::Bound::#unbounded_upper_camel_case, std::ops::Bound::#included_upper_camel_case(#end_snake_case)) => {
                                                        #unbounded_included_token_stream
                                                    },
                                                    (std::ops::Bound::#unbounded_upper_camel_case, std::ops::Bound::#excluded_upper_camel_case(#end_snake_case)) => {
                                                        #unbounded_excluded_token_stream
                                                    },
                                                    (std::ops::Bound::#unbounded_upper_camel_case, std::ops::Bound::#unbounded_upper_camel_case) => #sqlx_pg_types_pg_range_unbounded_unbounded_token_stream,
                                                }))
                                            }
                                        };
                                        let generate_if_start_end_eq_token_stream = |true_token_stream: &dyn quote::ToTokens, false_token_stream: &dyn quote::ToTokens| {
                                            quote::quote! {
                                                if #start_snake_case == #end_snake_case {
                                                    #true_token_stream
                                                } else {
                                                    #false_token_stream
                                                }
                                            }
                                        };
                                        let if_eq_unbounded_unbounded_or_included_excluded_token_stream = generate_if_start_end_eq_token_stream(&sqlx_pg_types_pg_range_unbounded_unbounded_token_stream, &sqlx_pg_types_pg_range_included_excluded_token_stream);
                                        let int_range_normalize_token_stream = {
                                            let (
                                                included_start_checked_add_token_stream,
                                                excluded_end_checked_add_token_stream
                                            ) = {
                                                let generate_token_stream = |first_token_stream: &dyn quote::ToTokens, second_token_stream: &dyn quote::ToTokens| {
                                                    quote::quote! {#first_token_stream(#second_token_stream.checked_add(1).expect("0ec0992f"))}
                                                };
                                                (
                                                    generate_token_stream(&included_upper_camel_case, &start_snake_case),
                                                    generate_token_stream(&excluded_upper_camel_case, &end_snake_case)
                                                )
                                            };
                                            let included_excluded_checked_add_token_stream = generate_sqlx_pg_types_pg_range_token_stream(&included_start_token_stream, &excluded_end_checked_add_token_stream);
                                            let included_unbounded_token_stream = generate_sqlx_pg_types_pg_range_token_stream(&included_start_token_stream, &unbounded_upper_camel_case);
                                            let included_checked_add_excluded_checked_add_token_stream = generate_sqlx_pg_types_pg_range_token_stream(&included_start_checked_add_token_stream, &excluded_end_checked_add_token_stream);
                                            let included_checked_add_excluded_token_stream = generate_sqlx_pg_types_pg_range_token_stream(&included_start_checked_add_token_stream, &excluded_end_token_stream);
                                            let included_checked_add_unbounded_token_stream = generate_sqlx_pg_types_pg_range_token_stream(&included_start_checked_add_token_stream, &unbounded_upper_camel_case);
                                            let unbounded_excluded_checked_add_token_stream = generate_sqlx_pg_types_pg_range_token_stream(&unbounded_upper_camel_case, &excluded_end_checked_add_token_stream);
                                            let unbounded_excluded_token_stream = generate_sqlx_pg_types_pg_range_token_stream(&unbounded_upper_camel_case, &excluded_end_token_stream);
                                            generate_range_match_token_stream(
                                                &included_excluded_checked_add_token_stream,
                                                &generate_if_start_end_eq_token_stream(&sqlx_pg_types_pg_range_unbounded_unbounded_token_stream, &sqlx_pg_types_pg_range_included_excluded_token_stream),
                                                &included_unbounded_token_stream,
                                                &generate_if_start_end_eq_token_stream(&sqlx_pg_types_pg_range_unbounded_unbounded_token_stream, &included_checked_add_excluded_checked_add_token_stream),
                                                &generate_if_start_end_eq_token_stream(&sqlx_pg_types_pg_range_unbounded_unbounded_token_stream, &included_checked_add_excluded_token_stream),
                                                &included_checked_add_unbounded_token_stream,
                                                &unbounded_excluded_checked_add_token_stream,
                                                &unbounded_excluded_token_stream,
                                            )
                                        };
                                        let range_match_timestamp_and_timestamp_tz_token_stream = generate_range_match_token_stream(
                                            &generate_sqlx_pg_types_pg_range_token_stream(&included_start_token_stream, &included_end_token_stream),
                                            &if_eq_unbounded_unbounded_or_included_excluded_token_stream,
                                            &sqlx_pg_types_pg_range_included_unbounded_token_stream,
                                            &generate_if_start_end_eq_token_stream(&sqlx_pg_types_pg_range_unbounded_unbounded_token_stream, &sqlx_pg_types_pg_range_excluded_included_token_stream),
                                            &generate_if_start_end_eq_token_stream(&sqlx_pg_types_pg_range_unbounded_unbounded_token_stream, &sqlx_pg_types_pg_range_excluded_excluded_token_stream),
                                            &generate_sqlx_pg_types_pg_range_token_stream(&excluded_start_token_stream, &unbounded_upper_camel_case),
                                            &generate_sqlx_pg_types_pg_range_token_stream(&unbounded_upper_camel_case, &included_end_token_stream),
                                            &sqlx_pg_types_pg_range_unbounded_excluded_token_stream,
                                        );
                                        match &range {
                                            Range::I32AsInt4 | Range::I64AsInt8 => int_range_normalize_token_stream,
                                            Range::SqlxTypesChronoNaiveDateAsDate => {
                                                let generate_dot_succ_opt_expect_token_stream = |id: &dyn std::fmt::Display| {
                                                    let id_double_quoted_token_stream = generate_quotes::dq_token_stream(&id);
                                                    quote::quote! {.succ_opt().expect(#id_double_quoted_token_stream)}
                                                };
                                                let generate_included_start_succ_opt_token_stream = |id: &dyn std::fmt::Display| {
                                                    let dot_succ_opt_expect_token_stream = generate_dot_succ_opt_expect_token_stream(&id);
                                                    quote::quote! {#included_upper_camel_case(#start_snake_case #dot_succ_opt_expect_token_stream)}
                                                };
                                                let generate_excluded_end_succ_opt_token_stream = |id: &dyn std::fmt::Display| {
                                                    let dot_succ_opt_expect_token_stream = generate_dot_succ_opt_expect_token_stream(&id);
                                                    quote::quote! {#excluded_upper_camel_case(#end_snake_case #dot_succ_opt_expect_token_stream)}
                                                };
                                                generate_range_match_token_stream(
                                                    &generate_sqlx_pg_types_pg_range_token_stream(&included_start_token_stream, &quote::quote! {#excluded_upper_camel_case(#end_snake_case.succ_opt().expect("9ebce3b4"))}),
                                                    &if_eq_unbounded_unbounded_or_included_excluded_token_stream,
                                                    &sqlx_pg_types_pg_range_included_unbounded_token_stream,
                                                    &generate_if_start_end_eq_token_stream(
                                                        &sqlx_pg_types_pg_range_unbounded_unbounded_token_stream,
                                                        &generate_sqlx_pg_types_pg_range_token_stream(&generate_included_start_succ_opt_token_stream(&str_constants::VALUE_98A0357B_D21A_4949_A101_C641528D2376), &generate_excluded_end_succ_opt_token_stream(&str_constants::FE53A6B9_2D7E_4605_9F5A_7F5C21CC01E6)),
                                                    ),
                                                    &generate_if_start_end_eq_token_stream(&sqlx_pg_types_pg_range_unbounded_unbounded_token_stream, &generate_sqlx_pg_types_pg_range_token_stream(&generate_included_start_succ_opt_token_stream(&str_constants::D8A26635_C478_4A2A_ACF4_BF1765702889), &excluded_end_token_stream)),
                                                    &generate_sqlx_pg_types_pg_range_token_stream(&generate_included_start_succ_opt_token_stream(&str_constants::VALUE_9811C7C7_D7F5_4FB7_9D25_AFFB0BD4F5FB), &unbounded_upper_camel_case),
                                                    &generate_sqlx_pg_types_pg_range_token_stream(&unbounded_upper_camel_case, &generate_excluded_end_succ_opt_token_stream(&str_constants::D6288F19_0A24_42AD_9E69_36036D9F2C1D)),
                                                    &sqlx_pg_types_pg_range_unbounded_excluded_token_stream,
                                                )
                                            }
                                            Range::SqlxTypesChronoNaiveDateTimeAsTimestamp | Range::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => range_match_timestamp_and_timestamp_tz_token_stream,
                                        }
                                    }
                                )
                            }
                            pg_crud_macros_common::IsNullable::True => generate_identifier_read_identifier_origin_token_stream(&quote::quote! {
                                #v_snake_case.0.0.map(
                                    |v_4561270e|
                                    <
                                        #identifier_standard_non_null_upper_camel_case
                                        as
                                        #import::PgType
                                    >::normalize(
                                        #identifier_standard_non_null_read_upper_camel_case(v_4561270e)
                                    ).0
                                )
                            }),
                        },
                    }
                },
                &if matches!(&is_non_null_standard_can_be_primary_key, IsNonNullStandardCanBePrimaryKey::True) {
                    quote::quote! {#identifier_read_ids_upper_camel_case}
                } else {
                    quote::quote! {#import_non_primary_key_pg_type_read_ids_token_stream}
                },
                &select_only_ids_and_select_only_updated_ids_query_common_token_stream,
                &identifier_read_inner_upper_camel_case,
                &{
                    let generate_identifier_standard_non_null_into_inner_identifier_standard_non_null_read_token_stream = |ts: &dyn quote::ToTokens| {
                        quote::quote! {
                            #identifier_standard_non_null_as_pg_type_token_stream::into_inner(
                                #identifier_standard_non_null_read_upper_camel_case(#ts)
                            )
                        }
                    };
                    let v_dot_zero_token_stream = quote::quote! {#v_snake_case.0};
                    let v_dot_zero_dot_zero_token_stream = quote::quote! {#v_dot_zero_token_stream.0};
                    match &pg_type_pattern {
                        PgTypePattern::Standard => match &is_nullable {
                            pg_crud_macros_common::IsNullable::False => {
                                if range_try_from_pg_type_is_ok {
                                    generate_pg_range_conversion_token_stream(&v_dot_zero_dot_zero_token_stream, &quote::quote! {v_af65ccce})
                                } else {
                                    v_dot_zero_dot_zero_token_stream
                                }
                            }
                            pg_crud_macros_common::IsNullable::True => {
                                let ts = if range_try_from_pg_type_is_ok {
                                    generate_identifier_standard_non_null_into_inner_identifier_standard_non_null_read_token_stream(&quote::quote! {v_bd169d3b})
                                } else {
                                    quote::quote! {v_bd169d3b.0}
                                };
                                quote::quote! {#v_dot_zero_dot_zero_token_stream.map(|v_bd169d3b| #ts)}
                            }
                        },
                    }
                },
                &identifier_update_upper_camel_case,
                &identifier_update_for_query_upper_camel_case,
                &pg_crud_macros_common::UpdateQueryPartValueUndrscr::True,
                &pg_crud_macros_common::UpdateQueryPartAccumulatorUndrscr::True,
                &pg_crud_macros_common::UpdateQueryPartTargetUndrscr::True,
                &pg_crud_macros_common::UpdateQueryPartPathUndrscr::True,
                &typical_query_part_token_stream,
                &pg_crud_macros_common::IsUpdateQueryBindMut::True,
                &typical_query_bind_token_stream,
                &select_only_ids_and_select_only_updated_ids_query_common_token_stream,
                &pg_crud_macros_common::IsSelectOnlyUpdatedIdsQueryBindMut::False,
                &quote::quote! {Ok(#query_snake_case)},
            )
        };
        let impl_pg_type_test_cases_for_identifier_token_stream = {
            enum IsNeedToUseInto {
                False,
                True,
            }
            let generate_read_or_read_inner_into_update_with_new_or_try_new_unwraped_token_stream = |read_or_update: &pg_crud_macros_common::ReadOrUpdate| {
                let read_or_update_upper_camel_case = read_or_update.ucc();
                let ts = if pg_type_initialization_try_new_try_from_pg_type.is_ok() {
                    quote::quote! {#try_new_snake_case(#v_snake_case).expect("69477d2f")}
                } else {
                    quote::quote! {#new_snake_case(#v_snake_case)}
                };
                quote::quote! {<#self_upper_camel_case::#pg_type_upper_camel_case
                    as
                #import::#pg_type_upper_camel_case>::#read_or_update_upper_camel_case:: #ts}
            };
            let generate_standard_non_null_test_case_handle_token_stream = |is_need_to_use_into: &IsNeedToUseInto| {
                let generate_range_read_ids_to_2_dimensions_vec_read_inner_token_stream =
                    |min_token_stream: &dyn quote::ToTokens, negative_less_typical_token_stream: &dyn quote::ToTokens, negative_more_typical_token_stream: &dyn quote::ToTokens, near_zero_token_stream: &dyn quote::ToTokens, positive_less_typical_token_stream: &dyn quote::ToTokens, positive_more_typical_token_stream: &dyn quote::ToTokens, max_token_stream: &dyn quote::ToTokens| {
                        enum Bnd<'lt> {
                            Excl(&'lt dyn quote::ToTokens),
                            Incl(&'lt dyn quote::ToTokens),
                            Unb,
                        }
                        let test_cases_array_token_stream = [
                            (Bnd::Incl(&min_snake_case),Bnd::Incl(&min_snake_case)),
                            (Bnd::Incl(&negative_less_typical_snake_case),Bnd::Incl(&negative_more_typical_snake_case)),
                            (Bnd::Incl(&near_zero_snake_case), Bnd::Incl(&near_zero_snake_case)),
                            (Bnd::Incl(&positive_less_typical_snake_case), Bnd::Incl(&positive_more_typical_snake_case)),
                            (Bnd::Incl(&max_snake_case), Bnd::Incl(&max_snake_case)),
                            (Bnd::Incl(&min_snake_case), Bnd::Incl(&max_snake_case)),
                            (Bnd::Incl(&min_snake_case), Bnd::Excl(&min_snake_case)),
                            (Bnd::Incl(&negative_less_typical_snake_case), Bnd::Excl(&negative_more_typical_snake_case)),
                            (Bnd::Incl(&near_zero_snake_case), Bnd::Excl(&near_zero_snake_case)),
                            (Bnd::Incl(&positive_less_typical_snake_case), Bnd::Excl(&positive_more_typical_snake_case)),
                            (Bnd::Incl(&max_snake_case), Bnd::Excl(&max_snake_case)),
                            (Bnd::Incl(&min_snake_case), Bnd::Excl(&max_snake_case)),
                            (Bnd::Incl(&min_snake_case), Bnd::Unb),
                            (Bnd::Incl(&negative_less_typical_snake_case), Bnd::Unb),
                            (Bnd::Incl(&near_zero_snake_case), Bnd::Unb),
                            (Bnd::Incl(&positive_less_typical_snake_case), Bnd::Unb),
                            (Bnd::Incl(&max_snake_case), Bnd::Unb),
                            (Bnd::Excl(&min_snake_case), Bnd::Incl(&min_snake_case)),
                            (Bnd::Excl(&negative_less_typical_snake_case), Bnd::Incl(&negative_more_typical_snake_case)),
                            (Bnd::Excl(&near_zero_snake_case), Bnd::Incl(&near_zero_snake_case)),
                            (Bnd::Excl(&positive_less_typical_snake_case), Bnd::Incl(&positive_more_typical_snake_case)),
                            (Bnd::Excl(&max_snake_case), Bnd::Incl(&max_snake_case)),
                            (Bnd::Excl(&min_snake_case), Bnd::Incl(&max_snake_case)),
                            (Bnd::Excl(&min_snake_case), Bnd::Excl(&min_snake_case)),
                            (Bnd::Excl(&negative_less_typical_snake_case), Bnd::Excl(&negative_more_typical_snake_case)),
                            (Bnd::Excl(&near_zero_snake_case), Bnd::Excl(&near_zero_snake_case)),
                            (Bnd::Excl(&positive_less_typical_snake_case), Bnd::Excl(&positive_more_typical_snake_case)),
                            (Bnd::Excl(&max_snake_case), Bnd::Excl(&max_snake_case)),
                            (Bnd::Excl(&min_snake_case), Bnd::Excl(&max_snake_case)),
                            (Bnd::Excl(&min_snake_case), Bnd::Unb),
                            (Bnd::Excl(&negative_less_typical_snake_case), Bnd::Unb),
                            (Bnd::Excl(&near_zero_snake_case), Bnd::Unb),
                            (Bnd::Excl(&positive_less_typical_snake_case), Bnd::Unb),
                            (Bnd::Excl(&max_snake_case), Bnd::Unb),
                            (Bnd::Unb, Bnd::Incl(&min_snake_case)),
                            (Bnd::Unb, Bnd::Incl(&negative_more_typical_snake_case)),
                            (Bnd::Unb, Bnd::Incl(&near_zero_snake_case)),
                            (Bnd::Unb, Bnd::Incl(&positive_more_typical_snake_case)),
                            (Bnd::Unb, Bnd::Incl(&max_snake_case)),
                            (Bnd::Unb, Bnd::Excl(&min_snake_case)),
                            (Bnd::Unb, Bnd::Excl(&negative_more_typical_snake_case)),
                            (Bnd::Unb, Bnd::Excl(&near_zero_snake_case)),
                            (Bnd::Unb, Bnd::Excl(&positive_more_typical_snake_case)),
                            (Bnd::Unb, Bnd::Excl(&max_snake_case)),
                            (Bnd::Unb, Bnd::Unb),
                        ]
                        .into_iter()
                        .map(|(start, end)|{
                            let (start_token_stream,end_token_stream) = {
                                let generate_bound_token_stream = |bnd: Bnd<'_>|{
                                    let ts = match bnd {
                                        Bnd::Excl(ts) => quote::quote! {Excluded(#ts)},
                                        Bnd::Incl(ts) => quote::quote! {Included(#ts)},
                                        Bnd::Unb => quote::quote! {Unbounded},
                                    };
                                    quote::quote!{std::ops::Bound::#ts}
                                };
                                (generate_bound_token_stream(start), generate_bound_token_stream(end))
                            };
                            quote::quote! {sqlx::postgres::types::PgRange { start: #start_token_stream, end: #end_token_stream}}
                        });
                        quote::quote! {{
                            let #min_snake_case = #min_token_stream;
                            let #max_snake_case = #max_token_stream;
                            let #negative_less_typical_snake_case = #negative_less_typical_token_stream;
                            let #negative_more_typical_snake_case = #negative_more_typical_token_stream;
                            let #near_zero_snake_case = #near_zero_token_stream;
                            let #positive_less_typical_snake_case = #positive_less_typical_token_stream;
                            let #positive_more_typical_snake_case = #positive_more_typical_token_stream;
                            vec![#(#test_cases_array_token_stream),*]
                        }}
                    };
                let generate_int_pgrange_read_ids_to_2_dimensions_vec_read_inner_token_stream = |int_range_type: &IntRangeType| {
                    let range_inner_type_token_stream = int_range_type_to_range_inner_type_token_stream(int_range_type);
                    generate_range_read_ids_to_2_dimensions_vec_read_inner_token_stream(&quote::quote! {#range_inner_type_token_stream::MIN}, &quote::quote! {-20}, &quote::quote! {-10}, &quote::quote! {0}, &quote::quote! {10}, &quote::quote! {20}, &quote::quote! {#range_inner_type_token_stream::MAX - 1})
                };
                let empty_vec_token_stream = quote::quote! {Vec::new()};
                let generate_identifier_standard_non_null_fn_token_stream = |
                    identifier_parameter: &dyn quote::ToTokens,
                    ts: &dyn quote::ToTokens
                |quote::quote! {#identifier_parameter::#ts()};
                let (
                    identifier_sqlx_types_chrono_naive_time_min_token_stream,
                    identifier_sqlx_types_chrono_naive_time_ten_token_stream,
                    identifier_sqlx_types_chrono_naive_time_twenty_token_stream,
                    identifier_sqlx_types_chrono_naive_time_max_token_stream
                ) = {
                    let generate_token_stream = |
                        ts_parameter: &dyn quote::ToTokens
                    |generate_identifier_standard_non_null_fn_token_stream(
                        &generate_identifier_standard_non_null_token_stream(&PgType::SqlxTypesChronoNaiveTimeAsTime),
                        &ts_parameter
                    );
                    (
                        generate_token_stream(
                            &sqlx_types_chrono_naive_time_min_fn_token_stream
                        ),
                        generate_token_stream(
                            &sqlx_types_chrono_naive_time_ten_fn_token_stream
                        ),
                        generate_token_stream(
                            &sqlx_types_chrono_naive_time_twenty_fn_token_stream
                        ),
                        generate_token_stream(
                            &sqlx_types_chrono_naive_time_max_fn_token_stream
                        )
                    )
                };
                let (
                    identifier_sqlx_types_chrono_naive_date_min_token_stream,
                    identifier_sqlx_types_chrono_naive_date_negative_less_typical_token_stream,
                    identifier_sqlx_types_chrono_naive_date_negative_more_typical_token_stream,
                    identifier_sqlx_types_chrono_naive_date_near_zero_token_stream,
                    identifier_sqlx_types_chrono_naive_date_positive_less_typical_token_stream,
                    identifier_sqlx_types_chrono_naive_date_positive_more_typical_token_stream,
                    identifier_sqlx_types_chrono_naive_date_max_token_stream,
                    identifier_sqlx_types_chrono_naive_date_max_pred_opt_expect_token_stream,
                ) = {
                    let generate_token_stream = |
                        ts_parameter: &dyn quote::ToTokens
                    |generate_identifier_standard_non_null_fn_token_stream(
                        &generate_identifier_standard_non_null_token_stream(&PgType::SqlxTypesChronoNaiveDateAsDate),
                        &ts_parameter
                    );
                    (
                        generate_token_stream(
                            &sqlx_types_chrono_naive_date_min_fn_token_stream,
                        ),
                        generate_token_stream(
                            &sqlx_types_chrono_naive_date_negative_less_typical_fn_token_stream,
                        ),
                        generate_token_stream(
                            &sqlx_types_chrono_naive_date_negative_more_typical_fn_token_stream,
                        ),
                        generate_token_stream(
                            &sqlx_types_chrono_naive_date_near_zero_fn_token_stream,
                        ),
                        generate_token_stream(
                            &sqlx_types_chrono_naive_date_positive_less_typical_fn_token_stream,
                        ),
                        generate_token_stream(
                            &sqlx_types_chrono_naive_date_positive_more_typical_fn_token_stream,
                        ),
                        generate_token_stream(
                            &sqlx_types_chrono_naive_date_max_fn_token_stream,
                        ),
                        generate_token_stream(
                            &sqlx_types_chrono_naive_date_max_pred_opt_expect_fn_token_stream,
                        ),
                    )
                };
                let (
                    sqlx_types_chrono_naive_date_time_min_token_stream,
                    sqlx_types_chrono_naive_date_time_negative_less_typical_token_stream,
                    sqlx_types_chrono_naive_date_time_negative_more_typical_token_stream,
                    sqlx_types_chrono_naive_date_time_near_zero_token_stream,
                    sqlx_types_chrono_naive_date_time_positive_less_typical_token_stream,
                    sqlx_types_chrono_naive_date_time_positive_more_typical_token_stream,
                    sqlx_types_chrono_naive_date_time_max_token_stream,
                ) = {
                    let generate_token_stream = |date: &dyn quote::ToTokens, time: &dyn quote::ToTokens| {
                        generate_sqlx_types_chrono_naive_date_time_new_token_stream(&quote::quote! { #date, #time })
                    };
                    (
                        generate_token_stream(&identifier_sqlx_types_chrono_naive_date_min_token_stream, &identifier_sqlx_types_chrono_naive_time_min_token_stream),
                        generate_token_stream(&identifier_sqlx_types_chrono_naive_date_negative_less_typical_token_stream, &identifier_sqlx_types_chrono_naive_time_twenty_token_stream),
                        generate_token_stream(&identifier_sqlx_types_chrono_naive_date_negative_more_typical_token_stream, &identifier_sqlx_types_chrono_naive_time_ten_token_stream),
                        generate_token_stream(&identifier_sqlx_types_chrono_naive_date_near_zero_token_stream, &identifier_sqlx_types_chrono_naive_time_min_token_stream),
                        generate_token_stream(&identifier_sqlx_types_chrono_naive_date_positive_less_typical_token_stream, &identifier_sqlx_types_chrono_naive_time_ten_token_stream),
                        generate_token_stream(&identifier_sqlx_types_chrono_naive_date_positive_more_typical_token_stream, &identifier_sqlx_types_chrono_naive_time_twenty_token_stream),
                        generate_token_stream(&identifier_sqlx_types_chrono_naive_date_max_token_stream, &identifier_sqlx_types_chrono_naive_time_max_token_stream),
                    )
                };
                let (
                    sqlx_types_chrono_date_time_sqlx_types_chrono_utc_min_token_stream,
                    sqlx_types_chrono_date_time_sqlx_types_chrono_utc_negative_less_typical_token_stream,
                    sqlx_types_chrono_date_time_sqlx_types_chrono_utc_negative_more_typical_token_stream,
                    sqlx_types_chrono_date_time_sqlx_types_chrono_utc_near_zero_token_stream,
                    sqlx_types_chrono_date_time_sqlx_types_chrono_utc_positive_less_typical_token_stream,
                    sqlx_types_chrono_date_time_sqlx_types_chrono_utc_positive_more_typical_token_stream,
                    sqlx_types_chrono_date_time_sqlx_types_chrono_utc_max_token_stream,
                ) = {
                    let generate_token_stream = |ts: &dyn quote::ToTokens| generate_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_from_naive_utc_and_offset_token_stream(ts);
                    (
                        generate_token_stream(&sqlx_types_chrono_naive_date_time_min_token_stream),
                        generate_token_stream(&sqlx_types_chrono_naive_date_time_negative_less_typical_token_stream),
                        generate_token_stream(&sqlx_types_chrono_naive_date_time_negative_more_typical_token_stream),
                        generate_token_stream(&sqlx_types_chrono_naive_date_time_near_zero_token_stream),
                        generate_token_stream(&sqlx_types_chrono_naive_date_time_positive_less_typical_token_stream),
                        generate_token_stream(&sqlx_types_chrono_naive_date_time_positive_more_typical_token_stream),
                        generate_token_stream(&sqlx_types_chrono_naive_date_time_max_token_stream),
                    )
                };
                let generate_typical_test_cases_vec_token_stream = |ts: &dyn quote::ToTokens| {
                    let ts0 = match &is_need_to_use_into {
                        IsNeedToUseInto::True => quote::quote! {.into()},
                        IsNeedToUseInto::False => proc_macro2::TokenStream::new(),
                    };
                    quote::quote! {#import::#ts()#ts0}
                };
                let generate_token_stream = |ts: &dyn quote::ToTokens| generate_identifier_standard_non_null_fn_token_stream(&self_upper_camel_case, &ts);
                match &pg_type {
                    PgType::I16AsInt2 => generate_typical_test_cases_vec_token_stream(&quote::quote! {i16_test_cases_vec}),
                    PgType::I32AsInt4 => generate_typical_test_cases_vec_token_stream(&quote::quote! {i32_test_cases_vec}),
                    PgType::I64AsInt8 => generate_typical_test_cases_vec_token_stream(&quote::quote! {i64_test_cases_vec}),
                    PgType::F32AsFloat4 => generate_typical_test_cases_vec_token_stream(&quote::quote! {f32_test_cases_vec}),
                    PgType::F64AsFloat8 => generate_typical_test_cases_vec_token_stream(&quote::quote! {f64_test_cases_vec}),
                    PgType::I16AsSmallSerialInitializationByPg | PgType::I32AsSerialInitializationByPg | PgType::I64AsBigSerialInitializationByPg => empty_vec_token_stream,
                    PgType::SqlxPgTypesPgMoneyAsMoney => quote::quote! {
                        #import::i64_test_cases_vec().into_iter().map(
                            #inner_type_standard_non_null_token_stream
                        ).collect::<Vec<#inner_type_standard_non_null_token_stream>>()
                    },
                    PgType::BoolAsBool => generate_typical_test_cases_vec_token_stream(&quote::quote! {bool_test_cases_vec}),
                    PgType::StringAsText => generate_typical_test_cases_vec_token_stream(&quote::quote! {string_test_cases_vec}),
                    PgType::StdVecVecU8AsBytea => quote::quote! {vec![
                        Vec::new(),
                        (0u8..=255).collect(),
                        vec![0; 1024],
                        vec![0; 1024 * 1024 * 2],
                    ]},
                    PgType::SqlxTypesChronoNaiveTimeAsTime => {
                        let self_sqlx_types_chrono_naive_time_min_token_stream = generate_token_stream(&sqlx_types_chrono_naive_time_min_fn_token_stream);
                        let self_sqlx_types_chrono_naive_time_ten_token_stream = generate_token_stream(&sqlx_types_chrono_naive_time_ten_fn_token_stream);
                        let self_sqlx_types_chrono_naive_time_twenty_token_stream = generate_token_stream(&sqlx_types_chrono_naive_time_twenty_fn_token_stream);
                        let self_sqlx_types_chrono_naive_time_max_token_stream = generate_token_stream(&sqlx_types_chrono_naive_time_max_fn_token_stream);
                        quote::quote! {vec![
                            #self_sqlx_types_chrono_naive_time_min_token_stream,
                            #self_sqlx_types_chrono_naive_time_ten_token_stream,
                            #self_sqlx_types_chrono_naive_time_twenty_token_stream,
                            #self_sqlx_types_chrono_naive_time_max_token_stream,
                        ]}
                    },
                    PgType::SqlxTypesTimeTimeAsTime => {
                        let sqlx_types_time_time_from_hms_micro_min_unwrap_token_stream = generate_sqlx_types_time_time_from_hms_micro_unwrap_token_stream(&quote::quote! {0,0,0,0});
                        let sqlx_types_time_time_from_hms_micro_ten_unwrap_token_stream = generate_sqlx_types_time_time_from_hms_micro_unwrap_token_stream(&quote::quote! {10,10,10,10});
                        let sqlx_types_time_time_from_hms_micro_twenty_unwrap_token_stream = generate_sqlx_types_time_time_from_hms_micro_unwrap_token_stream(&quote::quote! {20,20,20,20});
                        let sqlx_types_time_time_from_hms_micro_max_unwrap_token_stream = generate_sqlx_types_time_time_from_hms_micro_unwrap_token_stream(&quote::quote! {23,59,59,999_999});
                        quote::quote! {vec![
                            #sqlx_types_time_time_from_hms_micro_min_unwrap_token_stream,
                            #sqlx_types_time_time_from_hms_micro_ten_unwrap_token_stream,
                            #sqlx_types_time_time_from_hms_micro_twenty_unwrap_token_stream,
                            #sqlx_types_time_time_from_hms_micro_max_unwrap_token_stream,
                        ]}
                    }
                    PgType::SqlxPgTypesPgIntervalAsInterval => {
                        let min_token_stream = quote::quote! {MIN};
                        let max_token_stream = quote::quote! {MAX};
                        let i32_min_token_stream = quote::quote! {#i32_token_stream::#min_token_stream};
                        let i32_max_token_stream = quote::quote! {#i32_token_stream::#max_token_stream};
                        let generate_sqlx_pg_types_pg_interval_token_stream = |months_token_stream: &dyn quote::ToTokens, days_token_stream: &dyn quote::ToTokens, microseconds_token_stream: &dyn quote::ToTokens| {
                            quote::quote! {sqlx::postgres::types::PgInterval {
                                months: #months_token_stream,
                                days: #days_token_stream,
                                microseconds: #microseconds_token_stream
                            }}
                        };
                        let interval_min_token_stream = generate_sqlx_pg_types_pg_interval_token_stream(&i32_min_token_stream, &i32_min_token_stream, &quote::quote! {#i64_token_stream::#min_token_stream});
                        let interval_max_token_stream = generate_sqlx_pg_types_pg_interval_token_stream(&i32_max_token_stream, &i32_max_token_stream, &quote::quote! {#i64_token_stream::#max_token_stream});
                        quote::quote! {vec![
                            #interval_min_token_stream,
                            #interval_max_token_stream
                        ]}
                    }
                    PgType::SqlxTypesChronoNaiveDateAsDate => {
                        let sqlx_types_chrono_naive_date_min_token_stream = generate_token_stream(&sqlx_types_chrono_naive_date_min_fn_token_stream);
                        let sqlx_types_chrono_naive_date_negative_less_typical_token_stream = generate_token_stream(&sqlx_types_chrono_naive_date_negative_less_typical_fn_token_stream);
                        let sqlx_types_chrono_naive_date_negative_more_typical_token_stream = generate_token_stream(&sqlx_types_chrono_naive_date_negative_more_typical_fn_token_stream);
                        let sqlx_types_chrono_naive_date_near_zero_token_stream = generate_token_stream(&sqlx_types_chrono_naive_date_near_zero_fn_token_stream);
                        let sqlx_types_chrono_naive_date_positive_less_typical_token_stream = generate_token_stream(&sqlx_types_chrono_naive_date_positive_less_typical_fn_token_stream);
                        let sqlx_types_chrono_naive_date_positive_more_typical_token_stream = generate_token_stream(&sqlx_types_chrono_naive_date_positive_more_typical_fn_token_stream);
                        let sqlx_types_chrono_naive_date_max_token_stream = generate_token_stream(&sqlx_types_chrono_naive_date_max_fn_token_stream);
                        quote::quote! {vec![
                            #sqlx_types_chrono_naive_date_min_token_stream,
                            #sqlx_types_chrono_naive_date_negative_less_typical_token_stream,
                            #sqlx_types_chrono_naive_date_negative_more_typical_token_stream,
                            #sqlx_types_chrono_naive_date_near_zero_token_stream,
                            #sqlx_types_chrono_naive_date_positive_less_typical_token_stream,
                            #sqlx_types_chrono_naive_date_positive_more_typical_token_stream,
                            #sqlx_types_chrono_naive_date_max_token_stream,
                        ]}
                    },
                    PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp => quote::quote! {vec![
                        #sqlx_types_chrono_naive_date_time_min_token_stream,
                        #sqlx_types_chrono_naive_date_time_negative_less_typical_token_stream,
                        #sqlx_types_chrono_naive_date_time_negative_more_typical_token_stream,
                        #sqlx_types_chrono_naive_date_time_near_zero_token_stream,
                        #sqlx_types_chrono_naive_date_time_positive_less_typical_token_stream,
                        #sqlx_types_chrono_naive_date_time_positive_more_typical_token_stream,
                        #sqlx_types_chrono_naive_date_time_max_token_stream,
                    ]},
                    PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => quote::quote! {vec![
                        #sqlx_types_chrono_date_time_sqlx_types_chrono_utc_min_token_stream,
                        #sqlx_types_chrono_date_time_sqlx_types_chrono_utc_negative_less_typical_token_stream,
                        #sqlx_types_chrono_date_time_sqlx_types_chrono_utc_negative_more_typical_token_stream,
                        #sqlx_types_chrono_date_time_sqlx_types_chrono_utc_near_zero_token_stream,
                        #sqlx_types_chrono_date_time_sqlx_types_chrono_utc_positive_less_typical_token_stream,
                        #sqlx_types_chrono_date_time_sqlx_types_chrono_utc_positive_more_typical_token_stream,
                        #sqlx_types_chrono_date_time_sqlx_types_chrono_utc_max_token_stream,
                    ]},
                    PgType::SqlxTypesUuidUuidAsUuidV4InitializationByPg => quote::quote! {Vec::new()},
                    PgType::SqlxTypesUuidUuidAsUuidInitializationByClient => quote::quote! {vec![
                        sqlx::types::Uuid::from_u128(1u128)
                    ]},
                    PgType::SqlxTypesIpnetworkIpNetworkAsInet => quote::quote! {vec![
                        <sqlx::types::ipnetwork::IpNetwork as std::str::FromStr>::from_str("192.168.0.0/24").expect("478dbded"),
                        <sqlx::types::ipnetwork::IpNetwork as std::str::FromStr>::from_str("10.0.0.0/8").expect("8af9e27e"),
                        <sqlx::types::ipnetwork::IpNetwork as std::str::FromStr>::from_str("172.16.0.0/12").expect("ba86505f"),
                        <sqlx::types::ipnetwork::IpNetwork as std::str::FromStr>::from_str("127.0.0.1/32").expect("32c744a0"),
                        <sqlx::types::ipnetwork::IpNetwork as std::str::FromStr>::from_str("::1/128").expect("560815f8"),
                        <sqlx::types::ipnetwork::IpNetwork as std::str::FromStr>::from_str("2001:db8::/32").expect("793db0ef"),
                        sqlx::types::ipnetwork::IpNetwork::V4(sqlx::types::ipnetwork::Ipv4Network::#new_snake_case(std::net::Ipv4Addr::#new_snake_case(192, 168, 0, 0), 24).expect("c44934f2")),
                        sqlx::types::ipnetwork::IpNetwork::V4(sqlx::types::ipnetwork::Ipv4Network::#new_snake_case(std::net::Ipv4Addr::#new_snake_case(10, 0, 0, 0), 8).expect("39e588d9")),
                        sqlx::types::ipnetwork::IpNetwork::V4(sqlx::types::ipnetwork::Ipv4Network::#new_snake_case(std::net::Ipv4Addr::LOCALHOST, 32).expect("43fb25bd")),
                        sqlx::types::ipnetwork::IpNetwork::V6(sqlx::types::ipnetwork::Ipv6Network::#new_snake_case(std::net::Ipv6Addr::LOCALHOST, 128).expect("b443be46")),
                        sqlx::types::ipnetwork::IpNetwork::V6(sqlx::types::ipnetwork::Ipv6Network::#new_snake_case("2001:db8::".parse().expect("d4e6df27"), 32).expect("a7486c5e")),
                    ]},
                    PgType::SqlxTypesMacAddressMacAddressAsMacAddr => quote::quote! {vec![
                        sqlx::types::mac_address::MacAddress::#new_snake_case([0x00, 0x00, 0x00, 0x00, 0x00, 0x00]), // All zeros
                        sqlx::types::mac_address::MacAddress::#new_snake_case([0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]), // All ones (broadcast address)
                        sqlx::types::mac_address::MacAddress::#new_snake_case([0x02, 0x00, 0x00, 0x00, 0x00, 0x01]), // Locally administered address
                        sqlx::types::mac_address::MacAddress::#new_snake_case([0x00, 0x1A, 0x2B, 0x3C, 0x4D, 0x5E]), // Universally administered address
                        sqlx::types::mac_address::MacAddress::#new_snake_case([0x01, 0x00, 0x5E, 0x00, 0x00, 0xFB]), // Multicast address
                        sqlx::types::mac_address::MacAddress::#new_snake_case([0xDE, 0xAD, 0xBE, 0xEF, 0xCA, 0xFE]), // Random valid MAC
                    ]},
                    PgType::SqlxPgTypesPgRangeI32AsInt4Range => generate_int_pgrange_read_ids_to_2_dimensions_vec_read_inner_token_stream(&IntRangeType::SqlxPgTypesPgRangeI32AsInt4Range),
                    PgType::SqlxPgTypesPgRangeI64AsInt8Range => generate_int_pgrange_read_ids_to_2_dimensions_vec_read_inner_token_stream(&IntRangeType::SqlxPgTypesPgRangeI64AsInt8Range),
                    PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => generate_range_read_ids_to_2_dimensions_vec_read_inner_token_stream(
                        &identifier_sqlx_types_chrono_naive_date_min_token_stream,
                        &identifier_sqlx_types_chrono_naive_date_negative_less_typical_token_stream,
                        &identifier_sqlx_types_chrono_naive_date_negative_more_typical_token_stream,
                        &identifier_sqlx_types_chrono_naive_date_near_zero_token_stream,
                        &identifier_sqlx_types_chrono_naive_date_positive_less_typical_token_stream,
                        &identifier_sqlx_types_chrono_naive_date_positive_more_typical_token_stream,
                        &identifier_sqlx_types_chrono_naive_date_max_pred_opt_expect_token_stream
                    ),
                    PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => generate_range_read_ids_to_2_dimensions_vec_read_inner_token_stream(
                        &sqlx_types_chrono_naive_date_time_min_token_stream,
                        &sqlx_types_chrono_naive_date_time_negative_less_typical_token_stream,
                        &sqlx_types_chrono_naive_date_time_negative_more_typical_token_stream,
                        &sqlx_types_chrono_naive_date_time_near_zero_token_stream,
                        &sqlx_types_chrono_naive_date_time_positive_less_typical_token_stream,
                        &sqlx_types_chrono_naive_date_time_positive_more_typical_token_stream,
                        &sqlx_types_chrono_naive_date_time_max_token_stream,
                    ),
                    PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => generate_range_read_ids_to_2_dimensions_vec_read_inner_token_stream(
                        &sqlx_types_chrono_date_time_sqlx_types_chrono_utc_min_token_stream,
                        &sqlx_types_chrono_date_time_sqlx_types_chrono_utc_negative_less_typical_token_stream,
                        &sqlx_types_chrono_date_time_sqlx_types_chrono_utc_negative_more_typical_token_stream,
                        &sqlx_types_chrono_date_time_sqlx_types_chrono_utc_near_zero_token_stream,
                        &sqlx_types_chrono_date_time_sqlx_types_chrono_utc_positive_less_typical_token_stream,
                        &sqlx_types_chrono_date_time_sqlx_types_chrono_utc_positive_more_typical_token_stream,
                        &sqlx_types_chrono_date_time_sqlx_types_chrono_utc_max_token_stream,
                    ),
                }
            };
            let optional_vec_create_token_stream: Option<proc_macro2::TokenStream> = {
                let generate_some_accumulator_token_stream = |
                    is_nullable_parameter: &pg_crud_macros_common::IsNullable,
                    identifier_token_stream_parameter: &dyn quote::ToTokens,
                    additonal_token_stream: &dyn quote::ToTokens
                | {
                    let (new_or_try_new_token_stream, maybe_accumulator_push_none_token_stream) = match (&is_nullable_parameter, pg_type_initialization_try_new_try_from_pg_type.is_ok()) {
                        (pg_crud_macros_common::IsNullable::False, true) => (quote::quote! {try_new(vec![element_0fd5865b.0.into()]).expect("adbae6b3")}, proc_macro2::TokenStream::new()),
                        (pg_crud_macros_common::IsNullable::False, false) => (quote::quote! {new(vec![element_0fd5865b.0.into()])}, proc_macro2::TokenStream::new()),
                        (pg_crud_macros_common::IsNullable::True, true) => (
                            quote::quote! {try_new(Some(element_0fd5865b.0.into())).expect("b244d498")},
                            quote::quote! {accumulator_0b59a062.push(#self_as_pg_type_token_stream::Create::try_new(None).expect("31878971"));},
                        ),
                        (pg_crud_macros_common::IsNullable::True, false) => (quote::quote! {new(Some(element_0fd5865b.0.into()))}, quote::quote! {accumulator_0b59a062.push(#self_as_pg_type_token_stream::Create::new(None));}),
                    };
                    let identifier_as_pg_type_test_cases_token_stream = generate_as_pg_type_test_cases_token_stream(&identifier_token_stream_parameter);
                    quote::quote! {Some({
                        let optional_vec_create_53debb64 = #identifier_as_pg_type_test_cases_token_stream::#optional_vec_create_snake_case().unwrap_or(Vec::new());
                        let mut accumulator_0b59a062 = Vec::with_capacity(optional_vec_create_53debb64.len().saturating_add(1));
                        for element_0fd5865b in optional_vec_create_53debb64 {
                            accumulator_0b59a062.push(#self_as_pg_type_token_stream::Create::#new_or_try_new_token_stream);
                        }
                        #maybe_accumulator_push_none_token_stream
                        #additonal_token_stream
                        accumulator_0b59a062
                    })}
                };
                match &pg_type_pattern {
                    PgTypePattern::Standard => match &is_nullable {
                        pg_crud_macros_common::IsNullable::False => match &can_be_primary_key {
                            CanBePrimaryKey::False => Some({
                                let ts = generate_standard_non_null_test_case_handle_token_stream(&IsNeedToUseInto::False);
                                let new_or_try_new_token_stream = {
                                    let self_as_pg_type_create_token_stream = quote::quote! {#self_as_pg_type_token_stream::Create};
                                    if pg_type_initialization_try_new_try_from_pg_type.is_ok() {
                                        quote::quote! {
                                            |element_043a7d30|#self_as_pg_type_create_token_stream::try_new(
                                                element_043a7d30
                                            ).expect("941bd15c")
                                        }
                                    } else {
                                        quote::quote! {#self_as_pg_type_create_token_stream::#new_snake_case}
                                    }
                                };
                                quote::quote! {Some(
                                    #ts.into_iter().map(
                                        #new_or_try_new_token_stream
                                    ).collect()
                                )}
                            }),
                            CanBePrimaryKey::True => None,
                        },
                        pg_crud_macros_common::IsNullable::True => Some(generate_some_accumulator_token_stream(is_nullable, &generate_identifier_token_stream(pg_type, &pg_crud_macros_common::IsNullable::False, &PgTypePattern::Standard), &proc_macro2::TokenStream::new())),
                    },
                }
            };
            let read_ids_to_2_dimensions_vec_read_inner_token_stream = {
                match &is_nullable {
                    pg_crud_macros_common::IsNullable::False => {
                        let ts = generate_standard_non_null_test_case_handle_token_stream(&IsNeedToUseInto::True);
                        quote::quote! {vec![{#ts}]}
                    }
                    pg_crud_macros_common::IsNullable::True => quote::quote! {{
                        let read_ids_to_2_dimensions_vec_read_inner_4a2fae01 = #identifier_standard_non_null_as_pg_type_test_cases_token_stream::#read_ids_to_2_dimensions_vec_read_inner_snake_case(#read_ids_snake_case);
                        let mut accumulator_4a2fae01 = Vec::with_capacity(
                            read_ids_to_2_dimensions_vec_read_inner_4a2fae01
                                .iter()
                                .map(Vec::len)
                                .sum::<usize>()
                                .saturating_add(1)
                        );
                        for el0_4a2fae01 in read_ids_to_2_dimensions_vec_read_inner_4a2fae01 {
                            for el1_4a2fae01 in el0_4a2fae01 {
                                accumulator_4a2fae01.push(vec![Some(el1_4a2fae01)]);
                            }
                        }
                        accumulator_4a2fae01.push(vec![None]);
                        accumulator_4a2fae01
                    }},
                }
            };
            let read_inner_into_read_with_new_or_try_new_unwraped_token_stream = generate_read_or_read_inner_into_update_with_new_or_try_new_unwraped_token_stream(&pg_crud_macros_common::ReadOrUpdate::Read);
            let read_inner_into_update_with_new_or_try_new_unwraped_token_stream = generate_read_or_read_inner_into_update_with_new_or_try_new_unwraped_token_stream(&pg_crud_macros_common::ReadOrUpdate::Update);
            let update_to_read_ids_token_stream = if matches!(&is_non_null_standard_can_be_primary_key, IsNonNullStandardCanBePrimaryKey::True) {
                quote::quote! {
                    #identifier_read_ids_upper_camel_case(#identifier_read_upper_camel_case(#v_snake_case.0 #maybe_dot_clone_token_stream))//todo its not correct. must be only for primary_key but it for all types what van be primary_key
                }
            } else {
                let ts = generate_v_initialization_ts0(&none_token_stream);
                quote::quote! {
                    #import_non_primary_key_pg_type_read_ids_token_stream::from(#ts)
                }
            };
            let read_ids_to_optional_v_read_default_some_one_element_token_stream = {
                //todo that is not correct for array of generated by pg pks but maybe just need to remove this variants and thats it?
                let ts = generate_v_initialization_ts0(&{
                    let ts: &dyn quote::ToTokens = if matches!(&is_non_null_standard_can_be_primary_key, IsNonNullStandardCanBePrimaryKey::True) {
                        &quote::quote! {#v_snake_case.0 #maybe_dot_clone_token_stream}
                    } else {
                        &pg_crud_common_default_some_one_element_call
                    };
                    quote::quote! {#self_pg_type_as_pg_type_token_stream::normalize(#ts)}
                });
                quote::quote! {Some(#ts)}
            };
            let previous_read_and_optional_update_into_read_token_stream = quote::quote! {
                #optional_update_snake_case.map_or(#read_snake_case, |#v_snake_case| #identifier_read_upper_camel_case(#v_snake_case.0))
            };
            let read_ids_and_create_into_read_token_stream = {
                let ts = if matches!(&is_non_null_standard_can_be_primary_key, IsNonNullStandardCanBePrimaryKey::True) {
                    quote::quote! {#read_ids_snake_case.0}
                } else {
                    quote::quote! {#identifier_read_upper_camel_case(#create_snake_case.0)}
                };
                quote::quote! {
                    #self_pg_type_as_pg_type_token_stream::normalize(#ts)
                }
            };
            let read_ids_and_create_into_optional_v_read_token_stream = {
                let ts = generate_v_initialization_ts0(&quote::quote! {
                    <Self as #import::PgTypeTestCases>::#read_ids_and_create_into_read_snake_case(
                        #read_ids_snake_case,
                        #create_snake_case
                    )
                });
                quote::quote! {Some(#ts)}
            };
            let read_ids_and_create_into_table_type_token_stream = {
                let ts = if matches!(&is_non_null_standard_can_be_primary_key, IsNonNullStandardCanBePrimaryKey::True) {
                    quote::quote! {#read_ids_snake_case.0.0}
                } else {
                    quote::quote! {#create_snake_case.0}
                };
                quote::quote! {#identifier_table_type_upper_camel_case(#ts)}
            };
            //todo maybe it into fn (not in proc macro)
            let read_ids_and_create_into_where_eq_token_stream = {
                let ts = if matches!(&pg_type_pattern, PgTypePattern::Standard)
                    && matches!(&is_nullable, pg_crud_macros_common::IsNullable::False)
                    && matches!(&is_non_null_standard_can_be_primary_key, IsNonNullStandardCanBePrimaryKey::True)
                {
                    quote::quote! {#read_ids_snake_case.0.0}
                } else {
                    quote::quote! {#create_snake_case.0}
                };
                quote::quote! {
                    #identifier_where_upper_camel_case::#eq_upper_camel_case(where_filters::PgTypeWhereEq {
                        operator: #import::Operator::Or,
                        #v_snake_case: #identifier_table_type_upper_camel_case(#ts),
                    })
                }
            };
            let read_ids_and_create_into_vec_where_eq_using_fields_token_stream = quote::quote! {
                #import::NotEmptyUniqueVec::try_new(vec![
                    #read_ids_and_create_into_where_eq_token_stream
                ].into()).expect("4c08b551")
            };
            let read_ids_and_create_into_optional_vec_where_eq_to_field_token_stream: Option<proc_macro2::TokenStream> = None;
            let pg_type_optional_vec_where_greater_than_test_token_stream: Option<proc_macro2::TokenStream> = {
                let greater_than = pg_crud_common::PgTypeGreaterThanVariant::GreaterThan;
                let not_greater_than = pg_crud_common::PgTypeGreaterThanVariant::NotGreaterThan;
                let eq_not_greater_than = pg_crud_common::PgTypeGreaterThanVariant::EqNotGreaterThan;
                let generate_greater_than_test_token_stream = |greater_than_variant: &pg_crud_common::PgTypeGreaterThanVariant, create_token_stream: &dyn quote::ToTokens, table_type_token_stream: &dyn quote::ToTokens| {
                    let greater_than_variant_token_stream =
                        macros_helpers::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream::from(
                            match greater_than_variant {
                                pg_crud_common::PgTypeGreaterThanVariant::EqNotGreaterThan => {
                                    quote::quote! { EqNotGreaterThan }
                                }
                                pg_crud_common::PgTypeGreaterThanVariant::GreaterThan => {
                                    quote::quote! { GreaterThan }
                                }
                                pg_crud_common::PgTypeGreaterThanVariant::NotGreaterThan => {
                                    quote::quote! { NotGreaterThan }
                                }
                            },
                        );
                    quote::quote! {
                        #import::PgTypeGreaterThanTest {
                            variant: #import::PgTypeGreaterThanVariant::#greater_than_variant_token_stream,
                            create: #self_as_pg_type_token_stream::Create::#create_token_stream,
                            greater_than: #self_as_pg_type_token_stream::TableType::#table_type_token_stream,
                        }
                    }
                };
                let generate_greater_than_test_new_new_token_stream =
                    |greater_than_variant_token_stream: &pg_crud_common::PgTypeGreaterThanVariant, create_token_stream: &dyn quote::ToTokens, greater_than_token_stream: &dyn quote::ToTokens| generate_greater_than_test_token_stream(greater_than_variant_token_stream, &quote::quote! {new(#create_token_stream)}, &quote::quote! {new(#greater_than_token_stream)});
                let generate_greater_than_test_try_new_try_new_token_stream = |greater_than_variant_token_stream: &pg_crud_common::PgTypeGreaterThanVariant, create_token_stream: &dyn quote::ToTokens, greater_than_token_stream: &dyn quote::ToTokens| {
                    generate_greater_than_test_token_stream(
                        greater_than_variant_token_stream,
                        &quote::quote! {try_new(#create_token_stream).expect("8327c651")},
                        &quote::quote! {try_new(#greater_than_token_stream).expect("c369e6ea")},
                    )
                };
                let generate_greater_than_test_vec_token_stream = |
                    generate_token_stream: &dyn Fn(&pg_crud_common::PgTypeGreaterThanVariant, &dyn quote::ToTokens, &dyn quote::ToTokens) -> proc_macro2::TokenStream,
                    less_token_stream: &dyn quote::ToTokens,
                    less_with_more_token_stream: &dyn quote::ToTokens,
                    zero_token_stream: &dyn quote::ToTokens,
                    one_token_stream: &dyn quote::ToTokens,
                    more_token_stream: &dyn quote::ToTokens,
                    more_with_less_token_stream: &dyn quote::ToTokens
                | {
                    let greater_than_less_token_stream = generate_token_stream(&greater_than, &less_with_more_token_stream, &less_token_stream);
                    let greater_than_zero_token_stream = generate_token_stream(&greater_than, &one_token_stream, &zero_token_stream);
                    let greater_than_more_token_stream = generate_token_stream(&greater_than, &more_token_stream, &more_with_less_token_stream);
                    let not_greater_than_less_token_stream = generate_token_stream(&not_greater_than, &less_token_stream, &less_with_more_token_stream);
                    let not_greater_than_zero_token_stream = generate_token_stream(&not_greater_than, &zero_token_stream, &one_token_stream);
                    let not_greater_than_more_token_stream = generate_token_stream(&not_greater_than, &more_with_less_token_stream, &more_token_stream);
                    let eq_not_greater_than_less_token_stream = generate_token_stream(&eq_not_greater_than, &less_token_stream, &less_token_stream);
                    let eq_not_greater_than_zero_token_stream = generate_token_stream(&eq_not_greater_than, &zero_token_stream, &zero_token_stream);
                    let eq_not_greater_than_more_token_stream = generate_token_stream(&eq_not_greater_than, &more_token_stream, &more_token_stream);
                    quote::quote! {
                        #greater_than_less_token_stream,
                        #greater_than_zero_token_stream,
                        #greater_than_more_token_stream,
                        #not_greater_than_less_token_stream,
                        #not_greater_than_zero_token_stream,
                        #not_greater_than_more_token_stream,
                        #eq_not_greater_than_less_token_stream,
                        #eq_not_greater_than_zero_token_stream,
                        #eq_not_greater_than_more_token_stream
                    }
                };
                let generate_greater_than_test_new_new_vec_token_stream = |
                    less_token_stream: &dyn quote::ToTokens,
                    less_with_more_token_stream: &dyn quote::ToTokens,
                    zero_token_stream: &dyn quote::ToTokens,
                    one_token_stream: &dyn quote::ToTokens,
                    more_token_stream: &dyn quote::ToTokens,
                    more_with_less_token_stream: &dyn quote::ToTokens
                | generate_greater_than_test_vec_token_stream(&generate_greater_than_test_new_new_token_stream, less_token_stream, less_with_more_token_stream, zero_token_stream, one_token_stream, more_token_stream, more_with_less_token_stream);
                let generate_greater_than_test_try_new_try_new_vec_token_stream = |
                    less_token_stream: &dyn quote::ToTokens,
                    less_with_more_token_stream: &dyn quote::ToTokens,
                    zero_token_stream: &dyn quote::ToTokens,
                    one_token_stream: &dyn quote::ToTokens,
                    more_token_stream: &dyn quote::ToTokens,
                    more_with_less_token_stream: &dyn quote::ToTokens
                | generate_greater_than_test_vec_token_stream(&generate_greater_than_test_try_new_try_new_token_stream, less_token_stream, less_with_more_token_stream, zero_token_stream, one_token_stream, more_token_stream, more_with_less_token_stream);
                match &pg_type_pattern {
                    PgTypePattern::Standard => match &is_nullable {
                        pg_crud_macros_common::IsNullable::False => {
                            let wrap_into_not_empty_unique_vec_token_stream = |ts: &dyn quote::ToTokens| Some(quote::quote! {Some(
                                #import::NotEmptyUniqueVec::try_new(vec![#ts].into()).expect("3ad4b6bf")
                            )});
                            let sqlx_types_chrono_naive_time_as_time_standard_non_null_token_stream = &generate_identifier_token_stream(
                                &PgType::SqlxTypesChronoNaiveTimeAsTime,
                                &pg_crud_macros_common::IsNullable::False,
                                &PgTypePattern::Standard
                            );
                            let sqlx_types_chrono_naive_date_as_date_standard_non_null_token_stream = &generate_identifier_token_stream(
                                &PgType::SqlxTypesChronoNaiveDateAsDate,
                                &pg_crud_macros_common::IsNullable::False,
                                &PgTypePattern::Standard
                            );
                            match &pg_type {
                                PgType::I16AsInt2 => wrap_into_not_empty_unique_vec_token_stream(&generate_greater_than_test_new_new_vec_token_stream(
                                    &quote::quote! {#i16_token_stream::MIN},
                                    &quote::quote! {#i16_token_stream::MIN + 1},
                                    &quote::quote! {0},
                                    &quote::quote! {1},
                                    &quote::quote! {#i16_token_stream::MAX},
                                    &quote::quote! {#i16_token_stream::MAX - 1}
                                )),
                                PgType::I32AsInt4 => wrap_into_not_empty_unique_vec_token_stream(&generate_greater_than_test_new_new_vec_token_stream(
                                    &quote::quote! {#i32_token_stream::MIN},
                                    &quote::quote! {#i32_token_stream::MIN + 1},
                                    &quote::quote! {0},
                                    &quote::quote! {1},
                                    &quote::quote! {#i32_token_stream::MAX},
                                    &quote::quote! {#i32_token_stream::MAX - 1}
                                )),
                                PgType::I64AsInt8 => wrap_into_not_empty_unique_vec_token_stream(&generate_greater_than_test_new_new_vec_token_stream(
                                    &quote::quote! {#i64_token_stream::MIN},
                                    &quote::quote! {#i64_token_stream::MIN + 1},
                                    &quote::quote! {0},
                                    &quote::quote! {1},
                                    &quote::quote! {#i64_token_stream::MAX},
                                    &quote::quote! {#i64_token_stream::MAX - 1}
                                )),
                                PgType::F32AsFloat4 => wrap_into_not_empty_unique_vec_token_stream(&generate_greater_than_test_new_new_vec_token_stream(
                                    &quote::quote! {#f32_token_stream::MIN},
                                    &quote::quote! {#f32_token_stream::MIN.next_up()},
                                    &quote::quote! {0.0},
                                    &quote::quote! {1.0},
                                    &quote::quote! {#f32_token_stream::MAX},
                                    &quote::quote! {#f32_token_stream::MAX.next_down()}
                                )),
                                PgType::F64AsFloat8 => wrap_into_not_empty_unique_vec_token_stream(&generate_greater_than_test_try_new_try_new_vec_token_stream(
                                //todo rust f64 != pg float8
                                    &quote::quote! {-2.0},
                                    &quote::quote! {-2.0 + 1.0},
                                    &quote::quote! {0.0},
                                    &quote::quote! {1.0},
                                    &quote::quote! {2.0},
                                    &quote::quote! {2.0 - 1.0}
                                )),
                                PgType::SqlxTypesChronoNaiveTimeAsTime => wrap_into_not_empty_unique_vec_token_stream(&generate_greater_than_test_try_new_try_new_vec_token_stream(
                                    &quote::quote! {Self::min_inner_type()},
                                    &quote::quote! {Self::slightly_more_than_min_inner_type()},
                                    &quote::quote! {Self::middle_inner_type()},
                                    &quote::quote! {Self::slightly_more_than_middle_inner_type()},
                                    &quote::quote! {Self::max_inner_type()},
                                    &quote::quote! {Self::slightly_less_than_max_inner_type()},
                                )),
                                PgType::SqlxTypesTimeTimeAsTime => wrap_into_not_empty_unique_vec_token_stream(&generate_greater_than_test_try_new_try_new_vec_token_stream(
                                    &quote::quote! {Self::min_inner_type()},
                                    &quote::quote! {Self::slightly_more_than_min_inner_type()},
                                    &quote::quote! {Self::middle_inner_type()},
                                    &quote::quote! {Self::slightly_more_than_middle_inner_type()},
                                    &quote::quote! {sqlx::types::time::Time::from_hms_micro(23, 59, 59, 999_999).expect("f3d895bb")},
                                    &quote::quote! {sqlx::types::time::Time::from_hms_micro(23, 59, 59, 999_998).expect("1e71f8c6")},
                                )),
                                PgType::SqlxTypesChronoNaiveDateAsDate => wrap_into_not_empty_unique_vec_token_stream(&generate_greater_than_test_try_new_try_new_vec_token_stream(
                                    &quote::quote! {sqlx::types::chrono::NaiveDate::from_ymd_opt(-4712, 12, 30)?},//todo not sure about this values. maybe reuse
                                    &quote::quote! {sqlx::types::chrono::NaiveDate::from_ymd_opt(-4712, 12, 31)?},
                                    &quote::quote! {Self::middle_inner_type()},
                                    &quote::quote! {sqlx::types::chrono::NaiveDate::from_ymd_opt(0, 1, 2)?},
                                    &quote::quote! {Self::max_inner_type()},
                                    &quote::quote! {sqlx::types::chrono::NaiveDate::from_ymd_opt(262_142, 12, 30)?},
                                )),
                                PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp => wrap_into_not_empty_unique_vec_token_stream(&generate_greater_than_test_try_new_try_new_vec_token_stream(
                                    &quote::quote! {sqlx::types::chrono::NaiveDateTime::new(
                                        sqlx::types::chrono::NaiveDate::from_ymd_opt(-4713, 12, 31)?,
                                        #sqlx_types_chrono_naive_time_as_time_standard_non_null_token_stream::min_inner_type()
                                    )},
                                    &quote::quote! {sqlx::types::chrono::NaiveDateTime::new(
                                        sqlx::types::chrono::NaiveDate::from_ymd_opt(-4713, 12, 31)?,
                                        #sqlx_types_chrono_naive_time_as_time_standard_non_null_token_stream::slightly_more_than_min_inner_type()
                                    )},
                                    &quote::quote! {sqlx::types::chrono::NaiveDateTime::new(
                                        #sqlx_types_chrono_naive_date_as_date_standard_non_null_token_stream::middle_inner_type(),
                                        #sqlx_types_chrono_naive_time_as_time_standard_non_null_token_stream::min_inner_type()
                                    )},
                                    &quote::quote! {sqlx::types::chrono::NaiveDateTime::new(
                                        #sqlx_types_chrono_naive_date_as_date_standard_non_null_token_stream::middle_inner_type(),
                                        #sqlx_types_chrono_naive_time_as_time_standard_non_null_token_stream::slightly_more_than_min_inner_type()
                                    )},
                                    &quote::quote! {sqlx::types::chrono::NaiveDateTime::new(
                                        sqlx::types::chrono::NaiveDate::MAX,
                                        #sqlx_types_chrono_naive_time_as_time_standard_non_null_token_stream::max_inner_type()
                                    )},
                                    &quote::quote! {sqlx::types::chrono::NaiveDateTime::new(
                                        sqlx::types::chrono::NaiveDate::MAX,
                                        #sqlx_types_chrono_naive_time_as_time_standard_non_null_token_stream::slightly_less_than_max_inner_type()
                                    )},
                                )),
                                PgType::I16AsSmallSerialInitializationByPg |//todo diffrent test logic for autogenerated?
                                PgType::I32AsSerialInitializationByPg |//todo diffrent test logic for autogenerated?
                                PgType::I64AsBigSerialInitializationByPg |//todo diffrent test logic for autogenerated?
                                PgType::SqlxPgTypesPgMoneyAsMoney |
                                PgType::BoolAsBool |
                                PgType::StringAsText |
                                PgType::StdVecVecU8AsBytea |
                                PgType::SqlxPgTypesPgIntervalAsInterval |
                                PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz |
                                PgType::SqlxTypesUuidUuidAsUuidV4InitializationByPg |
                                PgType::SqlxTypesUuidUuidAsUuidInitializationByClient |
                                PgType::SqlxTypesIpnetworkIpNetworkAsInet |
                                PgType::SqlxTypesMacAddressMacAddressAsMacAddr |
                                PgType::SqlxPgTypesPgRangeI32AsInt4Range |
                                PgType::SqlxPgTypesPgRangeI64AsInt8Range |
                                PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange |
                                PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange |
                                PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => None,
                            }
                        }
                        pg_crud_macros_common::IsNullable::True => Some(quote::quote! {
                            <#identifier_standard_non_null_upper_camel_case as #import::PgTypeTestCases>::pg_type_optional_vec_where_greater_than_test().map(
                                |element_e4af7fd9|
                                #import::NotEmptyUniqueVec::try_new(
                                    element_e4af7fd9
                                    .into_vec()
                                    .into_iter()
                                    .map(|element_504739e6| #import::PgTypeGreaterThanTest {
                                        variant: element_504739e6.variant,
                                        create: #identifier_create_upper_camel_case(#identifier_origin_upper_camel_case(Some(element_504739e6.create.0))),
                                        greater_than: #identifier_table_type_upper_camel_case(#identifier_origin_upper_camel_case(Some(element_504739e6.greater_than.0))),
                                    })
                                    .collect::<Vec<_>>()
                                    .into()
                                ).expect("63ce5df3")
                            )
                        }),
                    },
                }
            };
            let read_ids_and_table_type_into_pg_type_optional_where_greater_than_token_stream: Option<proc_macro2::TokenStream> = match &pg_type_pattern {
                PgTypePattern::Standard => {
                    enum IsNeedToImplPgTypeGreaterThanTest {
                        False,
                        TrueFromCreate,
                        TrueFromReadIds,
                    }
                    enum CreateReadIds {
                        Create,
                        ReadIds,
                    }
                    let is_need_to_impl_greater_than_test = match &pg_type {
                        PgType::I16AsInt2 |
                        PgType::I32AsInt4 |
                        PgType::I64AsInt8 |
                        PgType::F32AsFloat4 |
                        PgType::F64AsFloat8 |
                        PgType::SqlxTypesChronoNaiveTimeAsTime |
                        PgType::SqlxTypesTimeTimeAsTime |
                        PgType::SqlxTypesChronoNaiveDateAsDate |
                        PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp => IsNeedToImplPgTypeGreaterThanTest::TrueFromCreate,
                        PgType::I16AsSmallSerialInitializationByPg |
                        PgType::I32AsSerialInitializationByPg |
                        PgType::I64AsBigSerialInitializationByPg => IsNeedToImplPgTypeGreaterThanTest::TrueFromReadIds,
                        PgType::SqlxPgTypesPgMoneyAsMoney |//todo why no support?
                        PgType::BoolAsBool |
                        PgType::StringAsText |
                        PgType::StdVecVecU8AsBytea |
                        PgType::SqlxPgTypesPgIntervalAsInterval |
                        PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz |//todo why no support?
                        PgType::SqlxTypesUuidUuidAsUuidV4InitializationByPg |
                        PgType::SqlxTypesUuidUuidAsUuidInitializationByClient |
                        PgType::SqlxTypesIpnetworkIpNetworkAsInet |
                        PgType::SqlxTypesMacAddressMacAddressAsMacAddr |
                        PgType::SqlxPgTypesPgRangeI32AsInt4Range |
                        PgType::SqlxPgTypesPgRangeI64AsInt8Range |
                        PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange |
                        PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange |
                        PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => IsNeedToImplPgTypeGreaterThanTest::False,
                    };
                    let generate_some_token_stream = |create_read_ids_parameter: &CreateReadIds| match &is_nullable {
                        pg_crud_macros_common::IsNullable::False => {
                            let ts = match &create_read_ids_parameter {
                                CreateReadIds::ReadIds => quote::quote! {#identifier_standard_non_null_table_type_upper_camel_case(#read_ids_snake_case.0.0)},
                                CreateReadIds::Create => quote::quote! {table_type},
                            };
                            quote::quote! {Some(#identifier_where_upper_camel_case::GreaterThan(
                                where_filters::PgTypeWhereGreaterThan {
                                    operator: greater_than_variant.operator(),
                                    #v_snake_case: #ts,
                                }
                            ))}
                        }
                        pg_crud_macros_common::IsNullable::True => {
                            let ts = match &create_read_ids_parameter {
                                CreateReadIds::ReadIds => quote::quote! {#read_ids_snake_case.0},
                                CreateReadIds::Create => quote::quote! {#table_type_snake_case.0.0},
                            };
                            quote::quote! {
                                #ts.map(|element_886032ca| #identifier_where_upper_camel_case::GreaterThan(where_filters::PgTypeWhereGreaterThan {
                                    operator: greater_than_variant.operator(),
                                    #v_snake_case: #identifier_standard_non_null_table_type_upper_camel_case(element_886032ca),
                                }))
                            }
                        }
                    };
                    match &is_need_to_impl_greater_than_test {
                        IsNeedToImplPgTypeGreaterThanTest::TrueFromReadIds => Some(generate_some_token_stream(&CreateReadIds::ReadIds)),
                        IsNeedToImplPgTypeGreaterThanTest::TrueFromCreate => Some(generate_some_token_stream(&CreateReadIds::Create)),
                        IsNeedToImplPgTypeGreaterThanTest::False => None,
                    }
                }
            };
            let optional_vec_create_to_tokens: Option<&dyn quote::ToTokens> =
                optional_vec_create_token_stream.as_ref().map(|v| {
                    let v_ref: &dyn quote::ToTokens = v;
                    v_ref
                });
            let read_ids_and_create_into_optional_vec_where_eq_to_field_to_tokens: Option<&dyn quote::ToTokens> =
                read_ids_and_create_into_optional_vec_where_eq_to_field_token_stream
                    .as_ref()
                    .map(|v| {
                        let v_ref: &dyn quote::ToTokens = v;
                        v_ref
                    });
            let pg_type_optional_vec_where_greater_than_test_to_tokens: Option<&dyn quote::ToTokens> =
                pg_type_optional_vec_where_greater_than_test_token_stream.as_ref().map(|v| {
                    let v_ref: &dyn quote::ToTokens = v;
                    v_ref
                });
            let read_ids_and_table_type_into_pg_type_optional_where_greater_than_to_tokens: Option<
                &dyn quote::ToTokens,
            > = read_ids_and_table_type_into_pg_type_optional_where_greater_than_token_stream
                .as_ref()
                .map(|v| {
                    let v_ref: &dyn quote::ToTokens = v;
                    v_ref
                });
            pg_crud_macros_common::pg_type_test_cases::generate_impl_pg_type_test_cases_for_identifier_token_stream(
                &quote::quote! {#[cfg(feature = "test-utils")]},
                &import,
                &identifier_inner_type_token_stream,
                &identifier,
                optional_vec_create_to_tokens,
                &read_ids_to_2_dimensions_vec_read_inner_token_stream,
                &read_inner_into_read_with_new_or_try_new_unwraped_token_stream,
                &read_inner_into_update_with_new_or_try_new_unwraped_token_stream,
                &update_to_read_ids_token_stream,
                &read_ids_to_optional_v_read_default_some_one_element_token_stream,
                &previous_read_and_optional_update_into_read_token_stream,
                &read_ids_and_create_into_read_token_stream,
                &read_ids_and_create_into_optional_v_read_token_stream,
                &read_ids_and_create_into_table_type_token_stream,
                &read_ids_and_create_into_where_eq_token_stream,
                &read_ids_and_create_into_vec_where_eq_using_fields_token_stream,
                read_ids_and_create_into_optional_vec_where_eq_to_field_to_tokens,
                pg_type_optional_vec_where_greater_than_test_to_tokens,
                read_ids_and_table_type_into_pg_type_optional_where_greater_than_to_tokens,
            )
        };
        let maybe_impl_pg_type_primary_key_for_identifier_standard_non_null_if_can_be_primary_key_token_stream = if matches!(&is_non_null_standard_can_be_primary_key, IsNonNullStandardCanBePrimaryKey::True) {
            let v_as_read_ids_token_stream = quote::quote! {#v_snake_case: #self_as_pg_type_token_stream::#read_ids_upper_camel_case};
            quote::quote! {
                #allow_clippy_arbitrary_src_item_ordering
                impl #import::#pg_type_primary_key_upper_camel_case for #identifier_standard_non_null_upper_camel_case {
                    type #pg_type_upper_camel_case = Self;
                    type #table_type_upper_camel_case = #identifier_standard_non_null_table_type_upper_camel_case;
                    fn #read_ids_into_table_type_snake_case(#v_as_read_ids_token_stream) -> #self_as_pg_type_token_stream::#table_type_upper_camel_case {
                        #identifier_table_type_upper_camel_case(#v_snake_case.0.0)
                    }
                    fn #read_ids_into_read_snake_case(#v_as_read_ids_token_stream) -> #self_as_pg_type_token_stream::#read_upper_camel_case {
                        #v_snake_case.0
                    }
                    fn #read_ids_into_update_snake_case(#v_as_read_ids_token_stream) -> #self_as_pg_type_token_stream::#update_upper_camel_case {
                        #identifier_update_upper_camel_case(#v_snake_case.0.0)
                    }
                    fn #read_into_table_type_snake_case(
                        #v_snake_case: #self_as_pg_type_token_stream::#read_upper_camel_case
                    ) -> #self_as_pg_type_token_stream::#table_type_upper_camel_case {
                        #identifier_table_type_upper_camel_case(#v_snake_case.0)
                    }
                }
            }
        } else {
            proc_macro2::TokenStream::new()
        };
        let maybe_impl_pg_type_not_primary_key_for_identifier_token_stream = if matches!(&is_non_null_standard_can_be_primary_key, IsNonNullStandardCanBePrimaryKey::True) {
            macros_helpers::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream::from(proc_macro2::TokenStream::new())
        } else {
            pg_crud_macros_common::generate_impl_pg_type_not_primary_key_for_identifier_token_stream(&import, &identifier)
        };
        let frontend_nullability_token_stream = match &is_nullable {
            pg_crud_macros_common::IsNullable::False => quote::quote! {frontend_contract::Nullability::NonNullable},
            pg_crud_macros_common::IsNullable::True => quote::quote! {frontend_contract::Nullability::Nullable},
        };
        let db_nullable = matches!(is_nullable, pg_crud_macros_common::IsNullable::True);
        let db_data_type = match pg_type {
            PgType::I16AsSmallSerialInitializationByPg =>
                PgSqlName::from(str_constants::PG_CRUD_PG_INT2),
            PgType::I32AsSerialInitializationByPg =>
                PgSqlName::from(str_constants::PG_CRUD_PG_INT4),
            PgType::I64AsBigSerialInitializationByPg =>
                PgSqlName::from(str_constants::PG_CRUD_PG_INT8),
            PgType::BoolAsBool
            | PgType::F32AsFloat4
            | PgType::F64AsFloat8
            | PgType::I16AsInt2
            | PgType::I32AsInt4
            | PgType::I64AsInt8
            | PgType::SqlxPgTypesPgIntervalAsInterval
            | PgType::SqlxPgTypesPgMoneyAsMoney
            | PgType::SqlxPgTypesPgRangeI32AsInt4Range
            | PgType::SqlxPgTypesPgRangeI64AsInt8Range
            | PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange
            | PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange
            | PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange
            | PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz
            | PgType::SqlxTypesChronoNaiveDateAsDate
            | PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp
            | PgType::SqlxTypesChronoNaiveTimeAsTime
            | PgType::SqlxTypesIpnetworkIpNetworkAsInet
            | PgType::SqlxTypesMacAddressMacAddressAsMacAddr
            | PgType::SqlxTypesTimeTimeAsTime
            | PgType::SqlxTypesUuidUuidAsUuidInitializationByClient
            | PgType::SqlxTypesUuidUuidAsUuidV4InitializationByPg
            | PgType::StdVecVecU8AsBytea
            | PgType::StringAsText => crate::catalog::pg_name(pg_type_dsc),
        };
        let db_has_server_default = matches!(
            pg_type,
            PgType::I16AsSmallSerialInitializationByPg
                | PgType::I32AsSerialInitializationByPg
                | PgType::I64AsBigSerialInitializationByPg
                | PgType::SqlxTypesUuidUuidAsUuidV4InitializationByPg
        );
        let (frontend_input_kind_token_stream, frontend_value_format_token_stream, frontend_step_token_stream, frontend_example_token_stream) = match crate::rust_type::wire_kind(pg_type_dsc) {
            WireKind::Bool => (quote::quote! {frontend_contract::InputKind::Checkbox}, quote::quote! {frontend_contract::ValueFormat::Bool}, quote::quote! {frontend_contract::InputStep::Any}, quote::quote! {frontend_contract::ValueExample::Boolean}),
            WireKind::Bytes => (quote::quote! {frontend_contract::InputKind::Text}, quote::quote! {frontend_contract::ValueFormat::Bytes}, quote::quote! {frontend_contract::InputStep::Any}, quote::quote! {frontend_contract::ValueExample::Text}),
            WireKind::Date => (quote::quote! {frontend_contract::InputKind::Date}, quote::quote! {frontend_contract::ValueFormat::Date}, quote::quote! {frontend_contract::InputStep::Any}, quote::quote! {frontend_contract::ValueExample::Date}),
            WireKind::Float32 => (quote::quote! {frontend_contract::InputKind::Number}, quote::quote! {frontend_contract::ValueFormat::Float32}, quote::quote! {frontend_contract::InputStep::Decimal}, quote::quote! {frontend_contract::ValueExample::Decimal}),
            WireKind::Float64 => (quote::quote! {frontend_contract::InputKind::Number}, quote::quote! {frontend_contract::ValueFormat::Float64}, quote::quote! {frontend_contract::InputStep::Decimal}, quote::quote! {frontend_contract::ValueExample::Decimal}),
            WireKind::Inet => (quote::quote! {frontend_contract::InputKind::Text}, quote::quote! {frontend_contract::ValueFormat::Inet}, quote::quote! {frontend_contract::InputStep::Any}, quote::quote! {frontend_contract::ValueExample::Text}),
            WireKind::Int16 => (quote::quote! {frontend_contract::InputKind::Number}, quote::quote! {frontend_contract::ValueFormat::Int16}, quote::quote! {frontend_contract::InputStep::Integer}, quote::quote! {frontend_contract::ValueExample::Integer}),
            WireKind::Int32 => (quote::quote! {frontend_contract::InputKind::Number}, quote::quote! {frontend_contract::ValueFormat::Int32}, quote::quote! {frontend_contract::InputStep::Integer}, quote::quote! {frontend_contract::ValueExample::Integer}),
            WireKind::Int64 => (quote::quote! {frontend_contract::InputKind::Number}, quote::quote! {frontend_contract::ValueFormat::Int64}, quote::quote! {frontend_contract::InputStep::Integer}, quote::quote! {frontend_contract::ValueExample::Integer}),
            WireKind::Interval => (quote::quote! {frontend_contract::InputKind::Text}, quote::quote! {frontend_contract::ValueFormat::Interval}, quote::quote! {frontend_contract::InputStep::Any}, quote::quote! {frontend_contract::ValueExample::Text}),
            WireKind::Mac => (quote::quote! {frontend_contract::InputKind::Text}, quote::quote! {frontend_contract::ValueFormat::Mac}, quote::quote! {frontend_contract::InputStep::Any}, quote::quote! {frontend_contract::ValueExample::Text}),
            WireKind::RangeDate | WireKind::RangeInt32 | WireKind::RangeInt64 | WireKind::RangeTimestamp | WireKind::RangeTimestampTz => (quote::quote! {frontend_contract::InputKind::Text}, quote::quote! {frontend_contract::ValueFormat::Range}, quote::quote! {frontend_contract::InputStep::Any}, quote::quote! {frontend_contract::ValueExample::Text}),
            WireKind::String => (quote::quote! {frontend_contract::InputKind::Text}, quote::quote! {frontend_contract::ValueFormat::Text}, quote::quote! {frontend_contract::InputStep::Any}, quote::quote! {frontend_contract::ValueExample::Text}),
            WireKind::TimeChrono | WireKind::TimeTime => (quote::quote! {frontend_contract::InputKind::Time}, quote::quote! {frontend_contract::ValueFormat::Time}, quote::quote! {frontend_contract::InputStep::Any}, quote::quote! {frontend_contract::ValueExample::Time}),
            WireKind::Timestamp => (quote::quote! {frontend_contract::InputKind::DateTime}, quote::quote! {frontend_contract::ValueFormat::Timestamp}, quote::quote! {frontend_contract::InputStep::Any}, quote::quote! {frontend_contract::ValueExample::DateTime}),
            WireKind::TimestampTz => (quote::quote! {frontend_contract::InputKind::DateTime}, quote::quote! {frontend_contract::ValueFormat::TimestampTz}, quote::quote! {frontend_contract::InputStep::Any}, quote::quote! {frontend_contract::ValueExample::DateTime}),
            WireKind::Uuid => (quote::quote! {frontend_contract::InputKind::Uuid}, quote::quote! {frontend_contract::ValueFormat::Uuid}, quote::quote! {frontend_contract::InputStep::Any}, quote::quote! {frontend_contract::ValueExample::Uuid}),
        };
        let frontend_bounds_token_stream = match crate::rust_type::wire_kind(pg_type_dsc) {
            WireKind::Int16 => quote::quote! {.with_minimum(frontend_contract::NumericBound::Inclusive(frontend_contract::ContractI64::i16_min())).with_maximum(frontend_contract::NumericBound::Inclusive(frontend_contract::ContractI64::i16_max()))},
            WireKind::Int32 => quote::quote! {.with_minimum(frontend_contract::NumericBound::Inclusive(frontend_contract::ContractI64::i32_min())).with_maximum(frontend_contract::NumericBound::Inclusive(frontend_contract::ContractI64::i32_max()))},
            WireKind::Int64 => quote::quote! {.with_minimum(frontend_contract::NumericBound::Inclusive(frontend_contract::ContractI64::min())).with_maximum(frontend_contract::NumericBound::Inclusive(frontend_contract::ContractI64::max()))},
            WireKind::Bool | WireKind::Bytes | WireKind::Date | WireKind::Float32 | WireKind::Float64 | WireKind::Inet | WireKind::Interval | WireKind::Mac | WireKind::RangeDate | WireKind::RangeInt32 | WireKind::RangeInt64 | WireKind::RangeTimestamp | WireKind::RangeTimestampTz | WireKind::String | WireKind::TimeChrono | WireKind::TimeTime | WireKind::Timestamp | WireKind::TimestampTz | WireKind::Uuid => proc_macro2::TokenStream::new(),
        };
        let impl_frontend_type_contract_token_stream = quote::quote! {
            impl frontend_contract::HasTypeContract for #identifier {
                fn type_contract() -> frontend_contract::TypeContract {
                    frontend_contract::TypeContract::new(#frontend_input_kind_token_stream, #frontend_value_format_token_stream, #frontend_nullability_token_stream)
                        .with_step(#frontend_step_token_stream)
                        .with_example(#frontend_example_token_stream)
                        #frontend_bounds_token_stream
                }
            }
        };
        let impl_frontend_filter_contracts_token_stream = quote::quote! {
            impl frontend_contract::HasFilterContracts for #identifier {
                const FILTER_CONTRACTS: &'static [frontend_contract::FilterOperation] = &[#frontend_filter_contracts_token_stream];
            }
        };
        let impl_pg_column_schema_token_stream = quote::quote! {
            impl pg_crud_common::PgColumnSchema for #identifier {
                const HAS_SERVER_DEFAULT: bool = #db_has_server_default;
                const NULLABLE: bool = #db_nullable;
                fn data_type() -> pg_crud_common::DbStaticSchemaText {
                    pg_crud_common::DbStaticSchemaText::from(#db_data_type)
                }
            }
        };
        let frontend_time_json_token_stream = |time_value_token_stream: &dyn quote::ToTokens, minute_name, second_name, microsecond_name| {
            quote::quote! {{
                let mut parts = #time_value_token_stream.split(':');
                let hour = parts.next().ok_or_else(|| frontend_contract::FormValueError::try_from("time hour is missing".to_owned()).unwrap_or_default())?.parse::<u32>().map_err(|error| frontend_contract::FormValueError::try_from(error.to_string()).unwrap_or_default())?;
                let minute = parts.next().ok_or_else(|| frontend_contract::FormValueError::try_from("time minute is missing".to_owned()).unwrap_or_default())?.parse::<u32>().map_err(|error| frontend_contract::FormValueError::try_from(error.to_string()).unwrap_or_default())?;
                let second_and_fraction = parts.next().unwrap_or("0");
                if parts.next().is_some() {
                    return Err(frontend_contract::FormValueError::try_from("time contains too many components".to_owned()).unwrap_or_default());
                }
                let (second_text, fraction) = second_and_fraction
                    .split_once('.')
                    .unwrap_or((second_and_fraction, ""));
                let second = second_text.parse::<u32>().map_err(|error| frontend_contract::FormValueError::try_from(error.to_string()).unwrap_or_default())?;
                if fraction.len() > 6usize || !fraction.bytes().all(|byte| byte.is_ascii_digit()) {
                    return Err(frontend_contract::FormValueError::try_from("time fraction must contain at most six digits".to_owned()).unwrap_or_default());
                }
                let mut microsecond_text = fraction.to_owned();
                microsecond_text.extend(std::iter::repeat_n('0', 6usize.saturating_sub(microsecond_text.len())));
                let microsecond = microsecond_text.parse::<u32>().map_err(|error| frontend_contract::FormValueError::try_from(error.to_string()).unwrap_or_default())?;
                serde_json::json!({"hour": hour, #minute_name: minute, #second_name: second, #microsecond_name: microsecond})
            }}
        };
        let frontend_parse_json_value_token_stream = match crate::rust_type::wire_kind(pg_type_dsc) {
            WireKind::Date | WireKind::Inet | WireKind::Mac | WireKind::String | WireKind::Uuid => quote::quote! {serde_json::Value::String(value.as_ref().to_owned())},
            WireKind::TimeChrono => frontend_time_json_token_stream(&quote::quote! {value.as_ref()}, str_constants::MIN, str_constants::SEC, str_constants::MICRO),
            WireKind::TimeTime => frontend_time_json_token_stream(&quote::quote! {value.as_ref()}, str_constants::MINUTE, str_constants::SECOND_ALT, str_constants::MICROSECOND),
            WireKind::Timestamp | WireKind::TimestampTz => {
                let date_name = match crate::rust_type::wire_kind(pg_type_dsc) {
                    WireKind::Timestamp => str_constants::PG_CRUD_PG_DATE,
                    WireKind::TimestampTz => str_constants::DATE_NAIVE,
                    _ => unreachable!(),
                };
                let time_json_token_stream = frontend_time_json_token_stream(&quote::quote! {time}, str_constants::MIN, str_constants::SEC, str_constants::MICRO);
                quote::quote! {{
                    let (date, time) = value.as_ref().split_once('T').ok_or_else(|| frontend_contract::FormValueError::try_from("timestamp must contain `T` between date and time".to_owned()).unwrap_or_default())?;
                    let time = #time_json_token_stream;
                    serde_json::json!({#date_name: date, "time": time})
                }}
            },
            WireKind::Bool | WireKind::Bytes | WireKind::Float32 | WireKind::Float64 | WireKind::Int16 | WireKind::Int32 | WireKind::Int64 | WireKind::Interval | WireKind::RangeDate | WireKind::RangeInt32 | WireKind::RangeInt64 | WireKind::RangeTimestamp | WireKind::RangeTimestampTz => quote::quote! {serde_json::from_str::<serde_json::Value>(value.as_ref()).map_err(|error| frontend_contract::FormValueError::try_from(error.to_string()).unwrap_or_default())?},
        };
        let frontend_empty_value_token_stream = match &is_nullable {
            pg_crud_macros_common::IsNullable::False => quote::quote! {#frontend_parse_json_value_token_stream},
            pg_crud_macros_common::IsNullable::True => quote::quote! {
                if value.as_ref().is_empty() {
                    serde_json::Value::Null
                } else {
                    #frontend_parse_json_value_token_stream
                }
            },
        };
        let frontend_format_time_token_stream = |value_token_stream: &dyn quote::ToTokens, minute_name, second_name, microsecond_name| quote::quote! {{
            let object = #value_token_stream.as_object().ok_or_else(|| frontend_contract::FormValueError::try_from("time wire value must be an object".to_owned()).unwrap_or_default())?;
            let field = |name| object.get(name).and_then(serde_json::Value::as_u64).ok_or_else(|| frontend_contract::FormValueError::try_from(format!("time wire field `{name}` is missing")).unwrap_or_default());
            let hour = field("hour")?;
            let minute = field(#minute_name)?;
            let second = field(#second_name)?;
            let microsecond = field(#microsecond_name)?;
            let fraction = format!("{microsecond:06}").trim_end_matches('0').to_owned();
            if fraction.is_empty() {
                format!("{hour:02}:{minute:02}:{second:02}")
            } else {
                format!("{hour:02}:{minute:02}:{second:02}.{fraction}")
            }
        }};
        let frontend_format_value_token_stream = match crate::rust_type::wire_kind(pg_type_dsc) {
            WireKind::TimeChrono => frontend_format_time_token_stream(&quote::quote! {value}, str_constants::MIN, str_constants::SEC, str_constants::MICRO),
            WireKind::TimeTime => frontend_format_time_token_stream(&quote::quote! {value}, str_constants::MINUTE, str_constants::SECOND_ALT, str_constants::MICROSECOND),
            WireKind::Timestamp | WireKind::TimestampTz => {
                let date_name = match crate::rust_type::wire_kind(pg_type_dsc) {
                    WireKind::Timestamp => str_constants::PG_CRUD_PG_DATE,
                    WireKind::TimestampTz => str_constants::DATE_NAIVE,
                    _ => unreachable!(),
                };
                let time_token_stream = frontend_format_time_token_stream(&quote::quote! {time}, str_constants::MIN, str_constants::SEC, str_constants::MICRO);
                quote::quote! {{
                    let object = value.as_object().ok_or_else(|| frontend_contract::FormValueError::try_from("timestamp wire value must be an object".to_owned()).unwrap_or_default())?;
                    let date = object.get(#date_name).and_then(serde_json::Value::as_str).ok_or_else(|| frontend_contract::FormValueError::try_from("timestamp date wire field is missing".to_owned()).unwrap_or_default())?;
                    let time = object.get("time").ok_or_else(|| frontend_contract::FormValueError::try_from("timestamp time wire field is missing".to_owned()).unwrap_or_default())?;
                    let time = #time_token_stream;
                    format!("{date}T{time}")
                }}
            },
            WireKind::Bool | WireKind::Bytes | WireKind::Date | WireKind::Float32 | WireKind::Float64 | WireKind::Inet | WireKind::Int16 | WireKind::Int32 | WireKind::Int64 | WireKind::Interval | WireKind::Mac | WireKind::RangeDate | WireKind::RangeInt32 | WireKind::RangeInt64 | WireKind::RangeTimestamp | WireKind::RangeTimestampTz | WireKind::String | WireKind::Uuid => quote::quote! {
                match value {
                    serde_json::Value::Null => String::new(),
                    serde_json::Value::String(value) => value,
                    value => value.to_string(),
                }
            },
        };
        let impl_frontend_form_value_contract_token_stream = quote::quote! {
            impl frontend_contract::FormValueContract for #identifier_origin_upper_camel_case {
                fn format_form_value(&self) -> Result<frontend_contract::FormValue, frontend_contract::FormValueError> {
                    let value = serde_json::to_value(self).map_err(|error| frontend_contract::FormValueError::try_from(error.to_string()).unwrap_or_default())?;
                    frontend_contract::FormValue::try_from(#frontend_format_value_token_stream).map_err(|error| frontend_contract::FormValueError::try_from(error.to_string()).unwrap_or_default())
                }
                fn parse_form_value(value: frontend_contract::FormValueRef<'_>) -> Result<Self, frontend_contract::FormValueError> {
                    let json_value = #frontend_empty_value_token_stream;
                    serde_json::from_value(json_value).map_err(|error| frontend_contract::FormValueError::try_from(error.to_string()).unwrap_or_default())
                }
            }
            impl frontend_contract::FilterFormValueContract for #identifier {
                fn parse_filter_form_value(
                    value: frontend_contract::FormValueRef<'_>,
                ) -> Result<frontend_contract::FilterWireJson, frontend_contract::FormValueError> {
                    let parsed = <#identifier_origin_upper_camel_case as frontend_contract::FormValueContract>::parse_form_value(value)?;
                    let json = serde_json::to_string(&parsed)
                        .map_err(|error| frontend_contract::FormValueError::try_from(error.to_string()).unwrap_or_default())?;
                    frontend_contract::FilterWireJson::try_from(json)
                        .map_err(|error| frontend_contract::FormValueError::try_from(error.to_string()).unwrap_or_default())
                }
            }
        };
        let generated = quote::quote! {
            #identifier_token_stream
            #identifier_origin_token_stream
            #identifier_table_type_token_stream
            #identifier_create_token_stream
            #identifier_select_token_stream
            #identifier_where_token_stream
            #identifier_read_token_stream
            #identifier_read_ids_token_stream
            #identifier_read_inner_token_stream
            #identifier_update_token_stream
            #identifier_update_for_query_token_stream
            #impl_pg_type_for_identifier_token_stream
            #impl_pg_type_test_cases_for_identifier_token_stream
            #maybe_impl_pg_type_primary_key_for_identifier_standard_non_null_if_can_be_primary_key_token_stream
            #maybe_impl_pg_type_not_primary_key_for_identifier_token_stream
            #impl_frontend_type_contract_token_stream
            #impl_frontend_filter_contracts_token_stream
            #impl_pg_column_schema_token_stream
            #impl_frontend_form_value_contract_token_stream
        };
        (
            {
                let field = quote::format_ident!("column_{i}");
                quote::quote! {
                    pub #field: crate::#identifier,
                }
                .to_string()
            },
            generated.to_string(),
        )
    })
    .collect::<(Vec<String>, Vec<String>)>();
    if generate_pg_types_config.generate_secret_text.0 {
        pg_type_array.push(quote::quote! {
            /// Secret PostgreSQL text deliberately has no serialization contract.
            ///
            /// ```compile_fail
            /// fn assert_serialize<Value: serde::Serialize>() {}
            /// assert_serialize::<pg_types_text_misc::StringAsNonNullTextSecret>();
            /// ```
            #[derive(Clone, PartialEq, Eq)]
            pub struct StringAsNonNullTextSecret(String);
            impl From<String> for StringAsNonNullTextSecret {
                fn from(value: String) -> Self {
                    Self(value)
                }
            }
            impl std::fmt::Debug for StringAsNonNullTextSecret {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_str("[REDACTED]")
                }
            }

            impl AsRef<str> for StringAsNonNullTextSecret {
                fn as_ref(&self) -> &str {
                    self.0.as_str()
                }
            }
            impl std::borrow::Borrow<str> for StringAsNonNullTextSecret {
                fn borrow(&self) -> &str {
                    self.0.as_str()
                }
            }

            impl sqlx::Type<sqlx::Postgres> for StringAsNonNullTextSecret {
                fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
                    <String as sqlx::Type<sqlx::Postgres>>::type_info()
                }
                fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
                    <String as sqlx::Type<sqlx::Postgres>>::compatible(ty)
                }
            }
            impl<'query_lt> sqlx::Encode<'query_lt, sqlx::Postgres> for StringAsNonNullTextSecret {
                fn encode_by_ref(&self, buffer: &mut <sqlx::Postgres as sqlx::Database>::ArgumentBuffer) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
                    <String as sqlx::Encode<'query_lt, sqlx::Postgres>>::encode_by_ref(&self.0, buffer)
                }
                fn size_hint(&self) -> usize {
                    <String as sqlx::Encode<'query_lt, sqlx::Postgres>>::size_hint(&self.0)
                }
            }
            impl<'row_lt> sqlx::Decode<'row_lt, sqlx::Postgres> for StringAsNonNullTextSecret {
                fn decode(value: <sqlx::Postgres as sqlx::Database>::ValueRef<'row_lt>) -> Result<Self, sqlx::error::BoxDynError> {
                    <String as sqlx::Decode<'row_lt, sqlx::Postgres>>::decode(value).map(Self)
                }
            }
            #[derive(Clone, Copy)]
            pub struct StringAsNonNullTextSecretRef<'value_lt>(&'value_lt str);
            impl std::fmt::Debug for StringAsNonNullTextSecretRef<'_> {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_str("[REDACTED]")
                }
            }
            impl<'value_lt> From<&'value_lt StringAsNonNullTextSecret> for StringAsNonNullTextSecretRef<'value_lt> {
                fn from(value: &'value_lt StringAsNonNullTextSecret) -> Self {
                    Self(value.0.as_str())
                }
            }
            impl AsRef<str> for StringAsNonNullTextSecretRef<'_> {
                fn as_ref(&self) -> &str {
                    self.0
                }
            }
        }.to_string());
    }
    let parse_strs_to_ts2_vec = pg_crud_macros_common::token_stream_helpers::parse_strs_to_ts2_vec;
    let pg_table_cols_token_stream = {
        let ts = parse_strs_to_ts2_vec(
            pg_crud_macros_common::ParseTokenStreamStrings::from(cols_token_stream),
            pg_crud_macros_common::ParseErrorIdRef::from(str_constants::VALUE_79EE6381),
        );
        quote::quote! {
            struct PgTableColsUsingPgTypes {
                #ts
            }
        }
    };
    if let Err(error) = macros_helpers::ts_writer::maybe_write_token_stream_into_file(
        generate_pg_types_config.pg_table_cols_write_into_file,
        str_constants::PG_TABLE_COLS_USING_PG_TYPES,
        macros_helpers::ts_writer::ProcMacro2TokenStreamRef::from(&pg_table_cols_token_stream),
        &macros_helpers::ts_writer::FormatWithCargofmt::True,
    ) {
        let message = format!("failed to write generated PG table columns: {error}");
        return macros_helpers::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream::from(
            quote::quote! { compile_error!(#message); },
        );
    }
    let generated = {
        let ts = parse_strs_to_ts2_vec(
            pg_crud_macros_common::ParseTokenStreamStrings::from(pg_type_array),
            pg_crud_macros_common::ParseErrorIdRef::from(str_constants::E0C9257D),
        );
        pg_crud_macros_common::token_stream_helpers::generate_mod_with_pub_use_token_stream(
            &generate_pg_types_mod_snake_case,
            &ts,
        )
    };
    if let Err(error) = macros_helpers::ts_writer::maybe_write_token_stream_into_file(
        generate_pg_types_config.whole_write_into_file,
        str_constants::CODE_STYLE_GENERATE_PG_TYPES_MACRO_NAME,
        macros_helpers::ts_writer::ProcMacro2TokenStreamRef::from(generated.as_ref()),
        &macros_helpers::ts_writer::FormatWithCargofmt::True,
    ) {
        let message = format!("failed to write generated PG types: {error}");
        return macros_helpers::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream::from(
            quote::quote! { compile_error!(#message); },
        );
    }
    generated
}

#[cfg(test)]
mod tests {
    #[test]
    fn model_can_be_parsed_and_validated_without_emitting_source() {
        let input = quote::quote! {{
            "pg_table_cols_write_into_file": "False",
            "whole_write_into_file": "False",
            "variant": {"Subset": ["I16AsInt2", "StringAsText"]}
        }};
        let parsed = super::parse_generate_pg_types(
            macros_helpers::ts_writer::ProcMacro2TokenStreamRef::from(&input),
        )
        .expect("35a0f719");
        let built = super::build_generate_pg_types(parsed).expect("3c8d514f");
        let validated = super::validate_generate_pg_types(built).expect("b24816de");
        assert_eq!(usize::from(validated.entry_count()), 2usize);
    }

    #[test]
    fn malformed_config_is_a_typed_parse_error() {
        let input = quote::quote! {{"variant": "MissingFields"}};
        assert!(matches!(
            super::parse_generate_pg_types(
                macros_helpers::ts_writer::ProcMacro2TokenStreamRef::from(&input),
            ),
            Err(super::GeneratePgTypesPipelineError::Parse(_error))
        ));
    }

    #[test]
    fn generated_type_list_deserialization_rejects_too_many_entries() {
        let serialized = serde_json::to_string(&vec![
            super::PgType::I16AsInt2;
            super::GENERATE_PG_TYPES_MAX_LEN + 1usize
        ])
        .expect("7cd2e0af");
        let _error =
            serde_json::from_str::<super::GeneratePgTypes>(&serialized).expect_err("40b96aa2");
    }
}
