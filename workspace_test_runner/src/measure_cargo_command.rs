pub(crate) fn measure_cargo_command(
    measurement_name: crate::domain_types::MeasurementName,
    args: crate::domain_types::CargoArgs,
) -> Result<(), ()> {
    let measurement_name_value = measurement_name.get();
    let started = std::time::Instant::now();
    let command_output = {
        let measurement_format = format!(
            "{}%M\n{}%R\n{}%F",
            constants_str::WORKSPACE_TEST_RUNNER_PEAK_RSS_PREFIX,
            constants_str::WORKSPACE_TEST_RUNNER_MINOR_PAGE_FAULTS_PREFIX,
            constants_str::WORKSPACE_TEST_RUNNER_MAJOR_PAGE_FAULTS_PREFIX,
        );
        macro_helpers::domain_types::tool_command::ToolCommand::new(
            macro_helpers::domain_types::tool_command::ToolProgramRef::from(
                constants_str::WORKSPACE_TEST_RUNNER_TIME_PATH,
            ),
        )
        .arg(macro_helpers::domain_types::tool_command::ToolArgRef::from(
            constants_str::F,
        ))
        .arg(macro_helpers::domain_types::tool_command::ToolArgRef::from(
            measurement_format.as_str(),
        ))
        .arg(macro_helpers::domain_types::tool_command::ToolArgRef::from(
            constants_str::WORKSPACE_TEST_RUNNER_CARGO,
        ))
        .args(macro_helpers::domain_types::tool_command::ToolArgsRef::from(args.get()))
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
                        .strip_prefix(constants_str::WORKSPACE_TEST_RUNNER_PEAK_RSS_PREFIX)
                })
                .unwrap_or(constants_str::UNAVAILABLE);
            let minor_page_faults = stderr
                .lines()
                .find_map(|line| {
                    line.trim()
                        .strip_prefix(constants_str::WORKSPACE_TEST_RUNNER_MINOR_PAGE_FAULTS_PREFIX)
                })
                .unwrap_or(constants_str::UNAVAILABLE);
            let major_page_faults = stderr
                .lines()
                .find_map(|line| {
                    line.trim()
                        .strip_prefix(constants_str::WORKSPACE_TEST_RUNNER_MAJOR_PAGE_FAULTS_PREFIX)
                })
                .unwrap_or(constants_str::UNAVAILABLE);
            {
                let stdout = String::from_utf8_lossy(output.stdout.as_slice());
                if !stdout.is_empty() {
                    print!("{stdout}");
                }
            }
            crate::adapters::print_without_measurement_footer::print_without_measurement_footer(
                crate::domain_types::StderrTextRef::from(stderr.as_ref()),
            );
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
            crate::adapters::print_without_measurement_footer::print_without_measurement_footer(
                crate::domain_types::StderrTextRef::from(stderr.as_ref()),
            );
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
