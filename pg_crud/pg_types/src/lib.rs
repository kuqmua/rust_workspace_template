#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct PgTypesFacade {
    chrono_date: pg_types_chrono_net::SqlxTypesChronoNaiveDateAsDate,
    numeric_bool: pg_types_numeric::BoolAsBool,
    pagination: pg_types_cmn::PgnStartsWithOne,
    text: pg_types_text_misc::StringAsText,
}

impl PgTypesFacade {
    #[must_use]
    pub const fn new(
        chrono_date: pg_types_chrono_net::SqlxTypesChronoNaiveDateAsDate,
        numeric_bool: pg_types_numeric::BoolAsBool,
        pagination: pg_types_cmn::PgnStartsWithOne,
        text: pg_types_text_misc::StringAsText,
    ) -> Self {
        Self {
            chrono_date,
            numeric_bool,
            pagination,
            text,
        }
    }
}
