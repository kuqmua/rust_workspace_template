pub(crate) async fn enrich_assignment_rows<Row, Identifier, Enriched, Error, BuildRow>(
    list_items: pg_crud_common::list_items::ListItems<Row>,
    list_items_primary_keys: pg_crud_common::list_items::ListItems<<pg_types_numeric::generate_pg_types_mod::I64AsNonNullBigSerialInitializationByPg as pg_crud_common::pg_type::PgType>::Read>,
    sqlx_pg_pool_ref: app_state::sqlx_pg_pool_ref::SqlxPgPoolRef<'_>,
    std_admin_str_ref: server_admin_core::std_admin_str_ref::StdAdminStrRef<'static>,
    mut build_row: BuildRow,
) -> Result<pg_crud_common::list_items::ListItems<Enriched>, Error>
where
    Identifier: TryFrom<
            i64,
            Error = server_admin_contract::admin_id_try_from_i64_error::AdminIdTryFromI64Error,
        >,
    Error: From<crate::sqlx_admin_error::SqlxAdminError>
        + From<server_admin_contract::admin_id_try_from_i64_error::AdminIdTryFromI64Error>,
    BuildRow:
        FnMut(Row, pg_crud_common::list_items::ListItems<Identifier>) -> Result<Enriched, Error>,
{
    let identifiers = Vec::from(list_items_primary_keys).into_iter().map(<pg_types_numeric::generate_pg_types_mod::I64AsNonNullBigSerialInitializationByPg as pg_crud_common::pg_type::PgType>::into_inner).collect::<Vec<_>>();
    let links = sqlx::query_as::<_, (i64, i64)>(std_admin_str_ref.get())
        .bind(identifiers.as_slice())
        .fetch_all(sqlx_pg_pool_ref.as_ref())
        .await
        .map_err(crate::sqlx_admin_error::SqlxAdminError::from)?;
    let mut assignments = links.into_iter().try_fold(
        std::collections::BTreeMap::new(),
        |mut values, (owner, identifier)| {
            values
                .entry(owner)
                .or_insert_with(Vec::new)
                .push(Identifier::try_from(identifier)?);
            Ok::<_, Error>(values)
        },
    )?;
    Vec::from(list_items)
        .into_iter()
        .zip(identifiers)
        .map(|(row, identifier)| {
            build_row(
                row,
                pg_crud_common::list_items::ListItems::from(
                    assignments.remove(&identifier).unwrap_or_default(),
                ),
            )
        })
        .collect::<Result<Vec<_>, Error>>()
        .map(pg_crud_common::list_items::ListItems::from)
}
