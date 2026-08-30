pub(crate) fn admin_entity_id_from_i64(
    value: i64,
) -> Result<
    server_admin_contract::positive_non_zero_i64::PositiveNonZeroI64,
    crate::admin_entity_id_try_from_i64_error::AdminEntityIdTryFromI64Error,
> {
    server_admin_contract::positive_non_zero_i64::PositiveNonZeroI64::try_from(value).map_err(
        |_error| crate::admin_entity_id_try_from_i64_error::AdminEntityIdTryFromI64Error::Invalid,
    )
}
