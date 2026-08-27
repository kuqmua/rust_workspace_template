#[path = "admin_fixture/admin_fixture_string.rs"]
mod admin_fixture_string;

pub(crate) use admin_fixture_string::admin_fixture_string;
#[cfg(test)]
use admin_fixture_string::{AdminFixtureString, AdminFixtureStringTryFromStringError};

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
