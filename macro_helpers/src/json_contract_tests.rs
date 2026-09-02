#[cfg(test)]
mod tests {
    #[derive(
        proc_macro_optimal_memory_layout::OptimalMemoryLayout,
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
        proc_macro_optimal_memory_layout::OptimalMemoryLayout,
        Debug,
        Eq,
        PartialEq,
        serde::Deserialize,
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
    #[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug, Eq, PartialEq)]
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
    fn test_round_trip_and_fixture_error_phases_are_stable() {
        crate::ensure_json_contract_round_trip::ensure_json_contract_round_trip::<TestValue>(
            crate::json_fixture_ref::JsonFixtureRef::from(constants_str::VALUE_1_ALT),
        )
        .expect(constants_str::DIAGNOSTIC_7557A4B4);
        assert!(matches!(
            crate::ensure_json_contract_round_trip::ensure_json_contract_round_trip::<TestValue>(
                crate::json_fixture_ref::JsonFixtureRef::from("{")
            ),
            Err(crate::contract_error::ContractError::DeserializeFixture(_))
        ));
    }
    #[test]
    fn test_serialization_error_phase_is_stable() {
        assert!(matches!(
            crate::ensure_json_contract_round_trip::ensure_json_contract_round_trip::<SerializeFails>(
                crate::json_fixture_ref::JsonFixtureRef::from("null")
            ),
            Err(crate::contract_error::ContractError::Serialize(_))
        ));
    }
    #[test]
    fn test_round_trip_deserialization_error_phase_is_stable() {
        assert!(matches!(
            crate::ensure_json_contract_round_trip::ensure_json_contract_round_trip::<ReparseFails>(
                crate::json_fixture_ref::JsonFixtureRef::from("1")
            ),
            Err(crate::contract_error::ContractError::DeserializeRoundTrip(
                _
            ))
        ));
    }
}
