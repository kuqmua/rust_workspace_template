#[test]
fn test_generation_stage_measurement_propagates_each_fallible_stage() {
    let input = crate::measurement_name::MeasurementName::from(constants_str::STATIC);
    let failed_parse = crate::measure_generation_stages::measure_generation_stages(
        input,
        |_| {
            Err::<
                crate::measurement_name::MeasurementName,
                crate::summary_text_append_error::SummaryTextAppendError,
            >(crate::summary_text_append_error::SummaryTextAppendError::CapacityExceeded)
        },
        Ok::<_, crate::summary_text_append_error::SummaryTextAppendError>,
        Ok::<_, crate::summary_text_append_error::SummaryTextAppendError>,
        |value| value,
        |_| constants_usize::ZERO,
    );
    let failed_build = crate::measure_generation_stages::measure_generation_stages(
        input,
        Ok::<_, crate::summary_text_append_error::SummaryTextAppendError>,
        |_| {
            Err::<
                crate::measurement_name::MeasurementName,
                crate::summary_text_append_error::SummaryTextAppendError,
            >(crate::summary_text_append_error::SummaryTextAppendError::CapacityExceeded)
        },
        Ok::<_, crate::summary_text_append_error::SummaryTextAppendError>,
        |value| value,
        |_| constants_usize::ZERO,
    );
    let failed_validate = crate::measure_generation_stages::measure_generation_stages(
        input,
        Ok::<_, crate::summary_text_append_error::SummaryTextAppendError>,
        Ok::<_, crate::summary_text_append_error::SummaryTextAppendError>,
        |_| {
            Err::<
                crate::measurement_name::MeasurementName,
                crate::summary_text_append_error::SummaryTextAppendError,
            >(crate::summary_text_append_error::SummaryTextAppendError::CapacityExceeded)
        },
        |value| value,
        |_| constants_usize::ZERO,
    );
    assert!(matches!(
        failed_parse,
        Err(crate::summary_text_append_error::SummaryTextAppendError::CapacityExceeded)
    ));
    assert!(matches!(
        failed_build,
        Err(crate::summary_text_append_error::SummaryTextAppendError::CapacityExceeded)
    ));
    assert!(matches!(
        failed_validate,
        Err(crate::summary_text_append_error::SummaryTextAppendError::CapacityExceeded)
    ));
}

#[test]
fn test_bounded_capture_fits_command_log_after_utf8_replacement() {
    let stream_limit = crate::domain_types::COMMAND_CAPTURE_BYTES_PER_STREAM;
    let invalid_bytes = vec![u8::MAX; stream_limit];
    let text = String::from_utf8_lossy(invalid_bytes.as_slice());
    let log = format!("{text}{text}");
    assert!(
        crate::command_text::CommandText::try_from(log).is_ok_and(|command_text| {
            command_text.as_ref().len()
                == stream_limit * constants_usize::TWO * constants_usize::THREE
        })
    );
}

#[test]
fn test_bounded_capture_fits_memusage_clean_text_after_utf8_replacement() {
    let stream_limit = crate::domain_types::COMMAND_CAPTURE_BYTES_PER_STREAM;
    let invalid_bytes = vec![u8::MAX; stream_limit];
    let text = String::from_utf8_lossy(invalid_bytes.as_slice());
    assert!(
        crate::clean_ansi_text::CleanAnsiText::try_from(text.into_owned())
            .is_ok_and(|clean| clean.as_ref().len() == stream_limit * constants_usize::THREE)
    );
}

#[test]
fn test_ansi_adapters_preserve_distinct_length_error_fallbacks() {
    let maximum = constants_usize::VALUE_16_777_216;
    let length = maximum + constants_usize::ONE;
    let input = constants_str::X.repeat(length);
    let command_text =
        crate::strip_ansi::strip_ansi(macro_helpers::tool_ansi_chars::ToolAnsiChars::from(
            macro_helpers::tool_ansi_text_ref::ToolAnsiTextRef::from(input.as_str()),
        ));
    let clean_ansi_text = crate::strip_ansi_codes::strip_ansi_codes(
        macro_helpers::tool_ansi_chars::ToolAnsiChars::from(
            macro_helpers::tool_ansi_text_ref::ToolAnsiTextRef::from(input.as_str()),
        ),
    );
    assert_eq!(
        command_text.as_ref(),
        crate::command_text::CommandText::from(
            crate::command_text::CommandTextTryFromStringError::TooLong {
                len: length,
                max: maximum
            },
        )
        .as_ref()
    );
    assert_eq!(
        clean_ansi_text.as_ref(),
        crate::clean_ansi_text::CleanAnsiText::from(
            crate::clean_ansi_text::CleanAnsiTextTryFromStringError::TooLong {
                len: length,
                max: maximum
            },
        )
        .as_ref()
    );
}
#[test]
fn test_memusage_program_text_preserves_output_without_footer() {
    let result =
        crate::clean_ansi_text::CleanAnsiText::try_from(String::from(constants_str::VALUE_1));
    assert!(result.is_ok_and(|clean| {
        crate::memusage_program_text::memusage_program_text(&clean).get() == constants_str::VALUE_1
    }));
}

