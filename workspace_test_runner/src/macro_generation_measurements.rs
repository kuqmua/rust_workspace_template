use super::{CargoArgs, MeasurementName};

pub(crate) fn macro_generation_measurements() -> [(MeasurementName, CargoArgs); 3] {
    [
        (
            MeasurementName::from(
                constants_str::WORKSPACE_TEST_RUNNER_GENERATE_PG_TABLE_MEASUREMENT,
            ),
            CargoArgs::from(&constants_str::WORKSPACE_TEST_RUNNER_CARGO_TEST_GEN_PG_TBL_ARGS[..]),
        ),
        (
            MeasurementName::from(
                constants_str::WORKSPACE_TEST_RUNNER_GENERATE_PG_TYPES_MEASUREMENT,
            ),
            CargoArgs::from(&constants_str::WORKSPACE_TEST_RUNNER_CARGO_TEST_GEN_PG_TYPES_ARGS[..]),
        ),
        (
            MeasurementName::from(
                constants_str::WORKSPACE_TEST_RUNNER_GENERATE_WHERE_FILTERS_MEASUREMENT,
            ),
            CargoArgs::from(&constants_str::WORKSPACE_TEST_RUNNER_CARGO_TEST_GEN_WH_FLTS_ARGS[..]),
        ),
    ]
}
