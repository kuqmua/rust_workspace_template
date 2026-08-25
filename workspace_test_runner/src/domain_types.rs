pub(crate) const DIRECT_GENERATION_REPEAT_COUNT: usize = 5;
pub(crate) const MEASURE_REPEAT_COUNT: usize = 1000;
const RUNNER_MODE_MAX_LEN: usize = 1_024usize;
pub(crate) const SQL_BUILDER_MEASURE_SERIES_COUNT: usize = 5;
const CLEAN_ANSI_TEXT_MAX_LEN: usize = 16_777_216;
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, newtype::FromInner)]
pub(crate) struct MeasurementName(&'static str);
impl MeasurementName {
    pub(crate) const fn get(self) -> &'static str {
        self.0
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, newtype::FromInner)]
pub(crate) struct CargoArgs(&'static [&'static str]);
impl<const N: usize> From<&'static [&'static str; N]> for CargoArgs {
    fn from(value: &'static [&'static str; N]) -> Self {
        Self(value.as_slice())
    }
}
impl CargoArgs {
    pub(crate) const fn get(self) -> &'static [&'static str] {
        self.0
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, newtype::FromInner)]
pub(crate) struct StderrTextRef<'lt>(&'lt str);
impl<'lt> StderrTextRef<'lt> {
    pub(crate) const fn get(self) -> &'lt str {
        self.0
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, newtype::FromInner)]
pub(crate) struct AnsiTextRef<'lt>(&'lt str);
impl<'lt> AnsiTextRef<'lt> {
    pub(crate) const fn get(self) -> &'lt str {
        self.0
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, newtype::AsRefStr, newtype::BoundedString)]
#[bounded_string(max = CLEAN_ANSI_TEXT_MAX_LEN)]
pub(crate) struct CleanAnsiText(String);
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, newtype::FromInner)]
pub(crate) struct MemusageKey(&'static str);
impl MemusageKey {
    pub(crate) const fn get(self) -> &'static str {
        self.0
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, newtype::FromInner)]
pub(crate) struct MemusageRowName(&'static str);
impl MemusageRowName {
    pub(crate) const fn get(self) -> &'static str {
        self.0
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, newtype::FromInner)]
pub(crate) struct MemusageColumnIdx(usize);
impl MemusageColumnIdx {
    pub(crate) const fn get(self) -> usize {
        self.0
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, newtype::FromInner)]
pub(crate) struct MemusageValueRef<'lt>(&'lt str);
impl<'lt> MemusageValueRef<'lt> {
    pub(crate) const fn get(self) -> &'lt str {
        self.0
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, newtype::FromInner)]
pub(crate) struct ProgramPathRef<'lt>(&'lt str);
impl<'lt> ProgramPathRef<'lt> {
    pub(crate) const fn get(self) -> &'lt str {
        self.0
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, newtype::FromInner)]
pub(crate) struct ProgramArgsRef<'lt>(&'lt [&'lt str]);
impl<'lt, const N: usize> From<&'lt [&'lt str; N]> for ProgramArgsRef<'lt> {
    fn from(value: &'lt [&'lt str; N]) -> Self {
        Self(value.as_slice())
    }
}
impl<'lt> ProgramArgsRef<'lt> {
    pub(crate) const fn get(self) -> &'lt [&'lt str] {
        self.0
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, newtype::FromInner)]
pub(crate) struct MemusageProgNameRef<'lt>(&'lt str);
impl<'lt> MemusageProgNameRef<'lt> {
    pub(crate) const fn get(self) -> &'lt str {
        self.0
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, newtype::AsRefOwned, newtype::FromInner,
)]
pub(crate) struct QuoteTokenStreamGeneratePgTableMeasureInputTokenStream(
    quote::__private::TokenStream,
);
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, newtype::FromInner)]
pub(crate) struct ToolName(&'static str);
impl ToolName {
    pub(crate) const fn get(self) -> &'static str {
        self.0
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, newtype::FromInner)]
pub(crate) struct ToolPath(&'static str);
impl ToolPath {
    pub(crate) const fn get(self) -> &'static str {
        self.0
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, newtype::FromInner)]
pub(crate) struct ToolAvailable(bool);
impl ToolAvailable {
    pub(crate) const fn get(self) -> bool {
        self.0
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, newtype::FromInner)]
pub(crate) struct RunnerIoErrorRef<'error_lt>(&'error_lt std::io::Error);
impl<'error_lt> RunnerIoErrorRef<'error_lt> {
    pub(crate) const fn get(self) -> &'error_lt std::io::Error {
        self.0
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, newtype::FromInner)]
pub(crate) struct RunnerPathRef<'path_lt>(&'path_lt std::path::Path);
impl<'path_lt> RunnerPathRef<'path_lt> {
    pub(crate) const fn get(self) -> &'path_lt std::path::Path {
        self.0
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::AsRefStr, newtype::BoundedString,
)]
#[bounded_string(max = RUNNER_MODE_MAX_LEN)]
pub(crate) struct RunnerMode(String);
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy)]
pub(crate) struct AllocationTool {
    name: ToolName,
    path: ToolPath,
}
impl AllocationTool {
    pub(crate) const fn name(self) -> ToolName {
        self.name
    }

