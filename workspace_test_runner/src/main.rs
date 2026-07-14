mod discovery;
mod execution;
mod reporting;
const FORMAT_QUERY_PART_FRAGMENT: &str = "QueryPartFragment :: try_from (format !";
const GENERATE_PG_TABLE_WORKLOAD: &str = "alloc-workload-generate-pg-table-src";
const GENERATE_PG_TYPES_WORKLOAD: &str = "alloc-workload-generate-pg-types-src";
const PG_CRUD_COMMON_QUERY_PART_WORKLOAD: &str = "alloc-workload-pg-crud-common-query_part";
const STD_FMT_WRITE_CALL: &str = "std :: fmt :: Write :: write_fmt";
const STRING_WITH_CAPACITY_CALL: &str = "String :: with_capacity";
const STATIC_WORKSPACE_PROFILE: &str = "static_workspace";
const WHERE_FILTERS_QUERY_PART_WORKLOAD: &str = "alloc-workload-where-filters-query_part";
const CARGO_FMT_CHECK_ARGS: [&str; 2] = ["fmt", "--check"];
const CARGO_CLIPPY_ARGS: [&str; 7] = [
    "clippy",
    "--locked",
    "--all-targets",
    "--all-features",
    "--",
    "-D",
    "warnings",
];
const CARGO_TEST_STYLE_ARGS: [&str; 5] = ["test", "--locked", "-p", "tests", "--lib"];
const CARGO_TEST_GEN_PG_TBL_ARGS: [&str; 6] = [
    "test",
    "--locked",
    "-p",
    "generate_pg_table_test",
    "--features",
    "test-utils",
];
const CARGO_TEST_GEN_PG_TYPES_ARGS: [&str; 6] = [
    "test",
    "--locked",
    "-p",
    "generate_pg_types_test",
    "--features",
    "test-utils",
];
const CARGO_TEST_GEN_WH_FLTS_ARGS: [&str; 6] = [
    "test",
    "--locked",
    "-p",
    "generate_where_filters_test",
    "--features",
    "test-utils",
];
const CARGO_TEST_DATABASE_ARGS: [&str; 4] = ["test", "--locked", "--features", "test-utils"];
const CARGO_TEST_WORKSPACE_ARGS: [&str; 5] = [
    "test",
    "--locked",
    "--workspace",
    "--all-features",
    "--no-fail-fast",
];
const CARGO_TEST_IGNORED_ARGS: [&str; 7] = [
    "test",
    "--locked",
    "--workspace",
    "--all-features",
    "--no-fail-fast",
    "--",
    "--ignored",
];
const CARGO_TEST_DOC_ARGS: [&str; 5] =
    ["test", "--locked", "--workspace", "--doc", "--all-features"];
