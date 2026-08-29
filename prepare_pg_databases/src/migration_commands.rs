#[must_use]
pub fn migration_commands<Specifications>(
    specs: Specifications,
) -> crate::process_commands::ProcessCommands
where
    Specifications: IntoIterator<Item = crate::database_preparation_spec::DatabasePreparationSpec>,
{
    crate::process_commands::ProcessCommands::from(
        bounded_types::bounded_vec::BoundedVec::from_max_iter(specs.into_iter().map(|spec| {
            crate::process_command::ProcessCommand {
                arguments: crate::process_arguments::ProcessArguments::from(
                    bounded_types::bounded_vec::BoundedVec::from_max_iter([
                        crate::process_argument::ProcessArgument::from(
                            constants_str::test_fixtures::DATABASE_URL_FLAG,
                        ),
                        crate::process_argument::ProcessArgument::from(spec.url),
                        crate::process_argument::ProcessArgument::from(
                            constants_str::test_fixtures::SOURCE_FLAG,
                        ),
                        crate::process_argument::ProcessArgument::from(spec.migrations_source),
                        crate::process_argument::ProcessArgument::from(
                            constants_str::test_fixtures::RUN,
                        ),
                    ]),
                ),
                program: crate::process_program::ProcessProgram::from(constants_str::catalog::SQLX),
            }
        })),
    )
}
