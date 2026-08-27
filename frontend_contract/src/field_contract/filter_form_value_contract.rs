use super::{FilterWireJson, FormValueError, FormValueRef};

pub trait FilterFormValueContract {
    fn parse_filter_form_value(value: FormValueRef<'_>) -> Result<FilterWireJson, FormValueError>;
}
