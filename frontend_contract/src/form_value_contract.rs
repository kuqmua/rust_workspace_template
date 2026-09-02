pub trait FormValueContract: Sized {
    fn format_form_value(
        &self,
    ) -> Result<crate::form_value::FormValue, crate::form_value_error::FormValueError>;
    fn parse_form_value(
        form_value_ref: crate::form_value_ref::FormValueRef<'_>,
    ) -> Result<Self, crate::form_value_error::FormValueError>;
}
