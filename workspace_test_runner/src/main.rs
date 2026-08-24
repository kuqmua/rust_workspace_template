mod admin_fixture;
mod discovery;
mod execution;
mod reporting;
const DIRECT_GENERATION_REPEAT_COUNT: usize = 5;
const MEASURE_REPEAT_COUNT: usize = 1000;
const RUNNER_MODE_MAX_LEN: usize = 1_024usize;
const SQL_BUILDER_MEASURE_SERIES_COUNT: usize = 5;
const CLEAN_ANSI_TEXT_MAX_LEN: usize = 16_777_216;
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, newtype::FromInner)]
struct MeasurementName(&'static str);
impl MeasurementName {
    const fn get(self) -> &'static str {
        self.0
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, newtype::FromInner)]
struct CargoArgs(&'static [&'static str]);
impl<const N: usize> From<&'static [&'static str; N]> for CargoArgs {
    fn from(value: &'static [&'static str; N]) -> Self {
        Self(value.as_slice())
    }
}
impl CargoArgs {
    const fn get(self) -> &'static [&'static str] {
        self.0
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, newtype::FromInner)]
struct StderrTextRef<'lt>(&'lt str);
impl<'lt> StderrTextRef<'lt> {
    const fn get(self) -> &'lt str {
        self.0
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, newtype::FromInner)]
struct AnsiTextRef<'lt>(&'lt str);
impl<'lt> AnsiTextRef<'lt> {
    const fn get(self) -> &'lt str {
        self.0
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, newtype::BoundedString)]
#[bounded_string(max = CLEAN_ANSI_TEXT_MAX_LEN)]
struct CleanAnsiText(String);
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, newtype::FromInner)]
struct MemusageKey(&'static str);
impl MemusageKey {
    const fn get(self) -> &'static str {
        self.0
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, newtype::FromInner)]
struct MemusageRowName(&'static str);
impl MemusageRowName {
    const fn get(self) -> &'static str {
        self.0
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, newtype::FromInner)]
struct MemusageColumnIdx(usize);
impl MemusageColumnIdx {
    const fn get(self) -> usize {
        self.0
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, newtype::FromInner)]
struct MemusageValueRef<'lt>(&'lt str);
impl<'lt> MemusageValueRef<'lt> {
    const fn get(self) -> &'lt str {
        self.0
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, newtype::FromInner)]
struct ProgramPathRef<'lt>(&'lt str);
impl<'lt> ProgramPathRef<'lt> {
    const fn get(self) -> &'lt str {
        self.0
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, newtype::FromInner)]
struct ProgramArgsRef<'lt>(&'lt [&'lt str]);
impl<'lt, const N: usize> From<&'lt [&'lt str; N]> for ProgramArgsRef<'lt> {
    fn from(value: &'lt [&'lt str; N]) -> Self {
        Self(value.as_slice())
    }
}
impl<'lt> ProgramArgsRef<'lt> {
    const fn get(self) -> &'lt [&'lt str] {
        self.0
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, newtype::FromInner)]
struct MemusageProgNameRef<'lt>(&'lt str);
impl<'lt> MemusageProgNameRef<'lt> {
    const fn get(self) -> &'lt str {
        self.0
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, newtype::AsRefOwned, newtype::FromInner,
)]
struct QuoteTokenStreamGeneratePgTableMeasureInputTokenStream(quote::__private::TokenStream);
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, newtype::FromInner)]
struct ToolName(&'static str);
impl ToolName {
    const fn get(self) -> &'static str {
        self.0
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, newtype::FromInner)]
struct ToolPath(&'static str);
impl ToolPath {
    const fn get(self) -> &'static str {
        self.0
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, newtype::FromInner)]
struct ToolAvailable(bool);
impl ToolAvailable {
    const fn get(self) -> bool {
        self.0
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, newtype::FromInner)]
struct StdRunnerIoErrorRef<'error_lt>(&'error_lt std::io::Error);
impl<'error_lt> StdRunnerIoErrorRef<'error_lt> {
    const fn get(self) -> &'error_lt std::io::Error {
        self.0
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, newtype::FromInner)]
struct StdRunnerPathRef<'path_lt>(&'path_lt std::path::Path);
impl<'path_lt> StdRunnerPathRef<'path_lt> {
    const fn get(self) -> &'path_lt std::path::Path {
        self.0
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::AsRefStr, newtype::BoundedString,
)]
#[bounded_string(max = RUNNER_MODE_MAX_LEN)]
struct RunnerMode(String);
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy)]
struct AllocationTool {
    name: ToolName,
    path: ToolPath,
}
fn macro_generation_measurements() -> [(MeasurementName, CargoArgs); 3] {
    [
        (
            MeasurementName::from(
                str_constants::WORKSPACE_TEST_RUNNER_GENERATE_PG_TABLE_MEASUREMENT,
            ),
            CargoArgs::from(&str_constants::WORKSPACE_TEST_RUNNER_CARGO_TEST_GEN_PG_TBL_ARGS[..]),
        ),
        (
            MeasurementName::from(
                str_constants::WORKSPACE_TEST_RUNNER_GENERATE_PG_TYPES_MEASUREMENT,
            ),
            CargoArgs::from(&str_constants::WORKSPACE_TEST_RUNNER_CARGO_TEST_GEN_PG_TYPES_ARGS[..]),
        ),
        (
            MeasurementName::from(
                str_constants::WORKSPACE_TEST_RUNNER_GENERATE_WHERE_FILTERS_MEASUREMENT,
            ),
            CargoArgs::from(&str_constants::WORKSPACE_TEST_RUNNER_CARGO_TEST_GEN_WH_FLTS_ARGS[..]),
        ),
    ]
}
#[allow(clippy::single_call_fn)] // runtime construction keeps wrapper initialization on From while centralizing tool metadata
fn allocation_tools() -> [AllocationTool; 6] {
    [
        AllocationTool {
            name: ToolName::from(str_constants::WORKSPACE_TEST_RUNNER_LIBMEMUSAGE_TOOL),
            path: ToolPath::from(str_constants::WORKSPACE_TEST_RUNNER_MEMUSAGE_PATH),
        },
        AllocationTool {
            name: ToolName::from(str_constants::WORKSPACE_TEST_RUNNER_VALGRIND_TOOL),
            path: ToolPath::from(str_constants::WORKSPACE_TEST_RUNNER_VALGRIND_PATH),
        },
        AllocationTool {
            name: ToolName::from(str_constants::WORKSPACE_TEST_RUNNER_HEAPTRACK_TOOL),
            path: ToolPath::from(str_constants::WORKSPACE_TEST_RUNNER_HEAPTRACK_PATH),
        },
        AllocationTool {
            name: ToolName::from(str_constants::WORKSPACE_TEST_RUNNER_LTRACE_TOOL),
            path: ToolPath::from(str_constants::WORKSPACE_TEST_RUNNER_LTRACE_PATH),
        },
        AllocationTool {
            name: ToolName::from(str_constants::WORKSPACE_TEST_RUNNER_PERF_TOOL),
            path: ToolPath::from(str_constants::WORKSPACE_TEST_RUNNER_PERF_PATH),
        },
        AllocationTool {
            name: ToolName::from(str_constants::PG_CRUD_PG_TIME),
            path: ToolPath::from(str_constants::WORKSPACE_TEST_RUNNER_TIME_PATH),
        },
    ]
}
fn print_without_measurement_footer(stderr: StderrTextRef<'_>) {
    stderr
        .get()
        .lines()
        .filter(|line| {
            !line
                .trim()
                .starts_with(str_constants::WORKSPACE_TEST_RUNNER_PEAK_RSS_PREFIX)
        })
        .filter(|line| {
            !line
                .trim()
                .starts_with(str_constants::WORKSPACE_TEST_RUNNER_MINOR_PAGE_FAULTS_PREFIX)
        })
        .filter(|line| {
            !line
                .trim()
                .starts_with(str_constants::WORKSPACE_TEST_RUNNER_MAJOR_PAGE_FAULTS_PREFIX)
        })
        .for_each(|line| eprintln!("{line}"));
}
fn strip_ansi_codes(value: AnsiTextRef<'_>) -> CleanAnsiText {
    let clean = value
        .get()
        .chars()
        .fold(
            (String::with_capacity(value.get().len()), false),
            |(mut accumulator, in_escape), ch| match (in_escape, ch) {
                (true, 'm') => (accumulator, false),
                (false, '\u{1b}') | (true, _) => (accumulator, true),
                (false, _) => {
                    accumulator.push(ch);
                    (accumulator, false)
                }
            },
        )
        .0;
    CleanAnsiText::try_from(clean).unwrap_or_else(CleanAnsiText::from)
}
fn print_without_memusage_footer(stderr: StderrTextRef<'_>) {
    let clean = strip_ansi_codes(AnsiTextRef::from(stderr.get()));
    clean
        .0
        .as_str()
        .lines()
        .take_while(|line| !line.contains(str_constants::MEMORY_USAGE_SUMMARY))
        .filter(|line| !line.trim().is_empty())
        .for_each(|line| eprintln!("{line}"));
}
fn memusage_heap_value(text: &CleanAnsiText, key: MemusageKey) -> MemusageValueRef<'_> {
    text.0
        .as_str()
        .lines()
        .find_map(|line| line.split_once(key.get()).map(|(_, tail)| tail.trim()))
        .and_then(|tail| tail.split([',', ' ']).find(|part| !part.is_empty()))
        .map_or_else(
            || MemusageValueRef::from(str_constants::UNAVAILABLE),
            MemusageValueRef,
        )
}
fn memusage_table_value(
    text: &CleanAnsiText,
    row_name: MemusageRowName,
    column_idx: MemusageColumnIdx,
) -> MemusageValueRef<'_> {
    text.0
        .as_str()
        .lines()
        .find(|line| line.contains(row_name.get()))
        .and_then(|line| line.split('|').nth(1))
        .and_then(|tail| tail.split_whitespace().nth(column_idx.get()))
        .map_or_else(
            || MemusageValueRef::from(str_constants::UNAVAILABLE),
            MemusageValueRef,
        )
}
fn measure_memusage_command(
    measurement_name: MeasurementName,
    program: ProgramPathRef<'_>,
    args: ProgramArgsRef<'_>,
    memusage_prog_name: MemusageProgNameRef<'_>,
) -> Result<(), ()> {
    let measurement_name_value = measurement_name.get();
    if !std::path::Path::new(str_constants::WORKSPACE_TEST_RUNNER_MEMUSAGE_PATH).exists() {
        println!(
            "measurement={measurement_name_value}_allocations status=unavailable reason=libmemusage_not_found path={}",
            str_constants::WORKSPACE_TEST_RUNNER_MEMUSAGE_PATH
        );
        return Ok(());
    }
    let command_output = macros_helpers::tool_command::ToolCommand::new(
        macros_helpers::tool_command::ToolProgramRef::from(program.get()),
    )
    .args(macros_helpers::tool_command::ToolArgsRef::from(args.get()))
    .env(
        macros_helpers::tool_command::ToolEnvKeyRef::from(str_constants::LD_PRELOAD),
        macros_helpers::tool_command::ToolEnvValueRef::from(
            str_constants::WORKSPACE_TEST_RUNNER_MEMUSAGE_PATH,
        ),
    )
    .env(
        macros_helpers::tool_command::ToolEnvKeyRef::from(str_constants::MEMUSAGE_PROG_NAME),
        macros_helpers::tool_command::ToolEnvValueRef::from(memusage_prog_name.get()),
    )
    .output();
    match command_output {
        Ok(output) if output.status.success() => {
            {
                let stdout = String::from_utf8_lossy(output.stdout.as_slice());
                if !stdout.is_empty() {
                    print!("{stdout}");
                }
            }
            let stderr = String::from_utf8_lossy(output.stderr.as_slice());
            print_without_memusage_footer(StderrTextRef::from(stderr.as_ref()));
            let clean = strip_ansi_codes(AnsiTextRef::from(stderr.as_ref()));
            let heap_total =
                memusage_heap_value(&clean, MemusageKey::from(str_constants::HEAP_TOTAL)).get();
            let heap_peak =
                memusage_heap_value(&clean, MemusageKey::from(str_constants::HEAP_PEAK)).get();
            let stack_peak =
                memusage_heap_value(&clean, MemusageKey::from(str_constants::STACK_PEAK)).get();
            let malloc_calls = memusage_table_value(
                &clean,
                MemusageRowName::from(str_constants::MALLOC),
                MemusageColumnIdx::from(0),
            )
            .get();
            let malloc_memory = memusage_table_value(
                &clean,
                MemusageRowName::from(str_constants::MALLOC),
                MemusageColumnIdx::from(1),
            )
            .get();
            let malloc_failed = memusage_table_value(
                &clean,
                MemusageRowName::from(str_constants::MALLOC),
                MemusageColumnIdx::from(2),
            )
            .get();
            let realloc_calls = memusage_table_value(
                &clean,
                MemusageRowName::from(str_constants::REALLOC),
                MemusageColumnIdx::from(0),
            )
            .get();
            let realloc_memory = memusage_table_value(
                &clean,
                MemusageRowName::from(str_constants::REALLOC),
                MemusageColumnIdx::from(1),
            )
            .get();
            let realloc_failed = memusage_table_value(
                &clean,
                MemusageRowName::from(str_constants::REALLOC),
                MemusageColumnIdx::from(2),
            )
            .get();
            let calloc_calls = memusage_table_value(
                &clean,
                MemusageRowName::from(str_constants::CALLOC),
                MemusageColumnIdx::from(0),
            )
            .get();
            let calloc_memory = memusage_table_value(
                &clean,
                MemusageRowName::from(str_constants::CALLOC),
                MemusageColumnIdx::from(1),
            )
            .get();
            let calloc_failed = memusage_table_value(
                &clean,
                MemusageRowName::from(str_constants::CALLOC),
                MemusageColumnIdx::from(2),
            )
            .get();
            let free_calls = memusage_table_value(
                &clean,
                MemusageRowName::from(str_constants::FREE),
                MemusageColumnIdx::from(0),
            )
            .get();
            let free_memory = memusage_table_value(
                &clean,
                MemusageRowName::from(str_constants::FREE),
                MemusageColumnIdx::from(1),
            )
            .get();
            println!(
                "measurement={measurement_name_value}_allocations status=ok tool=libmemusage heap_total_bytes={heap_total} heap_peak_bytes={heap_peak} stack_peak_bytes={stack_peak} malloc_calls={malloc_calls} malloc_bytes={malloc_memory} malloc_failed={malloc_failed} realloc_calls={realloc_calls} realloc_bytes={realloc_memory} realloc_failed={realloc_failed} calloc_calls={calloc_calls} calloc_bytes={calloc_memory} calloc_failed={calloc_failed} free_calls={free_calls} free_bytes={free_memory}"
            );
            Ok(())
        }
        Ok(output) => {
            {
                let stdout = String::from_utf8_lossy(output.stdout.as_slice());
                if !stdout.is_empty() {
                    print!("{stdout}");
                }
            }
            let stderr = String::from_utf8_lossy(output.stderr.as_slice());
            print_without_memusage_footer(StderrTextRef::from(stderr.as_ref()));
            eprintln!(
                "measurement={measurement_name_value}_allocations status=failed exit_status={}",
                output.status
            );
            Err(())
        }
        Err(error) => {
            eprintln!(
                "measurement={measurement_name_value}_allocations status=spawn_failed error={error}"
            );
            Err(())
        }
    }
}
fn measure_cargo_command(measurement_name: MeasurementName, args: CargoArgs) -> Result<(), ()> {
    let measurement_name_value = measurement_name.get();
    let started = std::time::Instant::now();
    let command_output = {
        let measurement_format = format!(
            "{}%M\n{}%R\n{}%F",
            str_constants::WORKSPACE_TEST_RUNNER_PEAK_RSS_PREFIX,
            str_constants::WORKSPACE_TEST_RUNNER_MINOR_PAGE_FAULTS_PREFIX,
            str_constants::WORKSPACE_TEST_RUNNER_MAJOR_PAGE_FAULTS_PREFIX,
        );
        macros_helpers::tool_command::ToolCommand::new(
            macros_helpers::tool_command::ToolProgramRef::from(
                str_constants::WORKSPACE_TEST_RUNNER_TIME_PATH,
            ),
        )
        .arg(macros_helpers::tool_command::ToolArgRef::from(
            str_constants::F,
        ))
        .arg(macros_helpers::tool_command::ToolArgRef::from(
            measurement_format.as_str(),
        ))
        .arg(macros_helpers::tool_command::ToolArgRef::from(
            str_constants::WORKSPACE_TEST_RUNNER_CARGO,
        ))
        .args(macros_helpers::tool_command::ToolArgsRef::from(args.get()))
        .output()
    };
    let duration = started.elapsed();
    match command_output {
        Ok(output) if output.status.success() => {
            let stderr = String::from_utf8_lossy(output.stderr.as_slice());
            let peak_rss_kb = stderr
                .lines()
                .find_map(|line| {
                    line.trim()
                        .strip_prefix(str_constants::WORKSPACE_TEST_RUNNER_PEAK_RSS_PREFIX)
                })
                .unwrap_or(str_constants::UNAVAILABLE);
            let minor_page_faults = stderr
                .lines()
                .find_map(|line| {
                    line.trim()
                        .strip_prefix(str_constants::WORKSPACE_TEST_RUNNER_MINOR_PAGE_FAULTS_PREFIX)
                })
                .unwrap_or(str_constants::UNAVAILABLE);
            let major_page_faults = stderr
                .lines()
                .find_map(|line| {
                    line.trim()
                        .strip_prefix(str_constants::WORKSPACE_TEST_RUNNER_MAJOR_PAGE_FAULTS_PREFIX)
                })
                .unwrap_or(str_constants::UNAVAILABLE);
            {
                let stdout = String::from_utf8_lossy(output.stdout.as_slice());
                if !stdout.is_empty() {
                    print!("{stdout}");
                }
            }
            print_without_measurement_footer(StderrTextRef::from(stderr.as_ref()));
            println!(
                "measurement={measurement_name_value} wall_ms={} memory_proxy_peak_rss_kb={} memory_proxy_minor_page_faults={} memory_proxy_major_page_faults={} status=ok",
                duration.as_millis(),
                peak_rss_kb,
                minor_page_faults,
                major_page_faults
            );
            Ok(())
        }
        Ok(output) => {
            {
                let stdout = String::from_utf8_lossy(output.stdout.as_slice());
                if !stdout.is_empty() {
                    print!("{stdout}");
                }
            }
            let stderr = String::from_utf8_lossy(output.stderr.as_slice());
            print_without_measurement_footer(StderrTextRef::from(stderr.as_ref()));
            eprintln!(
                "measurement={measurement_name_value} status=failed exit_status={}",
                output.status
            );
            Err(())
        }
        Err(error) => {
            eprintln!("measurement={measurement_name_value} status=spawn_failed error={error}");
            Err(())
        }
    }
}
fn generate_pg_table_measure_input_token_stream(
    tests_write_into_file: &dyn quote::ToTokens,
) -> QuoteTokenStreamGeneratePgTableMeasureInputTokenStream {
    let allow_clippy_arbitrary_src_item_ordering =
        token_patterns::AllowClippyArbitrarySrcItemOrdering;
    QuoteTokenStreamGeneratePgTableMeasureInputTokenStream::from(quote::quote! {
        #allow_clippy_arbitrary_src_item_ordering
        #[derive(Debug, Clone, Copy, optimal_memory_layout::OptimalMemoryLayout)]
        #[generate_pg_table::generate_pg_table_config{{
            "cm_write_into_file": "False",
            "co_write_into_file": "False",
            "rm_write_into_file": "False",
            "ro_write_into_file": "False",
            "um_write_into_file": "False",
            "uo_write_into_file": "False",
            "dm_write_into_file": "False",
            "dlo_write_into_file": "False",
            "tests_write_into_file": #tests_write_into_file,
            "common_write_into_file": "False",
            "whole_write_into_file": "False"
        }}]
        #[generate_pg_table::common_error_variants{
            enum CommonErrorVariants {
                CheckCommit {
                    #[eo_location]
                    check_commit: route_validators::check_commit::CommitError,
                    location: location_lib::location::Location,
                },
            }
        }]
        #[generate_pg_table::cm_logic{}]
        #[generate_pg_table::co_logic{}]
        #[generate_pg_table::rm_logic{}]
        #[generate_pg_table::ro_logic{}]
        #[generate_pg_table::um_logic{}]
        #[generate_pg_table::uo_logic{}]
        #[generate_pg_table::dm_logic{}]
        #[generate_pg_table::dlo_logic{}]
        #[generate_pg_table::common_logic{}]
        pub struct TableExample {
            #[generate_pg_table_primary_key]
            pub primary_key_column: pg_types_text_misc::SqlxTypesUuidUuidAsNonNullUuidV4InitializationByPg,
            pub column_0: pg_types_numeric::I16AsNonNullInt2,
            pub column_1: pg_types_numeric::OptionalI16AsNullableInt2,
            pub column_2: pg_types_numeric::I32AsNonNullInt4,
        }
    })
}
// Allocation workloads are separate process entry points dispatched by CLI mode.
#[allow(clippy::single_call_fn)]
fn run_alloc_workload_generate_pg_table_src() {
    let input = generate_pg_table_measure_input_token_stream(&quote::quote! {"False"});
    let output_bytes =
        (0..DIRECT_GENERATION_REPEAT_COUNT).fold(usize_constants::ZERO, |accumulator, _| {
            let output = generate_pg_table_src::generate_pg_table(
                macros_helpers::ts_writer::ProcMacro2TokenStreamRef::from(input.as_ref()),
            );
            accumulator.saturating_add(output.to_string().len())
        });
    println!(
        "allocation_workload=generate_pg_table_src repeat_count={DIRECT_GENERATION_REPEAT_COUNT} output_bytes={output_bytes}"
    );
}
// Allocation workloads are separate process entry points dispatched by CLI mode.
#[allow(clippy::single_call_fn)]
fn run_alloc_workload_generate_pg_types_src() {
    let input = quote::quote! {
        {
            "pg_table_cols_write_into_file": "False",
            "whole_write_into_file": "False",
            "variant": "All"
        }
    };
    let output_bytes =
        (0..DIRECT_GENERATION_REPEAT_COUNT).fold(usize_constants::ZERO, |accumulator, _| {
            let output = generate_pg_types_src::generate_pg_types(
                macros_helpers::ts_writer::ProcMacro2TokenStreamRef::from(&input),
            );
            accumulator.saturating_add(output.to_string().len())
        });
    println!(
        "allocation_workload=generate_pg_types_src repeat_count={DIRECT_GENERATION_REPEAT_COUNT} output_bytes={output_bytes}"
    );
}
// Allocation workloads are separate process entry points dispatched by CLI mode.
#[allow(clippy::single_call_fn)]
fn run_alloc_workload_pg_crud_common_query_part() -> Result<(), ()> {
    let output_bytes =
        (0..SQL_BUILDER_MEASURE_SERIES_COUNT).try_fold(usize_constants::ZERO, |series_accumulator, _| {
            (0..MEASURE_REPEAT_COUNT).try_fold(series_accumulator, |accumulator, _| {
                let mut increment = u64_constants::ZERO;
                match pg_crud_common::PgTypeWhereFilter::query_part(
                    &pg_crud_common::PaginationBase::default(),
                    &mut increment,
                    pg_crud_common::SqlColumnRef::from(&str_constants::COLUMN),
                    pg_crud_common::AddOperator::from(false),
                ) {
                    Ok(fragment) => Ok(accumulator.saturating_add(fragment.as_ref().len())),
                    Err(error) => {
                        eprintln!(
                            "allocation_workload=pg_crud_common_query_part status=failed error={error:?}"
                        );
                        Err(())
                    }
                }
            })
        })?;
    println!(
        "allocation_workload=pg_crud_common_query_part series_count={SQL_BUILDER_MEASURE_SERIES_COUNT} repeat_count={MEASURE_REPEAT_COUNT} output_bytes={output_bytes}"
    );
    Ok(())
}
// Allocation workloads are separate process entry points dispatched by CLI mode.
#[allow(clippy::single_call_fn)]
fn run_alloc_workload_where_filters_query_part() -> Result<(), ()> {
    let where_filters_values = (i32_constants::ZERO..64i32).collect::<Vec<i32>>();
    let where_filters_bounded_vec = match where_filters::BoundedVec::<i32, 64>::try_from(
        where_filters_values,
    ) {
        Ok(value) => value,
        Err(error) => {
            eprintln!(
                "allocation_workload=where_filters_query_part status=setup_failed error={error:?}"
            );
            return Err(());
        }
    };
    let output_bytes =
        (0..SQL_BUILDER_MEASURE_SERIES_COUNT).try_fold(usize_constants::ZERO, |series_accumulator, _| {
            (0..MEASURE_REPEAT_COUNT).try_fold(series_accumulator, |accumulator, _| {
                let mut increment = u64_constants::ZERO;
                match where_filters_bounded_vec.pg_type_query_part(
                    &mut increment,
                    pg_crud_common::SqlColumnRef::from(&str_constants::COLUMN),
                    pg_crud_common::AddOperator::from(false),
                ) {
                    Ok(fragment) => Ok(accumulator.saturating_add(fragment.as_ref().len())),
                    Err(error) => {
                        eprintln!("allocation_workload=where_filters_query_part status=failed error={error:?}");
                        Err(())
                    }
                }
            })
        })?;
    println!(
        "allocation_workload=where_filters_query_part series_count={SQL_BUILDER_MEASURE_SERIES_COUNT} repeat_count={MEASURE_REPEAT_COUNT} output_bytes={output_bytes}"
    );
    Ok(())
}
fn cargo_subcommand_available(subcommand: ToolName) -> ToolAvailable {
    let args = [subcommand.get(), str_constants::VERSION];
    macros_helpers::tool_command::ToolCommand::new(
        macros_helpers::tool_command::ToolProgramRef::from(
            str_constants::WORKSPACE_TEST_RUNNER_CARGO,
        ),
    )
    .args(macros_helpers::tool_command::ToolArgsRef::from(
        args.as_slice(),
    ))
    .output()
    .is_ok_and(|output| output.status.success())
    .into()
}
#[allow(
    clippy::needless_for_each,
    clippy::single_call_fn,
    reason = "keeps release-tool reporting separate and repository policy forbids for loops"
)]
fn print_optional_release_tools() {
    [
        str_constants::WORKSPACE_TEST_RUNNER_AUDIT_SUBCOMMAND,
        str_constants::WORKSPACE_TEST_RUNNER_DENY_SUBCOMMAND,
        str_constants::WORKSPACE_TEST_RUNNER_HACK_SUBCOMMAND,
        str_constants::SEMVER_CHECKS,
        str_constants::UDEPS,
        str_constants::MACHETE,
        str_constants::LLVM_COV,
    ]
    .into_iter()
    .for_each(|tool| {
        println!(
            "release_tool={tool} available={}",
            cargo_subcommand_available(ToolName::from(tool)).get()
        );
    });
}
#[allow(clippy::single_call_fn)] // release orchestration is an explicit CLI mode boundary
fn run_release() -> Result<(), ()> {
    print_optional_release_tools();
    let mut commands =
        Vec::<(&str, &[&str])>::from(str_constants::WORKSPACE_TEST_RUNNER_STATIC_COMMANDS);
    if cargo_subcommand_available(ToolName::from(str_constants::NEXTEST)).get() {
        commands.extend(str_constants::WORKSPACE_TEST_RUNNER_NEXTEST_COMMANDS);
    } else {
        commands.extend(str_constants::WORKSPACE_TEST_RUNNER_CARGO_TEST_COMMANDS);
    }
    [
        (
            str_constants::WORKSPACE_TEST_RUNNER_AUDIT_SUBCOMMAND,
            str_constants::WORKSPACE_TEST_RUNNER_CARGO_AUDIT_ARGS.as_slice(),
        ),
        (
            str_constants::WORKSPACE_TEST_RUNNER_DENY_SUBCOMMAND,
            str_constants::WORKSPACE_TEST_RUNNER_CARGO_DENY_ARGS.as_slice(),
        ),
        (
            str_constants::WORKSPACE_TEST_RUNNER_HACK_SUBCOMMAND,
            str_constants::WORKSPACE_TEST_RUNNER_CARGO_HACK_ARGS.as_slice(),
        ),
        (
            str_constants::MACHETE,
            str_constants::WORKSPACE_TEST_RUNNER_CARGO_MACHETE_ARGS.as_slice(),
        ),
        (
            str_constants::SEMVER_CHECKS,
            str_constants::WORKSPACE_TEST_RUNNER_CARGO_SEMVER_CHECKS_ARGS.as_slice(),
        ),
        (
            str_constants::UDEPS,
            str_constants::WORKSPACE_TEST_RUNNER_CARGO_UDEPS_ARGS.as_slice(),
        ),
    ]
    .into_iter()
    .filter(|(subcommand, _args)| cargo_subcommand_available(ToolName::from(*subcommand)).get())
    .for_each(|(_subcommand, args)| {
        commands.push((str_constants::WORKSPACE_TEST_RUNNER_CARGO, args));
    });
    execution::run_commands(execution::CommandsRef::from(commands.as_slice()))
}
fn run_workspace_tests() -> Result<(), ()> {
    if cargo_subcommand_available(ToolName::from(str_constants::NEXTEST)).get() {
        println!("test_executor=nextest");
        execution::run_commands(execution::CommandsRef::from(
            &str_constants::WORKSPACE_TEST_RUNNER_NEXTEST_COMMANDS,
        ))
    } else {
        println!("test_executor=cargo fallback=true");
        execution::run_commands(execution::CommandsRef::from(
            &str_constants::WORKSPACE_TEST_RUNNER_CARGO_TEST_COMMANDS,
        ))
    }
}
fn main() {
    let mode = discovery::mode();
    let result = match mode.as_ref().map(RunnerMode::as_ref) {
        None | Some(str_constants::STATIC) => execution::run_commands(
            execution::CommandsRef::from(&str_constants::WORKSPACE_TEST_RUNNER_STATIC_COMMANDS),
        ),
        Some(str_constants::DATABASE) => {
            match std::env::var(str_constants::ENV_NAMES_DATABASE_URL) {
                Ok(database_url) => {
                    match macros_helpers::test_database::validate_test_database_url(
                        macros_helpers::test_database::UrlRef::from(database_url.as_str()),
                    ) {
                        Ok(_target) => execution::run_commands(execution::CommandsRef::from(&[(
                            str_constants::WORKSPACE_TEST_RUNNER_CARGO,
                            &str_constants::WORKSPACE_TEST_RUNNER_CARGO_TEST_DATABASE_ARGS[..],
                        )])),
                        Err(error) => {
                            eprintln!("database test guard rejected DATABASE_URL: {error}");
                            Err(())
                        }
                    }
                }
                Err(error) => {
                    eprintln!("database test mode requires DATABASE_URL: {error}");
                    Err(())
                }
            }
        }
        Some(str_constants::WORKSPACE_TEST_RUNNER_GENERATE_PG_TABLE_WORKLOAD) => {
            run_alloc_workload_generate_pg_table_src();
            Ok(())
        }
        Some(str_constants::WORKSPACE_TEST_RUNNER_GENERATE_PG_TYPES_WORKLOAD) => {
            run_alloc_workload_generate_pg_types_src();
            Ok(())
        }
        Some(str_constants::WORKSPACE_TEST_RUNNER_ADMIN_CONTRACT_FIXTURE) => {
            admin_fixture::write_admin_contract_fixture()
        }
        Some(str_constants::WORKSPACE_TEST_RUNNER_PG_CRUD_COMMON_QUERY_PART_WORKLOAD) => {
            run_alloc_workload_pg_crud_common_query_part()
        }
        Some(str_constants::WORKSPACE_TEST_RUNNER_WHERE_FILTERS_QUERY_PART_WORKLOAD) => {
            run_alloc_workload_where_filters_query_part()
        }
        Some(str_constants::MACRO_GENERATION) => macro_generation_measurements()
            .iter()
            .try_fold((), |(), (measurement_name, args)| {
                measure_cargo_command(*measurement_name, *args)
            }),
        Some(str_constants::TESTS_ALT) => run_workspace_tests(),
        Some(str_constants::HEAVY_LOAD) => {
            if cargo_subcommand_available(ToolName::from(str_constants::NEXTEST)).get() {
                execution::run_commands(execution::CommandsRef::from(&[(
                    str_constants::WORKSPACE_TEST_RUNNER_CARGO,
                    &str_constants::WORKSPACE_TEST_RUNNER_NEXTEST_HEAVY_ARGS[..],
                )]))
            } else {
                eprintln!("heavy-load mode requires cargo-nextest; optional tool is unavailable");
                Err(())
            }
        }
        Some(str_constants::RELEASE) => run_release(),
        Some(str_constants::MEASURE) => {
            let allocation_tools_printed: Result<(), std::convert::Infallible> =
                allocation_tools().iter().try_fold((), |(), tool| {
                    let available = discovery::tool_available(tool.path);
                    reporting::allocation_tool(tool.name, tool.path, available);
                    Ok(())
                });
            match allocation_tools_printed {
                Ok(()) => {}
                Err(error) => match error {},
            }
            if std::path::Path::new(str_constants::WORKSPACE_TEST_RUNNER_MEMUSAGE_PATH).exists() {
                println!(
                    "measurement=exact_allocations status=available tool=libmemusage path={}",
                    str_constants::WORKSPACE_TEST_RUNNER_MEMUSAGE_PATH
                );
                measure_memusage_command(
                    MeasurementName::from(str_constants::CODE_STYLE),
                    ProgramPathRef::from(str_constants::WORKSPACE_TEST_RUNNER_CARGO),
                    ProgramArgsRef::from(&[
                        str_constants::TEST_ALT_3,
                        str_constants::P,
                        str_constants::TESTS_ALT,
                        str_constants::CODE_STYLE,
                    ]),
                    MemusageProgNameRef::from(str_constants::WORKSPACE_TEST_RUNNER_CARGO),
                )
                .unwrap_or_else(|()| std::process::exit(1));
                let current_exe = match std::env::current_exe() {
                    Ok(value) => value,
                    Err(error) => {
                        eprintln!(
                            "measurement=exact_allocations status=current_exe_failed error={error}"
                        );
                        std::process::exit(1);
                    }
                };
                let current_exe_string = current_exe.to_string_lossy().to_string();
                let current_exe_prog_name = current_exe
                    .file_name()
                    .and_then(|value| value.to_str())
                    .unwrap_or(str_constants::WORKSPACE_TEST_RUNNER_ALT);
                [
                    (
                        MeasurementName::from(str_constants::GENERATE_PG_TABLE_SRC),
                        str_constants::WORKSPACE_TEST_RUNNER_GENERATE_PG_TABLE_WORKLOAD,
                    ),
                    (
                        MeasurementName::from(str_constants::GENERATE_PG_TYPES_SRC),
                        str_constants::WORKSPACE_TEST_RUNNER_GENERATE_PG_TYPES_WORKLOAD,
                    ),
                    (
                        MeasurementName::from(str_constants::PG_CRUD_COMMON_QUERY_PART),
                        str_constants::WORKSPACE_TEST_RUNNER_PG_CRUD_COMMON_QUERY_PART_WORKLOAD,
                    ),
                    (
                        MeasurementName::from(str_constants::WHERE_FILTERS_QUERY_PART),
                        str_constants::WORKSPACE_TEST_RUNNER_WHERE_FILTERS_QUERY_PART_WORKLOAD,
                    ),
                ]
                .into_iter()
                .try_fold((), |(), (measurement_name, workload_mode)| {
                    measure_memusage_command(
                        measurement_name,
                        ProgramPathRef::from(current_exe_string.as_str()),
                        ProgramArgsRef::from(&[workload_mode]),
                        MemusageProgNameRef::from(current_exe_prog_name),
                    )
                })
                .unwrap_or_else(|()| std::process::exit(1));
            } else {
                println!(
                    "measurement=exact_allocations status=unavailable reason=no_safe_allocator_counter_or_external_allocation_profiler memory_proxy_fields=memory_proxy_peak_rss_kb,memory_proxy_minor_page_faults,memory_proxy_major_page_faults"
                );
            }
            measure_cargo_command(
                MeasurementName::from(str_constants::CODE_STYLE),
                CargoArgs::from(&[
                    str_constants::TEST_ALT_3,
                    str_constants::P,
                    str_constants::TESTS_ALT,
                    str_constants::CODE_STYLE,
                ]),
            )
            .unwrap_or_else(|()| std::process::exit(1));
            measure_cargo_command(
                MeasurementName::from(str_constants::CLIPPY),
                CargoArgs::from(&str_constants::WORKSPACE_TEST_RUNNER_CARGO_CLIPPY_ARGS),
            )
            .unwrap_or_else(|()| std::process::exit(1));
            let generate_pg_table_input_token_stream =
                generate_pg_table_measure_input_token_stream(&quote::quote! {"False"});
            let generate_pg_table_input_with_tests_token_stream =
                generate_pg_table_measure_input_token_stream(&quote::quote! {"True"});
            let parse_started = std::time::Instant::now();
            let parsed = generate_pg_table_src::parse_generate_pg_table(
                macros_helpers::ts_writer::ProcMacro2TokenStreamRef::from(
                    generate_pg_table_input_token_stream.as_ref(),
                ),
            )
            .unwrap_or_else(|error| panic!("d6399cbf: {error}"));
            let parse_us = parse_started.elapsed().as_micros();
            let build_started = std::time::Instant::now();
            let built = generate_pg_table_src::build_generate_pg_table(parsed)
                .unwrap_or_else(|error| panic!("6acb4e92: {error}"));
            let build_us = build_started.elapsed().as_micros();
            let validate_started = std::time::Instant::now();
            let validated = generate_pg_table_src::validate_generate_pg_table(built)
                .unwrap_or_else(|error| panic!("4533a758: {error}"));
            let validate_us = validate_started.elapsed().as_micros();
            let emit_started = std::time::Instant::now();
            let staged_output = generate_pg_table_src::emit_generate_pg_table(validated);
            let emit_us = emit_started.elapsed().as_micros();
            println!(
                "measurement=generate_pg_table_typed_stages parse_us={parse_us} build_us={build_us} validate_us={validate_us} emit_us={emit_us} output_bytes={}",
                staged_output.to_string().len()
            );
            let generate_pg_table_measurement = (0..DIRECT_GENERATION_REPEAT_COUNT).fold(
                (
                    u128::MAX,
                    u128_constants::ZERO,
                    u128_constants::ZERO,
                    usize_constants::ZERO,
                    usize_constants::ZERO,
                ),
                |(min_wall_us, max_wall_us, total_wall_us, _, _), _| {
                    let started = std::time::Instant::now();
                    let output = generate_pg_table_src::generate_pg_table(
                        macros_helpers::ts_writer::ProcMacro2TokenStreamRef::from(
                            generate_pg_table_input_token_stream.as_ref(),
                        ),
                    );
                    let wall_us = started.elapsed().as_micros();
                    (
                        min_wall_us.min(wall_us),
                        max_wall_us.max(wall_us),
                        total_wall_us.saturating_add(wall_us),
                        output.to_string().len(),
                        output.as_ref().clone().into_iter().count(),
                    )
                },
            );
            println!(
                "measurement=generate_pg_table_src repeat_count={} wall_min_us={} wall_total_us={} wall_max_us={} output_bytes={} output_token_trees={}",
                DIRECT_GENERATION_REPEAT_COUNT,
                generate_pg_table_measurement.0,
                generate_pg_table_measurement.2,
                generate_pg_table_measurement.1,
                generate_pg_table_measurement.3,
                generate_pg_table_measurement.4
            );
            let generate_pg_table_with_tests_dir =
                std::path::Path::new(str_constants::TARGET_MEASURE_GENERATE_PG_TABLE_WITH_TESTS);
            if let Err(error) = std::fs::create_dir_all(generate_pg_table_with_tests_dir) {
                eprintln!(
                    "measurement=generate_pg_table_src_with_tests status=create_dir_failed error={error}"
                );
                std::process::exit(1);
            }
            if let Err(error) = std::fs::write(
                generate_pg_table_with_tests_dir.join(str_constants::RUSTFMT_TOML),
                str_constants::EDITION_2024_NEWLINE,
            ) {
                eprintln!(
                    "measurement=generate_pg_table_src_with_tests status=rustfmt_config_write_failed error={error}"
                );
                std::process::exit(1);
            }
            let current_dir = match std::env::current_dir() {
                Ok(value) => value,
                Err(error) => {
                    eprintln!(
                        "measurement=generate_pg_table_src_with_tests status=current_dir_failed error={error}"
                    );
                    std::process::exit(1);
                }
            };
            if let Err(error) = std::env::set_current_dir(generate_pg_table_with_tests_dir) {
                eprintln!(
                    "measurement=generate_pg_table_src_with_tests status=set_current_dir_failed error={error}"
                );
                std::process::exit(1);
            }
            let generate_pg_table_with_tests_measurement = (0..DIRECT_GENERATION_REPEAT_COUNT)
                .fold(
                    (
                        u128::MAX,
                        u128_constants::ZERO,
                        u128_constants::ZERO,
                        usize_constants::ZERO,
                        usize_constants::ZERO,
                    ),
                    |(min_wall_us, max_wall_us, total_wall_us, _, _), _| {
                        let started = std::time::Instant::now();
                        let output = generate_pg_table_src::generate_pg_table(
                            macros_helpers::ts_writer::ProcMacro2TokenStreamRef::from(
                                generate_pg_table_input_with_tests_token_stream.as_ref(),
                            ),
                        );
                        let wall_us = started.elapsed().as_micros();
                        (
                            min_wall_us.min(wall_us),
                            max_wall_us.max(wall_us),
                            total_wall_us.saturating_add(wall_us),
                            output.to_string().len(),
                            output.as_ref().clone().into_iter().count(),
                        )
                    },
                );
            if let Err(error) = std::env::set_current_dir(current_dir) {
                eprintln!(
                    "measurement=generate_pg_table_src_with_tests status=restore_current_dir_failed error={error}"
                );
                std::process::exit(1);
            }
            let generate_pg_table_tests_stage_output_path =
                generate_pg_table_with_tests_dir.join(str_constants::GENERATE_PG_TABLE_TESTS_RS);
            let generate_pg_table_tests_stage_output = match server_runtime_http::read_bounded_file(
                server_runtime_http::StdPathRef::from(
                    generate_pg_table_tests_stage_output_path.as_path(),
                ),
                server_runtime_http::BoundedReadMaximumBytes::from(
                    usize_constants::VALUE_16_777_216,
                ),
            )
            .and_then(server_runtime_http::BoundedText::try_from)
            {
                Ok(content) => (content.as_ref().len(), content.as_ref().lines().count()),
                Err(error) => {
                    eprintln!(
                        "measurement=generate_pg_table_tests_stage_output status=read_failed error={error}"
                    );
                    std::process::exit(1);
                }
            };
            println!(
                "measurement=generate_pg_table_src_with_tests repeat_count={} wall_min_us={} wall_total_us={} wall_max_us={} output_bytes={} output_token_trees={}",
                DIRECT_GENERATION_REPEAT_COUNT,
                generate_pg_table_with_tests_measurement.0,
                generate_pg_table_with_tests_measurement.2,
                generate_pg_table_with_tests_measurement.1,
                generate_pg_table_with_tests_measurement.3,
                generate_pg_table_with_tests_measurement.4
            );
            println!(
                "measurement=generate_pg_table_tests_stage_output bytes={} lines={}",
                generate_pg_table_tests_stage_output.0, generate_pg_table_tests_stage_output.1
            );
            println!(
                "measurement=generate_pg_table_tests_emit_delta repeat_count={} wall_total_delta_us={} wall_min_delta_us={} wall_max_delta_us={} output_bytes_delta={}",
                DIRECT_GENERATION_REPEAT_COUNT,
                generate_pg_table_with_tests_measurement
                    .2
                    .saturating_sub(generate_pg_table_measurement.2),
                generate_pg_table_with_tests_measurement
                    .0
                    .saturating_sub(generate_pg_table_measurement.0),
                generate_pg_table_with_tests_measurement
                    .1
                    .saturating_sub(generate_pg_table_measurement.1),
                generate_pg_table_with_tests_measurement
                    .3
                    .saturating_sub(generate_pg_table_measurement.3)
            );
            let generate_pg_types_input_token_stream = quote::quote! {
                {
                    "pg_table_cols_write_into_file": "False",
                    "whole_write_into_file": "False",
                    "variant": "All"
                }
            };
            let pg_types_parse_started = std::time::Instant::now();
            let parsed_pg_types = generate_pg_types_src::parse_generate_pg_types(
                macros_helpers::ts_writer::ProcMacro2TokenStreamRef::from(
                    &generate_pg_types_input_token_stream,
                ),
            )
            .unwrap_or_else(|error| panic!("a19c725e: {error}"));
            let pg_types_parse_us = pg_types_parse_started.elapsed().as_micros();
            let pg_types_build_started = std::time::Instant::now();
            let built_pg_types = generate_pg_types_src::build_generate_pg_types(parsed_pg_types)
                .unwrap_or_else(|error| panic!("c47612bd: {error}"));
            let pg_types_build_us = pg_types_build_started.elapsed().as_micros();
            let pg_types_validate_started = std::time::Instant::now();
            let validated_pg_types =
                generate_pg_types_src::validate_generate_pg_types(built_pg_types)
                    .unwrap_or_else(|error| panic!("d3e581a4: {error}"));
            let pg_types_validate_us = pg_types_validate_started.elapsed().as_micros();
            let pg_types_emit_started = std::time::Instant::now();
            let staged_pg_types = generate_pg_types_src::emit_generate_pg_types(validated_pg_types);
            let pg_types_emit_us = pg_types_emit_started.elapsed().as_micros();
            println!(
                "measurement=generate_pg_types_typed_stages parse_us={pg_types_parse_us} build_us={pg_types_build_us} validate_us={pg_types_validate_us} emit_us={pg_types_emit_us} output_bytes={}",
                staged_pg_types.to_string().len()
            );
            let generate_pg_types_measurement = (0..DIRECT_GENERATION_REPEAT_COUNT).fold(
                (
                    u128::MAX,
                    u128_constants::ZERO,
                    u128_constants::ZERO,
                    usize_constants::ZERO,
                    usize_constants::ZERO,
                ),
                |(min_wall_us, max_wall_us, total_wall_us, _, _), _| {
                    let started = std::time::Instant::now();
                    let output = generate_pg_types_src::generate_pg_types(
                        macros_helpers::ts_writer::ProcMacro2TokenStreamRef::from(
                            &generate_pg_types_input_token_stream,
                        ),
                    );
                    let wall_us = started.elapsed().as_micros();
                    (
                        min_wall_us.min(wall_us),
                        max_wall_us.max(wall_us),
                        total_wall_us.saturating_add(wall_us),
                        output.to_string().len(),
                        output.as_ref().clone().into_iter().count(),
                    )
                },
            );
            println!(
                "measurement=generate_pg_types_src repeat_count={} wall_min_us={} wall_total_us={} wall_max_us={} output_bytes={} output_token_trees={}",
                DIRECT_GENERATION_REPEAT_COUNT,
                generate_pg_types_measurement.0,
                generate_pg_types_measurement.2,
                generate_pg_types_measurement.1,
                generate_pg_types_measurement.3,
                generate_pg_types_measurement.4
            );
            let generate_where_filters_input_token_stream = quote::quote! {
                {
                    "pg_types_write_into_file": "False",
                    "whole_write_into_file": "False"
                }
            };
            let where_filters_parse_started = std::time::Instant::now();
            let parsed_where_filters = generate_where_filters_src::parse_generate_where_filters(
                generate_where_filters_src::ProcMacro2GenerateWhereFiltersInput::from(
                    &generate_where_filters_input_token_stream,
                ),
            )
            .unwrap_or_else(|error| panic!("8f246dc1: {error}"));
            let where_filters_parse_us = where_filters_parse_started.elapsed().as_micros();
            let where_filters_build_started = std::time::Instant::now();
            let built_where_filters =
                generate_where_filters_src::build_generate_where_filters(parsed_where_filters)
                    .unwrap_or_else(|error| panic!("912f6bce: {error}"));
            let where_filters_build_us = where_filters_build_started.elapsed().as_micros();
            let where_filters_validate_started = std::time::Instant::now();
            let validated_where_filters =
                generate_where_filters_src::validate_generate_where_filters(built_where_filters)
                    .unwrap_or_else(|error| panic!("54b73a29: {error}"));
            let where_filters_validate_us = where_filters_validate_started.elapsed().as_micros();
            let where_filters_emit_started = std::time::Instant::now();
            let staged_where_filters =
                generate_where_filters_src::emit_generate_where_filters(validated_where_filters);
            let where_filters_emit_us = where_filters_emit_started.elapsed().as_micros();
            println!(
                "measurement=generate_where_filters_typed_stages parse_us={where_filters_parse_us} build_us={where_filters_build_us} validate_us={where_filters_validate_us} emit_us={where_filters_emit_us} output_bytes={}",
                staged_where_filters.to_string().len()
            );
            let generate_where_filters_measurement = (0..DIRECT_GENERATION_REPEAT_COUNT).fold(
                (
                    u128::MAX,
                    u128_constants::ZERO,
                    u128_constants::ZERO,
                    usize_constants::ZERO,
                    usize_constants::ZERO,
                ),
                |(min_wall_us, max_wall_us, total_wall_us, _, _), _| {
                    let started = std::time::Instant::now();
                    let output = generate_where_filters_src::generate_where_filters(
                        generate_where_filters_src::ProcMacro2GenerateWhereFiltersInput::from(
                            &generate_where_filters_input_token_stream,
                        ),
                    );
                    let wall_us = started.elapsed().as_micros();
                    (
                        min_wall_us.min(wall_us),
                        max_wall_us.max(wall_us),
                        total_wall_us.saturating_add(wall_us),
                        output.to_string().len(),
                        output.as_ref().clone().into_iter().count(),
                    )
                },
            );
            println!(
                "measurement=generate_where_filters_src repeat_count={} wall_min_us={} wall_total_us={} wall_max_us={} output_bytes={} output_token_trees={}",
                DIRECT_GENERATION_REPEAT_COUNT,
                generate_where_filters_measurement.0,
                generate_where_filters_measurement.2,
                generate_where_filters_measurement.1,
                generate_where_filters_measurement.3,
                generate_where_filters_measurement.4
            );
            let pg_crud_common_query_part: Result<
                (u128, u128, u128, usize),
                pg_crud_common::QueryPartError,
            > = (0..SQL_BUILDER_MEASURE_SERIES_COUNT).try_fold(
                (
                    u128::MAX,
                    u128_constants::ZERO,
                    u128_constants::ZERO,
                    usize_constants::ZERO,
                ),
                |(min_wall_us, max_wall_us, total_wall_us, _), _| {
                    let started = std::time::Instant::now();
                    let output_bytes = (0..MEASURE_REPEAT_COUNT).try_fold(
                        usize_constants::ZERO,
                        |accumulator, _| {
                            let mut increment = u64_constants::ZERO;
                            match pg_crud_common::PgTypeWhereFilter::query_part(
                                &pg_crud_common::PaginationBase::default(),
                                &mut increment,
                                pg_crud_common::SqlColumnRef::from(&str_constants::COLUMN),
                                pg_crud_common::AddOperator::from(false),
                            ) {
                                Ok(fragment) => {
                                    Ok(accumulator.saturating_add(fragment.as_ref().len()))
                                }
                                Err(error) => Err(error),
                            }
                        },
                    )?;
                    let wall_us = started.elapsed().as_micros();
                    Ok((
                        min_wall_us.min(wall_us),
                        max_wall_us.max(wall_us),
                        total_wall_us.saturating_add(wall_us),
                        output_bytes,
                    ))
                },
            );
            match pg_crud_common_query_part {
                Ok((min_wall_us, max_wall_us, total_wall_us, output_bytes)) => {
                    println!(
                        "measurement=pg_crud_common_query_part series_count={SQL_BUILDER_MEASURE_SERIES_COUNT} repeat_count={MEASURE_REPEAT_COUNT} wall_min_us={min_wall_us} wall_total_us={total_wall_us} wall_max_us={max_wall_us} output_bytes={output_bytes}"
                    );
                }
                Err(error) => {
                    eprintln!(
                        "measurement=pg_crud_common_query_part status=failed error={error:?}"
                    );
                    std::process::exit(1);
                }
            }
            let where_filters_values = (i32_constants::ZERO..64i32).collect::<Vec<i32>>();
            let where_filters_bounded_vec = match where_filters::BoundedVec::<i32, 64>::try_from(
                where_filters_values,
            ) {
                Ok(value) => value,
                Err(error) => {
                    eprintln!(
                        "measurement=where_filters_query_part status=setup_failed error={error:?}"
                    );
                    std::process::exit(1);
                }
            };
            let where_filters_query_part: Result<
                (u128, u128, u128, usize),
                pg_crud_common::QueryPartError,
            > = (0..SQL_BUILDER_MEASURE_SERIES_COUNT).try_fold(
                (
                    u128::MAX,
                    u128_constants::ZERO,
                    u128_constants::ZERO,
                    usize_constants::ZERO,
                ),
                |(min_wall_us, max_wall_us, total_wall_us, _), _| {
                    let started = std::time::Instant::now();
                    let output_bytes = (0..MEASURE_REPEAT_COUNT).try_fold(
                        usize_constants::ZERO,
                        |accumulator, _| {
                            let mut increment = u64_constants::ZERO;
                            match where_filters_bounded_vec.pg_type_query_part(
                                &mut increment,
                                pg_crud_common::SqlColumnRef::from(&str_constants::COLUMN),
                                pg_crud_common::AddOperator::from(false),
                            ) {
                                Ok(fragment) => {
                                    Ok(accumulator.saturating_add(fragment.as_ref().len()))
                                }
                                Err(error) => Err(error),
                            }
                        },
                    )?;
                    let wall_us = started.elapsed().as_micros();
                    Ok((
                        min_wall_us.min(wall_us),
                        max_wall_us.max(wall_us),
                        total_wall_us.saturating_add(wall_us),
                        output_bytes,
                    ))
                },
            );
            match where_filters_query_part {
                Ok((min_wall_us, max_wall_us, total_wall_us, output_bytes)) => {
                    println!(
                        "measurement=where_filters_query_part series_count={SQL_BUILDER_MEASURE_SERIES_COUNT} repeat_count={MEASURE_REPEAT_COUNT} wall_min_us={min_wall_us} wall_total_us={total_wall_us} wall_max_us={max_wall_us} output_bytes={output_bytes}"
                    );
                    Ok(())
                }
                Err(error) => {
                    eprintln!("measurement=where_filters_query_part status=failed error={error:?}");
                    Err(())
                }
            }
        }
        Some(str_constants::ALL_ALT) => execution::run_commands(execution::CommandsRef::from(
            &str_constants::WORKSPACE_TEST_RUNNER_STATIC_COMMANDS,
        ))
        .and_then(|()| run_workspace_tests())
        .and_then(|()| {
            macro_generation_measurements()
                .iter()
                .try_fold((), |(), (measurement_name, args)| {
                    measure_cargo_command(*measurement_name, *args)
                })
        }),
        Some(other) => {
            eprintln!(
                "unknown mode `{other}`; expected `static`, `database`, `tests`, `heavy-load`, `release`, `macro-generation`, `measure`, `all`, or `alloc-workload-*`"
            );
            Err(())
        }
    };
    if result.is_err() {
        std::process::exit(1);
    }
}
#[cfg(test)]
mod tests;
