pub fn try_from_bounded_error_text<Value, Error>(error: Error) -> Value
where
    Value: TryFrom<String, Error = Error>,
    Error: std::fmt::Display,
{
    bounded_string_core::try_from_error_text::try_from_error_text(error)
}
