#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, Hash, PartialEq,
)]
pub struct SingleFlightKey(
    bounded_types::bounded_string::BoundedString<
        1usize,
        { crate::single_flight_key_maximum_bytes::SINGLE_FLIGHT_KEY_MAXIMUM_BYTES },
        false,
    >,
);
impl TryFrom<String> for SingleFlightKey {
    type Error = crate::single_flight_key_error::SingleFlightKeyError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > crate::single_flight_key_maximum_bytes::SINGLE_FLIGHT_KEY_MAXIMUM_BYTES {
            return Err(crate::single_flight_key_error::SingleFlightKeyError::TooLong);
        }
        if value.is_empty() {
            Err(crate::single_flight_key_error::SingleFlightKeyError::Empty)
        } else if value.contains('\0') {
            Err(crate::single_flight_key_error::SingleFlightKeyError::ContainsNul)
        } else {
            bounded_types::bounded_string::BoundedString::try_from(value)
                .map(Self)
                .map_err(|source| match source {
                    bounded_types::bounded_string_error::BoundedStringError::AboveMaximum {
                        ..
                    } => crate::single_flight_key_error::SingleFlightKeyError::TooLong,
                    bounded_types::bounded_string_error::BoundedStringError::BelowMinimum {
                        ..
                    } => crate::single_flight_key_error::SingleFlightKeyError::Empty,
                })
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_single_flight_key_bounds_use_bytes_and_preserve_error_precedence() {
        let maximum = crate::single_flight_key_maximum_bytes::SINGLE_FLIGHT_KEY_MAXIMUM_BYTES;
        [
            (
                constants_str::EMPTY.to_owned(),
                Err(crate::single_flight_key_error::SingleFlightKeyError::Empty),
            ),
            (constants_str::SPACE.to_owned(), Ok(())),
            (
                constants_str::TEST_TEXT_WITH_NUL.to_owned(),
                Err(crate::single_flight_key_error::SingleFlightKeyError::ContainsNul),
            ),
            (constants_str::X.repeat(maximum), Ok(())),
            (
                constants_str::X.repeat(maximum + constants_usize::ONE),
                Err(crate::single_flight_key_error::SingleFlightKeyError::TooLong),
            ),
            (
                '\u{e9}'
                    .to_string()
                    .repeat(maximum.div_euclid(constants_usize::TWO)),
                Ok(()),
            ),
            (
                '\u{e9}'
                    .to_string()
                    .repeat(maximum.div_euclid(constants_usize::TWO) + constants_usize::ONE),
                Err(crate::single_flight_key_error::SingleFlightKeyError::TooLong),
            ),
            (
                '\0'.to_string().repeat(maximum + constants_usize::ONE),
                Err(crate::single_flight_key_error::SingleFlightKeyError::TooLong),
            ),
        ]
        .into_iter()
        .fold((), |(), (value, expected)| {
            assert_eq!(
                super::SingleFlightKey::try_from(value.clone()).map(|key| key.0.as_str() == value),
                expected.map(|()| true)
            );
        });
    }
}
