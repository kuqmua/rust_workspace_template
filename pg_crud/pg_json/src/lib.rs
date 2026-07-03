#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct PgJsonFacade {
    number_leaf: pg_json_nbr::PgJsonNumberLeaf,
    other_leaf: pg_json_other::PgJsonOtherLeaf,
}

impl PgJsonFacade {
    #[must_use]
    pub const fn new(
        number_leaf: pg_json_nbr::PgJsonNumberLeaf,
        other_leaf: pg_json_other::PgJsonOtherLeaf,
    ) -> Self {
        Self {
            number_leaf,
            other_leaf,
        }
    }
}
