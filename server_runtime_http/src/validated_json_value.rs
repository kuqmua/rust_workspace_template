#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
pub(crate) enum ValidatedJsonValue {
    Valid,
}

impl<'de> serde::Deserialize<'de> for ValidatedJsonValue {
    fn deserialize<Deserializer>(deserializer: Deserializer) -> Result<Self, Deserializer::Error>
    where
        Deserializer: serde::Deserializer<'de>,
    {
        deserializer
            .deserialize_any(crate::json_validation_visitor::JsonValidationVisitor::Document)
    }
}
