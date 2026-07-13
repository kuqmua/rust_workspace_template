#[derive(Clone, Copy, Debug)]
pub struct JsonFixtureRef<'fixture_lt>(&'fixture_lt str);
impl<'fixture_lt> From<&'fixture_lt str> for JsonFixtureRef<'fixture_lt> {
    fn from(value: &'fixture_lt str) -> Self {
        Self(value)
    }
}
#[derive(Debug)]
pub struct SerdeJsonEr(serde_json::Error);
impl std::fmt::Display for SerdeJsonEr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl std::error::Error for SerdeJsonEr {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.0)
    }
}
#[derive(Debug, thiserror::Error)]
pub enum ContractEr {
    #[error("fixture JSON deserialization failed: {0}")]
    DeserializeFixture(SerdeJsonEr),
    #[error("round-trip JSON deserialization failed: {0}")]
    DeserializeRoundTrip(SerdeJsonEr),
    #[error("JSON serialization failed: {0}")]
    Serialize(SerdeJsonEr),
    #[error("round-trip value differs from fixture value")]
    ValueMismatch,
}
pub fn ensure_json_contract_round_trip<Value>(fixture: JsonFixtureRef<'_>) -> Result<(), ContractEr>
where
    Value: Eq + serde::Serialize + serde::de::DeserializeOwned,
{
    let fixture_value = serde_json::from_str::<Value>(fixture.0)
        .map_err(|error| ContractEr::DeserializeFixture(SerdeJsonEr(error)))?;
    let serialized = serde_json::to_string(&fixture_value)
        .map_err(|error| ContractEr::Serialize(SerdeJsonEr(error)))?;
    let round_trip_value = serde_json::from_str::<Value>(serialized.as_str())
        .map_err(|error| ContractEr::DeserializeRoundTrip(SerdeJsonEr(error)))?;
    if fixture_value == round_trip_value {
        Ok(())
    } else {
        Err(ContractEr::ValueMismatch)
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
                "intentional serialization failure",
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
                    "only fixture value one is accepted",
                ))
            }
        }
    }
    #[test]
    fn round_trip_and_fixture_error_phases_are_stable() {
        super::ensure_json_contract_round_trip::<TestValue>(super::JsonFixtureRef::from(
            r#"{"value":1}"#,
        ))
        .expect("7557a4b4");
        assert!(matches!(
            super::ensure_json_contract_round_trip::<TestValue>(super::JsonFixtureRef::from("{")),
            Err(super::ContractEr::DeserializeFixture(_))
        ));
    }
    #[test]
    fn serialization_error_phase_is_stable() {
        assert!(matches!(
            super::ensure_json_contract_round_trip::<SerializeFails>(super::JsonFixtureRef::from(
                "null"
            )),
            Err(super::ContractEr::Serialize(_))
        ));
    }
    #[test]
    fn round_trip_deserialization_error_phase_is_stable() {
        assert!(matches!(
            super::ensure_json_contract_round_trip::<ReparseFails>(super::JsonFixtureRef::from(
                "1"
            )),
            Err(super::ContractEr::DeserializeRoundTrip(_))
        ));
    }
}
