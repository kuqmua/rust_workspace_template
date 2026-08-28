#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
use super::*;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    newtype::AsRefInner,
    newtype::FromInner,
    newtype::ToTokens,
)]
pub(super) struct PgSqlName(pub(super) &'static str);
impl PgType {
    pub(super) fn pg_type_can_be_nullable(self) -> CanBeNullable {
        crate::domain_types::sqlx::pg_type_can_be_nullable::pg_type_can_be_nullable(self.spec())
    }
    pub(super) fn spec(
        self,
    ) -> crate::domain_types::pg_type_spec::PgTypeSpec<
        CanBeNullable,
        CanBePrimaryKey,
        FilterKind,
        PgSqlName,
        WireKind,
    > {
        match self {
            Self::I16AsInt2 => crate::domain_types::pg_type_spec::PgTypeSpec {
                can_be_primary_key: CanBePrimaryKey::False,
                can_be_nullable: CanBeNullable::True,
                filter_kind: FilterKind::Number,
                pg_name: PgSqlName::from(constants_str::PG_CRUD_PG_INT2),
                wire_kind: WireKind::Int16,
            },
            Self::I32AsInt4 => crate::domain_types::pg_type_spec::PgTypeSpec {
                can_be_primary_key: CanBePrimaryKey::False,
                can_be_nullable: CanBeNullable::True,
                filter_kind: FilterKind::Number,
                pg_name: PgSqlName::from(constants_str::PG_CRUD_PG_INT4),
                wire_kind: WireKind::Int32,
            },
            Self::I64AsInt8 => crate::domain_types::pg_type_spec::PgTypeSpec {
                can_be_primary_key: CanBePrimaryKey::False,
                can_be_nullable: CanBeNullable::True,
                filter_kind: FilterKind::Number,
                pg_name: PgSqlName::from(constants_str::PG_CRUD_PG_INT8),
                wire_kind: WireKind::Int64,
            },
            Self::F32AsFloat4 => crate::domain_types::pg_type_spec::PgTypeSpec {
                can_be_primary_key: CanBePrimaryKey::False,
                can_be_nullable: CanBeNullable::True,
                filter_kind: FilterKind::Number,
                pg_name: PgSqlName::from(constants_str::PG_CRUD_PG_FLOAT4),
                wire_kind: WireKind::Float32,
            },
            Self::F64AsFloat8 => crate::domain_types::pg_type_spec::PgTypeSpec {
                can_be_primary_key: CanBePrimaryKey::False,
                can_be_nullable: CanBeNullable::True,
                filter_kind: FilterKind::Number,
                pg_name: PgSqlName::from(constants_str::PG_CRUD_PG_FLOAT8),
                wire_kind: WireKind::Float64,
            },
            Self::I16AsSmallSerialInitializationByPg => {
                crate::domain_types::pg_type_spec::PgTypeSpec {
                    can_be_primary_key: CanBePrimaryKey::True,
                    can_be_nullable: CanBeNullable::False,
                    filter_kind: FilterKind::Number,
                    pg_name: PgSqlName::from(constants_str::PG_CRUD_PG_SMALLSERIAL),
                    wire_kind: WireKind::Int16,
                }
            }
            Self::I32AsSerialInitializationByPg => crate::domain_types::pg_type_spec::PgTypeSpec {
                can_be_primary_key: CanBePrimaryKey::True,
                can_be_nullable: CanBeNullable::False,
                filter_kind: FilterKind::Number,
                pg_name: PgSqlName::from(constants_str::PG_CRUD_PG_SERIAL),
                wire_kind: WireKind::Int32,
            },
            Self::I64AsBigSerialInitializationByPg => {
                crate::domain_types::pg_type_spec::PgTypeSpec {
                    can_be_primary_key: CanBePrimaryKey::True,
                    can_be_nullable: CanBeNullable::False,
                    filter_kind: FilterKind::Number,
                    pg_name: PgSqlName::from(constants_str::PG_CRUD_PG_BIGSERIAL),
                    wire_kind: WireKind::Int64,
                }
            }
            Self::SqlxPgTypesPgMoneyAsMoney => crate::domain_types::pg_type_spec::PgTypeSpec {
                can_be_primary_key: CanBePrimaryKey::False,
                can_be_nullable: CanBeNullable::True,
                filter_kind: FilterKind::Money,
                pg_name: PgSqlName::from(constants_str::PG_CRUD_PG_MONEY),
                wire_kind: WireKind::Int64,
            },
            Self::BoolAsBool => crate::domain_types::pg_type_spec::PgTypeSpec {
                can_be_primary_key: CanBePrimaryKey::False,
                can_be_nullable: CanBeNullable::True,
                filter_kind: FilterKind::Bool,
                pg_name: PgSqlName::from(constants_str::PG_CRUD_PG_BOOL),
                wire_kind: WireKind::Bool,
            },
            Self::StringAsText => crate::domain_types::pg_type_spec::PgTypeSpec {
                can_be_primary_key: CanBePrimaryKey::False,
                can_be_nullable: CanBeNullable::True,
                filter_kind: FilterKind::String,
                pg_name: PgSqlName::from(constants_str::PG_CRUD_PG_TEXT),
                wire_kind: WireKind::String,
            },
            Self::StdVecVecU8AsBytea => crate::domain_types::pg_type_spec::PgTypeSpec {
                can_be_primary_key: CanBePrimaryKey::False,
                can_be_nullable: CanBeNullable::True,
                filter_kind: FilterKind::Bytes,
                pg_name: PgSqlName::from(constants_str::PG_CRUD_PG_BYTEA),
                wire_kind: WireKind::Bytes,
            },
            Self::SqlxTypesChronoNaiveTimeAsTime => crate::domain_types::pg_type_spec::PgTypeSpec {
                can_be_primary_key: CanBePrimaryKey::False,
                can_be_nullable: CanBeNullable::True,
                filter_kind: FilterKind::Time,
                pg_name: PgSqlName::from(constants_str::PG_CRUD_PG_TIME),
                wire_kind: WireKind::TimeChrono,
            },
            Self::SqlxTypesTimeTimeAsTime => crate::domain_types::pg_type_spec::PgTypeSpec {
                can_be_primary_key: CanBePrimaryKey::False,
                can_be_nullable: CanBeNullable::True,
                filter_kind: FilterKind::Time,
                pg_name: PgSqlName::from(constants_str::PG_CRUD_PG_TIME),
                wire_kind: WireKind::TimeTime,
            },
            Self::SqlxPgTypesPgIntervalAsInterval => {
                crate::domain_types::pg_type_spec::PgTypeSpec {
                    can_be_primary_key: CanBePrimaryKey::False,
                    can_be_nullable: CanBeNullable::True,
                    filter_kind: FilterKind::IntervalOrInet,
                    pg_name: PgSqlName::from(constants_str::PG_CRUD_PG_INTERVAL),
                    wire_kind: WireKind::Interval,
                }
            }
            Self::SqlxTypesChronoNaiveDateAsDate => crate::domain_types::pg_type_spec::PgTypeSpec {
                can_be_primary_key: CanBePrimaryKey::False,
                can_be_nullable: CanBeNullable::True,
                filter_kind: FilterKind::Date,
                pg_name: PgSqlName::from(constants_str::PG_CRUD_PG_DATE),
                wire_kind: WireKind::Date,
            },
            Self::SqlxTypesChronoNaiveDateTimeAsTimestamp => {
                crate::domain_types::pg_type_spec::PgTypeSpec {
                    can_be_primary_key: CanBePrimaryKey::False,
                    can_be_nullable: CanBeNullable::True,
                    filter_kind: FilterKind::Timestamp,
                    pg_name: PgSqlName::from(constants_str::PG_CRUD_PG_TIMESTAMP),
                    wire_kind: WireKind::Timestamp,
                }
            }
            Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => {
                crate::domain_types::pg_type_spec::PgTypeSpec {
                    can_be_primary_key: CanBePrimaryKey::False,
                    can_be_nullable: CanBeNullable::True,
                    filter_kind: FilterKind::TimestampTz,
                    pg_name: PgSqlName::from(constants_str::PG_CRUD_PG_TIMESTAMPTZ),
                    wire_kind: WireKind::TimestampTz,
                }
            }
            Self::SqlxTypesUuidUuidAsUuidV4InitializationByPg => {
                crate::domain_types::pg_type_spec::PgTypeSpec {
                    can_be_primary_key: CanBePrimaryKey::True,
                    can_be_nullable: CanBeNullable::False,
                    filter_kind: FilterKind::Uuid,
                    pg_name: PgSqlName::from(constants_str::PG_CRUD_PG_UUID),
                    wire_kind: WireKind::Uuid,
                }
            }
            Self::SqlxTypesUuidUuidAsUuidInitializationByClient => {
                crate::domain_types::pg_type_spec::PgTypeSpec {
                    can_be_primary_key: CanBePrimaryKey::False,
                    can_be_nullable: CanBeNullable::True,
                    filter_kind: FilterKind::Uuid,
                    pg_name: PgSqlName::from(constants_str::PG_CRUD_PG_UUID),
                    wire_kind: WireKind::Uuid,
                }
            }
            Self::SqlxTypesIpnetworkIpNetworkAsInet => {
                crate::domain_types::pg_type_spec::PgTypeSpec {
                    can_be_primary_key: CanBePrimaryKey::False,
                    can_be_nullable: CanBeNullable::True,
                    filter_kind: FilterKind::IntervalOrInet,
                    pg_name: PgSqlName::from(constants_str::PG_CRUD_PG_INET),
                    wire_kind: WireKind::Inet,
                }
            }
            Self::SqlxTypesMacAddressMacAddressAsMacAddr => {
                crate::domain_types::pg_type_spec::PgTypeSpec {
                    can_be_primary_key: CanBePrimaryKey::False,
                    can_be_nullable: CanBeNullable::True,
                    filter_kind: FilterKind::Mac,
                    pg_name: PgSqlName::from(constants_str::PG_CRUD_PG_MACADDR),
                    wire_kind: WireKind::Mac,
                }
            }
            Self::SqlxPgTypesPgRangeI32AsInt4Range => {
                crate::domain_types::pg_type_spec::PgTypeSpec {
                    can_be_primary_key: CanBePrimaryKey::False,
                    can_be_nullable: CanBeNullable::True,
                    filter_kind: FilterKind::Range,
                    pg_name: PgSqlName::from(constants_str::PG_CRUD_PG_INT4RANGE),
                    wire_kind: WireKind::RangeInt32,
                }
            }
            Self::SqlxPgTypesPgRangeI64AsInt8Range => {
                crate::domain_types::pg_type_spec::PgTypeSpec {
                    can_be_primary_key: CanBePrimaryKey::False,
                    can_be_nullable: CanBeNullable::True,
                    filter_kind: FilterKind::Range,
                    pg_name: PgSqlName::from(constants_str::PG_CRUD_PG_INT8RANGE),
                    wire_kind: WireKind::RangeInt64,
                }
            }
            Self::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => {
                crate::domain_types::pg_type_spec::PgTypeSpec {
                    can_be_primary_key: CanBePrimaryKey::False,
                    can_be_nullable: CanBeNullable::True,
                    filter_kind: FilterKind::Range,
                    pg_name: PgSqlName::from(constants_str::PG_CRUD_PG_DATERANGE),
                    wire_kind: WireKind::RangeDate,
                }
            }
            Self::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => {
                crate::domain_types::pg_type_spec::PgTypeSpec {
                    can_be_primary_key: CanBePrimaryKey::False,
                    can_be_nullable: CanBeNullable::True,
                    filter_kind: FilterKind::Range,
                    pg_name: PgSqlName::from(constants_str::PG_CRUD_PG_TSRANGE),
                    wire_kind: WireKind::RangeTimestamp,
                }
            }
            Self::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => {
                crate::domain_types::pg_type_spec::PgTypeSpec {
                    can_be_primary_key: CanBePrimaryKey::False,
                    can_be_nullable: CanBeNullable::True,
                    filter_kind: FilterKind::Range,
                    pg_name: PgSqlName::from(constants_str::PG_CRUD_PG_TSTZRANGE),
                    wire_kind: WireKind::RangeTimestampTz,
                }
            }
        }
    }
}
