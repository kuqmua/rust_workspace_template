#[derive(optml::Optml, Debug, newtype::FromInner)]
pub struct SynParsedGeneratePgTableInput(syn::DeriveInput);

#[derive(optml::Optml, Debug, newtype::FromInner)]
pub struct SynBuiltGeneratePgTableInput(crate::model::GeneratePgTableModel);

impl SynBuiltGeneratePgTableInput {
    #[must_use]
    pub const fn model(&self) -> &crate::model::GeneratePgTableModel {
        &self.0
    }
}

#[derive(optml::Optml, Debug, newtype::FromInner)]
pub struct SynValidatedGeneratePgTableInput(crate::model::GeneratePgTableModel);

impl SynValidatedGeneratePgTableInput {
    pub(crate) fn into_model(self) -> crate::model::GeneratePgTableModel {
        self.0
    }
}

#[derive(optml::Optml, Debug, thiserror::Error, newtype::FromInner, newtype::IntoInnerFrom)]
#[error(transparent)]
pub struct SynGeneratePgTablePipelineError(syn::Error);

#[derive(optml::Optml, Debug, thiserror::Error)]
pub enum GeneratePgTablePipelineError {
    #[error("{0}")]
    Build(SynGeneratePgTablePipelineError),
    #[error("{0}")]
    Parse(SynGeneratePgTablePipelineError),
    #[error("{0}")]
    Validate(SynGeneratePgTablePipelineError),
}

pub fn parse_generate_pg_table(
    input: macros_helpers::ts_writer::ProcMacro2TokenStreamRef<'_>,
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
    let _shape =
        crate::parse::struct_shape(workspace_macro_helpers::SynDeriveInputRef::from(&parsed.0))
            .map_err(|error| {
                GeneratePgTablePipelineError::Build(SynGeneratePgTablePipelineError::from(error))
            })?;
    Ok(SynBuiltGeneratePgTableInput::from(
        crate::model::GeneratePgTableModel::from_struct(parsed.0.into()),
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
