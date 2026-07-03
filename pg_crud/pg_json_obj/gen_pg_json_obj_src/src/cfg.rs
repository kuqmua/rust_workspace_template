#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pattern {
    Arr,
    Stdrt,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TraitGen {
    PgJson,
    PgTypeAndPgJson,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GenPgJsonsConfig {
    pg_tbl_cols_write_into_pg_tbl_cols_using_pg_json_objs: WriteIntoPgTblColsUsingPgJsonObjs,
    record: PgJsonObjRecord,
    whole_write_into_gen_pg_json_obj: WriteIntoGenPgJsonObj,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PgJsonObjRecord {
    is_nl: pg_crud_macros_cmn::IsNl,
    pattern: Pattern,
    trait_gen: TraitGen,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WriteIntoGenPgJsonObj;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WriteIntoPgTblColsUsingPgJsonObjs;

impl GenPgJsonsConfig {
    #[must_use]
    pub const fn new(
        pg_tbl_cols_write_into_pg_tbl_cols_using_pg_json_objs: WriteIntoPgTblColsUsingPgJsonObjs,
        record: PgJsonObjRecord,
        whole_write_into_gen_pg_json_obj: WriteIntoGenPgJsonObj,
    ) -> Self {
        Self {
            pg_tbl_cols_write_into_pg_tbl_cols_using_pg_json_objs,
            record,
            whole_write_into_gen_pg_json_obj,
        }
    }

    #[must_use]
    pub const fn pg_tbl_cols_write_into_pg_tbl_cols_using_pg_json_objs(
        &self,
    ) -> &WriteIntoPgTblColsUsingPgJsonObjs {
        &self.pg_tbl_cols_write_into_pg_tbl_cols_using_pg_json_objs
    }

    #[must_use]
    pub const fn record(&self) -> &PgJsonObjRecord {
        &self.record
    }

    #[must_use]
    pub const fn whole_write_into_gen_pg_json_obj(&self) -> &WriteIntoGenPgJsonObj {
        &self.whole_write_into_gen_pg_json_obj
    }
}

impl PgJsonObjRecord {
    #[must_use]
    pub const fn is_nl(&self) -> pg_crud_macros_cmn::IsNl {
        self.is_nl
    }

    #[must_use]
    pub const fn new(
        is_nl: pg_crud_macros_cmn::IsNl,
        pattern: Pattern,
        trait_gen: TraitGen,
    ) -> Self {
        Self {
            is_nl,
            pattern,
            trait_gen,
        }
    }

    #[must_use]
    pub const fn pattern(&self) -> Pattern {
        self.pattern
    }

    #[must_use]
    pub const fn trait_gen(&self) -> TraitGen {
        self.trait_gen
    }
}
