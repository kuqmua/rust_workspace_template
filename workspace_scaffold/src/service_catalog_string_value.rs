pub(super) fn service_catalog_string_value(
    line: crate::scaffold_text_ref::ScaffoldTextRef<'_>,
    key: crate::scaffold_text_ref::ScaffoldTextRef<'_>,
) -> Result<Option<crate::scaffold_text::ScaffoldText>, crate::scaffold_error::ScaffoldError> {
    let Some(assignment) = line
        .get()
        .strip_prefix(key.get())
        .and_then(|remaining| remaining.trim().strip_prefix('='))
    else {
        return Ok(None);
    };
    let Some(text) = assignment
        .trim()
        .strip_prefix('"')
        .and_then(|without_opening_quote| without_opening_quote.strip_suffix('"'))
    else {
        return Err(crate::scaffold_error::ScaffoldError::Catalog);
    };
    let mut decoded = String::with_capacity(text.len());
    let mut characters = text.chars();
    while let Some(character) = characters.next() {
        let decoded_character = if character == '\\' {
            match characters.next() {
                Some('"') => '"',
                Some('\\') => '\\',
                Some('b') => '\u{0008}',
                Some('t') => '\t',
                Some('n') => '\n',
                Some('f') => '\u{000C}',
                Some('r') => '\r',
                Some(width @ ('u' | 'U')) => {
                    let digits = if width == 'u' { 4usize } else { 8usize };
                    let codepoint = (0..digits).try_fold(0u32, |codepoint, _index| {
                        let digit = characters.next().and_then(|digit| digit.to_digit(16u32))?;
                        codepoint.checked_mul(16u32)?.checked_add(digit)
                    });
                    codepoint
                        .and_then(char::from_u32)
                        .ok_or(crate::scaffold_error::ScaffoldError::Catalog)?
                }
                _ => return Err(crate::scaffold_error::ScaffoldError::Catalog),
            }
        } else if character == '"' || (character.is_control() && character != '\t') {
            return Err(crate::scaffold_error::ScaffoldError::Catalog);
        } else {
            character
        };
        decoded.push(decoded_character);
    }
    crate::scaffold_text::ScaffoldText::try_from(decoded)
        .map(Some)
        .map_err(|_error| crate::scaffold_error::ScaffoldError::Catalog)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_catalog_string_decodes_unicode_escape() {
        let result = crate::service_catalog_string_value::service_catalog_string_value(
            crate::scaffold_text_ref::ScaffoldTextRef::from(
                constants_str::WORKSPACE_SCAFFOLD_ESCAPED_CRATE_LINE,
            ),
            crate::scaffold_text_ref::ScaffoldTextRef::from(constants_str::CRATE),
        );
        assert!(result.is_ok_and(|maybe_text| {
            maybe_text.is_some_and(|text| text.as_ref() == constants_str::VALUE_B3EACD33)
        }));
    }

    #[test]
    fn test_catalog_string_rejects_invalid_escape() {
        assert!(matches!(
            crate::service_catalog_string_value::service_catalog_string_value(
                crate::scaffold_text_ref::ScaffoldTextRef::from(
                    constants_str::WORKSPACE_SCAFFOLD_INVALID_ESCAPE_CRATE_LINE,
                ),
                crate::scaffold_text_ref::ScaffoldTextRef::from(constants_str::CRATE),
            ),
            Err(crate::scaffold_error::ScaffoldError::Catalog)
        ));
    }
}
