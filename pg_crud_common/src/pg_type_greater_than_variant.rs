#[derive(Debug, Clone, Copy, PartialEq, optimal_memory_layout::OptimalMemoryLayout)]
pub enum PgTypeGreaterThanVariant {
    EqNotGreaterThan,
    GreaterThan,
    NotGreaterThan,
}

impl PgTypeGreaterThanVariant {
    #[must_use]
    pub const fn operator(&self) -> crate::domain_types::Operator {
        match *self {
            Self::GreaterThan => crate::domain_types::Operator::Or,
            Self::NotGreaterThan | Self::EqNotGreaterThan => crate::domain_types::Operator::OrNot,
        }
    }
}
