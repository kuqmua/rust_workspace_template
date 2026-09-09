#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, serde::Deserialize)]
#[serde(untagged)]
pub(crate) enum AdminReadField<Value> {
    Selected(crate::admin_selected_value::AdminSelectedValue<Value>),
    Plain(Value),
}
impl<'de, Value> AdminReadField<Value>
where
    Value: serde::Deserialize<'de>,
{
    pub(crate) fn deserialize_value<Deserializer>(
        deserializer: Deserializer,
    ) -> Result<Value, Deserializer::Error>
    where
        Deserializer: serde::Deserializer<'de>,
    {
        <Self as serde::Deserialize>::deserialize(deserializer).map(|field| match field {
            Self::Selected(selected) => selected.into_value(),
            Self::Plain(value) => value,
        })
    }
}
