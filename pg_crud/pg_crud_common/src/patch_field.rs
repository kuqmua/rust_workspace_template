#[derive(optml::Optml, Clone, Debug, Default, Eq, PartialEq, schemars::JsonSchema)]
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
    #[derive(optml::Optml, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    struct Patch {
        #[serde(default)]
        value: super::PatchField<String>,
    }

    #[test]
    fn deserialization_distinguishes_omitted_null_and_value() {
        assert_eq!(
            serde_json::from_str::<Patch>("{}").expect("d3e7aa4a"),
            Patch {
                value: super::PatchField::Omitted,
            }
        );
        assert_eq!(
            serde_json::from_str::<Patch>(r#"{"value":null}"#).expect("3c55056d"),
            Patch {
                value: super::PatchField::Null,
            }
        );
        assert_eq!(
            serde_json::from_str::<Patch>(r#"{"value":"next"}"#).expect("4471155f"),
            Patch {
                value: super::PatchField::Value(String::from("next")),
            }
        );
    }

    #[test]
    fn serialization_preserves_null_and_value_wire_shapes() {
        assert_eq!(
            serde_json::to_string(&Patch {
                value: super::PatchField::<String>::Null,
            })
            .expect("f2053f9c"),
            r#"{"value":null}"#
        );
        assert_eq!(
            serde_json::to_string(&Patch {
                value: super::PatchField::Value(String::from("next")),
            })
            .expect("cccae65f"),
            r#"{"value":"next"}"#
        );
    }
}