    pub(crate) const fn path(self) -> ToolPath {
        self.path
    }
}
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
#[allow(clippy::single_call_fn)] // runtime construction keeps wrapper initialization on From while centralizing tool metadata
pub(crate) fn allocation_tools() -> [AllocationTool; 6] {
    [
        AllocationTool {
            name: ToolName::from(constants_str::WORKSPACE_TEST_RUNNER_LIBMEMUSAGE_TOOL),
            path: ToolPath::from(constants_str::WORKSPACE_TEST_RUNNER_MEMUSAGE_PATH),
        },
        AllocationTool {
            name: ToolName::from(constants_str::WORKSPACE_TEST_RUNNER_VALGRIND_TOOL),
            path: ToolPath::from(constants_str::WORKSPACE_TEST_RUNNER_VALGRIND_PATH),
        },
        AllocationTool {
            name: ToolName::from(constants_str::WORKSPACE_TEST_RUNNER_HEAPTRACK_TOOL),
            path: ToolPath::from(constants_str::WORKSPACE_TEST_RUNNER_HEAPTRACK_PATH),
        },
        AllocationTool {
            name: ToolName::from(constants_str::WORKSPACE_TEST_RUNNER_LTRACE_TOOL),
            path: ToolPath::from(constants_str::WORKSPACE_TEST_RUNNER_LTRACE_PATH),
        },
        AllocationTool {
            name: ToolName::from(constants_str::WORKSPACE_TEST_RUNNER_PERF_TOOL),
            path: ToolPath::from(constants_str::WORKSPACE_TEST_RUNNER_PERF_PATH),
        },
        AllocationTool {
            name: ToolName::from(constants_str::PG_CRUD_PG_TIME),
            path: ToolPath::from(constants_str::WORKSPACE_TEST_RUNNER_TIME_PATH),
        },
    ]
}
pub(crate) fn strip_ansi_codes(value: AnsiTextRef<'_>) -> CleanAnsiText {
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
pub(crate) fn memusage_heap_value(text: &CleanAnsiText, key: MemusageKey) -> MemusageValueRef<'_> {
    text.0
        .as_str()
        .lines()
        .find_map(|line| line.split_once(key.get()).map(|(_, tail)| tail.trim()))
        .and_then(|tail| tail.split([',', ' ']).find(|part| !part.is_empty()))
        .map_or_else(
            || MemusageValueRef::from(constants_str::UNAVAILABLE),
            MemusageValueRef,
        )
}
pub(crate) fn memusage_table_value(
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
            || MemusageValueRef::from(constants_str::UNAVAILABLE),
            MemusageValueRef,
        )
}
#[cfg(test)]
mod tests;
