use super::TracingLevelName;

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
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::EnumFromStr,
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
    fn as_str(self) -> TracingLevelName {
        TracingLevelName::from(match self {
            Self::Trace => constants_str::CONFIG_TRACING_TRACE,
            Self::Debug => constants_str::CONFIG_TRACING_DEBUG,
            Self::Info => constants_str::CONFIG_TRACING_INFO,
            Self::Warn => constants_str::CONFIG_TRACING_WARN,
            Self::Error => constants_str::CONFIG_TRACING_ERROR,
        })
    }
}
impl std::fmt::Display for TracingLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", (*self).as_str().0)
    }
}
