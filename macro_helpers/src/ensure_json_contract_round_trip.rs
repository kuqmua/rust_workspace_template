pub fn ensure_json_contract_round_trip<Value>(
    fixture: crate::json_fixture_ref::JsonFixtureRef<'_>,
) -> Result<(), crate::contract_error::ContractError>
where
    Value: Eq + serde::Serialize + serde::de::DeserializeOwned,
{
    let fixture_value = serde_json::from_str::<Value>(fixture.0).map_err(|error| {
        crate::contract_error::ContractError::DeserializeFixture(
            crate::macro_serde_json_error::MacroSerdeJsonError::from(error),
        )
    })?;
    let serialized = serde_json::to_string(&fixture_value).map_err(|error| {
        crate::contract_error::ContractError::Serialize(
            crate::macro_serde_json_error::MacroSerdeJsonError::from(error),
        )
    })?;
    let round_trip_value = serde_json::from_str::<Value>(serialized.as_str()).map_err(|error| {
        crate::contract_error::ContractError::DeserializeRoundTrip(
            crate::macro_serde_json_error::MacroSerdeJsonError::from(error),
        )
    })?;
    if fixture_value == round_trip_value {
        Ok(())
    } else {
        Err(crate::contract_error::ContractError::ValueMismatch)
    }
}