#[test]
fn test_memusage_program_text_keeps_lines_before_final_summary() {
    let input = [
        constants_str::VALUE_1,
        constants_str::MEMORY_USAGE_SUMMARY,
        constants_str::VALUE_2,
        constants_str::MEMORY_USAGE_SUMMARY,
        constants_str::VALUE_4,
    ]
    .join(constants_str::NEWLINE);
    let result = crate::clean_ansi_text::CleanAnsiText::try_from(input);
    assert!(result.is_ok_and(|clean| {
        crate::memusage_program_text::memusage_program_text(&clean)
            .get()
            .lines()
            .eq([
                constants_str::VALUE_1,
                constants_str::MEMORY_USAGE_SUMMARY,
                constants_str::VALUE_2,
            ])
    }));
}

#[test]
fn test_memusage_parsers_require_summary_marker() {
    let result = crate::clean_ansi_text::CleanAnsiText::try_from(format!(
        "{}{}{}{}{}",
        constants_str::HEAP_TOTAL,
        constants_str::SPACE,
        constants_str::VALUE_1,
        constants_str::NEWLINE,
        constants_str::MALLOC,
    ));
    assert!(result.is_ok_and(|text| {
        crate::memusage_heap_value::memusage_heap_value(
            &text,
            crate::memusage_key::MemusageKey::from(constants_str::HEAP_TOTAL),
        )
        .get()
            == constants_str::UNAVAILABLE
            && crate::memusage_table_value::memusage_table_value(
                &text,
                crate::memusage_row_name::MemusageRowName::from(constants_str::MALLOC),
                crate::memory_usage_column_index::MemoryUsageColumnIndex::from(
                    constants_usize::ZERO,
                ),
            )
            .get()
                == constants_str::UNAVAILABLE
    }));
}

#[test]
fn test_memusage_parsers_ignore_program_stderr_before_summary() {
    let text = [
        format!(
            "{}{}{}",
            constants_str::HEAP_TOTAL,
            constants_str::SPACE,
            constants_str::VALUE_1
        ),
        format!(
            "{}{}{}",
            constants_str::MALLOC,
            constants_str::SPACE,
            constants_str::VALUE_1
        ),
        format!(
            "{}{}{}{}{}",
            constants_str::MEMORY_USAGE_SUMMARY,
            constants_str::SPACE,
            constants_str::HEAP_TOTAL,
            constants_str::SPACE,
            constants_str::VALUE_2
        ),
        format!(
            "{}{}{}",
            constants_str::MALLOC,
            constants_str::SPACE,
            constants_str::VALUE_4
        ),
    ]
    .join(constants_str::NEWLINE);
    let result = crate::clean_ansi_text::CleanAnsiText::try_from(text);
    assert!(result.is_ok_and(|clean| {
        crate::memusage_heap_value::memusage_heap_value(
            &clean,
            crate::memusage_key::MemusageKey::from(constants_str::HEAP_TOTAL),
        )
        .get()
            == constants_str::VALUE_2
            && crate::memusage_table_value::memusage_table_value(
                &clean,
                crate::memusage_row_name::MemusageRowName::from(constants_str::MALLOC),
                crate::memory_usage_column_index::MemoryUsageColumnIndex::from(
                    constants_usize::ZERO,
                ),
            )
            .get()
                == constants_str::VALUE_4
    }));
}

