#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    utoipa::ToSchema,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub enum EncodeFormat {
    #[default]
    Base64,
    Escape,
    Hex,
}
impl std::fmt::Display for EncodeFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Base64 => write!(f, "base64"),
            Self::Escape => write!(f, "escape"),
            Self::Hex => write!(f, "hex"),
        }
    }
}
impl pg_crud_common::default_some_one_element::DefaultSomeOneElement for EncodeFormat {
    fn default_some_one_element() -> Self {
        Self::default()
    }
}
