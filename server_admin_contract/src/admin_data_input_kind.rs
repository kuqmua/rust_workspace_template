#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    utoipa::ToSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum AdminDataInputKind {
    Checkbox,
    Date,
    DateTime,
    Number,
    Text,
    Time,
    Uuid,
}
impl From<frontend_contract::domain_types::InputKind> for AdminDataInputKind {
    fn from(value: frontend_contract::domain_types::InputKind) -> Self {
        match value {
            frontend_contract::domain_types::InputKind::Checkbox => Self::Checkbox,
            frontend_contract::domain_types::InputKind::Date => Self::Date,
            frontend_contract::domain_types::InputKind::DateTime => Self::DateTime,
            frontend_contract::domain_types::InputKind::Number => Self::Number,
            frontend_contract::domain_types::InputKind::Text => Self::Text,
            frontend_contract::domain_types::InputKind::Time => Self::Time,
            frontend_contract::domain_types::InputKind::Uuid => Self::Uuid,
        }
    }
}
