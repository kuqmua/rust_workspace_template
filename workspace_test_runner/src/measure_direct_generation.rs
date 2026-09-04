pub(crate) fn measure_direct_generation<Generator, Inspector, Output>(
    generator: Generator,
    inspector: Inspector,
) -> crate::direct_generation_measurement::DirectGenerationMeasurement
where
    Generator: Fn() -> Output,
    Inspector: Fn(
        &Output,
    )
        -> crate::direct_generation_output_measurement::DirectGenerationOutputMeasurement,
{
    let measurement = (0..crate::domain_types::DIRECT_GENERATION_REPEAT_COUNT).fold(
        (
            u128::MAX,
            constants_u128::ZERO,
            constants_u128::ZERO,
            constants_usize::ZERO,
            constants_usize::ZERO,
        ),
        |(minimum_wall_microseconds, maximum_wall_microseconds, total_wall_microseconds, _, _),
         _| {
            let started = std::time::Instant::now();
            let output = generator();
            let wall_microseconds = started.elapsed().as_micros();
            let output_measurement = inspector(&output);
            (
                minimum_wall_microseconds.min(wall_microseconds),
                maximum_wall_microseconds.max(wall_microseconds),
                total_wall_microseconds.saturating_add(wall_microseconds),
                *output_measurement.get_output_bytes(),
                *output_measurement.get_output_token_trees(),
            )
        },
    );
    crate::direct_generation_measurement::DirectGenerationMeasurement::new(
        measurement.1,
        measurement.0,
        measurement.2,
        measurement.3,
        measurement.4,
    )
}
