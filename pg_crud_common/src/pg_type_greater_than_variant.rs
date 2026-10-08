#[derive(Debug, Clone, Copy, PartialEq, proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
pub enum PgTypeGreaterThanVariant {
    EqNotGreaterThan,
    GreaterThan,
    NotGreaterThan,
}

impl PgTypeGreaterThanVariant {
    #[must_use]
    pub const fn operator(&self) -> crate::operator::Operator {
        match *self {
            Self::GreaterThan => crate::operator::Operator::Or,
            Self::NotGreaterThan | Self::EqNotGreaterThan => crate::operator::Operator::OrNot,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_greater_than_variants_preserve_boolean_operator_mapping() {
        assert!(
            [
                (
                    crate::pg_type_greater_than_variant::PgTypeGreaterThanVariant::GreaterThan,
                    crate::operator::Operator::Or
                ),
                (
                    crate::pg_type_greater_than_variant::PgTypeGreaterThanVariant::NotGreaterThan,
                    crate::operator::Operator::OrNot
                ),
                (
                    crate::pg_type_greater_than_variant::PgTypeGreaterThanVariant::EqNotGreaterThan,
                    crate::operator::Operator::OrNot
                ),
            ]
            .into_iter()
            .all(
                |(pg_type_greater_than_variant, expected)| pg_type_greater_than_variant.operator()
                    == expected
            )
        );
    }
}
