pub trait FilterFormValueContract {
    fn parse_filter_form_value(
        value: crate::form_value_ref::FormValueRef<'_>,
    ) -> Result<crate::filter_wire_json::FilterWireJson, crate::form_value_error::FormValueError>;
}
