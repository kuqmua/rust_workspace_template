#[test]
fn test_pagination_reports_start_and_end() {
    let pagination =
        crate::pagination_starts_with_zero::PaginationStartsWithZero::try_new(20i32, 5i32)
            .expect(constants_str::DIAGNOSTIC_5E74C1A9);
    assert_eq!(pagination.start().get(), 5i64);
    assert_eq!(pagination.end().get(), 25i64);
}

#[test]
fn test_pagination_rejects_invalid_bounds() {
    assert!(matches!(
        crate::pagination_starts_with_zero::PaginationStartsWithZero::try_new(
            constants_i32::ZERO,
            constants_i32::ZERO,
        ),
        Err(
            crate::pagination_starts_with_zero_try_new_error::PaginationStartsWithZeroTryNewError::LimitIsLessThanOrEqToZero { .. }
        )
    ));
    assert!(matches!(
        crate::pagination_starts_with_zero::PaginationStartsWithZero::try_new(1i32, -1i32),
        Err(crate::pagination_starts_with_zero_try_new_error::PaginationStartsWithZeroTryNewError::OffsetIsLessThanZero { .. })
    ));
}

#[test]
fn test_pagination_accepts_exact_integer_sum_boundary_and_json_round_trip() {
    assert!([(i64::MAX, 0i64), (1i64, i64::MAX - 1)]
        .into_iter()
        .all(|(limit, offset)| {
            let pagination = crate::pagination_starts_with_zero::PaginationStartsWithZero::try_new(
                crate::pagination_limit::PaginationLimit::from(limit),
                crate::pagination_offset::PaginationOffset::from(offset),
            );
            matches!(pagination, Ok(value) if {
                let json = serde_json::json!({(stringify!(limit)): limit, (stringify!(offset)): offset});
                value.start().get() == offset
                    && value.end().get() == i64::MAX
                    && matches!(serde_json::to_value(value), Ok(serialized) if serialized == json)
                    && matches!(serde_json::from_value::<crate::pagination_starts_with_zero::PaginationStartsWithZero>(json), Ok(decoded) if decoded == value)
            })
        }));
}

#[test]
fn test_pagination_error_priority_and_json_preserve_integer_validation() {
    assert!([
        (0i64, -1i64),
        (i64::MIN, -1i64),
        (1i64, i64::MIN),
        (i64::MAX, 1i64),
        (1i64, i64::MAX),
    ]
    .into_iter()
    .all(|(limit, offset)| {
        let pagination_limit = crate::pagination_limit::PaginationLimit::from(limit);
        let pagination_offset = crate::pagination_offset::PaginationOffset::from(offset);
        let error_matches = match crate::pagination_starts_with_zero::PaginationStartsWithZero::try_new(
            pagination_limit,
            pagination_offset,
        ) {
            Err(crate::pagination_starts_with_zero_try_new_error::PaginationStartsWithZeroTryNewError::LimitIsLessThanOrEqToZero { limit: rejected_limit, .. }) => {
                limit <= 0 && rejected_limit == pagination_limit
            }
            Err(crate::pagination_starts_with_zero_try_new_error::PaginationStartsWithZeroTryNewError::OffsetIsLessThanZero { offset: rejected_offset, .. }) => {
                limit > 0 && offset < 0 && rejected_offset == pagination_offset
            }
            Err(crate::pagination_starts_with_zero_try_new_error::PaginationStartsWithZeroTryNewError::OffsetPlusLimitIsIntOverflow { limit: rejected_limit, offset: rejected_offset, .. }) => {
                limit > 0 && offset >= 0 && rejected_limit == pagination_limit && rejected_offset == pagination_offset
            }
            Ok(_) => false,
        };
        error_matches && serde_json::from_value::<crate::pagination_starts_with_zero::PaginationStartsWithZero>(
            serde_json::json!({(stringify!(limit)): limit, (stringify!(offset)): offset}),
        ).is_err()
    }));
}
