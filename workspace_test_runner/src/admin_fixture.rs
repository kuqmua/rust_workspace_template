#[cfg(test)]
mod tests {
    #[test]
    fn test_fixture_text_enforces_the_owned_bound() {
        let oversized = String::from(constants_str::A_ALT)
            .repeat(constants_usize::VALUE_1_048_576.saturating_add(constants_usize::ONE));
        assert!(matches!(
            crate::admin_fixture_string::AdminFixtureString::try_from(oversized),
            Err(crate::admin_fixture_string::AdminFixtureStringTryFromStringError::TooLong { .. })
        ));
    }
}
