pub fn escape_admin_like_pattern(
    std_admin_str_ref: crate::std_admin_str_ref::StdAdminStrRef<'_>,
) -> Result<
    crate::std_admin_string::StdAdminString,
    crate::std_admin_string::StdAdminStringTryFromStringError,
> {
    let pattern = std_admin_str_ref.get().chars().fold(
        String::with_capacity(std_admin_str_ref.get().len()),
        |mut pattern, character| {
            if matches!(character, '%' | '_' | '\\') {
                pattern.push('\\');
            }
            pattern.push(character);
            pattern
        },
    );
    crate::std_admin_string::StdAdminString::try_from(pattern)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_escape_admin_like_pattern_preserves_text_and_escapes_pattern_characters() {
        let input = String::from_iter(['a', '%', '_', '\\', '\u{e9}']);
        let expected = String::from_iter(['a', '\\', '%', '\\', '_', '\\', '\\', '\u{e9}']);
        let escaped = super::escape_admin_like_pattern(
            crate::std_admin_str_ref::StdAdminStrRef::from(input.as_str()),
        );
        assert!(escaped.is_ok_and(|pattern| pattern.as_ref().as_str() == expected));
    }

    #[test]
    fn test_escape_admin_like_pattern_preserves_exact_byte_bounds() {
        let limit = constants_usize::VALUE_8_192;
        let escaped_count = 4_096usize;
        [
            (
                constants_str::EMPTY.to_owned(),
                constants_str::EMPTY.to_owned(),
            ),
            (
                constants_str::X.repeat(limit),
                constants_str::X.repeat(limit),
            ),
            (
                String::from_iter(std::iter::repeat_n('%', escaped_count)),
                String::from_iter(['\\', '%']).repeat(escaped_count),
            ),
            (
                String::from_iter(std::iter::repeat_n('_', escaped_count)),
                String::from_iter(['\\', '_']).repeat(escaped_count),
            ),
            (
                String::from_iter(std::iter::repeat_n('\\', escaped_count)),
                String::from_iter(['\\', '\\']).repeat(escaped_count),
            ),
            (
                [
                    constants_str::X.repeat(limit.saturating_sub(constants_str::U_1F496.len())),
                    constants_str::U_1F496.to_owned(),
                ]
                .concat(),
                [
                    constants_str::X.repeat(limit.saturating_sub(constants_str::U_1F496.len())),
                    constants_str::U_1F496.to_owned(),
                ]
                .concat(),
            ),
        ]
        .into_iter()
        .fold((), |(), (input, expected)| {
            assert!(
                super::escape_admin_like_pattern(crate::std_admin_str_ref::StdAdminStrRef::from(
                    input.as_str()
                ))
                .is_ok_and(|pattern| pattern.as_ref().as_str() == expected)
            );
        });
    }

    #[test]
    fn test_escape_admin_like_pattern_rejects_oversized_result() {
        let limit = constants_usize::VALUE_8_192;
        let escaped_count = 4_097usize;
        [
            (
                constants_str::X.repeat(limit.saturating_add(1usize)),
                limit.saturating_add(1usize),
            ),
            (
                String::from_iter(std::iter::repeat_n('%', escaped_count)),
                8_194usize,
            ),
            (
                String::from_iter(std::iter::repeat_n('_', escaped_count)),
                8_194usize,
            ),
            (
                String::from_iter(std::iter::repeat_n('\\', escaped_count)),
                8_194usize,
            ),
            (
                constants_str::U_1F496.repeat(limit),
                limit.saturating_mul(constants_str::U_1F496.len()),
            ),
        ]
        .into_iter()
        .fold((), |(), (input, expected_length)| {
            assert_eq!(
                super::escape_admin_like_pattern(crate::std_admin_str_ref::StdAdminStrRef::from(
                    input.as_str()
                ))
                .err(),
                Some(
                    crate::std_admin_string::StdAdminStringTryFromStringError::TooLong {
                        len: expected_length,
                        max: limit
                    }
                )
            );
        });
    }
}