#[test]
fn test_memusage_parsers_distinguish_values_and_missing_fields() {
    let text = crate::clean_ansi_text::CleanAnsiText::try_from(format!(
        "{}{}{}",
        constants_str::MEMORY_USAGE_SUMMARY,
        constants_str::NEWLINE,
        constants_str::VALUE_D36CD261
    ))
    .expect(constants_str::DIAGNOSTIC_AFA44055);
    assert_eq!(
        crate::memusage_heap_value::memusage_heap_value(
            &text,
            crate::memusage_key::MemusageKey::from(constants_str::VALUE_557B66DC)
        )
        .get(),
        constants_str::VALUE_1
    );
    assert_eq!(
        crate::memusage_heap_value::memusage_heap_value(
            &text,
            crate::memusage_key::MemusageKey::from(constants_str::VALUE_9164CD33)
        )
        .get(),
        constants_str::UNAVAILABLE
    );
    assert_eq!(
        crate::memusage_table_value::memusage_table_value(
            &text,
            crate::memusage_row_name::MemusageRowName::from(constants_str::VALUE_E3C52EBF),
            crate::memory_usage_column_index::MemoryUsageColumnIndex::from(constants_usize::ONE)
        )
        .get(),
        constants_str::VALUE_CD70BEA0
    );
    assert_eq!(
        crate::memusage_table_value::memusage_table_value(
            &text,
            crate::memusage_row_name::MemusageRowName::from(constants_str::VALUE_30EBF387),
            crate::memory_usage_column_index::MemoryUsageColumnIndex::from(constants_usize::ZERO)
        )
        .get(),
        constants_str::UNAVAILABLE
    );
    assert_eq!(
        crate::memusage_table_value::memusage_table_value(
            &text,
            crate::memusage_row_name::MemusageRowName::from(constants_str::VALUE_AD95D5FA),
            crate::memory_usage_column_index::MemoryUsageColumnIndex::from(9usize)
        )
        .get(),
        constants_str::UNAVAILABLE
    );
}
#[test]
fn test_measurement_catalogs_are_complete_and_ordered() {
    let measurements = crate::macro_generation_measurements::macro_generation_measurements();
    assert_eq!(measurements.len(), 3usize);
    assert_eq!(
        measurements[0].0.get(),
        constants_str::WORKSPACE_TEST_RUNNER_GENERATE_PG_TABLE_MEASUREMENT
    );
    assert_eq!(
        measurements[2].0.get(),
        constants_str::WORKSPACE_TEST_RUNNER_GENERATE_WHERE_FILTERS_MEASUREMENT
    );
    let tools = crate::allocation_tools::allocation_tools();
    assert_eq!(tools.len(), 6usize);
    assert_eq!(
        tools[0].get_name().get(),
        constants_str::WORKSPACE_TEST_RUNNER_LIBMEMUSAGE_TOOL
    );
    assert_eq!(tools[5].get_name().get(), constants_str::PG_CRUD_PG_TIME);
}
#[test]
fn test_tool_discovery_checks_the_exact_path() {
    assert!(
        !crate::check_tool_available::check_tool_available(crate::tool_path::ToolPath::from(env!(
            "CARGO_MANIFEST_DIR"
        )))
        .get()
    );
    assert!(
        !crate::check_tool_available::check_tool_available(crate::tool_path::ToolPath::from(
            constants_str::VALUE_54283E25
        ))
        .get()
    );
}
#[test]
fn test_database_mode_runs_the_workspace_ignored_suite() {
    assert!(
        constants_str::WORKSPACE_TEST_RUNNER_CARGO_TEST_DATABASE_ARGS
            .contains(&constants_str::SHARED_VALUES_WORKSPACE)
    );
    assert!(
        constants_str::WORKSPACE_TEST_RUNNER_CARGO_TEST_DATABASE_ARGS
            .contains(&constants_str::SHARED_VALUES_ALL_FEATURES)
    );
    assert!(
        constants_str::WORKSPACE_TEST_RUNNER_CARGO_TEST_DATABASE_ARGS
            .contains(&constants_str::SHARED_VALUES_IGNORED)
    );
}
#[test]
fn test_database_mode_isolates_each_observability_fixture_without_omitting_it() {
    assert_eq!(
        constants_str::WORKSPACE_TEST_RUNNER_DATABASE_COMMANDS.len(),
        3usize
    );
    assert!(
        [
            (
                constants_str::WORKSPACE_TEST_RUNNER_EXPORTER_TEST_TARGET,
                constants_str::WORKSPACE_TEST_RUNNER_EXPORTER_TEST_FILTER
            ),
            (
                constants_str::WORKSPACE_TEST_RUNNER_SUBSCRIBER_TEST_TARGET,
                constants_str::WORKSPACE_TEST_RUNNER_SUBSCRIBER_TEST_FILTER
            ),
        ]
        .into_iter()
        .all(|(target, filter)| {
            assert_eq!(
                constants_str::WORKSPACE_TEST_RUNNER_CARGO_TEST_DATABASE_ARGS
                    .windows(2)
                    .filter(|arguments| {
                        *arguments == [constants_str::MIGRATED_SKIP_ARGUMENT, filter]
                    })
                    .count(),
                1usize
            );
            constants_str::WORKSPACE_TEST_RUNNER_DATABASE_COMMANDS
                .iter()
                .filter(|(program, args)| {
                    *program == constants_str::SHARED_VALUES_ENV
                        && args.windows(2).any(|arguments| {
                            arguments
                                == [
                                    constants_str::WORKSPACE_TEST_RUNNER_TEST_TARGET_ARGUMENT,
                                    target,
                                ]
                        })
                        && args.last() == Some(&filter)
                        && args.contains(&constants_str::SHARED_VALUES_IGNORED)
                        && args.contains(&constants_str::SOURCE_PLACE_TEST_EXACT_ARGUMENT)
                        && args.windows(2).any(|arguments| {
                            arguments == [constants_str::P, stringify!(server_observability)]
                        })
                })
                .count()
                == 1usize
        })
    );
    assert!(
        constants_str::WORKSPACE_TEST_RUNNER_EXPORTER_TEST_ARGS
            .iter()
            .any(|argument| {
                argument.split_once('=').is_some_and(|(key, value)| {
                    key == stringify!(OTEL_EXPORTER_OTLP_TRACES_COMPRESSION)
                        && value == stringify!(unsupported)
                })
            })
    );
    assert!(
        [
            stringify!(OTEL_EXPORTER_OTLP_ENDPOINT),
            stringify!(OTEL_EXPORTER_OTLP_TRACES_ENDPOINT)
        ]
        .into_iter()
        .all(|key| {
            constants_str::WORKSPACE_TEST_RUNNER_SUBSCRIBER_TEST_ARGS
                .iter()
                .any(|argument| {
                    argument
                        .split_once('=')
                        .is_some_and(|(actual_key, value)| actual_key == key && value.is_empty())
                })
        })
    );
}
#[test]
fn test_tests_mode_leaves_ignored_suite_to_database_mode() {
    assert!(
        constants_str::WORKSPACE_TEST_RUNNER_CARGO_TEST_COMMANDS
            .iter()
            .all(|(_program, args)| !args.contains(&constants_str::SHARED_VALUES_IGNORED))
    );
    assert!(
        constants_str::WORKSPACE_TEST_RUNNER_NEXTEST_COMMANDS
            .iter()
            .all(|(_program, args)| !args.contains(&constants_str::SHARED_VALUES_RUN_IGNORED))
    );
}

