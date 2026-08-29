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
impl pg_crud_common::default_some_one_element::DefaultSomeOneElement for RegexCase {
    fn default_some_one_element() -> Self {
        Self::Sensitive
    }
}
impl RegexCase {
    #[must_use]
    pub fn postgreql_syntax(&self) -> crate::regex_case_postgreql_syntax::RegexCasePostgreqlSyntax {
        match &self {
            Self::Insensitive => {
                crate::regex_case_postgreql_syntax::RegexCasePostgreqlSyntax::from(
                    constants_str::integration_fixtures::ASTERISK_ALT,
                )
            }
            Self::Sensitive => crate::regex_case_postgreql_syntax::RegexCasePostgreqlSyntax::from(
                constants_str::integration_fixtures::TEXT_ALT_15,
            ),
        }
    }
}
