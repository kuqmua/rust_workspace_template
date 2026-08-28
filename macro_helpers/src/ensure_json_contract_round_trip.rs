pub fn ensure_json_contract_round_trip<Value>(
    fixture: super::JsonFixtureRef<'_>,
) -> Result<(), super::ContractError>
where
    Value: Eq + serde::Serialize + serde::de::DeserializeOwned,
{
    let fixture_value = serde_json::from_str::<Value>(fixture.0).map_err(|error| {
        super::ContractError::DeserializeFixture(super::SerdeJsonError::from(error))
    })?;
    let serialized = serde_json::to_string(&fixture_value)
        .map_err(|error| super::ContractError::Serialize(super::SerdeJsonError::from(error)))?;
    let round_trip_value = serde_json::from_str::<Value>(serialized.as_str()).map_err(|error| {
        super::ContractError::DeserializeRoundTrip(super::SerdeJsonError::from(error))
    })?;
    if fixture_value == round_trip_value {
        Ok(())
    } else {
        Err(super::ContractError::ValueMismatch)
    }
}