#[test]
fn test_database_mode_excludes_tests_requiring_a_deployed_application() {
    assert!(
        constants_str::WORKSPACE_TEST_RUNNER_CARGO_TEST_DATABASE_ARGS
            .windows(2)
            .any(|arguments| arguments
                == [
                    constants_str::MIGRATED_SKIP_ARGUMENT,
                    constants_str::MIGRATED_DEPLOYMENT_TEST_NAME
                ])
    );
}

#[test]
fn test_ansi_adapters_bound_filtered_output_and_preserve_empty_unicode_and_limit_text() {
    let maximum = constants_usize::VALUE_16_777_216;
    [
        String::new(),
        char::from(233u8).to_string(),
        constants_str::X.repeat(maximum),
    ]
    .into_iter()
    .fold((), |(), expected| {
        let input = format!(
            "{}{}{}",
            constants_str::WORKSPACE_TEST_RUNNER_ANSI_RED,
            expected,
            constants_str::WORKSPACE_TEST_RUNNER_ANSI_RESET
        );
        if expected.len() == maximum {
            assert!(input.len() > maximum);
        }
        let command_text =
            crate::strip_ansi::strip_ansi(macro_helpers::tool_ansi_chars::ToolAnsiChars::from(
                macro_helpers::tool_ansi_text_ref::ToolAnsiTextRef::from(input.as_str()),
            ));
        let clean_text = crate::strip_ansi_codes::strip_ansi_codes(
            macro_helpers::tool_ansi_chars::ToolAnsiChars::from(
                macro_helpers::tool_ansi_text_ref::ToolAnsiTextRef::from(input.as_str()),
            ),
        );
        assert_eq!(command_text.as_ref(), expected.as_str());
        assert_eq!(clean_text.as_ref(), expected.as_str());
    });
}

