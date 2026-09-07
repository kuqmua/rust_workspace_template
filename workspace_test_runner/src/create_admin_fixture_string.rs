pub(crate) fn create_admin_fixture_string<Value>(
    admin_fixture_string: impl TryInto<
        crate::admin_fixture_string::AdminFixtureString,
        Error = crate::admin_fixture_string::AdminFixtureStringTryFromStringError,
    >,
) -> Result<Value, crate::admin_fixture_conversion_error::AdminFixtureConversionError>
where
    Value: TryFrom<String>,
    Value::Error: Into<crate::admin_fixture_conversion_error::AdminFixtureConversionError>,
{
    let bounded_value = admin_fixture_string
        .try_into()
        .map_err(crate::admin_fixture_conversion_error::AdminFixtureConversionError::Input)?;
    Value::try_from(String::from(bounded_value)).map_err(Into::into)
}
