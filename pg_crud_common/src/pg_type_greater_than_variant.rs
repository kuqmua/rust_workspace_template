#[derive(Debug, Clone, Copy, PartialEq, optimal_memory_layout::OptimalMemoryLayout)]
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
