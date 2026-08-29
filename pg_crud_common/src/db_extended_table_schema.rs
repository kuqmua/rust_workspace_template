pub trait DbExtendedTableSchema: crate::db_table_schema::DbTableSchema {
    fn checks_and_indexes() -> crate::db_object_specs::DbObjectSpecs;
    fn exact_defaults() -> crate::db_default_specs::DbDefaultSpecs;
}
