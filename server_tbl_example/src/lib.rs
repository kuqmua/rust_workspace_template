#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, Clone, Copy, pg_crud::GenPgTbl, optml::Optml)]
#[pg_crud::gen_pg_tbl_config{{
    "tests_write_into_file": "False",
    "cmn_write_into_file": "False",
    "whole_write_into_file": "False"
}}]
#[pg_crud::cm_er_vrts{enum CmErVrts{}}]
#[pg_crud::co_er_vrts{enum CoErVrts{}}]
#[pg_crud::rm_er_vrts{enum RmErVrts{}}]
#[pg_crud::ro_er_vrts{enum RoErVrts{}}]
#[pg_crud::um_er_vrts{enum UmErVrts{}}]
#[pg_crud::uo_er_vrts{enum UoErVrts{}}]
#[pg_crud::dm_er_vrts{enum DmErVrts{}}]
#[pg_crud::dlo_er_vrts{enum DloErVrts{}}]
#[pg_crud::cmn_er_vrts{
    enum CmnErVrts {
        // CheckCommit {
        //     #[eo_loc]
        //     check_commit: pg_crud::check_commit::CommitEr,
        //     loc: loc_lib::loc::Loc,
        // },
    }
}]
#[pg_crud::cm_logic{}]
#[pg_crud::co_logic{}]
#[pg_crud::rm_logic{}]
#[pg_crud::ro_logic{}]
#[pg_crud::um_logic{}]
#[pg_crud::uo_logic{}]
#[pg_crud::dm_logic{}]
#[pg_crud::dlo_logic{}]
#[pg_crud::cmn_logic{}]
pub struct TblExample {
    // #[gen_pg_crud_pk]
    // pub pk: pg_crud::I64AsNnBigSerialInitByPg,
    #[gen_pg_tbl_pk]
    pub pk_col: pg_crud::SqlxTypesUuidUuidAsNnUuidV4InitByPg,
    pub col_0: pg_crud::I16AsNnInt2,
    pub col_1: pg_crud::OptI16AsNlInt2,
    pub col_2: pg_crud::I32AsNnInt4,
    // pub col_6: pg_crud::I32AsNnInt4,
    // pub col_7: pg_crud::OptI32AsNlInt4,
    // pub col_12: pg_crud::I64AsNnInt8,
    // pub col_13: pg_crud::OptI64AsNlInt8,
    // pub col_18: pg_crud::F32AsNnFloat4,
    // pub col_19: pg_crud::OptF32AsNlFloat4,
    // pub col_24: pg_crud::F64AsNnFloat8,
    // pub col_25: pg_crud::OptF64AsNlFloat8,
    // pub col_30: pg_crud::I16AsNnSmallSerialInitByPg,
    // pub col_31: pg_crud::I32AsNnSerialInitByPg,
    // pub col_32: pg_crud::I64AsNnBigSerialInitByPg,
    // pub col_33: pg_crud::SqlxPgTypesPgMoneyAsNnMoney,
    // pub col_34: pg_crud::OptSqlxPgTypesPgMoneyAsNlMoney,
    // pub col_39: pg_crud::BoolAsNnBool,
    // pub col_40: pg_crud::OptBoolAsNlBool,
    // pub col_45: pg_crud::StringAsNnText,
    // pub col_46: pg_crud::OptStringAsNlText,
    // pub col_51: pg_crud::StdVecVecU8AsNnBytea,
    // pub col_52: pg_crud::OptStdVecVecU8AsNlBytea,
    // pub col_57: pg_crud::SqlxTypesChronoNaiveTimeAsNnTime,
    // pub col_58: pg_crud::OptSqlxTypesChronoNaiveTimeAsNlTime,
    // pub col_63: pg_crud::SqlxTypesTimeTimeAsNnTime,
    // pub col_64: pg_crud::OptSqlxTypesTimeTimeAsNlTime,
    // pub col_69: pg_crud::SqlxPgTypesPgIntervalAsNnInterval,
    // pub col_70: pg_crud::OptSqlxPgTypesPgIntervalAsNlInterval,
    // pub col_75: pg_crud::SqlxTypesChronoNaiveDateAsNnDate,
    // pub col_76: pg_crud::OptSqlxTypesChronoNaiveDateAsNlDate,
    // pub col_81: pg_crud::SqlxTypesChronoNaiveDateTimeAsNnTimestamp,
    // pub col_82: pg_crud::OptSqlxTypesChronoNaiveDateTimeAsNlTimestamp,
    // pub col_87: pg_crud::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNnTimestampTz,
    // pub col_88: pg_crud::OptSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNlTimestampTz,
    // pub col_93: pg_crud::SqlxTypesUuidUuidAsNnUuidV4InitByPg,
    // pub col_94: pg_crud::SqlxTypesUuidUuidAsNnUuidInitByClient,
    // pub col_95: pg_crud::OptSqlxTypesUuidUuidAsNlUuidInitByClient,
    // pub col_100: pg_crud::SqlxTypesIpnetworkIpNetworkAsNnInet,
    // pub col_101: pg_crud::OptSqlxTypesIpnetworkIpNetworkAsNlInet,
    // pub col_106: pg_crud::SqlxTypesMacAddressMacAddressAsNnMacAddr,
    // pub col_107: pg_crud::OptSqlxTypesMacAddressMacAddressAsNlMacAddr,
    // pub col_112: pg_crud::SqlxPgTypesPgRangeI32AsNnInt4Range,
    // pub col_113: pg_crud::OptSqlxPgTypesPgRangeI32AsNlInt4Range,
    // pub col_118: pg_crud::SqlxPgTypesPgRangeI64AsNnInt8Range,
    // pub col_119: pg_crud::OptSqlxPgTypesPgRangeI64AsNlInt8Range,
    // pub col_124: pg_crud::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsNnDateRange,
    // pub col_125: pg_crud::OptSqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsNlDateRange,
    // pub col_130: pg_crud::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsNnTimestampRange,
    // pub col_131: pg_crud::OptSqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsNlTimestampRange,
    // pub col_136: pg_crud::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNnTimestampTzRange,
    // pub col_137: pg_crud::OptSqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNlTimestampTzRange,
}
