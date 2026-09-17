#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    proc_macro_newtype_into_inner_from::IntoInnerFrom,
)]
pub struct ParameterizedRoutePath(
    bounded_types::bounded_string::BoundedString<0usize, 8_192usize, false>,
);
impl TryFrom<String> for ParameterizedRoutePath {
    type Error = crate::parameterized_route_path_try_from_string_error::ParameterizedRoutePathTryFromStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > constants_usize::VALUE_8_192 {
            return Err(crate::parameterized_route_path_try_from_string_error::ParameterizedRoutePathTryFromStringError);
        }
        bounded_types::bounded_string::BoundedString::try_from(value)
            .map(Self)
            .map_err(|source| match source {
                bounded_types::bounded_string_error::BoundedStringError::AboveMaximum {
                    ..
                }
                | bounded_types::bounded_string_error::BoundedStringError::BelowMinimum {
                    ..
                } => crate::parameterized_route_path_try_from_string_error::ParameterizedRoutePathTryFromStringError,
            })
    }
}
