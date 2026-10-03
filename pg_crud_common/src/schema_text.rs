pub(crate) fn schema_text(
    string: String,
) -> Result<
    crate::db_schema_text::DbSchemaText,
    crate::db_schema_conformance_error::DbSchemaConformanceError,
> {
    crate::db_schema_text::DbSchemaText::try_from(string)
        .map_err(crate::db_schema_conformance_error::DbSchemaConformanceError::SchemaTextTooLong)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_schema_text_accepts_empty_and_exact_byte_limit() {
        assert!(super::schema_text(String::new()).is_ok_and(|text| text.as_ref().is_empty()));
        let cases = [
            constants_str::X.repeat(constants_usize::VALUE_1_048_576),
            '\u{00e9}'.to_string().repeat(524_288usize),
        ];
        assert!(cases.into_iter().all(|text| {
            super::schema_text(text.clone()).is_ok_and(|schema_text| schema_text.as_ref() == text)
        }));
    }

    #[test]
    fn test_schema_text_preserves_oversized_conversion_error() {
        let mut oversized_utf8 = '\u{00e9}'.to_string().repeat(524_288usize);
        oversized_utf8.push_str(constants_str::X);
        let cases = [
            constants_str::X.repeat(constants_usize::VALUE_1_048_576 + constants_usize::ONE),
            oversized_utf8,
        ];
        assert!(cases.into_iter().all(|text| {
            let expected = crate::db_schema_text::DbSchemaText::try_from(text.clone());
            let observed = super::schema_text(text);
            matches!((expected, observed), (Err(expected_error), Err(crate::db_schema_conformance_error::DbSchemaConformanceError::SchemaTextTooLong(observed_error))) if expected_error == observed_error)
        }));
    }
}
