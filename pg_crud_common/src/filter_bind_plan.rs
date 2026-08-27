#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    newtype::FromInner,
)]
pub struct FilterBindPlan(Vec<crate::domain_types::PgFilterBindValue>);
impl FilterBindPlan {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push_bool(&mut self, value: crate::domain_types::PgFilterBool) {
        self.0
            .push(crate::domain_types::PgFilterBindValue::Bool(value));
    }

    pub fn push_i64(&mut self, value: crate::domain_types::PgFilterI64) {
        self.0
            .push(crate::domain_types::PgFilterBindValue::I64(value));
    }

    pub fn push_text(&mut self, value: crate::domain_types::PgFilterText) {
        self.0
            .push(crate::domain_types::PgFilterBindValue::Text(value));
    }

    #[must_use]
    pub const fn values(&self) -> &[crate::domain_types::PgFilterBindValue] {
        self.0.as_slice()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn bind_plan_preserves_cross_type_order() {
        let mut plan = super::FilterBindPlan::new();
        plan.push_text(
            crate::domain_types::PgFilterText::try_from(String::from(
                constants_str::TEST_FILTER_TEXT,
            ))
            .expect("43d8053d bind_plan_preserves_cross_type_order invariant must hold"),
        );
        plan.push_i64(7i64.into());
        plan.push_bool(true.into());
        assert!(matches!(
            plan.values(),
            [
                crate::domain_types::PgFilterBindValue::Text(_),
                crate::domain_types::PgFilterBindValue::I64(_),
                crate::domain_types::PgFilterBindValue::Bool(_)
            ]
        ));
    }
}
