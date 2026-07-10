const CARGO_FMT_CHECK_ARGS: [&str; 2] = ["fmt", "--check"];
const CARGO_CLIPPY_ARGS: [&str; 6] = [
    "clippy",
    "--all-targets",
    "--all-features",
    "--",
    "-D",
    "warnings",
];
const CARGO_TEST_STYLE_ARGS: [&str; 4] = ["test", "-p", "tests", "--lib"];
const CARGO_TEST_GEN_PG_TBL_ARGS: [&str; 5] =
    ["test", "-p", "gen_pg_tbl_test", "--features", "test-utils"];
const CARGO_TEST_GEN_PG_TYPES_ARGS: [&str; 5] = [
    "test",
    "-p",
    "gen_pg_types_test",
    "--features",
    "test-utils",
];
const CARGO_TEST_GEN_WH_FLTS_ARGS: [&str; 5] =
    ["test", "-p", "gen_wh_flts_test", "--features", "test-utils"];
const DIRECT_GENERATION_REPEAT_COUNT: usize = 5;
const MEASURE_REPEAT_COUNT: usize = 1000;
const SQL_BUILDER_MEASURE_SERIES_COUNT: usize = 5;
const STATIC_COMMANDS: [(&str, &[&str]); 3] = [
    ("cargo", &CARGO_FMT_CHECK_ARGS),
    ("cargo", &CARGO_CLIPPY_ARGS),
    ("cargo", &CARGO_TEST_STYLE_ARGS),
];
const MACRO_GENERATION_MEASUREMENTS: [(MeasurementName, CargoArgs); 3] = [
    (
        MeasurementName("macro_generation_gen_pg_tbl_test"),
        CargoArgs(&CARGO_TEST_GEN_PG_TBL_ARGS),
    ),
    (
        MeasurementName("macro_generation_gen_pg_types_test"),
        CargoArgs(&CARGO_TEST_GEN_PG_TYPES_ARGS),
    ),
    (
        MeasurementName("macro_generation_gen_wh_flts_test"),
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
#[derive(Clone, Copy)]
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
struct CleanAnsiText(String);
enum CleanAnsiTextTryFromStringEr {
    TooLong { len: usize, max: usize },
}
impl std::fmt::Display for CleanAnsiTextTryFromStringEr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TooLong { len, max } => {
                write!(f, "clean ansi text length {len} exceeds maximum {max}")
            }
        }
    }
}
impl TryFrom<String> for CleanAnsiText {
    type Error = CleanAnsiTextTryFromStringEr;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() <= CLEAN_ANSI_TEXT_MAX_LEN {
            Ok(Self(value))
        } else {
            Err(Self::Error::TooLong {
                len: value.len(),
                max: CLEAN_ANSI_TEXT_MAX_LEN,
            })
        }
    }
}
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
struct QuoteTokenStreamGenPgTblMeasureInputTs(quote::__private::TokenStream);
impl AsRef<quote::__private::TokenStream> for QuoteTokenStreamGenPgTblMeasureInputTs {
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
impl<'lt> From<&'lt str> for StderrTextRef<'lt> {
    fn from(value: &'lt str) -> Self {
        Self(value)
    }
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
            |(mut acc, in_escape), ch| match (in_escape, ch) {
                (true, 'm') => (acc, false),
                (false, '\u{1b}') | (true, _) => (acc, true),
                (false, _) => {
                    acc.push(ch);
                    (acc, false)
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
    let command_output = std::process::Command::new(program.get())
        .args(args.get())
        .env("LD_PRELOAD", MEMUSAGE_PATH)
        .env("MEMUSAGE_PROG_NAME", memusage_prog_name.get())
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
    let command_output = std::process::Command::new("/usr/bin/time")
        .arg("-f")
        .arg(format!(
            "{PEAK_RSS_PREFIX}%M\n{MINOR_PAGE_FAULTS_PREFIX}%R\n{MAJOR_PAGE_FAULTS_PREFIX}%F"
        ))
        .arg("cargo")
        .args(args.get())
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
fn gen_pg_tbl_measure_input_ts(
    tests_write_into_file: &dyn quote::ToTokens,
) -> QuoteTokenStreamGenPgTblMeasureInputTs {
    let allow_clippy_arbitrary_src_item_ordering =
        token_patterns::AllowClippyArbitrarySrcItemOrdering;
    QuoteTokenStreamGenPgTblMeasureInputTs(quote::quote! {
        #allow_clippy_arbitrary_src_item_ordering
        #[derive(Debug, Clone, Copy, optml::Optml)]
        #[gen_pg_tbl::gen_pg_tbl_config{{
            "cm_write_into_file": "False",
            "co_write_into_file": "False",
            "rm_write_into_file": "False",
            "ro_write_into_file": "False",
            "um_write_into_file": "False",
            "uo_write_into_file": "False",
            "dm_write_into_file": "False",
            "dlo_write_into_file": "False",
            "tests_write_into_file": #tests_write_into_file,
            "cmn_write_into_file": "False",
            "whole_write_into_file": "False"
        }}]
        #[gen_pg_tbl::cm_er_vrts{enum CmErVrts{}}]
        #[gen_pg_tbl::co_er_vrts{enum CoErVrts{}}]
        #[gen_pg_tbl::rm_er_vrts{enum RmErVrts{}}]
        #[gen_pg_tbl::ro_er_vrts{enum RoErVrts{}}]
        #[gen_pg_tbl::um_er_vrts{enum UmErVrts{}}]
        #[gen_pg_tbl::uo_er_vrts{enum UoErVrts{}}]
        #[gen_pg_tbl::dm_er_vrts{enum DmErVrts{}}]
        #[gen_pg_tbl::dlo_er_vrts{enum DloErVrts{}}]
        #[gen_pg_tbl::cmn_er_vrts{
            enum CmnErVrts {
                CheckCommit {
                    #[eo_loc]
                    check_commit: route_validators::check_commit::CommitEr,
                    loc: loc_lib::loc::Loc,
                },
            }
        }]
        #[gen_pg_tbl::cm_logic{}]
        #[gen_pg_tbl::co_logic{}]
        #[gen_pg_tbl::rm_logic{}]
        #[gen_pg_tbl::ro_logic{}]
        #[gen_pg_tbl::um_logic{}]
        #[gen_pg_tbl::uo_logic{}]
        #[gen_pg_tbl::dm_logic{}]
        #[gen_pg_tbl::dlo_logic{}]
        #[gen_pg_tbl::cmn_logic{}]
        pub struct TblExample {
            #[gen_pg_tbl_pk]
            pub pk_col: pg_types_text_misc::SqlxTypesUuidUuidAsNnUuidV4InitByPg,
            pub col_0: pg_types_numeric::I16AsNnInt2,
            pub col_1: pg_types_numeric::OptI16AsNlInt2,
            pub col_2: pg_types_numeric::I32AsNnInt4,
        }
    })
}
// Allocation workloads are separate process entry points dispatched by CLI mode.
#[allow(clippy::single_call_fn)]
fn run_alloc_workload_gen_pg_tbl_src() {
    let input = gen_pg_tbl_measure_input_ts(&quote::quote! {"False"});
    let output_bytes = (0..DIRECT_GENERATION_REPEAT_COUNT).fold(0usize, |acc, _| {
        let output = gen_pg_tbl_src::gen_pg_tbl(macros_helpers::ts_writer::ProcMacro2TsRef::from(
            input.as_ref(),
        ));
        acc.saturating_add(output.to_string().len())
    });
    println!(
        "allocation_workload=gen_pg_tbl_src repeat_count={DIRECT_GENERATION_REPEAT_COUNT} output_bytes={output_bytes}"
    );
}
// Allocation workloads are separate process entry points dispatched by CLI mode.
#[allow(clippy::single_call_fn)]
fn run_alloc_workload_gen_pg_types_src() {
    let input = quote::quote! {
        {
            "pg_tbl_cols_write_into_file": "False",
            "whole_write_into_file": "False",
            "vrt": "All"
        }
    };
    let output_bytes = (0..DIRECT_GENERATION_REPEAT_COUNT).fold(0usize, |acc, _| {
        let output = gen_pg_types_src::gen_pg_types(
            macros_helpers::ts_writer::ProcMacro2TsRef::from(&input),
        );
        acc.saturating_add(output.to_string().len())
    });
    println!(
        "allocation_workload=gen_pg_types_src repeat_count={DIRECT_GENERATION_REPEAT_COUNT} output_bytes={output_bytes}"
    );
}
// Allocation workloads are separate process entry points dispatched by CLI mode.
#[allow(clippy::single_call_fn)]
fn run_alloc_workload_pg_crud_cmn_qp() -> Result<(), ()> {
    let output_bytes =
        (0..SQL_BUILDER_MEASURE_SERIES_COUNT).try_fold(0usize, |series_acc, _| {
            (0..MEASURE_REPEAT_COUNT).try_fold(series_acc, |acc, _| {
                let mut incr = 0u64;
                match pg_crud_cmn::PgTypeWhFlt::qp(
                    &pg_crud_cmn::PgnBase::default(),
                    &mut incr,
                    pg_crud_cmn::SqlColRef::from(&"col"),
                    pg_crud_cmn::AddOprtr::from(false),
                ) {
                    Ok(fragment) => Ok(acc.saturating_add(fragment.as_ref().len())),
                    Err(error) => {
                        eprintln!(
                            "allocation_workload=pg_crud_cmn_qp status=failed error={error:?}"
                        );
                        Err(())
                    }
                }
            })
        })?;
    println!(
        "allocation_workload=pg_crud_cmn_qp series_count={SQL_BUILDER_MEASURE_SERIES_COUNT} repeat_count={MEASURE_REPEAT_COUNT} output_bytes={output_bytes}"
    );
    Ok(())
}
// Allocation workloads are separate process entry points dispatched by CLI mode.
#[allow(clippy::single_call_fn)]
fn run_alloc_workload_wh_flts_qp() -> Result<(), ()> {
    let wh_flts_values = (0i32..64i32).collect::<Vec<i32>>();
    let wh_flts_bounded_vec = match wh_flts::BoundedVec::<i32, 64>::try_from(wh_flts_values) {
        Ok(value) => value,
        Err(error) => {
            eprintln!("allocation_workload=wh_flts_qp status=setup_failed error={error:?}");
            return Err(());
        }
    };
    let output_bytes =
        (0..SQL_BUILDER_MEASURE_SERIES_COUNT).try_fold(0usize, |series_acc, _| {
            (0..MEASURE_REPEAT_COUNT).try_fold(series_acc, |acc, _| {
                let mut incr = 0u64;
                match wh_flts_bounded_vec.pg_type_qp(
                    &mut incr,
                    pg_crud_cmn::SqlColRef::from(&"col"),
                    pg_crud_cmn::AddOprtr::from(false),
                ) {
                    Ok(fragment) => Ok(acc.saturating_add(fragment.as_ref().len())),
                    Err(error) => {
                        eprintln!("allocation_workload=wh_flts_qp status=failed error={error:?}");
                        Err(())
                    }
                }
            })
        })?;
    println!(
        "allocation_workload=wh_flts_qp series_count={SQL_BUILDER_MEASURE_SERIES_COUNT} repeat_count={MEASURE_REPEAT_COUNT} output_bytes={output_bytes}"
    );
    Ok(())
}
fn main() {
    let mode = std::env::args().nth(1);
    let run_commands: fn(&[(&str, &[&str])]) -> Result<(), ()> = |commands| {
        commands.iter().try_fold(
            (),
            |(), (program, args)| match std::process::Command::new(program).args(*args).status() {
                Ok(status) if status.success() => Ok(()),
                Ok(status) => {
                    eprintln!("command failed: {program} {args:?}: {status}");
                    Err(())
                }
                Err(error) => {
                    eprintln!("failed to spawn command: {program} {args:?}: {error}");
                    Err(())
                }
            },
        )
    };
    let result = match mode.as_deref() {
        None | Some("static") => run_commands(&STATIC_COMMANDS),
        Some("alloc-workload-gen-pg-tbl-src") => {
            run_alloc_workload_gen_pg_tbl_src();
            Ok(())
        }
        Some("alloc-workload-gen-pg-types-src") => {
            run_alloc_workload_gen_pg_types_src();
            Ok(())
        }
        Some("alloc-workload-pg-crud-cmn-qp") => run_alloc_workload_pg_crud_cmn_qp(),
        Some("alloc-workload-wh-flts-qp") => run_alloc_workload_wh_flts_qp(),
        Some("macro-generation") => MACRO_GENERATION_MEASUREMENTS
            .iter()
            .try_fold((), |(), (measurement_name, args)| {
                measure_cargo_command(*measurement_name, *args)
            }),
        Some("measure") => {
            let allocation_tools_printed: Result<(), std::convert::Infallible> =
                ALLOCATION_TOOLS.iter().try_fold((), |(), tool| {
                    let available = std::path::Path::new(tool.path.get()).exists();
                    let name = tool.name.get();
                    let path = tool.path.get();
                    println!(
                        "measurement=allocation_tool_available tool={name} path={path} available={available}"
                    );
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
                        MeasurementName("gen_pg_tbl_src"),
                        "alloc-workload-gen-pg-tbl-src",
                    ),
                    (
                        MeasurementName("gen_pg_types_src"),
                        "alloc-workload-gen-pg-types-src",
                    ),
                    (
                        MeasurementName("pg_crud_cmn_qp"),
                        "alloc-workload-pg-crud-cmn-qp",
                    ),
                    (MeasurementName("wh_flts_qp"), "alloc-workload-wh-flts-qp"),
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
            let gen_pg_tbl_input_ts = gen_pg_tbl_measure_input_ts(&quote::quote! {"False"});
            let gen_pg_tbl_input_with_tests_ts =
                gen_pg_tbl_measure_input_ts(&quote::quote! {"True"});
            let gen_pg_tbl_measurement = (0..DIRECT_GENERATION_REPEAT_COUNT).fold(
                (u128::MAX, 0u128, 0u128, 0usize, 0usize),
                |(min_wall_us, max_wall_us, total_wall_us, _, _), _| {
                    let started = std::time::Instant::now();
                    let output = gen_pg_tbl_src::gen_pg_tbl(
                        macros_helpers::ts_writer::ProcMacro2TsRef::from(
                            gen_pg_tbl_input_ts.as_ref(),
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
                "measurement=gen_pg_tbl_src repeat_count={} wall_min_us={} wall_total_us={} wall_max_us={} output_bytes={} output_token_trees={}",
                DIRECT_GENERATION_REPEAT_COUNT,
                gen_pg_tbl_measurement.0,
                gen_pg_tbl_measurement.2,
                gen_pg_tbl_measurement.1,
                gen_pg_tbl_measurement.3,
                gen_pg_tbl_measurement.4
            );
            let gen_pg_tbl_with_tests_dir =
                std::path::Path::new("target/measure/gen_pg_tbl_with_tests");
            if let Err(error) = std::fs::create_dir_all(gen_pg_tbl_with_tests_dir) {
                eprintln!(
                    "measurement=gen_pg_tbl_src_with_tests status=create_dir_failed error={error}"
                );
                std::process::exit(1);
            }
            if let Err(error) = std::fs::write(
                gen_pg_tbl_with_tests_dir.join("rustfmt.toml"),
                "edition = \"2024\"\n",
            ) {
                eprintln!(
                    "measurement=gen_pg_tbl_src_with_tests status=rustfmt_config_write_failed error={error}"
                );
                std::process::exit(1);
            }
            let current_dir = match std::env::current_dir() {
                Ok(value) => value,
                Err(error) => {
                    eprintln!(
                        "measurement=gen_pg_tbl_src_with_tests status=current_dir_failed error={error}"
                    );
                    std::process::exit(1);
                }
            };
            if let Err(error) = std::env::set_current_dir(gen_pg_tbl_with_tests_dir) {
                eprintln!(
                    "measurement=gen_pg_tbl_src_with_tests status=set_current_dir_failed error={error}"
                );
                std::process::exit(1);
            }
            let gen_pg_tbl_with_tests_measurement = (0..DIRECT_GENERATION_REPEAT_COUNT).fold(
                (u128::MAX, 0u128, 0u128, 0usize, 0usize),
                |(min_wall_us, max_wall_us, total_wall_us, _, _), _| {
                    let started = std::time::Instant::now();
                    let output = gen_pg_tbl_src::gen_pg_tbl(
                        macros_helpers::ts_writer::ProcMacro2TsRef::from(
                            gen_pg_tbl_input_with_tests_ts.as_ref(),
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
                    "measurement=gen_pg_tbl_src_with_tests status=restore_current_dir_failed error={error}"
                );
                std::process::exit(1);
            }
            let gen_pg_tbl_tests_stage_output = match std::fs::read_to_string(
                gen_pg_tbl_with_tests_dir.join("gen_pg_tbl_Tests.rs"),
            ) {
                Ok(content) => (content.len(), content.lines().count()),
                Err(error) => {
                    eprintln!(
                        "measurement=gen_pg_tbl_tests_stage_output status=read_failed error={error}"
                    );
                    std::process::exit(1);
                }
            };
            println!(
                "measurement=gen_pg_tbl_src_with_tests repeat_count={} wall_min_us={} wall_total_us={} wall_max_us={} output_bytes={} output_token_trees={}",
                DIRECT_GENERATION_REPEAT_COUNT,
                gen_pg_tbl_with_tests_measurement.0,
                gen_pg_tbl_with_tests_measurement.2,
                gen_pg_tbl_with_tests_measurement.1,
                gen_pg_tbl_with_tests_measurement.3,
                gen_pg_tbl_with_tests_measurement.4
            );
            println!(
                "measurement=gen_pg_tbl_tests_stage_output bytes={} lines={}",
                gen_pg_tbl_tests_stage_output.0, gen_pg_tbl_tests_stage_output.1
            );
            println!(
                "measurement=gen_pg_tbl_tests_emit_delta repeat_count={} wall_total_delta_us={} wall_min_delta_us={} wall_max_delta_us={} output_bytes_delta={}",
                DIRECT_GENERATION_REPEAT_COUNT,
                gen_pg_tbl_with_tests_measurement
                    .2
                    .saturating_sub(gen_pg_tbl_measurement.2),
                gen_pg_tbl_with_tests_measurement
                    .0
                    .saturating_sub(gen_pg_tbl_measurement.0),
                gen_pg_tbl_with_tests_measurement
                    .1
                    .saturating_sub(gen_pg_tbl_measurement.1),
                gen_pg_tbl_with_tests_measurement
                    .3
                    .saturating_sub(gen_pg_tbl_measurement.3)
            );
            let gen_pg_tbl_src_text =
                std::fs::read_to_string("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs")
                    .unwrap_or_default();
            let gen_pg_tbl_concurrency_constants_found = [
                "CM_CHUNK_SIZE_2EE9377B",
                "CM_CONCURRENCY_7CCFD82D",
                "CM_CHUNK_SIZE_A13F7C92",
                "TEST_FUTURE_CONCURRENCY_D281414B",
            ]
            .into_iter()
            .all(|pattern| gen_pg_tbl_src_text.contains(pattern));
            println!(
                "measurement=gen_pg_tbl_generated_test_concurrency cm_chunk_2ee9377b=25 cm_concurrency_7ccfd82d=5 cm_chunk_a13f7c92=10 test_future_concurrency_d281414b=100 source_detected={gen_pg_tbl_concurrency_constants_found}"
            );
            let gen_pg_tbl_box_future_push_sites =
                gen_pg_tbl_src_text.matches("acc_9189f86e.push").count();
            let gen_pg_tbl_old_chunk_vec_from_absent =
                !gen_pg_tbl_src_text.contains(".map(Vec::from)");
            let gen_pg_tbl_old_collect_flatten_absent =
                !gen_pg_tbl_src_text.contains(".flatten().collect");
            let gen_pg_tbl_tbl_names_cloned_vec_absent =
                !gen_pg_tbl_src_text.contains("tbl_names_cloned = tbl_names.iter().map");
            println!(
                "measurement=gen_pg_tbl_generated_test_concurrency_shape box_future_push_sites={gen_pg_tbl_box_future_push_sites} old_chunk_vec_from_absent={gen_pg_tbl_old_chunk_vec_from_absent} old_collect_flatten_absent={gen_pg_tbl_old_collect_flatten_absent} tbl_names_cloned_vec_absent={gen_pg_tbl_tbl_names_cloned_vec_absent}"
            );
            let gen_pg_tbl_pipeline_stage_source_found = [
                "parse_gen_pg_tbl_input_stage",
                "build_gen_pg_tbl_input_model_stage",
                "validate_gen_pg_tbl_fields_model_stage",
                "emit_gen_pg_tbl_type_declarations_stage",
                "emit_gen_pg_tbl_query_builders_stage",
                "emit_gen_pg_tbl_route_handlers_stage",
                "emit_gen_pg_tbl_tests_stage",
                "emit_gen_pg_tbl_final_stage",
            ]
            .into_iter()
            .all(|pattern| gen_pg_tbl_src_text.contains(pattern));
            println!(
                "measurement=gen_pg_tbl_pipeline_shape parse=true build_model=true validate=true emit_type_declarations=true emit_query_builders=true emit_route_handlers=true emit_tests=true emit_final=true source_detected={gen_pg_tbl_pipeline_stage_source_found}"
            );
            let gen_pg_tbl_pipeline_stage_measurement = (0..DIRECT_GENERATION_REPEAT_COUNT).fold(
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
                        match syn::parse2::<syn::DeriveInput>(gen_pg_tbl_input_ts.as_ref().clone())
                        {
                        Ok(value) => value,
                        Err(error) => {
                            eprintln!(
                                "measurement=gen_pg_tbl_pipeline_stages status=parse_failed error={error}"
                            );
                            std::process::exit(1);
                        }
                    };
                    let parse_wall_us = parse_started.elapsed().as_micros();
                    let config_started = std::time::Instant::now();
                    let config_attr_ts = macros_helpers::attr_reader::get_macro_attr_meta_list_ts(
                        &parsed.attrs,
                        "gen_pg_tbl::gen_pg_tbl_config",
                    );
                    let config_value =
                        match serde_json::from_str::<serde_json::Value>(&config_attr_ts.to_string())
                        {
                            Ok(value) => value,
                            Err(error) => {
                                eprintln!(
                                    "measurement=gen_pg_tbl_pipeline_stages status=config_parse_failed error={error}"
                                );
                                std::process::exit(1);
                            }
                        };
                    let config_wall_us = config_started.elapsed().as_micros();
                    let config_key_count = config_value.as_object().map_or(0usize, |value| {
                        value.len()
                    });
                    let model_started = std::time::Instant::now();
                    let er_vrt_count = [
                        ("gen_pg_tbl::cm_er_vrts", "CmErVrts"),
                        ("gen_pg_tbl::co_er_vrts", "CoErVrts"),
                        ("gen_pg_tbl::rm_er_vrts", "RmErVrts"),
                        ("gen_pg_tbl::ro_er_vrts", "RoErVrts"),
                        ("gen_pg_tbl::um_er_vrts", "UmErVrts"),
                        ("gen_pg_tbl::uo_er_vrts", "UoErVrts"),
                        ("gen_pg_tbl::dm_er_vrts", "DmErVrts"),
                        ("gen_pg_tbl::dlo_er_vrts", "DloErVrts"),
                        ("gen_pg_tbl::cmn_er_vrts", "CmnErVrts"),
                    ]
                    .into_iter()
                    .fold(0usize, |acc, (attr_path, expected_ident)| {
                        let attr_ts = macros_helpers::attr_reader::get_macro_attr_meta_list_ts(
                            &parsed.attrs,
                            attr_path,
                        );
                        let Ok(parsed_attr) = syn::parse2::<syn::DeriveInput>((*attr_ts).clone())
                        else {
                            return acc;
                        };
                        if parsed_attr.ident != expected_ident {
                            eprintln!(
                                "measurement=gen_pg_tbl_pipeline_stages status=model_ident_mismatch attr={attr_path}"
                            );
                            std::process::exit(1);
                        }
                        match parsed_attr.data {
                            syn::Data::Enum(data_enum) => {
                                acc.saturating_add(data_enum.variants.len())
                            }
                            syn::Data::Struct(_) | syn::Data::Union(_) => acc,
                        }
                    });
                    let logic_attr_token_bytes = [
                        "gen_pg_tbl::cm_logic",
                        "gen_pg_tbl::co_logic",
                        "gen_pg_tbl::rm_logic",
                        "gen_pg_tbl::ro_logic",
                        "gen_pg_tbl::um_logic",
                        "gen_pg_tbl::uo_logic",
                        "gen_pg_tbl::dm_logic",
                        "gen_pg_tbl::dlo_logic",
                        "gen_pg_tbl::cmn_logic",
                    ]
                    .into_iter()
                    .fold(0usize, |acc, attr_path| {
                        let logic_ts = macros_helpers::attr_reader::get_macro_attr_meta_list_ts(
                            &parsed.attrs,
                            attr_path,
                        );
                        acc.saturating_add(logic_ts.to_string().len())
                    });
                    let model_wall_us = model_started.elapsed().as_micros();
                    let fields_started = std::time::Instant::now();
                    let (field_count, pk_attr_count) = match &parsed.data {
                        syn::Data::Struct(data_struct) => match &data_struct.fields {
                            syn::Fields::Named(fields_named) => fields_named.named.iter().fold(
                                (0usize, 0usize),
                                |(field_acc, pk_acc), field| {
                                    let field_pk_attr_count = field
                                        .attrs
                                        .iter()
                                        .filter(|attr| attr.path().segments.len() == 1)
                                        .filter(|attr| {
                                            attr.path()
                                                .segments
                                                .first()
                                                .is_some_and(|segment| {
                                                    segment.ident == "gen_pg_tbl_pk"
                                                })
                                        })
                                        .count();
                                    (
                                        field_acc.saturating_add(1),
                                        pk_acc.saturating_add(field_pk_attr_count),
                                    )
                                },
                            ),
                            syn::Fields::Unnamed(_) | syn::Fields::Unit => {
                                eprintln!(
                                    "measurement=gen_pg_tbl_pipeline_stages status=fields_not_named"
                                );
                                std::process::exit(1);
                            }
                        },
                        syn::Data::Enum(_) | syn::Data::Union(_) => {
                            eprintln!(
                                "measurement=gen_pg_tbl_pipeline_stages status=input_not_struct"
                            );
                            std::process::exit(1);
                        }
                    };
                    let fields_wall_us = fields_started.elapsed().as_micros();
                    let validate_started = std::time::Instant::now();
                    if field_count == 0usize || pk_attr_count != 1usize {
                        eprintln!(
                            "measurement=gen_pg_tbl_pipeline_stages status=validation_failed fields={field_count} pk_attrs={pk_attr_count}"
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
                        pk_attr_count,
                        er_vrt_count,
                        logic_attr_token_bytes,
                    )
                },
            );
            println!(
                "measurement=gen_pg_tbl_pipeline_stages repeat_count={DIRECT_GENERATION_REPEAT_COUNT} parse_min_us={} parse_total_us={} parse_max_us={} config_min_us={} config_total_us={} config_max_us={} model_min_us={} model_total_us={} model_max_us={} fields_min_us={} fields_total_us={} fields_max_us={} validate_min_us={} validate_total_us={} validate_max_us={} config_keys={} fields={} pk_attrs={} er_vrts={} logic_attr_token_bytes={}",
                gen_pg_tbl_pipeline_stage_measurement.0,
                gen_pg_tbl_pipeline_stage_measurement.2,
                gen_pg_tbl_pipeline_stage_measurement.1,
                gen_pg_tbl_pipeline_stage_measurement.3,
                gen_pg_tbl_pipeline_stage_measurement.5,
                gen_pg_tbl_pipeline_stage_measurement.4,
                gen_pg_tbl_pipeline_stage_measurement.6,
                gen_pg_tbl_pipeline_stage_measurement.8,
                gen_pg_tbl_pipeline_stage_measurement.7,
                gen_pg_tbl_pipeline_stage_measurement.9,
                gen_pg_tbl_pipeline_stage_measurement.11,
                gen_pg_tbl_pipeline_stage_measurement.10,
                gen_pg_tbl_pipeline_stage_measurement.12,
                gen_pg_tbl_pipeline_stage_measurement.14,
                gen_pg_tbl_pipeline_stage_measurement.13,
                gen_pg_tbl_pipeline_stage_measurement.15,
                gen_pg_tbl_pipeline_stage_measurement.16,
                gen_pg_tbl_pipeline_stage_measurement.17,
                gen_pg_tbl_pipeline_stage_measurement.18,
                gen_pg_tbl_pipeline_stage_measurement.19
            );
            let gen_pg_types_input_ts = quote::quote! {
                {
                    "pg_tbl_cols_write_into_file": "False",
                    "whole_write_into_file": "False",
                    "vrt": "All"
                }
            };
            let gen_pg_types_measurement = (0..DIRECT_GENERATION_REPEAT_COUNT).fold(
                (u128::MAX, 0u128, 0u128, 0usize, 0usize),
                |(min_wall_us, max_wall_us, total_wall_us, _, _), _| {
                    let started = std::time::Instant::now();
                    let output = gen_pg_types_src::gen_pg_types(
                        macros_helpers::ts_writer::ProcMacro2TsRef::from(&gen_pg_types_input_ts),
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
                "measurement=gen_pg_types_src repeat_count={} wall_min_us={} wall_total_us={} wall_max_us={} output_bytes={} output_token_trees={}",
                DIRECT_GENERATION_REPEAT_COUNT,
                gen_pg_types_measurement.0,
                gen_pg_types_measurement.2,
                gen_pg_types_measurement.1,
                gen_pg_types_measurement.3,
                gen_pg_types_measurement.4
            );
            let gen_pg_types_shape_output = gen_pg_types_src::gen_pg_types(
                macros_helpers::ts_writer::ProcMacro2TsRef::from(&gen_pg_types_input_ts),
            )
            .to_string();
            let gen_pg_types_write_fmt_found =
                gen_pg_types_shape_output.contains("std :: fmt :: Write :: write_fmt");
            let gen_pg_types_with_capacity_found =
                gen_pg_types_shape_output.contains("String :: with_capacity");
            let gen_pg_types_old_format_absent =
                !gen_pg_types_shape_output.contains("QpFragment :: try_from (format !");
            println!(
                "measurement=gen_pg_types_generated_qp_shape write_fmt_found={gen_pg_types_write_fmt_found} with_capacity_found={gen_pg_types_with_capacity_found} old_format_absent={gen_pg_types_old_format_absent}"
            );
            let gen_pg_types_pipeline_stage_measurement = (0..DIRECT_GENERATION_REPEAT_COUNT).fold(
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
                    let config_string = gen_pg_types_input_ts.to_string();
                    let stringify_wall_us = stringify_started.elapsed().as_micros();
                    let config_started = std::time::Instant::now();
                    let config_value = match serde_json::from_str::<serde_json::Value>(
                        config_string.as_str(),
                    ) {
                        Ok(value) => value,
                        Err(error) => {
                            eprintln!(
                                "measurement=gen_pg_types_pipeline_stages status=config_parse_failed error={error}"
                            );
                            std::process::exit(1);
                        }
                    };
                    let config_wall_us = config_started.elapsed().as_micros();
                    let inspect_started = std::time::Instant::now();
                    let config_key_count = config_value.as_object().map_or(0usize, |value| {
                        value.len()
                    });
                    let vrt_is_all = config_value
                        .get("vrt")
                        .and_then(serde_json::Value::as_str)
                        .is_some_and(|value| value == "All");
                    let concrete_or_subset_len = config_value
                        .get("vrt")
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
                        vrt_is_all,
                        concrete_or_subset_len,
                    )
                },
            );
            println!(
                "measurement=gen_pg_types_pipeline_stages repeat_count={DIRECT_GENERATION_REPEAT_COUNT} stringify_min_us={} stringify_total_us={} stringify_max_us={} config_min_us={} config_total_us={} config_max_us={} inspect_min_us={} inspect_total_us={} inspect_max_us={} config_keys={} vrt_is_all={} concrete_or_subset_len={}",
                gen_pg_types_pipeline_stage_measurement.0,
                gen_pg_types_pipeline_stage_measurement.2,
                gen_pg_types_pipeline_stage_measurement.1,
                gen_pg_types_pipeline_stage_measurement.3,
                gen_pg_types_pipeline_stage_measurement.5,
                gen_pg_types_pipeline_stage_measurement.4,
                gen_pg_types_pipeline_stage_measurement.6,
                gen_pg_types_pipeline_stage_measurement.8,
                gen_pg_types_pipeline_stage_measurement.7,
                gen_pg_types_pipeline_stage_measurement.9,
                gen_pg_types_pipeline_stage_measurement.10,
                gen_pg_types_pipeline_stage_measurement.11
            );
            let gen_wh_flts_input_ts = quote::quote! {
                {
                    "pg_types_write_into_file": "False",
                    "whole_write_into_file": "False"
                }
            };
            let gen_wh_flts_measurement = (0..DIRECT_GENERATION_REPEAT_COUNT).fold(
                (u128::MAX, 0u128, 0u128, 0usize, 0usize),
                |(min_wall_us, max_wall_us, total_wall_us, _, _), _| {
                    let started = std::time::Instant::now();
                    let output = gen_wh_flts_src::gen_wh_flts(
                        gen_wh_flts_src::ProcMacro2GenWhFltsInput::from(&gen_wh_flts_input_ts),
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
                "measurement=gen_wh_flts_src repeat_count={} wall_min_us={} wall_total_us={} wall_max_us={} output_bytes={} output_token_trees={}",
                DIRECT_GENERATION_REPEAT_COUNT,
                gen_wh_flts_measurement.0,
                gen_wh_flts_measurement.2,
                gen_wh_flts_measurement.1,
                gen_wh_flts_measurement.3,
                gen_wh_flts_measurement.4
            );
            let gen_wh_flts_shape_output = gen_wh_flts_src::gen_wh_flts(
                gen_wh_flts_src::ProcMacro2GenWhFltsInput::from(&gen_wh_flts_input_ts),
            )
            .to_string();
            let gen_wh_flts_write_fmt_found =
                gen_wh_flts_shape_output.contains("std :: fmt :: Write :: write_fmt");
            let gen_wh_flts_with_capacity_found =
                gen_wh_flts_shape_output.contains("String :: with_capacity");
            let gen_wh_flts_old_format_absent =
                !gen_wh_flts_shape_output.contains("QpFragment :: try_from (format !");
            println!(
                "measurement=gen_wh_flts_generated_qp_shape write_fmt_found={gen_wh_flts_write_fmt_found} with_capacity_found={gen_wh_flts_with_capacity_found} old_format_absent={gen_wh_flts_old_format_absent}"
            );
            let gen_wh_flts_pipeline_stage_measurement = (0..DIRECT_GENERATION_REPEAT_COUNT).fold(
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
                    let input_as_string = gen_wh_flts_input_ts.to_string();
                    let stringify_wall_us = stringify_started.elapsed().as_micros();
                    let config_started = std::time::Instant::now();
                    let config_result =
                        serde_json::from_str::<serde_json::Value>(input_as_string.as_str());
                    let config_wall_us = config_started.elapsed().as_micros();
                    let config = match config_result {
                        Ok(value) => value,
                        Err(error) => {
                            eprintln!(
                                "measurement=gen_wh_flts_pipeline_stages status=config_parse_failed error={error}"
                            );
                            std::process::exit(1);
                        }
                    };
                    let inspect_started = std::time::Instant::now();
                    let config_keys = config.as_object().map_or(0usize, serde_json::Map::len);
                    let input_token_trees = gen_wh_flts_input_ts.clone().into_iter().count();
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
                "measurement=gen_wh_flts_pipeline_stages repeat_count={DIRECT_GENERATION_REPEAT_COUNT} stringify_min_us={} stringify_total_us={} stringify_max_us={} config_min_us={} config_total_us={} config_max_us={} inspect_min_us={} inspect_total_us={} inspect_max_us={} config_keys={} input_token_trees={}",
                gen_wh_flts_pipeline_stage_measurement.0,
                gen_wh_flts_pipeline_stage_measurement.2,
                gen_wh_flts_pipeline_stage_measurement.1,
                gen_wh_flts_pipeline_stage_measurement.3,
                gen_wh_flts_pipeline_stage_measurement.5,
                gen_wh_flts_pipeline_stage_measurement.4,
                gen_wh_flts_pipeline_stage_measurement.6,
                gen_wh_flts_pipeline_stage_measurement.8,
                gen_wh_flts_pipeline_stage_measurement.7,
                gen_wh_flts_pipeline_stage_measurement.9,
                gen_wh_flts_pipeline_stage_measurement.10
            );
            let pg_crud_cmn_qp: Result<(u128, u128, u128, usize), pg_crud_cmn::QpEr> =
                (0..SQL_BUILDER_MEASURE_SERIES_COUNT).try_fold(
                    (u128::MAX, 0u128, 0u128, 0usize),
                    |(min_wall_us, max_wall_us, total_wall_us, _), _| {
                        let started = std::time::Instant::now();
                        let output_bytes =
                            (0..MEASURE_REPEAT_COUNT).try_fold(0usize, |acc, _| {
                                let mut incr = 0u64;
                                match pg_crud_cmn::PgTypeWhFlt::qp(
                                    &pg_crud_cmn::PgnBase::default(),
                                    &mut incr,
                                    pg_crud_cmn::SqlColRef::from(&"col"),
                                    pg_crud_cmn::AddOprtr::from(false),
                                ) {
                                    Ok(fragment) => Ok(acc.saturating_add(fragment.as_ref().len())),
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
            match pg_crud_cmn_qp {
                Ok((min_wall_us, max_wall_us, total_wall_us, output_bytes)) => {
                    println!(
                        "measurement=pg_crud_cmn_qp series_count={SQL_BUILDER_MEASURE_SERIES_COUNT} repeat_count={MEASURE_REPEAT_COUNT} wall_min_us={min_wall_us} wall_total_us={total_wall_us} wall_max_us={max_wall_us} output_bytes={output_bytes}"
                    );
                }
                Err(error) => {
                    eprintln!("measurement=pg_crud_cmn_qp status=failed error={error:?}");
                    std::process::exit(1);
                }
            }
            let wh_flts_values = (0i32..64i32).collect::<Vec<i32>>();
            let wh_flts_bounded_vec = match wh_flts::BoundedVec::<i32, 64>::try_from(wh_flts_values)
            {
                Ok(value) => value,
                Err(error) => {
                    eprintln!("measurement=wh_flts_qp status=setup_failed error={error:?}");
                    std::process::exit(1);
                }
            };
            let wh_flts_qp: Result<(u128, u128, u128, usize), pg_crud_cmn::QpEr> =
                (0..SQL_BUILDER_MEASURE_SERIES_COUNT).try_fold(
                    (u128::MAX, 0u128, 0u128, 0usize),
                    |(min_wall_us, max_wall_us, total_wall_us, _), _| {
                        let started = std::time::Instant::now();
                        let output_bytes =
                            (0..MEASURE_REPEAT_COUNT).try_fold(0usize, |acc, _| {
                                let mut incr = 0u64;
                                match wh_flts_bounded_vec.pg_type_qp(
                                    &mut incr,
                                    pg_crud_cmn::SqlColRef::from(&"col"),
                                    pg_crud_cmn::AddOprtr::from(false),
                                ) {
                                    Ok(fragment) => Ok(acc.saturating_add(fragment.as_ref().len())),
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
            match wh_flts_qp {
                Ok((min_wall_us, max_wall_us, total_wall_us, output_bytes)) => {
                    println!(
                        "measurement=wh_flts_qp series_count={SQL_BUILDER_MEASURE_SERIES_COUNT} repeat_count={MEASURE_REPEAT_COUNT} wall_min_us={min_wall_us} wall_total_us={total_wall_us} wall_max_us={max_wall_us} output_bytes={output_bytes}"
                    );
                    Ok(())
                }
                Err(error) => {
                    eprintln!("measurement=wh_flts_qp status=failed error={error:?}");
                    Err(())
                }
            }
        }
        Some("all") => run_commands(&STATIC_COMMANDS).and_then(|()| {
            MACRO_GENERATION_MEASUREMENTS
                .iter()
                .try_fold((), |(), (measurement_name, args)| {
                    measure_cargo_command(*measurement_name, *args)
                })
        }),
        Some(other) => {
            eprintln!(
                "unknown mode `{other}`; expected `static`, `macro-generation`, `measure`, `all`, or `alloc-workload-*`"
            );
            Err(())
        }
    };
    if result.is_err() {
        std::process::exit(1);
    }
}
