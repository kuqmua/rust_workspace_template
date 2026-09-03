#[derive(
    Debug,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    PartialEq,
    Eq,
    proc_macro_newtype_into_inner::IntoInner,
)]
pub struct AdminFieldLabel(Box<str>);

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
