#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub enum ProcessArgument {
    DatabaseUrl(crate::database_url::DatabaseUrl),
    MigrationsSource(crate::migrations_source::MigrationsSource),
    Static(crate::process_static_argument::ProcessStaticArgument),
}

impl From<crate::database_url::DatabaseUrl> for ProcessArgument {
    fn from(value: crate::database_url::DatabaseUrl) -> Self {
        Self::DatabaseUrl(value)
    }
}

impl From<crate::migrations_source::MigrationsSource> for ProcessArgument {
    fn from(value: crate::migrations_source::MigrationsSource) -> Self {
        Self::MigrationsSource(value)
    }
}

impl From<&'static str> for ProcessArgument {
    fn from(value: &'static str) -> Self {
        Self::Static(crate::process_static_argument::ProcessStaticArgument::from(
            value,
        ))
    }
}

impl AsRef<str> for ProcessArgument {
    fn as_ref(&self) -> &str {
        match self {
            Self::DatabaseUrl(value) => value.as_ref(),
            Self::MigrationsSource(value) => value.as_ref(),
            Self::Static(value) => value.get(),
        }
    }
}