#[test]
fn test_ansi_adapters_report_filtered_unicode_byte_lengths_in_overflow_fallbacks() {
    let maximum = constants_usize::VALUE_16_777_216;
    let unicode = char::from(233u8).to_string();
    let text = unicode.repeat(
        maximum
            .checked_div(unicode.len())
            .unwrap_or_default()
            .saturating_add(constants_usize::ONE),
    );
    let input = format!(
        "{}{}{}",
        constants_str::WORKSPACE_TEST_RUNNER_ANSI_RED,
        text,
        constants_str::WORKSPACE_TEST_RUNNER_ANSI_RESET
    );
    assert_eq!(text.len(), maximum.saturating_add(2usize));
    assert!(input.len() > text.len());
    let command_text =
        crate::strip_ansi::strip_ansi(macro_helpers::tool_ansi_chars::ToolAnsiChars::from(
            macro_helpers::tool_ansi_text_ref::ToolAnsiTextRef::from(input.as_str()),
        ));
    let clean_text = crate::strip_ansi_codes::strip_ansi_codes(
        macro_helpers::tool_ansi_chars::ToolAnsiChars::from(
            macro_helpers::tool_ansi_text_ref::ToolAnsiTextRef::from(input.as_str()),
        ),
    );
    assert_eq!(
        command_text.as_ref(),
        crate::command_text::CommandText::from(
            crate::command_text::CommandTextTryFromStringError::TooLong {
                len: text.len(),
                max: maximum
            },
        )
        .as_ref()
    );
    assert_eq!(
        clean_text.as_ref(),
        crate::clean_ansi_text::CleanAnsiText::from(
            crate::clean_ansi_text::CleanAnsiTextTryFromStringError::TooLong {
                len: text.len(),
                max: maximum
            },
        )
        .as_ref()
    );
}

#[test]
fn test_memusage_value_parsers_use_final_summary_even_when_empty() {
    let preceding = format!(
        "{}{}{}{}{}{}{}{}",
        constants_str::MEMORY_USAGE_SUMMARY,
        constants_str::NEWLINE,
        constants_str::HEAP_TOTAL,
        constants_str::VALUE_1,
        constants_str::NEWLINE,
        constants_str::MALLOC,
        constants_str::VALUE_1,
        constants_str::NEWLINE
    );
    [false, true].into_iter().fold((), |(), empty| {
        let summary = if empty {
            String::new()
        } else {
            format!(
                "{}{}{}{}{}{}{}",
                constants_str::NEWLINE,
                constants_str::HEAP_TOTAL,
                constants_str::VALUE_2,
                constants_str::NEWLINE,
                constants_str::MALLOC,
                constants_str::VALUE_4,
                constants_str::NEWLINE
            )
        };
        let input = format!(
            "{preceding}{}{summary}",
            constants_str::MEMORY_USAGE_SUMMARY
        );
        assert!(
            crate::clean_ansi_text::CleanAnsiText::try_from(input).is_ok_and(|text| {
                crate::memusage_summary_text::memusage_summary_text(&text)
                    .is_some_and(|value| value.get() == summary.as_str())
                    && crate::memusage_program_text::memusage_program_text(&text).get()
                        == preceding.as_str()
                    && crate::memusage_heap_value::memusage_heap_value(
                        &text,
                        crate::memusage_key::MemusageKey::from(constants_str::HEAP_TOTAL),
                    )
                    .get()
                        == if empty {
                            constants_str::UNAVAILABLE
                        } else {
                            constants_str::VALUE_2
                        }
                    && crate::memusage_table_value::memusage_table_value(
                        &text,
                        crate::memusage_row_name::MemusageRowName::from(
                            constants_str::VALUE_E3C52EBF,
                        ),
                        crate::memory_usage_column_index::MemoryUsageColumnIndex::from(
                            constants_usize::ZERO,
                        ),
                    )
                    .get()
                        == if empty {
                            constants_str::UNAVAILABLE
                        } else {
                            constants_str::VALUE_4
                        }
            })
        );
    });
}

