#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub enum ProcessArgument {
    DatabaseUrl(super::DatabaseUrl),
    MigrationsSource(super::MigrationsSource),
    Static(super::ProcessStaticArgument),
}

impl From<super::DatabaseUrl> for ProcessArgument {
    fn from(value: super::DatabaseUrl) -> Self {
        Self::DatabaseUrl(value)
    }
}

impl From<super::MigrationsSource> for ProcessArgument {
    fn from(value: super::MigrationsSource) -> Self {
        Self::MigrationsSource(value)
    }
}

impl From<&'static str> for ProcessArgument {
    fn from(value: &'static str) -> Self {
        Self::Static(super::ProcessStaticArgument::from(value))
    }
}

impl AsRef<str> for ProcessArgument {
    fn as_ref(&self) -> &str {
        match self {
            Self::DatabaseUrl(value) => value.as_ref(),
            Self::MigrationsSource(value) => value.as_ref(),
            Self::Static(value) => value.0,
        }
    }
}
