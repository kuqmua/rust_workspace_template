#[cfg(test)]
mod tests {
    #[test]
    fn test_builds_one_migration_command_per_database() {
        let url =
            crate::database_url::DatabaseUrl::try_from(constants_str::TEST_DATABASE_URL.to_owned());
        let source = crate::migrations_source::MigrationsSource::try_from(
            constants_str::TEST_MIGRATIONS_PATH.to_owned(),
        );
        assert!(url.is_ok());
        assert!(source.is_ok());
        let commands =
            crate::migration_commands::migration_commands(url.into_iter().zip(source).map(
                |(valid_url, valid_source)| {
                    crate::database_preparation_spec::DatabasePreparationSpec::new(
                        valid_url,
                        valid_source,
                    )
                },
            ));
        assert_eq!(commands.as_ref().len(), constants_usize::ONE);
        let command = commands
            .as_ref()
            .first()
            .expect(constants_str::DIAGNOSTIC_989C8D37);
        assert_eq!(command.program().as_ref(), constants_str::SQLX);
        assert_eq!(command.arguments().as_ref().len(), 5usize);
    }

    #[test]
    fn test_rejects_empty_database_url() {
        assert_eq!(
            crate::database_url::DatabaseUrl::try_from(String::new()),
            Err(crate::database_url_error::DatabaseUrlError::Empty)
        );
    }
}
