#[test]
fn pagination_reports_start_and_end() {
    let pagination = crate::domain_types::PaginationStartsWithZero::try_new(20i32, 5i32)
        .expect("5e74c1a9 pagination_reports_start_and_end invariant must hold");
    assert_eq!(pagination.start().get(), 5i64);
    assert_eq!(pagination.end().get(), 25i64);
}

#[test]
fn pagination_rejects_invalid_bounds() {
    assert!(matches!(
        crate::domain_types::PaginationStartsWithZero::try_new(
            constants_i32::ZERO,
            constants_i32::ZERO,
        ),
        Err(
            crate::domain_types::PaginationStartsWithZeroTryNewError::LimitIsLessThanOrEqToZero { .. }
        )
    ));
    assert!(matches!(
        crate::domain_types::PaginationStartsWithZero::try_new(1i32, -1i32),
        Err(crate::domain_types::PaginationStartsWithZeroTryNewError::OffsetIsLessThanZero { .. })
    ));
}
