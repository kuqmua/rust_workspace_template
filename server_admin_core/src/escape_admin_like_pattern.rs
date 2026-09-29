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
    fn test_escape_admin_like_pattern_rejects_oversized_result() {
        let input = String::from_iter(std::iter::repeat_n('%', 8_192usize));
        assert!(
            super::escape_admin_like_pattern(crate::std_admin_str_ref::StdAdminStrRef::from(
                input.as_str(),
            ))
            .err()
            .is_some()
        );
    }
}
