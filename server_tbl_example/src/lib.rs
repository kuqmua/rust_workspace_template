#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, Clone, Copy, gen_pg_tbl::GenPgTbl, optml::Optml)]
#[gen_pg_tbl::gen_pg_tbl_config{{
    "tests_write_into_file": "False",
    "cmn_write_into_file": "False",
    "whole_write_into_file": "False"
}}]
#[gen_pg_tbl::cm_er_vrts{enum CmErVrts{}}]
#[gen_pg_tbl::co_er_vrts{enum CoErVrts{}}]
#[gen_pg_tbl::rm_er_vrts{enum RmErVrts{}}]
#[gen_pg_tbl::ro_er_vrts{enum RoErVrts{}}]
#[gen_pg_tbl::um_er_vrts{enum UmErVrts{}}]
#[gen_pg_tbl::uo_er_vrts{enum UoErVrts{}}]
#[gen_pg_tbl::dm_er_vrts{enum DmErVrts{}}]
#[gen_pg_tbl::dlo_er_vrts{enum DloErVrts{}}]
#[gen_pg_tbl::cmn_er_vrts{enum CmnErVrts{}}]
#[gen_pg_tbl::cm_logic{}]
#[gen_pg_tbl::co_logic{}]
#[gen_pg_tbl::rm_logic{}]
#[gen_pg_tbl::ro_logic{}]
#[gen_pg_tbl::um_logic{}]
#[gen_pg_tbl::uo_logic{}]
#[gen_pg_tbl::dm_logic{}]
#[gen_pg_tbl::dlo_logic{}]
#[gen_pg_tbl::cmn_logic{}]
pub struct TblExample {
    #[gen_pg_tbl_pk]
    pub pk_col: pg_types_text_misc::SqlxTypesUuidUuidAsNnUuidV4InitByPg,
    pub col_0: pg_types_numeric::I16AsNnInt2,
    pub col_1: pg_types_numeric::OptI16AsNlInt2,
    pub col_2: pg_types_numeric::I32AsNnInt4,
}