#[test]
fn test_memusage_parsers_keep_first_matching_row_including_malformed_matches() {
    [
        (
            format!("{}{}", constants_str::HEAP_TOTAL, constants_str::VALUE_1),
            format!("{}{}", constants_str::MALLOC, constants_str::VALUE_1),
            constants_str::VALUE_1,
        ),
        (
            constants_str::HEAP_TOTAL.to_owned(),
            constants_str::VALUE_E3C52EBF.to_owned(),
            constants_str::UNAVAILABLE,
        ),
        (
            format!(
                "{}{}{}",
                constants_str::HEAP_TOTAL,
                ',',
                constants_str::SPACE
            ),
            format!("{}{}", constants_str::MALLOC, constants_str::SPACE),
            constants_str::UNAVAILABLE,
        ),
    ]
    .into_iter()
    .fold((), |(), (first_heap, first_row, expected)| {
        let input = [
            constants_str::MEMORY_USAGE_SUMMARY.to_owned(),
            first_heap,
            first_row,
            format!("{}{}", constants_str::HEAP_TOTAL, constants_str::VALUE_2),
            format!("{}{}", constants_str::MALLOC, constants_str::VALUE_4),
        ]
        .join(constants_str::NEWLINE);
        assert!(
            crate::clean_ansi_text::CleanAnsiText::try_from(input).is_ok_and(|text| {
                crate::memusage_heap_value::memusage_heap_value(
                    &text,
                    crate::memusage_key::MemusageKey::from(constants_str::HEAP_TOTAL),
                )
                .get()
                    == expected
                    && crate::memusage_table_value::memusage_table_value(
                        &text,
                        crate::memusage_row_name::MemusageRowName::from(
                            constants_str::VALUE_E3C52EBF,
                        ),
                        crate::memory_usage_column_index::MemoryUsageColumnIndex::from(
                            constants_usize::ZERO,
                        ),
                    )
                    .get()
                        == expected
            })
        );
    });
}

#[test]
fn test_memusage_heap_preserves_substring_and_comma_space_internal_tab_rules() {
    let internal_tab = format!(
        "{}{}{}",
        constants_str::VALUE_1,
        char::from(9u8),
        constants_str::VALUE_2
    );
    [
        (
            format!(
                "{0},{0},{1},{0}{2}",
                constants_str::SPACE,
                constants_str::VALUE_1,
                constants_str::VALUE_2
            ),
            constants_str::VALUE_1,
        ),
        (
            format!("{}{}{}", char::from(9u8), internal_tab, char::from(9u8)),
            internal_tab.as_str(),
        ),
    ]
    .into_iter()
    .fold((), |(), (tail, expected)| {
        let input = format!(
            "{}{}{}{}{tail}",
            constants_str::MEMORY_USAGE_SUMMARY,
            constants_str::NEWLINE,
            constants_str::X,
            constants_str::HEAP_TOTAL
        );
        assert!(
            crate::clean_ansi_text::CleanAnsiText::try_from(input).is_ok_and(|text| {
                crate::memusage_heap_value::memusage_heap_value(
                    &text,
                    crate::memusage_key::MemusageKey::from(constants_str::HEAP_TOTAL),
                )
                .get()
                    == expected
            })
        );
    });
}

#[test]
fn test_memusage_table_preserves_substring_first_pipe_and_whitespace_column_rules() {
    let input = format!(
        "{}{}{}{}{}|{}{}{}{}{}|{}",
        constants_str::MEMORY_USAGE_SUMMARY,
        constants_str::NEWLINE,
        constants_str::X,
        constants_str::VALUE_E3C52EBF,
        constants_str::X,
        constants_str::VALUE_1,
        char::from(9u8),
        constants_str::VALUE_2,
        constants_str::SPACE,
        constants_str::VALUE_4,
        constants_str::JSON
    );
    assert!(
        crate::clean_ansi_text::CleanAnsiText::try_from(input).is_ok_and(|text| {
            [
                (constants_usize::ZERO, constants_str::VALUE_1),
                (constants_usize::ONE, constants_str::VALUE_2),
                (2usize, constants_str::VALUE_4),
                (3usize, constants_str::UNAVAILABLE),
                (
                    std::num::NonZeroUsize::MAX.get(),
                    constants_str::UNAVAILABLE,
                ),
            ]
            .into_iter()
            .all(|(index, expected)| {
                crate::memusage_table_value::memusage_table_value(
                    &text,
                    crate::memusage_row_name::MemusageRowName::from(constants_str::VALUE_E3C52EBF),
                    crate::memory_usage_column_index::MemoryUsageColumnIndex::from(index),
                )
                .get()
                    == expected
            })
        })
    );
}

