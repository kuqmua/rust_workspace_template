#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum AdminDataGridInputType {
    Date,
    DateTime,
    Number,
    Text,
    Time,
}

impl From<frontend_contract::domain_types::InputKind> for AdminDataGridInputType {
    fn from(value: frontend_contract::domain_types::InputKind) -> Self {
        match value {
            frontend_contract::domain_types::InputKind::Date => Self::Date,
            frontend_contract::domain_types::InputKind::DateTime => Self::DateTime,
            frontend_contract::domain_types::InputKind::Number => Self::Number,
            frontend_contract::domain_types::InputKind::Time => Self::Time,
            frontend_contract::domain_types::InputKind::Checkbox
            | frontend_contract::domain_types::InputKind::Text
            | frontend_contract::domain_types::InputKind::Uuid => Self::Text,
        }
    }
}

impl AsRef<str> for AdminDataGridInputType {
    fn as_ref(&self) -> &str {
        match self {
            Self::Date => constants_str::HTML_DATE_INPUT_TYPE,
            Self::DateTime => constants_str::HTML_DATETIME_LOCAL_INPUT_TYPE,
            Self::Number => constants_str::HTML_NUMBER_INPUT_TYPE,
            Self::Text => constants_str::HTML_TEXT_INPUT_TYPE,
            Self::Time => constants_str::HTML_TIME_INPUT_TYPE,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn maps_contract_input_kinds_to_html_input_types() {
        [
            (
                frontend_contract::domain_types::InputKind::Date,
                constants_str::HTML_DATE_INPUT_TYPE,
            ),
            (
                frontend_contract::domain_types::InputKind::DateTime,
                constants_str::HTML_DATETIME_LOCAL_INPUT_TYPE,
            ),
            (
                frontend_contract::domain_types::InputKind::Number,
                constants_str::HTML_NUMBER_INPUT_TYPE,
            ),
            (
                frontend_contract::domain_types::InputKind::Time,
                constants_str::HTML_TIME_INPUT_TYPE,
            ),
            (
                frontend_contract::domain_types::InputKind::Checkbox,
                constants_str::HTML_TEXT_INPUT_TYPE,
            ),
            (
                frontend_contract::domain_types::InputKind::Text,
                constants_str::HTML_TEXT_INPUT_TYPE,
            ),
            (
                frontend_contract::domain_types::InputKind::Uuid,
                constants_str::HTML_TEXT_INPUT_TYPE,
            ),
        ]
        .into_iter()
        .all(|(kind, expected)| super::AdminDataGridInputType::from(kind).as_ref() == expected)
        .then_some(())
        .expect("8f1547af input kind mappings must match HTML input types");
    }
}
