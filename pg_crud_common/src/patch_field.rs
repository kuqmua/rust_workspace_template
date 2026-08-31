#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    schemars::JsonSchema,
)]
#[serde(untagged)]
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
impl<Value> utoipa::PartialSchema for PatchField<Value>
where
    Value: utoipa::ToSchema,
{
    fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
        <Value as utoipa::PartialSchema>::schema()
    }
}
impl<Value: utoipa::ToSchema> utoipa::ToSchema for PatchField<Value> {}

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
    struct Patch {
        #[serde(default)]
        value: crate::patch_field::PatchField<String>,
    }

    #[test]
    fn test_deserialization_distinguishes_omitted_null_and_value() {
        assert_eq!(
            serde_json::from_str::<Patch>("{}").expect(
                "d3e7aa4a deserialization_distinguishes_omitted_null_and_value invariant must hold"
            ),
            Patch {
                value: crate::patch_field::PatchField::Omitted,
            }
        );
        assert_eq!(
            serde_json::from_str::<Patch>(r#"{"value":null}"#).expect(
                "3c55056d deserialization_distinguishes_omitted_null_and_value invariant must hold"
            ),
            Patch {
                value: crate::patch_field::PatchField::Null,
            }
        );
        assert_eq!(
            serde_json::from_str::<Patch>(r#"{"value":"next"}"#).expect(
                "4471155f deserialization_distinguishes_omitted_null_and_value invariant must hold"
            ),
            Patch {
                value: crate::patch_field::PatchField::Value(String::from("next")),
            }
        );
    }

    #[test]
    fn test_serialization_preserves_null_and_value_wire_shapes() {
        assert_eq!(
            serde_json::to_string(&Patch {
                value: crate::patch_field::PatchField::<String>::Null,
            })
            .expect(
                "f2053f9c serialization_preserves_null_and_value_wire_shapes invariant must hold"
            ),
            r#"{"value":null}"#
        );
        assert_eq!(
            serde_json::to_string(&Patch {
                value: crate::patch_field::PatchField::Value(String::from("next")),
            })
            .expect(
                "cccae65f serialization_preserves_null_and_value_wire_shapes invariant must hold"
            ),
            r#"{"value":"next"}"#
        );
    }
}
