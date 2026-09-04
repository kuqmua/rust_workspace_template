pub(crate) fn measure_generation_stages<
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
    Inspect,
>(
    input: Input,
    parse: Parse,
    build: Build,
    validate: Validate,
    emit: Emit,
    inspect: Inspect,
) -> Result<crate::generation_stage_measurement::GenerationStageMeasurement, Error>
where
    Parse: FnOnce(Input) -> Result<Parsed, Error>,
    Build: FnOnce(Parsed) -> Result<Built, Error>,
    Validate: FnOnce(Built) -> Result<Validated, Error>,
    Emit: FnOnce(Validated) -> Output,
    Inspect: FnOnce(&Output) -> usize,
{
    let parse_started = std::time::Instant::now();
    let parsed = parse(input)?;
    let parse_microseconds = parse_started.elapsed().as_micros();
    let build_started = std::time::Instant::now();
    let built = build(parsed)?;
    let build_microseconds = build_started.elapsed().as_micros();
    let validate_started = std::time::Instant::now();
    let validated = validate(built)?;
    let validate_microseconds = validate_started.elapsed().as_micros();
    let emit_started = std::time::Instant::now();
    let output = emit(validated);
    let emit_microseconds = emit_started.elapsed().as_micros();
    Ok(
        crate::generation_stage_measurement::GenerationStageMeasurement::new(
            parse_microseconds,
            build_microseconds,
            validate_microseconds,
            emit_microseconds,
            inspect(&output),
        ),
    )
}
