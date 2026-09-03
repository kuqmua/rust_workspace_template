#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    schemars::JsonSchema,
    proc_macro_newtype_utoipa_schema::UtoipaSchema,
)]
#[serde(untagged)]
#[utoipa_schema(Value)]
pub enum PatchField<Value> {
    Null,
    #[default]
    Omitted,
    Value(Value),
}
impl<Value> serde::Serialize for PatchField<Value>
where
    Value: serde::Serialize,
{
    fn serialize<Serializer>(
        &self,
        serializer: Serializer,
    ) -> Result<Serializer::Ok, Serializer::Error>
    where
        Serializer: serde::Serializer,
    {
        match self {
            Self::Omitted | Self::Null => serializer.serialize_none(),
            Self::Value(value) => value.serialize(serializer),
        }
    }
}
impl<'de, Value> serde::Deserialize<'de> for PatchField<Value>
where
    Value: serde::Deserialize<'de>,
{
    fn deserialize<Deserializer>(deserializer: Deserializer) -> Result<Self, Deserializer::Error>
    where
        Deserializer: serde::Deserializer<'de>,
    {
        Option::<Value>::deserialize(deserializer)
            .map(|value| value.map_or(Self::Null, Self::Value))
    }
}
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
    struct Patch {
        #[serde(default)]
        value: crate::patch_field::PatchField<String>,
    }

    #[test]
    fn test_deserialization_distinguishes_omitted_null_and_value() {
        assert_eq!(
            serde_json::from_str::<Patch>(constants_str::TEXT_ALT_14)
                .expect(constants_str::DIAGNOSTIC_D3E7AA4A),
            Patch {
                value: crate::patch_field::PatchField::Omitted,
            }
        );
        assert_eq!(
            serde_json::from_str::<Patch>(constants_str::VALUE_1C197DAE)
                .expect(constants_str::DIAGNOSTIC_3C55056D),
            Patch {
                value: crate::patch_field::PatchField::Null,
            }
        );
        assert_eq!(
            serde_json::from_str::<Patch>(constants_str::VALUE_ABE62BC5)
                .expect(constants_str::DIAGNOSTIC_4471155F),
            Patch {
                value: crate::patch_field::PatchField::Value(String::from(
                    constants_str::VALUE_C6C1C9A9
                )),
            }
        );
    }

    #[test]
    fn test_serialization_preserves_null_and_value_wire_shapes() {
        assert_eq!(
            serde_json::to_string(&Patch {
                value: crate::patch_field::PatchField::<String>::Null,
            })
            .expect(constants_str::DIAGNOSTIC_F2053F9C),
            constants_str::VALUE_1C197DAE
        );
        assert_eq!(
            serde_json::to_string(&Patch {
                value: crate::patch_field::PatchField::Value(String::from(
                    constants_str::VALUE_C6C1C9A9
                )),
            })
            .expect(constants_str::DIAGNOSTIC_CCCAE65F),
            constants_str::VALUE_ABE62BC5
        );
    }
}
