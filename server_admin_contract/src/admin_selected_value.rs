#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, serde::Deserialize)]
pub(crate) struct AdminSelectedValue<Value> {
    value: Value,
}
impl<Value> AdminSelectedValue<Value> {
    pub(crate) fn into_value(self) -> Value {
        self.value
    }
}
