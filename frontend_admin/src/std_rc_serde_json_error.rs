#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, thiserror::Error)]
#[error(transparent)]
pub(crate) struct StdRcSerdeJsonError(std::rc::Rc<serde_json::Error>);

impl From<serde_json::Error> for StdRcSerdeJsonError {
    fn from(value: serde_json::Error) -> Self {
        Self(std::rc::Rc::new(value))
    }
}