const NEXTEST_WORKSPACE_ARGS: [&str; 7] = [
    "nextest",
    "run",
    "--no-fail-fast",
    "--workspace",
    "--all-features",
    "-P",
    STATIC_WORKSPACE_PROFILE,
];
const NEXTEST_IGNORED_ARGS: [&str; 9] = [
    "nextest",
    "run",
    "--no-fail-fast",
    "--workspace",
    "--all-features",
    "-P",
    STATIC_WORKSPACE_PROFILE,
    "--run-ignored",
    "only",
];
const NEXTEST_HEAVY_ARGS: [&str; 7] = [
    "nextest",
    "run",
    "--no-fail-fast",
    "--workspace",
    "--all-features",
    "-P",
    "heavy_load",
];
const DIRECT_GENERATION_REPEAT_COUNT: usize = 5;
const MEASURE_REPEAT_COUNT: usize = 1000;
const SQL_BUILDER_MEASURE_SERIES_COUNT: usize = 5;
const STATIC_COMMANDS: [(&str, &[&str]); 3] = [
    ("cargo", &CARGO_FMT_CHECK_ARGS),
    ("cargo", &CARGO_CLIPPY_ARGS),
    ("cargo", &CARGO_TEST_STYLE_ARGS),
];
const CARGO_TEST_COMMANDS: [(&str, &[&str]); 3] = [
    ("cargo", &CARGO_TEST_WORKSPACE_ARGS),
    ("cargo", &CARGO_TEST_IGNORED_ARGS),
    ("cargo", &CARGO_TEST_DOC_ARGS),
];
const NEXTEST_COMMANDS: [(&str, &[&str]); 3] = [
    ("cargo", &NEXTEST_WORKSPACE_ARGS),
    ("cargo", &NEXTEST_IGNORED_ARGS),
    ("cargo", &CARGO_TEST_DOC_ARGS),
];
const MACRO_GENERATION_MEASUREMENTS: [(MeasurementName, CargoArgs); 3] = [
    (
        MeasurementName("macro_generation_generate_pg_table_test"),
        CargoArgs(&CARGO_TEST_GEN_PG_TBL_ARGS),
    ),
    (
        MeasurementName("macro_generation_generate_pg_types_test"),
        CargoArgs(&CARGO_TEST_GEN_PG_TYPES_ARGS),
    ),
    (
        MeasurementName("macro_generation_generate_where_filters_test"),
        CargoArgs(&CARGO_TEST_GEN_WH_FLTS_ARGS),
    ),
];
const PEAK_RSS_PREFIX: &str = "codex_peak_rss_kb=";
const MINOR_PAGE_FAULTS_PREFIX: &str = "codex_minor_page_faults=";
const MAJOR_PAGE_FAULTS_PREFIX: &str = "codex_major_page_faults=";
const MEMUSAGE_PATH: &str = "/usr/lib/x86_64-linux-gnu/libmemusage.so";
const CLEAN_ANSI_TEXT_MAX_LEN: usize = 16_777_216;
const ALLOCATION_TOOLS: [AllocationTool; 6] = [
    AllocationTool {
        name: ToolName("libmemusage"),
        path: ToolPath(MEMUSAGE_PATH),
    },
    AllocationTool {
        name: ToolName("valgrind"),
        path: ToolPath("/usr/bin/valgrind"),
    },
    AllocationTool {
        name: ToolName("heaptrack"),
        path: ToolPath("/usr/bin/heaptrack"),
    },
    AllocationTool {
        name: ToolName("ltrace"),
        path: ToolPath("/usr/bin/ltrace"),
    },
    AllocationTool {
        name: ToolName("perf"),
        path: ToolPath("/usr/bin/perf"),
    },
    AllocationTool {
        name: ToolName("time"),
        path: ToolPath("/usr/bin/time"),
    },
];
#[derive(Clone, Copy)]
struct MeasurementName(&'static str);
impl MeasurementName {
    const fn get(self) -> &'static str {
        self.0
    }
}
#[derive(Clone, Copy)]
struct CargoArgs(&'static [&'static str]);
impl CargoArgs {
    const fn get(self) -> &'static [&'static str] {
        self.0
    }
}
#[derive(Clone, Copy, newtype::Newtype)]
#[newtype(from_inner)]
struct StderrTextRef<'lt>(&'lt str);
impl<'lt> StderrTextRef<'lt> {
    const fn get(self) -> &'lt str {
        self.0
    }
}
#[derive(Clone, Copy)]
struct AnsiTextRef<'lt>(&'lt str);
impl<'lt> AnsiTextRef<'lt> {
    const fn get(self) -> &'lt str {
        self.0
    }
}
#[derive(newtype::BoundedString)]
#[bounded_string(max = CLEAN_ANSI_TEXT_MAX_LEN)]
struct CleanAnsiText(String);
#[derive(Clone, Copy)]
struct MemusageKey(&'static str);
impl MemusageKey {
    const fn get(self) -> &'static str {
        self.0
    }
}
#[derive(Clone, Copy)]
struct MemusageRowName(&'static str);
impl MemusageRowName {
    const fn get(self) -> &'static str {
        self.0
    }
}
#[derive(Clone, Copy)]
struct MemusageColumnIdx(usize);
impl MemusageColumnIdx {
    const fn get(self) -> usize {
        self.0
    }
}
#[derive(Clone, Copy)]
struct MemusageValueRef<'lt>(&'lt str);
impl<'lt> MemusageValueRef<'lt> {
    const fn get(self) -> &'lt str {
        self.0
    }
}
#[derive(Clone, Copy)]
struct ProgramPathRef<'lt>(&'lt str);
impl<'lt> ProgramPathRef<'lt> {
    const fn get(self) -> &'lt str {
        self.0
    }
}
#[derive(Clone, Copy)]
struct ProgramArgsRef<'lt>(&'lt [&'lt str]);
impl<'lt> ProgramArgsRef<'lt> {
    const fn get(self) -> &'lt [&'lt str] {
        self.0
    }
}
#[derive(Clone, Copy)]
struct MemusageProgNameRef<'lt>(&'lt str);
impl<'lt> MemusageProgNameRef<'lt> {
    const fn get(self) -> &'lt str {
        self.0
    }
}
#[derive(Clone)]
struct QuoteTokenStreamGeneratePgTableMeasureInputTokenStream(quote::__private::TokenStream);
impl AsRef<quote::__private::TokenStream>
    for QuoteTokenStreamGeneratePgTableMeasureInputTokenStream
{
    fn as_ref(&self) -> &quote::__private::TokenStream {
        &self.0
    }
}
#[derive(Clone, Copy)]
struct ToolName(&'static str);
impl ToolName {
    const fn get(self) -> &'static str {
        self.0
    }
}
#[derive(Clone, Copy)]
struct ToolPath(&'static str);
impl ToolPath {
    const fn get(self) -> &'static str {
        self.0
    }
}
#[derive(Clone, Copy)]
struct AllocationTool {
    name: ToolName,
    path: ToolPath,
}
fn print_without_measurement_footer(stderr: StderrTextRef<'_>) {
    stderr
        .get()
        .lines()
        .filter(|line| !line.trim().starts_with(PEAK_RSS_PREFIX))
        .filter(|line| !line.trim().starts_with(MINOR_PAGE_FAULTS_PREFIX))
        .filter(|line| !line.trim().starts_with(MAJOR_PAGE_FAULTS_PREFIX))
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
    CleanAnsiText::try_from(clean).unwrap_or_else(|_| CleanAnsiText(String::new()))
}
fn print_without_memusage_footer(stderr: StderrTextRef<'_>) {
    let clean = strip_ansi_codes(AnsiTextRef(stderr.get()));
    clean
        .0
        .as_str()
        .lines()
        .take_while(|line| !line.contains("Memory usage summary:"))
        .filter(|line| !line.trim().is_empty())
        .for_each(|line| eprintln!("{line}"));
}
fn memusage_heap_value(text: &CleanAnsiText, key: MemusageKey) -> MemusageValueRef<'_> {
    text.0
        .as_str()
        .lines()
        .find_map(|line| line.split_once(key.get()).map(|(_, tail)| tail.trim()))
        .and_then(|tail| tail.split([',', ' ']).find(|part| !part.is_empty()))
        .map_or(MemusageValueRef("unavailable"), MemusageValueRef)
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
        .map_or(MemusageValueRef("unavailable"), MemusageValueRef)
}
fn measure_memusage_command(
    measurement_name: MeasurementName,
    program: ProgramPathRef<'_>,
    args: ProgramArgsRef<'_>,
    memusage_prog_name: MemusageProgNameRef<'_>,
) -> Result<(), ()> {
    let measurement_name_value = measurement_name.get();
    if !std::path::Path::new(MEMUSAGE_PATH).exists() {
        println!(
            "measurement={measurement_name_value}_allocations status=unavailable reason=libmemusage_not_found path={MEMUSAGE_PATH}"
        );
        return Ok(());
    }
    let command_output = macros_helpers::tool_command::ToolCommand::new(
        macros_helpers::tool_command::ToolProgramRef::from(program.get()),
    )
    .args(macros_helpers::tool_command::ToolArgsRef::from(args.get()))
    .env(
        macros_helpers::tool_command::ToolEnvKeyRef::from("LD_PRELOAD"),
        macros_helpers::tool_command::ToolEnvValueRef::from(MEMUSAGE_PATH),
    )
    .env(
        macros_helpers::tool_command::ToolEnvKeyRef::from("MEMUSAGE_PROG_NAME"),
        macros_helpers::tool_command::ToolEnvValueRef::from(memusage_prog_name.get()),
    )
    .output();
    match command_output {
        Ok(output) if output.status.success() => {
            let stdout = String::from_utf8_lossy(output.stdout.as_slice());
            if !stdout.is_empty() {
                print!("{stdout}");
            }
            let stderr = String::from_utf8_lossy(output.stderr.as_slice());
            print_without_memusage_footer(StderrTextRef::from(stderr.as_ref()));
            let clean = strip_ansi_codes(AnsiTextRef(stderr.as_ref()));
            let heap_total = memusage_heap_value(&clean, MemusageKey("heap total:")).get();
            let heap_peak = memusage_heap_value(&clean, MemusageKey("heap peak:")).get();
            let stack_peak = memusage_heap_value(&clean, MemusageKey("stack peak:")).get();
            let malloc_calls =
                memusage_table_value(&clean, MemusageRowName("malloc|"), MemusageColumnIdx(0))
                    .get();
            let malloc_memory =
                memusage_table_value(&clean, MemusageRowName("malloc|"), MemusageColumnIdx(1))
                    .get();
            let malloc_failed =
                memusage_table_value(&clean, MemusageRowName("malloc|"), MemusageColumnIdx(2))
                    .get();
            let realloc_calls =
                memusage_table_value(&clean, MemusageRowName("realloc|"), MemusageColumnIdx(0))
                    .get();
            let realloc_memory =
                memusage_table_value(&clean, MemusageRowName("realloc|"), MemusageColumnIdx(1))
                    .get();
            let realloc_failed =
                memusage_table_value(&clean, MemusageRowName("realloc|"), MemusageColumnIdx(2))
                    .get();
            let calloc_calls =
                memusage_table_value(&clean, MemusageRowName("calloc|"), MemusageColumnIdx(0))
                    .get();
            let calloc_memory =
                memusage_table_value(&clean, MemusageRowName("calloc|"), MemusageColumnIdx(1))
                    .get();
            let calloc_failed =
                memusage_table_value(&clean, MemusageRowName("calloc|"), MemusageColumnIdx(2))
                    .get();
            let free_calls =
                memusage_table_value(&clean, MemusageRowName("free|"), MemusageColumnIdx(0)).get();
            let free_memory =
                memusage_table_value(&clean, MemusageRowName("free|"), MemusageColumnIdx(1)).get();
            println!(
                "measurement={measurement_name_value}_allocations status=ok tool=libmemusage heap_total_bytes={heap_total} heap_peak_bytes={heap_peak} stack_peak_bytes={stack_peak} malloc_calls={malloc_calls} malloc_bytes={malloc_memory} malloc_failed={malloc_failed} realloc_calls={realloc_calls} realloc_bytes={realloc_memory} realloc_failed={realloc_failed} calloc_calls={calloc_calls} calloc_bytes={calloc_memory} calloc_failed={calloc_failed} free_calls={free_calls} free_bytes={free_memory}"
            );
            Ok(())
        }
        Ok(output) => {
            let stdout = String::from_utf8_lossy(output.stdout.as_slice());
            if !stdout.is_empty() {
                print!("{stdout}");
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
    let measurement_format =
        format!("{PEAK_RSS_PREFIX}%M\n{MINOR_PAGE_FAULTS_PREFIX}%R\n{MAJOR_PAGE_FAULTS_PREFIX}%F");
    let command_output = macros_helpers::tool_command::ToolCommand::new(
        macros_helpers::tool_command::ToolProgramRef::from("/usr/bin/time"),
    )
    .arg(macros_helpers::tool_command::ToolArgRef::from("-f"))
    .arg(macros_helpers::tool_command::ToolArgRef::from(
        measurement_format.as_str(),
    ))
    .arg(macros_helpers::tool_command::ToolArgRef::from("cargo"))
    .args(macros_helpers::tool_command::ToolArgsRef::from(args.get()))
    .output();
    let duration = started.elapsed();
    match command_output {
        Ok(output) if output.status.success() => {
            let stderr = String::from_utf8_lossy(output.stderr.as_slice());
            let peak_rss_kb = stderr
                .lines()
                .find_map(|line| line.trim().strip_prefix(PEAK_RSS_PREFIX))
                .unwrap_or("unavailable");
            let minor_page_faults = stderr
                .lines()
                .find_map(|line| line.trim().strip_prefix(MINOR_PAGE_FAULTS_PREFIX))
                .unwrap_or("unavailable");
            let major_page_faults = stderr
                .lines()
                .find_map(|line| line.trim().strip_prefix(MAJOR_PAGE_FAULTS_PREFIX))
                .unwrap_or("unavailable");
            let stdout = String::from_utf8_lossy(output.stdout.as_slice());
            if !stdout.is_empty() {
                print!("{stdout}");
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
            let stdout = String::from_utf8_lossy(output.stdout.as_slice());
            if !stdout.is_empty() {
                print!("{stdout}");
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
    QuoteTokenStreamGeneratePgTableMeasureInputTokenStream(quote::quote! {
        #allow_clippy_arbitrary_src_item_ordering
        #[derive(Debug, Clone, Copy, optml::Optml)]
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
        #[generate_pg_table::cm_error_variants{enum CmErrorVariants{}}]
        #[generate_pg_table::co_error_variants{enum CoErrorVariants{}}]
        #[generate_pg_table::rm_error_variants{enum RmErrorVariants{}}]
        #[generate_pg_table::ro_error_variants{enum RoErrorVariants{}}]
        #[generate_pg_table::um_error_variants{enum UmErrorVariants{}}]
        #[generate_pg_table::uo_error_variants{enum UoErrorVariants{}}]
        #[generate_pg_table::dm_error_variants{enum DmErrorVariants{}}]
        #[generate_pg_table::dlo_error_variants{enum DloErrorVariants{}}]
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
    let output_bytes = (0..DIRECT_GENERATION_REPEAT_COUNT).fold(0usize, |accumulator, _| {
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
    let output_bytes = (0..DIRECT_GENERATION_REPEAT_COUNT).fold(0usize, |accumulator, _| {
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
        (0..SQL_BUILDER_MEASURE_SERIES_COUNT).try_fold(0usize, |series_accumulator, _| {
            (0..MEASURE_REPEAT_COUNT).try_fold(series_accumulator, |accumulator, _| {
                let mut increment = 0u64;
                match pg_crud_common::PgTypeWhereFilter::query_part(
                    &pg_crud_common::PaginationBase::default(),
                    &mut increment,
                    pg_crud_common::SqlColumnRef::from(&"column"),
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
    let where_filters_values = (0i32..64i32).collect::<Vec<i32>>();
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
        (0..SQL_BUILDER_MEASURE_SERIES_COUNT).try_fold(0usize, |series_accumulator, _| {
            (0..MEASURE_REPEAT_COUNT).try_fold(series_accumulator, |accumulator, _| {
                let mut increment = 0u64;
                match where_filters_bounded_vec.pg_type_query_part(
                    &mut increment,
                    pg_crud_common::SqlColumnRef::from(&"column"),
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
fn cargo_subcommand_available(subcommand: &str) -> bool {
    let args = [subcommand, "--version"];
    macros_helpers::tool_command::ToolCommand::new(
        macros_helpers::tool_command::ToolProgramRef::from("cargo"),
    )
    .args(macros_helpers::tool_command::ToolArgsRef::from(
        args.as_slice(),
    ))
    .output()
    .is_ok_and(|output| output.status.success())
}
#[allow(
    clippy::needless_for_each,
    clippy::single_call_fn,
    reason = "keeps release-tool reporting separate and repository policy forbids for loops"
)]
fn print_optional_release_tools() {
    ["semver-checks", "udeps", "machete", "llvm-cov"]
        .into_iter()
        .for_each(|tool| {
            println!(
                "release_tool={tool} available={}",
                cargo_subcommand_available(tool)
            );
        });
}
fn run_workspace_tests() -> Result<(), ()> {
    if cargo_subcommand_available("nextest") {
        println!("test_executor=nextest");
        execution::run_commands(&NEXTEST_COMMANDS)
    } else {
        println!("test_executor=cargo fallback=true");
        execution::run_commands(&CARGO_TEST_COMMANDS)
    }
}
fn main() {
    let mode = discovery::mode();
    let result = match mode.as_deref() {
        None | Some("static") => execution::run_commands(&STATIC_COMMANDS),
        Some("database") => match std::env::var("DATABASE_URL") {
            Ok(database_url) => match macros_helpers::test_database::validate_test_database_url(
                macros_helpers::test_database::UrlRef::from(database_url.as_str()),
            ) {
                Ok(_target) => execution::run_commands(&[("cargo", &CARGO_TEST_DATABASE_ARGS)]),
                Err(error) => {
                    eprintln!("database test guard rejected DATABASE_URL: {error}");
                    Err(())
                }
            },
            Err(error) => {
                eprintln!("database test mode requires DATABASE_URL: {error}");
                Err(())
            }
        },
        Some(GENERATE_PG_TABLE_WORKLOAD) => {
            run_alloc_workload_generate_pg_table_src();
            Ok(())
        }
        Some(GENERATE_PG_TYPES_WORKLOAD) => {
            run_alloc_workload_generate_pg_types_src();
            Ok(())
        }
        Some(PG_CRUD_COMMON_QUERY_PART_WORKLOAD) => run_alloc_workload_pg_crud_common_query_part(),
        Some(WHERE_FILTERS_QUERY_PART_WORKLOAD) => run_alloc_workload_where_filters_query_part(),
        Some("macro-generation") => MACRO_GENERATION_MEASUREMENTS
            .iter()
            .try_fold((), |(), (measurement_name, args)| {
                measure_cargo_command(*measurement_name, *args)
            }),
        Some("tests") => run_workspace_tests(),
        Some("heavy-load") => {
            if cargo_subcommand_available("nextest") {
                execution::run_commands(&[("cargo", &NEXTEST_HEAVY_ARGS)])
            } else {
                eprintln!("heavy-load mode requires cargo-nextest; optional tool is unavailable");
                Err(())
            }
        }
        Some("release") => {
            print_optional_release_tools();
            execution::run_commands(&STATIC_COMMANDS).and_then(|()| run_workspace_tests())
        }
        Some("measure") => {
            let allocation_tools_printed: Result<(), std::convert::Infallible> =
                ALLOCATION_TOOLS.iter().try_fold((), |(), tool| {
                    let available = discovery::tool_available(tool.path.get());
                    let name = tool.name.get();
                    let path = tool.path.get();
                    reporting::allocation_tool(name, path, available);
                    Ok(())
                });
            match allocation_tools_printed {
                Ok(()) => {}
                Err(error) => match error {},
            }
            if std::path::Path::new(MEMUSAGE_PATH).exists() {
                println!(
                    "measurement=exact_allocations status=available tool=libmemusage path={MEMUSAGE_PATH}"
                );
                measure_memusage_command(
                    MeasurementName("code_style"),
                    ProgramPathRef("cargo"),
                    ProgramArgsRef(&["test", "-p", "tests", "code_style"]),
                    MemusageProgNameRef("cargo"),
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
                    .unwrap_or("workspace_test_runner");
                [
                    (
                        MeasurementName("generate_pg_table_src"),
                        GENERATE_PG_TABLE_WORKLOAD,
                    ),
                    (
                        MeasurementName("generate_pg_types_src"),
                        GENERATE_PG_TYPES_WORKLOAD,
                    ),
                    (
                        MeasurementName("pg_crud_common_query_part"),
                        PG_CRUD_COMMON_QUERY_PART_WORKLOAD,
                    ),
                    (
                        MeasurementName("where_filters_query_part"),
                        WHERE_FILTERS_QUERY_PART_WORKLOAD,
                    ),
                ]
                .into_iter()
                .try_fold((), |(), (measurement_name, workload_mode)| {
                    measure_memusage_command(
                        measurement_name,
                        ProgramPathRef(current_exe_string.as_str()),
                        ProgramArgsRef(&[workload_mode]),
                        MemusageProgNameRef(current_exe_prog_name),
                    )
                })
                .unwrap_or_else(|()| std::process::exit(1));
            } else {
                println!(
                    "measurement=exact_allocations status=unavailable reason=no_safe_allocator_counter_or_external_allocation_profiler memory_proxy_fields=memory_proxy_peak_rss_kb,memory_proxy_minor_page_faults,memory_proxy_major_page_faults"
                );
            }
            measure_cargo_command(
                MeasurementName("code_style"),
                CargoArgs(&["test", "-p", "tests", "code_style"]),
            )
            .unwrap_or_else(|()| std::process::exit(1));
            measure_cargo_command(MeasurementName("clippy"), CargoArgs(&CARGO_CLIPPY_ARGS))
                .unwrap_or_else(|()| std::process::exit(1));
            let generate_pg_table_input_token_stream =
                generate_pg_table_measure_input_token_stream(&quote::quote! {"False"});
            let generate_pg_table_input_with_tests_token_stream =
                generate_pg_table_measure_input_token_stream(&quote::quote! {"True"});
            let generate_pg_table_measurement = (0..DIRECT_GENERATION_REPEAT_COUNT).fold(
                (u128::MAX, 0u128, 0u128, 0usize, 0usize),
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
                std::path::Path::new("target/measure/generate_pg_table_with_tests");
            if let Err(error) = std::fs::create_dir_all(generate_pg_table_with_tests_dir) {
                eprintln!(
                    "measurement=generate_pg_table_src_with_tests status=create_dir_failed error={error}"
                );
                std::process::exit(1);
            }
            if let Err(error) = std::fs::write(
                generate_pg_table_with_tests_dir.join("rustfmt.toml"),
                "edition = \"2024\"\n",
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
                    (u128::MAX, 0u128, 0u128, 0usize, 0usize),
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
            let generate_pg_table_tests_stage_output = match std::fs::read_to_string(
                generate_pg_table_with_tests_dir.join("generate_pg_table_Tests.rs"),
            ) {
                Ok(content) => (content.len(), content.lines().count()),
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
            let generate_pg_table_src_text =
                std::fs::read_to_string("pg_crud/pg_table/generate_pg_table_src/src/lib.rs")
                    .unwrap_or_default();
            let generate_pg_table_concurrency_constants_found = [
                "CM_CHUNK_SIZE_2EE9377B",
                "CM_CONCURRENCY_7CCFD82D",
                "CM_CHUNK_SIZE_A13F7C92",
                "TEST_FUTURE_CONCURRENCY_D281414B",
            ]
            .into_iter()
            .all(|pattern| generate_pg_table_src_text.contains(pattern));
            println!(
                "measurement=generate_pg_table_generated_test_concurrency cm_chunk_2ee9377b=25 cm_concurrency_7ccfd82d=5 cm_chunk_a13f7c92=10 test_future_concurrency_d281414b=100 source_detected={generate_pg_table_concurrency_constants_found}"
            );
            let generate_pg_table_box_future_push_sites = generate_pg_table_src_text
                .matches("accumulator_9189f86e.push")
                .count();
            let generate_pg_table_old_chunk_vec_from_absent =
                !generate_pg_table_src_text.contains(".map(Vec::from)");
            let generate_pg_table_old_collect_flatten_absent =
                !generate_pg_table_src_text.contains(".flatten().collect");
            let generate_pg_table_table_names_cloned_vec_absent =
                !generate_pg_table_src_text.contains("table_names_cloned = table_names.iter().map");
            println!(
                "measurement=generate_pg_table_generated_test_concurrency_shape box_future_push_sites={generate_pg_table_box_future_push_sites} old_chunk_vec_from_absent={generate_pg_table_old_chunk_vec_from_absent} old_collect_flatten_absent={generate_pg_table_old_collect_flatten_absent} table_names_cloned_vec_absent={generate_pg_table_table_names_cloned_vec_absent}"
            );
            let generate_pg_table_pipeline_stage_source_found = [
                "parse_generate_pg_table_input_stage",
                "build_generate_pg_table_input_model_stage",
                "validate_generate_pg_table_fields_model_stage",
                "emit_generate_pg_table_tests_stage",
                "emit_generate_pg_table_final_stage",
            ]
            .into_iter()
            .all(|pattern| generate_pg_table_src_text.contains(pattern));
            println!(
                "measurement=generate_pg_table_pipeline_shape parse=true build_model=true validate=true emit_tests=true emit_final=true source_detected={generate_pg_table_pipeline_stage_source_found}"
            );
            let generate_pg_table_pipeline_stage_measurement = (0..DIRECT_GENERATION_REPEAT_COUNT).fold(
                (
                    u128::MAX, 0u128, 0u128, u128::MAX, 0u128, 0u128, u128::MAX, 0u128,
                    0u128, u128::MAX, 0u128, 0u128, u128::MAX, 0u128, 0u128, 0usize,
                    0usize, 0usize, 0usize, 0usize,
                ),
                |(
                    parse_min_us,
                    parse_max_us,
                    parse_total_us,
                    config_min_us,
                    config_max_us,
                    config_total_us,
                    model_min_us,
                    model_max_us,
                    model_total_us,
                    fields_min_us,
                    fields_max_us,
                    fields_total_us,
                    validate_min_us,
                    validate_max_us,
                    validate_total_us,
                    _,
                    _,
                    _,
                    _,
                    _,
                ),
                 _| {
                    let parse_started = std::time::Instant::now();
                    let parsed =
                        match syn::parse2::<syn::DeriveInput>(generate_pg_table_input_token_stream.as_ref().clone())
                        {
                        Ok(value) => value,
                        Err(error) => {
                            eprintln!(
                                "measurement=generate_pg_table_pipeline_stages status=parse_failed error={error}"
                            );
                            std::process::exit(1);
                        }
                    };
                    let parse_wall_us = parse_started.elapsed().as_micros();
                    let config_started = std::time::Instant::now();
                    let config_attr_token_stream = macros_helpers::attr_reader::get_macro_attr_meta_list_token_stream(
                        &parsed.attrs,
                        generate_pg_table_src::GENERATE_PG_TABLE_CONFIG_PATH,
                    );
                    let config_value =
                        match serde_json::from_str::<serde_json::Value>(&config_attr_token_stream.to_string())
                        {
                            Ok(value) => value,
                            Err(error) => {
                                eprintln!(
                                    "measurement=generate_pg_table_pipeline_stages status=config_parse_failed error={error}"
                                );
                                std::process::exit(1);
                            }
                        };
                    let config_wall_us = config_started.elapsed().as_micros();
                    let config_key_count = config_value.as_object().map_or(0usize, |value| {
                        value.len()
                    });
                    let model_started = std::time::Instant::now();
                    let error_variant_count = [
                        ("generate_pg_table::cm_error_variants", "CmErrorVariants"),
                        ("generate_pg_table::co_error_variants", "CoErrorVariants"),
                        ("generate_pg_table::rm_error_variants", "RmErrorVariants"),
                        ("generate_pg_table::ro_error_variants", "RoErrorVariants"),
                        ("generate_pg_table::um_error_variants", "UmErrorVariants"),
                        ("generate_pg_table::uo_error_variants", "UoErrorVariants"),
                        ("generate_pg_table::dm_error_variants", "DmErrorVariants"),
                        ("generate_pg_table::dlo_error_variants", "DloErrorVariants"),
                        ("generate_pg_table::common_error_variants", "CommonErrorVariants"),
                    ]
                    .into_iter()
                    .fold(0usize, |accumulator, (attr_path, expected_identifier)| {
                        let attr_token_stream = macros_helpers::attr_reader::get_macro_attr_meta_list_token_stream(
                            &parsed.attrs,
                            attr_path,
                        );
                        let Ok(parsed_attr) = syn::parse2::<syn::DeriveInput>((*attr_token_stream).clone())
                        else {
                            return accumulator;
                        };
                        if parsed_attr.ident != expected_identifier {
                            eprintln!(
                                "measurement=generate_pg_table_pipeline_stages status=model_identifier_mismatch attr={attr_path}"
                            );
                            std::process::exit(1);
                        }
                        match parsed_attr.data {
                            syn::Data::Enum(data_enum) => {
                                accumulator.saturating_add(data_enum.variants.len())
                            }
                            syn::Data::Struct(_) | syn::Data::Union(_) => accumulator,
                        }
                    });
                    let logic_attr_token_bytes = [
                        "generate_pg_table::cm_logic",
                        "generate_pg_table::co_logic",
                        "generate_pg_table::rm_logic",
                        "generate_pg_table::ro_logic",
                        "generate_pg_table::um_logic",
                        "generate_pg_table::uo_logic",
                        "generate_pg_table::dm_logic",
                        "generate_pg_table::dlo_logic",
                        "generate_pg_table::common_logic",
                    ]
                    .into_iter()
                    .fold(0usize, |accumulator, attr_path| {
                        let logic_token_stream = macros_helpers::attr_reader::get_macro_attr_meta_list_token_stream(
                            &parsed.attrs,
                            attr_path,
                        );
                        accumulator.saturating_add(logic_token_stream.to_string().len())
                    });
                    let model_wall_us = model_started.elapsed().as_micros();
                    let fields_started = std::time::Instant::now();
                    let (field_count, primary_key_attr_count) = match &parsed.data {
                        syn::Data::Struct(data_struct) => match &data_struct.fields {
                            syn::Fields::Named(fields_named) => fields_named.named.iter().fold(
                                (0usize, 0usize),
                                |(field_accumulator, primary_key_accumulator), field| {
                                    let field_primary_key_attr_count = field
                                        .attrs
                                        .iter()
                                        .filter(|attr| attr.path().segments.len() == 1)
                                        .filter(|attr| {
                                            attr.path()
                                                .segments
                                                .first()
                                                .is_some_and(|segment| {
                                                    segment.ident == "generate_pg_table_primary_key"
                                                })
                                        })
                                        .count();
                                    (
                                        field_accumulator.saturating_add(1),
                                        primary_key_accumulator.saturating_add(field_primary_key_attr_count),
                                    )
                                },
                            ),
                            syn::Fields::Unnamed(_) | syn::Fields::Unit => {
                                eprintln!(
                                    "measurement=generate_pg_table_pipeline_stages status=fields_not_named"
                                );
                                std::process::exit(1);
                            }
                        },
                        syn::Data::Enum(_) | syn::Data::Union(_) => {
                            eprintln!(
                                "measurement=generate_pg_table_pipeline_stages status=input_not_struct"
                            );
                            std::process::exit(1);
                        }
                    };
                    let fields_wall_us = fields_started.elapsed().as_micros();
                    let validate_started = std::time::Instant::now();
                    if field_count == 0usize || primary_key_attr_count != 1usize {
                        eprintln!(
                            "measurement=generate_pg_table_pipeline_stages status=validation_failed fields={field_count} primary_key_attrs={primary_key_attr_count}"
                        );
                        std::process::exit(1);
                    }
                    let validate_wall_us = validate_started.elapsed().as_micros();
                    (
                        parse_min_us.min(parse_wall_us),
                        parse_max_us.max(parse_wall_us),
                        parse_total_us.saturating_add(parse_wall_us),
                        config_min_us.min(config_wall_us),
                        config_max_us.max(config_wall_us),
                        config_total_us.saturating_add(config_wall_us),
                        model_min_us.min(model_wall_us),
                        model_max_us.max(model_wall_us),
                        model_total_us.saturating_add(model_wall_us),
                        fields_min_us.min(fields_wall_us),
                        fields_max_us.max(fields_wall_us),
                        fields_total_us.saturating_add(fields_wall_us),
                        validate_min_us.min(validate_wall_us),
                        validate_max_us.max(validate_wall_us),
                        validate_total_us.saturating_add(validate_wall_us),
                        config_key_count,
                        field_count,
                        primary_key_attr_count,
                        error_variant_count,
                        logic_attr_token_bytes,
                    )
                },
            );
            println!(
                "measurement=generate_pg_table_pipeline_stages repeat_count={DIRECT_GENERATION_REPEAT_COUNT} parse_min_us={} parse_total_us={} parse_max_us={} config_min_us={} config_total_us={} config_max_us={} model_min_us={} model_total_us={} model_max_us={} fields_min_us={} fields_total_us={} fields_max_us={} validate_min_us={} validate_total_us={} validate_max_us={} config_keys={} fields={} primary_key_attrs={} error_variants={} logic_attr_token_bytes={}",
                generate_pg_table_pipeline_stage_measurement.0,
                generate_pg_table_pipeline_stage_measurement.2,
                generate_pg_table_pipeline_stage_measurement.1,
                generate_pg_table_pipeline_stage_measurement.3,
                generate_pg_table_pipeline_stage_measurement.5,
                generate_pg_table_pipeline_stage_measurement.4,
                generate_pg_table_pipeline_stage_measurement.6,
                generate_pg_table_pipeline_stage_measurement.8,
                generate_pg_table_pipeline_stage_measurement.7,
                generate_pg_table_pipeline_stage_measurement.9,
                generate_pg_table_pipeline_stage_measurement.11,
                generate_pg_table_pipeline_stage_measurement.10,
                generate_pg_table_pipeline_stage_measurement.12,
                generate_pg_table_pipeline_stage_measurement.14,
                generate_pg_table_pipeline_stage_measurement.13,
                generate_pg_table_pipeline_stage_measurement.15,
                generate_pg_table_pipeline_stage_measurement.16,
                generate_pg_table_pipeline_stage_measurement.17,
                generate_pg_table_pipeline_stage_measurement.18,
                generate_pg_table_pipeline_stage_measurement.19
            );
            let generate_pg_types_input_token_stream = quote::quote! {
                {
                    "pg_table_cols_write_into_file": "False",
                    "whole_write_into_file": "False",
                    "variant": "All"
                }
            };
            let generate_pg_types_measurement = (0..DIRECT_GENERATION_REPEAT_COUNT).fold(
                (u128::MAX, 0u128, 0u128, 0usize, 0usize),
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
            let generate_pg_types_shape_output = generate_pg_types_src::generate_pg_types(
                macros_helpers::ts_writer::ProcMacro2TokenStreamRef::from(
                    &generate_pg_types_input_token_stream,
                ),
            )
            .to_string();
            let generate_pg_types_write_fmt_found =
                generate_pg_types_shape_output.contains(STD_FMT_WRITE_CALL);
            let generate_pg_types_with_capacity_found =
                generate_pg_types_shape_output.contains(STRING_WITH_CAPACITY_CALL);
            let generate_pg_types_old_format_absent =
                !generate_pg_types_shape_output.contains(FORMAT_QUERY_PART_FRAGMENT);
            println!(
                "measurement=generate_pg_types_generated_query_part_shape write_fmt_found={generate_pg_types_write_fmt_found} with_capacity_found={generate_pg_types_with_capacity_found} old_format_absent={generate_pg_types_old_format_absent}"
            );
            let generate_pg_types_pipeline_stage_measurement = (0..DIRECT_GENERATION_REPEAT_COUNT).fold(
                (
                    u128::MAX,
                    0u128,
                    0u128,
                    u128::MAX,
                    0u128,
                    0u128,
                    u128::MAX,
                    0u128,
                    0u128,
                    0usize,
                    false,
                    0usize,
                ),
                |(
                    stringify_min_us,
                    stringify_max_us,
                    stringify_total_us,
                    config_min_us,
                    config_max_us,
                    config_total_us,
                    inspect_min_us,
                    inspect_max_us,
                    inspect_total_us,
                    _,
                    _,
                    _,
                ),
                 _| {
                    let stringify_started = std::time::Instant::now();
                    let config_string = generate_pg_types_input_token_stream.to_string();
                    let stringify_wall_us = stringify_started.elapsed().as_micros();
                    let config_started = std::time::Instant::now();
                    let config_value = match serde_json::from_str::<serde_json::Value>(
                        config_string.as_str(),
                    ) {
                        Ok(value) => value,
                        Err(error) => {
                            eprintln!(
                                "measurement=generate_pg_types_pipeline_stages status=config_parse_failed error={error}"
                            );
                            std::process::exit(1);
                        }
                    };
                    let config_wall_us = config_started.elapsed().as_micros();
                    let inspect_started = std::time::Instant::now();
                    let config_key_count = config_value.as_object().map_or(0usize, |value| {
                        value.len()
                    });
                    let variant_is_all = config_value
                        .get("variant")
                        .and_then(serde_json::Value::as_str)
                        .is_some_and(|value| value == "All");
                    let concrete_or_subset_len = config_value
                        .get("variant")
                        .and_then(serde_json::Value::as_array)
                        .map_or(0usize, Vec::len);
                    let inspect_wall_us = inspect_started.elapsed().as_micros();
                    (
                        stringify_min_us.min(stringify_wall_us),
                        stringify_max_us.max(stringify_wall_us),
                        stringify_total_us.saturating_add(stringify_wall_us),
                        config_min_us.min(config_wall_us),
                        config_max_us.max(config_wall_us),
                        config_total_us.saturating_add(config_wall_us),
                        inspect_min_us.min(inspect_wall_us),
                        inspect_max_us.max(inspect_wall_us),
                        inspect_total_us.saturating_add(inspect_wall_us),
                        config_key_count,
                        variant_is_all,
                        concrete_or_subset_len,
                    )
                },
            );
            println!(
                "measurement=generate_pg_types_pipeline_stages repeat_count={DIRECT_GENERATION_REPEAT_COUNT} stringify_min_us={} stringify_total_us={} stringify_max_us={} config_min_us={} config_total_us={} config_max_us={} inspect_min_us={} inspect_total_us={} inspect_max_us={} config_keys={} variant_is_all={} concrete_or_subset_len={}",
                generate_pg_types_pipeline_stage_measurement.0,
                generate_pg_types_pipeline_stage_measurement.2,
                generate_pg_types_pipeline_stage_measurement.1,
                generate_pg_types_pipeline_stage_measurement.3,
                generate_pg_types_pipeline_stage_measurement.5,
                generate_pg_types_pipeline_stage_measurement.4,
                generate_pg_types_pipeline_stage_measurement.6,
                generate_pg_types_pipeline_stage_measurement.8,
                generate_pg_types_pipeline_stage_measurement.7,
                generate_pg_types_pipeline_stage_measurement.9,
                generate_pg_types_pipeline_stage_measurement.10,
                generate_pg_types_pipeline_stage_measurement.11
            );
            let generate_where_filters_input_token_stream = quote::quote! {
                {
                    "pg_types_write_into_file": "False",
                    "whole_write_into_file": "False"
                }
            };
            let generate_where_filters_measurement = (0..DIRECT_GENERATION_REPEAT_COUNT).fold(
                (u128::MAX, 0u128, 0u128, 0usize, 0usize),
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
            let generate_where_filters_shape_output =
                generate_where_filters_src::generate_where_filters(
                    generate_where_filters_src::ProcMacro2GenerateWhereFiltersInput::from(
                        &generate_where_filters_input_token_stream,
                    ),
                )
                .to_string();
            let generate_where_filters_write_fmt_found =
                generate_where_filters_shape_output.contains(STD_FMT_WRITE_CALL);
            let generate_where_filters_with_capacity_found =
                generate_where_filters_shape_output.contains(STRING_WITH_CAPACITY_CALL);
            let generate_where_filters_old_format_absent =
                !generate_where_filters_shape_output.contains(FORMAT_QUERY_PART_FRAGMENT);
            println!(
                "measurement=generate_where_filters_generated_query_part_shape write_fmt_found={generate_where_filters_write_fmt_found} with_capacity_found={generate_where_filters_with_capacity_found} old_format_absent={generate_where_filters_old_format_absent}"
            );
            let generate_where_filters_pipeline_stage_measurement = (0..DIRECT_GENERATION_REPEAT_COUNT).fold(
                (u128::MAX, 0u128, 0u128, u128::MAX, 0u128, 0u128, u128::MAX, 0u128, 0u128, 0usize, 0usize),
                |(
                    stringify_min_wall_us,
                    stringify_max_wall_us,
                    stringify_total_wall_us,
                    config_min_wall_us,
                    config_max_wall_us,
                    config_total_wall_us,
                    inspect_min_wall_us,
                    inspect_max_wall_us,
                    inspect_total_wall_us,
                    _,
                    _,
                ), _| {
                    let stringify_started = std::time::Instant::now();
                    let input_as_string = generate_where_filters_input_token_stream.to_string();
                    let stringify_wall_us = stringify_started.elapsed().as_micros();
                    let config_started = std::time::Instant::now();
                    let config_result =
                        serde_json::from_str::<serde_json::Value>(input_as_string.as_str());
                    let config_wall_us = config_started.elapsed().as_micros();
                    let config = match config_result {
                        Ok(value) => value,
                        Err(error) => {
                            eprintln!(
                                "measurement=generate_where_filters_pipeline_stages status=config_parse_failed error={error}"
                            );
                            std::process::exit(1);
                        }
                    };
                    let inspect_started = std::time::Instant::now();
                    let config_keys = config.as_object().map_or(0usize, serde_json::Map::len);
                    let input_token_trees = generate_where_filters_input_token_stream.clone().into_iter().count();
                    let inspect_wall_us = inspect_started.elapsed().as_micros();
                    (
                        stringify_min_wall_us.min(stringify_wall_us),
                        stringify_max_wall_us.max(stringify_wall_us),
                        stringify_total_wall_us.saturating_add(stringify_wall_us),
                        config_min_wall_us.min(config_wall_us),
                        config_max_wall_us.max(config_wall_us),
                        config_total_wall_us.saturating_add(config_wall_us),
                        inspect_min_wall_us.min(inspect_wall_us),
                        inspect_max_wall_us.max(inspect_wall_us),
                        inspect_total_wall_us.saturating_add(inspect_wall_us),
                        config_keys,
                        input_token_trees,
                    )
                },
            );
            println!(
                "measurement=generate_where_filters_pipeline_stages repeat_count={DIRECT_GENERATION_REPEAT_COUNT} stringify_min_us={} stringify_total_us={} stringify_max_us={} config_min_us={} config_total_us={} config_max_us={} inspect_min_us={} inspect_total_us={} inspect_max_us={} config_keys={} input_token_trees={}",
                generate_where_filters_pipeline_stage_measurement.0,
                generate_where_filters_pipeline_stage_measurement.2,
                generate_where_filters_pipeline_stage_measurement.1,
                generate_where_filters_pipeline_stage_measurement.3,
                generate_where_filters_pipeline_stage_measurement.5,
                generate_where_filters_pipeline_stage_measurement.4,
                generate_where_filters_pipeline_stage_measurement.6,
                generate_where_filters_pipeline_stage_measurement.8,
                generate_where_filters_pipeline_stage_measurement.7,
                generate_where_filters_pipeline_stage_measurement.9,
                generate_where_filters_pipeline_stage_measurement.10
            );
            let pg_crud_common_query_part: Result<
                (u128, u128, u128, usize),
                pg_crud_common::QueryPartError,
            > = (0..SQL_BUILDER_MEASURE_SERIES_COUNT).try_fold(
                (u128::MAX, 0u128, 0u128, 0usize),
                |(min_wall_us, max_wall_us, total_wall_us, _), _| {
                    let started = std::time::Instant::now();
                    let output_bytes =
                        (0..MEASURE_REPEAT_COUNT).try_fold(0usize, |accumulator, _| {
                            let mut increment = 0u64;
                            match pg_crud_common::PgTypeWhereFilter::query_part(
                                &pg_crud_common::PaginationBase::default(),
                                &mut increment,
                                pg_crud_common::SqlColumnRef::from(&"column"),
                                pg_crud_common::AddOperator::from(false),
                            ) {
                                Ok(fragment) => {
                                    Ok(accumulator.saturating_add(fragment.as_ref().len()))
                                }
                                Err(error) => Err(error),
                            }
                        })?;
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
            let where_filters_values = (0i32..64i32).collect::<Vec<i32>>();
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
                (u128::MAX, 0u128, 0u128, 0usize),
                |(min_wall_us, max_wall_us, total_wall_us, _), _| {
                    let started = std::time::Instant::now();
                    let output_bytes =
                        (0..MEASURE_REPEAT_COUNT).try_fold(0usize, |accumulator, _| {
                            let mut increment = 0u64;
                            match where_filters_bounded_vec.pg_type_query_part(
                                &mut increment,
                                pg_crud_common::SqlColumnRef::from(&"column"),
                                pg_crud_common::AddOperator::from(false),
                            ) {
                                Ok(fragment) => {
                                    Ok(accumulator.saturating_add(fragment.as_ref().len()))
                                }
                                Err(error) => Err(error),
                            }
                        })?;
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
        Some("all") => execution::run_commands(&STATIC_COMMANDS)
            .and_then(|()| run_workspace_tests())
            .and_then(|()| {
                MACRO_GENERATION_MEASUREMENTS
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
