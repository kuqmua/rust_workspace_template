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
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum Order {
    #[default]
    Ascending,
    Descending,
}

impl std::fmt::Display for Order {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ascending => write!(formatter, "{}", naming::domain_types::AscUpperCamelCase),
            Self::Descending => write!(formatter, "{}", naming::domain_types::DescUpperCamelCase),
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
        crate::order_snake_case_str::OrderSnakeCaseStr::try_from(String::from(match self {
            Self::Ascending => constants_str::ASC_ALT,
            Self::Descending => constants_str::DESC_ALT,
        }))
        .unwrap_or_else(crate::order_snake_case_str::OrderSnakeCaseStr::from)
    }

    #[must_use]
    pub fn to_upper_camel_case_str(
        &self,
    ) -> crate::order_upper_camel_case_str::OrderUpperCamelCaseStr {
        crate::order_upper_camel_case_str::OrderUpperCamelCaseStr::try_from(match self {
            Self::Ascending => naming::domain_types::AscUpperCamelCase.to_string(),
            Self::Descending => naming::domain_types::DescUpperCamelCase.to_string(),
        })
        .unwrap_or_else(crate::order_upper_camel_case_str::OrderUpperCamelCaseStr::from)
    }
}
