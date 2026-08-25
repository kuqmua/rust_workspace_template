#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::BoundedString)]
#[bounded_string(max = constants_usize::VALUE_1_048_576)]
pub(crate) struct AdminFixtureString(String);
pub(crate) fn admin_fixture_string<Value>(
    value: impl TryInto<AdminFixtureString>,
) -> Result<Value, ()>
where
    Value: TryFrom<String>,
    Value::Error: std::fmt::Display,
{
    let bounded_value = value.try_into().map_err(|_error| {
        eprintln!(
            "{}",
            constants_str::WORKSPACE_TEST_RUNNER_ADMIN_FIXTURE_STRING_INVALID
        );
    })?;
    Value::try_from(bounded_value.0).map_err(|error| {
        eprintln!("{error}");
    })
}

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
