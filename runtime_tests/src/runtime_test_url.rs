#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype_as_ref_str::AsRefStr,
)]
pub(crate) struct RuntimeTestUrl(
    bounded_types::bounded_string::BoundedString<0usize, 8_192usize, false>,
);

impl TryFrom<String> for RuntimeTestUrl {
    type Error = crate::service_base_url_error::ServiceBaseUrlError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > constants_usize::VALUE_8_192 {
            Err(crate::service_base_url_error::ServiceBaseUrlError::Length)
        } else {
            bounded_types::bounded_string::BoundedString::try_from(value)
                .map(Self)
                .map_err(|source| match source {
                    bounded_types::bounded_string_error::BoundedStringError::AboveMaximum {
                        ..
                    }
                    | bounded_types::bounded_string_error::BoundedStringError::BelowMinimum {
                        ..
                    } => crate::service_base_url_error::ServiceBaseUrlError::Length,
                })
        }
    }
}
