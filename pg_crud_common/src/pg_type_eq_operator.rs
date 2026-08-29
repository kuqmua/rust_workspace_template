pub trait PgTypeEqOperator {
    fn operator(&self) -> crate::eq_operator::EqOperator;
}
