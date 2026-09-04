#[must_use]
pub fn generate_validated_tokens<
    Input,
    Parsed,
    Built,
    Validated,
    Output,
    Error,
    Parse,
    Build,
    Validate,
    Emit,
    ErrorTokens,
>(
    input: Input,
    parse: Parse,
    build: Build,
    validate: Validate,
    emit: Emit,
    error_tokens: ErrorTokens,
) -> Output
where
    Parse: FnOnce(Input) -> Result<Parsed, Error>,
    Build: FnOnce(Parsed) -> Result<Built, Error>,
    Validate: FnOnce(Built) -> Result<Validated, Error>,
    Emit: FnOnce(Validated) -> Output,
    ErrorTokens: FnOnce(Error) -> Output,
{
    match parse(input).and_then(build).and_then(validate) {
        Ok(validated) => emit(validated),
        Err(error) => error_tokens(error),
    }
}
