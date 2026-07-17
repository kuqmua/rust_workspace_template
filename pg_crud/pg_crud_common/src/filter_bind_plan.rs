const FILTER_TEXT_MAXIMUM_BYTES: usize = 1_048_576usize;

#[derive(Clone, Copy, Debug, Eq, PartialEq, newtype::FromInner)]
pub struct PgFilterBool(bool);

#[derive(Clone, Copy, Debug, Eq, PartialEq, newtype::FromInner)]
pub struct PgFilterI64(i64);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PgFilterText(String);
impl TryFrom<String> for PgFilterText {
    type Error = PgFilterTextError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > FILTER_TEXT_MAXIMUM_BYTES {
            Err(PgFilterTextError)
        } else {
            Ok(Self(value))
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
#[error("PostgreSQL filter text exceeds its maximum size")]
pub struct PgFilterTextError;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PgFilterBindValue {
    Bool(PgFilterBool),
    I64(PgFilterI64),
    Text(PgFilterText),
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct PgFilterBindValues(Vec<PgFilterBindValue>);

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct FilterBindPlan {
    values: PgFilterBindValues,
}
impl FilterBindPlan {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push_bool(&mut self, value: PgFilterBool) {
        self.values.0.push(PgFilterBindValue::Bool(value));
    }

    pub fn push_i64(&mut self, value: PgFilterI64) {
        self.values.0.push(PgFilterBindValue::I64(value));
    }

    pub fn push_text(&mut self, value: PgFilterText) {
        self.values.0.push(PgFilterBindValue::Text(value));
    }

    #[must_use]
    pub const fn values(&self) -> &[PgFilterBindValue] {
        self.values.0.as_slice()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn bind_plan_preserves_cross_type_order() {
        let mut plan = super::FilterBindPlan::new();
        plan.push_text(
            super::PgFilterText::try_from(String::from(str_constants::TEST_FILTER_TEXT))
                .expect("43d8053d"),
        );
        plan.push_i64(7i64.into());
        plan.push_bool(true.into());
        assert!(matches!(
            plan.values(),
            [
                super::PgFilterBindValue::Text(_),
                super::PgFilterBindValue::I64(_),
                super::PgFilterBindValue::Bool(_)
            ]
        ));
    }
}
