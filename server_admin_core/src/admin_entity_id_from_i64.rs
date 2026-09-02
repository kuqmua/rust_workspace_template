pub(crate) fn admin_entity_id_from_i64(
    i64: i64,
) -> Result<
    server_admin_contract::positive_non_zero_i64::PositiveNonZeroI64,
    crate::admin_entity_id_try_from_i64_error::AdminEntityIdTryFromI64Error,
> {
    server_admin_contract::positive_non_zero_i64::PositiveNonZeroI64::try_from(i64).map_err(
        |_error| crate::admin_entity_id_try_from_i64_error::AdminEntityIdTryFromI64Error::Invalid,
    )
}
