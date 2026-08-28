#[path = "contract_error.rs"]
mod contract_error;
#[path = "ensure_json_contract_round_trip.rs"]
mod ensure_json_contract_round_trip;
#[path = "json_fixture_ref.rs"]
mod json_fixture_ref;
#[path = "serde_json_error.rs"]
mod serde_json_error;

pub use contract_error::ContractError;
pub use ensure_json_contract_round_trip::ensure_json_contract_round_trip;
pub use json_fixture_ref::JsonFixtureRef;
pub use serde_json_error::SerdeJsonError;
#[cfg(test)]
mod tests {
    #[derive(
        optimal_memory_layout::OptimalMemoryLayout,
        Debug,
        Eq,
        PartialEq,
        serde::Deserialize,
        serde::Serialize,
    )]
    struct TestValue {
        value: u8,
    }
    #[derive(
        optimal_memory_layout::OptimalMemoryLayout, Debug, Eq, PartialEq, serde::Deserialize,
    )]
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
                constants_str::INTENTIONAL_SERIALIZATION_FAILURE,
            ))
        }
    }
    #[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Eq, PartialEq)]
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
                    constants_str::ONLY_FIXTURE_VALUE_ONE_IS_ACCEPTED,
                ))
            }
        }
    }
    #[test]
    fn round_trip_and_fixture_error_phases_are_stable() {
        super::ensure_json_contract_round_trip::<TestValue>(super::JsonFixtureRef::from(
            constants_str::VALUE_1_ALT,
        ))
        .expect("7557a4b4 round_trip_and_fixture_error_phases_are_stable invariant must hold");
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
