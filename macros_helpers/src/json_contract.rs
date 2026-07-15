#[derive(Clone, Copy, Debug)]
pub struct JsonFixtureRef<'fixture_lt>(&'fixture_lt str);
impl<'fixture_lt> From<&'fixture_lt str> for JsonFixtureRef<'fixture_lt> {
    fn from(value: &'fixture_lt str) -> Self {
        Self(value)
    }
}
#[derive(Debug)]
pub struct SerdeJsonError(serde_json::Error);
impl std::fmt::Display for SerdeJsonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl std::error::Error for SerdeJsonError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.0)
    }
}
#[derive(Debug, thiserror::Error)]
pub enum ContractError {
    #[error("fixture JSON deserialization failed: {0}")]
    DeserializeFixture(SerdeJsonError),
    #[error("round-trip JSON deserialization failed: {0}")]
    DeserializeRoundTrip(SerdeJsonError),
    #[error("JSON serialization failed: {0}")]
    Serialize(SerdeJsonError),
    #[error("round-trip value differs from fixture value")]
    ValueMismatch,
}
pub fn ensure_json_contract_round_trip<Value>(
    fixture: JsonFixtureRef<'_>,
) -> Result<(), ContractError>
where
    Value: Eq + serde::Serialize + serde::de::DeserializeOwned,
{
    let fixture_value = serde_json::from_str::<Value>(fixture.0)
        .map_err(|error| ContractError::DeserializeFixture(SerdeJsonError(error)))?;
    let serialized = serde_json::to_string(&fixture_value)
        .map_err(|error| ContractError::Serialize(SerdeJsonError(error)))?;
    let round_trip_value = serde_json::from_str::<Value>(serialized.as_str())
        .map_err(|error| ContractError::DeserializeRoundTrip(SerdeJsonError(error)))?;
    if fixture_value == round_trip_value {
        Ok(())
    } else {
        Err(ContractError::ValueMismatch)
    }
}
#[cfg(test)]
mod tests {
    #[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    struct TestValue {
        value: u8,
    }
    #[derive(Debug, Eq, PartialEq, serde::Deserialize)]
    struct SerializeFails;
    impl serde::Serialize for SerializeFails {
        fn serialize<Serializer>(
            &self,
            _serializer: Serializer,
        ) -> Result<Serializer::Ok, Serializer::Error>
        where
            Serializer: serde::Serializer,
        {
            Err(serde::ser::Error::custom(
                str_constants::text::INTENTIONAL_SERIALIZATION_FAILURE,
            ))
        }
    }
    #[derive(Debug, Eq, PartialEq)]
    struct ReparseFails;
    impl serde::Serialize for ReparseFails {
        fn serialize<Serializer>(
            &self,
            serializer: Serializer,
        ) -> Result<Serializer::Ok, Serializer::Error>
        where
            Serializer: serde::Serializer,
        {
            serializer.serialize_u8(2u8)
        }
    }
    impl<'de> serde::Deserialize<'de> for ReparseFails {
        fn deserialize<Deserializer>(
            deserializer: Deserializer,
        ) -> Result<Self, Deserializer::Error>
        where
            Deserializer: serde::Deserializer<'de>,
        {
            let value = <u8 as serde::Deserialize>::deserialize(deserializer)?;
            if value == 1u8 {
                Ok(Self)
            } else {
                Err(serde::de::Error::custom(
                    str_constants::text::ONLY_FIXTURE_VALUE_ONE_IS_ACCEPTED,
                ))
            }
        }
    }
    #[test]
    fn round_trip_and_fixture_error_phases_are_stable() {
        super::ensure_json_contract_round_trip::<TestValue>(super::JsonFixtureRef::from(
            str_constants::text::VALUE_1_ALT,
        ))
        .expect("7557a4b4");
        assert!(matches!(
            super::ensure_json_contract_round_trip::<TestValue>(super::JsonFixtureRef::from("{")),
            Err(super::ContractError::DeserializeFixture(_))
        ));
    }
    #[test]
    fn serialization_error_phase_is_stable() {
        assert!(matches!(
            super::ensure_json_contract_round_trip::<SerializeFails>(super::JsonFixtureRef::from(
                "null"
            )),
            Err(super::ContractError::Serialize(_))
        ));
    }
    #[test]
    fn round_trip_deserialization_error_phase_is_stable() {
        assert!(matches!(
            super::ensure_json_contract_round_trip::<ReparseFails>(super::JsonFixtureRef::from(
                "1"
            )),
            Err(super::ContractError::DeserializeRoundTrip(_))
        ));
    }
}
