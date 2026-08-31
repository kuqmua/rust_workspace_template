#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    newtype::AsRefInner,
    newtype::FromInner,
    newtype::ToTokens,
)]
pub(super) struct PgSqlName(&'static str);
impl crate::pg_type_catalog_kind::PgTypeCatalogKind {
    pub(super) fn pg_type_can_be_nullable(self) -> crate::can_be_nullable::CanBeNullable {
        crate::pg_type_can_be_nullable::pg_type_can_be_nullable(self.spec())
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
            Self::I16AsInt2 => crate::pg_type_spec::PgTypeSpec {
                can_be_primary_key: crate::can_be_primary_key::CanBePrimaryKey::False,
                can_be_nullable: crate::can_be_nullable::CanBeNullable::True,
                filter_kind: crate::filter_kind::FilterKind::Number,
                pg_name: PgSqlName::from(constants_str::PG_CRUD_PG_INT2),
                wire_kind: crate::wire_kind::WireKind::Int16,
            },
            Self::I32AsInt4 => crate::pg_type_spec::PgTypeSpec {
                can_be_primary_key: crate::can_be_primary_key::CanBePrimaryKey::False,
                can_be_nullable: crate::can_be_nullable::CanBeNullable::True,
                filter_kind: crate::filter_kind::FilterKind::Number,
                pg_name: PgSqlName::from(constants_str::PG_CRUD_PG_INT4),
                wire_kind: crate::wire_kind::WireKind::Int32,
            },
            Self::I64AsInt8 => crate::pg_type_spec::PgTypeSpec {
                can_be_primary_key: crate::can_be_primary_key::CanBePrimaryKey::False,
                can_be_nullable: crate::can_be_nullable::CanBeNullable::True,
                filter_kind: crate::filter_kind::FilterKind::Number,
                pg_name: PgSqlName::from(constants_str::PG_CRUD_PG_INT8),
                wire_kind: crate::wire_kind::WireKind::Int64,
            },
            Self::F32AsFloat4 => crate::pg_type_spec::PgTypeSpec {
                can_be_primary_key: crate::can_be_primary_key::CanBePrimaryKey::False,
                can_be_nullable: crate::can_be_nullable::CanBeNullable::True,
                filter_kind: crate::filter_kind::FilterKind::Number,
                pg_name: PgSqlName::from(constants_str::PG_CRUD_PG_FLOAT4),
                wire_kind: crate::wire_kind::WireKind::Float32,
            },
            Self::F64AsFloat8 => crate::pg_type_spec::PgTypeSpec {
                can_be_primary_key: crate::can_be_primary_key::CanBePrimaryKey::False,
                can_be_nullable: crate::can_be_nullable::CanBeNullable::True,
                filter_kind: crate::filter_kind::FilterKind::Number,
                pg_name: PgSqlName::from(constants_str::PG_CRUD_PG_FLOAT8),
                wire_kind: crate::wire_kind::WireKind::Float64,
            },
            Self::I16AsSmallSerialInitializationByPg => crate::pg_type_spec::PgTypeSpec {
                can_be_primary_key: crate::can_be_primary_key::CanBePrimaryKey::True,
                can_be_nullable: crate::can_be_nullable::CanBeNullable::False,
                filter_kind: crate::filter_kind::FilterKind::Number,
                pg_name: PgSqlName::from(constants_str::PG_CRUD_PG_SMALLSERIAL),
                wire_kind: crate::wire_kind::WireKind::Int16,
            },
            Self::I32AsSerialInitializationByPg => crate::pg_type_spec::PgTypeSpec {
                can_be_primary_key: crate::can_be_primary_key::CanBePrimaryKey::True,
                can_be_nullable: crate::can_be_nullable::CanBeNullable::False,
                filter_kind: crate::filter_kind::FilterKind::Number,
                pg_name: PgSqlName::from(constants_str::PG_CRUD_PG_SERIAL),
                wire_kind: crate::wire_kind::WireKind::Int32,
            },
            Self::I64AsBigSerialInitializationByPg => crate::pg_type_spec::PgTypeSpec {
                can_be_primary_key: crate::can_be_primary_key::CanBePrimaryKey::True,
                can_be_nullable: crate::can_be_nullable::CanBeNullable::False,
                filter_kind: crate::filter_kind::FilterKind::Number,
                pg_name: PgSqlName::from(constants_str::PG_CRUD_PG_BIGSERIAL),
                wire_kind: crate::wire_kind::WireKind::Int64,
            },
            Self::SqlxPgTypesPgMoneyAsMoney => crate::pg_type_spec::PgTypeSpec {
                can_be_primary_key: crate::can_be_primary_key::CanBePrimaryKey::False,
                can_be_nullable: crate::can_be_nullable::CanBeNullable::True,
                filter_kind: crate::filter_kind::FilterKind::Money,
                pg_name: PgSqlName::from(constants_str::PG_CRUD_PG_MONEY),
                wire_kind: crate::wire_kind::WireKind::Int64,
            },
            Self::BoolAsBool => crate::pg_type_spec::PgTypeSpec {
                can_be_primary_key: crate::can_be_primary_key::CanBePrimaryKey::False,
                can_be_nullable: crate::can_be_nullable::CanBeNullable::True,
                filter_kind: crate::filter_kind::FilterKind::Bool,
                pg_name: PgSqlName::from(constants_str::PG_CRUD_PG_BOOL),
                wire_kind: crate::wire_kind::WireKind::Bool,
            },
            Self::StringAsText => crate::pg_type_spec::PgTypeSpec {
                can_be_primary_key: crate::can_be_primary_key::CanBePrimaryKey::False,
                can_be_nullable: crate::can_be_nullable::CanBeNullable::True,
                filter_kind: crate::filter_kind::FilterKind::String,
                pg_name: PgSqlName::from(constants_str::PG_CRUD_PG_TEXT),
                wire_kind: crate::wire_kind::WireKind::String,
            },
            Self::StdVecVecU8AsBytea => crate::pg_type_spec::PgTypeSpec {
                can_be_primary_key: crate::can_be_primary_key::CanBePrimaryKey::False,
                can_be_nullable: crate::can_be_nullable::CanBeNullable::True,
                filter_kind: crate::filter_kind::FilterKind::Bytes,
                pg_name: PgSqlName::from(constants_str::PG_CRUD_PG_BYTEA),
                wire_kind: crate::wire_kind::WireKind::Bytes,
            },
            Self::SqlxTypesChronoNaiveTimeAsTime => crate::pg_type_spec::PgTypeSpec {
                can_be_primary_key: crate::can_be_primary_key::CanBePrimaryKey::False,
                can_be_nullable: crate::can_be_nullable::CanBeNullable::True,
                filter_kind: crate::filter_kind::FilterKind::Time,
                pg_name: PgSqlName::from(constants_str::PG_CRUD_PG_TIME),
                wire_kind: crate::wire_kind::WireKind::TimeChrono,
            },
            Self::SqlxTypesTimeTimeAsTime => crate::pg_type_spec::PgTypeSpec {
                can_be_primary_key: crate::can_be_primary_key::CanBePrimaryKey::False,
                can_be_nullable: crate::can_be_nullable::CanBeNullable::True,
                filter_kind: crate::filter_kind::FilterKind::Time,
                pg_name: PgSqlName::from(constants_str::PG_CRUD_PG_TIME),
                wire_kind: crate::wire_kind::WireKind::TimeTime,
            },
            Self::SqlxPgTypesPgIntervalAsInterval => crate::pg_type_spec::PgTypeSpec {
                can_be_primary_key: crate::can_be_primary_key::CanBePrimaryKey::False,
                can_be_nullable: crate::can_be_nullable::CanBeNullable::True,
                filter_kind: crate::filter_kind::FilterKind::IntervalOrInet,
                pg_name: PgSqlName::from(constants_str::PG_CRUD_PG_INTERVAL),
                wire_kind: crate::wire_kind::WireKind::Interval,
            },
            Self::SqlxTypesChronoNaiveDateAsDate => crate::pg_type_spec::PgTypeSpec {
                can_be_primary_key: crate::can_be_primary_key::CanBePrimaryKey::False,
                can_be_nullable: crate::can_be_nullable::CanBeNullable::True,
                filter_kind: crate::filter_kind::FilterKind::Date,
                pg_name: PgSqlName::from(constants_str::PG_CRUD_PG_DATE),
                wire_kind: crate::wire_kind::WireKind::Date,
            },
            Self::SqlxTypesChronoNaiveDateTimeAsTimestamp => crate::pg_type_spec::PgTypeSpec {
                can_be_primary_key: crate::can_be_primary_key::CanBePrimaryKey::False,
                can_be_nullable: crate::can_be_nullable::CanBeNullable::True,
                filter_kind: crate::filter_kind::FilterKind::Timestamp,
                pg_name: PgSqlName::from(constants_str::PG_CRUD_PG_TIMESTAMP),
                wire_kind: crate::wire_kind::WireKind::Timestamp,
            },
            Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => {
                crate::pg_type_spec::PgTypeSpec {
                    can_be_primary_key: crate::can_be_primary_key::CanBePrimaryKey::False,
                    can_be_nullable: crate::can_be_nullable::CanBeNullable::True,
                    filter_kind: crate::filter_kind::FilterKind::TimestampTz,
                    pg_name: PgSqlName::from(constants_str::PG_CRUD_PG_TIMESTAMPTZ),
                    wire_kind: crate::wire_kind::WireKind::TimestampTz,
                }
            }
            Self::SqlxTypesUuidUuidAsUuidV4InitializationByPg => crate::pg_type_spec::PgTypeSpec {
                can_be_primary_key: crate::can_be_primary_key::CanBePrimaryKey::True,
                can_be_nullable: crate::can_be_nullable::CanBeNullable::False,
                filter_kind: crate::filter_kind::FilterKind::Uuid,
                pg_name: PgSqlName::from(constants_str::PG_CRUD_PG_UUID),
                wire_kind: crate::wire_kind::WireKind::Uuid,
            },
            Self::SqlxTypesUuidUuidAsUuidInitializationByClient => {
                crate::pg_type_spec::PgTypeSpec {
                    can_be_primary_key: crate::can_be_primary_key::CanBePrimaryKey::False,
                    can_be_nullable: crate::can_be_nullable::CanBeNullable::True,
                    filter_kind: crate::filter_kind::FilterKind::Uuid,
                    pg_name: PgSqlName::from(constants_str::PG_CRUD_PG_UUID),
                    wire_kind: crate::wire_kind::WireKind::Uuid,
                }
            }
            Self::SqlxTypesIpnetworkIpNetworkAsInet => crate::pg_type_spec::PgTypeSpec {
                can_be_primary_key: crate::can_be_primary_key::CanBePrimaryKey::False,
                can_be_nullable: crate::can_be_nullable::CanBeNullable::True,
                filter_kind: crate::filter_kind::FilterKind::IntervalOrInet,
                pg_name: PgSqlName::from(constants_str::PG_CRUD_PG_INET),
                wire_kind: crate::wire_kind::WireKind::Inet,
            },
            Self::SqlxTypesMacAddressMacAddressAsMacAddr => crate::pg_type_spec::PgTypeSpec {
                can_be_primary_key: crate::can_be_primary_key::CanBePrimaryKey::False,
                can_be_nullable: crate::can_be_nullable::CanBeNullable::True,
                filter_kind: crate::filter_kind::FilterKind::Mac,
                pg_name: PgSqlName::from(constants_str::PG_CRUD_PG_MACADDR),
                wire_kind: crate::wire_kind::WireKind::Mac,
            },
            Self::SqlxPgTypesPgRangeI32AsInt4Range => crate::pg_type_spec::PgTypeSpec {
                can_be_primary_key: crate::can_be_primary_key::CanBePrimaryKey::False,
                can_be_nullable: crate::can_be_nullable::CanBeNullable::True,
                filter_kind: crate::filter_kind::FilterKind::Range,
                pg_name: PgSqlName::from(constants_str::PG_CRUD_PG_INT4RANGE),
                wire_kind: crate::wire_kind::WireKind::RangeInt32,
            },
            Self::SqlxPgTypesPgRangeI64AsInt8Range => crate::pg_type_spec::PgTypeSpec {
                can_be_primary_key: crate::can_be_primary_key::CanBePrimaryKey::False,
                can_be_nullable: crate::can_be_nullable::CanBeNullable::True,
                filter_kind: crate::filter_kind::FilterKind::Range,
                pg_name: PgSqlName::from(constants_str::PG_CRUD_PG_INT8RANGE),
                wire_kind: crate::wire_kind::WireKind::RangeInt64,
            },
            Self::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => {
                crate::pg_type_spec::PgTypeSpec {
                    can_be_primary_key: crate::can_be_primary_key::CanBePrimaryKey::False,
                    can_be_nullable: crate::can_be_nullable::CanBeNullable::True,
                    filter_kind: crate::filter_kind::FilterKind::Range,
                    pg_name: PgSqlName::from(constants_str::PG_CRUD_PG_DATERANGE),
                    wire_kind: crate::wire_kind::WireKind::RangeDate,
                }
            }
            Self::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => {
                crate::pg_type_spec::PgTypeSpec {
                    can_be_primary_key: crate::can_be_primary_key::CanBePrimaryKey::False,
                    can_be_nullable: crate::can_be_nullable::CanBeNullable::True,
                    filter_kind: crate::filter_kind::FilterKind::Range,
                    pg_name: PgSqlName::from(constants_str::PG_CRUD_PG_TSRANGE),
                    wire_kind: crate::wire_kind::WireKind::RangeTimestamp,
                }
            }
            Self::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => {
                crate::pg_type_spec::PgTypeSpec {
                    can_be_primary_key: crate::can_be_primary_key::CanBePrimaryKey::False,
                    can_be_nullable: crate::can_be_nullable::CanBeNullable::True,
                    filter_kind: crate::filter_kind::FilterKind::Range,
                    pg_name: PgSqlName::from(constants_str::PG_CRUD_PG_TSTZRANGE),
                    wire_kind: crate::wire_kind::WireKind::RangeTimestampTz,
                }
            }
        }
    }
}
