pub fn parse_timestamp_filter_wire_json(
    form_value_ref: crate::form_value_ref::FormValueRef<'_>,
    value_format: crate::value_format::ValueFormat,
) -> Result<crate::filter_wire_json::FilterWireJson, crate::form_value_error::FormValueError> {
    let invalid_value = || {
        crate::form_value_error::FormValueError::try_from(
            constants_str::INVALID_FILTER_SPECIFICATION.to_owned(),
        )
        .unwrap_or_default()
    };
    let date_name = match value_format {
        crate::value_format::ValueFormat::Timestamp => constants_str::PG_CRUD_PG_DATE,
        crate::value_format::ValueFormat::TimestampTz => constants_str::DATE_NAIVE,
        crate::value_format::ValueFormat::Bool
        | crate::value_format::ValueFormat::Bytes
        | crate::value_format::ValueFormat::Date
        | crate::value_format::ValueFormat::DateTime
        | crate::value_format::ValueFormat::Float32
        | crate::value_format::ValueFormat::Float64
        | crate::value_format::ValueFormat::Inet
        | crate::value_format::ValueFormat::Int16
        | crate::value_format::ValueFormat::Int32
        | crate::value_format::ValueFormat::Int64
        | crate::value_format::ValueFormat::Interval
        | crate::value_format::ValueFormat::Mac
        | crate::value_format::ValueFormat::Range
        | crate::value_format::ValueFormat::Text
        | crate::value_format::ValueFormat::Time
        | crate::value_format::ValueFormat::Uuid => return Err(invalid_value()),
    };
    let (date, time) = form_value_ref
        .as_ref()
        .split_once('T')
        .ok_or_else(invalid_value)?;
    let (year_text, month_and_day) = date.split_once('-').ok_or_else(invalid_value)?;
    let (month_text, day_text) = month_and_day.split_once('-').ok_or_else(invalid_value)?;
    if year_text.len() != 4usize
        || month_text.len() != 2usize
        || day_text.len() != 2usize
        || ![year_text, month_text, day_text]
            .into_iter()
            .all(|part| part.bytes().all(|byte| byte.is_ascii_digit()))
    {
        return Err(invalid_value());
    }
    let parse_date_component =
        |date_component_form_value_ref: crate::form_value_ref::FormValueRef<'_>| {
            date_component_form_value_ref
                .as_ref()
                .parse::<u32>()
                .map_err(|error| {
                    crate::form_value_error::FormValueError::try_from(error.to_string())
                        .unwrap_or_default()
                })
        };
    let year = parse_date_component(crate::form_value_ref::FormValueRef::from(year_text))?;
    let month = parse_date_component(crate::form_value_ref::FormValueRef::from(month_text))?;
    let day = parse_date_component(crate::form_value_ref::FormValueRef::from(day_text))?;
    let leap_year =
        year.is_multiple_of(4u32) && (!year.is_multiple_of(100u32) || year.is_multiple_of(400u32));
    let maximum_day = match month {
        1u32 | 3u32 | 5u32 | 7u32 | 8u32 | 10u32 | 12u32 => 31u32,
        4u32 | 6u32 | 9u32 | 11u32 => 30u32,
        2u32 if leap_year => 29u32,
        2u32 => 28u32,
        _ => return Err(invalid_value()),
    };
    if year == 0u32 || day == 0u32 || day > maximum_day {
        return Err(invalid_value());
    }
    let mut time_parts = time.split(':');
    let hour = time_parts
        .next()
        .ok_or_else(invalid_value)?
        .parse::<u32>()
        .map_err(|error| {
            crate::form_value_error::FormValueError::try_from(error.to_string()).unwrap_or_default()
        })?;
    let minute = time_parts
        .next()
        .ok_or_else(invalid_value)?
        .parse::<u32>()
        .map_err(|error| {
            crate::form_value_error::FormValueError::try_from(error.to_string()).unwrap_or_default()
        })?;
    let second_and_fraction = time_parts.next().unwrap_or(stringify!(0));
    if time_parts.next().is_some() || hour > 23u32 || minute > 59u32 {
        return Err(invalid_value());
    }
    let (second_text, fraction) = second_and_fraction
        .split_once('.')
        .unwrap_or((second_and_fraction, constants_str::EMPTY));
    let second = second_text.parse::<u32>().map_err(|error| {
        crate::form_value_error::FormValueError::try_from(error.to_string()).unwrap_or_default()
    })?;
    if second > 59u32
        || fraction.len() > 6usize
        || !fraction.bytes().all(|byte| byte.is_ascii_digit())
    {
        return Err(invalid_value());
    }
    let mut microsecond_text = fraction.to_owned();
    microsecond_text.extend(std::iter::repeat_n(
        '0',
        6usize.saturating_sub(fraction.len()),
    ));
    let microsecond = microsecond_text.parse::<u32>().map_err(|error| {
        crate::form_value_error::FormValueError::try_from(error.to_string()).unwrap_or_default()
    })?;
    let wire = serde_json::json!({
        (date_name): date,
        (constants_str::PG_CRUD_PG_TIME): {
            (constants_str::HOUR): hour,
            (constants_str::MIN): minute,
            (constants_str::SEC): second,
            (constants_str::MICRO): microsecond,
        }
    });
    crate::filter_wire_json::FilterWireJson::try_from(wire.to_string()).map_err(|error| {
        crate::form_value_error::FormValueError::try_from(error.to_string()).unwrap_or_default()
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_timestamp_filter_normalizes_zero_variable_width_and_leading_plus_time_fields() {
        assert!(constants_str::VALUE_2026_07_13T12_30_00.split_once('T').is_some_and(|(date, _time)| {
            [([0u32, 0u32, 0u32], false), ([1u32, 2u32, 3u32], false), ([1u32, 2u32, 3u32], true)].into_iter().all(|(components, leading_plus)| {
                let texts = components.map(|component| if leading_plus { format!("+{component}") } else { component.to_string() });
                let input = format!("{}T{}:{}:{}", date, texts[0usize], texts[1usize], texts[2usize]);
                [(crate::value_format::ValueFormat::Timestamp, constants_str::PG_CRUD_PG_DATE), (crate::value_format::ValueFormat::TimestampTz, constants_str::DATE_NAIVE)].into_iter().all(|(value_format, date_name)| {
                    let expected = serde_json::json!({(date_name): date, (constants_str::PG_CRUD_PG_TIME): {(constants_str::HOUR): components[0usize], (constants_str::MIN): components[1usize], (constants_str::SEC): components[2usize], (constants_str::MICRO): 0u32}});
                    super::parse_timestamp_filter_wire_json(crate::form_value_ref::FormValueRef::from(input.as_str()), value_format).is_ok_and(|wire| serde_json::from_str::<serde_json::Value>(wire.as_ref()).is_ok_and(|actual| actual == expected))
                })
            })
        }));
    }

    #[test]
    fn test_timestamp_filter_rejects_non_digit_date_parts_before_integer_parsing() {
        assert!([0usize, 5usize, 8usize].into_iter().all(|position| {
            ['x', '+', ' '].into_iter().all(|replacement| {
                let input = constants_str::VALUE_2026_07_13T12_30_00
                    .chars()
                    .enumerate()
                    .map(|(index, character)| {
                        if index == position {
                            replacement
                        } else {
                            character
                        }
                    })
                    .collect::<String>();
                [
                    crate::value_format::ValueFormat::Timestamp,
                    crate::value_format::ValueFormat::TimestampTz,
                ]
                .into_iter()
                .all(|value_format| {
                    super::parse_timestamp_filter_wire_json(
                        crate::form_value_ref::FormValueRef::from(input.as_str()),
                        value_format,
                    )
                    .is_err_and(|error| {
                        error.to_string() == constants_str::INVALID_FILTER_SPECIFICATION
                    })
                })
            })
        }));
    }

    #[test]
    fn test_timestamp_filter_accepts_last_day_and_last_second_with_exact_wire_values() {
        let input = constants_str::VALUE_2026_07_13T12_30_00
            .chars()
            .enumerate()
            .map(|(index, character)| match index {
                8usize | 12usize => '3',
                9usize => '1',
                11usize => '2',
                14usize | 17usize => '5',
                15usize | 18usize => '9',
                _ => character,
            })
            .collect::<String>();
        assert!(
            [
                crate::value_format::ValueFormat::Timestamp,
                crate::value_format::ValueFormat::TimestampTz
            ]
            .into_iter()
            .all(|value_format| super::parse_timestamp_filter_wire_json(
                crate::form_value_ref::FormValueRef::from(input.as_str()),
                value_format
            )
            .is_ok_and(
                |wire| serde_json::from_str::<serde_json::Value>(wire.as_ref()).is_ok_and(|json| [
                    (constants_str::HOUR, 23u64),
                    (constants_str::MIN, 59u64),
                    (constants_str::SEC, 59u64),
                    (constants_str::MICRO, 0u64)
                ]
                .into_iter()
                .all(|(field, expected)| json
                    .get(constants_str::PG_CRUD_PG_TIME)
                    .and_then(|time| time.get(field))
                    .and_then(serde_json::Value::as_u64)
                    == Some(expected)))
            ))
        );
    }

    #[test]
    fn test_timestamp_filter_rejects_every_non_timestamp_format() {
        assert!(
            [
                crate::value_format::ValueFormat::Bool,
                crate::value_format::ValueFormat::Bytes,
                crate::value_format::ValueFormat::Date,
                crate::value_format::ValueFormat::DateTime,
                crate::value_format::ValueFormat::Float32,
                crate::value_format::ValueFormat::Float64,
                crate::value_format::ValueFormat::Inet,
                crate::value_format::ValueFormat::Int16,
                crate::value_format::ValueFormat::Int32,
                crate::value_format::ValueFormat::Int64,
                crate::value_format::ValueFormat::Interval,
                crate::value_format::ValueFormat::Mac,
                crate::value_format::ValueFormat::Range,
                crate::value_format::ValueFormat::Text,
                crate::value_format::ValueFormat::Time,
                crate::value_format::ValueFormat::Uuid,
            ]
            .into_iter()
            .all(|value_format| super::parse_timestamp_filter_wire_json(
                crate::form_value_ref::FormValueRef::from(constants_str::VALUE_2026_07_13T12_30_00),
                value_format
            )
            .is_err_and(|error| error.to_string() == constants_str::INVALID_FILTER_SPECIFICATION))
        );
    }

    #[test]
    fn test_timestamp_filter_incomplete_time_and_date_widths_preserve_invalid_diagnostic() {
        let missing_minute = constants_str::VALUE_2026_07_13T12_30_00
            .split_once(':')
            .map(|(prefix, _remaining)| prefix);
        assert!(missing_minute.is_some());
        let Some(prefix) = missing_minute else {
            return;
        };
        let inputs = [
            constants_str::VALUE_2026_07_13T12_30_00.replace('-', constants_str::EMPTY),
            prefix.to_owned(),
            constants_str::VALUE_2026_07_13T12_30_00
                .chars()
                .chain([':', '0'])
                .collect::<String>(),
        ]
        .into_iter()
        .chain([0usize, 5usize, 8usize].into_iter().map(|position| {
            constants_str::VALUE_2026_07_13T12_30_00
                .chars()
                .enumerate()
                .filter_map(|(index, character)| (index != position).then_some(character))
                .collect::<String>()
        }));
        assert!(inputs.into_iter().all(|input| {
            super::parse_timestamp_filter_wire_json(
                crate::form_value_ref::FormValueRef::from(input.as_str()),
                crate::value_format::ValueFormat::Timestamp,
            )
            .is_err_and(|error| error.to_string() == constants_str::INVALID_FILTER_SPECIFICATION)
        }));
    }

    #[test]
    fn test_timestamp_filter_thirty_day_month_and_zero_calendar_parts() {
        let april = constants_str::VALUE_2026_07_13T12_30_00
            .chars()
            .enumerate()
            .map(|(index, character)| if index == 6usize { '4' } else { character })
            .collect::<String>();
        assert!(
            super::parse_timestamp_filter_wire_json(
                crate::form_value_ref::FormValueRef::from(april.as_str()),
                crate::value_format::ValueFormat::Timestamp
            )
            .is_ok_and(|wire| !wire.as_ref().is_empty())
        );
        let invalid_april = april
            .chars()
            .enumerate()
            .map(|(index, character)| match index {
                8usize => '3',
                9usize => '1',
                _ => character,
            })
            .collect::<String>();
        let zero_year = constants_str::VALUE_2026_07_13T12_30_00
            .chars()
            .enumerate()
            .map(|(index, character)| if index < 4usize { '0' } else { character })
            .collect::<String>();
        let zero_day = constants_str::VALUE_2026_07_13T12_30_00
            .chars()
            .enumerate()
            .map(|(index, character)| {
                if (8usize..10usize).contains(&index) {
                    '0'
                } else {
                    character
                }
            })
            .collect::<String>();
        assert!(
            [invalid_april, zero_year, zero_day]
                .into_iter()
                .all(|input| super::parse_timestamp_filter_wire_json(
                    crate::form_value_ref::FormValueRef::from(input.as_str()),
                    crate::value_format::ValueFormat::Timestamp
                )
                .is_err_and(
                    |error| error.to_string() == constants_str::INVALID_FILTER_SPECIFICATION
                ))
        );
    }

    #[test]
    fn test_timestamp_filter_time_parse_errors_preserve_integer_diagnostics() {
        let overflow = u32::MAX
            .to_string()
            .chars()
            .chain(['0'])
            .collect::<String>();
        assert!(
            [constants_str::X, overflow.as_str()]
                .into_iter()
                .all(|invalid| {
                    let expected = invalid.parse::<u32>().err().map(|error| error.to_string());
                    assert!(expected.is_some());
                    [11usize, 14usize, 17usize].into_iter().all(|position| {
                        let mut input = constants_str::VALUE_2026_07_13T12_30_00.to_owned();
                        input.replace_range(position..position.saturating_add(2usize), invalid);
                        super::parse_timestamp_filter_wire_json(
                            crate::form_value_ref::FormValueRef::from(input.as_str()),
                            crate::value_format::ValueFormat::Timestamp,
                        )
                        .is_err_and(|error| Some(error.to_string()) == expected)
                    })
                })
        );
    }

    #[test]
    fn test_timestamp_filter_missing_date_separators_preserve_invalid_diagnostic() {
        assert!([4usize, 7usize, 10usize].into_iter().all(|position| {
            let input = constants_str::VALUE_2026_07_13T12_30_00
                .chars()
                .enumerate()
                .filter_map(|(index, character)| (index != position).then_some(character))
                .collect::<String>();
            super::parse_timestamp_filter_wire_json(
                crate::form_value_ref::FormValueRef::from(input.as_str()),
                crate::value_format::ValueFormat::Timestamp,
            )
            .is_err_and(|error| error.to_string() == constants_str::INVALID_FILTER_SPECIFICATION)
        }));
    }

    #[test]
    fn test_timestamp_filter_calendar_and_time_bounds_preserve_invalid_diagnostic() {
        assert!(
            [
                crate::value_format::ValueFormat::Timestamp,
                crate::value_format::ValueFormat::TimestampTz
            ]
            .into_iter()
            .all(|value_format| {
                [
                    (0usize, 'x'),
                    (5usize, '1'),
                    (6usize, '0'),
                    (8usize, '3'),
                    (11usize, '3'),
                    (14usize, '6'),
                    (17usize, '6'),
                ]
                .into_iter()
                .all(|(position, replacement)| {
                    let input = constants_str::VALUE_2026_07_13T12_30_00
                        .chars()
                        .enumerate()
                        .map(|(index, character)| {
                            if index == position {
                                replacement
                            } else {
                                character
                            }
                        })
                        .collect::<String>();
                    super::parse_timestamp_filter_wire_json(
                        crate::form_value_ref::FormValueRef::from(input.as_str()),
                        value_format,
                    )
                    .is_err_and(|error| {
                        error.to_string() == constants_str::INVALID_FILTER_SPECIFICATION
                    })
                })
            })
        );
    }

    #[test]
    fn test_timestamp_filter_century_leap_year_rules_preserve_calendar_semantics() {
        assert!(
            [(['1', '9', '0', '0'], false), (['2', '0', '0', '0'], true)]
                .into_iter()
                .all(|(year, accepted)| {
                    let input = year
                        .into_iter()
                        .chain(
                            constants_str::VALUE_2024_02_29T12_30_00
                                .chars()
                                .skip(4usize),
                        )
                        .collect::<String>();
                    let result = super::parse_timestamp_filter_wire_json(
                        crate::form_value_ref::FormValueRef::from(input.as_str()),
                        crate::value_format::ValueFormat::Timestamp,
                    );
                    if accepted {
                        result.is_ok()
                    } else {
                        result.is_err_and(|error| {
                            error.to_string() == constants_str::INVALID_FILTER_SPECIFICATION
                        })
                    }
                })
        );
    }

    #[test]
    fn test_timestamp_filter_fraction_padding_and_precision_preserve_wire_values() {
        assert!(
            [
                (vec![], Some(0u64)),
                (vec!['.'], Some(0u64)),
                (vec!['.', '1'], Some(100_000u64)),
                (vec!['.', '1', '2'], Some(120_000u64)),
                (vec!['.', '1', '2', '3'], Some(123_000u64)),
                (vec!['.', '1', '2', '3', '4'], Some(123_400u64)),
                (vec!['.', '1', '2', '3', '4', '5'], Some(123_450u64)),
                (vec!['.', '1', '2', '3', '4', '5', '6'], Some(123_456u64)),
                (vec!['.', '0', '0', '0', '0', '0', '0'], Some(0u64)),
                (vec!['.', '9', '9', '9', '9', '9', '9'], Some(999_999u64)),
                (vec!['.', '1', '2', '3', '4', '5', '6', '7'], None),
                (vec!['.', 'x'], None),
                (vec!['.', '\u{0661}'], None),
                (vec!['.', '1', '.'], None),
            ]
            .into_iter()
            .all(|(suffix, expected_microsecond)| {
                let input = constants_str::VALUE_2026_07_13T12_30_00
                    .chars()
                    .chain(suffix)
                    .collect::<String>();
                [
                    crate::value_format::ValueFormat::Timestamp,
                    crate::value_format::ValueFormat::TimestampTz,
                ]
                .into_iter()
                .all(|value_format| {
                    let result = super::parse_timestamp_filter_wire_json(
                        crate::form_value_ref::FormValueRef::from(input.as_str()),
                        value_format,
                    );
                    if let Some(expected) = expected_microsecond {
                        result.is_ok_and(|wire| {
                            serde_json::from_str::<serde_json::Value>(wire.as_ref()).is_ok_and(
                                |json| {
                                    json.get(constants_str::PG_CRUD_PG_TIME)
                                        .and_then(|time| time.get(constants_str::MICRO))
                                        .and_then(serde_json::Value::as_u64)
                                        == Some(expected)
                                },
                            )
                        })
                    } else {
                        result.is_err_and(|error| {
                            error.to_string() == constants_str::INVALID_FILTER_SPECIFICATION
                        })
                    }
                })
            })
        );
    }

    #[test]
    fn test_timestamp_filter_omitted_seconds_produces_zero_second_and_microsecond() {
        let input = constants_str::VALUE_2026_07_13T12_30_00
            .rsplit_once(':')
            .map(|(prefix, _second)| prefix);
        assert!(input.is_some());
        let Some(prefix) = input else {
            return;
        };
        assert!(
            super::parse_timestamp_filter_wire_json(
                crate::form_value_ref::FormValueRef::from(prefix),
                crate::value_format::ValueFormat::Timestamp
            )
            .is_ok_and(
                |wire| serde_json::from_str::<serde_json::Value>(wire.as_ref()).is_ok_and(|json| [
                    constants_str::SEC,
                    constants_str::MICRO
                ]
                .into_iter()
                .all(|field| json
                    .get(constants_str::PG_CRUD_PG_TIME)
                    .and_then(|time| time.get(field))
                    .and_then(serde_json::Value::as_u64)
                    == Some(0u64)))
            )
        );
    }

    #[test]
    fn test_timestamp_form_value_rejects_impossible_calendar_date() {
        assert!(
            super::parse_timestamp_filter_wire_json(
                crate::form_value_ref::FormValueRef::from(constants_str::VALUE_2026_02_30T12_30_00),
                crate::value_format::ValueFormat::Timestamp,
            )
            .is_err_and(|error| error.to_string() == constants_str::INVALID_FILTER_SPECIFICATION)
        );
    }

    #[test]
    fn test_timestamp_form_value_accepts_leap_day() {
        assert!(
            super::parse_timestamp_filter_wire_json(
                crate::form_value_ref::FormValueRef::from(constants_str::VALUE_2024_02_29T12_30_00),
                crate::value_format::ValueFormat::Timestamp,
            )
            .is_ok_and(|wire| !wire.as_ref().is_empty())
        );
    }

    #[test]
    fn test_timestamp_form_value_uses_generated_wire_shape() {
        let result = super::parse_timestamp_filter_wire_json(
            crate::form_value_ref::FormValueRef::from(constants_str::VALUE_2026_07_13T12_30_00),
            crate::value_format::ValueFormat::Timestamp,
        );
        assert!(result.is_ok_and(|value| {
            serde_json::from_str::<serde_json::Value>(value.as_ref()).is_ok_and(|wire| {
                serde_json::from_str::<serde_json::Value>(constants_str::VALUE_41FE6651)
                    .is_ok_and(|expected| wire == expected)
            })
        }));
    }

    #[test]
    fn test_timestamp_tz_form_value_uses_naive_date_field() {
        let result = super::parse_timestamp_filter_wire_json(
            crate::form_value_ref::FormValueRef::from(constants_str::VALUE_2026_07_13T12_30_00),
            crate::value_format::ValueFormat::TimestampTz,
        );
        assert!(result.is_ok_and(|value| {
            serde_json::from_str::<serde_json::Value>(value.as_ref()).is_ok_and(|wire| {
                wire.get(constants_str::DATE_NAIVE)
                    .and_then(serde_json::Value::as_str)
                    == constants_str::VALUE_2026_07_13T12_30_00
                        .split_once('T')
                        .map(|(date, _time)| date)
            })
        }));
    }
}
