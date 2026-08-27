#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    serde::Serialize,
    serde::Deserialize,
    Eq,
    PartialEq,
    utoipa::ToSchema,
    schemars::JsonSchema,
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::DebugDisplay,
)]
pub enum Operator {
    And,
    AndNot,
    #[default]
    Or,
    OrNot,
}

impl crate::domain_types::DefaultSomeOneElement for Operator {
    fn default_some_one_element() -> Self {
        Self::default()
    }
}

impl Operator {
    #[must_use]
    pub fn to_query_part(
        &self,
        add_operator: crate::domain_types::AddOperator,
    ) -> crate::domain_types::QueryPartFragment {
        let fragment = match (bool::from(add_operator), *self) {
            (false, Self::And | Self::Or) => {
                return match crate::domain_types::QueryPartFragment::try_from(String::new()) {
                    Ok(value) => value,
                    Err(error) => crate::domain_types::QueryPartFragment::from(error),
                };
            }
            (false, Self::AndNot | Self::OrNot) => constants_str::NOT,
            (true, Self::And) => constants_str::AND_ALT,
            (true, Self::AndNot) => constants_str::AND_NOT,
            (true, Self::Or) => constants_str::OR,
            (true, Self::OrNot) => constants_str::OR_NOT,
        };
        match crate::domain_types::QueryPartFragment::try_from(String::from(fragment)) {
            Ok(value) => value,
            Err(error) => crate::domain_types::QueryPartFragment::from(error),
        }
    }
}
