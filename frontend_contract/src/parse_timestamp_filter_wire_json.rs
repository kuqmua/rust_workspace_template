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
