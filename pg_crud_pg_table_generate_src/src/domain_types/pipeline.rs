#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner)]
pub struct SynParsedGeneratePgTableInput(syn::DeriveInput);

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner)]
pub struct SynBuiltGeneratePgTableInput(crate::domain_types::table::GeneratePgTableModel);

impl SynBuiltGeneratePgTableInput {
    #[must_use]
    pub const fn model(&self) -> &crate::domain_types::table::GeneratePgTableModel {
        &self.0
    }
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner)]
pub struct SynValidatedGeneratePgTableInput(crate::domain_types::table::GeneratePgTableModel);

impl SynValidatedGeneratePgTableInput {
    pub(crate) fn into_model(self) -> crate::domain_types::table::GeneratePgTableModel {
        self.0
    }
}

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    thiserror::Error,
    newtype::FromInner,
    newtype::IntoInnerFrom,
)]
#[error(transparent)]
pub struct SynGeneratePgTablePipelineError(syn::Error);

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub enum GeneratePgTablePipelineError {
    #[error("{0}")]
    Build(SynGeneratePgTablePipelineError),
    #[error("{0}")]
    Parse(SynGeneratePgTablePipelineError),
    #[error("{0}")]
    Validate(SynGeneratePgTablePipelineError),
}

pub fn parse_generate_pg_table(
    input: macro_helpers::domain_types::ts_writer::ProcMacro2TokenStreamRef<'_>,
) -> Result<SynParsedGeneratePgTableInput, GeneratePgTablePipelineError> {
    syn::parse2(input.as_ref().clone())
        .map(SynParsedGeneratePgTableInput)
        .map_err(|error| {
            GeneratePgTablePipelineError::Parse(SynGeneratePgTablePipelineError::from(error))
        })
}

pub fn build_generate_pg_table(
    parsed: SynParsedGeneratePgTableInput,
) -> Result<SynBuiltGeneratePgTableInput, GeneratePgTablePipelineError> {
    let _shape = crate::domain_types::parse::struct_shape(
        workspace_macro_helpers::domain_types::SynDeriveInputRef::from(&parsed.0),
    )
    .map_err(|error| {
        GeneratePgTablePipelineError::Build(SynGeneratePgTablePipelineError::from(error))
    })?;
    Ok(SynBuiltGeneratePgTableInput::from(
        crate::domain_types::table::GeneratePgTableModel::from_struct(parsed.0.into()),
    ))
}

pub fn validate_generate_pg_table(
    built: SynBuiltGeneratePgTableInput,
) -> Result<SynValidatedGeneratePgTableInput, GeneratePgTablePipelineError> {
    built
        .0
        .validate()
        .map(SynValidatedGeneratePgTableInput)
        .map_err(|error| {
            GeneratePgTablePipelineError::Validate(SynGeneratePgTablePipelineError::from(
                syn::Error::from(error),
            ))
        })
}
