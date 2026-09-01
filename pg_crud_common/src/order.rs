#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    Eq,
    utoipa::ToSchema,
    strum_macros::EnumString,
    optimal_memory_layout::OptimalMemoryLayout,
)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum Order {
    #[default]
    Ascending,
    Descending,
}

impl std::fmt::Display for Order {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ascending => write!(f, "{}", naming::domain_types::AscUpperCamelCase),
            Self::Descending => write!(f, "{}", naming::domain_types::DescUpperCamelCase),
        }
    }
}

impl crate::default_some_one_element::DefaultSomeOneElement for Order {
    fn default_some_one_element() -> Self {
        Self::default()
    }
}

impl Order {
    #[must_use]
    pub fn to_snake_case_str(&self) -> crate::order_snake_case_str::OrderSnakeCaseStr {
        crate::order_snake_case_str::OrderSnakeCaseStr::try_from(
            naming_common::domain_types::DisplayToSnakeCaseStr::case(self),
        )
        .unwrap_or_else(crate::order_snake_case_str::OrderSnakeCaseStr::from)
    }

    #[must_use]
    pub fn to_upper_camel_case_str(
        &self,
    ) -> crate::order_upper_camel_case_str::OrderUpperCamelCaseStr {
        crate::order_upper_camel_case_str::OrderUpperCamelCaseStr::try_from(
            naming_common::domain_types::DisplayToUpperCamelCaseStr::case(self),
        )
        .unwrap_or_else(crate::order_upper_camel_case_str::OrderUpperCamelCaseStr::from)
    }
}
