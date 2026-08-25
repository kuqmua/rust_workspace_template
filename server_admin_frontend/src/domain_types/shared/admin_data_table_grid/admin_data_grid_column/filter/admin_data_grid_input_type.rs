#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum AdminDataGridInputType {
    Date,
    DateTime,
    Number,
    Text,
    Time,
}

impl From<server_admin_contract::domain_types::AdminDataInputKind> for AdminDataGridInputType {
    fn from(value: server_admin_contract::domain_types::AdminDataInputKind) -> Self {
        match value {
            server_admin_contract::domain_types::AdminDataInputKind::Date => Self::Date,
            server_admin_contract::domain_types::AdminDataInputKind::DateTime => Self::DateTime,
            server_admin_contract::domain_types::AdminDataInputKind::Number => Self::Number,
            server_admin_contract::domain_types::AdminDataInputKind::Time => Self::Time,
            server_admin_contract::domain_types::AdminDataInputKind::Checkbox
            | server_admin_contract::domain_types::AdminDataInputKind::Text
            | server_admin_contract::domain_types::AdminDataInputKind::Uuid => Self::Text,
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
                server_admin_contract::domain_types::AdminDataInputKind::Date,
                constants_str::HTML_DATE_INPUT_TYPE,
            ),
            (
                server_admin_contract::domain_types::AdminDataInputKind::DateTime,
                constants_str::HTML_DATETIME_LOCAL_INPUT_TYPE,
            ),
            (
                server_admin_contract::domain_types::AdminDataInputKind::Number,
                constants_str::HTML_NUMBER_INPUT_TYPE,
            ),
            (
                server_admin_contract::domain_types::AdminDataInputKind::Time,
                constants_str::HTML_TIME_INPUT_TYPE,
            ),
            (
                server_admin_contract::domain_types::AdminDataInputKind::Checkbox,
                constants_str::HTML_TEXT_INPUT_TYPE,
            ),
            (
                server_admin_contract::domain_types::AdminDataInputKind::Text,
                constants_str::HTML_TEXT_INPUT_TYPE,
            ),
            (
                server_admin_contract::domain_types::AdminDataInputKind::Uuid,
                constants_str::HTML_TEXT_INPUT_TYPE,
            ),
        ]
        .into_iter()
        .all(|(kind, expected)| super::AdminDataGridInputType::from(kind).as_ref() == expected)
        .then_some(())
        .expect("8f1547af input kind mappings must match HTML input types");
    }
}
