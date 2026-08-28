use super::{FormValue, FormValueError, FormValueRef};

pub trait FormValueContract: Sized {
    fn format_form_value(&self) -> Result<FormValue, FormValueError>;
    fn parse_form_value(value: FormValueRef<'_>) -> Result<Self, FormValueError>;
}
