#[must_use]
pub fn gen_pg_types(
    input_ts: macros_helpers::ts_writer::ProcMacro2TsRef<'_>,
) -> macros_helpers::generated_rust_ts::GeneratedRustTs {
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
                PgType::I16AsInt2 | PgType::I16AsSmallSerialInitByPg => Self::I16,
                PgType::I32AsInt4 | PgType::I32AsSerialInitByPg => Self::I32,
                PgType::I64AsInt8 | PgType::I64AsBigSerialInitByPg => Self::I64,
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
                PgType::SqlxTypesUuidUuidAsUuidV4InitByPg | PgType::SqlxTypesUuidUuidAsUuidInitByClient => Self::SqlxTypesUuidUuid,
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
        SmallSerialInitByPg,
        SerialInitByPg,
        BigSerialInitByPg,
        Money,
        Bool,
        Text,
        Bytea,
        Time,
        Interval,
        Date,
        Timestamp,
        TimestampTz,
        UuidV4InitByPg,
        UuidInitByClient,
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
                PgType::I16AsSmallSerialInitByPg => Self::SmallSerialInitByPg,
                PgType::I32AsSerialInitByPg => Self::SerialInitByPg,
                PgType::I64AsBigSerialInitByPg => Self::BigSerialInitByPg,
                PgType::SqlxPgTypesPgMoneyAsMoney => Self::Money,
                PgType::BoolAsBool => Self::Bool,
                PgType::StringAsText => Self::Text,
                PgType::StdVecVecU8AsBytea => Self::Bytea,
                PgType::SqlxTypesChronoNaiveTimeAsTime | PgType::SqlxTypesTimeTimeAsTime => Self::Time,
                PgType::SqlxPgTypesPgIntervalAsInterval => Self::Interval,
                PgType::SqlxTypesChronoNaiveDateAsDate => Self::Date,
                PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp => Self::Timestamp,
                PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => Self::TimestampTz,
                PgType::SqlxTypesUuidUuidAsUuidV4InitByPg => Self::UuidV4InitByPg,
                PgType::SqlxTypesUuidUuidAsUuidInitByClient => Self::UuidInitByClient,
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
        PartialEq,
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
        I16AsSmallSerialInitByPg,
        I32AsSerialInitByPg,
        I64AsBigSerialInitByPg,
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
        SqlxTypesUuidUuidAsUuidV4InitByPg,
        SqlxTypesUuidUuidAsUuidInitByClient,
        SqlxTypesIpnetworkIpNetworkAsInet,
        SqlxTypesMacAddressMacAddressAsMacAddr,
        SqlxPgTypesPgRangeI32AsInt4Range,
        SqlxPgTypesPgRangeI64AsInt8Range,
        SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange,
        SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange,
        SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange,
    }
    enum CanBeNl {
        False,
        True,
    }
    impl PgType {
        const fn can_be_nl(&self) -> CanBeNl {
            match &self {
                Self::I16AsInt2
                | Self::I32AsInt4
                | Self::I64AsInt8
                | Self::F32AsFloat4
                | Self::F64AsFloat8
                | Self::SqlxPgTypesPgMoneyAsMoney
                | Self::BoolAsBool
                | Self::StringAsText
                | Self::StdVecVecU8AsBytea
                | Self::SqlxTypesChronoNaiveTimeAsTime
                | Self::SqlxTypesTimeTimeAsTime
                | Self::SqlxPgTypesPgIntervalAsInterval
                | Self::SqlxTypesChronoNaiveDateAsDate
                | Self::SqlxTypesChronoNaiveDateTimeAsTimestamp
                | Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz
                | Self::SqlxTypesUuidUuidAsUuidInitByClient
                | Self::SqlxTypesIpnetworkIpNetworkAsInet
                | Self::SqlxTypesMacAddressMacAddressAsMacAddr
                | Self::SqlxPgTypesPgRangeI32AsInt4Range
                | Self::SqlxPgTypesPgRangeI64AsInt8Range
                | Self::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange
                | Self::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange
                | Self::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => CanBeNl::True,
                Self::I16AsSmallSerialInitByPg | Self::I32AsSerialInitByPg | Self::I64AsBigSerialInitByPg | Self::SqlxTypesUuidUuidAsUuidV4InitByPg => CanBeNl::False,
            }
        }
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
                | PgType::I16AsSmallSerialInitByPg
                | PgType::I32AsSerialInitByPg
                | PgType::I64AsBigSerialInitByPg
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
                | PgType::SqlxTypesUuidUuidAsUuidV4InitByPg
                | PgType::SqlxTypesUuidUuidAsUuidInitByClient
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
                naming::prm::SelfNnUcc::from_display(&PgType::from(self))
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
        PartialEq,
        serde::Serialize,
        serde::Deserialize,
        strum_macros::Display,
        strum_macros::EnumIter,
        optml::Optml,
    )]
    enum PgTypePattern {
        Stdrt,
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize, optml::Optml)]
    #[serde(try_from = "PgTypeRecordRaw")]
    struct PgTypeRecord {
        pg_type: PgType,
        is_nl: pg_crud_macros_cmn::IsNl,
        pg_type_pattern: PgTypePattern,
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug, serde::Deserialize, optml::Optml)]
    struct PgTypeRecordRaw {
        pg_type: PgType,
        is_nl: pg_crud_macros_cmn::IsNl,
        pg_type_pattern: PgTypePattern,
    }
    impl TryFrom<PgTypeRecordRaw> for PgTypeRecord {
        type Error = String;
        fn try_from(v: PgTypeRecordRaw) -> Result<Self, Self::Error> {
            let cant_supp_nl_vrts_msg = "cant support nl vrts: ";
            match &v.pg_type.can_be_nl() {
                CanBeNl::False => {
                    if matches!(&v.is_nl, pg_crud_macros_cmn::IsNl::True) {
                        return Err(format!("{cant_supp_nl_vrts_msg}{v:#?}"));
                    }
                    Ok(Self {
                        pg_type: v.pg_type,
                        is_nl: v.is_nl,
                        pg_type_pattern: v.pg_type_pattern,
                    })
                }
                CanBeNl::True => Ok(Self {
                    pg_type: v.pg_type,
                    is_nl: v.is_nl,
                    pg_type_pattern: v.pg_type_pattern,
                }),
            }
        }
    }
    #[derive(Debug, serde::Deserialize, optml::Optml)]
    enum GenPgTypesConfigVrt {
        All,
        Concrete(Vec<PgTypeRecord>),
        Subset(Vec<PgType>),
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug, serde::Deserialize, optml::Optml)]
    struct GenPgTypesConfig {
        vrt: GenPgTypesConfigVrt,
        pg_tbl_cols_write_into_file: macros_helpers::ts_writer::ShouldWriteTsIntoFile,
        whole_write_into_file: macros_helpers::ts_writer::ShouldWriteTsIntoFile,
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug, optml::Optml)]
    enum PgTypeInitTryNew {
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
    impl TryFrom<&PgType> for PgTypeInitTryNew {
        type Error = ();
        fn try_from(v: &PgType) -> Result<Self, Self::Error> {
            match v {
                PgType::I16AsInt2
                | PgType::I32AsInt4
                | PgType::I64AsInt8
                | PgType::F32AsFloat4
                | PgType::F64AsFloat8
                | PgType::I16AsSmallSerialInitByPg
                | PgType::I32AsSerialInitByPg
                | PgType::I64AsBigSerialInitByPg
                | PgType::SqlxPgTypesPgMoneyAsMoney
                | PgType::BoolAsBool
                | PgType::StdVecVecU8AsBytea
                | PgType::SqlxPgTypesPgIntervalAsInterval
                | PgType::SqlxTypesUuidUuidAsUuidV4InitByPg
                | PgType::SqlxTypesUuidUuidAsUuidInitByClient
                | PgType::SqlxTypesIpnetworkIpNetworkAsInet
                | PgType::SqlxTypesMacAddressMacAddressAsMacAddr => Err(()),
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
    impl From<&PgTypeInitTryNew> for PgType {
        fn from(v: &PgTypeInitTryNew) -> Self {
            match v {
                PgTypeInitTryNew::StringAsText => Self::StringAsText,
                PgTypeInitTryNew::SqlxTypesChronoNaiveTimeAsTime => Self::SqlxTypesChronoNaiveTimeAsTime,
                PgTypeInitTryNew::SqlxTypesTimeTimeAsTime => Self::SqlxTypesTimeTimeAsTime,
                PgTypeInitTryNew::SqlxTypesChronoNaiveDateAsDate => Self::SqlxTypesChronoNaiveDateAsDate,
                PgTypeInitTryNew::SqlxTypesChronoNaiveDateTimeAsTimestamp => Self::SqlxTypesChronoNaiveDateTimeAsTimestamp,
                PgTypeInitTryNew::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz,
                PgTypeInitTryNew::SqlxPgTypesPgRangeI32AsInt4Range => Self::SqlxPgTypesPgRangeI32AsInt4Range,
                PgTypeInitTryNew::SqlxPgTypesPgRangeI64AsInt8Range => Self::SqlxPgTypesPgRangeI64AsInt8Range,
                PgTypeInitTryNew::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => Self::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange,
                PgTypeInitTryNew::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => Self::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange,
                PgTypeInitTryNew::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => Self::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange,
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
        SqlxTypesUuidUuidAsUuidV4InitByPg,
        SqlxTypesUuidUuidAsUuidInitByClient,
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
                | PgType::F64AsFloat8
                | PgType::I16AsSmallSerialInitByPg
                | PgType::I32AsSerialInitByPg
                | PgType::I64AsBigSerialInitByPg
                | PgType::SqlxPgTypesPgMoneyAsMoney
                | PgType::BoolAsBool
                | PgType::StdVecVecU8AsBytea
                | PgType::SqlxTypesUuidUuidAsUuidV4InitByPg
                | PgType::SqlxTypesUuidUuidAsUuidInitByClient
                | PgType::SqlxTypesIpnetworkIpNetworkAsInet
                | PgType::SqlxTypesMacAddressMacAddressAsMacAddr => Self::Derive,
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
    panic_loc::panic_loc();
    let gen_pg_types_config =
        match serde_json::from_str::<GenPgTypesConfig>(&input_ts.as_ref().to_string()) {
            Ok(v) => v,
            Err(er) => {
                let msg = format!("failed to parse GenPgTypesConfig: {er}");
                return macros_helpers::generated_rust_ts::GeneratedRustTs::from(
                    quote::quote! { compile_error!(#msg); },
                );
            }
        };
    let allow_clippy_arbitrary_src_item_ordering =
        token_patterns::AllowClippyArbitrarySrcItemOrdering;
    let as_ucc = naming::AsUcc;
    let col_sc = naming::ColSc;
    let contains_null_byte_ucc = naming::ContainsNullByteUcc;
    let core_default = token_patterns::CoreDefault;
    let cr_sc = naming::CrSc;
    let date_naive_sc = naming::DateNaiveSc;
    let date_naive_ucc = naming::DateNaiveUcc;
    let date_sc = naming::DateSc;
    let date_ucc = naming::DateUcc;
    let days_sc = naming::DaysSc;
    let earlier_date_not_supported_ucc = naming::EarlierDateNotSupportedUcc;
    let earliest_supported_date_sc = naming::EarliestSupportedDateSc;
    let end_sc = naming::EndSc;
    let end_ucc = naming::EndUcc;
    let eq_ucc = naming::EqUcc;
    let er_sc = naming::ErSc;
    let excluded_start_greater_than_excluded_end_ucc =
        naming::ExcludedStartGreaterThanExcludedEndUcc;
    let excluded_start_greater_than_included_end_ucc =
        naming::ExcludedStartGreaterThanIncludedEndUcc;
    let excluded_ucc = naming::ExcludedUcc;
    let f32_ts = token_patterns::F32;
    let gen_pg_types_mod_sc = naming::GenPgTypesModSc;
    let hour_sc = naming::HourSc;
    let i16_ts = token_patterns::I16;
    let i32_ts = token_patterns::I32;
    let i64_ts = token_patterns::I64;
    let included_end_cannot_be_max_ucc = naming::IncludedEndCannotBeMaxUcc;
    let included_start_greater_than_excluded_end_ucc =
        naming::IncludedStartGreaterThanExcludedEndUcc;
    let included_start_greater_than_included_end_ucc =
        naming::IncludedStartGreaterThanIncludedEndUcc;
    let included_ucc = naming::IncludedUcc;
    let incr_sc = naming::IncrSc;
    let invalid_hour_or_minute_or_second_or_microsecond_ucc =
        naming::InvalidHourOrMinuteOrSecondOrMicrosecondUcc;
    let max_sc = naming::MaxSc;
    let micro_sc = naming::MicroSc;
    let microsecond_sc = naming::MicrosecondSc;
    let microseconds_sc = naming::MicrosecondsSc;
    let min_sc = naming::MinSc;
    let minute_sc = naming::MinuteSc;
    let months_sc = naming::MonthsSc;
    let must_use = token_patterns::MustUse;
    let nanosecond_precision_is_not_supported_ucc = naming::NanosecondPrecisionIsNotSupportedUcc;
    let nanosecond_sc = naming::NanosecondSc;
    let near_zero_sc = naming::NearZeroSc;
    let negative_less_typical_sc = naming::NegativeLessTypicalSc;
    let negative_more_typical_sc = naming::NegativeMoreTypicalSc;
    let new_sc = naming::NewSc;
    let not_uuid_ucc = naming::NotUuidUcc;
    let opt_upd_sc = naming::OptUpdSc;
    let opt_vec_cr_sc = naming::OptVecCrSc;
    let pg_crud_cmn_dflt_some_one_el_call = token_patterns::PgCrudCmnDfltSomeOneElCall;
    let pg_type_pk_ucc = naming::PgTypePkUcc;
    let pg_type_ucc = naming::PgTypeUcc;
    let positive_less_typical_sc = naming::PositiveLessTypicalSc;
    let positive_more_typical_sc = naming::PositiveMoreTypicalSc;
    let query_sc = naming::QuerySc;
    let rd_ids_and_cr_into_rd_sc = naming::RdIdsAndCrIntoRdSc;
    let rd_ids_into_rd_sc = naming::RdIdsIntoRdSc;
    let rd_ids_into_tt_sc = naming::RdIdsIntoTtSc;
    let rd_ids_into_upd_sc = naming::RdIdsIntoUpdSc;
    let rd_ids_sc = naming::RdIdsSc;
    let rd_ids_to_2_dims_vec_rd_inn_sc = naming::RdIdsTo2DimsVecRdInnSc;
    let rd_ids_ucc = naming::RdIdsUcc;
    let rd_into_tt_sc = naming::RdIntoTtSc;
    let rd_sc = naming::RdSc;
    let rd_ucc = naming::RdUcc;
    let sec_sc = naming::SecSc;
    let second_sc = naming::SecondSc;
    let self_sc = naming::SelfSc;
    let self_ucc = naming::SelfUcc;
    let start_sc = naming::StartSc;
    let start_ucc = naming::StartUcc;
    let string_ts = token_patterns::StringTs;
    let time_sc = naming::TimeSc;
    let time_ucc = naming::TimeUcc;
    let to_err_string_sc = naming::ToErrStringSc;
    let try_new_sc = naming::TryNewSc;
    let tt_sc = naming::TtSc;
    let tt_ucc = naming::TtUcc;
    let u8_ts = token_patterns::U8;
    let u32_ts = token_patterns::U32;
    let unbounded_ucc = naming::UnboundedUcc;
    let upd_ucc = naming::UpdUcc;
    let v_sc = naming::VSc;
    let (cols_ts, pg_type_arr) = {
        let gen_vrts = |filter: Option<&[PgType]>| {
            <PgType as strum::IntoEnumIterator>::iter().filter(|el| filter.is_none_or(|f| f.contains(el))).fold(Vec::new(), |mut acc0, el| {
                match &el.can_be_nl() {
                    CanBeNl::False => {
                        acc0.push(PgTypeRecord {
                            pg_type: el,
                            is_nl: pg_crud_macros_cmn::IsNl::False,
                            pg_type_pattern: PgTypePattern::Stdrt,
                        });
                    },
                    CanBeNl::True => {
                        <pg_crud_macros_cmn::IsNl as strum::IntoEnumIterator>::iter().for_each(|el1| {
                            acc0.push(PgTypeRecord {
                                pg_type: el.clone(),
                                is_nl: el1,
                                pg_type_pattern: PgTypePattern::Stdrt,
                            });
                        });
                    },
                }
                acc0
            })
        };
        let acc = match gen_pg_types_config.vrt {
            GenPgTypesConfigVrt::All => gen_vrts(None),
            GenPgTypesConfigVrt::Subset(types) => gen_vrts(Some(&types)),
            GenPgTypesConfigVrt::Concrete(v) => v,
        };
        {
            let mut check_acc = Vec::with_capacity(acc.len());
            let duplicate_found = acc.iter().any(|el| {
                if check_acc.contains(&el) {
                    true
                } else {
                    check_acc.push(el);
                    false
                }
            });
            if duplicate_found {
                let msg_value = "536036f9: duplicate pg type config entry";
                return macros_helpers::generated_rust_ts::GeneratedRustTs::from(
                    quote::quote! {compile_error!(#msg_value);},
                );
            }
        }
        acc
    }.into_iter()
    .fold(
        Vec::new(),
        |mut acc, el| {
            #[derive(Clone, optml::Optml)]
            struct PgTypeRecordH {
                is_nl: pg_crud_macros_cmn::IsNl,
                pg_type_pattern: PgTypePattern,
            }
            fn gen_pg_type_record_h_vec(
                pg_type_record_h: PgTypeRecordH,
            ) -> Vec<PgTypeRecordH> {
                match &pg_type_record_h.is_nl {
                    pg_crud_macros_cmn::IsNl::False => vec![pg_type_record_h],
                    pg_crud_macros_cmn::IsNl::True => gen_pg_type_record_h_vec(PgTypeRecordH {
                        is_nl: pg_crud_macros_cmn::IsNl::False,
                        pg_type_pattern: PgTypePattern::Stdrt,
                    })
                    .into_iter()
                    .chain(std::iter::once(pg_type_record_h.clone()))
                    .collect(),
                }
            }
            let records_to_add = gen_pg_type_record_h_vec(PgTypeRecordH {
                is_nl: el.is_nl,
                pg_type_pattern: el.pg_type_pattern,
            })
            .into_iter()
            .map(|el0| PgTypeRecord {
                pg_type: el.pg_type.clone(),
                is_nl: el0.is_nl,
                pg_type_pattern: el0.pg_type_pattern,
            })
            .filter(|pg_type_record| !acc.contains(pg_type_record))
            .collect::<Vec<PgTypeRecord>>();
            acc.extend(records_to_add);
            acc
        },
    )
    .into_iter()
    .enumerate()
    .collect::<Vec<(usize, PgTypeRecord)>>()
    .iter()
    //.into_iter() //just for console prints ordering
    .map(|(i, el)| {
        enum PgTypeOrPgTypeTestCases {
            PgType,
            PgTypeTestCases,
        }
        enum CanBePk {
            False,
            True,
        }
        enum IsNnStdrtCanBePk {
            False,
            True,
        }
        enum StartOrEnd {
            End,
            Start,
        }
        enum IntRangeType {
            SqlxPgTypesPgRangeI32AsInt4Range,
            SqlxPgTypesPgRangeI64AsInt8Range,
        }
        fn gen_pg_range_conversion_ts(match_ts: &dyn quote::ToTokens, input_ts: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
            let arms_ts = quote::quote! {
                std::ops::Bound::Included(v_af65ccce) => std::ops::Bound::Included(#input_ts),
                std::ops::Bound::Excluded(v_af65ccce) => std::ops::Bound::Excluded(#input_ts),
                std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
            };
            quote::quote! {
                sqlx::postgres::types::PgRange {
                    start: match #match_ts.start { #arms_ts },
                    end: match #match_ts.end { #arms_ts },
                }
            }
        }
        let pg_type = &el.pg_type;
        let is_nl = &el.is_nl;
        let pg_type_pattern = &el.pg_type_pattern;
        let pg_type_init_try_new_try_from_pg_type = PgTypeInitTryNew::try_from(pg_type);
        let pg_type_deserialize = PgTypeDeserialize::from(pg_type);
        let range_try_from_pg_type = Range::try_from(pg_type);
        let range_try_from_pg_type_is_ok = range_try_from_pg_type.is_ok();
        let import = pg_crud_macros_cmn::Import::PgCrudCmn;
        let import_non_pk_pg_type_rd_ids_ts = quote::quote! {#import::NonPkPgTypeRdIds};
        let none_ts = quote::quote! {None};
        let dot_clone_ts = quote::quote! {.clone()};
        let mb_dot_clone_ts: &dyn quote::ToTokens = if matches!(&pg_type_pattern, PgTypePattern::Stdrt) &&
            matches!(&is_nl, pg_crud_macros_cmn::IsNl::False) && !matches!(
                pg_type,
                PgType::StdVecVecU8AsBytea | PgType::StringAsText
            )
        {
            &proc_macro2::TokenStream::new()
        } else {
            &dot_clone_ts
        };
        let gen_v_init_ts0 = |ts: &dyn quote::ToTokens| pg_crud_macros_cmn::gen_v_init_ts(&import, &ts);
        let gen_ident_str = |
            pg_type_prm: &PgType,
            is_nl_prm: &pg_crud_macros_cmn::IsNl,
            _pg_type_pattern_prm: &PgTypePattern
        | {
            let rust_type_name = RustTypeName::from(pg_type_prm);
            let pg_type_name = PgTypeName::from(pg_type_prm);
            let is_nl_rust = is_nl_prm.rust();
            let nn_or_nl_str = is_nl_prm.nn_or_nl_str();
            let (rust_part, pg_part) = (format!("{rust_type_name}"), format!("{pg_type_name}"));
            format!("{is_nl_rust}{rust_part}{as_ucc}{nn_or_nl_str}{pg_part}")
        };
        let gen_ident_ts = |
            pg_type_prm: &PgType,
            is_nl_prm: &pg_crud_macros_cmn::IsNl,
            pg_type_pattern_prm: &PgTypePattern
        | {
            let ident_str = gen_ident_str(
                pg_type_prm,
                is_nl_prm,
                pg_type_pattern_prm
            );
            let ident = quote::format_ident!("{}", ident_str);
            quote::quote! {#ident}
        };
        let ident = &gen_ident_ts(pg_type, is_nl, pg_type_pattern);
        let gen_ident_stdrt_nn_ts = |v: &PgType| gen_ident_ts(v, &pg_crud_macros_cmn::IsNl::False, &PgTypePattern::Stdrt);
        let ident_stdrt_nn_ucc = gen_ident_stdrt_nn_ts(pg_type);
        let gen_as_trait_ts = |ts: &dyn quote::ToTokens, pg_type_or_pg_type_test_cases: &PgTypeOrPgTypeTestCases| {
            let trait_ts = match &pg_type_or_pg_type_test_cases {
                PgTypeOrPgTypeTestCases::PgType => quote::quote! {PgType},
                PgTypeOrPgTypeTestCases::PgTypeTestCases => quote::quote! {PgTypeTestCases},
            };
            quote::quote! {<#ts as #import::#trait_ts>}
        };
        let gen_as_pg_type_ts = |ts: &dyn quote::ToTokens| gen_as_trait_ts(&ts, &PgTypeOrPgTypeTestCases::PgType);
        let gen_as_pg_type_test_cases_ts = |ts: &dyn quote::ToTokens| gen_as_trait_ts(&ts, &PgTypeOrPgTypeTestCases::PgTypeTestCases);
        let self_as_pg_type_ts = gen_as_pg_type_ts(&self_ucc);
        let ident_stdrt_nn_as_pg_type_ts = gen_as_pg_type_ts(&ident_stdrt_nn_ucc);
        let self_pg_type_as_pg_type_ts = gen_as_pg_type_ts(&quote::quote! {Self::#pg_type_ucc});
        let ident_stdrt_nn_as_pg_type_test_cases_ts = gen_as_pg_type_test_cases_ts(&ident_stdrt_nn_ucc);
        let gen_ident_stdrt_nn_orgn_ts = |pg_type_prm: &PgType| naming::prm::SelfOrgnUcc::from_tokens(
            &gen_ident_stdrt_nn_ts(pg_type_prm)
        );
        let ident_stdrt_nn_orgn_ucc = gen_ident_stdrt_nn_orgn_ts(pg_type);
        let ident_orgn_ucc = naming::prm::SelfOrgnUcc::from_tokens(&ident);
        let sqlx_types_chrono_naive_date_as_nn_date_orgn_ucc = gen_ident_stdrt_nn_orgn_ts(&PgType::SqlxTypesChronoNaiveDateAsDate);
        let sqlx_types_chrono_naive_time_as_nn_time_orgn_ucc = gen_ident_stdrt_nn_orgn_ts(&PgType::SqlxTypesChronoNaiveTimeAsTime);
        let sqlx_types_chrono_naive_date_time_as_nn_timestamp_orgn_ucc = gen_ident_stdrt_nn_orgn_ts(&PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp);
        let sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_nn_timestamptz_orgn_ucc = gen_ident_stdrt_nn_orgn_ts(&PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz);
        let gen_ident_stdrt_nn_orgn_try_new_er_ts = |pg_type_prm: &PgType| naming::prm::SelfOrgnTryNewErUcc::from_tokens(
            &gen_ident_stdrt_nn_ts(pg_type_prm)
        );
        let sqlx_types_chrono_naive_date_as_nn_date_orgn_try_new_er_ucc = gen_ident_stdrt_nn_orgn_try_new_er_ts(&PgType::SqlxTypesChronoNaiveDateAsDate);
        let sqlx_types_chrono_naive_time_as_nn_time_orgn_try_new_er_ucc = gen_ident_stdrt_nn_orgn_try_new_er_ts(&PgType::SqlxTypesChronoNaiveTimeAsTime);
        let sqlx_types_chrono_naive_date_time_as_nn_timestamp_orgn_try_new_er_ucc = gen_ident_stdrt_nn_orgn_try_new_er_ts(&PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp);
        let sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_nn_timestamptz_orgn_try_new_er_ucc = gen_ident_stdrt_nn_orgn_try_new_er_ts(&PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz);
        let inn_type_stdrt_nn_ts = match &pg_type {
            PgType::F32AsFloat4 => quote::quote! {f32},
            PgType::F64AsFloat8 => quote::quote! {f64},
            PgType::I16AsInt2 | PgType::I16AsSmallSerialInitByPg => quote::quote! {i16},
            PgType::I32AsInt4 | PgType::I32AsSerialInitByPg => quote::quote! {i32},
            PgType::I64AsInt8 | PgType::I64AsBigSerialInitByPg => quote::quote! {i64},
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
            PgType::SqlxTypesUuidUuidAsUuidV4InitByPg | PgType::SqlxTypesUuidUuidAsUuidInitByClient => quote::quote! {uuid::Uuid},
            PgType::SqlxTypesIpnetworkIpNetworkAsInet => quote::quote! {sqlx::types::ipnetwork::IpNetwork},
            PgType::SqlxTypesMacAddressMacAddressAsMacAddr => quote::quote! {sqlx::types::mac_address::MacAddress},
            PgType::SqlxPgTypesPgRangeI32AsInt4Range => quote::quote! {sqlx::postgres::types::PgRange<i32>},
            PgType::SqlxPgTypesPgRangeI64AsInt8Range => quote::quote! {sqlx::postgres::types::PgRange<i64>},
            PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => quote::quote! {sqlx::postgres::types::PgRange<sqlx::types::chrono::NaiveDate>},
            PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => quote::quote! {sqlx::postgres::types::PgRange<sqlx::types::chrono::NaiveDateTime>},
            PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => quote::quote! {sqlx::postgres::types::PgRange<sqlx::types::chrono::DateTime::<sqlx::types::chrono::Utc>>},
        };
        let ft_h_opt_ts = pg_crud_macros_cmn::gen_opt_type_dcl_ts(&ident_stdrt_nn_orgn_ucc);
        let ft_h: &dyn quote::ToTokens = match &pg_type_pattern {
            PgTypePattern::Stdrt => match &is_nl {
                pg_crud_macros_cmn::IsNl::False => &inn_type_stdrt_nn_ts,
                pg_crud_macros_cmn::IsNl::True => &ft_h_opt_ts,
            },
        };
        let gen_typical_pg_query_qb_ts = |ts: &dyn quote::ToTokens| match &is_nl {
            pg_crud_macros_cmn::IsNl::False => quote::quote! {
                if let Err(er) = #query_sc.as_mut().try_bind(#ts) {
                    return Err(#import::SqlxPostgresQueryBindEr::try_from(er.to_string()).unwrap_or_else(#import::SqlxPostgresQueryBindEr::from));
                }
                Ok(#query_sc)
            },
            pg_crud_macros_cmn::IsNl::True => quote::quote! {
                if let Err(er) = #query_sc.as_mut().try_bind(#ts.0.0) {
                    return Err(#import::SqlxPostgresQueryBindEr::try_from(er.to_string()).unwrap_or_else(#import::SqlxPostgresQueryBindEr::from));
                }
                Ok(#query_sc)
            },
        };
        let typical_qb_ts = gen_typical_pg_query_qb_ts(&v_sc);
        let ident_inn_type_opt_ts = pg_crud_macros_cmn::gen_opt_type_dcl_ts(&inn_type_stdrt_nn_ts);
        let ident_inn_type_ts: &dyn quote::ToTokens = match &el.pg_type_pattern {
            PgTypePattern::Stdrt => match &is_nl {
                pg_crud_macros_cmn::IsNl::False => &inn_type_stdrt_nn_ts,
                pg_crud_macros_cmn::IsNl::True => &ident_inn_type_opt_ts,
            },
        };
        let can_be_pk = match &pg_type {
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
            | PgType::SqlxTypesUuidUuidAsUuidInitByClient
            | PgType::SqlxTypesIpnetworkIpNetworkAsInet
            | PgType::SqlxTypesMacAddressMacAddressAsMacAddr
            | PgType::SqlxPgTypesPgRangeI32AsInt4Range
            | PgType::SqlxPgTypesPgRangeI64AsInt8Range
            | PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange
            | PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange
            | PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => CanBePk::False,
            PgType::I16AsSmallSerialInitByPg | PgType::I32AsSerialInitByPg | PgType::I64AsBigSerialInitByPg | PgType::SqlxTypesUuidUuidAsUuidV4InitByPg => CanBePk::True,
        };
        let is_stdrt_nn = if matches!((&pg_type_pattern, &is_nl), (PgTypePattern::Stdrt, pg_crud_macros_cmn::IsNl::False)) {
            pg_crud_macros_cmn::IsStdrtNn::True
        } else {
            pg_crud_macros_cmn::IsStdrtNn::False
        };
        let d_partial_ord = match &is_stdrt_nn {
            pg_crud_macros_cmn::IsStdrtNn::False => macros_helpers::derive_ts_builder::DPartialOrd::False,
            pg_crud_macros_cmn::IsStdrtNn::True => match &pg_type {
                PgType::I16AsInt2
                | PgType::I32AsInt4
                | PgType::I64AsInt8
                | PgType::F32AsFloat4
                | PgType::F64AsFloat8
                | PgType::I16AsSmallSerialInitByPg
                | PgType::I32AsSerialInitByPg
                | PgType::I64AsBigSerialInitByPg
                | PgType::BoolAsBool
                | PgType::StringAsText
                | PgType::StdVecVecU8AsBytea
                | PgType::SqlxTypesChronoNaiveTimeAsTime
                | PgType::SqlxTypesTimeTimeAsTime
                | PgType::SqlxTypesChronoNaiveDateAsDate
                | PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp
                | PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz
                | PgType::SqlxTypesUuidUuidAsUuidV4InitByPg => macros_helpers::derive_ts_builder::DPartialOrd::True,
                PgType::SqlxPgTypesPgMoneyAsMoney
                | PgType::SqlxPgTypesPgIntervalAsInterval
                | PgType::SqlxTypesUuidUuidAsUuidInitByClient
                | PgType::SqlxTypesIpnetworkIpNetworkAsInet
                | PgType::SqlxTypesMacAddressMacAddressAsMacAddr
                | PgType::SqlxPgTypesPgRangeI32AsInt4Range
                | PgType::SqlxPgTypesPgRangeI64AsInt8Range
                | PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange
                | PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange
                | PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => macros_helpers::derive_ts_builder::DPartialOrd::False,
            },
        };
        let is_nn_stdrt_can_be_pk = if matches!((&is_nl, &pg_type_pattern, &can_be_pk), (pg_crud_macros_cmn::IsNl::False, PgTypePattern::Stdrt, CanBePk::True)) {
            IsNnStdrtCanBePk::True
        } else {
            IsNnStdrtCanBePk::False
        };
        let gen_start_or_end_ucc = |start_or_end: &StartOrEnd| -> &dyn naming::DisplayPlusToTokens {
            match &start_or_end {
                StartOrEnd::End => &end_ucc,
                StartOrEnd::Start => &start_ucc,
            }
        };
        let gen_start_or_end_sc = |start_or_end: &StartOrEnd| -> &dyn naming::DisplayPlusToTokens {
            match &start_or_end {
                StartOrEnd::End => &end_sc,
                StartOrEnd::Start => &start_sc,
            }
        };
        let (ser_derive_or_impl, de_derive_or_impl) = if matches!(&is_stdrt_nn, pg_crud_macros_cmn::IsStdrtNn::True) {
            #[allow(clippy::arbitrary_source_item_ordering)]
            enum PrmNbr {
                Two,
                Three,
                Four,
            }
            impl PrmNbr {
                const fn get_i(&self) -> usize {
                    match &self {
                        Self::Two => 1,
                        Self::Three => 2,
                        Self::Four => 3,
                    }
                }
                fn get_vec_from_i_starting_with_zero(&self) -> Vec<usize> {
                    (0..=self.get_i()).collect()
                }
            }
            let self_dot_zero_ts = quote::quote! {#self_sc.0};
            let prm_nbr_two = PrmNbr::Two;
            let prm_nbr_three = PrmNbr::Three;
            let prm_nbr_four = PrmNbr::Four;
            let ident_stdrt_nn_orgn_dq_ts = gen_quotes::dq_ts(&ident_stdrt_nn_orgn_ucc);
            (
                {
                    let gen_impl_ser_for_ident_stdrt_nn_orgn_tokens = |ts: &dyn quote::ToTokens| {
                        quote::quote! {
                            #[allow(unused_qualifications)]
                            #[allow(clippy::absolute_paths)]
                            #allow_clippy_arbitrary_src_item_ordering
                            const _: () = {
                                #[allow(unused_extern_crates, clippy::useless_attribute)]
                                extern crate serde as _serde;
                                #[automatically_derived]
                                impl _serde::Serialize for #ident_stdrt_nn_orgn_ucc {
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
                    let gen_ser_cnt = |ts: &dyn quote::ToTokens| {
                        quote::quote! {_serde::Serializer::serialize_newtype_struct(__serializer, #ident_stdrt_nn_orgn_dq_ts, &#self_dot_zero_ts #ts)}
                    };
                    let gen_serde_state_init_ts = |prm_nbr: &PrmNbr| {
                        let prm_nbr_ts = {
                            let ts = prm_nbr.get_vec_from_i_starting_with_zero().into_iter().map(|_| quote::quote! {+ 1});
                            quote::quote! {#(#ts)*}
                        };
                        quote::quote! {
                            let mut __serde_state = _serde::Serializer::serialize_struct(__serializer, #ident_stdrt_nn_orgn_dq_ts, usize::from(false) #prm_nbr_ts)?;
                        }
                    };
                    let serde_state_init_two_fields_ts = gen_serde_state_init_ts(&prm_nbr_two);
                    let serde_state_init_three_fields_ts = gen_serde_state_init_ts(&prm_nbr_three);
                    let serde_state_init_four_fields_ts = gen_serde_state_init_ts(&prm_nbr_four);
                    let gen_ser_field_ts = |field_name: &dyn std::fmt::Display, third_prm_ts: &dyn quote::ToTokens| {
                        let field_name_dq_ts = gen_quotes::dq_ts(&field_name);
                        quote::quote! {_serde::ser::SerializeStruct::serialize_field(&mut __serde_state, #field_name_dq_ts, #third_prm_ts)?;}
                    };
                    let serde_ser_ser_struct_end_ts = quote::quote! {_serde::ser::SerializeStruct::end(__serde_state)};
                    let ser_cnt_start_end_ts = {
                        let gen_self_zero_tokens_ts = |ts: &dyn quote::ToTokens| {
                            quote::quote! {&#self_dot_zero_ts.#ts}
                        };
                        let start_ser_field_ts = gen_ser_field_ts(&start_sc, &gen_self_zero_tokens_ts(&start_sc));
                        let end_ser_field_ts = gen_ser_field_ts(&end_sc, &gen_self_zero_tokens_ts(&end_sc));
                        quote::quote! {
                            #serde_state_init_two_fields_ts
                            #start_ser_field_ts
                            #end_ser_field_ts
                            #serde_ser_ser_struct_end_ts
                        }
                    };
                    let impl_ser_for_nn_orgn_start_end_ts = gen_impl_ser_for_ident_stdrt_nn_orgn_tokens(&ser_cnt_start_end_ts);
                    let impl_ser_for_uuid_uuid_ts = gen_impl_ser_for_ident_stdrt_nn_orgn_tokens(&gen_ser_cnt(&quote::quote! {.to_string()}));
                    let gen_impl_ser_for_ident_stdrt_nn_orgn_start_end_range_tokens = |ts: &dyn quote::ToTokens| {
                        let gen_ser_field_match_std_ops_bound_ts = |start_or_end: &StartOrEnd| {
                            let start_or_end_ts = gen_start_or_end_sc(start_or_end);
                            gen_ser_field_ts(
                                &start_or_end_ts,
                                &quote::quote! {
                                    &match self.0.#start_or_end_ts {
                                        std::ops::Bound::Included(v_7d755c7c) => std::ops::Bound::Included(#ts::#try_new_sc(v_7d755c7c).map_err(_serde::ser::Error::custom)?),
                                        std::ops::Bound::Excluded(v_cfbe64e9) => std::ops::Bound::Excluded(#ts::#try_new_sc(v_cfbe64e9).map_err(_serde::ser::Error::custom)?),
                                        std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
                                    }
                                },
                            )
                        };
                        let start_ser_field_ts = gen_ser_field_match_std_ops_bound_ts(&StartOrEnd::Start);
                        let end_ser_field_ts = gen_ser_field_match_std_ops_bound_ts(&StartOrEnd::End);
                        gen_impl_ser_for_ident_stdrt_nn_orgn_tokens(&quote::quote! {
                            #serde_state_init_two_fields_ts
                            #start_ser_field_ts
                            #end_ser_field_ts
                            #serde_ser_ser_struct_end_ts
                        })
                    };
                    let gen_impl_ser_wrapping_self_zero_ts = |ts: &dyn quote::ToTokens|{
                        pg_crud_macros_cmn::DeriveOrImpl::Impl(macros_helpers::generated_rust_ts::GeneratedRustTs::from(gen_impl_ser_for_ident_stdrt_nn_orgn_tokens(
                            &gen_ser_cnt(&ts)
                        )))
                    };
                    let gen_four_field_time_ser_ts = |f1: &dyn quote::ToTokens, f2: &dyn quote::ToTokens, f3: &dyn quote::ToTokens, f4: &dyn quote::ToTokens| quote::quote! {
                        #serde_state_init_four_fields_ts
                        #f1
                        #f2
                        #f3
                        #f4
                        #serde_ser_ser_struct_end_ts
                    };
                    match &pg_type {
                        PgType::I16AsInt2
                        | PgType::I32AsInt4
                        | PgType::I64AsInt8
                        | PgType::F32AsFloat4
                        | PgType::F64AsFloat8
                        | PgType::I16AsSmallSerialInitByPg
                        | PgType::I32AsSerialInitByPg
                        | PgType::I64AsBigSerialInitByPg
                        | PgType::BoolAsBool
                        | PgType::StringAsText
                        | PgType::StdVecVecU8AsBytea
                        | PgType::SqlxTypesChronoNaiveDateAsDate
                        | PgType::SqlxTypesIpnetworkIpNetworkAsInet => pg_crud_macros_cmn::DeriveOrImpl::Derive,
                        PgType::SqlxPgTypesPgMoneyAsMoney => gen_impl_ser_wrapping_self_zero_ts(&quote::quote! {.0}),
                        PgType::SqlxTypesMacAddressMacAddressAsMacAddr => gen_impl_ser_wrapping_self_zero_ts(&quote::quote! {.bytes()}),
                        PgType::SqlxTypesChronoNaiveTimeAsTime => pg_crud_macros_cmn::DeriveOrImpl::Impl(macros_helpers::generated_rust_ts::GeneratedRustTs::from(gen_impl_ser_for_ident_stdrt_nn_orgn_tokens(&{
                            let gen_field_inn_type_stdrt_nn_ts_as_chrono_timelike_ts = |ts: &dyn quote::ToTokens| {
                                quote::quote! {&(<#inn_type_stdrt_nn_ts as chrono::Timelike>::#ts)}
                            };
                            let hour_ser_field_ts = gen_ser_field_ts(&hour_sc, &gen_field_inn_type_stdrt_nn_ts_as_chrono_timelike_ts(&quote::quote! {hour(&self.0)}));
                            let min_ser_field_ts = gen_ser_field_ts(&min_sc, &gen_field_inn_type_stdrt_nn_ts_as_chrono_timelike_ts(&quote::quote! {minute(&self.0)}));
                            let sec_ser_field_ts = gen_ser_field_ts(&sec_sc, &gen_field_inn_type_stdrt_nn_ts_as_chrono_timelike_ts(&quote::quote! {second(&self.0)}));
                            let micro_ser_field_ts = gen_ser_field_ts(
                                &micro_sc,
                                &gen_field_inn_type_stdrt_nn_ts_as_chrono_timelike_ts(&quote::quote! {
                                    #nanosecond_sc(&self.0).checked_div(1000).expect("aea037b7")
                                }),
                            );
                            gen_four_field_time_ser_ts(&hour_ser_field_ts, &min_ser_field_ts, &sec_ser_field_ts, &micro_ser_field_ts)
                        }))),
                        PgType::SqlxTypesTimeTimeAsTime => pg_crud_macros_cmn::DeriveOrImpl::Impl(macros_helpers::generated_rust_ts::GeneratedRustTs::from(gen_impl_ser_for_ident_stdrt_nn_orgn_tokens(&{
                            let gen_ser_field_self_zero_ts = |v: &dyn naming::DisplayPlusToTokens| gen_ser_field_ts(&v, &quote::quote! {&self.0.#v()});
                            let hour_ser_field_ts = gen_ser_field_self_zero_ts(&hour_sc);
                            let minute_ser_field_ts = gen_ser_field_self_zero_ts(&minute_sc);
                            let second_ser_field_ts = gen_ser_field_self_zero_ts(&second_sc);
                            let microsecond_ser_field_ts = gen_ser_field_self_zero_ts(&microsecond_sc);
                            gen_four_field_time_ser_ts(&hour_ser_field_ts, &minute_ser_field_ts, &second_ser_field_ts, &microsecond_ser_field_ts)
                        }))),
                        PgType::SqlxPgTypesPgIntervalAsInterval => pg_crud_macros_cmn::DeriveOrImpl::Impl(macros_helpers::generated_rust_ts::GeneratedRustTs::from(gen_impl_ser_for_ident_stdrt_nn_orgn_tokens(&{
                            let gen_ser_field_h_ts = |v: &dyn naming::DisplayPlusToTokens| gen_ser_field_ts(&v, &quote::quote! {&#self_dot_zero_ts.#v});
                            let months_ser_field_ts = gen_ser_field_h_ts(&months_sc);
                            let days_ser_field_ts = gen_ser_field_h_ts(&days_sc);
                            let microseconds_ser_field_ts = gen_ser_field_h_ts(&microseconds_sc);
                            quote::quote! {
                                #serde_state_init_three_fields_ts
                                #months_ser_field_ts
                                #days_ser_field_ts
                                #microseconds_ser_field_ts
                                #serde_ser_ser_struct_end_ts
                            }
                        }))),
                        PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp => pg_crud_macros_cmn::DeriveOrImpl::Impl(macros_helpers::generated_rust_ts::GeneratedRustTs::from(gen_impl_ser_for_ident_stdrt_nn_orgn_tokens(&{
                            enum DateOrTime {
                                Date,
                                Time,
                            }
                            let gen_ser_field_try_new_unwrap_ts = |date_or_time: &DateOrTime| {
                                let date_or_time_ts: &dyn naming::DisplayPlusToTokens = match &date_or_time {
                                    DateOrTime::Date => &date_sc,
                                    DateOrTime::Time => &time_sc,
                                };
                                gen_ser_field_ts(&date_or_time_ts, &{
                                    let ident_ts_date: &dyn quote::ToTokens = match &date_or_time {
                                        DateOrTime::Date => &sqlx_types_chrono_naive_date_as_nn_date_orgn_ucc,
                                        DateOrTime::Time => &sqlx_types_chrono_naive_time_as_nn_time_orgn_ucc,
                                    };
                                    quote::quote! {
                                        &match #ident_ts_date::#try_new_sc(self.0.#date_or_time_ts()) {
                                            Ok(v_b2ac0c33) => v_b2ac0c33,
                                            Err(er) => {
                                                return Err(_serde::ser::Error::custom(er));
                                            },
                                        }
                                    }
                                })
                            };
                            let date_ser_field_ts = gen_ser_field_try_new_unwrap_ts(&DateOrTime::Date);
                            let time_ser_field_ts = gen_ser_field_try_new_unwrap_ts(&DateOrTime::Time);
                            quote::quote! {
                                #serde_state_init_two_fields_ts
                                #date_ser_field_ts
                                #time_ser_field_ts
                                #serde_ser_ser_struct_end_ts
                            }
                        }))),
                        PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => pg_crud_macros_cmn::DeriveOrImpl::Impl(macros_helpers::generated_rust_ts::GeneratedRustTs::from(gen_impl_ser_for_ident_stdrt_nn_orgn_tokens(&{
                            enum DateNaiveOrTime {
                                Date,
                                Time,
                            }
                            let gen_ser_field_try_new_unwrap_ts = |date_naive_or_time: &DateNaiveOrTime| {
                                let date_naive_or_time_ts: &dyn naming::DisplayPlusToTokens = match &date_naive_or_time {
                                    DateNaiveOrTime::Date => &date_naive_sc,
                                    DateNaiveOrTime::Time => &time_sc,
                                };
                                gen_ser_field_ts(&date_naive_or_time_ts, &{
                                    let ident_ts_time: &dyn quote::ToTokens = match &date_naive_or_time {
                                        DateNaiveOrTime::Date => &sqlx_types_chrono_naive_date_as_nn_date_orgn_ucc,
                                        DateNaiveOrTime::Time => &sqlx_types_chrono_naive_time_as_nn_time_orgn_ucc,
                                    };
                                    quote::quote! {&#ident_ts_time::#try_new_sc(self.0.#date_naive_or_time_ts()).map_err(_serde::ser::Error::custom)?}
                                })
                            };
                            let date_naive_ser_field_ts = gen_ser_field_try_new_unwrap_ts(&DateNaiveOrTime::Date);
                            let time_ser_field_ts = gen_ser_field_try_new_unwrap_ts(&DateNaiveOrTime::Time);
                            quote::quote! {
                                #serde_state_init_two_fields_ts
                                #date_naive_ser_field_ts
                                #time_ser_field_ts
                                #serde_ser_ser_struct_end_ts
                            }
                        }))),
                        PgType::SqlxTypesUuidUuidAsUuidV4InitByPg | PgType::SqlxTypesUuidUuidAsUuidInitByClient => pg_crud_macros_cmn::DeriveOrImpl::Impl(macros_helpers::generated_rust_ts::GeneratedRustTs::from(impl_ser_for_uuid_uuid_ts)),
                        PgType::SqlxPgTypesPgRangeI32AsInt4Range | PgType::SqlxPgTypesPgRangeI64AsInt8Range => pg_crud_macros_cmn::DeriveOrImpl::Impl(macros_helpers::generated_rust_ts::GeneratedRustTs::from(impl_ser_for_nn_orgn_start_end_ts)),
                        PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => pg_crud_macros_cmn::DeriveOrImpl::Impl(macros_helpers::generated_rust_ts::GeneratedRustTs::from(gen_impl_ser_for_ident_stdrt_nn_orgn_start_end_range_tokens(&sqlx_types_chrono_naive_date_as_nn_date_orgn_ucc))),
                        PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => pg_crud_macros_cmn::DeriveOrImpl::Impl(macros_helpers::generated_rust_ts::GeneratedRustTs::from(gen_impl_ser_for_ident_stdrt_nn_orgn_start_end_range_tokens(&sqlx_types_chrono_naive_date_time_as_nn_timestamp_orgn_ucc))),
                        PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => pg_crud_macros_cmn::DeriveOrImpl::Impl(macros_helpers::generated_rust_ts::GeneratedRustTs::from(gen_impl_ser_for_ident_stdrt_nn_orgn_start_end_range_tokens(&sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_nn_timestamptz_orgn_ucc))),
                    }
                },
                pg_crud_macros_cmn::DeriveOrImpl::Derive
            )
        } else {
            (pg_crud_macros_cmn::DeriveOrImpl::Derive, pg_crud_macros_cmn::DeriveOrImpl::Derive)
        };
        let v_ident_inn_type_ts = quote::quote! {#v_sc: #ident_inn_type_ts};
        let ident_stdrt_nn_rd_ucc = naming::prm::SelfRdUcc::from_tokens(&ident_stdrt_nn_ucc);
        let ident_stdrt_nn_orgn_try_new_er_ucc = naming::prm::SelfOrgnTryNewErUcc::from_display(&ident_stdrt_nn_ucc);
        let ident_stdrt_nn_orgn_try_new_for_de_er_ucc = naming::prm::SelfOrgnTryNewForDeErUcc::from_display(&ident_stdrt_nn_ucc);
        let int_range_type_to_range_inn_type_ts = |int_range_type: &IntRangeType| -> proc_macro2::TokenStream {
            match &int_range_type {
                IntRangeType::SqlxPgTypesPgRangeI32AsInt4Range => quote::quote! {#i32_ts},
                IntRangeType::SqlxPgTypesPgRangeI64AsInt8Range => quote::quote! {#i64_ts},
            }
        };
        let gen_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_from_naive_utc_and_offset_ts = |ts: &dyn quote::ToTokens| {
            quote::quote! {sqlx::types::chrono::DateTime::<sqlx::types::chrono::Utc>::from_naive_utc_and_offset(
                #ts,
                sqlx::types::chrono::Utc
            )}
        };
        let gen_sqlx_types_chrono_naive_date_time_new_ts = |ts: &dyn quote::ToTokens| {
            quote::quote! {sqlx::types::chrono::NaiveDateTime::#new_sc(#ts)}
        };
        let gen_sqlx_types_time_time_from_hms_micro_unwrap_ts = |ts: &dyn quote::ToTokens| {
            quote::quote! {sqlx::types::time::Time::from_hms_micro(#ts).expect("7a1a18fa")}
        };
        let gen_pub_const_new_or_pub_try_new_ts = |ts: &dyn quote::ToTokens| {
            let pub_fn_new_or_try_new_ts = if pg_type_init_try_new_try_from_pg_type.is_ok() {
                &macros_helpers::gen_new_or_try_new::gen_pub_try_new_ts(
                    &proc_macro2::TokenStream::new(),
                    &v_ident_inn_type_ts,
                    &ident_stdrt_nn_orgn_try_new_er_ucc,
                    &quote::quote! {
                        match #ident_orgn_ucc::#try_new_sc(#v_sc) {
                            Ok(v_0f9f1a61) => Ok(Self(v_0f9f1a61)),
                            Err(er) => Err(er)
                        }
                    },
                )
            } else {
                &{
                    let self_ident_orgn_new_v_ts = quote::quote! {Self(#ident_orgn_ucc::#new_sc(#v_sc))};
                    if matches!(&pg_type_pattern, PgTypePattern::Stdrt)
                        && matches!(&is_nl, pg_crud_macros_cmn::IsNl::False)
                    {
                        macros_helpers::gen_new_or_try_new::gen_pub_const_new_ts(
                            &must_use,
                            &v_ident_inn_type_ts,
                            &self_ident_orgn_new_v_ts
                        )
                    } else {
                        macros_helpers::gen_new_or_try_new::gen_pub_new_ts(
                            &must_use,
                            &v_ident_inn_type_ts,
                            &self_ident_orgn_new_v_ts
                        )
                    }
                }
            };
            quote::quote! {
                impl #ts {
                    #pub_fn_new_or_try_new_ts
                }
            }
        };
        let derive_copy = match &pg_type {
            PgType::I16AsInt2 |
            PgType::I32AsInt4 |
            PgType::I64AsInt8 |
            PgType::F32AsFloat4 |
            PgType::F64AsFloat8 |
            PgType::I16AsSmallSerialInitByPg |
            PgType::I32AsSerialInitByPg |
            PgType::I64AsBigSerialInitByPg |
            PgType::SqlxPgTypesPgMoneyAsMoney |
            PgType::BoolAsBool |
            PgType::SqlxTypesChronoNaiveTimeAsTime |
            PgType::SqlxTypesTimeTimeAsTime |
            PgType::SqlxPgTypesPgIntervalAsInterval |
            PgType::SqlxTypesChronoNaiveDateAsDate |
            PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp |
            PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz |
            PgType::SqlxTypesUuidUuidAsUuidV4InitByPg |
            PgType::SqlxTypesUuidUuidAsUuidInitByClient |
            PgType::SqlxTypesIpnetworkIpNetworkAsInet |
            PgType::SqlxTypesMacAddressMacAddressAsMacAddr |
            PgType::SqlxPgTypesPgRangeI32AsInt4Range |
            PgType::SqlxPgTypesPgRangeI64AsInt8Range |
            PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange |
            PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange |
            PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => macros_helpers::derive_ts_builder::DCopy::True,
            PgType::StringAsText |
            PgType::StdVecVecU8AsBytea => macros_helpers::derive_ts_builder::DCopy::False,
        };
        let sqlx_types_chrono_naive_time_min_fn_ts = quote::quote! {sqlx_types_chrono_naive_time_min};
        let sqlx_types_chrono_naive_time_ten_fn_ts = quote::quote! {sqlx_types_chrono_naive_time_ten};
        let sqlx_types_chrono_naive_time_twenty_fn_ts = quote::quote! {sqlx_types_chrono_naive_time_twenty};
        let sqlx_types_chrono_naive_time_max_fn_ts = quote::quote! {sqlx_types_chrono_naive_time_max};
        let sqlx_types_chrono_naive_date_min_fn_ts = quote::quote! {sqlx_types_chrono_naive_date_min};
        let sqlx_types_chrono_naive_date_negative_less_typical_fn_ts = quote::quote! {sqlx_types_chrono_naive_date_negative_less_typical};
        let sqlx_types_chrono_naive_date_negative_more_typical_fn_ts = quote::quote! {sqlx_types_chrono_naive_date_negative_more_typical};
        let sqlx_types_chrono_naive_date_near_zero_fn_ts = quote::quote! {sqlx_types_chrono_naive_date_near_zero};
        let sqlx_types_chrono_naive_date_positive_less_typical_fn_ts = quote::quote! {sqlx_types_chrono_naive_date_positive_less_typical};
        let sqlx_types_chrono_naive_date_positive_more_typical_fn_ts = quote::quote! {sqlx_types_chrono_naive_date_positive_more_typical};
        let sqlx_types_chrono_naive_date_max_fn_ts = quote::quote! {sqlx_types_chrono_naive_date_max};
        let sqlx_types_chrono_naive_date_max_pred_opt_expect_fn_ts = quote::quote! {sqlx_types_chrono_naive_date_max_pred_opt_expect};
        let ident_ts = {
            let ident_ts = macros_helpers::derive_ts_builder::DTsBuilder::new()
                .make_pub()
                .d_debug()
                .d_clone()
                .d_copy()
                .d_partial_eq()
                .build_struct(
                    &proc_macro2::TokenStream::new(),
                    &ident,
                    &proc_macro2::TokenStream::new(),
                    &quote::quote! {;},
                );
            let mb_impl_ident_ts = if matches!(&pg_type_pattern, PgTypePattern::Stdrt) &&
                matches!(&is_nl, pg_crud_macros_cmn::IsNl::False)
            {
                enum IsConst {
                    False,
                    True,
                }
                let gen_inn_type_ts = |
                    is_const: IsConst,
                    name_ts: &dyn quote::ToTokens,
                    ts: &dyn quote::ToTokens
                |{
                    let mb_const_ts = match is_const {
                        IsConst::False => proc_macro2::TokenStream::new(),
                        IsConst::True => quote::quote! {const},
                    };
                    quote::quote! {
                        #mb_const_ts fn #name_ts() -> #ident_inn_type_ts {
                            #ts
                        }
                    }
                };
                let mb_min_inn_type_ts = {
                    let gen_min_inn_type_ts = |is_const: IsConst, ts: &dyn quote::ToTokens| gen_inn_type_ts(is_const, &quote::quote! {min_inn_type}, ts);
                    match &pg_type {
                        PgType::SqlxTypesChronoNaiveTimeAsTime => Some(
                            gen_min_inn_type_ts(
                                IsConst::True,
                                &quote::quote! {
                                    sqlx::types::chrono::NaiveTime::from_hms_micro_opt(0, 0, 0, 0).expect("000ddcc2")
                                }
                            )
                        ),
                        PgType::SqlxTypesTimeTimeAsTime => Some(
                            gen_min_inn_type_ts(
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
                        PgType::I16AsSmallSerialInitByPg |
                        PgType::I32AsSerialInitByPg |
                        PgType::I64AsBigSerialInitByPg |
                        PgType::SqlxPgTypesPgMoneyAsMoney |
                        PgType::BoolAsBool |
                        PgType::StringAsText |
                        PgType::StdVecVecU8AsBytea |
                        PgType::SqlxPgTypesPgIntervalAsInterval |
                        PgType::SqlxTypesChronoNaiveDateAsDate |
                        PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp |
                        PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz |
                        PgType::SqlxTypesUuidUuidAsUuidV4InitByPg |
                        PgType::SqlxTypesUuidUuidAsUuidInitByClient |
                        PgType::SqlxTypesIpnetworkIpNetworkAsInet |
                        PgType::SqlxTypesMacAddressMacAddressAsMacAddr |
                        PgType::SqlxPgTypesPgRangeI32AsInt4Range |
                        PgType::SqlxPgTypesPgRangeI64AsInt8Range |
                        PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange |
                        PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange |
                        PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => None,
                    }
                };
                let mb_slightly_more_than_min_inn_type_ts = {
                    let gen_slightly_more_than_min_inn_type_ts = |is_const: IsConst, ts: &dyn quote::ToTokens| gen_inn_type_ts(is_const, &quote::quote! {slightly_more_than_min_inn_type}, ts);
                    match &pg_type {
                        PgType::SqlxTypesChronoNaiveTimeAsTime => Some(
                            gen_slightly_more_than_min_inn_type_ts(
                                IsConst::True,
                                &quote::quote! {
                                    sqlx::types::chrono::NaiveTime::from_hms_micro_opt(0, 0, 0, 1).expect("9545a47c")
                                }
                            )
                        ),
                        PgType::SqlxTypesTimeTimeAsTime => Some(
                            gen_slightly_more_than_min_inn_type_ts(
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
                        PgType::I16AsSmallSerialInitByPg |
                        PgType::I32AsSerialInitByPg |
                        PgType::I64AsBigSerialInitByPg |
                        PgType::SqlxPgTypesPgMoneyAsMoney |
                        PgType::BoolAsBool |
                        PgType::StringAsText |
                        PgType::StdVecVecU8AsBytea |
                        PgType::SqlxPgTypesPgIntervalAsInterval |
                        PgType::SqlxTypesChronoNaiveDateAsDate |
                        PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp |
                        PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz |
                        PgType::SqlxTypesUuidUuidAsUuidV4InitByPg |
                        PgType::SqlxTypesUuidUuidAsUuidInitByClient |
                        PgType::SqlxTypesIpnetworkIpNetworkAsInet |
                        PgType::SqlxTypesMacAddressMacAddressAsMacAddr |
                        PgType::SqlxPgTypesPgRangeI32AsInt4Range |
                        PgType::SqlxPgTypesPgRangeI64AsInt8Range |
                        PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange |
                        PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange |
                        PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => None,
                    }
                };
                let mb_middle_inn_type_ts = {
                    let gen_middle_inn_type_ts = |is_const: IsConst, ts: &dyn quote::ToTokens| gen_inn_type_ts(is_const, &quote::quote! {middle_inn_type}, ts);
                    match &pg_type {
                        PgType::SqlxTypesChronoNaiveTimeAsTime => Some(
                            gen_middle_inn_type_ts(
                                IsConst::True,
                                &quote::quote! {
                                    sqlx::types::chrono::NaiveTime::from_hms_micro_opt(0, 0, 0, 0).expect("0dafc3fc")
                                }
                            )
                        ),
                        PgType::SqlxTypesTimeTimeAsTime => Some(
                            gen_middle_inn_type_ts(
                                IsConst::False,
                                &quote::quote! {
                                    sqlx::types::time::Time::from_hms_micro(0, 0, 0, 0).expect("d2ec329f")
                                }
                            )
                        ),
                        PgType::SqlxTypesChronoNaiveDateAsDate => Some(
                            gen_middle_inn_type_ts(
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
                        PgType::I16AsSmallSerialInitByPg |
                        PgType::I32AsSerialInitByPg |
                        PgType::I64AsBigSerialInitByPg |
                        PgType::SqlxPgTypesPgMoneyAsMoney |
                        PgType::BoolAsBool |
                        PgType::StringAsText |
                        PgType::StdVecVecU8AsBytea |
                        PgType::SqlxPgTypesPgIntervalAsInterval |
                        PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp |
                        PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz |
                        PgType::SqlxTypesUuidUuidAsUuidV4InitByPg |
                        PgType::SqlxTypesUuidUuidAsUuidInitByClient |
                        PgType::SqlxTypesIpnetworkIpNetworkAsInet |
                        PgType::SqlxTypesMacAddressMacAddressAsMacAddr |
                        PgType::SqlxPgTypesPgRangeI32AsInt4Range |
                        PgType::SqlxPgTypesPgRangeI64AsInt8Range |
                        PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange |
                        PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange |
                        PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => None,
                    }
                };
                let mb_slightly_more_than_middle_inn_type_ts = {
                    let gen_slightly_more_than_middle_inn_type_ts = |is_const: IsConst, ts: &dyn quote::ToTokens| gen_inn_type_ts(is_const, &quote::quote! {slightly_more_than_middle_inn_type}, ts);
                    match &pg_type {
                        PgType::SqlxTypesChronoNaiveTimeAsTime => Some(
                            gen_slightly_more_than_middle_inn_type_ts(
                                IsConst::True,
                                &quote::quote! {
                                    sqlx::types::chrono::NaiveTime::from_hms_micro_opt(0, 0, 0, 1).expect("235276a7")
                                }
                            )
                        ),
                        PgType::SqlxTypesTimeTimeAsTime => Some(
                            gen_slightly_more_than_middle_inn_type_ts(
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
                        PgType::I16AsSmallSerialInitByPg |
                        PgType::I32AsSerialInitByPg |
                        PgType::I64AsBigSerialInitByPg |
                        PgType::SqlxPgTypesPgMoneyAsMoney |
                        PgType::BoolAsBool |
                        PgType::StringAsText |
                        PgType::StdVecVecU8AsBytea |
                        PgType::SqlxPgTypesPgIntervalAsInterval |
                        PgType::SqlxTypesChronoNaiveDateAsDate |
                        PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp |
                        PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz |
                        PgType::SqlxTypesUuidUuidAsUuidV4InitByPg |
                        PgType::SqlxTypesUuidUuidAsUuidInitByClient |
                        PgType::SqlxTypesIpnetworkIpNetworkAsInet |
                        PgType::SqlxTypesMacAddressMacAddressAsMacAddr |
                        PgType::SqlxPgTypesPgRangeI32AsInt4Range |
                        PgType::SqlxPgTypesPgRangeI64AsInt8Range |
                        PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange |
                        PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange |
                        PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => None,
                    }
                };
                let mb_max_inn_type_ts = {
                    let gen_max_inn_type_ts = |is_const: IsConst, ts: &dyn quote::ToTokens| gen_inn_type_ts(is_const, &quote::quote! {max_inn_type}, ts);
                    match &pg_type {
                        PgType::SqlxTypesChronoNaiveTimeAsTime => Some(
                            gen_max_inn_type_ts(
                                IsConst::True,
                                &quote::quote! {
                                    sqlx::types::chrono::NaiveTime::from_hms_micro_opt(23, 59, 59, 999_999).expect("b217e3bf")
                                }
                            )
                        ),
                        PgType::SqlxTypesChronoNaiveDateAsDate => Some(
                            gen_max_inn_type_ts(
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
                        PgType::I16AsSmallSerialInitByPg |
                        PgType::I32AsSerialInitByPg |
                        PgType::I64AsBigSerialInitByPg |
                        PgType::SqlxPgTypesPgMoneyAsMoney |
                        PgType::BoolAsBool |
                        PgType::StringAsText |
                        PgType::StdVecVecU8AsBytea |
                        PgType::SqlxTypesTimeTimeAsTime |
                        PgType::SqlxPgTypesPgIntervalAsInterval |
                        PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp |
                        PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz |
                        PgType::SqlxTypesUuidUuidAsUuidV4InitByPg |
                        PgType::SqlxTypesUuidUuidAsUuidInitByClient |
                        PgType::SqlxTypesIpnetworkIpNetworkAsInet |
                        PgType::SqlxTypesMacAddressMacAddressAsMacAddr |
                        PgType::SqlxPgTypesPgRangeI32AsInt4Range |
                        PgType::SqlxPgTypesPgRangeI64AsInt8Range |
                        PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange |
                        PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange |
                        PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => None,
                    }
                };
                let mb_slightly_less_than_max_inn_type_ts = {
                    let gen_slightly_less_than_max_inn_type_ts = |is_const: IsConst, ts: &dyn quote::ToTokens| gen_inn_type_ts(is_const, &quote::quote! {slightly_less_than_max_inn_type}, ts);
                    match &pg_type {
                        PgType::SqlxTypesChronoNaiveTimeAsTime => Some(
                            gen_slightly_less_than_max_inn_type_ts(
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
                        PgType::I16AsSmallSerialInitByPg |
                        PgType::I32AsSerialInitByPg |
                        PgType::I64AsBigSerialInitByPg |
                        PgType::SqlxPgTypesPgMoneyAsMoney |
                        PgType::BoolAsBool |
                        PgType::StringAsText |
                        PgType::StdVecVecU8AsBytea |
                        PgType::SqlxTypesTimeTimeAsTime |
                        PgType::SqlxPgTypesPgIntervalAsInterval |
                        PgType::SqlxTypesChronoNaiveDateAsDate |
                        PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp |
                        PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz |
                        PgType::SqlxTypesUuidUuidAsUuidV4InitByPg |
                        PgType::SqlxTypesUuidUuidAsUuidInitByClient |
                        PgType::SqlxTypesIpnetworkIpNetworkAsInet |
                        PgType::SqlxTypesMacAddressMacAddressAsMacAddr |
                        PgType::SqlxPgTypesPgRangeI32AsInt4Range |
                        PgType::SqlxPgTypesPgRangeI64AsInt8Range |
                        PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange |
                        PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange |
                        PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => None,
                    }
                };
                let mb_rd_inn_inits_ts = {
                    let gen_fn_ts: &dyn Fn(
                        &dyn quote::ToTokens,
                        &dyn quote::ToTokens,
                    ) -> proc_macro2::TokenStream = &|name_ts, ts_prm| quote::quote! {
                        const fn #name_ts() -> #ident_inn_type_ts {
                            #ts_prm
                        }
                    };
                    match &pg_type {
                        PgType::I16AsInt2 |
                        PgType::I32AsInt4 |
                        PgType::I64AsInt8 |
                        PgType::F32AsFloat4 |
                        PgType::F64AsFloat8 |
                        PgType::I16AsSmallSerialInitByPg |
                        PgType::I32AsSerialInitByPg |
                        PgType::I64AsBigSerialInitByPg |
                        PgType::SqlxPgTypesPgMoneyAsMoney |
                        PgType::BoolAsBool |
                        PgType::StringAsText |
                        PgType::StdVecVecU8AsBytea |
                        PgType::SqlxTypesTimeTimeAsTime |
                        PgType::SqlxPgTypesPgIntervalAsInterval |
                        PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp |
                        PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz |
                        PgType::SqlxTypesUuidUuidAsUuidV4InitByPg |
                        PgType::SqlxTypesUuidUuidAsUuidInitByClient |
                        PgType::SqlxTypesIpnetworkIpNetworkAsInet |
                        PgType::SqlxTypesMacAddressMacAddressAsMacAddr |
                        PgType::SqlxPgTypesPgRangeI32AsInt4Range |
                        PgType::SqlxPgTypesPgRangeI64AsInt8Range |
                        PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange |
                        PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange |
                        PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => None,
                        PgType::SqlxTypesChronoNaiveTimeAsTime => Some({
                            let ser_de_arr_ts = [
                                (&sqlx_types_chrono_naive_time_min_fn_ts, &quote::quote! {0,0,0,0}),
                                (&sqlx_types_chrono_naive_time_ten_fn_ts, &quote::quote! {10,10,10,10}),
                                (&sqlx_types_chrono_naive_time_twenty_fn_ts, &quote::quote! {20,20,20,20}),
                                (&sqlx_types_chrono_naive_time_max_fn_ts, &quote::quote! {23,59,59,999_999}),
                            ].iter().map(|(name_ts, prms_ts)| quote::quote! {
                                const fn #name_ts() -> #ident_inn_type_ts {
                                    #ident_inn_type_ts::from_hms_micro_opt(#prms_ts).expect("149e01cc")
                                }
                            }).collect::<Vec<proc_macro2::TokenStream>>();
                            quote::quote! {#(#ser_de_arr_ts)*}
                        }),
                        PgType::SqlxTypesChronoNaiveDateAsDate => Some({
                            let ser_de_arr_ts = {
                                let gen_fn_ident_inn_type_ts: &dyn Fn(
                                    &dyn quote::ToTokens,
                                    &dyn quote::ToTokens,
                                ) -> proc_macro2::TokenStream = &|name_ts, ts_prm| {
                                    gen_fn_ts(
                                        name_ts,
                                        &quote::quote! {#ident_inn_type_ts::#ts_prm},
                                    )
                                };
                                [
                                    gen_fn_ident_inn_type_ts(
                                        &sqlx_types_chrono_naive_date_max_fn_ts,
                                        &quote::quote! { MAX }
                                    ),
                                    gen_fn_ts(
                                        &sqlx_types_chrono_naive_date_max_pred_opt_expect_fn_ts,
                                        &quote::quote! {Self::#sqlx_types_chrono_naive_date_max_fn_ts().pred_opt().expect("b7e16bf1")}
                                    )
                                ]
                                .into_iter()
                                .chain(
                                    [
                                        (&sqlx_types_chrono_naive_date_min_fn_ts, &quote::quote! { -4713, 12, 31 }),
                                        (&sqlx_types_chrono_naive_date_negative_less_typical_fn_ts, &quote::quote! { -2000, 1, 1 }),
                                        (&sqlx_types_chrono_naive_date_negative_more_typical_fn_ts, &quote::quote! { -1000, 1, 1 }),
                                        (&sqlx_types_chrono_naive_date_near_zero_fn_ts, &quote::quote! { 0, 1, 1 }),
                                        (&sqlx_types_chrono_naive_date_positive_less_typical_fn_ts, &quote::quote! { 1000, 1, 1 }),
                                        (&sqlx_types_chrono_naive_date_positive_more_typical_fn_ts, &quote::quote! { 2000, 1, 1 }),
                                    ]
                                    .into_iter()
                                    .map(|(name_ts, prms_ts)| {
                                        gen_fn_ident_inn_type_ts(
                                            name_ts,
                                            &quote::quote! {
                                                from_ymd_opt(#prms_ts)
                                                    .expect("d25ee0e9")
                                            }
                                        )
                                    })
                                ).collect::<Vec<proc_macro2::TokenStream>>()
                            };
                            quote::quote! {#(#ser_de_arr_ts)*}
                        }),
                    }
                };
                if mb_min_inn_type_ts.is_some() ||
                    mb_slightly_more_than_min_inn_type_ts.is_some() ||
                    mb_middle_inn_type_ts.is_some() ||
                    mb_slightly_more_than_middle_inn_type_ts.is_some() ||
                    mb_max_inn_type_ts.is_some() ||
                    mb_slightly_less_than_max_inn_type_ts.is_some() ||
                    mb_rd_inn_inits_ts.is_some()
                {
                    quote::quote! {
                        #allow_clippy_arbitrary_src_item_ordering
                        impl #ident {
                            #mb_min_inn_type_ts
                            #mb_slightly_more_than_min_inn_type_ts
                            #mb_middle_inn_type_ts
                            #mb_slightly_more_than_middle_inn_type_ts
                            #mb_max_inn_type_ts
                            #mb_slightly_less_than_max_inn_type_ts
                            #mb_rd_inn_inits_ts
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
                #ident_ts
                #mb_impl_ident_ts
            }
        };
        let sqlx_types_chrono_naive_date_as_date_stdrt_nn_orig_ts = naming::prm::SelfOrgnUcc::from_tokens(&gen_ident_stdrt_nn_ts(&PgType::SqlxTypesChronoNaiveDateAsDate));
        let ident_upd_ucc = naming::prm::SelfUpdUcc::from_tokens(&ident);
        let sqlx_encode_self_dot_zero_ts = quote::quote! {#self_sc.0};
        let ident_orgn_ts = {
            let ident_orgn_ts = macros_helpers::derive_ts_builder::DTsBuilder::new()
                .make_pub()
                .d_debug()
                .d_clone()
                .d_copy_if(derive_copy)
                .d_partial_eq()
                .d_eq_if(match &is_nn_stdrt_can_be_pk {
                    IsNnStdrtCanBePk::False => macros_helpers::derive_ts_builder::DEq::False,
                    IsNnStdrtCanBePk::True => macros_helpers::derive_ts_builder::DEq::True,
                })
                .d_partial_ord_if(d_partial_ord)
                .d_ord_if(match &is_nn_stdrt_can_be_pk {
                    IsNnStdrtCanBePk::False => macros_helpers::derive_ts_builder::DOrd::False,
                    IsNnStdrtCanBePk::True => macros_helpers::derive_ts_builder::DOrd::True,
                })
                .d_serde_serialize_if(match &ser_derive_or_impl {
                    pg_crud_macros_cmn::DeriveOrImpl::Derive => macros_helpers::derive_ts_builder::DSerdeSerialize::True,
                    pg_crud_macros_cmn::DeriveOrImpl::Impl(_) => macros_helpers::derive_ts_builder::DSerdeSerialize::False,
                })
                .d_serde_deserialize_if(match &de_derive_or_impl {
                    pg_crud_macros_cmn::DeriveOrImpl::Derive => macros_helpers::derive_ts_builder::DSerdeDeserialize::True,
                    pg_crud_macros_cmn::DeriveOrImpl::Impl(_) => macros_helpers::derive_ts_builder::DSerdeDeserialize::False,
                })
                .build_struct(
                    &if matches!(&is_stdrt_nn, pg_crud_macros_cmn::IsStdrtNn::True) {
                        let gen_serde_from_ts = |ts: &dyn quote::ToTokens|quote::quote! {#[serde(from = #ts)]};
                        let gen_serde_try_from_ts = |ts: &dyn quote::ToTokens|quote::quote! {#[serde(try_from = #ts)]};
                        match &pg_type {
                            PgType::I16AsInt2 |
                            PgType::I32AsInt4 |
                            PgType::I64AsInt8 |
                            PgType::F32AsFloat4 |
                            PgType::F64AsFloat8 |
                            PgType::I16AsSmallSerialInitByPg |
                            PgType::I32AsSerialInitByPg |
                            PgType::I64AsBigSerialInitByPg |
                            PgType::BoolAsBool |
                            PgType::StdVecVecU8AsBytea |
                            PgType::SqlxTypesIpnetworkIpNetworkAsInet => proc_macro2::TokenStream::new(),
                            PgType::SqlxPgTypesPgMoneyAsMoney => gen_serde_from_ts(&quote::quote! {"i64"}),
                            PgType::SqlxTypesChronoNaiveTimeAsTime => gen_serde_try_from_ts(&quote::quote! {"(u32,u32,u32,u32)"}),
                            PgType::SqlxTypesTimeTimeAsTime => gen_serde_try_from_ts(&quote::quote! {"(u8,u8,u8,u32)"}),
                            PgType::SqlxPgTypesPgIntervalAsInterval => gen_serde_from_ts(&quote::quote! {"(i32,i32,i64)"}),
                            PgType::SqlxTypesChronoNaiveDateAsDate => gen_serde_try_from_ts(&quote::quote! {"sqlx::types::chrono::NaiveDate"}),
                            PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp |
                            PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => gen_serde_from_ts(&gen_quotes::dq_ts(&format!("({sqlx_types_chrono_naive_date_as_date_stdrt_nn_orig_ts},SqlxTypesChronoNaiveTimeAsNnTimeOrgn)"))),
                            PgType::StringAsText |
                            PgType::SqlxTypesUuidUuidAsUuidV4InitByPg |
                            PgType::SqlxTypesUuidUuidAsUuidInitByClient => quote::quote! {#[serde(try_from = "String")]},
                            PgType::SqlxTypesMacAddressMacAddressAsMacAddr => quote::quote! {#[serde(from = "[u8; 6]")]},
                            PgType::SqlxPgTypesPgRangeI32AsInt4Range => quote::quote! {#[serde(try_from = "(std::ops::Bound<i32>,std::ops::Bound<i32>)")]},
                            PgType::SqlxPgTypesPgRangeI64AsInt8Range => quote::quote! {#[serde(try_from = "(std::ops::Bound<i64>,std::ops::Bound<i64>)")]},
                            PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => {
                                let bound = format!("std::ops::Bound<{sqlx_types_chrono_naive_date_as_date_stdrt_nn_orig_ts}>");
                                let ts = gen_quotes::dq_ts(&format!("({bound},{bound})"));
                                quote::quote! {#[serde(from = #ts)]}
                            },
                            PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => quote::quote! {#[serde(from = "(std::ops::Bound<SqlxTypesChronoNaiveDateTimeAsNnTimestampOrgn>,std::ops::Bound<SqlxTypesChronoNaiveDateTimeAsNnTimestampOrgn>)")]},//todo reuse name
                            PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => quote::quote! {#[serde(from = "(std::ops::Bound<SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNnTimestampTzOrgn>,std::ops::Bound<SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNnTimestampTzOrgn>)")]},//todo reuse name
                        }
                    }
                    else {
                        proc_macro2::TokenStream::new()
                    },
                    &ident_orgn_ucc,
                    &proc_macro2::TokenStream::new(),
                    &quote::quote! {(#ft_h);},
                );
            let gen_loc_var_ts = |name_ts: &dyn quote::ToTokens, ts: &dyn quote::ToTokens|quote::quote! {
                #name_ts {
                    loc: loc_lib::loc::Loc,
                    #ts
                }
            };
            let gen_int_range_type_er_vrts_ts = |int_range_type: &IntRangeType| {
                let range_inn_type_ts = int_range_type_to_range_inn_type_ts(int_range_type);
                let (
                    included_start_greater_than_included_end_ts,
                    included_start_greater_than_excluded_end_ts,
                    excluded_start_greater_than_included_end_ts,
                    excluded_start_greater_than_excluded_end_ts
                ) = {
                    let gen_ts = |ts: &dyn quote::ToTokens|gen_loc_var_ts(
                        &ts,
                        &quote::quote! {
                            #[eo_to_err_string_serde]
                            #start_sc: #range_inn_type_ts,
                            #[eo_to_err_string_serde]
                            #end_sc: #range_inn_type_ts,
                        }
                    );
                    (
                        gen_ts(&included_start_greater_than_included_end_ucc),
                        gen_ts(&included_start_greater_than_excluded_end_ucc),
                        gen_ts(&excluded_start_greater_than_included_end_ucc),
                        gen_ts(&excluded_start_greater_than_excluded_end_ucc)
                    )
                };
                let included_end_cannot_be_max_ucc_ts = gen_loc_var_ts(
                    &included_end_cannot_be_max_ucc,
                    &quote::quote! {
                        #[eo_to_err_string_serde]
                        #end_sc: #range_inn_type_ts,
                    }
                );
                quote::quote! {
                    #included_start_greater_than_included_end_ts,
                    #included_start_greater_than_excluded_end_ts,
                    #excluded_start_greater_than_included_end_ts,
                    #excluded_start_greater_than_excluded_end_ts,
                    #included_end_cannot_be_max_ucc_ts,
                }
            };
            let nanosecond_precision_is_not_supported_vrt_try_new_ts = gen_loc_var_ts(
                &nanosecond_precision_is_not_supported_ucc,
                &quote::quote! {
                    #[eo_to_err_string_serde]
                    #v_sc: #string_ts,
                }
            );
            let sqlx_types_chrono_naive_date_as_date_try_new_er_vrts_ts = gen_loc_var_ts(
                &earlier_date_not_supported_ucc,
                &quote::quote! {
                    #[eo_to_err_string_serde]
                    value: #string_ts,
                    #[eo_to_err_string_serde]
                    #earliest_supported_date_sc: #string_ts,
                }
            );
            let string_as_text_try_new_er_vrts_ts = gen_loc_var_ts(
                &contains_null_byte_ucc,
                &quote::quote! {
                    #[eo_to_err_string_serde]
                    #v_sc: #ident_inn_type_ts,
                }
            );
            let uuid_as_uuid_v4_as_string_try_new_er_vrts_ts = gen_loc_var_ts(
                &not_uuid_ucc,
                &quote::quote! {
                    #[eo_to_err_string_serde]
                    #v_sc: String,
                }
            );
            let mb_pub_enum_ident_stdrt_nn_orgn_try_new_er_ts = if matches!(&is_stdrt_nn, pg_crud_macros_cmn::IsStdrtNn::True)
                && let Ok(pg_type_init_try_new) = &pg_type_init_try_new_try_from_pg_type
            {
                let serde_er_enum_ts = pg_crud_macros_cmn::ts_helpers::serde_er_enum_d_ts_builder()
                    .build_enum(
                        &proc_macro2::TokenStream::new(),
                        &ident_stdrt_nn_orgn_try_new_er_ucc,
                        &proc_macro2::TokenStream::new(),
                        &{
                            let gen_ts = |ts: &dyn quote::ToTokens| {
                                let (start_vrt_ts, end_vrt_ts) = {
                                    let gen_vrt_ts = |start_or_end: &StartOrEnd| gen_loc_var_ts(
                                        &gen_start_or_end_ucc(start_or_end),
                                        &quote::quote! {
                                            #[eo_loc]
                                            #er_sc: #ts,
                                        }
                                    );
                                    (gen_vrt_ts(&StartOrEnd::Start), gen_vrt_ts(&StartOrEnd::End))
                                };
                                quote::quote! {
                                    #start_vrt_ts,
                                    #end_vrt_ts,
                                }
                            };
                            let time_var_ts = gen_loc_var_ts(
                                &time_ucc,
                                &quote::quote! {
                                    #[eo_loc]
                                    #er_sc: #sqlx_types_chrono_naive_time_as_nn_time_orgn_try_new_er_ucc,
                                }
                            );
                            let ts: &dyn quote::ToTokens = match &pg_type_init_try_new {
                                PgTypeInitTryNew::StringAsText => &string_as_text_try_new_er_vrts_ts,
                                PgTypeInitTryNew::SqlxTypesChronoNaiveTimeAsTime | PgTypeInitTryNew::SqlxTypesTimeTimeAsTime => &nanosecond_precision_is_not_supported_vrt_try_new_ts,
                                PgTypeInitTryNew::SqlxTypesChronoNaiveDateAsDate => &sqlx_types_chrono_naive_date_as_date_try_new_er_vrts_ts,
                                PgTypeInitTryNew::SqlxTypesChronoNaiveDateTimeAsTimestamp | PgTypeInitTryNew::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => &{
                                    let date_name_ucc: &dyn naming::DisplayPlusToTokens = if matches!(&pg_type_init_try_new, PgTypeInitTryNew::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz) {
                                        &date_naive_ucc
                                    } else {
                                        &date_ucc
                                    };
                                    let date_var_ts = gen_loc_var_ts(
                                        date_name_ucc,
                                        &quote::quote! {
                                            #[eo_loc]
                                            #er_sc: #sqlx_types_chrono_naive_date_as_nn_date_orgn_try_new_er_ucc,
                                        }
                                    );
                                    quote::quote! {
                                        #date_var_ts,
                                        #time_var_ts,
                                    }
                                },
                                PgTypeInitTryNew::SqlxPgTypesPgRangeI32AsInt4Range => &gen_int_range_type_er_vrts_ts(&IntRangeType::SqlxPgTypesPgRangeI32AsInt4Range),
                                PgTypeInitTryNew::SqlxPgTypesPgRangeI64AsInt8Range => &gen_int_range_type_er_vrts_ts(&IntRangeType::SqlxPgTypesPgRangeI64AsInt8Range),
                                PgTypeInitTryNew::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => &gen_ts(
                                    &sqlx_types_chrono_naive_date_as_nn_date_orgn_try_new_er_ucc
                                ),
                                PgTypeInitTryNew::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => &gen_ts(
                                    &sqlx_types_chrono_naive_date_time_as_nn_timestamp_orgn_try_new_er_ucc
                                ),
                                PgTypeInitTryNew::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => &gen_ts(
                                    &sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_nn_timestamptz_orgn_try_new_er_ucc
                                ),
                            };
                            quote::quote! {{#ts}}
                        }
                    );
                quote::quote! {
                    #allow_clippy_arbitrary_src_item_ordering
                    #serde_er_enum_ts
                }
            } else {
                proc_macro2::TokenStream::new()
            };
            let mb_pub_enum_ident_stdrt_nn_orgn_try_new_for_de_er_ts = if matches!(&is_stdrt_nn, pg_crud_macros_cmn::IsStdrtNn::True) {
                //todo this is bad design. refactor later
                let gen_er_ts = |pg_type_impl_try_new_for_deserialize: &PgTypeImplTryNewForDe|{
                    let serde_er_enum_ts = pg_crud_macros_cmn::ts_helpers::serde_er_enum_d_ts_builder()
                    .build_enum(
                        &proc_macro2::TokenStream::new(),
                        &ident_stdrt_nn_orgn_try_new_for_de_er_ucc,
                        &proc_macro2::TokenStream::new(),
                        &{
                            let ts: &dyn quote::ToTokens = match &pg_type_impl_try_new_for_deserialize {
                                PgTypeImplTryNewForDe::StringAsText => &string_as_text_try_new_er_vrts_ts,
                                PgTypeImplTryNewForDe::SqlxTypesChronoNaiveTimeAsTime => &{
                                    let invalid_hour_or_minute_or_second_or_microsecond_var_ts = gen_loc_var_ts(
                                        &invalid_hour_or_minute_or_second_or_microsecond_ucc,
                                        &quote::quote! {
                                            #[eo_to_err_string_serde]
                                            #hour_sc: #u32_ts,
                                            #[eo_to_err_string_serde]
                                            #min_sc: #u32_ts,
                                            #[eo_to_err_string_serde]
                                            #sec_sc: #u32_ts,
                                            #[eo_to_err_string_serde]
                                            #micro_sc: #u32_ts,
                                        }
                                    );
                                    quote::quote! {
                                        #invalid_hour_or_minute_or_second_or_microsecond_var_ts,
                                        #nanosecond_precision_is_not_supported_vrt_try_new_ts
                                    }
                                },
                                PgTypeImplTryNewForDe::SqlxTypesTimeTimeAsTime => &{
                                    let invalid_hour_or_minute_or_second_or_microsecond_var_ts = gen_loc_var_ts(
                                        &invalid_hour_or_minute_or_second_or_microsecond_ucc,
                                        &quote::quote! {
                                            #[eo_to_err_string_serde]
                                            #er_sc: #string_ts,
                                            #[eo_to_err_string_serde]
                                            #microsecond_sc: #u32_ts,
                                            #[eo_to_err_string_serde]
                                            #hour_sc: #u8_ts,
                                            #[eo_to_err_string_serde]
                                            #minute_sc: #u8_ts,
                                            #[eo_to_err_string_serde]
                                            #second_sc: #u8_ts,
                                        }
                                    );
                                    quote::quote! {
                                        #invalid_hour_or_minute_or_second_or_microsecond_var_ts,
                                        #nanosecond_precision_is_not_supported_vrt_try_new_ts
                                    }
                                },
                                PgTypeImplTryNewForDe::SqlxTypesChronoNaiveDateAsDate => &sqlx_types_chrono_naive_date_as_date_try_new_er_vrts_ts,
                                PgTypeImplTryNewForDe::SqlxPgTypesPgRangeI32AsInt4Range => &gen_int_range_type_er_vrts_ts(&IntRangeType::SqlxPgTypesPgRangeI32AsInt4Range),
                                PgTypeImplTryNewForDe::SqlxPgTypesPgRangeI64AsInt8Range => &gen_int_range_type_er_vrts_ts(&IntRangeType::SqlxPgTypesPgRangeI64AsInt8Range),
                                PgTypeImplTryNewForDe::SqlxTypesUuidUuidAsUuidV4InitByPg |
                                PgTypeImplTryNewForDe::SqlxTypesUuidUuidAsUuidInitByClient => &uuid_as_uuid_v4_as_string_try_new_er_vrts_ts,
                            };
                            quote::quote! {{#ts}}
                        }
                    );
                    quote::quote! {
                        #allow_clippy_arbitrary_src_item_ordering
                        #serde_er_enum_ts
                    }
                };
                match &de_derive_or_impl {
                    pg_crud_macros_cmn::DeriveOrImpl::Derive => if matches!(&is_stdrt_nn, pg_crud_macros_cmn::IsStdrtNn::True) {
                        match &pg_type {
                            PgType::I16AsInt2 |
                            PgType::I32AsInt4 |
                            PgType::I64AsInt8 |
                            PgType::F32AsFloat4 |
                            PgType::F64AsFloat8 |
                            PgType::I16AsSmallSerialInitByPg |
                            PgType::I32AsSerialInitByPg |
                            PgType::I64AsBigSerialInitByPg |
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
                            PgType::StringAsText => gen_er_ts(&PgTypeImplTryNewForDe::StringAsText),
                            PgType::SqlxTypesChronoNaiveTimeAsTime => gen_er_ts(&PgTypeImplTryNewForDe::SqlxTypesChronoNaiveTimeAsTime),
                            PgType::SqlxTypesTimeTimeAsTime => gen_er_ts(&PgTypeImplTryNewForDe::SqlxTypesTimeTimeAsTime),
                            PgType::SqlxTypesChronoNaiveDateAsDate => gen_er_ts(&PgTypeImplTryNewForDe::SqlxTypesChronoNaiveDateAsDate),
                            PgType::SqlxTypesUuidUuidAsUuidV4InitByPg => gen_er_ts(&PgTypeImplTryNewForDe::SqlxTypesUuidUuidAsUuidV4InitByPg),
                            PgType::SqlxTypesUuidUuidAsUuidInitByClient => gen_er_ts(&PgTypeImplTryNewForDe::SqlxTypesUuidUuidAsUuidInitByClient),
                            PgType::SqlxPgTypesPgRangeI32AsInt4Range => gen_er_ts(&PgTypeImplTryNewForDe::SqlxPgTypesPgRangeI32AsInt4Range),
                            PgType::SqlxPgTypesPgRangeI64AsInt8Range => gen_er_ts(&PgTypeImplTryNewForDe::SqlxPgTypesPgRangeI64AsInt8Range),
                        }
                    }
                    else {
                        proc_macro2::TokenStream::new()
                    },
                    pg_crud_macros_cmn::DeriveOrImpl::Impl(_) => match &pg_type_deserialize {
                        PgTypeDeserialize::Derive => proc_macro2::TokenStream::new(),
                        PgTypeDeserialize::ImplNewForDeserializeOrTryNewForDe(pg_type_impl_new_for_de_or_try_new_for_deserialize) => match &pg_type_impl_new_for_de_or_try_new_for_deserialize {
                            PgTypeImplNewForDeserializeOrTryNewForDe::NewForDeserialize => proc_macro2::TokenStream::new(),
                            PgTypeImplNewForDeserializeOrTryNewForDe::TryNewForDe(pg_type_impl_try_new_for_deserialize) => gen_er_ts(pg_type_impl_try_new_for_deserialize)
                        },
                    }
                }
            } else {
                proc_macro2::TokenStream::new()
            };
            let impl_ident_orgn_ts = {
                let fn_new_or_try_new_ts = pg_type_init_try_new_try_from_pg_type.as_ref().map_or_else(
                |()| {
                    let ts = {
                        let ts = {
                            let gen_match_opt_ts = |ts: &dyn quote::ToTokens| {
                                quote::quote! {#v_sc.map(#ts::#new_sc)}
                            };
                            match &pg_type_pattern {
                                PgTypePattern::Stdrt => match &is_nl {
                                    pg_crud_macros_cmn::IsNl::False => {
                                        range_try_from_pg_type.as_ref().map_or_else(
                                            |()| quote::quote! {#v_sc},
                                            |range_try_from| gen_pg_range_conversion_ts(
                                                &v_sc,
                                                &{
                                                    let range_pg_type_ident_orgn = naming::prm::SelfOrgnUcc::from_display(&gen_ident_str(&PgType::from(range_try_from), is_nl, pg_type_pattern));
                                                    quote::quote! {#range_pg_type_ident_orgn::#new_sc(v_af65ccce)}
                                                }
                                            )
                                        )
                                    }
                                    pg_crud_macros_cmn::IsNl::True => gen_match_opt_ts(&ident_stdrt_nn_orgn_ucc),
                                },
                            }
                        };
                        quote::quote! {Self(#ts)}
                    };
                    match &pg_type_pattern {
                        PgTypePattern::Stdrt => match &is_nl {
                            pg_crud_macros_cmn::IsNl::False => macros_helpers::gen_new_or_try_new::gen_const_new_ts(
                                &must_use,
                                &v_ident_inn_type_ts,
                                &ts
                            ),
                            pg_crud_macros_cmn::IsNl::True => macros_helpers::gen_new_or_try_new::gen_new_ts(
                                &must_use,
                                &v_ident_inn_type_ts,
                                &ts
                            ),
                        },
                    }
                },
                |pg_type_init_try_new| {
                    let ts = {
                        let gen_match_opt_ts = |ts: &dyn quote::ToTokens| {
                            quote::quote! {Ok(Self(match #v_sc {
                                Some(v_989d943e) => Some(match #ts::#try_new_sc(v_989d943e) {
                                    Ok(v_ea2a4a8c) => v_ea2a4a8c,
                                    Err(er) => {
                                        return Err(er);
                                    },
                                }),
                                None => None
                            }))}
                        };
                        match &pg_type_pattern {
                            PgTypePattern::Stdrt => match &is_nl {
                                pg_crud_macros_cmn::IsNl::False => {
                                    let gen_int_range_check_ts = |int_range_type: &IntRangeType| {
                                        let max_v_ts = {
                                            let type_ts = int_range_type_to_range_inn_type_ts(int_range_type);
                                            quote::quote! {#type_ts::MAX}
                                        };
                                        quote::quote! {
                                            let max = #max_v_ts;
                                            let (#start_sc, #end_sc) = match (#v_sc.#start_sc, #v_sc.#end_sc) {
                                                (std::ops::Bound::Included(#start_sc), std::ops::Bound::Included(#end_sc)) => {
                                                    if #start_sc > #end_sc {
                                                        return Err(#ident_stdrt_nn_orgn_try_new_er_ucc::#included_start_greater_than_included_end_ucc {
                                                            #start_sc,
                                                            #end_sc,
                                                            loc: loc_macros::loc!(),
                                                        });
                                                    }
                                                    if #end_sc == max {
                                                        return Err(#ident_stdrt_nn_orgn_try_new_er_ucc::#included_end_cannot_be_max_ucc {
                                                            #end_sc,
                                                            loc: loc_macros::loc!(),
                                                        });
                                                    }
                                                    (std::ops::Bound::Included(#start_sc), std::ops::Bound::Included(#end_sc))
                                                }
                                                (std::ops::Bound::Included(#start_sc), std::ops::Bound::Excluded(#end_sc)) => {
                                                    if #start_sc > #end_sc {
                                                        return Err(#ident_stdrt_nn_orgn_try_new_er_ucc::#included_start_greater_than_excluded_end_ucc {
                                                            #start_sc,
                                                            #end_sc,
                                                            loc: loc_macros::loc!(),
                                                        });
                                                    }
                                                    (std::ops::Bound::Included(#start_sc), std::ops::Bound::Excluded(#end_sc))
                                                }
                                                (std::ops::Bound::Included(#start_sc), std::ops::Bound::Unbounded) => (std::ops::Bound::Included(#start_sc), std::ops::Bound::Unbounded),
                                                (std::ops::Bound::Excluded(#start_sc), std::ops::Bound::Included(#end_sc)) => {
                                                    if #start_sc > #end_sc {
                                                        return Err(#ident_stdrt_nn_orgn_try_new_er_ucc::#excluded_start_greater_than_included_end_ucc {
                                                            #start_sc,
                                                            #end_sc,
                                                            loc: loc_macros::loc!(),
                                                        });
                                                    }
                                                    if #end_sc == max {
                                                        return Err(#ident_stdrt_nn_orgn_try_new_er_ucc::#included_end_cannot_be_max_ucc {
                                                            #end_sc,
                                                            loc: loc_macros::loc!(),
                                                        });
                                                    }
                                                    (std::ops::Bound::Excluded(#start_sc), std::ops::Bound::Included(#end_sc))
                                                }
                                                (std::ops::Bound::Excluded(#start_sc), std::ops::Bound::Excluded(#end_sc)) => {
                                                    if #start_sc > #end_sc {
                                                        return Err(#ident_stdrt_nn_orgn_try_new_er_ucc::#excluded_start_greater_than_excluded_end_ucc {
                                                            #start_sc,
                                                            #end_sc,
                                                            loc: loc_macros::loc!(),
                                                        });
                                                    }
                                                    (std::ops::Bound::Excluded(#start_sc), std::ops::Bound::Excluded(#end_sc))
                                                }
                                                (std::ops::Bound::Excluded(#start_sc), std::ops::Bound::Unbounded) => (std::ops::Bound::Excluded(#start_sc), std::ops::Bound::Unbounded),
                                                (std::ops::Bound::Unbounded, std::ops::Bound::Included(#end_sc)) => {
                                                    if #end_sc == max {
                                                        return Err(#ident_stdrt_nn_orgn_try_new_er_ucc::#included_end_cannot_be_max_ucc {
                                                            #end_sc,
                                                            loc: loc_macros::loc!(),
                                                        });
                                                    }
                                                    (std::ops::Bound::Unbounded, std::ops::Bound::Included(#end_sc))
                                                }
                                                (std::ops::Bound::Unbounded, std::ops::Bound::Excluded(#end_sc)) => (std::ops::Bound::Unbounded, std::ops::Bound::Excluded(#end_sc)),
                                                (std::ops::Bound::Unbounded, std::ops::Bound::Unbounded) => (std::ops::Bound::Unbounded, std::ops::Bound::Unbounded),
                                            };
                                            Ok(Self(sqlx::postgres::types::PgRange { #start_sc, #end_sc }))
                                        }
                                    };
                                    let gen_ok_self_sqlx_pg_types_pg_range_ts = |ts: &dyn quote::ToTokens| {
                                        let gen_bound_arms_ts = |variant_ts: &dyn quote::ToTokens| quote::quote! {
                                            std::ops::Bound::Included(v_bound_incl) => match #ts::#try_new_sc(v_bound_incl) {
                                                Ok(v_bound_ok) => std::ops::Bound::Included(v_bound_ok.0),
                                                Err(er) => {
                                                    return Err(#ident_stdrt_nn_orgn_try_new_er_ucc::#variant_ts {
                                                        #er_sc,
                                                        loc: loc_macros::loc!(),
                                                    });
                                                }
                                            },
                                            std::ops::Bound::Excluded(v_bound_excl) => match #ts::#try_new_sc(v_bound_excl) {
                                                Ok(v_bound_ok) => std::ops::Bound::Excluded(v_bound_ok.0),
                                                Err(er) => {
                                                    return Err(#ident_stdrt_nn_orgn_try_new_er_ucc::#variant_ts {
                                                        #er_sc,
                                                        loc: loc_macros::loc!(),
                                                    });
                                                }
                                            },
                                            std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
                                        };
                                        let start_arms_ts = gen_bound_arms_ts(&quote::quote! {#start_ucc});
                                        let end_arms_ts = gen_bound_arms_ts(&quote::quote! {#end_ucc});
                                        quote::quote! {
                                            Ok(Self(sqlx::postgres::types::PgRange {
                                                #start_sc: match #v_sc.#start_sc { #start_arms_ts },
                                                #end_sc: match #v_sc.#end_sc { #end_arms_ts },
                                            }))
                                        }
                                    };
                                    match &pg_type_init_try_new {
                                        PgTypeInitTryNew::StringAsText => quote::quote! {
                                            if #v_sc.find('\0').is_some() {
                                                Err(#ident_stdrt_nn_orgn_try_new_er_ucc::#contains_null_byte_ucc {
                                                    #v_sc,
                                                    loc: loc_macros::loc!(),
                                                })
                                            } else {
                                                Ok(Self(#v_sc))
                                            }
                                        },
                                        PgTypeInitTryNew::SqlxTypesChronoNaiveTimeAsTime => quote::quote! {
                                            if <#inn_type_stdrt_nn_ts as chrono::Timelike>::nanosecond(&#v_sc).checked_rem(1000).expect("7c8b4e12") != 0 {
                                                return Err(#ident_stdrt_nn_orgn_try_new_er_ucc::#nanosecond_precision_is_not_supported_ucc {
                                                    #v_sc: #v_sc.to_string(),
                                                    loc: loc_macros::loc!(),
                                                });
                                            }
                                            Ok(Self(#v_sc))
                                        },
                                        PgTypeInitTryNew::SqlxTypesTimeTimeAsTime => quote::quote! {
                                            if #v_sc.nanosecond().checked_rem(1000).expect("ce47524f") != 0 {
                                                return Err(#ident_stdrt_nn_orgn_try_new_er_ucc::#nanosecond_precision_is_not_supported_ucc {
                                                    #v_sc: #v_sc.to_string(),
                                                    loc: loc_macros::loc!(),
                                                });
                                            }
                                            Ok(Self(#v_sc))
                                        },
                                        PgTypeInitTryNew::SqlxTypesChronoNaiveDateAsDate => quote::quote! {
                                            let #earliest_supported_date_sc = #inn_type_stdrt_nn_ts::from_ymd_opt(-4713, 12, 31).expect("9f6241e5");
                                            if #v_sc >= #earliest_supported_date_sc {
                                                Ok(Self(#v_sc))
                                            }
                                            else {
                                                Err(#ident_stdrt_nn_orgn_try_new_er_ucc::#earlier_date_not_supported_ucc {
                                                    value: #v_sc.to_string(),
                                                    #earliest_supported_date_sc: #earliest_supported_date_sc.to_string(),
                                                    loc: loc_macros::loc!(),
                                                })
                                            }
                                        },
                                        PgTypeInitTryNew::SqlxTypesChronoNaiveDateTimeAsTimestamp => quote::quote! {
                                            let #date_sc = match #sqlx_types_chrono_naive_date_as_nn_date_orgn_ucc::#try_new_sc(
                                                #v_sc.#date_sc()
                                            ) {
                                                Ok(v_9be8eddb) => v_9be8eddb,
                                                Err(er) => {
                                                    return Err(#ident_stdrt_nn_orgn_try_new_er_ucc::#date_ucc {
                                                        #er_sc,
                                                        loc: loc_macros::loc!(),
                                                    });
                                                }
                                            };
                                            let #time_sc = match #sqlx_types_chrono_naive_time_as_nn_time_orgn_ucc::#try_new_sc(
                                                #v_sc.#time_sc()
                                            ) {
                                                Ok(v_993484ce) => v_993484ce,
                                                Err(er) => {
                                                    return Err(#ident_stdrt_nn_orgn_try_new_er_ucc::#time_ucc {
                                                        #er_sc,
                                                        loc: loc_macros::loc!(),
                                                    });
                                                }
                                            };
                                            Ok(Self(#inn_type_stdrt_nn_ts::#new_sc(#date_sc.0, #time_sc.0)))
                                        },
                                        PgTypeInitTryNew::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => {
                                            let sqlx_types_chrono_date_time_sqlx_types_chrono_utc_from_naive_utc_and_offset_ts = gen_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_from_naive_utc_and_offset_ts(&gen_sqlx_types_chrono_naive_date_time_new_ts(&quote::quote! {
                                                #date_naive_sc.0,
                                                #time_sc.0
                                            }));
                                            quote::quote! {
                                                let #date_naive_sc = match #sqlx_types_chrono_naive_date_as_nn_date_orgn_ucc::#try_new_sc(#v_sc.date_naive()) {
                                                    Ok(v_158945ad) => v_158945ad,
                                                    Err(er) => {
                                                        return Err(#ident_stdrt_nn_orgn_try_new_er_ucc::#date_naive_ucc {
                                                            #er_sc,
                                                            loc: loc_macros::loc!(),
                                                        });
                                                    }
                                                };
                                                let #time_sc = match #sqlx_types_chrono_naive_time_as_nn_time_orgn_ucc::#try_new_sc(#v_sc.time()) {
                                                    Ok(v_c5af739c) => v_c5af739c,
                                                    Err(er) => {
                                                        return Err(#ident_stdrt_nn_orgn_try_new_er_ucc::#time_ucc {
                                                            #er_sc,
                                                            loc: loc_macros::loc!(),
                                                        });
                                                    }
                                                };
                                                Ok(Self(#sqlx_types_chrono_date_time_sqlx_types_chrono_utc_from_naive_utc_and_offset_ts))
                                            }
                                        }
                                        PgTypeInitTryNew::SqlxPgTypesPgRangeI32AsInt4Range => gen_int_range_check_ts(&IntRangeType::SqlxPgTypesPgRangeI32AsInt4Range),
                                        PgTypeInitTryNew::SqlxPgTypesPgRangeI64AsInt8Range => gen_int_range_check_ts(&IntRangeType::SqlxPgTypesPgRangeI64AsInt8Range),
                                        PgTypeInitTryNew::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => gen_ok_self_sqlx_pg_types_pg_range_ts(&sqlx_types_chrono_naive_date_as_nn_date_orgn_ucc),
                                        PgTypeInitTryNew::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => gen_ok_self_sqlx_pg_types_pg_range_ts(&sqlx_types_chrono_naive_date_time_as_nn_timestamp_orgn_ucc),
                                        PgTypeInitTryNew::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => gen_ok_self_sqlx_pg_types_pg_range_ts(&sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_nn_timestamptz_orgn_ucc),
                                    }
                                }
                                pg_crud_macros_cmn::IsNl::True => gen_match_opt_ts(&ident_stdrt_nn_orgn_ucc),
                            },
                        }
                    };
                    quote::quote! {
                        pub fn #try_new_sc(#v_ident_inn_type_ts) -> Result<Self, #ident_stdrt_nn_orgn_try_new_er_ucc> {
                            #ts
                        }
                    }
                    .into()
                });
                let mb_fn_new_or_try_new_for_de_token = {
                    let gen_v_pg_range_int_type_ts = |int_range_type: &IntRangeType| {
                        let type_ts = {
                            let ts = int_range_type_to_range_inn_type_ts(int_range_type);
                            quote::quote! {std::ops::Bound<#ts>}
                        };
                        quote::quote! {
                            start_9a8ef454: #type_ts,
                            end_a14eb2b9: #type_ts
                        }
                    };
                    match &pg_type_pattern {
                        PgTypePattern::Stdrt => match &is_nl {
                            pg_crud_macros_cmn::IsNl::False => match &pg_type_deserialize {
                                PgTypeDeserialize::Derive => if matches!(&is_stdrt_nn, pg_crud_macros_cmn::IsStdrtNn::True) {
                                    match &pg_type {
                                        PgType::I16AsInt2 |
                                        PgType::I32AsInt4 |
                                        PgType::I64AsInt8 |
                                        PgType::F32AsFloat4 |
                                        PgType::F64AsFloat8 |
                                        PgType::I16AsSmallSerialInitByPg |
                                        PgType::I32AsSerialInitByPg |
                                        PgType::I64AsBigSerialInitByPg |
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
                                        PgType::SqlxTypesUuidUuidAsUuidInitByClient |
                                        PgType::SqlxTypesUuidUuidAsUuidV4InitByPg => proc_macro2::TokenStream::new(),
                                        PgType::SqlxPgTypesPgRangeI32AsInt4Range => gen_v_pg_range_int_type_ts(&IntRangeType::SqlxPgTypesPgRangeI32AsInt4Range),
                                        PgType::SqlxPgTypesPgRangeI64AsInt8Range => gen_v_pg_range_int_type_ts(&IntRangeType::SqlxPgTypesPgRangeI64AsInt8Range),
                                    }
                                }
                                else {
                                    proc_macro2::TokenStream::new()
                                },
                                PgTypeDeserialize::ImplNewForDeserializeOrTryNewForDe(_) => proc_macro2::TokenStream::new()
                            },
                            pg_crud_macros_cmn::IsNl::True => proc_macro2::TokenStream::new(),
                        },
                    }
                };
                quote::quote! {
                    #allow_clippy_arbitrary_src_item_ordering
                    impl #ident_orgn_ucc {
                        #fn_new_or_try_new_ts
                        #mb_fn_new_or_try_new_for_de_token
                    }
                }
            };
            let impl_from_ident_orgn_for_ident_inn_type_ts = macros_helpers::gen_impl_from_ts::gen_impl_from_ts(
                &ident_orgn_ucc,
                &ident_inn_type_ts,
                &{
                    let v_dot_zero = quote::quote! {#v_sc.0};
                    let gen_match_ts = |
                        match_ts: &dyn quote::ToTokens,
                        some_ts: &dyn quote::ToTokens,
                        some_v_ts: &dyn quote::ToTokens,
                    | quote::quote! {
                        #match_ts.map(|#some_v_ts|#some_v_ts.0#some_ts)
                    };
                    match &pg_type_pattern {
                        PgTypePattern::Stdrt => match &is_nl {
                            pg_crud_macros_cmn::IsNl::False => v_dot_zero,
                            pg_crud_macros_cmn::IsNl::True => gen_match_ts(
                                &v_dot_zero,
                                &proc_macro2::TokenStream::new(),
                                &quote::quote! {v_6bfd70fa}
                            ),
                        },
                    }
                }
            );
            let mb_impl_is_string_empty_for_ident_orgn_ts = if matches!(&is_stdrt_nn, pg_crud_macros_cmn::IsStdrtNn::True) {
                match &is_nl {
                    pg_crud_macros_cmn::IsNl::False => match &pg_type {
                        PgType::I16AsInt2
                        | PgType::I32AsInt4
                        | PgType::I64AsInt8
                        | PgType::F32AsFloat4
                        | PgType::F64AsFloat8
                        | PgType::I16AsSmallSerialInitByPg
                        | PgType::I32AsSerialInitByPg
                        | PgType::I64AsBigSerialInitByPg
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
                        | PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => macros_helpers::generated_rust_ts::GeneratedRustTs::from(proc_macro2::TokenStream::new()),
                        PgType::StringAsText => pg_crud_macros_cmn::gen_impl_crate_is_string_empty_for_ident_ts(
                            &ident_orgn_ucc,
                            &quote::quote! {self.0.clone().is_empty()},
                        ),
                        PgType::SqlxTypesUuidUuidAsUuidV4InitByPg |
                        PgType::SqlxTypesUuidUuidAsUuidInitByClient |
                        PgType::SqlxTypesMacAddressMacAddressAsMacAddr => pg_crud_macros_cmn::gen_impl_crate_is_string_empty_for_ident_ts(
                            &ident_orgn_ucc,
                            &quote::quote! {self.0.to_string().is_empty()},
                        ),
                    },
                    pg_crud_macros_cmn::IsNl::True => macros_helpers::generated_rust_ts::GeneratedRustTs::from(proc_macro2::TokenStream::new()),
                }
            } else {
                macros_helpers::generated_rust_ts::GeneratedRustTs::from(proc_macro2::TokenStream::new())
            };
            let empty_generated_ts = macros_helpers::generated_rust_ts::GeneratedRustTs::from(proc_macro2::TokenStream::new());
            let mb_impl_ser_for_ident_stdrt_nn_orgn_ts = match &ser_derive_or_impl {
                pg_crud_macros_cmn::DeriveOrImpl::Derive => &empty_generated_ts,
                pg_crud_macros_cmn::DeriveOrImpl::Impl(v) => v,
            };
            let mb_impl_de_for_ident_stdrt_nn_orgn_ts = match &de_derive_or_impl {
                pg_crud_macros_cmn::DeriveOrImpl::Derive => &empty_generated_ts,
                pg_crud_macros_cmn::DeriveOrImpl::Impl(v) => v,
            };
            let md_de_from_for_ident_stndrt_nn_orgn_ts = if matches!(&is_stdrt_nn, pg_crud_macros_cmn::IsStdrtNn::True) {
                let self_sqlx_pg_types_pg_range_ts = {
                    let (start_ts, end_ts) = {
                        let gen_ts = |start_or_end: StartOrEnd|{
                            let name_ts = match start_or_end {
                                StartOrEnd::End => quote::quote! {end},
                                StartOrEnd::Start => quote::quote! {start},
                            };
                            let ts0 = match start_or_end {
                                StartOrEnd::End => quote::quote! {v.1},
                                StartOrEnd::Start => quote::quote! {v.0},
                            };
                            quote::quote! {
                                #name_ts: match #ts0 {
                                    std::ops::Bound::Included(v0) => std::ops::Bound::Included(v0.0),
                                    std::ops::Bound::Excluded(v0) => std::ops::Bound::Excluded(v0.0),
                                    std::ops::Bound::Unbounded => std::ops::Bound::Unbounded,
                                },
                            }
                        };
                        (gen_ts(StartOrEnd::Start), gen_ts(StartOrEnd::End))
                    };
                    quote::quote! {Self(sqlx::postgres::types::PgRange {
                        #start_ts
                        #end_ts
                    })}
                };
                let gen_impl_from_orgn_ts = |
                    from_type_ts: &dyn quote::ToTokens,
                    ts: &dyn quote::ToTokens,
                |macros_helpers::gen_impl_from_ts::gen_impl_from_ts(
                    from_type_ts,
                    &ident_orgn_ucc,
                    ts,
                ).into();
                match &pg_type {
                    PgType::I16AsInt2 |
                    PgType::I32AsInt4 |
                    PgType::I64AsInt8 |
                    PgType::F32AsFloat4 |
                    PgType::F64AsFloat8 |
                    PgType::I16AsSmallSerialInitByPg |
                    PgType::I32AsSerialInitByPg |
                    PgType::I64AsBigSerialInitByPg |
                    PgType::BoolAsBool |
                    PgType::StringAsText |
                    PgType::StdVecVecU8AsBytea |
                    PgType::SqlxTypesChronoNaiveTimeAsTime |
                    PgType::SqlxTypesTimeTimeAsTime |
                    PgType::SqlxTypesChronoNaiveDateAsDate |
                    PgType::SqlxTypesUuidUuidAsUuidV4InitByPg |
                    PgType::SqlxTypesUuidUuidAsUuidInitByClient |
                    PgType::SqlxTypesIpnetworkIpNetworkAsInet |
                    PgType::SqlxPgTypesPgRangeI32AsInt4Range |
                    PgType::SqlxPgTypesPgRangeI64AsInt8Range => proc_macro2::TokenStream::new(),
                    PgType::SqlxPgTypesPgMoneyAsMoney => gen_impl_from_orgn_ts(
                        &quote::quote! {i64},
                        &quote::quote! {Self::new(#inn_type_stdrt_nn_ts(v))}
                    ),
                    PgType::SqlxPgTypesPgIntervalAsInterval => gen_impl_from_orgn_ts(
                        &quote::quote! {(i32,i32,i64)},
                        &quote::quote! {
                            Self(sqlx::postgres::types::PgInterval {
                                #months_sc: v.0,
                                #days_sc: v.1,
                                #microseconds_sc: v.2,
                            })
                        }
                    ),
                    //todo reuse naming
                    PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp => gen_impl_from_orgn_ts(
                        &quote::quote! {(#sqlx_types_chrono_naive_date_as_date_stdrt_nn_orig_ts,SqlxTypesChronoNaiveTimeAsNnTimeOrgn)},
                        &quote::quote! {Self(#inn_type_stdrt_nn_ts::#new_sc(v.0.0, v.1.0))}
                    ),
                    PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => gen_impl_from_orgn_ts(
                        &quote::quote! {(#sqlx_types_chrono_naive_date_as_date_stdrt_nn_orig_ts,SqlxTypesChronoNaiveTimeAsNnTimeOrgn)},
                        &{
                            let ts = gen_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_from_naive_utc_and_offset_ts(&gen_sqlx_types_chrono_naive_date_time_new_ts(&quote::quote! {
                                v.0.0,
                                v.1.0
                            }));
                            quote::quote! {Self(#ts)}
                        }
                    ),
                    PgType::SqlxTypesMacAddressMacAddressAsMacAddr => gen_impl_from_orgn_ts(
                        &quote::quote! {[u8; 6]},
                        &quote::quote! {Self(#inn_type_stdrt_nn_ts::new(v))}
                    ),
                    PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => gen_impl_from_orgn_ts(
                        &{
                            let bound_ts = quote::quote! {std::ops::Bound<#sqlx_types_chrono_naive_date_as_date_stdrt_nn_orig_ts>};
                            quote::quote! {(#bound_ts,#bound_ts)}
                        },
                        &self_sqlx_pg_types_pg_range_ts
                    ),
                    PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => gen_impl_from_orgn_ts(
                        &{
                            let bound_ts = quote::quote! {std::ops::Bound<SqlxTypesChronoNaiveDateTimeAsNnTimestampOrgn>};
                            quote::quote! {(#bound_ts,#bound_ts)}
                        },
                        &self_sqlx_pg_types_pg_range_ts
                    ),
                    PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => gen_impl_from_orgn_ts(
                        &{
                            let bound_ts = quote::quote! {std::ops::Bound<SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNnTimestampTzOrgn>};
                            quote::quote! {(#bound_ts,#bound_ts)}
                        },
                        &self_sqlx_pg_types_pg_range_ts
                    ),
                }
            }
            else {
                proc_macro2::TokenStream::new()
            };
            let md_de_try_from_for_ident_stndrt_nn_orgn_ts = if matches!(&is_stdrt_nn, pg_crud_macros_cmn::IsStdrtNn::True) {
                let gen_self_match_try_new_ts = |prms_ts: &dyn quote::ToTokens, match_er_vrts_ts: &dyn quote::ToTokens| {
                    quote::quote! {
                        match Self::#try_new_sc(#prms_ts) {
                            Ok(v_b318fc86) => Ok(v_b318fc86),
                            Err(er) => match er {
                                #match_er_vrts_ts
                            }
                        }
                    }
                };
                let gen_impl_try_from_orgn_ts = |
                    from_type_ts: &dyn quote::ToTokens,
                    er_type_ts: &dyn quote::ToTokens,
                    ts: &dyn quote::ToTokens
                |macros_helpers::gen_impl_try_from_ts::gen_impl_try_from_ts(
                    from_type_ts,
                    &ident_orgn_ucc,
                    er_type_ts,
                    ts
                ).into();
                let gen_impl_try_from_de_er_ts = |
                    from_type_ts: &dyn quote::ToTokens,
                    ts: &dyn quote::ToTokens
                |gen_impl_try_from_orgn_ts(
                    from_type_ts,
                    &ident_stdrt_nn_orgn_try_new_for_de_er_ucc,
                    ts
                );
                let gen_impl_try_from_int_range_ts = |
                    int_range_type: IntRangeType,
                |gen_impl_try_from_de_er_ts(
                    &{
                        let ts0 = match int_range_type {
                            IntRangeType::SqlxPgTypesPgRangeI32AsInt4Range => &quote::quote! {i32},
                            IntRangeType::SqlxPgTypesPgRangeI64AsInt8Range => &quote::quote! {i64},
                        };
                        quote::quote! {(std::ops::Bound<#ts0>,std::ops::Bound<#ts0>)}
                    },
                    &gen_self_match_try_new_ts(
                        &quote::quote! {sqlx::postgres::types::PgRange { #start_sc: v.0, #end_sc: v.1 }},
                        &{
                            let gen_match_ts = |name_ts: &dyn quote::ToTokens, ts: &dyn quote::ToTokens|quote::quote! {
                                #ident_stdrt_nn_orgn_try_new_er_ucc::#name_ts {
                                    loc,
                                    #ts
                                } => Err(#ident_stdrt_nn_orgn_try_new_for_de_er_ucc::#name_ts {
                                    loc,
                                    #ts
                                }),
                            };
                            let (
                                included_start_greater_than_included_end_ts,
                                included_start_greater_than_excluded_end_ts,
                                excluded_start_greater_than_included_end_ts,
                                excluded_start_greater_than_excluded_end_ts,
                            ) = {
                                let gen_ts = |ts: &dyn quote::ToTokens|gen_match_ts(
                                    &ts,
                                    &quote::quote! {
                                        #start_sc,
                                        #end_sc,
                                    }
                                );
                                (
                                    gen_ts(&included_start_greater_than_included_end_ucc),
                                    gen_ts(&included_start_greater_than_excluded_end_ucc),
                                    gen_ts(&excluded_start_greater_than_included_end_ucc),
                                    gen_ts(&excluded_start_greater_than_excluded_end_ucc),
                                )
                            };
                            let included_end_cannot_be_max_ts = gen_match_ts(
                                &included_end_cannot_be_max_ucc,
                                &quote::quote! {#end_sc,}
                            );
                            quote::quote! {
                                #included_start_greater_than_included_end_ts
                                #included_start_greater_than_excluded_end_ts
                                #excluded_start_greater_than_included_end_ts
                                #excluded_start_greater_than_excluded_end_ts
                                #included_end_cannot_be_max_ts
                            }
                        },
                    )
                );
                match &pg_type {
                    PgType::I16AsInt2 |
                    PgType::I32AsInt4 |
                    PgType::I64AsInt8 |
                    PgType::F32AsFloat4 |
                    PgType::F64AsFloat8 |
                    PgType::I16AsSmallSerialInitByPg |
                    PgType::I32AsSerialInitByPg |
                    PgType::I64AsBigSerialInitByPg |
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
                    PgType::StringAsText => gen_impl_try_from_orgn_ts(
                        &inn_type_stdrt_nn_ts,
                        &ident_stdrt_nn_orgn_try_new_er_ucc,
                        &quote::quote! {Self::try_new(v)}//todo use try_from instead of try_new ?
                    ),
                    PgType::SqlxTypesChronoNaiveTimeAsTime => gen_impl_try_from_de_er_ts(
                        &quote::quote! {(u32,u32,u32,u32)},
                        &quote::quote! {
                            match #inn_type_stdrt_nn_ts::from_hms_micro_opt(
                                v.0,
                                v.1,
                                v.2,
                                v.3,
                            ) {
                                Some(v_b143b9e1) => {
                                    if <#inn_type_stdrt_nn_ts as chrono::Timelike>::nanosecond(&v_b143b9e1).checked_rem(1000).expect("c0514180") != 0 {
                                        return Err(#ident_stdrt_nn_orgn_try_new_for_de_er_ucc::#nanosecond_precision_is_not_supported_ucc {
                                            #v_sc: v_b143b9e1.to_string(),
                                            loc: loc_macros::loc!(),
                                        });
                                    }
                                    Ok(Self(v_b143b9e1))
                                },
                                None => Err(#ident_stdrt_nn_orgn_try_new_for_de_er_ucc::#invalid_hour_or_minute_or_second_or_microsecond_ucc {
                                    #hour_sc: v.0,
                                    #min_sc: v.1,
                                    #sec_sc: v.2,
                                    #micro_sc: v.3,
                                    loc: loc_macros::loc!(),
                                })
                            }
                        }
                    ),
                    PgType::SqlxTypesTimeTimeAsTime => gen_impl_try_from_de_er_ts(
                        &quote::quote! {(u8,u8,u8,u32)},
                        &quote::quote! {
                            match #inn_type_stdrt_nn_ts::from_hms_micro(
                                v.0,
                                v.1,
                                v.2,
                                v.3,
                            ) {
                                Ok(v_9932d535) => {
                                    if v_9932d535.nanosecond().checked_rem(1000).expect("0def33ce") != 0 {
                                        return Err(#ident_stdrt_nn_orgn_try_new_for_de_er_ucc::#nanosecond_precision_is_not_supported_ucc {
                                            #v_sc: v_9932d535.to_string(),
                                            loc: loc_macros::loc!(),
                                        });
                                    }
                                    Ok(Self(v_9932d535))
                                },
                                Err(er) => Err(#ident_stdrt_nn_orgn_try_new_for_de_er_ucc::#invalid_hour_or_minute_or_second_or_microsecond_ucc {
                                    #hour_sc: v.0,
                                    #minute_sc: v.1,
                                    #second_sc: v.2,
                                    #microsecond_sc: v.3,
                                    #er_sc: er.to_string(),
                                    loc: loc_macros::loc!(),
                                })
                            }
                        }
                    ),
                    PgType::SqlxTypesChronoNaiveDateAsDate => gen_impl_try_from_de_er_ts(
                        &quote::quote! {sqlx::types::chrono::NaiveDate},
                        &gen_self_match_try_new_ts(
                            &v_sc,
                            &quote::quote! {
                                #ident_stdrt_nn_orgn_try_new_er_ucc::#earlier_date_not_supported_ucc {
                                    value,
                                    #earliest_supported_date_sc,
                                    loc,
                                } => Err(#ident_stdrt_nn_orgn_try_new_for_de_er_ucc::#earlier_date_not_supported_ucc {
                                    value,
                                    #earliest_supported_date_sc,
                                    loc,
                                }),
                            }
                        )
                    ),
                    PgType::SqlxTypesUuidUuidAsUuidInitByClient | PgType::SqlxTypesUuidUuidAsUuidV4InitByPg => gen_impl_try_from_de_er_ts(
                        &quote::quote! {String},
                        &quote::quote! {
                            match uuid::Uuid::try_parse(&v) {
                                Ok(v0) => Ok(Self(v0)),
                                Err(er) => Err(#ident_stdrt_nn_orgn_try_new_for_de_er_ucc::#not_uuid_ucc {
                                    #v_sc: er.to_string(),
                                    loc: loc_macros::loc!(),
                                })
                            }
                        }
                    ),
                    PgType::SqlxPgTypesPgRangeI32AsInt4Range => gen_impl_try_from_int_range_ts(
                        IntRangeType::SqlxPgTypesPgRangeI32AsInt4Range
                    ),
                    PgType::SqlxPgTypesPgRangeI64AsInt8Range => gen_impl_try_from_int_range_ts(
                        IntRangeType::SqlxPgTypesPgRangeI64AsInt8Range
                    ),
                }
            }
            else {
                proc_macro2::TokenStream::new()
            };
            let impl_display_for_ident_orgn_ts = macros_helpers::gen_impl_display_ts::gen_impl_display_ts(&proc_macro2::TokenStream::new(), &ident_orgn_ucc, &proc_macro2::TokenStream::new(), &quote::quote! {write!(f, "{self:?}")});
            let impl_loc_lib_to_err_string_for_ident_orgn_ts = pg_crud_macros_cmn::gen_impl_to_err_string_no_generics_ts(&ident_orgn_ucc, &quote::quote! {self.to_string()});
            let some_dflt_some_one_el_call_ts = quote::quote! {Some(#pg_crud_cmn_dflt_some_one_el_call)};
            let impl_dflt_some_one_el_for_ident_orgn_ts = pg_crud_macros_cmn::gen_impl_pg_crud_cmn_dflt_some_one_el_ts(&ident_orgn_ucc, &{
                let ts = match &pg_type_pattern {
                    PgTypePattern::Stdrt => match &is_nl {
                        pg_crud_macros_cmn::IsNl::False => {
                            let pg_range_int_dflt_init_ts = quote::quote! {
                                sqlx::postgres::types::PgRange {
                                    start: std::ops::Bound::Included(#core_default),
                                    end: std::ops::Bound::Excluded(#core_default),
                                }
                            };
                            let gen_as_dflt_some_one_el_call_ts = |ts: &dyn quote::ToTokens| {
                                quote::quote! {<#ts as #import::DfltSomeOneEl>::dflt_some_one_el()}
                            };
                            let gen_sqlx_pg_types_pg_range_dflt_some_one_el_ts = |ts: &dyn quote::ToTokens| {
                                let ts0 = gen_as_dflt_some_one_el_call_ts(&ts);
                                quote::quote! {sqlx::postgres::types::PgRange {
                                    #start_sc: std::ops::Bound::Included(#ts0.0),
                                    #end_sc: std::ops::Bound::Excluded(#ts0.0),
                                }}
                            };
                            let sqlx_types_chrono_naive_date_as_nn_date_orgn_as_dflt_some_one_el_call_ts = gen_as_dflt_some_one_el_call_ts(&sqlx_types_chrono_naive_date_as_nn_date_orgn_ucc);
                            let sqlx_types_chrono_naive_time_as_nn_time_orgn_as_dflt_some_one_el_call_ts = gen_as_dflt_some_one_el_call_ts(&sqlx_types_chrono_naive_time_as_nn_time_orgn_ucc);
                            let init_ts: &dyn quote::ToTokens = match &pg_type {
                                PgType::I16AsInt2
                                | PgType::I32AsInt4
                                | PgType::I64AsInt8
                                | PgType::F32AsFloat4
                                | PgType::F64AsFloat8
                                | PgType::I16AsSmallSerialInitByPg
                                | PgType::I32AsSerialInitByPg
                                | PgType::I64AsBigSerialInitByPg
                                | PgType::BoolAsBool
                                | PgType::StringAsText
                                | PgType::SqlxTypesChronoNaiveDateAsDate
                                | PgType::SqlxTypesChronoNaiveTimeAsTime
                                | PgType::SqlxTypesMacAddressMacAddressAsMacAddr
                                | PgType::SqlxTypesUuidUuidAsUuidV4InitByPg => &quote::quote! {#ft_h::default()},
                                PgType::SqlxTypesUuidUuidAsUuidInitByClient => &quote::quote! {#ident_inn_type_ts::default()},
                                PgType::SqlxPgTypesPgMoneyAsMoney => &quote::quote! {#inn_type_stdrt_nn_ts(#core_default)},
                                PgType::StdVecVecU8AsBytea => &quote::quote! {vec![#core_default]},
                                PgType::SqlxTypesTimeTimeAsTime => &gen_sqlx_types_time_time_from_hms_micro_unwrap_ts(&quote::quote! {0,0,0,0}),
                                PgType::SqlxPgTypesPgIntervalAsInterval => &quote::quote! {#inn_type_stdrt_nn_ts {
                                    #months_sc: #core_default,
                                    #days_sc: #core_default,
                                    #microseconds_sc: #core_default
                                }},
                                PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp => &gen_sqlx_types_chrono_naive_date_time_new_ts(&quote::quote! {
                                    #sqlx_types_chrono_naive_date_as_nn_date_orgn_as_dflt_some_one_el_call_ts.0,
                                    #sqlx_types_chrono_naive_time_as_nn_time_orgn_as_dflt_some_one_el_call_ts.0,
                                }),
                                PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => &gen_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_from_naive_utc_and_offset_ts(&gen_sqlx_types_chrono_naive_date_time_new_ts(&quote::quote! {
                                    #sqlx_types_chrono_naive_date_as_nn_date_orgn_as_dflt_some_one_el_call_ts.0,
                                    #sqlx_types_chrono_naive_time_as_nn_time_orgn_as_dflt_some_one_el_call_ts.0,
                                })),
                                PgType::SqlxTypesIpnetworkIpNetworkAsInet => &quote::quote! {
                                    sqlx::types::ipnetwork::IpNetwork::V4(sqlx::types::ipnetwork::Ipv4Network::#new_sc(core::net::Ipv4Addr::UNSPECIFIED, #core_default).expect("9e9c9b57"))
                                },
                                PgType::SqlxPgTypesPgRangeI32AsInt4Range | PgType::SqlxPgTypesPgRangeI64AsInt8Range => &pg_range_int_dflt_init_ts,
                                PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => &gen_sqlx_pg_types_pg_range_dflt_some_one_el_ts(&sqlx_types_chrono_naive_date_as_nn_date_orgn_ucc),
                                PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => &gen_sqlx_pg_types_pg_range_dflt_some_one_el_ts(&sqlx_types_chrono_naive_date_time_as_nn_timestamp_orgn_ucc),
                                PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => &gen_sqlx_pg_types_pg_range_dflt_some_one_el_ts(&sqlx_types_chrono_date_time_sqlx_types_chrono_utc_as_nn_timestamptz_orgn_ucc),
                            };
                            quote::quote! {#init_ts}
                        }
                        pg_crud_macros_cmn::IsNl::True => some_dflt_some_one_el_call_ts,
                    },
                };
                quote::quote! {Self(#ts)}
            });
            let impl_sqlx_type_and_encode_for_ident_orgn_ts = pg_crud_macros_cmn::gen_impl_sqlx_type_and_encode_for_ident_ts(&ident_orgn_ucc, &ft_h, &sqlx_encode_self_dot_zero_ts);
            let impl_sqlx_decode_sqlx_pg_for_ident_orgn_ts = pg_crud_macros_cmn::gen_impl_sqlx_decode_sqlx_pg_for_ident_ts(&ident_orgn_ucc, &ft_h, &{
                let scopes_v_ts = quote::quote! {(v)};
                let ok_self_scopes_v_ts = quote::quote! {Ok(Self #scopes_v_ts)};
                match &pg_type_pattern {
                    PgTypePattern::Stdrt => match &is_nl {
                        pg_crud_macros_cmn::IsNl::False => match &pg_type {
                            PgType::I16AsInt2
                            | PgType::I32AsInt4
                            | PgType::I64AsInt8
                            | PgType::F32AsFloat4
                            | PgType::F64AsFloat8
                            | PgType::I16AsSmallSerialInitByPg
                            | PgType::I32AsSerialInitByPg
                            | PgType::I64AsBigSerialInitByPg
                            | PgType::SqlxPgTypesPgMoneyAsMoney
                            | PgType::BoolAsBool
                            | PgType::StringAsText
                            | PgType::StdVecVecU8AsBytea
                            | PgType::SqlxTypesChronoNaiveTimeAsTime
                            | PgType::SqlxTypesTimeTimeAsTime
                            | PgType::SqlxPgTypesPgIntervalAsInterval
                            | PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp
                            | PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz
                            | PgType::SqlxTypesUuidUuidAsUuidV4InitByPg
                            | PgType::SqlxTypesUuidUuidAsUuidInitByClient
                            | PgType::SqlxTypesIpnetworkIpNetworkAsInet
                            | PgType::SqlxTypesMacAddressMacAddressAsMacAddr
                            | PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange
                            | PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange
                            | PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => ok_self_scopes_v_ts,
                            PgType::SqlxTypesChronoNaiveDateAsDate | PgType::SqlxPgTypesPgRangeI32AsInt4Range | PgType::SqlxPgTypesPgRangeI64AsInt8Range => quote::quote! {
                                match Self::#try_new_sc #scopes_v_ts {
                                    Ok(v_93eb5329) => Ok(v_93eb5329),
                                    Err(er) => Err(Box::#new_sc(er)),
                                }
                            },
                        },
                        pg_crud_macros_cmn::IsNl::True => ok_self_scopes_v_ts,
                    },
                }
            });
            let mb_impl_from_ident_rd_for_ident_orgn_ts = match &is_nn_stdrt_can_be_pk {
                IsNnStdrtCanBePk::False => proc_macro2::TokenStream::new(),
                IsNnStdrtCanBePk::True => macros_helpers::gen_impl_from_ts::gen_impl_from_ts(&ident_stdrt_nn_rd_ucc, &ident_orgn_ucc, &{
                    let ident_stdrt_nn_as_crate_pg_type_ts = gen_as_pg_type_ts(&ident_stdrt_nn_ucc);
                    quote::quote! {Self::#new_sc(#ident_stdrt_nn_as_crate_pg_type_ts::into_inn(#v_sc))}
                }).into(),
            };
            quote::quote! {
                #ident_orgn_ts
                #mb_pub_enum_ident_stdrt_nn_orgn_try_new_er_ts
                #mb_pub_enum_ident_stdrt_nn_orgn_try_new_for_de_er_ts
                #impl_ident_orgn_ts
                #impl_from_ident_orgn_for_ident_inn_type_ts
                #mb_impl_is_string_empty_for_ident_orgn_ts
                #mb_impl_ser_for_ident_stdrt_nn_orgn_ts
                #mb_impl_de_for_ident_stdrt_nn_orgn_ts
                #md_de_from_for_ident_stndrt_nn_orgn_ts
                #md_de_try_from_for_ident_stndrt_nn_orgn_ts
                #impl_display_for_ident_orgn_ts
                #impl_loc_lib_to_err_string_for_ident_orgn_ts
                #impl_dflt_some_one_el_for_ident_orgn_ts
                #impl_sqlx_type_and_encode_for_ident_orgn_ts
                #impl_sqlx_decode_sqlx_pg_for_ident_orgn_ts
                #mb_impl_from_ident_rd_for_ident_orgn_ts
            }
        };
        let gen_pub_struct_tokens_ts = |ident_ts_prm: &dyn quote::ToTokens, ts: &dyn quote::ToTokens, derive_dflt| {
            macros_helpers::derive_ts_builder::DTsBuilder::new()
                .make_pub()
                .d_debug()
                .d_default_if(derive_dflt)
                .d_clone()
                .d_copy()
                .d_partial_eq()
                .d_serde_serialize()
                .d_serde_deserialize()
                .build_struct(
                    &proc_macro2::TokenStream::new(),
                    &ident_ts_prm,
                    &proc_macro2::TokenStream::new(),
                    &ts
                )
        };
        let ident_orgn_struct_ts = quote::quote! {(#ident_orgn_ucc);};
        let self_dflt_some_one_el_call_ts = quote::quote! {Self(#pg_crud_cmn_dflt_some_one_el_call)};
        let ok_self_v_ts = quote::quote! {Ok(Self(v))};
        let ident_tt_ucc = naming::prm::SelfTtUcc::from_tokens(&ident);
        let ident_tt_ts = {
            let ident_tt_ts = macros_helpers::derive_ts_builder::DTsBuilder::new()
                .make_pub()
                .d_debug()
                .d_clone()
                .d_copy_if(derive_copy)
                .d_partial_eq()
                .d_partial_ord_if(d_partial_ord)
                .d_serde_serialize()
                .d_serde_deserialize()
                .build_struct(
                    &proc_macro2::TokenStream::new(),
                    &ident_tt_ucc,
                    &proc_macro2::TokenStream::new(),
                    &ident_orgn_struct_ts
                );
            let impl_ident_tt_ts = gen_pub_const_new_or_pub_try_new_ts(&ident_tt_ucc);
            let impl_dflt_some_one_el_for_ident_tt_ts =
                pg_crud_macros_cmn::gen_impl_pg_crud_cmn_dflt_some_one_el_ts(&ident_tt_ucc, &self_dflt_some_one_el_call_ts);
            let impl_sqlx_type_and_encode_for_ident_tt_ts = pg_crud_macros_cmn::gen_impl_sqlx_type_and_encode_for_ident_ts(&ident_tt_ucc, &ident_orgn_ucc, &sqlx_encode_self_dot_zero_ts);
            let impl_sqlx_decode_sqlx_pg_for_ident_tt_ts = pg_crud_macros_cmn::gen_impl_sqlx_decode_sqlx_pg_for_ident_ts(&ident_tt_ucc, &ident_orgn_ucc, &ok_self_v_ts);
            //todo rewrite as dependency of PgType trait?
            let impl_pg_type_eq_oprtr_for_ident_tt_ts = pg_crud_macros_cmn::impl_pg_type_eq_oprtr_for_ident_ts(
                &import,
                &ident_tt_ucc,
                //todo
                &{
                    let eq_ts = pg_crud_macros_cmn::EqOprtrH::Eq.to_tokens_path(&import);
                    let is_null_ts = pg_crud_macros_cmn::EqOprtrH::IsNull.to_tokens_path(&import);
                    let nl_eq_oprtr_ts = quote::quote! {
                        if self.0.0.is_some() {
                            #eq_ts
                        }
                        else {
                            #is_null_ts
                        }
                    };
                    match &pg_type_pattern {
                        PgTypePattern::Stdrt => match &is_nl {
                            pg_crud_macros_cmn::IsNl::False => eq_ts,
                            pg_crud_macros_cmn::IsNl::True => macros_helpers::generated_rust_ts::GeneratedRustTs::from(nl_eq_oprtr_ts),
                        },
                    }
                },
            );
            quote::quote! {
                #ident_tt_ts
                #impl_ident_tt_ts
                #impl_dflt_some_one_el_for_ident_tt_ts
                #impl_sqlx_type_and_encode_for_ident_tt_ts
                #impl_sqlx_decode_sqlx_pg_for_ident_tt_ts
                #impl_pg_type_eq_oprtr_for_ident_tt_ts
            }
        };
        let ident_stdrt_nn_tt_ucc = naming::prm::SelfTtUcc::from_tokens(&ident_stdrt_nn_ucc);
        let cmn_d_ts_builder = pg_crud_macros_cmn::ts_helpers::cmn_d_ts_builder()
            .d_copy_if(derive_copy);
        let ident_cr_ucc = naming::prm::SelfCrUcc::from_tokens(&ident);
        let ident_cr_ts = {
            let ident_cr_ts = match &can_be_pk {
                CanBePk::False => cmn_d_ts_builder.build_struct(
                        &proc_macro2::TokenStream::new(),
                        &ident_cr_ucc,
                        &proc_macro2::TokenStream::new(),
                        &ident_orgn_struct_ts
                    ),
                CanBePk::True => gen_pub_struct_tokens_ts(&ident_cr_ucc, &quote::quote! {(());}, macros_helpers::derive_ts_builder::DDefault::False),
            };
            let mb_impl_ident_cr_ts = match &can_be_pk {
                CanBePk::False => gen_pub_const_new_or_pub_try_new_ts(&ident_cr_ucc),
                CanBePk::True => proc_macro2::TokenStream::new(),
            };
            let impl_dflt_some_one_el_for_ident_cr_ts = pg_crud_macros_cmn::gen_impl_pg_crud_cmn_dflt_some_one_el_ts(&ident_cr_ucc, &{
                let ts: &dyn quote::ToTokens = match &can_be_pk {
                    CanBePk::False => &pg_crud_cmn_dflt_some_one_el_call,
                    CanBePk::True => &quote::quote! {()},
                };
                quote::quote! {Self(#ts)}
            });
            let mb_impl_sqlx_type_and_encode_for_ident_cr_ts = match &can_be_pk {
                CanBePk::False => pg_crud_macros_cmn::gen_impl_sqlx_type_and_encode_for_ident_ts(&ident_cr_ucc, &ident_orgn_ucc, &sqlx_encode_self_dot_zero_ts),
                CanBePk::True => macros_helpers::generated_rust_ts::GeneratedRustTs::from(proc_macro2::TokenStream::new()),
            };
            quote::quote! {
                #ident_cr_ts
                #mb_impl_ident_cr_ts
                #impl_dflt_some_one_el_for_ident_cr_ts
                #mb_impl_sqlx_type_and_encode_for_ident_cr_ts
            }
        };
        let ident_sel_ucc = naming::prm::SelfSelUcc::from_tokens(&ident);
        let ident_sel_ts = {
            let pub_struct_ident_sel_ts = gen_pub_struct_tokens_ts(
                &ident_sel_ucc,
                &quote::quote! {;},
                macros_helpers::derive_ts_builder::DDefault::True,
            );
            let (impl_dflt_some_one_el_for_ident_sel_ts, impl_dflt_some_one_el_max_page_size_for_ident_sel_ts) = {
                (
                    pg_crud_macros_cmn::gen_impl_pg_crud_cmn_dflt_some_one_el_ts(&ident_sel_ucc, &quote::quote! {Self}),
                    pg_crud_macros_cmn::gen_impl_pg_crud_cmn_dflt_some_one_el_max_page_size_ts(&ident_sel_ucc, &quote::quote! {Self}),
                )
            };
            quote::quote! {
                #pub_struct_ident_sel_ts
                #impl_dflt_some_one_el_for_ident_sel_ts
                #impl_dflt_some_one_el_max_page_size_for_ident_sel_ts
            }
        };
        let ident_rd_ucc = naming::prm::SelfRdUcc::from_tokens(&ident);
        let ident_wh_ucc = naming::prm::SelfWhUcc::from_tokens(&ident);
        let ident_wh_ts = pg_crud_macros_cmn::gen_pg_type_wh_ts(
            &allow_clippy_arbitrary_src_item_ordering,
            &{
                let cmn_pg_type_flts = vec![pg_crud_macros_cmn::flts::PgTypeFlt::Eq {
                    ident: macros_helpers::generated_rust_ts::GeneratedRustTs::from(quote::quote! {#ident_tt_ucc}),
                }];
                let gen_flts_with = |base: Vec<pg_crud_macros_cmn::flts::PgTypeFlt>, extra: &[pg_crud_macros_cmn::flts::PgTypeFlt]| {
                    let mut vec = base;
                    vec.extend_from_slice(extra);
                    vec
                };
                match &pg_type_pattern {
                    PgTypePattern::Stdrt => {
                        let greater_than = pg_crud_macros_cmn::flts::PgTypeFlt::GreaterThan {
                            ident: macros_helpers::generated_rust_ts::GeneratedRustTs::from(quote::quote! {#ident_stdrt_nn_tt_ucc}),
                        };
                        let btwn = pg_crud_macros_cmn::flts::PgTypeFlt::Btwn {
                            ident: macros_helpers::generated_rust_ts::GeneratedRustTs::from(quote::quote! {#ident_stdrt_nn_tt_ucc}),
                        };
                        let in_h = pg_crud_macros_cmn::flts::PgTypeFlt::In {
                            ident: macros_helpers::generated_rust_ts::GeneratedRustTs::from(quote::quote! {#ident_tt_ucc}),
                        };
                        let rgx = pg_crud_macros_cmn::flts::PgTypeFlt::Rgx;
                        let eq_to_encoded_string_representation = pg_crud_macros_cmn::flts::PgTypeFlt::EqToEncodedStringRepresentation;
                        let crnt_date_flt = pg_crud_macros_cmn::flts::PgTypeFlt::CrntDate;
                        let greater_than_crnt_date = pg_crud_macros_cmn::flts::PgTypeFlt::GreaterThanCrntDate;
                        let crnt_time_flt = pg_crud_macros_cmn::flts::PgTypeFlt::CrntTime;
                        let greater_than_crnt_time = pg_crud_macros_cmn::flts::PgTypeFlt::GreaterThanCrntTime;
                        let crnt_timestamp_flt = pg_crud_macros_cmn::flts::PgTypeFlt::CrntTimestamp;
                        let greater_than_crnt_timestamp = pg_crud_macros_cmn::flts::PgTypeFlt::GreaterThanCrntTimestamp;
                        let before = pg_crud_macros_cmn::flts::PgTypeFlt::Before {
                            ident: macros_helpers::generated_rust_ts::GeneratedRustTs::from(quote::quote! {#ident_stdrt_nn_tt_ucc}),
                        };
                        let cmn_stdrt_pg_type_flts = { cmn_pg_type_flts };
                        let cmn_stdrt_pg_type_nbr_flts = gen_flts_with(
                            cmn_stdrt_pg_type_flts.clone(),
                            &[greater_than.clone(), btwn.clone(), in_h.clone()],
                        );
                        let ranges_cmn_flt_vec = {
                            let range_ident_ts = macros_helpers::generated_rust_ts::GeneratedRustTs::from(quote::quote! {#ident_stdrt_nn_tt_ucc});
                            gen_flts_with(cmn_stdrt_pg_type_flts.clone(), &[
                                pg_crud_macros_cmn::flts::PgTypeFlt::FindRangesWithinGivenRange { ident: range_ident_ts.clone() },
                                pg_crud_macros_cmn::flts::PgTypeFlt::FindRangesThatFullyContainTheGivenRange { ident: range_ident_ts.clone() },
                                pg_crud_macros_cmn::flts::PgTypeFlt::StrictlyToLeftOfRange { ident: range_ident_ts.clone() },
                                pg_crud_macros_cmn::flts::PgTypeFlt::StrictlyToRightOfRange { ident: range_ident_ts.clone() },
                                pg_crud_macros_cmn::flts::PgTypeFlt::IncludedLowerBound { ident: range_ident_ts.clone() },
                                pg_crud_macros_cmn::flts::PgTypeFlt::ExcludedUpperBound { ident: range_ident_ts.clone() },
                                pg_crud_macros_cmn::flts::PgTypeFlt::GreaterThanIncludedLowerBound { ident: range_ident_ts.clone() },
                                pg_crud_macros_cmn::flts::PgTypeFlt::GreaterThanExcludedUpperBound { ident: range_ident_ts.clone() },
                                pg_crud_macros_cmn::flts::PgTypeFlt::OverlapWithRange { ident: range_ident_ts.clone() },
                                pg_crud_macros_cmn::flts::PgTypeFlt::AdjacentWithRange { ident: range_ident_ts },
                                pg_crud_macros_cmn::flts::PgTypeFlt::RangeLen,
                            ])
                        };
                        match &pg_type {
                            PgType::I16AsInt2
                            | PgType::I32AsInt4
                            | PgType::I64AsInt8
                            | PgType::F32AsFloat4
                            | PgType::F64AsFloat8
                            | PgType::I16AsSmallSerialInitByPg
                            | PgType::I32AsSerialInitByPg
                            | PgType::I64AsBigSerialInitByPg => cmn_stdrt_pg_type_nbr_flts,
                            PgType::SqlxPgTypesPgMoneyAsMoney => gen_flts_with(cmn_stdrt_pg_type_flts, &[in_h]),
                            PgType::StdVecVecU8AsBytea => gen_flts_with(cmn_stdrt_pg_type_flts, &[eq_to_encoded_string_representation]),
                            PgType::SqlxTypesChronoNaiveTimeAsTime | PgType::SqlxTypesTimeTimeAsTime => gen_flts_with(cmn_stdrt_pg_type_flts, &[greater_than, btwn, crnt_time_flt, greater_than_crnt_time]),
                            PgType::SqlxTypesChronoNaiveDateAsDate => gen_flts_with(cmn_stdrt_pg_type_flts, &[greater_than, btwn, crnt_date_flt, greater_than_crnt_date]),
                            PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp => gen_flts_with(cmn_stdrt_pg_type_flts, &[greater_than, btwn, crnt_timestamp_flt, greater_than_crnt_timestamp]),
                            PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => gen_flts_with(cmn_stdrt_pg_type_flts, &[before, btwn]),
                            PgType::StringAsText | PgType::SqlxTypesUuidUuidAsUuidV4InitByPg | PgType::SqlxTypesUuidUuidAsUuidInitByClient => gen_flts_with(cmn_stdrt_pg_type_flts, &[rgx]),
                            PgType::BoolAsBool | PgType::SqlxPgTypesPgIntervalAsInterval | PgType::SqlxTypesIpnetworkIpNetworkAsInet => cmn_stdrt_pg_type_flts,
                            PgType::SqlxTypesMacAddressMacAddressAsMacAddr => gen_flts_with(cmn_stdrt_pg_type_flts, &[greater_than, rgx]),
                            PgType::SqlxPgTypesPgRangeI32AsInt4Range |
                            PgType::SqlxPgTypesPgRangeI64AsInt8Range |
                            PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange |
                            PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange |
                            PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => ranges_cmn_flt_vec,
                        }
                    }
                }
            }
            .iter()
            .map(|el0| {
                let el1: &dyn pg_crud_macros_cmn::flts::PgFlt = el0;
                el1
            })
            .collect(),
            &ident,
            &pg_crud_macros_cmn::ShouldDeriveUtoipaToSchema::False,
            &pg_crud_macros_cmn::ShouldDSchemarsJsonSchema::False,
            &pg_crud_macros_cmn::IsQbMut::False,
        );
        let ident_rd_ts = {
            let ident_rd_ts = {
                let (
                    derive_eq,
                    derive_partial_ord,
                    derive_ord
                ) = match &is_nn_stdrt_can_be_pk {
                    IsNnStdrtCanBePk::False => (
                        macros_helpers::derive_ts_builder::DEq::False,
                        macros_helpers::derive_ts_builder::DPartialOrd::False,
                        macros_helpers::derive_ts_builder::DOrd::False
                    ),
                    IsNnStdrtCanBePk::True => (
                        macros_helpers::derive_ts_builder::DEq::True,
                        macros_helpers::derive_ts_builder::DPartialOrd::True,
                        macros_helpers::derive_ts_builder::DOrd::True
                    ),
                };
                macros_helpers::derive_ts_builder::DTsBuilder::new()
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
                    .build_struct(
                        &proc_macro2::TokenStream::new(),
                        &ident_rd_ucc,
                        &proc_macro2::TokenStream::new(),
                        &ident_orgn_struct_ts
                    )
            };
            let impl_ident_rd_ts = gen_pub_const_new_or_pub_try_new_ts(&ident_rd_ucc);
            let impl_loc_lib_to_err_string_for_ident_rd_ts = pg_crud_macros_cmn::gen_impl_to_err_string_no_generics_ts(&ident_rd_ucc, &quote::quote! {self.0.to_string()});
            let impl_crate_dflt_some_one_el_for_ident_rd_ts =
                pg_crud_macros_cmn::gen_impl_pg_crud_cmn_dflt_some_one_el_ts(&ident_rd_ucc, &self_dflt_some_one_el_call_ts);
            let impl_sqlx_type_and_encode_for_ident_rd_ts = pg_crud_macros_cmn::gen_impl_sqlx_type_and_encode_for_ident_ts(&ident_rd_ucc, &ident_orgn_ucc, &sqlx_encode_self_dot_zero_ts);
            let impl_sqlx_decode_sqlx_pg_for_ident_rd_ts = pg_crud_macros_cmn::gen_impl_sqlx_decode_sqlx_pg_for_ident_ts(
                &ident_rd_ucc,
                &ident_orgn_ucc,
                &ok_self_v_ts
            );
            let mb_impl_pg_type_wh_flt_for_ident_rd_if_can_be_pk_ts = if matches!(&is_nn_stdrt_can_be_pk, IsNnStdrtCanBePk::True) {
                pg_crud_macros_cmn::impl_pg_type_wh_flt_for_ident_ts(
                    &quote::quote! {<'lt>},
                    &ident_stdrt_nn_rd_ucc,
                    &proc_macro2::TokenStream::new(),
                    &pg_crud_macros_cmn::IncrPrmUndrscr::False,
                    &pg_crud_macros_cmn::ColPrmUndrscr::False,
                    &pg_crud_macros_cmn::AddOprtrUndrscr::True,
                    &quote::quote! {
                        match #import::incr_checked_add_one_returning_incr(#incr_sc) {
                            Ok(v_8da76391) => Ok(#import::QpFragment::try_from(format!("({col} = ${v_8da76391})")).unwrap_or_else(#import::QpFragment::from)),
                            Err(er) => Err(er)
                        }
                    },
                    &pg_crud_macros_cmn::IsQbMut::True,
                    &gen_typical_pg_query_qb_ts(&self_sc),
                    &import,
                )
            } else {
                macros_helpers::generated_rust_ts::GeneratedRustTs::from(proc_macro2::TokenStream::new())
            };
            quote::quote! {
                #ident_rd_ts
                #impl_ident_rd_ts
                #impl_loc_lib_to_err_string_for_ident_rd_ts
                #impl_crate_dflt_some_one_el_for_ident_rd_ts
                #impl_sqlx_type_and_encode_for_ident_rd_ts
                #impl_sqlx_decode_sqlx_pg_for_ident_rd_ts
                #mb_impl_pg_type_wh_flt_for_ident_rd_if_can_be_pk_ts
            }
        };
        let ident_rd_ids_ucc = naming::prm::SelfRdIdsUcc::from_tokens(&ident);
        let ident_rd_ids_ts = if matches!(&is_nn_stdrt_can_be_pk, IsNnStdrtCanBePk::True) {
            let ident_rd_ids_ts = cmn_d_ts_builder.build_struct(
                    &proc_macro2::TokenStream::new(),
                    &ident_rd_ids_ucc,
                    &proc_macro2::TokenStream::new(),
                    &quote::quote! {(#ident_rd_ucc);},
                );
            let impl_sqlx_decode_sqlx_pg_for_ident_rd_ids_ts = pg_crud_macros_cmn::gen_impl_sqlx_decode_sqlx_pg_for_ident_ts(
                &ident_rd_ids_ucc,
                &ident_rd_ucc,
                &ok_self_v_ts
            );
            let impl_sqlx_type_for_ident_rd_ids_ts = pg_crud_macros_cmn::gen_impl_sqlx_type_for_ident_ts(&ident_rd_ids_ucc, &ident_rd_ucc);
            quote::quote! {
                #ident_rd_ids_ts
                #impl_sqlx_decode_sqlx_pg_for_ident_rd_ids_ts
                #impl_sqlx_type_for_ident_rd_ids_ts
            }
        } else {
            proc_macro2::TokenStream::new()
        };
        let ident_rd_inn_ucc = naming::prm::SelfRdInnUcc::from_tokens(&ident);
        let ident_rd_inn_ts = quote::quote! {
            pub type #ident_rd_inn_ucc = #ident_inn_type_ts;
        };
        let ident_upd_ts = {
            let ident_upd_ts = cmn_d_ts_builder.build_struct(
                    &proc_macro2::TokenStream::new(),
                    &ident_upd_ucc,
                    &proc_macro2::TokenStream::new(),
                    &ident_orgn_struct_ts
                );
            let impl_ident_upd_ts = gen_pub_const_new_or_pub_try_new_ts(&ident_upd_ucc);
            let impl_dflt_some_one_el_for_ident_upd_ts =
                pg_crud_macros_cmn::gen_impl_pg_crud_cmn_dflt_some_one_el_ts(&ident_upd_ucc, &self_dflt_some_one_el_call_ts);
            let impl_loc_lib_to_err_string_for_ident_upd_ts = pg_crud_macros_cmn::gen_impl_to_err_string_no_generics_ts(&ident_upd_ucc, &quote::quote! {self.0.#to_err_string_sc().into_inner()});
            quote::quote! {
                #ident_upd_ts
                #impl_ident_upd_ts
                #impl_dflt_some_one_el_for_ident_upd_ts
                #impl_loc_lib_to_err_string_for_ident_upd_ts
            }
        };
        let ident_upd_for_query_ucc = naming::prm::SelfUpdForQueryUcc::from_tokens(&ident);
        let ident_upd_for_query_ts = {
            let ident_upd_for_query_ts = cmn_d_ts_builder.build_struct(
                    &proc_macro2::TokenStream::new(),
                    &ident_upd_for_query_ucc,
                    &proc_macro2::TokenStream::new(),
                    &ident_orgn_struct_ts
                );
            let impl_sqlx_type_and_encode_for_ident_upd_for_query_ts = pg_crud_macros_cmn::gen_impl_sqlx_type_and_encode_for_ident_ts(&ident_upd_for_query_ucc, &ident_orgn_ucc, &sqlx_encode_self_dot_zero_ts);
            let impl_from_ident_upd_for_ident_upd_for_query_ts = macros_helpers::gen_impl_from_ts::gen_impl_from_ts(&ident_upd_ucc, &ident_upd_for_query_ucc, &quote::quote! {Self(#v_sc.0)});
            quote::quote! {
                #ident_upd_for_query_ts
                #impl_sqlx_type_and_encode_for_ident_upd_for_query_ts
                #impl_from_ident_upd_for_ident_upd_for_query_ts
            }
        };
        let impl_pg_type_for_ident_ts = {
            let gen_ok_string_from_tokens_ts = |ts: &dyn quote::ToTokens| {
                quote::quote! {Ok(#import::QpFragment::try_from(#string_ts::from(#ts)).unwrap_or_else(#import::QpFragment::from))}
            };
            let ok_string_from_dflt_ts = gen_ok_string_from_tokens_ts(&quote::quote! {"dflt"});
            let ok_string_from_uuid_generate_v4_ts = gen_ok_string_from_tokens_ts(&quote::quote! {"uuid_generate_v4()"});
            let typical_qp_ts = {
                let if_write_is_err_ts = macros_helpers::gen_if_write_is_err_ts::gen_if_write_is_err_ts(
                    &quote::quote! {acc_c7df00f5, "${v_ba581e0f}"},
                    &pg_crud_macros_cmn::gen_return_err_qp_er_write_into_buffer_ts(import)
                );
                quote::quote! {
                    let mut acc_c7df00f5 = String::default();
                    match #import::incr_checked_add_one_returning_incr(#incr_sc) {
                        Ok(v_ba581e0f) => {
                            #if_write_is_err_ts
                        },
                        Err(er) => {
                            return Err(er);
                        }
                    }
                    Ok(#import::QpFragment::try_from(acc_c7df00f5).unwrap_or_else(#import::QpFragment::from))
                }
            };
            let ok_query_ts = quote::quote! {Ok(#query_sc)};
            let (qp_cr_ts, bind_v_to_query_cr_ts): (&dyn quote::ToTokens, &dyn quote::ToTokens) = {
                let typical: (&dyn quote::ToTokens, &dyn quote::ToTokens) = { (&typical_qp_ts, &typical_qb_ts) };
                let dflt_init_by_pg: (&dyn quote::ToTokens, &dyn quote::ToTokens) = (&ok_string_from_dflt_ts, &ok_query_ts);
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
                    | PgType::SqlxTypesUuidUuidAsUuidInitByClient
                    | PgType::SqlxTypesIpnetworkIpNetworkAsInet
                    | PgType::SqlxTypesMacAddressMacAddressAsMacAddr
                    | PgType::SqlxPgTypesPgRangeI32AsInt4Range
                    | PgType::SqlxPgTypesPgRangeI64AsInt8Range
                    | PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange
                    | PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange
                    | PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => typical,
                    PgType::I16AsSmallSerialInitByPg | PgType::I32AsSerialInitByPg | PgType::I64AsBigSerialInitByPg => dflt_init_by_pg,
                    PgType::SqlxTypesUuidUuidAsUuidV4InitByPg => (&ok_string_from_uuid_generate_v4_ts, &ok_query_ts),
                }
            };
            let sel_only_ids_and_sel_only_updd_ids_query_cmn_ts = {
                let format_ts = gen_quotes::dq_ts(&{
                    let col_comma = "{col},";
                    col_comma.to_owned()
                });
                quote::quote! {Ok(#import::QpFragment::try_from(format!(#format_ts)).unwrap_or_else(#import::QpFragment::from))}
            };
            pg_crud_macros_cmn::gen_impl_pg_type_ts(
                &import,
                &ident,
                &ident_tt_ucc,
                &match &can_be_pk {
                    CanBePk::False => pg_crud_macros_cmn::IsPkUndrscr::True,
                    CanBePk::True => pg_crud_macros_cmn::IsPkUndrscr::False,
                },
                &{
                    let pg_query_type = match &pg_type {
                        PgType::I16AsInt2 => "int2",
                        PgType::I32AsInt4 => "int4",
                        PgType::I64AsInt8 => "int8",
                        PgType::F32AsFloat4 => "float4",
                        PgType::F64AsFloat8 => "float8",
                        PgType::I16AsSmallSerialInitByPg => "smallserial",
                        PgType::I32AsSerialInitByPg => "serial",
                        PgType::I64AsBigSerialInitByPg => "bigserial",
                        PgType::SqlxPgTypesPgMoneyAsMoney => "money",
                        PgType::BoolAsBool => "bool",
                        PgType::StringAsText => "text",
                        PgType::StdVecVecU8AsBytea => "bytea",
                        PgType::SqlxTypesChronoNaiveTimeAsTime | PgType::SqlxTypesTimeTimeAsTime => "time",
                        PgType::SqlxPgTypesPgIntervalAsInterval => "interval",
                        PgType::SqlxTypesChronoNaiveDateAsDate => "date",
                        PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp => "timestamp",
                        PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => "timestamptz",
                        PgType::SqlxTypesUuidUuidAsUuidV4InitByPg | PgType::SqlxTypesUuidUuidAsUuidInitByClient => "uuid",
                        PgType::SqlxTypesIpnetworkIpNetworkAsInet => "inet",
                        PgType::SqlxTypesMacAddressMacAddressAsMacAddr => "macaddr",
                        PgType::SqlxPgTypesPgRangeI32AsInt4Range => "int4range",
                        PgType::SqlxPgTypesPgRangeI64AsInt8Range => "int8range",
                        PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => "daterange",
                        PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => "tsrange",
                        PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => "tstzrange",
                    };
                    let mb_pk_is_pk_ts = quote::quote! {pg_types_cmn::mb_pk(is_pk)};
                    let col_pg_query_type = format!("{{col}} {pg_query_type}");
                    let col_pg_query_type_nn = format!("{{col}} {pg_query_type} not null");
                    let space_extra_prm = " {}";
                    match (&is_nl, &can_be_pk) {
                        (pg_crud_macros_cmn::IsNl::False, CanBePk::False) => {
                            let format_ts = gen_quotes::dq_ts(&col_pg_query_type_nn);
                            quote::quote! {
                                #import::QpFragment::try_from(format!(#format_ts)).unwrap_or_else(#import::QpFragment::from)
                            }
                        }
                        (pg_crud_macros_cmn::IsNl::False, CanBePk::True) => {
                            let format_ts = gen_quotes::dq_ts(&format!("{col_pg_query_type_nn}{space_extra_prm}"));
                            quote::quote! {
                                #import::QpFragment::try_from(format!(#format_ts, #mb_pk_is_pk_ts)).unwrap_or_else(#import::QpFragment::from)
                            }
                        }
                        (pg_crud_macros_cmn::IsNl::True, CanBePk::False) => {
                            let format_ts = gen_quotes::dq_ts(&col_pg_query_type);
                            quote::quote! {
                                #import::QpFragment::try_from(format!(#format_ts)).unwrap_or_else(#import::QpFragment::from)
                            }
                        }
                        (pg_crud_macros_cmn::IsNl::True, CanBePk::True) => {
                            let format_ts = gen_quotes::dq_ts(&format!("{col_pg_query_type}{space_extra_prm}"));
                            quote::quote! {
                                #import::QpFragment::try_from(format!(#format_ts, #mb_pk_is_pk_ts)).unwrap_or_else(#import::QpFragment::from)
                            }
                        }
                    }
                },
                &ident_cr_ucc,
                &pg_crud_macros_cmn::CrQpValueUndrscr::True,
                &match &can_be_pk {
                    CanBePk::False => pg_crud_macros_cmn::CrQpIncrUndrscr::False,
                    CanBePk::True => pg_crud_macros_cmn::CrQpIncrUndrscr::True,
                },
                &qp_cr_ts,
                &match &can_be_pk {
                    CanBePk::False => pg_crud_macros_cmn::CrQbValueUndrscr::False,
                    CanBePk::True => pg_crud_macros_cmn::CrQbValueUndrscr::True,
                },
                &match &can_be_pk {
                    CanBePk::False => pg_crud_macros_cmn::IsCrQbMut::True,
                    CanBePk::True => pg_crud_macros_cmn::IsCrQbMut::False,
                },
                &bind_v_to_query_cr_ts,
                &ident_sel_ucc,
                &pg_crud_macros_cmn::SelQpValueUndrscr::True,
                &{
                    let ts = quote::quote! {#import::QpFragment::try_from(#col_sc.to_string()).unwrap_or_else(#import::QpFragment::from)};
                    quote::quote! {Ok(#ts)}
                },
                &ident_wh_ucc,
                &ident_rd_ucc,
                &{
                    let gen_ident_rd_ident_orgn_ts = |ts: &dyn quote::ToTokens| {
                        quote::quote! {#ident_rd_ucc(#ident_orgn_ucc(#ts))}
                    };
                    match &pg_type_pattern {
                        PgTypePattern::Stdrt => match &is_nl {
                            pg_crud_macros_cmn::IsNl::False => {
                                Range::try_from(pg_type).as_ref().map_or_else(
                                    |()| quote::quote! {#v_sc},
                                    |range| {
                                        let gen_sqlx_pg_types_pg_range_ts = |start_ts: &dyn quote::ToTokens, end_ts: &dyn quote::ToTokens| {
                                            quote::quote! {
                                                sqlx::postgres::types::PgRange{
                                                    #start_sc: std::ops::Bound::#start_ts,
                                                    #end_sc: std::ops::Bound::#end_ts
                                                }
                                            }
                                        };
                                        let included_start_ts = quote::quote! {#included_ucc(#start_sc)};
                                        let excluded_end_ts = quote::quote! {#excluded_ucc(#end_sc)};
                                        let included_end_ts = quote::quote! {#included_ucc(#end_sc)};
                                        let excluded_start_ts = quote::quote! {#excluded_ucc(#start_sc)};
                                        let sqlx_pg_types_pg_range_excluded_excluded_ts = gen_sqlx_pg_types_pg_range_ts(&excluded_start_ts, &excluded_end_ts);
                                        let sqlx_pg_types_pg_range_excluded_included_ts = gen_sqlx_pg_types_pg_range_ts(&excluded_start_ts, &included_end_ts);
                                        let sqlx_pg_types_pg_range_included_unbounded_ts = gen_sqlx_pg_types_pg_range_ts(&included_start_ts, &unbounded_ucc);
                                        let sqlx_pg_types_pg_range_unbounded_excluded_ts = gen_sqlx_pg_types_pg_range_ts(&unbounded_ucc, &excluded_end_ts);
                                        let sqlx_pg_types_pg_range_included_excluded_ts = gen_sqlx_pg_types_pg_range_ts(&included_start_ts, &excluded_end_ts);
                                        let sqlx_pg_types_pg_range_unbounded_unbounded_ts = gen_sqlx_pg_types_pg_range_ts(&unbounded_ucc, &unbounded_ucc);
                                        let gen_range_match_ts = |
                                            included_included_ts: &dyn quote::ToTokens,
                                            included_excluded_ts: &dyn quote::ToTokens,
                                            included_unbounded_ts: &dyn quote::ToTokens,
                                            excluded_included_ts: &dyn quote::ToTokens,
                                            excluded_excluded_ts: &dyn quote::ToTokens,
                                            excluded_unbounded_ts: &dyn quote::ToTokens,
                                            unbounded_included_ts: &dyn quote::ToTokens,
                                            unbounded_excluded_ts: &dyn quote::ToTokens
                                        | {
                                            quote::quote! {
                                                #ident_stdrt_nn_rd_ucc(#ident_stdrt_nn_orgn_ucc(match (
                                                    #v_sc.0.0.#start_sc,
                                                    #v_sc.0.0.#end_sc
                                                ) {
                                                    (std::ops::Bound::#included_ucc(#start_sc), std::ops::Bound::#included_ucc(#end_sc)) => {
                                                        #included_included_ts
                                                    },
                                                    (std::ops::Bound::#included_ucc(#start_sc), std::ops::Bound::#excluded_ucc(#end_sc)) => {
                                                        #included_excluded_ts
                                                    },
                                                    (std::ops::Bound::#included_ucc(#start_sc), std::ops::Bound::#unbounded_ucc) => {
                                                        #included_unbounded_ts
                                                    },
                                                    (std::ops::Bound::#excluded_ucc(#start_sc), std::ops::Bound::#included_ucc(#end_sc)) => {
                                                        #excluded_included_ts
                                                    },
                                                    (std::ops::Bound::#excluded_ucc(#start_sc), std::ops::Bound::#excluded_ucc(#end_sc)) => {
                                                        #excluded_excluded_ts
                                                    },
                                                    (std::ops::Bound::#excluded_ucc(#start_sc), std::ops::Bound::#unbounded_ucc) => {
                                                        #excluded_unbounded_ts
                                                    },
                                                    (std::ops::Bound::#unbounded_ucc, std::ops::Bound::#included_ucc(#end_sc)) => {
                                                        #unbounded_included_ts
                                                    },
                                                    (std::ops::Bound::#unbounded_ucc, std::ops::Bound::#excluded_ucc(#end_sc)) => {
                                                        #unbounded_excluded_ts
                                                    },
                                                    (std::ops::Bound::#unbounded_ucc, std::ops::Bound::#unbounded_ucc) => #sqlx_pg_types_pg_range_unbounded_unbounded_ts,
                                                }))
                                            }
                                        };
                                        let gen_if_start_end_eq_ts = |true_ts: &dyn quote::ToTokens, false_ts: &dyn quote::ToTokens| {
                                            quote::quote! {
                                                if #start_sc == #end_sc {
                                                    #true_ts
                                                } else {
                                                    #false_ts
                                                }
                                            }
                                        };
                                        let if_eq_unbounded_unbounded_or_included_excluded_ts = gen_if_start_end_eq_ts(&sqlx_pg_types_pg_range_unbounded_unbounded_ts, &sqlx_pg_types_pg_range_included_excluded_ts);
                                        let int_range_normalize_ts = {
                                            let (
                                                included_start_checked_add_ts,
                                                excluded_end_checked_add_ts
                                            ) = {
                                                let gen_ts = |first_ts: &dyn quote::ToTokens, second_ts: &dyn quote::ToTokens| {
                                                    quote::quote! {#first_ts(#second_ts.checked_add(1).expect("0ec0992f"))}
                                                };
                                                (
                                                    gen_ts(&included_ucc, &start_sc),
                                                    gen_ts(&excluded_ucc, &end_sc)
                                                )
                                            };
                                            let included_excluded_checked_add_ts = gen_sqlx_pg_types_pg_range_ts(&included_start_ts, &excluded_end_checked_add_ts);
                                            let included_unbounded_ts = gen_sqlx_pg_types_pg_range_ts(&included_start_ts, &unbounded_ucc);
                                            let included_checked_add_excluded_checked_add_ts = gen_sqlx_pg_types_pg_range_ts(&included_start_checked_add_ts, &excluded_end_checked_add_ts);
                                            let included_checked_add_excluded_ts = gen_sqlx_pg_types_pg_range_ts(&included_start_checked_add_ts, &excluded_end_ts);
                                            let included_checked_add_unbounded_ts = gen_sqlx_pg_types_pg_range_ts(&included_start_checked_add_ts, &unbounded_ucc);
                                            let unbounded_excluded_checked_add_ts = gen_sqlx_pg_types_pg_range_ts(&unbounded_ucc, &excluded_end_checked_add_ts);
                                            let unbounded_excluded_ts = gen_sqlx_pg_types_pg_range_ts(&unbounded_ucc, &excluded_end_ts);
                                            gen_range_match_ts(
                                                &included_excluded_checked_add_ts,
                                                &gen_if_start_end_eq_ts(&sqlx_pg_types_pg_range_unbounded_unbounded_ts, &sqlx_pg_types_pg_range_included_excluded_ts),
                                                &included_unbounded_ts,
                                                &gen_if_start_end_eq_ts(&sqlx_pg_types_pg_range_unbounded_unbounded_ts, &included_checked_add_excluded_checked_add_ts),
                                                &gen_if_start_end_eq_ts(&sqlx_pg_types_pg_range_unbounded_unbounded_ts, &included_checked_add_excluded_ts),
                                                &included_checked_add_unbounded_ts,
                                                &unbounded_excluded_checked_add_ts,
                                                &unbounded_excluded_ts,
                                            )
                                        };
                                        let range_match_timestamp_and_timestamp_tz_ts = gen_range_match_ts(
                                            &gen_sqlx_pg_types_pg_range_ts(&included_start_ts, &included_end_ts),
                                            &if_eq_unbounded_unbounded_or_included_excluded_ts,
                                            &sqlx_pg_types_pg_range_included_unbounded_ts,
                                            &gen_if_start_end_eq_ts(&sqlx_pg_types_pg_range_unbounded_unbounded_ts, &sqlx_pg_types_pg_range_excluded_included_ts),
                                            &gen_if_start_end_eq_ts(&sqlx_pg_types_pg_range_unbounded_unbounded_ts, &sqlx_pg_types_pg_range_excluded_excluded_ts),
                                            &gen_sqlx_pg_types_pg_range_ts(&excluded_start_ts, &unbounded_ucc),
                                            &gen_sqlx_pg_types_pg_range_ts(&unbounded_ucc, &included_end_ts),
                                            &sqlx_pg_types_pg_range_unbounded_excluded_ts,
                                        );
                                        match &range {
                                            Range::I32AsInt4 | Range::I64AsInt8 => int_range_normalize_ts,
                                            Range::SqlxTypesChronoNaiveDateAsDate => {
                                                let gen_dot_succ_opt_expect_ts = |id: &dyn std::fmt::Display| {
                                                    let id_dq_ts = gen_quotes::dq_ts(&id);
                                                    quote::quote! {.succ_opt().expect(#id_dq_ts)}
                                                };
                                                let gen_included_start_succ_opt_ts = |id: &dyn std::fmt::Display| {
                                                    let dot_succ_opt_expect_ts = gen_dot_succ_opt_expect_ts(&id);
                                                    quote::quote! {#included_ucc(#start_sc #dot_succ_opt_expect_ts)}
                                                };
                                                let gen_excluded_end_succ_opt_ts = |id: &dyn std::fmt::Display| {
                                                    let dot_succ_opt_expect_ts = gen_dot_succ_opt_expect_ts(&id);
                                                    quote::quote! {#excluded_ucc(#end_sc #dot_succ_opt_expect_ts)}
                                                };
                                                gen_range_match_ts(
                                                    &gen_sqlx_pg_types_pg_range_ts(&included_start_ts, &quote::quote! {#excluded_ucc(#end_sc.succ_opt().expect("9ebce3b4"))}),
                                                    &if_eq_unbounded_unbounded_or_included_excluded_ts,
                                                    &sqlx_pg_types_pg_range_included_unbounded_ts,
                                                    &gen_if_start_end_eq_ts(
                                                        &sqlx_pg_types_pg_range_unbounded_unbounded_ts,
                                                        &gen_sqlx_pg_types_pg_range_ts(&gen_included_start_succ_opt_ts(&"98a0357b-d21a-4949-a101-c641528d2376"), &gen_excluded_end_succ_opt_ts(&"fe53a6b9-2d7e-4605-9f5a-7f5c21cc01e6")),
                                                    ),
                                                    &gen_if_start_end_eq_ts(&sqlx_pg_types_pg_range_unbounded_unbounded_ts, &gen_sqlx_pg_types_pg_range_ts(&gen_included_start_succ_opt_ts(&"d8a26635-c478-4a2a-acf4-bf1765702889"), &excluded_end_ts)),
                                                    &gen_sqlx_pg_types_pg_range_ts(&gen_included_start_succ_opt_ts(&"9811c7c7-d7f5-4fb7-9d25-affb0bd4f5fb"), &unbounded_ucc),
                                                    &gen_sqlx_pg_types_pg_range_ts(&unbounded_ucc, &gen_excluded_end_succ_opt_ts(&"d6288f19-0a24-42ad-9e69-36036d9f2c1d")),
                                                    &sqlx_pg_types_pg_range_unbounded_excluded_ts,
                                                )
                                            }
                                            Range::SqlxTypesChronoNaiveDateTimeAsTimestamp | Range::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => range_match_timestamp_and_timestamp_tz_ts,
                                        }
                                    }
                                )
                            }
                            pg_crud_macros_cmn::IsNl::True => gen_ident_rd_ident_orgn_ts(&quote::quote! {
                                #v_sc.0.0.map(
                                    |v_4561270e|
                                    <
                                        #ident_stdrt_nn_ucc
                                        as
                                        #import::PgType
                                    >::normalize(
                                        #ident_stdrt_nn_rd_ucc(v_4561270e)
                                    ).0
                                )
                            }),
                        },
                    }
                },
                &if matches!(&is_nn_stdrt_can_be_pk, IsNnStdrtCanBePk::True) {
                    quote::quote! {#ident_rd_ids_ucc}
                } else {
                    quote::quote! {#import_non_pk_pg_type_rd_ids_ts}
                },
                &sel_only_ids_and_sel_only_updd_ids_query_cmn_ts,
                &ident_rd_inn_ucc,
                &{
                    let gen_ident_stdrt_nn_into_inn_ident_stdrt_nn_rd_ts = |ts: &dyn quote::ToTokens| {
                        quote::quote! {
                            #ident_stdrt_nn_as_pg_type_ts::into_inn(
                                #ident_stdrt_nn_rd_ucc(#ts)
                            )
                        }
                    };
                    let v_dot_zero_ts = quote::quote! {#v_sc.0};
                    let v_dot_zero_dot_zero_ts = quote::quote! {#v_dot_zero_ts.0};
                    match &pg_type_pattern {
                        PgTypePattern::Stdrt => match &is_nl {
                            pg_crud_macros_cmn::IsNl::False => {
                                if range_try_from_pg_type_is_ok {
                                    gen_pg_range_conversion_ts(&v_dot_zero_dot_zero_ts, &quote::quote! {v_af65ccce})
                                } else {
                                    v_dot_zero_dot_zero_ts
                                }
                            }
                            pg_crud_macros_cmn::IsNl::True => {
                                let ts = if range_try_from_pg_type_is_ok {
                                    gen_ident_stdrt_nn_into_inn_ident_stdrt_nn_rd_ts(&quote::quote! {v_bd169d3b})
                                } else {
                                    quote::quote! {v_bd169d3b.0}
                                };
                                quote::quote! {#v_dot_zero_dot_zero_ts.map(|v_bd169d3b| #ts)}
                            }
                        },
                    }
                },
                &ident_upd_ucc,
                &ident_upd_for_query_ucc,
                &pg_crud_macros_cmn::UpdQpValueUndrscr::True,
                &pg_crud_macros_cmn::UpdQpAccumulatorUndrscr::True,
                &pg_crud_macros_cmn::UpdQpTargetUndrscr::True,
                &pg_crud_macros_cmn::UpdQpPathUndrscr::True,
                &typical_qp_ts,
                &pg_crud_macros_cmn::IsUpdQbMut::True,
                &typical_qb_ts,
                &sel_only_ids_and_sel_only_updd_ids_query_cmn_ts,
                &pg_crud_macros_cmn::IsSelOnlyUpddIdsQbMut::False,
                &quote::quote! {Ok(#query_sc)},
            )
        };
        let impl_pg_type_test_cases_for_ident_ts = {
            enum IsNeedToUseInto {
                False,
                True,
            }
            let gen_rd_or_rd_inn_into_upd_with_new_or_try_new_unwraped_ts = |rd_or_upd: &pg_crud_macros_cmn::RdOrUpd| {
                let rd_or_upd_ucc = rd_or_upd.ucc();
                let ts = if pg_type_init_try_new_try_from_pg_type.is_ok() {
                    quote::quote! {#try_new_sc(#v_sc).expect("69477d2f")}
                } else {
                    quote::quote! {#new_sc(#v_sc)}
                };
                quote::quote! {<#self_ucc::#pg_type_ucc
                    as
                #import::#pg_type_ucc>::#rd_or_upd_ucc:: #ts}
            };
            let gen_stdrt_nn_test_case_h_ts = |is_need_to_use_into: &IsNeedToUseInto| {
                let gen_range_rd_ids_to_2_dims_vec_rd_inn_ts =
                    |min_ts: &dyn quote::ToTokens, negative_less_typical_ts: &dyn quote::ToTokens, negative_more_typical_ts: &dyn quote::ToTokens, near_zero_ts: &dyn quote::ToTokens, positive_less_typical_ts: &dyn quote::ToTokens, positive_more_typical_ts: &dyn quote::ToTokens, max_ts: &dyn quote::ToTokens| {
                        enum Bnd<'lt> {
                            Excl(&'lt dyn quote::ToTokens),
                            Incl(&'lt dyn quote::ToTokens),
                            Unb,
                        }
                        let test_cases_arr_ts = [
                            (Bnd::Incl(&min_sc),Bnd::Incl(&min_sc)),
                            (Bnd::Incl(&negative_less_typical_sc),Bnd::Incl(&negative_more_typical_sc)),
                            (Bnd::Incl(&near_zero_sc), Bnd::Incl(&near_zero_sc)),
                            (Bnd::Incl(&positive_less_typical_sc), Bnd::Incl(&positive_more_typical_sc)),
                            (Bnd::Incl(&max_sc), Bnd::Incl(&max_sc)),
                            (Bnd::Incl(&min_sc), Bnd::Incl(&max_sc)),
                            (Bnd::Incl(&min_sc), Bnd::Excl(&min_sc)),
                            (Bnd::Incl(&negative_less_typical_sc), Bnd::Excl(&negative_more_typical_sc)),
                            (Bnd::Incl(&near_zero_sc), Bnd::Excl(&near_zero_sc)),
                            (Bnd::Incl(&positive_less_typical_sc), Bnd::Excl(&positive_more_typical_sc)),
                            (Bnd::Incl(&max_sc), Bnd::Excl(&max_sc)),
                            (Bnd::Incl(&min_sc), Bnd::Excl(&max_sc)),
                            (Bnd::Incl(&min_sc), Bnd::Unb),
                            (Bnd::Incl(&negative_less_typical_sc), Bnd::Unb),
                            (Bnd::Incl(&near_zero_sc), Bnd::Unb),
                            (Bnd::Incl(&positive_less_typical_sc), Bnd::Unb),
                            (Bnd::Incl(&max_sc), Bnd::Unb),
                            (Bnd::Excl(&min_sc), Bnd::Incl(&min_sc)),
                            (Bnd::Excl(&negative_less_typical_sc), Bnd::Incl(&negative_more_typical_sc)),
                            (Bnd::Excl(&near_zero_sc), Bnd::Incl(&near_zero_sc)),
                            (Bnd::Excl(&positive_less_typical_sc), Bnd::Incl(&positive_more_typical_sc)),
                            (Bnd::Excl(&max_sc), Bnd::Incl(&max_sc)),
                            (Bnd::Excl(&min_sc), Bnd::Incl(&max_sc)),
                            (Bnd::Excl(&min_sc), Bnd::Excl(&min_sc)),
                            (Bnd::Excl(&negative_less_typical_sc), Bnd::Excl(&negative_more_typical_sc)),
                            (Bnd::Excl(&near_zero_sc), Bnd::Excl(&near_zero_sc)),
                            (Bnd::Excl(&positive_less_typical_sc), Bnd::Excl(&positive_more_typical_sc)),
                            (Bnd::Excl(&max_sc), Bnd::Excl(&max_sc)),
                            (Bnd::Excl(&min_sc), Bnd::Excl(&max_sc)),
                            (Bnd::Excl(&min_sc), Bnd::Unb),
                            (Bnd::Excl(&negative_less_typical_sc), Bnd::Unb),
                            (Bnd::Excl(&near_zero_sc), Bnd::Unb),
                            (Bnd::Excl(&positive_less_typical_sc), Bnd::Unb),
                            (Bnd::Excl(&max_sc), Bnd::Unb),
                            (Bnd::Unb, Bnd::Incl(&min_sc)),
                            (Bnd::Unb, Bnd::Incl(&negative_more_typical_sc)),
                            (Bnd::Unb, Bnd::Incl(&near_zero_sc)),
                            (Bnd::Unb, Bnd::Incl(&positive_more_typical_sc)),
                            (Bnd::Unb, Bnd::Incl(&max_sc)),
                            (Bnd::Unb, Bnd::Excl(&min_sc)),
                            (Bnd::Unb, Bnd::Excl(&negative_more_typical_sc)),
                            (Bnd::Unb, Bnd::Excl(&near_zero_sc)),
                            (Bnd::Unb, Bnd::Excl(&positive_more_typical_sc)),
                            (Bnd::Unb, Bnd::Excl(&max_sc)),
                            (Bnd::Unb, Bnd::Unb),
                        ]
                        .into_iter()
                        .map(|(start, end)|{
                            let (start_ts,end_ts) = {
                                let gen_bound_ts = |bnd: Bnd<'_>|{
                                    let ts = match bnd {
                                        Bnd::Excl(ts) => quote::quote! {Excluded(#ts)},
                                        Bnd::Incl(ts) => quote::quote! {Included(#ts)},
                                        Bnd::Unb => quote::quote! {Unbounded},
                                    };
                                    quote::quote!{std::ops::Bound::#ts}
                                };
                                (gen_bound_ts(start), gen_bound_ts(end))
                            };
                            quote::quote! {sqlx::postgres::types::PgRange { start: #start_ts, end: #end_ts}}
                        });
                        quote::quote! {{
                            let #min_sc = #min_ts;
                            let #max_sc = #max_ts;
                            let #negative_less_typical_sc = #negative_less_typical_ts;
                            let #negative_more_typical_sc = #negative_more_typical_ts;
                            let #near_zero_sc = #near_zero_ts;
                            let #positive_less_typical_sc = #positive_less_typical_ts;
                            let #positive_more_typical_sc = #positive_more_typical_ts;
                            vec![#(#test_cases_arr_ts),*]
                        }}
                    };
                let gen_int_pgrange_rd_ids_to_2_dims_vec_rd_inn_ts = |int_range_type: &IntRangeType| {
                    let range_inn_type_ts = int_range_type_to_range_inn_type_ts(int_range_type);
                    gen_range_rd_ids_to_2_dims_vec_rd_inn_ts(&quote::quote! {#range_inn_type_ts::MIN}, &quote::quote! {-20}, &quote::quote! {-10}, &quote::quote! {0}, &quote::quote! {10}, &quote::quote! {20}, &quote::quote! {#range_inn_type_ts::MAX - 1})
                };
                let empty_vec_ts = quote::quote! {Vec::new()};
                let gen_ident_stdrt_nn_fn_ts = |
                    ident_prm: &dyn quote::ToTokens,
                    ts: &dyn quote::ToTokens
                |quote::quote! {#ident_prm::#ts()};
                let (
                    ident_sqlx_types_chrono_naive_time_min_ts,
                    ident_sqlx_types_chrono_naive_time_ten_ts,
                    ident_sqlx_types_chrono_naive_time_twenty_ts,
                    ident_sqlx_types_chrono_naive_time_max_ts
                ) = {
                    let gen_ts = |
                        ts_prm: &dyn quote::ToTokens
                    |gen_ident_stdrt_nn_fn_ts(
                        &gen_ident_stdrt_nn_ts(&PgType::SqlxTypesChronoNaiveTimeAsTime),
                        &ts_prm
                    );
                    (
                        gen_ts(
                            &sqlx_types_chrono_naive_time_min_fn_ts
                        ),
                        gen_ts(
                            &sqlx_types_chrono_naive_time_ten_fn_ts
                        ),
                        gen_ts(
                            &sqlx_types_chrono_naive_time_twenty_fn_ts
                        ),
                        gen_ts(
                            &sqlx_types_chrono_naive_time_max_fn_ts
                        )
                    )
                };
                let (
                    ident_sqlx_types_chrono_naive_date_min_ts,
                    ident_sqlx_types_chrono_naive_date_negative_less_typical_ts,
                    ident_sqlx_types_chrono_naive_date_negative_more_typical_ts,
                    ident_sqlx_types_chrono_naive_date_near_zero_ts,
                    ident_sqlx_types_chrono_naive_date_positive_less_typical_ts,
                    ident_sqlx_types_chrono_naive_date_positive_more_typical_ts,
                    ident_sqlx_types_chrono_naive_date_max_ts,
                    ident_sqlx_types_chrono_naive_date_max_pred_opt_expect_ts,
                ) = {
                    let gen_ts = |
                        ts_prm: &dyn quote::ToTokens
                    |gen_ident_stdrt_nn_fn_ts(
                        &gen_ident_stdrt_nn_ts(&PgType::SqlxTypesChronoNaiveDateAsDate),
                        &ts_prm
                    );
                    (
                        gen_ts(
                            &sqlx_types_chrono_naive_date_min_fn_ts,
                        ),
                        gen_ts(
                            &sqlx_types_chrono_naive_date_negative_less_typical_fn_ts,
                        ),
                        gen_ts(
                            &sqlx_types_chrono_naive_date_negative_more_typical_fn_ts,
                        ),
                        gen_ts(
                            &sqlx_types_chrono_naive_date_near_zero_fn_ts,
                        ),
                        gen_ts(
                            &sqlx_types_chrono_naive_date_positive_less_typical_fn_ts,
                        ),
                        gen_ts(
                            &sqlx_types_chrono_naive_date_positive_more_typical_fn_ts,
                        ),
                        gen_ts(
                            &sqlx_types_chrono_naive_date_max_fn_ts,
                        ),
                        gen_ts(
                            &sqlx_types_chrono_naive_date_max_pred_opt_expect_fn_ts,
                        ),
                    )
                };
                let (
                    sqlx_types_chrono_naive_date_time_min_ts,
                    sqlx_types_chrono_naive_date_time_negative_less_typical_ts,
                    sqlx_types_chrono_naive_date_time_negative_more_typical_ts,
                    sqlx_types_chrono_naive_date_time_near_zero_ts,
                    sqlx_types_chrono_naive_date_time_positive_less_typical_ts,
                    sqlx_types_chrono_naive_date_time_positive_more_typical_ts,
                    sqlx_types_chrono_naive_date_time_max_ts,
                ) = {
                    let gen_ts = |date: &dyn quote::ToTokens, time: &dyn quote::ToTokens| {
                        gen_sqlx_types_chrono_naive_date_time_new_ts(&quote::quote! { #date, #time })
                    };
                    (
                        gen_ts(&ident_sqlx_types_chrono_naive_date_min_ts, &ident_sqlx_types_chrono_naive_time_min_ts),
                        gen_ts(&ident_sqlx_types_chrono_naive_date_negative_less_typical_ts, &ident_sqlx_types_chrono_naive_time_twenty_ts),
                        gen_ts(&ident_sqlx_types_chrono_naive_date_negative_more_typical_ts, &ident_sqlx_types_chrono_naive_time_ten_ts),
                        gen_ts(&ident_sqlx_types_chrono_naive_date_near_zero_ts, &ident_sqlx_types_chrono_naive_time_min_ts),
                        gen_ts(&ident_sqlx_types_chrono_naive_date_positive_less_typical_ts, &ident_sqlx_types_chrono_naive_time_ten_ts),
                        gen_ts(&ident_sqlx_types_chrono_naive_date_positive_more_typical_ts, &ident_sqlx_types_chrono_naive_time_twenty_ts),
                        gen_ts(&ident_sqlx_types_chrono_naive_date_max_ts, &ident_sqlx_types_chrono_naive_time_max_ts),
                    )
                };
                let (
                    sqlx_types_chrono_date_time_sqlx_types_chrono_utc_min_ts,
                    sqlx_types_chrono_date_time_sqlx_types_chrono_utc_negative_less_typical_ts,
                    sqlx_types_chrono_date_time_sqlx_types_chrono_utc_negative_more_typical_ts,
                    sqlx_types_chrono_date_time_sqlx_types_chrono_utc_near_zero_ts,
                    sqlx_types_chrono_date_time_sqlx_types_chrono_utc_positive_less_typical_ts,
                    sqlx_types_chrono_date_time_sqlx_types_chrono_utc_positive_more_typical_ts,
                    sqlx_types_chrono_date_time_sqlx_types_chrono_utc_max_ts,
                ) = {
                    let gen_ts = |ts: &dyn quote::ToTokens| gen_sqlx_types_chrono_date_time_sqlx_types_chrono_utc_from_naive_utc_and_offset_ts(ts);
                    (
                        gen_ts(&sqlx_types_chrono_naive_date_time_min_ts),
                        gen_ts(&sqlx_types_chrono_naive_date_time_negative_less_typical_ts),
                        gen_ts(&sqlx_types_chrono_naive_date_time_negative_more_typical_ts),
                        gen_ts(&sqlx_types_chrono_naive_date_time_near_zero_ts),
                        gen_ts(&sqlx_types_chrono_naive_date_time_positive_less_typical_ts),
                        gen_ts(&sqlx_types_chrono_naive_date_time_positive_more_typical_ts),
                        gen_ts(&sqlx_types_chrono_naive_date_time_max_ts),
                    )
                };
                let gen_typical_test_cases_vec_ts = |ts: &dyn quote::ToTokens| {
                    let ts0 = match &is_need_to_use_into {
                        IsNeedToUseInto::True => quote::quote! {.into()},
                        IsNeedToUseInto::False => proc_macro2::TokenStream::new(),
                    };
                    quote::quote! {#import::#ts()#ts0}
                };
                let gen_ts = |ts: &dyn quote::ToTokens| gen_ident_stdrt_nn_fn_ts(&self_ucc, &ts);
                match &pg_type {
                    PgType::I16AsInt2 => gen_typical_test_cases_vec_ts(&quote::quote! {i16_test_cases_vec}),
                    PgType::I32AsInt4 => gen_typical_test_cases_vec_ts(&quote::quote! {i32_test_cases_vec}),
                    PgType::I64AsInt8 => gen_typical_test_cases_vec_ts(&quote::quote! {i64_test_cases_vec}),
                    PgType::F32AsFloat4 => gen_typical_test_cases_vec_ts(&quote::quote! {f32_test_cases_vec}),
                    PgType::F64AsFloat8 => gen_typical_test_cases_vec_ts(&quote::quote! {f64_test_cases_vec}),
                    PgType::I16AsSmallSerialInitByPg | PgType::I32AsSerialInitByPg | PgType::I64AsBigSerialInitByPg => empty_vec_ts,
                    PgType::SqlxPgTypesPgMoneyAsMoney => quote::quote! {
                        #import::i64_test_cases_vec().into_iter().map(
                            #inn_type_stdrt_nn_ts
                        ).collect::<Vec<#inn_type_stdrt_nn_ts>>()
                    },
                    PgType::BoolAsBool => gen_typical_test_cases_vec_ts(&quote::quote! {bool_test_cases_vec}),
                    PgType::StringAsText => gen_typical_test_cases_vec_ts(&quote::quote! {string_test_cases_vec}),
                    PgType::StdVecVecU8AsBytea => quote::quote! {vec![
                        Vec::new(),
                        (0u8..=255).collect(),
                        vec![0; 1024],
                        vec![0; 1024 * 1024 * 2],
                    ]},
                    PgType::SqlxTypesChronoNaiveTimeAsTime => {
                        let self_sqlx_types_chrono_naive_time_min_ts = gen_ts(&sqlx_types_chrono_naive_time_min_fn_ts);
                        let self_sqlx_types_chrono_naive_time_ten_ts = gen_ts(&sqlx_types_chrono_naive_time_ten_fn_ts);
                        let self_sqlx_types_chrono_naive_time_twenty_ts = gen_ts(&sqlx_types_chrono_naive_time_twenty_fn_ts);
                        let self_sqlx_types_chrono_naive_time_max_ts = gen_ts(&sqlx_types_chrono_naive_time_max_fn_ts);
                        quote::quote! {vec![
                            #self_sqlx_types_chrono_naive_time_min_ts,
                            #self_sqlx_types_chrono_naive_time_ten_ts,
                            #self_sqlx_types_chrono_naive_time_twenty_ts,
                            #self_sqlx_types_chrono_naive_time_max_ts,
                        ]}
                    },
                    PgType::SqlxTypesTimeTimeAsTime => {
                        let sqlx_types_time_time_from_hms_micro_min_unwrap_ts = gen_sqlx_types_time_time_from_hms_micro_unwrap_ts(&quote::quote! {0,0,0,0});
                        let sqlx_types_time_time_from_hms_micro_ten_unwrap_ts = gen_sqlx_types_time_time_from_hms_micro_unwrap_ts(&quote::quote! {10,10,10,10});
                        let sqlx_types_time_time_from_hms_micro_twenty_unwrap_ts = gen_sqlx_types_time_time_from_hms_micro_unwrap_ts(&quote::quote! {20,20,20,20});
                        let sqlx_types_time_time_from_hms_micro_max_unwrap_ts = gen_sqlx_types_time_time_from_hms_micro_unwrap_ts(&quote::quote! {23,59,59,999_999});
                        quote::quote! {vec![
                            #sqlx_types_time_time_from_hms_micro_min_unwrap_ts,
                            #sqlx_types_time_time_from_hms_micro_ten_unwrap_ts,
                            #sqlx_types_time_time_from_hms_micro_twenty_unwrap_ts,
                            #sqlx_types_time_time_from_hms_micro_max_unwrap_ts,
                        ]}
                    }
                    PgType::SqlxPgTypesPgIntervalAsInterval => {
                        let min_ts = quote::quote! {MIN};
                        let max_ts = quote::quote! {MAX};
                        let i32_min_ts = quote::quote! {#i32_ts::#min_ts};
                        let i32_max_ts = quote::quote! {#i32_ts::#max_ts};
                        let gen_sqlx_pg_types_pg_interval_ts = |months_ts: &dyn quote::ToTokens, days_ts: &dyn quote::ToTokens, microseconds_ts: &dyn quote::ToTokens| {
                            quote::quote! {sqlx::postgres::types::PgInterval {
                                months: #months_ts,
                                days: #days_ts,
                                microseconds: #microseconds_ts
                            }}
                        };
                        let interval_min_ts = gen_sqlx_pg_types_pg_interval_ts(&i32_min_ts, &i32_min_ts, &quote::quote! {#i64_ts::#min_ts});
                        let interval_max_ts = gen_sqlx_pg_types_pg_interval_ts(&i32_max_ts, &i32_max_ts, &quote::quote! {#i64_ts::#max_ts});
                        quote::quote! {vec![
                            #interval_min_ts,
                            #interval_max_ts
                        ]}
                    }
                    PgType::SqlxTypesChronoNaiveDateAsDate => {
                        let sqlx_types_chrono_naive_date_min_ts = gen_ts(&sqlx_types_chrono_naive_date_min_fn_ts);
                        let sqlx_types_chrono_naive_date_negative_less_typical_ts = gen_ts(&sqlx_types_chrono_naive_date_negative_less_typical_fn_ts);
                        let sqlx_types_chrono_naive_date_negative_more_typical_ts = gen_ts(&sqlx_types_chrono_naive_date_negative_more_typical_fn_ts);
                        let sqlx_types_chrono_naive_date_near_zero_ts = gen_ts(&sqlx_types_chrono_naive_date_near_zero_fn_ts);
                        let sqlx_types_chrono_naive_date_positive_less_typical_ts = gen_ts(&sqlx_types_chrono_naive_date_positive_less_typical_fn_ts);
                        let sqlx_types_chrono_naive_date_positive_more_typical_ts = gen_ts(&sqlx_types_chrono_naive_date_positive_more_typical_fn_ts);
                        let sqlx_types_chrono_naive_date_max_ts = gen_ts(&sqlx_types_chrono_naive_date_max_fn_ts);
                        quote::quote! {vec![
                            #sqlx_types_chrono_naive_date_min_ts,
                            #sqlx_types_chrono_naive_date_negative_less_typical_ts,
                            #sqlx_types_chrono_naive_date_negative_more_typical_ts,
                            #sqlx_types_chrono_naive_date_near_zero_ts,
                            #sqlx_types_chrono_naive_date_positive_less_typical_ts,
                            #sqlx_types_chrono_naive_date_positive_more_typical_ts,
                            #sqlx_types_chrono_naive_date_max_ts,
                        ]}
                    },
                    PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp => quote::quote! {vec![
                        #sqlx_types_chrono_naive_date_time_min_ts,
                        #sqlx_types_chrono_naive_date_time_negative_less_typical_ts,
                        #sqlx_types_chrono_naive_date_time_negative_more_typical_ts,
                        #sqlx_types_chrono_naive_date_time_near_zero_ts,
                        #sqlx_types_chrono_naive_date_time_positive_less_typical_ts,
                        #sqlx_types_chrono_naive_date_time_positive_more_typical_ts,
                        #sqlx_types_chrono_naive_date_time_max_ts,
                    ]},
                    PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => quote::quote! {vec![
                        #sqlx_types_chrono_date_time_sqlx_types_chrono_utc_min_ts,
                        #sqlx_types_chrono_date_time_sqlx_types_chrono_utc_negative_less_typical_ts,
                        #sqlx_types_chrono_date_time_sqlx_types_chrono_utc_negative_more_typical_ts,
                        #sqlx_types_chrono_date_time_sqlx_types_chrono_utc_near_zero_ts,
                        #sqlx_types_chrono_date_time_sqlx_types_chrono_utc_positive_less_typical_ts,
                        #sqlx_types_chrono_date_time_sqlx_types_chrono_utc_positive_more_typical_ts,
                        #sqlx_types_chrono_date_time_sqlx_types_chrono_utc_max_ts,
                    ]},
                    PgType::SqlxTypesUuidUuidAsUuidV4InitByPg => quote::quote! {Vec::new()},
                    PgType::SqlxTypesUuidUuidAsUuidInitByClient => quote::quote! {vec![
                        sqlx::types::Uuid::new_v4()
                    ]},
                    PgType::SqlxTypesIpnetworkIpNetworkAsInet => quote::quote! {vec![
                        <sqlx::types::ipnetwork::IpNetwork as std::str::FromStr>::from_str("192.168.0.0/24").expect("478dbded"),
                        <sqlx::types::ipnetwork::IpNetwork as std::str::FromStr>::from_str("10.0.0.0/8").expect("8af9e27e"),
                        <sqlx::types::ipnetwork::IpNetwork as std::str::FromStr>::from_str("172.16.0.0/12").expect("ba86505f"),
                        <sqlx::types::ipnetwork::IpNetwork as std::str::FromStr>::from_str("127.0.0.1/32").expect("32c744a0"),
                        <sqlx::types::ipnetwork::IpNetwork as std::str::FromStr>::from_str("::1/128").expect("560815f8"),
                        <sqlx::types::ipnetwork::IpNetwork as std::str::FromStr>::from_str("2001:db8::/32").expect("793db0ef"),
                        sqlx::types::ipnetwork::IpNetwork::V4(sqlx::types::ipnetwork::Ipv4Network::#new_sc(std::net::Ipv4Addr::#new_sc(192, 168, 0, 0), 24).expect("c44934f2")),
                        sqlx::types::ipnetwork::IpNetwork::V4(sqlx::types::ipnetwork::Ipv4Network::#new_sc(std::net::Ipv4Addr::#new_sc(10, 0, 0, 0), 8).expect("39e588d9")),
                        sqlx::types::ipnetwork::IpNetwork::V4(sqlx::types::ipnetwork::Ipv4Network::#new_sc(std::net::Ipv4Addr::LOCALHOST, 32).expect("43fb25bd")),
                        sqlx::types::ipnetwork::IpNetwork::V6(sqlx::types::ipnetwork::Ipv6Network::#new_sc(std::net::Ipv6Addr::LOCALHOST, 128).expect("b443be46")),
                        sqlx::types::ipnetwork::IpNetwork::V6(sqlx::types::ipnetwork::Ipv6Network::#new_sc("2001:db8::".parse().expect("d4e6df27"), 32).expect("a7486c5e")),
                    ]},
                    PgType::SqlxTypesMacAddressMacAddressAsMacAddr => quote::quote! {vec![
                        sqlx::types::mac_address::MacAddress::#new_sc([0x00, 0x00, 0x00, 0x00, 0x00, 0x00]), // All zeros
                        sqlx::types::mac_address::MacAddress::#new_sc([0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]), // All ones (broadcast address)
                        sqlx::types::mac_address::MacAddress::#new_sc([0x02, 0x00, 0x00, 0x00, 0x00, 0x01]), // Locally administered address
                        sqlx::types::mac_address::MacAddress::#new_sc([0x00, 0x1A, 0x2B, 0x3C, 0x4D, 0x5E]), // Universally administered address
                        sqlx::types::mac_address::MacAddress::#new_sc([0x01, 0x00, 0x5E, 0x00, 0x00, 0xFB]), // Multicast address
                        sqlx::types::mac_address::MacAddress::#new_sc([0xDE, 0xAD, 0xBE, 0xEF, 0xCA, 0xFE]), // Random valid MAC
                    ]},
                    PgType::SqlxPgTypesPgRangeI32AsInt4Range => gen_int_pgrange_rd_ids_to_2_dims_vec_rd_inn_ts(&IntRangeType::SqlxPgTypesPgRangeI32AsInt4Range),
                    PgType::SqlxPgTypesPgRangeI64AsInt8Range => gen_int_pgrange_rd_ids_to_2_dims_vec_rd_inn_ts(&IntRangeType::SqlxPgTypesPgRangeI64AsInt8Range),
                    PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => gen_range_rd_ids_to_2_dims_vec_rd_inn_ts(
                        &ident_sqlx_types_chrono_naive_date_min_ts,
                        &ident_sqlx_types_chrono_naive_date_negative_less_typical_ts,
                        &ident_sqlx_types_chrono_naive_date_negative_more_typical_ts,
                        &ident_sqlx_types_chrono_naive_date_near_zero_ts,
                        &ident_sqlx_types_chrono_naive_date_positive_less_typical_ts,
                        &ident_sqlx_types_chrono_naive_date_positive_more_typical_ts,
                        &ident_sqlx_types_chrono_naive_date_max_pred_opt_expect_ts
                    ),
                    PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => gen_range_rd_ids_to_2_dims_vec_rd_inn_ts(
                        &sqlx_types_chrono_naive_date_time_min_ts,
                        &sqlx_types_chrono_naive_date_time_negative_less_typical_ts,
                        &sqlx_types_chrono_naive_date_time_negative_more_typical_ts,
                        &sqlx_types_chrono_naive_date_time_near_zero_ts,
                        &sqlx_types_chrono_naive_date_time_positive_less_typical_ts,
                        &sqlx_types_chrono_naive_date_time_positive_more_typical_ts,
                        &sqlx_types_chrono_naive_date_time_max_ts,
                    ),
                    PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => gen_range_rd_ids_to_2_dims_vec_rd_inn_ts(
                        &sqlx_types_chrono_date_time_sqlx_types_chrono_utc_min_ts,
                        &sqlx_types_chrono_date_time_sqlx_types_chrono_utc_negative_less_typical_ts,
                        &sqlx_types_chrono_date_time_sqlx_types_chrono_utc_negative_more_typical_ts,
                        &sqlx_types_chrono_date_time_sqlx_types_chrono_utc_near_zero_ts,
                        &sqlx_types_chrono_date_time_sqlx_types_chrono_utc_positive_less_typical_ts,
                        &sqlx_types_chrono_date_time_sqlx_types_chrono_utc_positive_more_typical_ts,
                        &sqlx_types_chrono_date_time_sqlx_types_chrono_utc_max_ts,
                    ),
                }
            };
            let opt_vec_cr_ts: Option<proc_macro2::TokenStream> = {
                let gen_some_acc_ts = |
                    is_nl_prm: &pg_crud_macros_cmn::IsNl,
                    ident_ts_prm: &dyn quote::ToTokens,
                    additonal_ts: &dyn quote::ToTokens
                | {
                    let (new_or_try_new_ts, mb_acc_push_none_ts) = match (&is_nl_prm, pg_type_init_try_new_try_from_pg_type.is_ok()) {
                        (pg_crud_macros_cmn::IsNl::False, true) => (quote::quote! {try_new(vec![el_0fd5865b.0.into()]).expect("adbae6b3")}, proc_macro2::TokenStream::new()),
                        (pg_crud_macros_cmn::IsNl::False, false) => (quote::quote! {new(vec![el_0fd5865b.0.into()])}, proc_macro2::TokenStream::new()),
                        (pg_crud_macros_cmn::IsNl::True, true) => (
                            quote::quote! {try_new(Some(el_0fd5865b.0.into())).expect("b244d498")},
                            quote::quote! {acc_0b59a062.push(#self_as_pg_type_ts::Cr::try_new(None).expect("31878971"));},
                        ),
                        (pg_crud_macros_cmn::IsNl::True, false) => (quote::quote! {new(Some(el_0fd5865b.0.into()))}, quote::quote! {acc_0b59a062.push(#self_as_pg_type_ts::Cr::new(None));}),
                    };
                    let ident_as_pg_type_test_cases_ts = gen_as_pg_type_test_cases_ts(&ident_ts_prm);
                    quote::quote! {Some({
                        let mut acc_0b59a062 = Vec::new();
                        for el_0fd5865b in #ident_as_pg_type_test_cases_ts::#opt_vec_cr_sc().unwrap_or(Vec::new()) {
                            acc_0b59a062.push(#self_as_pg_type_ts::Cr::#new_or_try_new_ts);
                        }
                        #mb_acc_push_none_ts
                        #additonal_ts
                        acc_0b59a062
                    })}
                };
                match &pg_type_pattern {
                    PgTypePattern::Stdrt => match &is_nl {
                        pg_crud_macros_cmn::IsNl::False => match &can_be_pk {
                            CanBePk::False => Some({
                                let ts = gen_stdrt_nn_test_case_h_ts(&IsNeedToUseInto::False);
                                let new_or_try_new_ts = {
                                    let self_as_pg_type_cr_ts = quote::quote! {#self_as_pg_type_ts::Cr};
                                    if pg_type_init_try_new_try_from_pg_type.is_ok() {
                                        quote::quote! {
                                            |el_043a7d30|#self_as_pg_type_cr_ts::try_new(
                                                el_043a7d30
                                            ).expect("941bd15c")
                                        }
                                    } else {
                                        quote::quote! {#self_as_pg_type_cr_ts::#new_sc}
                                    }
                                };
                                quote::quote! {Some(
                                    #ts.into_iter().map(
                                        #new_or_try_new_ts
                                    ).collect()
                                )}
                            }),
                            CanBePk::True => None,
                        },
                        pg_crud_macros_cmn::IsNl::True => Some(gen_some_acc_ts(is_nl, &gen_ident_ts(pg_type, &pg_crud_macros_cmn::IsNl::False, &PgTypePattern::Stdrt), &proc_macro2::TokenStream::new())),
                    },
                }
            };
            let rd_ids_to_2_dims_vec_rd_inn_ts = {
                match &is_nl {
                    pg_crud_macros_cmn::IsNl::False => {
                        let ts = gen_stdrt_nn_test_case_h_ts(&IsNeedToUseInto::True);
                        quote::quote! {vec![{#ts}]}
                    }
                    pg_crud_macros_cmn::IsNl::True => quote::quote! {
                        #ident_stdrt_nn_as_pg_type_test_cases_ts::#rd_ids_to_2_dims_vec_rd_inn_sc(#rd_ids_sc)
                        .into_iter()
                        .flat_map(|el0| el0.into_iter().map(|el1| vec![Some(el1)]))
                        .chain(std::iter::once(vec![None]))
                        .collect()
                    },
                }
            };
            let rd_inn_into_rd_with_new_or_try_new_unwraped_ts = gen_rd_or_rd_inn_into_upd_with_new_or_try_new_unwraped_ts(&pg_crud_macros_cmn::RdOrUpd::Rd);
            let rd_inn_into_upd_with_new_or_try_new_unwraped_ts = gen_rd_or_rd_inn_into_upd_with_new_or_try_new_unwraped_ts(&pg_crud_macros_cmn::RdOrUpd::Upd);
            let upd_to_rd_ids_ts = if matches!(&is_nn_stdrt_can_be_pk, IsNnStdrtCanBePk::True) {
                quote::quote! {
                    #ident_rd_ids_ucc(#ident_rd_ucc(#v_sc.0 #mb_dot_clone_ts))//todo its not correct. must be only for pk but it for all types what van be pk
                }
            } else {
                let ts = gen_v_init_ts0(&none_ts);
                quote::quote! {
                    #import_non_pk_pg_type_rd_ids_ts::from(#ts)
                }
            };
            let rd_ids_to_opt_v_rd_dflt_some_one_el_ts = {
                //todo that is not correct for arr of generated by pg pks but mb just need to remove this vrts and thats it?
                let ts = gen_v_init_ts0(&{
                    let ts: &dyn quote::ToTokens = if matches!(&is_nn_stdrt_can_be_pk, IsNnStdrtCanBePk::True) {
                        &quote::quote! {#v_sc.0 #mb_dot_clone_ts}
                    } else {
                        &pg_crud_cmn_dflt_some_one_el_call
                    };
                    quote::quote! {#self_pg_type_as_pg_type_ts::normalize(#ts)}
                });
                quote::quote! {Some(#ts)}
            };
            let previous_rd_and_opt_upd_into_rd_ts = quote::quote! {
                #opt_upd_sc.map_or(#rd_sc, |#v_sc| #ident_rd_ucc(#v_sc.0))
            };
            let rd_ids_and_cr_into_rd_ts = {
                let ts = if matches!(&is_nn_stdrt_can_be_pk, IsNnStdrtCanBePk::True) {
                    quote::quote! {#rd_ids_sc.0}
                } else {
                    quote::quote! {#ident_rd_ucc(#cr_sc.0)}
                };
                quote::quote! {
                    #self_pg_type_as_pg_type_ts::normalize(#ts)
                }
            };
            let rd_ids_and_cr_into_opt_v_rd_ts = {
                let ts = gen_v_init_ts0(&quote::quote! {
                    <Self as #import::PgTypeTestCases>::#rd_ids_and_cr_into_rd_sc(
                        #rd_ids_sc,
                        #cr_sc
                    )
                });
                quote::quote! {Some(#ts)}
            };
            let rd_ids_and_cr_into_tt_ts = {
                let ts = if matches!(&is_nn_stdrt_can_be_pk, IsNnStdrtCanBePk::True) {
                    quote::quote! {#rd_ids_sc.0.0}
                } else {
                    quote::quote! {#cr_sc.0}
                };
                quote::quote! {#ident_tt_ucc(#ts)}
            };
            //todo mb it into fn (not in proc macro)
            let rd_ids_and_cr_into_wh_eq_ts = {
                let ts = if matches!(&pg_type_pattern, PgTypePattern::Stdrt)
                    && matches!(&is_nl, pg_crud_macros_cmn::IsNl::False)
                    && matches!(&is_nn_stdrt_can_be_pk, IsNnStdrtCanBePk::True)
                {
                    quote::quote! {#rd_ids_sc.0.0}
                } else {
                    quote::quote! {#cr_sc.0}
                };
                quote::quote! {
                    #ident_wh_ucc::#eq_ucc(wh_flts::PgTypeWhEq {
                        oprtr: #import::Oprtr::Or,
                        #v_sc: #ident_tt_ucc(#ts),
                    })
                }
            };
            let rd_ids_and_cr_into_vec_wh_eq_using_fields_ts = quote::quote! {
                #import::NotEmptyUnqVec::try_new(vec![
                    #rd_ids_and_cr_into_wh_eq_ts
                ]).expect("4c08b551")
            };
            let rd_ids_and_cr_into_opt_vec_wh_eq_to_field_ts: Option<proc_macro2::TokenStream> = None;
            let pg_type_opt_vec_wh_greater_than_test_ts: Option<proc_macro2::TokenStream> = {
                let greater_than = pg_crud_cmn::PgTypeGreaterThanVrt::GreaterThan;
                let not_greater_than = pg_crud_cmn::PgTypeGreaterThanVrt::NotGreaterThan;
                let eq_not_greater_than = pg_crud_cmn::PgTypeGreaterThanVrt::EqNotGreaterThan;
                let gen_greater_than_test_ts = |greater_than_vrt_ts: &pg_crud_cmn::PgTypeGreaterThanVrt, cr_ts: &dyn quote::ToTokens, tt_ts: &dyn quote::ToTokens| {
                    quote::quote! {
                        #import::PgTypeGreaterThanTest {
                            vrt: #import::PgTypeGreaterThanVrt::#greater_than_vrt_ts,
                            cr: #self_as_pg_type_ts::Cr::#cr_ts,
                            greater_than: #self_as_pg_type_ts::Tt::#tt_ts,
                        }
                    }
                };
                let gen_greater_than_test_new_new_ts =
                    |greater_than_vrt_ts: &pg_crud_cmn::PgTypeGreaterThanVrt, cr_ts: &dyn quote::ToTokens, greater_than_ts: &dyn quote::ToTokens| gen_greater_than_test_ts(greater_than_vrt_ts, &quote::quote! {new(#cr_ts)}, &quote::quote! {new(#greater_than_ts)});
                let gen_greater_than_test_try_new_try_new_ts = |greater_than_vrt_ts: &pg_crud_cmn::PgTypeGreaterThanVrt, cr_ts: &dyn quote::ToTokens, greater_than_ts: &dyn quote::ToTokens| {
                    gen_greater_than_test_ts(
                        greater_than_vrt_ts,
                        &quote::quote! {try_new(#cr_ts).expect("8327c651")},
                        &quote::quote! {try_new(#greater_than_ts).expect("c369e6ea")},
                    )
                };
                let gen_greater_than_test_vec_ts = |
                    gen_ts: &dyn Fn(&pg_crud_cmn::PgTypeGreaterThanVrt, &dyn quote::ToTokens, &dyn quote::ToTokens) -> proc_macro2::TokenStream,
                    less_ts: &dyn quote::ToTokens,
                    less_with_more_ts: &dyn quote::ToTokens,
                    zero_ts: &dyn quote::ToTokens,
                    one_ts: &dyn quote::ToTokens,
                    more_ts: &dyn quote::ToTokens,
                    more_with_less_ts: &dyn quote::ToTokens
                | {
                    let greater_than_less_ts = gen_ts(&greater_than, &less_with_more_ts, &less_ts);
                    let greater_than_zero_ts = gen_ts(&greater_than, &one_ts, &zero_ts);
                    let greater_than_more_ts = gen_ts(&greater_than, &more_ts, &more_with_less_ts);
                    let not_greater_than_less_ts = gen_ts(&not_greater_than, &less_ts, &less_with_more_ts);
                    let not_greater_than_zero_ts = gen_ts(&not_greater_than, &zero_ts, &one_ts);
                    let not_greater_than_more_ts = gen_ts(&not_greater_than, &more_with_less_ts, &more_ts);
                    let eq_not_greater_than_less_ts = gen_ts(&eq_not_greater_than, &less_ts, &less_ts);
                    let eq_not_greater_than_zero_ts = gen_ts(&eq_not_greater_than, &zero_ts, &zero_ts);
                    let eq_not_greater_than_more_ts = gen_ts(&eq_not_greater_than, &more_ts, &more_ts);
                    quote::quote! {
                        #greater_than_less_ts,
                        #greater_than_zero_ts,
                        #greater_than_more_ts,
                        #not_greater_than_less_ts,
                        #not_greater_than_zero_ts,
                        #not_greater_than_more_ts,
                        #eq_not_greater_than_less_ts,
                        #eq_not_greater_than_zero_ts,
                        #eq_not_greater_than_more_ts
                    }
                };
                let gen_greater_than_test_new_new_vec_ts = |
                    less_ts: &dyn quote::ToTokens,
                    less_with_more_ts: &dyn quote::ToTokens,
                    zero_ts: &dyn quote::ToTokens,
                    one_ts: &dyn quote::ToTokens,
                    more_ts: &dyn quote::ToTokens,
                    more_with_less_ts: &dyn quote::ToTokens
                | gen_greater_than_test_vec_ts(&gen_greater_than_test_new_new_ts, less_ts, less_with_more_ts, zero_ts, one_ts, more_ts, more_with_less_ts);
                let gen_greater_than_test_try_new_try_new_vec_ts = |
                    less_ts: &dyn quote::ToTokens,
                    less_with_more_ts: &dyn quote::ToTokens,
                    zero_ts: &dyn quote::ToTokens,
                    one_ts: &dyn quote::ToTokens,
                    more_ts: &dyn quote::ToTokens,
                    more_with_less_ts: &dyn quote::ToTokens
                | gen_greater_than_test_vec_ts(&gen_greater_than_test_try_new_try_new_ts, less_ts, less_with_more_ts, zero_ts, one_ts, more_ts, more_with_less_ts);
                match &pg_type_pattern {
                    PgTypePattern::Stdrt => match &is_nl {
                        pg_crud_macros_cmn::IsNl::False => {
                            let wrap_into_not_empty_unq_vec_ts = |ts: &dyn quote::ToTokens| Some(quote::quote! {Some(
                                #import::NotEmptyUnqVec::try_new(vec![#ts]).expect("3ad4b6bf")
                            )});
                            let sqlx_types_chrono_naive_time_as_time_stdrt_nn_ts = &gen_ident_ts(
                                &PgType::SqlxTypesChronoNaiveTimeAsTime,
                                &pg_crud_macros_cmn::IsNl::False,
                                &PgTypePattern::Stdrt
                            );
                            let sqlx_types_chrono_naive_date_as_date_stdrt_nn_ts = &gen_ident_ts(
                                &PgType::SqlxTypesChronoNaiveDateAsDate,
                                &pg_crud_macros_cmn::IsNl::False,
                                &PgTypePattern::Stdrt
                            );
                            match &pg_type {
                                PgType::I16AsInt2 => wrap_into_not_empty_unq_vec_ts(&gen_greater_than_test_new_new_vec_ts(
                                    &quote::quote! {#i16_ts::MIN},
                                    &quote::quote! {#i16_ts::MIN + 1},
                                    &quote::quote! {0},
                                    &quote::quote! {1},
                                    &quote::quote! {#i16_ts::MAX},
                                    &quote::quote! {#i16_ts::MAX - 1}
                                )),
                                PgType::I32AsInt4 => wrap_into_not_empty_unq_vec_ts(&gen_greater_than_test_new_new_vec_ts(
                                    &quote::quote! {#i32_ts::MIN},
                                    &quote::quote! {#i32_ts::MIN + 1},
                                    &quote::quote! {0},
                                    &quote::quote! {1},
                                    &quote::quote! {#i32_ts::MAX},
                                    &quote::quote! {#i32_ts::MAX - 1}
                                )),
                                PgType::I64AsInt8 => wrap_into_not_empty_unq_vec_ts(&gen_greater_than_test_new_new_vec_ts(
                                    &quote::quote! {#i64_ts::MIN},
                                    &quote::quote! {#i64_ts::MIN + 1},
                                    &quote::quote! {0},
                                    &quote::quote! {1},
                                    &quote::quote! {#i64_ts::MAX},
                                    &quote::quote! {#i64_ts::MAX - 1}
                                )),
                                PgType::F32AsFloat4 => wrap_into_not_empty_unq_vec_ts(&gen_greater_than_test_new_new_vec_ts(
                                    &quote::quote! {#f32_ts::MIN},
                                    &quote::quote! {#f32_ts::MIN.next_up()},
                                    &quote::quote! {0.0},
                                    &quote::quote! {1.0},
                                    &quote::quote! {#f32_ts::MAX},
                                    &quote::quote! {#f32_ts::MAX.next_down()}
                                )),
                                PgType::F64AsFloat8 => wrap_into_not_empty_unq_vec_ts(&gen_greater_than_test_new_new_vec_ts(
                                //todo rust f64 != pg float8
                                    &quote::quote! {-2.0},
                                    &quote::quote! {-2.0 + 1.0},
                                    &quote::quote! {0.0},
                                    &quote::quote! {1.0},
                                    &quote::quote! {2.0},
                                    &quote::quote! {2.0 - 1.0}
                                )),
                                PgType::SqlxTypesChronoNaiveTimeAsTime => wrap_into_not_empty_unq_vec_ts(&gen_greater_than_test_try_new_try_new_vec_ts(
                                    &quote::quote! {Self::min_inn_type()},
                                    &quote::quote! {Self::slightly_more_than_min_inn_type()},
                                    &quote::quote! {Self::middle_inn_type()},
                                    &quote::quote! {Self::slightly_more_than_middle_inn_type()},
                                    &quote::quote! {Self::max_inn_type()},
                                    &quote::quote! {Self::slightly_less_than_max_inn_type()},
                                )),
                                PgType::SqlxTypesTimeTimeAsTime => wrap_into_not_empty_unq_vec_ts(&gen_greater_than_test_try_new_try_new_vec_ts(
                                    &quote::quote! {Self::min_inn_type()},
                                    &quote::quote! {Self::slightly_more_than_min_inn_type()},
                                    &quote::quote! {Self::middle_inn_type()},
                                    &quote::quote! {Self::slightly_more_than_middle_inn_type()},
                                    &quote::quote! {sqlx::types::time::Time::from_hms_micro(23, 59, 59, 999_999).expect("f3d895bb")},
                                    &quote::quote! {sqlx::types::time::Time::from_hms_micro(23, 59, 59, 999_998).expect("1e71f8c6")},
                                )),
                                PgType::SqlxTypesChronoNaiveDateAsDate => wrap_into_not_empty_unq_vec_ts(&gen_greater_than_test_try_new_try_new_vec_ts(
                                    &quote::quote! {sqlx::types::chrono::NaiveDate::from_ymd_opt(-4712, 12, 30)?},//todo not sure about this values. mb reuse
                                    &quote::quote! {sqlx::types::chrono::NaiveDate::from_ymd_opt(-4712, 12, 31)?},
                                    &quote::quote! {Self::middle_inn_type()},
                                    &quote::quote! {sqlx::types::chrono::NaiveDate::from_ymd_opt(0, 1, 2)?},
                                    &quote::quote! {Self::max_inn_type()},
                                    &quote::quote! {sqlx::types::chrono::NaiveDate::from_ymd_opt(262_142, 12, 30)?},
                                )),
                                PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp => wrap_into_not_empty_unq_vec_ts(&gen_greater_than_test_try_new_try_new_vec_ts(
                                    &quote::quote! {sqlx::types::chrono::NaiveDateTime::new(
                                        sqlx::types::chrono::NaiveDate::from_ymd_opt(-4713, 12, 31)?,
                                        #sqlx_types_chrono_naive_time_as_time_stdrt_nn_ts::min_inn_type()
                                    )},
                                    &quote::quote! {sqlx::types::chrono::NaiveDateTime::new(
                                        sqlx::types::chrono::NaiveDate::from_ymd_opt(-4713, 12, 31)?,
                                        #sqlx_types_chrono_naive_time_as_time_stdrt_nn_ts::slightly_more_than_min_inn_type()
                                    )},
                                    &quote::quote! {sqlx::types::chrono::NaiveDateTime::new(
                                        #sqlx_types_chrono_naive_date_as_date_stdrt_nn_ts::middle_inn_type(),
                                        #sqlx_types_chrono_naive_time_as_time_stdrt_nn_ts::min_inn_type()
                                    )},
                                    &quote::quote! {sqlx::types::chrono::NaiveDateTime::new(
                                        #sqlx_types_chrono_naive_date_as_date_stdrt_nn_ts::middle_inn_type(),
                                        #sqlx_types_chrono_naive_time_as_time_stdrt_nn_ts::slightly_more_than_min_inn_type()
                                    )},
                                    &quote::quote! {sqlx::types::chrono::NaiveDateTime::new(
                                        sqlx::types::chrono::NaiveDate::MAX,
                                        #sqlx_types_chrono_naive_time_as_time_stdrt_nn_ts::max_inn_type()
                                    )},
                                    &quote::quote! {sqlx::types::chrono::NaiveDateTime::new(
                                        sqlx::types::chrono::NaiveDate::MAX,
                                        #sqlx_types_chrono_naive_time_as_time_stdrt_nn_ts::slightly_less_than_max_inn_type()
                                    )},
                                )),
                                PgType::I16AsSmallSerialInitByPg |//todo diffrent test logic for autogenerated?
                                PgType::I32AsSerialInitByPg |//todo diffrent test logic for autogenerated?
                                PgType::I64AsBigSerialInitByPg |//todo diffrent test logic for autogenerated?
                                PgType::SqlxPgTypesPgMoneyAsMoney |
                                PgType::BoolAsBool |
                                PgType::StringAsText |
                                PgType::StdVecVecU8AsBytea |
                                PgType::SqlxPgTypesPgIntervalAsInterval |
                                PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz |
                                PgType::SqlxTypesUuidUuidAsUuidV4InitByPg |
                                PgType::SqlxTypesUuidUuidAsUuidInitByClient |
                                PgType::SqlxTypesIpnetworkIpNetworkAsInet |
                                PgType::SqlxTypesMacAddressMacAddressAsMacAddr |
                                PgType::SqlxPgTypesPgRangeI32AsInt4Range |
                                PgType::SqlxPgTypesPgRangeI64AsInt8Range |
                                PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange |
                                PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange |
                                PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => None,
                            }
                        }
                        pg_crud_macros_cmn::IsNl::True => Some(quote::quote! {
                            <#ident_stdrt_nn_ucc as #import::PgTypeTestCases>::pg_type_opt_vec_wh_greater_than_test().map(
                                |el_e4af7fd9|
                                #import::NotEmptyUnqVec::try_new(
                                    el_e4af7fd9
                                    .into_vec()
                                    .into_iter()
                                    .map(|el_504739e6| #import::PgTypeGreaterThanTest {
                                        vrt: el_504739e6.vrt,
                                        cr: #ident_cr_ucc(#ident_orgn_ucc(Some(el_504739e6.cr.0))),
                                        greater_than: #ident_tt_ucc(#ident_orgn_ucc(Some(el_504739e6.greater_than.0))),
                                    })
                                    .collect()
                                ).expect("63ce5df3")
                            )
                        }),
                    },
                }
            };
            let rd_ids_and_tt_into_pg_type_opt_wh_greater_than_ts: Option<proc_macro2::TokenStream> = match &pg_type_pattern {
                PgTypePattern::Stdrt => {
                    enum IsNeedToImplPgTypeGreaterThanTest {
                        False,
                        TrueFromCr,
                        TrueFromRdIds,
                    }
                    enum CrRdIds {
                        Cr,
                        RdIds,
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
                        PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp => IsNeedToImplPgTypeGreaterThanTest::TrueFromCr,
                        PgType::I16AsSmallSerialInitByPg |
                        PgType::I32AsSerialInitByPg |
                        PgType::I64AsBigSerialInitByPg => IsNeedToImplPgTypeGreaterThanTest::TrueFromRdIds,
                        PgType::SqlxPgTypesPgMoneyAsMoney |//todo why no support?
                        PgType::BoolAsBool |
                        PgType::StringAsText |
                        PgType::StdVecVecU8AsBytea |
                        PgType::SqlxPgTypesPgIntervalAsInterval |
                        PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz |//todo why no support?
                        PgType::SqlxTypesUuidUuidAsUuidV4InitByPg |
                        PgType::SqlxTypesUuidUuidAsUuidInitByClient |
                        PgType::SqlxTypesIpnetworkIpNetworkAsInet |
                        PgType::SqlxTypesMacAddressMacAddressAsMacAddr |
                        PgType::SqlxPgTypesPgRangeI32AsInt4Range |
                        PgType::SqlxPgTypesPgRangeI64AsInt8Range |
                        PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange |
                        PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange |
                        PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => IsNeedToImplPgTypeGreaterThanTest::False,
                    };
                    let gen_some_ts = |cr_rd_ids_prm: &CrRdIds| match &is_nl {
                        pg_crud_macros_cmn::IsNl::False => {
                            let ts = match &cr_rd_ids_prm {
                                CrRdIds::RdIds => quote::quote! {#ident_stdrt_nn_tt_ucc(#rd_ids_sc.0.0)},
                                CrRdIds::Cr => quote::quote! {tt},
                            };
                            quote::quote! {Some(#ident_wh_ucc::GreaterThan(
                                wh_flts::PgTypeWhGreaterThan {
                                    oprtr: greater_than_vrt.oprtr(),
                                    #v_sc: #ts,
                                }
                            ))}
                        }
                        pg_crud_macros_cmn::IsNl::True => {
                            let ts = match &cr_rd_ids_prm {
                                CrRdIds::RdIds => quote::quote! {#rd_ids_sc.0},
                                CrRdIds::Cr => quote::quote! {#tt_sc.0.0},
                            };
                            quote::quote! {
                                #ts.map(|el_886032ca| #ident_wh_ucc::GreaterThan(wh_flts::PgTypeWhGreaterThan {
                                    oprtr: greater_than_vrt.oprtr(),
                                    #v_sc: #ident_stdrt_nn_tt_ucc(el_886032ca),
                                }))
                            }
                        }
                    };
                    match &is_need_to_impl_greater_than_test {
                        IsNeedToImplPgTypeGreaterThanTest::TrueFromRdIds => Some(gen_some_ts(&CrRdIds::RdIds)),
                        IsNeedToImplPgTypeGreaterThanTest::TrueFromCr => Some(gen_some_ts(&CrRdIds::Cr)),
                        IsNeedToImplPgTypeGreaterThanTest::False => None,
                    }
                }
            };
            let opt_vec_cr_generated_ts = opt_vec_cr_ts.as_ref().map(|v| macros_helpers::generated_rust_ts::GeneratedRustTs::from(v.clone()));
            let rd_ids_and_cr_into_opt_vec_wh_eq_to_field_generated_ts =
                rd_ids_and_cr_into_opt_vec_wh_eq_to_field_ts
                    .as_ref()
                    .map(|v| macros_helpers::generated_rust_ts::GeneratedRustTs::from(v.clone()));
            let pg_type_opt_vec_wh_greater_than_test_generated_ts =
                pg_type_opt_vec_wh_greater_than_test_ts
                    .as_ref()
                    .map(|v| macros_helpers::generated_rust_ts::GeneratedRustTs::from(v.clone()));
            let rd_ids_and_tt_into_pg_type_opt_wh_greater_than_generated_ts =
                rd_ids_and_tt_into_pg_type_opt_wh_greater_than_ts
                    .as_ref()
                    .map(|v| macros_helpers::generated_rust_ts::GeneratedRustTs::from(v.clone()));
            pg_crud_macros_cmn::pg_type_test_cases::gen_impl_pg_type_test_cases_for_ident_ts(
                &quote::quote! {#[cfg(feature = "test-utils")]},
                &import,
                &ident_inn_type_ts,
                &ident,
                opt_vec_cr_generated_ts.as_ref(),
                &rd_ids_to_2_dims_vec_rd_inn_ts,
                &rd_inn_into_rd_with_new_or_try_new_unwraped_ts,
                &rd_inn_into_upd_with_new_or_try_new_unwraped_ts,
                &upd_to_rd_ids_ts,
                &rd_ids_to_opt_v_rd_dflt_some_one_el_ts,
                &previous_rd_and_opt_upd_into_rd_ts,
                &rd_ids_and_cr_into_rd_ts,
                &rd_ids_and_cr_into_opt_v_rd_ts,
                &rd_ids_and_cr_into_tt_ts,
                &rd_ids_and_cr_into_wh_eq_ts,
                &rd_ids_and_cr_into_vec_wh_eq_using_fields_ts,
                rd_ids_and_cr_into_opt_vec_wh_eq_to_field_generated_ts.as_ref(),
                pg_type_opt_vec_wh_greater_than_test_generated_ts.as_ref(),
                rd_ids_and_tt_into_pg_type_opt_wh_greater_than_generated_ts.as_ref(),
            )
        };
        let mb_impl_pg_type_pk_for_ident_stdrt_nn_if_can_be_pk_ts = if matches!(&is_nn_stdrt_can_be_pk, IsNnStdrtCanBePk::True) {
            let v_as_rd_ids_ts = quote::quote! {#v_sc: #self_as_pg_type_ts::#rd_ids_ucc};
            quote::quote! {
                #allow_clippy_arbitrary_src_item_ordering
                impl #import::#pg_type_pk_ucc for #ident_stdrt_nn_ucc {
                    type #pg_type_ucc = Self;
                    type #tt_ucc = #ident_stdrt_nn_tt_ucc;
                    fn #rd_ids_into_tt_sc(#v_as_rd_ids_ts) -> #self_as_pg_type_ts::#tt_ucc {
                        #ident_tt_ucc(#v_sc.0.0)
                    }
                    fn #rd_ids_into_rd_sc(#v_as_rd_ids_ts) -> #self_as_pg_type_ts::#rd_ucc {
                        #v_sc.0
                    }
                    fn #rd_ids_into_upd_sc(#v_as_rd_ids_ts) -> #self_as_pg_type_ts::#upd_ucc {
                        #ident_upd_ucc(#v_sc.0.0)
                    }
                    fn #rd_into_tt_sc(
                        #v_sc: #self_as_pg_type_ts::#rd_ucc
                    ) -> #self_as_pg_type_ts::#tt_ucc {
                        #ident_tt_ucc(#v_sc.0)
                    }
                }
            }
        } else {
            proc_macro2::TokenStream::new()
        };
        let mb_impl_pg_type_not_pk_for_ident_ts = if matches!(&is_nn_stdrt_can_be_pk, IsNnStdrtCanBePk::True) {
            macros_helpers::generated_rust_ts::GeneratedRustTs::from(proc_macro2::TokenStream::new())
        } else {
            pg_crud_macros_cmn::gen_impl_pg_type_not_pk_for_ident_ts(&import, &ident)
        };
        let generated = quote::quote! {
            #ident_ts
            #ident_orgn_ts
            #ident_tt_ts
            #ident_cr_ts
            #ident_sel_ts
            #ident_wh_ts
            #ident_rd_ts
            #ident_rd_ids_ts
            #ident_rd_inn_ts
            #ident_upd_ts
            #ident_upd_for_query_ts
            #impl_pg_type_for_ident_ts
            #impl_pg_type_test_cases_for_ident_ts
            #mb_impl_pg_type_pk_for_ident_stdrt_nn_if_can_be_pk_ts
            #mb_impl_pg_type_not_pk_for_ident_ts
        };
        (
            {
                let fi = quote::format_ident!("col_{i}");
                quote::quote! {
                    pub #fi: crate::#ident,
                }
                .to_string()
            },
            generated.to_string(),
        )
    })
    .collect::<(Vec<String>, Vec<String>)>();
    let parse_strs_to_ts2_vec = pg_crud_macros_cmn::ts_helpers::parse_strs_to_ts2_vec;
    let pg_tbl_cols_ts = {
        let ts = parse_strs_to_ts2_vec(
            pg_crud_macros_cmn::ParseTsStrings::from(cols_ts),
            pg_crud_macros_cmn::ParseErIdRef::from("79ee6381"),
        );
        quote::quote! {
            struct PgTblColsUsingPgTypes {
                #ts
            }
        }
    };
    macros_helpers::ts_writer::mb_write_ts_into_file(
        gen_pg_types_config.pg_tbl_cols_write_into_file,
        "pg_tbl_cols_using_pg_types",
        macros_helpers::ts_writer::ProcMacro2TsRef::from(&pg_tbl_cols_ts),
        &macros_helpers::ts_writer::FormatWithCargofmt::True,
    );
    let generated = {
        let ts = parse_strs_to_ts2_vec(
            pg_crud_macros_cmn::ParseTsStrings::from(pg_type_arr),
            pg_crud_macros_cmn::ParseErIdRef::from("e0c9257d"),
        );
        pg_crud_macros_cmn::ts_helpers::gen_mod_with_pub_use_ts(&gen_pg_types_mod_sc, &ts)
    };
    macros_helpers::ts_writer::mb_write_ts_into_file(
        gen_pg_types_config.whole_write_into_file,
        "gen_pg_types",
        macros_helpers::ts_writer::ProcMacro2TsRef::from(generated.as_ref()),
        &macros_helpers::ts_writer::FormatWithCargofmt::True,
    );
    generated
}
