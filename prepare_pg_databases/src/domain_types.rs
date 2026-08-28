pub use crate::database_preparation_spec::DatabasePreparationSpec;
pub use crate::database_url::DatabaseUrl;
pub use crate::database_url_error::DatabaseUrlError;
pub use crate::migration_commands::migration_commands;
pub use crate::migrations_source::MigrationsSource;
pub use crate::migrations_source_error::MigrationsSourceError;
pub use crate::process_argument::ProcessArgument;
pub use crate::process_arguments::ProcessArguments;
pub use crate::process_command::ProcessCommand;
pub use crate::process_commands::ProcessCommands;
pub use crate::process_program::ProcessProgram;
pub use crate::process_static_argument::ProcessStaticArgument;
pub(crate) use crate::validate_database_url::validate_database_url;
pub(crate) use crate::validate_migrations_source::validate_migrations_source;

#[cfg(test)]
mod tests {
    #[test]
    fn builds_one_migration_command_per_database() {
        let url = super::DatabaseUrl::try_from(constants_str::TEST_DATABASE_URL.to_owned());
        let source =
            super::MigrationsSource::try_from(constants_str::TEST_MIGRATIONS_PATH.to_owned());
        assert!(url.is_ok());
        assert!(source.is_ok());
        let commands = super::migration_commands(url.into_iter().zip(source).map(
            |(valid_url, valid_source)| {
                super::DatabasePreparationSpec::new(valid_url, valid_source)
            },
        ));
        assert_eq!(commands.as_ref().len(), constants_usize::ONE);
        let command = commands
            .as_ref()
            .first()
            .expect("989c8d37 builds_one_migration_command_per_database invariant must hold");
        assert_eq!(command.program().as_ref(), constants_str::SQLX);
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
