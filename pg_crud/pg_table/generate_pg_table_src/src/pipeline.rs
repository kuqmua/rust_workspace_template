#[derive(Debug)]
pub struct SynParsedGeneratePgTableInput(syn::DeriveInput);
impl From<syn::DeriveInput> for SynParsedGeneratePgTableInput {
    fn from(value: syn::DeriveInput) -> Self {
        Self(value)
    }
}

#[derive(Debug)]
pub struct SynBuiltGeneratePgTableInput(crate::model::GeneratePgTableModel);
impl From<crate::model::GeneratePgTableModel> for SynBuiltGeneratePgTableInput {
    fn from(value: crate::model::GeneratePgTableModel) -> Self {
        Self(value)
    }
}

impl SynBuiltGeneratePgTableInput {
    #[must_use]
    pub const fn model(&self) -> &crate::model::GeneratePgTableModel {
        &self.0
    }
}

#[derive(Debug)]
pub struct SynValidatedGeneratePgTableInput(crate::model::GeneratePgTableModel);
impl From<crate::model::GeneratePgTableModel> for SynValidatedGeneratePgTableInput {
    fn from(value: crate::model::GeneratePgTableModel) -> Self {
        Self(value)
    }
}

impl SynValidatedGeneratePgTableInput {
    pub(crate) fn into_model(self) -> crate::model::GeneratePgTableModel {
        self.0
    }
}

#[derive(Debug)]
pub struct SynGeneratePgTablePipelineError(syn::Error);
impl From<syn::Error> for SynGeneratePgTablePipelineError {
    fn from(value: syn::Error) -> Self {
        Self(value)
    }
}

impl From<SynGeneratePgTablePipelineError> for syn::Error {
    fn from(value: SynGeneratePgTablePipelineError) -> Self {
        value.0
    }
}

#[derive(Debug)]
pub enum GeneratePgTablePipelineError {
    Build(SynGeneratePgTablePipelineError),
    Parse(SynGeneratePgTablePipelineError),
    Validate(SynGeneratePgTablePipelineError),
}

impl std::fmt::Display for GeneratePgTablePipelineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Parse(error) | Self::Build(error) | Self::Validate(error) => error.0.fmt(f),
        }
    }
}

impl std::error::Error for GeneratePgTablePipelineError {}

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
