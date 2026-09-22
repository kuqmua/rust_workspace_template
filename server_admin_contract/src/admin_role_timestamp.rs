#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
    serde::Serialize,
    proc_macro_newtype_bounded_string_wrapper::BoundedStringWrapper,
    proc_macro_newtype_display::Display,
)]
#[bounded_string(max = 64, chars, utoipa, description = "administrator role timestamp")]
pub struct AdminRoleTimestamp(bounded_types::bounded_string::BoundedString<0usize, 64, true>);

impl<'de> serde::Deserialize<'de> for AdminRoleTimestamp {
    fn deserialize<Deserializer>(deserializer: Deserializer) -> Result<Self, Deserializer::Error>
    where
        Deserializer: serde::Deserializer<'de>,
    {
        let value = <serde_json::Value as serde::Deserialize>::deserialize(deserializer)?;
        let timestamp = if let Some(timestamp) = value.as_str() {
            timestamp.to_owned()
        } else {
            let object = value.as_object().ok_or_else(|| {
                <Deserializer::Error as serde::de::Error>::custom(
                    constants_str::ADMIN_UI_THE_TABLE_RESPONSE_WAS_INVALID,
                )
            })?;
            let date = object
                .get(constants_str::DATE_NAIVE)
                .and_then(serde_json::Value::as_str)
                .ok_or_else(|| {
                    <Deserializer::Error as serde::de::Error>::custom(
                        constants_str::ADMIN_UI_THE_TABLE_RESPONSE_WAS_INVALID,
                    )
                })?;
            let time = object
                .get(constants_str::PG_CRUD_PG_TIME)
                .and_then(serde_json::Value::as_object)
                .ok_or_else(|| {
                    <Deserializer::Error as serde::de::Error>::custom(
                        constants_str::ADMIN_UI_THE_TABLE_RESPONSE_WAS_INVALID,
                    )
                })?;
            let field = |name| {
                time.get(name)
                    .and_then(serde_json::Value::as_u64)
                    .ok_or_else(|| {
                        <Deserializer::Error as serde::de::Error>::custom(
                            constants_str::ADMIN_UI_THE_TABLE_RESPONSE_WAS_INVALID,
                        )
                    })
            };
            let hour = field(constants_str::HOUR)?;
            let minute = field(constants_str::MIN)?;
            let second = field(constants_str::SEC)?;
            let microsecond = field(constants_str::MICRO)?;
            let fraction = format!("{microsecond:06}").trim_end_matches('0').to_owned();
            if fraction.is_empty() {
                format!("{date}T{hour:02}:{minute:02}:{second:02}")
            } else {
                format!("{date}T{hour:02}:{minute:02}:{second:02}.{fraction}")
            }
        };
        Self::try_from(timestamp)
            .map_err(|error| <Deserializer::Error as serde::de::Error>::custom(error.to_string()))
    }
}
