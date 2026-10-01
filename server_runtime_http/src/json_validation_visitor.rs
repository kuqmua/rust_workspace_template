#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
pub(crate) enum JsonValidationVisitor {
    Document,
}

impl<'de> serde::de::Visitor<'de> for JsonValidationVisitor {
    type Value = crate::validated_json_value::ValidatedJsonValue;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(constants_str::JSON)
    }

    fn visit_bool<Error>(self, _bool: bool) -> Result<Self::Value, Error>
    where
        Error: serde::de::Error,
    {
        Ok(crate::validated_json_value::ValidatedJsonValue::Valid)
    }

    fn visit_i64<Error>(self, _i64: i64) -> Result<Self::Value, Error>
    where
        Error: serde::de::Error,
    {
        Ok(crate::validated_json_value::ValidatedJsonValue::Valid)
    }

    fn visit_u64<Error>(self, _u64: u64) -> Result<Self::Value, Error>
    where
        Error: serde::de::Error,
    {
        Ok(crate::validated_json_value::ValidatedJsonValue::Valid)
    }

    fn visit_f64<Error>(self, _f64: f64) -> Result<Self::Value, Error>
    where
        Error: serde::de::Error,
    {
        Ok(crate::validated_json_value::ValidatedJsonValue::Valid)
    }

    fn visit_str<Error>(self, _str: &str) -> Result<Self::Value, Error>
    where
        Error: serde::de::Error,
    {
        Ok(crate::validated_json_value::ValidatedJsonValue::Valid)
    }

    fn visit_unit<Error>(self) -> Result<Self::Value, Error>
    where
        Error: serde::de::Error,
    {
        Ok(crate::validated_json_value::ValidatedJsonValue::Valid)
    }

    fn visit_seq<Access>(self, mut access: Access) -> Result<Self::Value, Access::Error>
    where
        Access: serde::de::SeqAccess<'de>,
    {
        loop {
            if access
                .next_element::<crate::validated_json_value::ValidatedJsonValue>()?
                .is_none()
            {
                return Ok(crate::validated_json_value::ValidatedJsonValue::Valid);
            }
        }
    }

    fn visit_map<Access>(self, mut access: Access) -> Result<Self::Value, Access::Error>
    where
        Access: serde::de::MapAccess<'de>,
    {
        loop {
            if access.next_entry::<crate::validated_json_value::ValidatedJsonValue, crate::validated_json_value::ValidatedJsonValue>()?.is_none() {
                return Ok(crate::validated_json_value::ValidatedJsonValue::Valid);
            }
        }
    }
}
