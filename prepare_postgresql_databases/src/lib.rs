#[derive(optml::Optml, Clone, Debug, Eq, PartialEq, newtype::AsRefStr, newtype::TryFrom)]
#[try_from(validator = validate_database_url)]
pub struct DatabaseUrl(String);

#[derive(optml::Optml, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum DatabaseUrlError {
    #[error("{0}", str_constants::DATABASE_URL_MUST_NOT_BE_EMPTY)]
    Empty,
    #[error("{0}", str_constants::DATABASE_URL_EXCEEDS_MAXIMUM_LENGTH)]
    TooLong,
}

#[derive(optml::Optml, Clone, Debug, Eq, PartialEq, newtype::AsRefStr, newtype::TryFrom)]
#[try_from(validator = validate_migrations_source)]
pub struct MigrationsSource(String);

#[derive(optml::Optml, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum MigrationsSourceError {
    #[error("{0}", str_constants::MIGRATIONS_SOURCE_EXCEEDS_MAXIMUM_LENGTH)]
    TooLong,
}

#[derive(optml::Optml, Clone, Debug, Eq, PartialEq)]
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

#[derive(optml::Optml, Clone, Debug, Eq, PartialEq)]
pub struct ProcessCommand {
    arguments: ProcessArguments,
    program: ProcessProgram,
}

#[derive(optml::Optml, Clone, Debug, Eq, PartialEq)]
pub enum ProcessArgument {
    DatabaseUrl(DatabaseUrl),
    MigrationsSource(MigrationsSource),
    Static(ProcessStaticArgument),
}

#[derive(optml::Optml, Clone, Copy, Debug, Eq, PartialEq, newtype::FromInner)]
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

#[derive(optml::Optml, Clone, Debug, Eq, PartialEq, newtype::AsRefTarget, newtype::FromInner)]
pub struct ProcessArguments(bounded_types::BoundedVec<ProcessArgument, 0, { usize::MAX }>);
#[derive(optml::Optml, Clone, Debug, Eq, PartialEq, newtype::AsRefTarget, newtype::FromInner)]
pub struct ProcessCommands(bounded_types::BoundedVec<ProcessCommand, 0, { usize::MAX }>);

#[derive(
    optml::Optml, Clone, Copy, Debug, Eq, PartialEq, newtype::AsRefInner, newtype::FromInner,
)]
pub struct ProcessProgram(&'static str);

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

#[allow(clippy::single_call_fn)] // named validation boundary is consumed by the Newtype derive
fn validate_database_url<Value>(value: &Value) -> Result<(), DatabaseUrlError>
where
    Value: AsRef<str>,
{
    let value_ref = value.as_ref();
    if value_ref.trim().is_empty() {
        Err(DatabaseUrlError::Empty)
    } else if value_ref.len() > 8_192usize {
        Err(DatabaseUrlError::TooLong)
    } else {
        Ok(())
    }
}

#[allow(clippy::single_call_fn)] // named validation boundary is consumed by the Newtype derive
fn validate_migrations_source<Value>(value: &Value) -> Result<(), MigrationsSourceError>
where
    Value: AsRef<str>,
{
    if value.as_ref().len() > 4_096usize {
        Err(MigrationsSourceError::TooLong)
    } else {
        Ok(())
    }
}

#[must_use]
pub fn migration_commands<Specifications>(specs: Specifications) -> ProcessCommands
where
    Specifications: IntoIterator<Item = DatabasePreparationSpec>,
{
    ProcessCommands::from(bounded_types::BoundedVec::from_max_iter(
        specs.into_iter().map(|spec| ProcessCommand {
            arguments: ProcessArguments::from(bounded_types::BoundedVec::from_max_iter([
                ProcessArgument::from(str_constants::DATABASE_URL_FLAG),
                ProcessArgument::from(spec.url),
                ProcessArgument::from(str_constants::SOURCE_FLAG),
                ProcessArgument::from(spec.migrations_source),
                ProcessArgument::from(str_constants::RUN),
            ])),
            program: ProcessProgram::from(str_constants::SQLX),
        }),
    ))
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
        assert_eq!(commands.as_ref().len(), 1usize);
        let command = commands
            .as_ref()
            .first()
            .expect("989c8d37 builds_one_migration_command_per_database invariant must hold");
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
