#[test]
fn unsigned_database_value_rejects_negative_input() {
    assert!(matches!(
        super::UnsignedPartOfI32::try_from(-1i32),
        Err(super::UnsignedPartOfI32TryFromI32Error::LessThanZero { .. })
    ));
    assert_eq!(
        super::UnsignedPartOfI32::try_from(7i32).expect("ea8c2d71"),
        super::UnsignedPartOfI32::from(7u16)
    );
}

#[test]
fn nonzero_database_value_rejects_zero() {
    assert!(matches!(
        super::NotZeroUnsignedPartOfI32::try_from(0i32),
        Err(super::NotZeroUnsignedPartOfI32TryFromI32Error::IsZero { .. })
    ));
    assert!(matches!(
        super::NotZeroUnsignedPartOfI32::try_from(1i32),
        Ok(_value)
    ));
}
