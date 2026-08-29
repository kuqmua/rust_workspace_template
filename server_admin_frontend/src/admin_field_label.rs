#[derive(Debug, optimal_memory_layout::OptimalMemoryLayout, Clone, PartialEq, Eq)]
pub struct AdminFieldLabel(Box<str>);

impl AdminFieldLabel {
    pub(super) fn into_inner(self) -> Box<str> {
        self.0
    }
}

impl From<&'static str> for AdminFieldLabel {
    fn from(value: &'static str) -> Self {
        Self(Box::<str>::from(value))
    }
}

impl From<String> for AdminFieldLabel {
    fn from(value: String) -> Self {
        Self(value.into_boxed_str())
    }
}

impl AsRef<str> for AdminFieldLabel {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
