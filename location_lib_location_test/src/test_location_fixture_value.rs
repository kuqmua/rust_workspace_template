#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
pub(crate) enum TestLocationFixtureValue {
    Absent,
    Coordinate,
    Duration,
    Text(crate::location_test_text::LocationTestText),
}

impl serde::de::IntoDeserializer<'_, serde::de::value::Error> for TestLocationFixtureValue {
    type Deserializer = Self;

    fn into_deserializer(self) -> Self::Deserializer {
        self
    }
}

impl<'de> serde::Deserializer<'de> for TestLocationFixtureValue {
    type Error = serde::de::value::Error;

    fn deserialize_any<Visitor>(self, visitor: Visitor) -> Result<Visitor::Value, Self::Error>
    where
        Visitor: serde::de::Visitor<'de>,
    {
        match self {
            Self::Text(location_test_text) => visitor.visit_str(
                to_err_string::to_err_string::ToErrString::to_err_string(&location_test_text)
                    .as_ref(),
            ),
            Self::Coordinate => visitor.visit_u32(1u32),
            Self::Duration => visitor.visit_map(serde::de::value::MapDeserializer::new(
                [(constants_str::SECS, 0u64), (constants_str::NANOS, 0u64)].into_iter(),
            )),
            Self::Absent => visitor.visit_none(),
        }
    }

    serde::forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string bytes
        byte_buf option unit unit_struct newtype_struct seq tuple tuple_struct map struct
        enum identifier ignored_any
    }
}
