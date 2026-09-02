#[derive(Debug, proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, PartialEq, Eq)]
pub struct AdminFieldLabel(Box<str>);

impl AdminFieldLabel {
    pub(super) fn into_inner(self) -> Box<str> {
        self.0
    }
}

impl From<&'static str> for AdminFieldLabel {
    fn from(str: &'static str) -> Self {
        Self(Box::<str>::from(str))
    }
}

impl From<String> for AdminFieldLabel {
    fn from(string: String) -> Self {
        Self(string.into_boxed_str())
    }
}

impl AsRef<str> for AdminFieldLabel {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
