pub(super) fn measure_memusage_command(
    measurement_name: crate::measurement_name::MeasurementName,
    program_path_ref: crate::program_path_ref::ProgramPathRef<'_>,
    program_args_ref: crate::program_args_ref::ProgramArgsRef<'_>,
    memusage_prog_name_ref: crate::memusage_prog_name_ref::MemusageProgNameRef<'_>,
) -> Result<(), ()> {
    let measurement_name_value = measurement_name.get();
    if !std::path::Path::new(constants_str::WORKSPACE_TEST_RUNNER_MEMUSAGE_PATH).exists() {
        println!(
            "measurement={measurement_name_value}_allocations status=unavailable reason=libmemusage_not_found path={}",
            constants_str::WORKSPACE_TEST_RUNNER_MEMUSAGE_PATH
        );
        return Ok(());
    }
    let command_output = macro_helpers::tool_command::ToolCommand::new(
        macro_helpers::tool_program_ref::ToolProgramRef::from(program_path_ref.get()),
    )
    .args(macro_helpers::tool_args_ref::ToolArgsRef::from(
        program_args_ref.get(),
    ))
    .env(
        macro_helpers::tool_env_key_ref::ToolEnvKeyRef::from(constants_str::LD_PRELOAD),
        macro_helpers::tool_env_value_ref::ToolEnvValueRef::from(
            constants_str::WORKSPACE_TEST_RUNNER_MEMUSAGE_PATH,
        ),
    )
    .env(
        macro_helpers::tool_env_key_ref::ToolEnvKeyRef::from(constants_str::MEMUSAGE_PROG_NAME),
        macro_helpers::tool_env_value_ref::ToolEnvValueRef::from(memusage_prog_name_ref.get()),
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
            crate::print_without_memusage_footer::print_without_memusage_footer(
                crate::stderr_text_ref::StderrTextRef::from(stderr.as_ref()),
            );
            let clean = crate::strip_ansi_codes::strip_ansi_codes(
                crate::ansi_text_ref::AnsiTextRef::from(stderr.as_ref()),
            );
            let heap_total = crate::memusage_heap_value::memusage_heap_value(
                &clean,
                crate::memusage_key::MemusageKey::from(constants_str::HEAP_TOTAL),
            )
            .get();
            let heap_peak = crate::memusage_heap_value::memusage_heap_value(
                &clean,
                crate::memusage_key::MemusageKey::from(constants_str::HEAP_PEAK),
            )
            .get();
            let stack_peak = crate::memusage_heap_value::memusage_heap_value(
                &clean,
                crate::memusage_key::MemusageKey::from(constants_str::STACK_PEAK),
            )
            .get();
            let malloc_calls = crate::memusage_table_value::memusage_table_value(
                &clean,
                crate::memusage_row_name::MemusageRowName::from(constants_str::MALLOC),
                crate::memory_usage_column_index::MemoryUsageColumnIndex::from(0),
            )
            .get();
            let malloc_memory = crate::memusage_table_value::memusage_table_value(
                &clean,
                crate::memusage_row_name::MemusageRowName::from(constants_str::MALLOC),
                crate::memory_usage_column_index::MemoryUsageColumnIndex::from(1),
            )
            .get();
            let malloc_failed = crate::memusage_table_value::memusage_table_value(
                &clean,
                crate::memusage_row_name::MemusageRowName::from(constants_str::MALLOC),
                crate::memory_usage_column_index::MemoryUsageColumnIndex::from(2),
            )
            .get();
            let realloc_calls = crate::memusage_table_value::memusage_table_value(
                &clean,
                crate::memusage_row_name::MemusageRowName::from(constants_str::REALLOC),
                crate::memory_usage_column_index::MemoryUsageColumnIndex::from(0),
            )
            .get();
            let realloc_memory = crate::memusage_table_value::memusage_table_value(
                &clean,
                crate::memusage_row_name::MemusageRowName::from(constants_str::REALLOC),
                crate::memory_usage_column_index::MemoryUsageColumnIndex::from(1),
            )
            .get();
            let realloc_failed = crate::memusage_table_value::memusage_table_value(
                &clean,
                crate::memusage_row_name::MemusageRowName::from(constants_str::REALLOC),
                crate::memory_usage_column_index::MemoryUsageColumnIndex::from(2),
            )
            .get();
            let calloc_calls = crate::memusage_table_value::memusage_table_value(
                &clean,
                crate::memusage_row_name::MemusageRowName::from(constants_str::CALLOC),
                crate::memory_usage_column_index::MemoryUsageColumnIndex::from(0),
            )
            .get();
            let calloc_memory = crate::memusage_table_value::memusage_table_value(
                &clean,
                crate::memusage_row_name::MemusageRowName::from(constants_str::CALLOC),
                crate::memory_usage_column_index::MemoryUsageColumnIndex::from(1),
            )
            .get();
            let calloc_failed = crate::memusage_table_value::memusage_table_value(
                &clean,
                crate::memusage_row_name::MemusageRowName::from(constants_str::CALLOC),
                crate::memory_usage_column_index::MemoryUsageColumnIndex::from(2),
            )
            .get();
            let free_calls = crate::memusage_table_value::memusage_table_value(
                &clean,
                crate::memusage_row_name::MemusageRowName::from(constants_str::FREE),
                crate::memory_usage_column_index::MemoryUsageColumnIndex::from(0),
            )
            .get();
            let free_memory = crate::memusage_table_value::memusage_table_value(
                &clean,
                crate::memusage_row_name::MemusageRowName::from(constants_str::FREE),
                crate::memory_usage_column_index::MemoryUsageColumnIndex::from(1),
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
            crate::print_without_memusage_footer::print_without_memusage_footer(
                crate::stderr_text_ref::StderrTextRef::from(stderr.as_ref()),
            );
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
