pub(crate) fn create_admin_fixture_string<Value>(
    value: impl TryInto<crate::admin_fixture_string::AdminFixtureString>,
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
    Value::try_from(String::from(bounded_value)).map_err(|error| {
        eprintln!("{error}");
    })
}