#[test]
fn test_failed_name_parser_preserves_empty_unicode_and_whitespace_names_across_crlf_formats() {
    let unicode = char::from(233u8).to_string();
    let whitespace = format!(
        "{}{}{}",
        constants_str::SPACE,
        constants_str::X,
        char::from(9u8)
    );
    [constants_str::EMPTY, unicode.as_str(), whitespace.as_str()]
        .into_iter()
        .fold((), |(), expected| {
            let cargo = format!(
                "{}{}{}",
                constants_str::TEST_ALT,
                expected,
                constants_str::FAILED_ALT
            );
            let nextest = format!(
                "{}{}{}",
                constants_str::FOUR_SPACES,
                expected,
                constants_str::FAILED
            );
            let input = format!(
                "{cargo}{}{}{nextest}{}{}",
                char::from(13u8),
                constants_str::NEWLINE,
                char::from(13u8),
                constants_str::NEWLINE
            );
            let names = crate::failed_test_names::failed_test_names(
                crate::text_ref::TextRef::from(input.as_str()),
            );
            assert!(
                names
                    .as_ref()
                    .iter()
                    .map(crate::command_text::CommandText::as_ref)
                    .eq([expected])
            );
        });
}

#[test]
fn test_failed_name_parser_preserves_exact_filtered_limit_and_discards_overflow_log() {
    let maximum = constants_usize::VALUE_16_777_216;
    let line = format!(
        "{}{}{}{}",
        constants_str::TEST_ALT,
        constants_str::X,
        constants_str::FAILED_ALT,
        constants_str::NEWLINE
    );
    [false, true].into_iter().fold((), |(), overflow| {
        let length = if overflow {
            maximum.saturating_add(constants_usize::ONE)
        } else {
            maximum
        };
        let padding = constants_str::X.repeat(length.saturating_sub(line.len()));
        let filtered = format!("{line}{padding}");
        assert_eq!(filtered.len(), length);
        let input = format!(
            "{}{}{}",
            constants_str::WORKSPACE_TEST_RUNNER_ANSI_RED,
            filtered,
            constants_str::WORKSPACE_TEST_RUNNER_ANSI_RESET
        );
        assert!(input.len() > maximum);
        let names = crate::failed_test_names::failed_test_names(crate::text_ref::TextRef::from(
            input.as_str(),
        ));
        if overflow {
            assert!(names.as_ref().is_empty());
        } else {
            assert!(
                names
                    .as_ref()
                    .iter()
                    .map(crate::command_text::CommandText::as_ref)
                    .eq([constants_str::X])
            );
        }
    });
}

#[test]
fn test_tool_discovery_tracks_file_creation_removal_and_directory_replacement() {
    let name =
        stringify!(test_tool_discovery_tracks_file_creation_removal_and_directory_replacement);
    let tool_path = crate::tool_path::ToolPath::from(name);
    let available = || crate::check_tool_available::check_tool_available(tool_path).get();
    assert!(!available());
    assert!(matches!(std::fs::write(name, constants_str::JSON), Ok(())));
    assert!(available());
    assert!(std::fs::read_to_string(name).is_ok_and(|text| text == constants_str::JSON));
    assert!(matches!(std::fs::remove_file(name), Ok(())));
    assert!(!available());
    assert!(matches!(std::fs::create_dir_all(name), Ok(())));
    assert!(!available());
    assert!(matches!(std::fs::remove_dir_all(name), Ok(())));
    assert!(!available());
}

#[test]
fn test_generation_stage_callbacks_preserve_handoff_order_and_stop_after_errors() {
    assert!([None, Some(0usize), Some(1usize), Some(2usize)]
        .into_iter()
        .all(|failed_stage| {
            let calls = std::cell::Cell::new(crate::command_index::CommandIndex::from(0usize));
            let advance = |command_index: crate::command_index::CommandIndex| {
                let stage = usize::from(command_index);
                assert_eq!(usize::from(calls.get()), stage);
                calls.set(crate::command_index::CommandIndex::from(stage + 1usize));
                if failed_stage == Some(stage) {
                    Err(crate::summary_text_append_error::SummaryTextAppendError::CapacityExceeded)
                } else {
                    Ok(crate::command_index::CommandIndex::from(stage + 1usize))
                }
            };
            let result = crate::measure_generation_stages::measure_generation_stages(
                crate::command_index::CommandIndex::from(0usize),
                advance,
                advance,
                advance,
                |command_index: crate::command_index::CommandIndex| {
                    assert_eq!(usize::from(command_index), 3usize);
                    assert_eq!(usize::from(calls.get()), 3usize);
                    calls.set(crate::command_index::CommandIndex::from(4usize));
                    crate::command_index::CommandIndex::from(4usize)
                },
                |command_index: &crate::command_index::CommandIndex| {
                    assert_eq!(usize::from(*command_index), 4usize);
                    assert_eq!(usize::from(calls.get()), 4usize);
                    calls.set(crate::command_index::CommandIndex::from(5usize));
                    17usize
                },
            );
            if let Some(stage) = failed_stage {
                assert_eq!(usize::from(calls.get()), stage + 1usize);
                matches!(result, Err(crate::summary_text_append_error::SummaryTextAppendError::CapacityExceeded))
            } else {
                assert_eq!(usize::from(calls.get()), 5usize);
                result.is_ok_and(|measurement| *measurement.get_output_bytes() == 17usize)
            }
        }));
}

