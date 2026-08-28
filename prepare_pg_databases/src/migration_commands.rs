#[must_use]
pub fn migration_commands<Specifications>(
    specs: Specifications,
) -> crate::domain_types::ProcessCommands
where
    Specifications: IntoIterator<Item = crate::domain_types::DatabasePreparationSpec>,
{
    crate::domain_types::ProcessCommands::from(
        bounded_types::domain_types::vector::BoundedVec::from_max_iter(specs.into_iter().map(
            |spec| crate::domain_types::ProcessCommand {
                arguments: crate::domain_types::ProcessArguments::from(
                    bounded_types::domain_types::vector::BoundedVec::from_max_iter([
                        crate::domain_types::ProcessArgument::from(
                            constants_str::DATABASE_URL_FLAG,
                        ),
                        crate::domain_types::ProcessArgument::from(spec.url),
                        crate::domain_types::ProcessArgument::from(constants_str::SOURCE_FLAG),
                        crate::domain_types::ProcessArgument::from(spec.migrations_source),
                        crate::domain_types::ProcessArgument::from(constants_str::RUN),
                    ]),
                ),
                program: crate::domain_types::ProcessProgram::from(constants_str::SQLX),
            },
        )),
    )
}
