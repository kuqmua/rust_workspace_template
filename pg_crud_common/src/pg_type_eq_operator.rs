pub trait PgTypeEqOperator {
    fn operator(&self) -> crate::domain_types::EqOperator;
}
