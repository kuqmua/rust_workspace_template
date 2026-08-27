#[must_use]
pub fn secret_texts_match(
    expected: super::SecretTextRef<'_>,
    provided: super::SecretTextRef<'_>,
) -> super::SecretTextMatch {
    let expected_bytes = expected.0.as_bytes();
    let provided_bytes = provided.0.as_bytes();
    let length_difference = expected_bytes.len() ^ provided_bytes.len();
    let difference = (constants_usize::ZERO..constants_usize::VALUE_8_192).fold(
        length_difference,
        |accumulated, index| {
            let expected_byte = expected_bytes.get(index).copied().unwrap_or_default();
            let provided_byte = provided_bytes.get(index).copied().unwrap_or_default();
            accumulated | usize::from(expected_byte ^ provided_byte)
        },
    );
    if difference == constants_usize::ZERO {
        super::SecretTextMatch::Equal
    } else {
        super::SecretTextMatch::Different
    }
}
