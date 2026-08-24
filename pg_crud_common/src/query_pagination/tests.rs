#[test]
fn pagination_reports_start_and_end() {
    let pagination = super::PaginationStartsWithZero::try_new(20i32, 5i32)
        .expect("5e74c1a9 pagination_reports_start_and_end invariant must hold");
    assert_eq!(pagination.start().get(), 5i64);
    assert_eq!(pagination.end().get(), 25i64);
}

#[test]
fn pagination_rejects_invalid_bounds() {
    assert!(matches!(
        super::PaginationStartsWithZero::try_new(i32_constants::ZERO, i32_constants::ZERO),
        Err(super::PaginationStartsWithZeroTryNewError::LimitIsLessThanOrEqToZero { .. })
    ));
    assert!(matches!(
        super::PaginationStartsWithZero::try_new(1i32, -1i32),
        Err(super::PaginationStartsWithZeroTryNewError::OffsetIsLessThanZero { .. })
    ));
}
