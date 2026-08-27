#[must_use]
pub fn migration_commands<Specifications>(specs: Specifications) -> super::ProcessCommands
where
    Specifications: IntoIterator<Item = super::DatabasePreparationSpec>,
{
    super::ProcessCommands::from(
        bounded_types::domain_types::vector::BoundedVec::from_max_iter(specs.into_iter().map(
            |spec| super::ProcessCommand {
                arguments: super::ProcessArguments::from(
                    bounded_types::domain_types::vector::BoundedVec::from_max_iter([
                        super::ProcessArgument::from(constants_str::DATABASE_URL_FLAG),
                        super::ProcessArgument::from(spec.url),
                        super::ProcessArgument::from(constants_str::SOURCE_FLAG),
                        super::ProcessArgument::from(spec.migrations_source),
                        super::ProcessArgument::from(constants_str::RUN),
                    ]),
                ),
                program: super::ProcessProgram::from(constants_str::SQLX),
            },
        )),
    )
}
