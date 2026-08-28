#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum AdminDataGridInputType {
    Date,
    DateTime,
    Number,
    Text,
    Time,
}

impl From<frontend_contract::InputKind> for AdminDataGridInputType {
    fn from(value: frontend_contract::InputKind) -> Self {
        match value {
            frontend_contract::InputKind::Date => Self::Date,
            frontend_contract::InputKind::DateTime => Self::DateTime,
            frontend_contract::InputKind::Number => Self::Number,
            frontend_contract::InputKind::Time => Self::Time,
            frontend_contract::InputKind::Checkbox
            | frontend_contract::InputKind::Text
            | frontend_contract::InputKind::Uuid => Self::Text,
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
                frontend_contract::InputKind::Date,
                constants_str::HTML_DATE_INPUT_TYPE,
            ),
            (
                frontend_contract::InputKind::DateTime,
                constants_str::HTML_DATETIME_LOCAL_INPUT_TYPE,
            ),
            (
                frontend_contract::InputKind::Number,
                constants_str::HTML_NUMBER_INPUT_TYPE,
            ),
            (
                frontend_contract::InputKind::Time,
                constants_str::HTML_TIME_INPUT_TYPE,
            ),
            (
                frontend_contract::InputKind::Checkbox,
                constants_str::HTML_TEXT_INPUT_TYPE,
            ),
            (
                frontend_contract::InputKind::Text,
                constants_str::HTML_TEXT_INPUT_TYPE,
            ),
            (
                frontend_contract::InputKind::Uuid,
                constants_str::HTML_TEXT_INPUT_TYPE,
            ),
        ]
        .into_iter()
        .all(|(kind, expected)| super::AdminDataGridInputType::from(kind).as_ref() == expected)
        .then_some(())
        .expect("8f1547af input kind mappings must match HTML input types");
    }
}
