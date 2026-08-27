use super::RegexCasePostgreqlSyntax;

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    utoipa::ToSchema,
    optimal_memory_layout::OptimalMemoryLayout,
)]
pub enum RegexCase {
    Insensitive,
    Sensitive,
}
impl pg_crud_common::domain_types::DefaultSomeOneElement for RegexCase {
    fn default_some_one_element() -> Self {
        Self::Sensitive
    }
}
impl RegexCase {
    #[must_use]
    pub fn postgreql_syntax(&self) -> RegexCasePostgreqlSyntax {
        match &self {
            Self::Insensitive => RegexCasePostgreqlSyntax::from(constants_str::ASTERISK_ALT),
            Self::Sensitive => RegexCasePostgreqlSyntax::from(constants_str::TEXT_ALT_15),
        }
    }
}
