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

    #[test]
    fn test_database_url_byte_limits_preserve_content_and_empty_precedence() {
        [constants_str::X, constants_str::NON_ASCII_U_E9]
            .into_iter()
            .fold((), |(), suffix| {
                let mut input = constants_str::X.repeat(8_192usize - suffix.len());
                input.push_str(suffix);
                let pointer = input.as_ptr();
                assert!(crate::database_url::DatabaseUrl::try_from(input).is_ok_and(
                    |database_url| database_url.as_ref().len() == 8_192usize
                        && database_url.as_ref().as_ptr() == pointer
                        && database_url.as_ref().ends_with(suffix)
                ));
                let mut oversized = constants_str::X.repeat(8_192usize);
                oversized.push_str(suffix);
                assert_eq!(
                    crate::database_url::DatabaseUrl::try_from(oversized),
                    Err(crate::database_url_error::DatabaseUrlError::TooLong)
                );
            });
        assert_eq!(
            crate::database_url::DatabaseUrl::try_from(' '.to_string().repeat(8_193usize)),
            Err(crate::database_url_error::DatabaseUrlError::Empty)
        );
        let mut padded = ' '.to_string();
        padded.push_str(constants_str::TEST_DATABASE_URL);
        padded.push(' ');
        assert!(
            crate::database_url::DatabaseUrl::try_from(padded).is_ok_and(
                |database_url| database_url.as_ref().starts_with(' ')
                    && database_url.as_ref().ends_with(' ')
                    && database_url
                        .as_ref()
                        .contains(constants_str::TEST_DATABASE_URL)
            )
        );
    }

    #[test]
    fn test_migrations_source_byte_limits_preserve_unrestricted_text() {
        [constants_str::X, constants_str::NON_ASCII_U_E9]
            .into_iter()
            .fold((), |(), suffix| {
                let mut input = constants_str::X.repeat(4_096usize - suffix.len());
                input.push_str(suffix);
                let pointer = input.as_ptr();
                assert!(
                    crate::migrations_source::MigrationsSource::try_from(input).is_ok_and(
                        |migrations_source| migrations_source.as_ref().len() == 4_096usize
                            && migrations_source.as_ref().as_ptr() == pointer
                            && migrations_source.as_ref().ends_with(suffix)
                    )
                );
                let mut oversized = constants_str::X.repeat(4_096usize);
                oversized.push_str(suffix);
                assert_eq!(
                    crate::migrations_source::MigrationsSource::try_from(oversized),
                    Err(crate::migrations_source_error::MigrationsSourceError::TooLong)
                );
            });
        [
            String::new(),
            ' '.to_string(),
            constants_str::TEST_TEXT_WITH_NUL.to_owned(),
        ]
        .into_iter()
        .fold((), |(), input| {
            let expected = input.clone();
            assert!(
                crate::migrations_source::MigrationsSource::try_from(input)
                    .is_ok_and(|migrations_source| migrations_source.as_ref() == expected)
            );
        });
    }

    #[test]
    fn test_migration_commands_preserve_empty_input_and_ordered_arguments() {
        let empty = crate::migration_commands::migration_commands(std::iter::empty());
        assert!(empty.as_ref().is_empty());
        let specifications = [constants_str::TEST_MIGRATIONS_PATH, constants_str::EMPTY]
            .into_iter()
            .flat_map(|source| {
                let url = crate::database_url::DatabaseUrl::try_from(
                    constants_str::TEST_DATABASE_URL.to_owned(),
                );
                let migrations =
                    crate::migrations_source::MigrationsSource::try_from(source.to_owned());
                assert!(url.is_ok());
                assert!(migrations.is_ok());
                url.into_iter()
                    .zip(migrations)
                    .map(|(database_url, migrations_source)| {
                        crate::database_preparation_spec::DatabasePreparationSpec::new(
                            database_url,
                            migrations_source,
                        )
                    })
            });
        let commands = crate::migration_commands::migration_commands(specifications);
        assert_eq!(commands.as_ref().len(), 2usize);
        commands
            .as_ref()
            .iter()
            .zip([constants_str::TEST_MIGRATIONS_PATH, constants_str::EMPTY])
            .for_each(|(command, source)| {
                assert_eq!(command.program().as_ref(), constants_str::SQLX);
                assert!(command.arguments().as_ref().iter().map(AsRef::as_ref).eq([
                    constants_str::DATABASE_URL_FLAG,
                    constants_str::TEST_DATABASE_URL,
                    constants_str::SOURCE_FLAG,
                    source,
                    constants_str::RUN,
                ]));
            });
    }
}
