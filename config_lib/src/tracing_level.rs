// The owner module retains lint-sensitive semantics from the original implementation.
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(
    std::fmt::Debug,
    Default,
    Clone,
    Copy,
    strum_macros::EnumIter,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    Eq,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype::EnumFromStr,
    proc_macro_naming::EnumWithUnitFieldsToSnakeCaseStr,
)]
#[strum(serialize_all = "snake_case")]
pub enum TracingLevel {
    Trace,
    Debug,
    Info,
    Warn,
    #[default]
    Error,
}
impl TracingLevel {
    fn as_str(self) -> crate::tracing_level_name::TracingLevelName {
        crate::tracing_level_name::TracingLevelName::from(self.as_snake_case_str())
    }
}
impl std::fmt::Display for TracingLevel {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", (*self).as_str())
    }
}
