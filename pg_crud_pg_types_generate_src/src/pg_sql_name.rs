#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    proc_macro_newtype_as_ref_inner::AsRefInner,
    proc_macro_newtype_from_inner::FromInner,
    proc_macro_newtype_to_tokens::ToTokens,
)]
pub(super) struct PgSqlName(&'static str);
impl crate::pg_type_catalog_kind::PgTypeCatalogKind {
    pub(super) fn pg_type_can_be_nullable(self) -> crate::can_be_nullable::CanBeNullable {
        crate::pg_type_can_be_nullable::pg_type_can_be_nullable(&self.spec())
    }
    pub(super) fn spec(
        self,
    ) -> crate::pg_type_spec::PgTypeSpec<
        crate::can_be_nullable::CanBeNullable,
        crate::can_be_primary_key::CanBePrimaryKey,
        crate::filter_kind::FilterKind,
        PgSqlName,
        crate::wire_kind::WireKind,
    > {
        match self {
            Self::I16AsInt2 => crate::pg_type_spec::PgTypeSpec::new(
                crate::can_be_nullable::CanBeNullable::True,
                crate::can_be_primary_key::CanBePrimaryKey::False,
                crate::filter_kind::FilterKind::Number,
                PgSqlName::from(constants_str::PG_CRUD_PG_INT2),
                crate::wire_kind::WireKind::Int16,
            ),
            Self::I32AsInt4 => crate::pg_type_spec::PgTypeSpec::new(
                crate::can_be_nullable::CanBeNullable::True,
                crate::can_be_primary_key::CanBePrimaryKey::False,
                crate::filter_kind::FilterKind::Number,
                PgSqlName::from(constants_str::PG_CRUD_PG_INT4),
                crate::wire_kind::WireKind::Int32,
            ),
            Self::I64AsInt8 => crate::pg_type_spec::PgTypeSpec::new(
                crate::can_be_nullable::CanBeNullable::True,
                crate::can_be_primary_key::CanBePrimaryKey::False,
                crate::filter_kind::FilterKind::Number,
                PgSqlName::from(constants_str::PG_CRUD_PG_INT8),
                crate::wire_kind::WireKind::Int64,
            ),
            Self::F32AsFloat4 => crate::pg_type_spec::PgTypeSpec::new(
                crate::can_be_nullable::CanBeNullable::True,
                crate::can_be_primary_key::CanBePrimaryKey::False,
                crate::filter_kind::FilterKind::Number,
                PgSqlName::from(constants_str::PG_CRUD_PG_FLOAT4),
                crate::wire_kind::WireKind::Float32,
            ),
            Self::F64AsFloat8 => crate::pg_type_spec::PgTypeSpec::new(
                crate::can_be_nullable::CanBeNullable::True,
                crate::can_be_primary_key::CanBePrimaryKey::False,
                crate::filter_kind::FilterKind::Number,
                PgSqlName::from(constants_str::PG_CRUD_PG_FLOAT8),
                crate::wire_kind::WireKind::Float64,
            ),
            Self::I16AsSmallSerialInitializationByPg => crate::pg_type_spec::PgTypeSpec::new(
                crate::can_be_nullable::CanBeNullable::False,
                crate::can_be_primary_key::CanBePrimaryKey::True,
                crate::filter_kind::FilterKind::Number,
                PgSqlName::from(constants_str::PG_CRUD_PG_SMALLSERIAL),
                crate::wire_kind::WireKind::Int16,
            ),
            Self::I32AsSerialInitializationByPg => crate::pg_type_spec::PgTypeSpec::new(
                crate::can_be_nullable::CanBeNullable::False,
                crate::can_be_primary_key::CanBePrimaryKey::True,
                crate::filter_kind::FilterKind::Number,
                PgSqlName::from(constants_str::PG_CRUD_PG_SERIAL),
                crate::wire_kind::WireKind::Int32,
            ),
            Self::I64AsBigSerialInitializationByPg => crate::pg_type_spec::PgTypeSpec::new(
                crate::can_be_nullable::CanBeNullable::False,
                crate::can_be_primary_key::CanBePrimaryKey::True,
                crate::filter_kind::FilterKind::Number,
                PgSqlName::from(constants_str::PG_CRUD_PG_BIGSERIAL),
                crate::wire_kind::WireKind::Int64,
            ),
            Self::SqlxPgTypesPgMoneyAsMoney => crate::pg_type_spec::PgTypeSpec::new(
                crate::can_be_nullable::CanBeNullable::True,
                crate::can_be_primary_key::CanBePrimaryKey::False,
                crate::filter_kind::FilterKind::Money,
                PgSqlName::from(constants_str::PG_CRUD_PG_MONEY),
                crate::wire_kind::WireKind::Int64,
            ),
            Self::BoolAsBool => crate::pg_type_spec::PgTypeSpec::new(
                crate::can_be_nullable::CanBeNullable::True,
                crate::can_be_primary_key::CanBePrimaryKey::False,
                crate::filter_kind::FilterKind::Bool,
                PgSqlName::from(constants_str::PG_CRUD_PG_BOOL),
                crate::wire_kind::WireKind::Bool,
            ),
            Self::StringAsText => crate::pg_type_spec::PgTypeSpec::new(
                crate::can_be_nullable::CanBeNullable::True,
                crate::can_be_primary_key::CanBePrimaryKey::False,
                crate::filter_kind::FilterKind::String,
                PgSqlName::from(constants_str::PG_CRUD_PG_TEXT),
                crate::wire_kind::WireKind::String,
            ),
            Self::StdVecVecU8AsBytea => crate::pg_type_spec::PgTypeSpec::new(
                crate::can_be_nullable::CanBeNullable::True,
                crate::can_be_primary_key::CanBePrimaryKey::False,
                crate::filter_kind::FilterKind::Bytes,
                PgSqlName::from(constants_str::PG_CRUD_PG_BYTEA),
                crate::wire_kind::WireKind::Bytes,
            ),
            Self::SqlxTypesChronoNaiveTimeAsTime => crate::pg_type_spec::PgTypeSpec::new(
                crate::can_be_nullable::CanBeNullable::True,
                crate::can_be_primary_key::CanBePrimaryKey::False,
                crate::filter_kind::FilterKind::Time,
                PgSqlName::from(constants_str::PG_CRUD_PG_TIME),
                crate::wire_kind::WireKind::TimeChrono,
            ),
            Self::SqlxTypesTimeTimeAsTime => crate::pg_type_spec::PgTypeSpec::new(
                crate::can_be_nullable::CanBeNullable::True,
                crate::can_be_primary_key::CanBePrimaryKey::False,
                crate::filter_kind::FilterKind::Time,
                PgSqlName::from(constants_str::PG_CRUD_PG_TIME),
                crate::wire_kind::WireKind::TimeTime,
            ),
            Self::SqlxPgTypesPgIntervalAsInterval => crate::pg_type_spec::PgTypeSpec::new(
                crate::can_be_nullable::CanBeNullable::True,
                crate::can_be_primary_key::CanBePrimaryKey::False,
                crate::filter_kind::FilterKind::IntervalOrInet,
                PgSqlName::from(constants_str::PG_CRUD_PG_INTERVAL),
                crate::wire_kind::WireKind::Interval,
            ),
            Self::SqlxTypesChronoNaiveDateAsDate => crate::pg_type_spec::PgTypeSpec::new(
                crate::can_be_nullable::CanBeNullable::True,
                crate::can_be_primary_key::CanBePrimaryKey::False,
                crate::filter_kind::FilterKind::Date,
                PgSqlName::from(constants_str::PG_CRUD_PG_DATE),
                crate::wire_kind::WireKind::Date,
            ),
            Self::SqlxTypesChronoNaiveDateTimeAsTimestamp => crate::pg_type_spec::PgTypeSpec::new(
                crate::can_be_nullable::CanBeNullable::True,
                crate::can_be_primary_key::CanBePrimaryKey::False,
                crate::filter_kind::FilterKind::Timestamp,
                PgSqlName::from(constants_str::PG_CRUD_PG_TIMESTAMP),
                crate::wire_kind::WireKind::Timestamp,
            ),
            Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => {
                crate::pg_type_spec::PgTypeSpec::new(
                    crate::can_be_nullable::CanBeNullable::True,
                    crate::can_be_primary_key::CanBePrimaryKey::False,
                    crate::filter_kind::FilterKind::TimestampTz,
                    PgSqlName::from(constants_str::PG_CRUD_PG_TIMESTAMPTZ),
                    crate::wire_kind::WireKind::TimestampTz,
                )
            }
            Self::SqlxTypesUuidUuidAsUuidV4InitializationByPg => {
                crate::pg_type_spec::PgTypeSpec::new(
                    crate::can_be_nullable::CanBeNullable::False,
                    crate::can_be_primary_key::CanBePrimaryKey::True,
                    crate::filter_kind::FilterKind::Uuid,
                    PgSqlName::from(constants_str::PG_CRUD_PG_UUID),
                    crate::wire_kind::WireKind::Uuid,
                )
            }
            Self::SqlxTypesUuidUuidAsUuidInitializationByClient => {
                crate::pg_type_spec::PgTypeSpec::new(
                    crate::can_be_nullable::CanBeNullable::True,
                    crate::can_be_primary_key::CanBePrimaryKey::False,
                    crate::filter_kind::FilterKind::Uuid,
                    PgSqlName::from(constants_str::PG_CRUD_PG_UUID),
                    crate::wire_kind::WireKind::Uuid,
                )
            }
            Self::SqlxTypesIpnetworkIpNetworkAsInet => crate::pg_type_spec::PgTypeSpec::new(
                crate::can_be_nullable::CanBeNullable::True,
                crate::can_be_primary_key::CanBePrimaryKey::False,
                crate::filter_kind::FilterKind::IntervalOrInet,
                PgSqlName::from(constants_str::PG_CRUD_PG_INET),
                crate::wire_kind::WireKind::Inet,
            ),
            Self::SqlxTypesMacAddressMacAddressAsMacAddr => crate::pg_type_spec::PgTypeSpec::new(
                crate::can_be_nullable::CanBeNullable::True,
                crate::can_be_primary_key::CanBePrimaryKey::False,
                crate::filter_kind::FilterKind::Mac,
                PgSqlName::from(constants_str::PG_CRUD_PG_MACADDR),
                crate::wire_kind::WireKind::Mac,
            ),
            Self::SqlxPgTypesPgRangeI32AsInt4Range => crate::pg_type_spec::PgTypeSpec::new(
                crate::can_be_nullable::CanBeNullable::True,
                crate::can_be_primary_key::CanBePrimaryKey::False,
                crate::filter_kind::FilterKind::Range,
                PgSqlName::from(constants_str::PG_CRUD_PG_INT4RANGE),
                crate::wire_kind::WireKind::RangeInt32,
            ),
            Self::SqlxPgTypesPgRangeI64AsInt8Range => crate::pg_type_spec::PgTypeSpec::new(
                crate::can_be_nullable::CanBeNullable::True,
                crate::can_be_primary_key::CanBePrimaryKey::False,
                crate::filter_kind::FilterKind::Range,
                PgSqlName::from(constants_str::PG_CRUD_PG_INT8RANGE),
                crate::wire_kind::WireKind::RangeInt64,
            ),
            Self::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => {
                crate::pg_type_spec::PgTypeSpec::new(
                    crate::can_be_nullable::CanBeNullable::True,
                    crate::can_be_primary_key::CanBePrimaryKey::False,
                    crate::filter_kind::FilterKind::Range,
                    PgSqlName::from(constants_str::PG_CRUD_PG_DATERANGE),
                    crate::wire_kind::WireKind::RangeDate,
                )
            }
            Self::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => {
                crate::pg_type_spec::PgTypeSpec::new(
                    crate::can_be_nullable::CanBeNullable::True,
                    crate::can_be_primary_key::CanBePrimaryKey::False,
                    crate::filter_kind::FilterKind::Range,
                    PgSqlName::from(constants_str::PG_CRUD_PG_TSRANGE),
                    crate::wire_kind::WireKind::RangeTimestamp,
                )
            }
            Self::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => {
                crate::pg_type_spec::PgTypeSpec::new(
                    crate::can_be_nullable::CanBeNullable::True,
                    crate::can_be_primary_key::CanBePrimaryKey::False,
                    crate::filter_kind::FilterKind::Range,
                    PgSqlName::from(constants_str::PG_CRUD_PG_TSTZRANGE),
                    crate::wire_kind::WireKind::RangeTimestampTz,
                )
            }
        }
    }
}
