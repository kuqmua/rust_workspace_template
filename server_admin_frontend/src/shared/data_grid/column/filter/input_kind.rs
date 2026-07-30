#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum AdminDataGridInputType {
    Date,
    DateTime,
    Number,
    Text,
    Time,
}

impl From<server_admin_contract::AdminDataInputKind> for AdminDataGridInputType {
    fn from(value: server_admin_contract::AdminDataInputKind) -> Self {
        match value {
            server_admin_contract::AdminDataInputKind::Date => Self::Date,
            server_admin_contract::AdminDataInputKind::DateTime => Self::DateTime,
            server_admin_contract::AdminDataInputKind::Number => Self::Number,
            server_admin_contract::AdminDataInputKind::Time => Self::Time,
            server_admin_contract::AdminDataInputKind::Checkbox
            | server_admin_contract::AdminDataInputKind::Text
            | server_admin_contract::AdminDataInputKind::Uuid => Self::Text,
        }
    }
}

impl AsRef<str> for AdminDataGridInputType {
    fn as_ref(&self) -> &str {
        match self {
            Self::Date => str_constants::HTML_DATE_INPUT_TYPE,
            Self::DateTime => str_constants::HTML_DATETIME_LOCAL_INPUT_TYPE,
            Self::Number => str_constants::HTML_NUMBER_INPUT_TYPE,
            Self::Text => str_constants::HTML_TEXT_INPUT_TYPE,
            Self::Time => str_constants::HTML_TIME_INPUT_TYPE,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn maps_contract_input_kinds_to_html_input_types() {
        [
            (
                server_admin_contract::AdminDataInputKind::Date,
                str_constants::HTML_DATE_INPUT_TYPE,
            ),
            (
                server_admin_contract::AdminDataInputKind::DateTime,
                str_constants::HTML_DATETIME_LOCAL_INPUT_TYPE,
            ),
            (
                server_admin_contract::AdminDataInputKind::Number,
                str_constants::HTML_NUMBER_INPUT_TYPE,
            ),
            (
                server_admin_contract::AdminDataInputKind::Time,
                str_constants::HTML_TIME_INPUT_TYPE,
            ),
            (
                server_admin_contract::AdminDataInputKind::Checkbox,
                str_constants::HTML_TEXT_INPUT_TYPE,
            ),
            (
                server_admin_contract::AdminDataInputKind::Text,
                str_constants::HTML_TEXT_INPUT_TYPE,
            ),
            (
                server_admin_contract::AdminDataInputKind::Uuid,
                str_constants::HTML_TEXT_INPUT_TYPE,
            ),
        ]
        .into_iter()
        .all(|(kind, expected)| super::AdminDataGridInputType::from(kind).as_ref() == expected)
        .then_some(())
        .expect("8f1547af input kind mappings must match HTML input types");
    }
}
