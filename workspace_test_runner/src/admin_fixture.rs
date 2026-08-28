pub(crate) use crate::admin_fixture_string::AdminFixtureString;
#[cfg(test)]
pub(crate) use crate::admin_fixture_string::AdminFixtureStringTryFromStringError;
pub(crate) use crate::create_admin_fixture_string::create_admin_fixture_string;

#[cfg(test)]
mod tests {
    #[test]
    fn fixture_text_enforces_the_owned_bound() {
        let oversized = String::from(constants_str::A_ALT)
            .repeat(constants_usize::VALUE_1_048_576.saturating_add(constants_usize::ONE));
        assert!(matches!(
            super::AdminFixtureString::try_from(oversized),
            Err(super::AdminFixtureStringTryFromStringError::TooLong { .. })
        ));
    }
}
