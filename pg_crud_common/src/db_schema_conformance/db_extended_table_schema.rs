pub trait DbExtendedTableSchema: super::DbTableSchema {
    fn checks_and_indexes() -> super::DbObjectSpecs;
    fn exact_defaults() -> super::DbDefaultSpecs;
}