#[test]
fn test_direct_generation_inspects_every_output_in_order_and_preserves_final_sizes() {
    let generated = std::cell::Cell::new(crate::command_index::CommandIndex::from(0usize));
    let inspected = std::cell::Cell::new(crate::command_index::CommandIndex::from(0usize));
    let measurement = crate::measure_direct_generation::measure_direct_generation(
        || {
            assert_eq!(usize::from(generated.get()), usize::from(inspected.get()));
            let command_index =
                crate::command_index::CommandIndex::from(usize::from(generated.get()) + 1usize);
            generated.set(command_index);
            command_index
        },
        |command_index: &crate::command_index::CommandIndex| {
            let index = usize::from(*command_index);
            assert_eq!(index, usize::from(generated.get()));
            assert_eq!(index, usize::from(inspected.get()) + 1usize);
            inspected.set(*command_index);
            crate::direct_generation_output_measurement::DirectGenerationOutputMeasurement::new(
                index,
                index + 1usize,
            )
        },
    );
    assert_eq!(
        usize::from(generated.get()),
        crate::domain_types::DIRECT_GENERATION_REPEAT_COUNT
    );
    assert_eq!(
        usize::from(inspected.get()),
        crate::domain_types::DIRECT_GENERATION_REPEAT_COUNT
    );
    assert_eq!(
        *measurement.get_output_bytes(),
        crate::domain_types::DIRECT_GENERATION_REPEAT_COUNT
    );
    assert_eq!(
        *measurement.get_output_token_trees(),
        crate::domain_types::DIRECT_GENERATION_REPEAT_COUNT + 1usize
    );
}

#[test]
fn test_program_array_conversion_preserves_empty_and_nonempty_borrowed_slices() {
    let empty = [];
    let single = [constants_str::EMPTY];
    let multiple = [
        constants_str::TEST_ALT_3,
        constants_str::P,
        constants_str::TESTS_ALT,
    ];
    assert!(
        [
            (
                crate::program_args_ref::ProgramArgsRef::from(&empty),
                empty.as_slice()
            ),
            (
                crate::program_args_ref::ProgramArgsRef::from(&single),
                single.as_slice()
            ),
            (
                crate::program_args_ref::ProgramArgsRef::from(&multiple),
                multiple.as_slice()
            ),
        ]
        .into_iter()
        .all(|(program_args_ref, expected)| {
            program_args_ref.get() == expected && std::ptr::eq(program_args_ref.get(), expected)
        })
    );
}

#[test]
fn test_generation_measurement_input_preserves_configurable_test_write_flag() {
    assert!([stringify!(False), stringify!(True)].into_iter().all(|text| {
        let input = crate::generate_pg_table_measure_input_token_stream::generate_pg_table_measure_input_token_stream(&text);
        generate_pg_table_src::parse_generate_pg_table::parse_generate_pg_table(
            macro_helpers::proc_macro2_token_stream_ref::ProcMacro2TokenStreamRef::from(input.as_ref()),
        ).is_ok_and(|parsed| {
            parsed.get_inner().attrs.iter().find(|attribute| {
                attribute.path().segments.last().is_some_and(|segment| segment.ident == stringify!(generate_pg_table_config))
            }).is_some_and(|attribute| {
                attribute.meta.require_list().is_ok_and(|list| {
                    serde_json::from_str::<serde_json::Value>(&list.tokens.to_string()).is_ok_and(|value| {
                        value.get(stringify!(tests_write_into_file)).and_then(serde_json::Value::as_str) == Some(text)
                    })
                })
            })
        })
    }));
}
