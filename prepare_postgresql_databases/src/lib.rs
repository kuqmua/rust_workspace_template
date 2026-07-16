#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DatabaseUrl(String);

impl AsRef<str> for DatabaseUrl {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum DatabaseUrlError {
    #[error("{0}", str_constants::DATABASE_URL_MUST_NOT_BE_EMPTY)]
    Empty,
    #[error("{0}", str_constants::DATABASE_URL_EXCEEDS_MAXIMUM_LENGTH)]
    TooLong,
}

impl TryFrom<String> for DatabaseUrl {
    type Error = DatabaseUrlError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.trim().is_empty() {
            Err(Self::Error::Empty)
        } else if value.len() > 8_192usize {
            Err(Self::Error::TooLong)
        } else {
            Ok(Self(value))
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MigrationsSource(String);

impl AsRef<str> for MigrationsSource {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum MigrationsSourceError {
    #[error("{0}", str_constants::MIGRATIONS_SOURCE_EXCEEDS_MAXIMUM_LENGTH)]
    TooLong,
}

impl TryFrom<String> for MigrationsSource {
    type Error = MigrationsSourceError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > 4_096usize {
            Err(Self::Error::TooLong)
        } else {
            Ok(Self(value))
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DatabasePreparationSpec {
    migrations_source: MigrationsSource,
    url: DatabaseUrl,
}

impl DatabasePreparationSpec {
    #[must_use]
    pub const fn new(url: DatabaseUrl, migrations_source: MigrationsSource) -> Self {
        Self {
            migrations_source,
            url,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProcessCommand {
    arguments: ProcessArguments,
    program: ProcessProgram,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProcessArgument {
    DatabaseUrl(DatabaseUrl),
    MigrationsSource(MigrationsSource),
    Static(ProcessStaticArgument),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ProcessStaticArgument(&'static str);

impl From<DatabaseUrl> for ProcessArgument {
    fn from(value: DatabaseUrl) -> Self {
        Self::DatabaseUrl(value)
    }
}

impl From<MigrationsSource> for ProcessArgument {
    fn from(value: MigrationsSource) -> Self {
        Self::MigrationsSource(value)
    }
}

impl From<&'static str> for ProcessArgument {
    fn from(value: &'static str) -> Self {
        Self::Static(ProcessStaticArgument(value))
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProcessArguments(Vec<ProcessArgument>);

impl AsRef<[ProcessArgument]> for ProcessArguments {
    fn as_ref(&self) -> &[ProcessArgument] {
        self.0.as_slice()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ProcessProgram(&'static str);

impl AsRef<str> for ProcessProgram {
    fn as_ref(&self) -> &str {
        self.0
    }
}

impl ProcessCommand {
    #[must_use]
    pub const fn arguments(&self) -> &ProcessArguments {
        &self.arguments
    }

    #[must_use]
    pub const fn program(&self) -> ProcessProgram {
        self.program
    }
}

#[must_use]
pub fn migration_commands<Specifications>(specs: Specifications) -> Vec<ProcessCommand>
where
    Specifications: IntoIterator<Item = DatabasePreparationSpec>,
{
    specs
        .into_iter()
        .map(|spec| ProcessCommand {
            arguments: ProcessArguments(vec![
                ProcessArgument::from(str_constants::DATABASE_URL_FLAG),
                ProcessArgument::from(spec.url),
                ProcessArgument::from(str_constants::SOURCE_FLAG),
                ProcessArgument::from(spec.migrations_source),
                ProcessArgument::from(str_constants::RUN),
            ]),
            program: ProcessProgram(str_constants::SQLX),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn builds_one_migration_command_per_database() {
        let url = super::DatabaseUrl::try_from(str_constants::TEST_DATABASE_URL.to_owned());
        let source =
            super::MigrationsSource::try_from(str_constants::TEST_MIGRATIONS_PATH.to_owned());
        assert!(url.is_ok());
        assert!(source.is_ok());
        let commands = super::migration_commands(url.into_iter().zip(source).map(
            |(valid_url, valid_source)| {
                super::DatabasePreparationSpec::new(valid_url, valid_source)
            },
        ));
        assert_eq!(commands.len(), 1usize);
        let command = commands.first().expect("989c8d37");
        assert_eq!(command.program().as_ref(), str_constants::SQLX);
        assert_eq!(command.arguments().as_ref().len(), 5usize);
    }

    #[test]
    fn rejects_empty_database_url() {
        assert_eq!(
            super::DatabaseUrl::try_from(String::new()),
            Err(super::DatabaseUrlError::Empty)
        );
    }
}
