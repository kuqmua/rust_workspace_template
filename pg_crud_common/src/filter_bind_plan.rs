#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    proc_macro_newtype_from_inner::FromInner,
)]
pub struct FilterBindPlan(Vec<crate::pg_filter_bind_value::PgFilterBindValue>);
impl FilterBindPlan {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push_bool(&mut self, pg_filter_bool: crate::pg_filter_bool::PgFilterBool) {
        self.0
            .push(crate::pg_filter_bind_value::PgFilterBindValue::Bool(
                pg_filter_bool,
            ));
    }

    pub fn push_i64(&mut self, pg_filter_i64: crate::pg_filter_i64::PgFilterI64) {
        self.0
            .push(crate::pg_filter_bind_value::PgFilterBindValue::I64(
                pg_filter_i64,
            ));
    }

    pub fn push_text(&mut self, pg_filter_text: crate::pg_filter_text::PgFilterText) {
        self.0
            .push(crate::pg_filter_bind_value::PgFilterBindValue::Text(
                pg_filter_text,
            ));
    }

    #[must_use]
    pub const fn values(&self) -> &[crate::pg_filter_bind_value::PgFilterBindValue] {
        self.0.as_slice()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_bind_plan_preserves_cross_type_order() {
        let mut plan = crate::filter_bind_plan::FilterBindPlan::new();
        assert!(plan.values().is_empty());
        assert!(
            crate::filter_bind_plan::FilterBindPlan::default()
                .values()
                .is_empty()
        );
        plan.push_text(
            crate::pg_filter_text::PgFilterText::try_from(String::from(
                constants_str::TEST_FILTER_TEXT,
            ))
            .expect(constants_str::DIAGNOSTIC_43D8053D),
        );
        plan.push_i64(7i64.into());
        plan.push_bool(true.into());
        assert!(matches!(
            plan.values(),
            [
                crate::pg_filter_bind_value::PgFilterBindValue::Text(text),
                crate::pg_filter_bind_value::PgFilterBindValue::I64(integer),
                crate::pg_filter_bind_value::PgFilterBindValue::Bool(boolean)
            ] if crate::pg_filter_text::PgFilterText::try_from(constants_str::TEST_FILTER_TEXT.to_owned()).as_ref().ok() == Some(text)
                && *integer == crate::pg_filter_i64::PgFilterI64::from(7i64)
                && *boolean == crate::pg_filter_bool::PgFilterBool::from(true)
        ));
    }
}
