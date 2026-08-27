#[path = "domain_types/database_preparation_spec.rs"]
mod database_preparation_spec;
#[path = "domain_types/database_url.rs"]
mod database_url;
#[path = "domain_types/database_url_error.rs"]
mod database_url_error;
#[path = "domain_types/migration_commands.rs"]
mod migration_commands;
#[path = "domain_types/migrations_source.rs"]
mod migrations_source;
#[path = "domain_types/migrations_source_error.rs"]
mod migrations_source_error;
#[path = "domain_types/process_argument.rs"]
mod process_argument;
#[path = "domain_types/process_arguments.rs"]
mod process_arguments;
#[path = "domain_types/process_command.rs"]
mod process_command;
#[path = "domain_types/process_commands.rs"]
mod process_commands;
#[path = "domain_types/process_program.rs"]
mod process_program;
#[path = "domain_types/process_static_argument.rs"]
mod process_static_argument;
#[path = "domain_types/validate_database_url.rs"]
mod validate_database_url;
#[path = "domain_types/validate_migrations_source.rs"]
mod validate_migrations_source;

pub use database_preparation_spec::DatabasePreparationSpec;
pub use database_url::DatabaseUrl;
pub use database_url_error::DatabaseUrlError;
pub use migration_commands::migration_commands;
pub use migrations_source::MigrationsSource;
pub use migrations_source_error::MigrationsSourceError;
pub use process_argument::ProcessArgument;
pub use process_arguments::ProcessArguments;
pub use process_command::ProcessCommand;
pub use process_commands::ProcessCommands;
pub use process_program::ProcessProgram;
pub use process_static_argument::ProcessStaticArgument;
use validate_database_url::validate_database_url;
use validate_migrations_source::validate_migrations_source;

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
