#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub enum ProcessArgument {
    DatabaseUrl(crate::domain_types::DatabaseUrl),
    MigrationsSource(crate::domain_types::MigrationsSource),
    Static(crate::domain_types::ProcessStaticArgument),
}

impl From<crate::domain_types::DatabaseUrl> for ProcessArgument {
    fn from(value: crate::domain_types::DatabaseUrl) -> Self {
        Self::DatabaseUrl(value)
    }
}

impl From<crate::domain_types::MigrationsSource> for ProcessArgument {
    fn from(value: crate::domain_types::MigrationsSource) -> Self {
        Self::MigrationsSource(value)
    }
}

impl From<&'static str> for ProcessArgument {
    fn from(value: &'static str) -> Self {
        Self::Static(crate::domain_types::ProcessStaticArgument::from(value))
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
