#[test]
fn test_sqlx_entity_identifier_conversion_preserves_decode_source() {
    let source = server_admin_core::admin_entity_id_try_from_i64_error::AdminEntityIdTryFromI64Error::Invalid;
    let error = crate::sqlx_admin_error::SqlxAdminError::from(source);
    assert!(
        matches!(error.get_inner(), sqlx::Error::Decode(observed) if observed.downcast_ref::<server_admin_core::admin_entity_id_try_from_i64_error::AdminEntityIdTryFromI64Error>() == Some(&source))
    );
}

#[test]
fn test_sqlx_contract_identifier_conversion_preserves_decode_source() {
    let source =
        server_admin_contract::admin_id_try_from_i64_error::AdminIdTryFromI64Error::Invalid;
    let error = crate::sqlx_admin_error::SqlxAdminError::from(source);
    assert!(
        matches!(error.get_inner(), sqlx::Error::Decode(observed) if observed.downcast_ref::<server_admin_contract::admin_id_try_from_i64_error::AdminIdTryFromI64Error>() == Some(&source))
    );
}
