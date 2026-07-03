#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct ServerTableExample(pg_tbl::PgTableLeaf);

impl ServerTableExample {
    #[must_use]
    pub const fn new(table_leaf: pg_tbl::PgTableLeaf) -> Self {
        Self(table_leaf)
    }
}
