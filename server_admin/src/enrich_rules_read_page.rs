#[allow(
    clippy::unused_async,
    reason = "the generated read-page enrichment contract requires an asynchronous function"
)]
pub async fn enrich_rules_read_page(
    list_items: pg_crud_common::list_items::ListItems<crate::admin_rules::AdminRulesRead>,
    list_items_primary_keys: pg_crud_common::list_items::ListItems<<pg_types_numeric::generate_pg_types_mod::I64AsNonNullBigSerialInitializationByPg as pg_crud_common::pg_type::PgType>::Read>,
    list_total: pg_crud_common::list_total::ListTotal,
    _sqlx_pg_pool_ref: app_state::sqlx_pg_pool_ref::SqlxPgPoolRef<'_>,
) -> Result<
    server_admin_contract::admin_rules_page::AdminRulesPage,
    crate::admin_rules_read_page_error::AdminRulesReadPageError,
> {
    let items = Vec::from(list_items)
        .into_iter()
        .zip(Vec::from(list_items_primary_keys))
        .map(|(item, id)| {
            let optional_name = item.get_name();
            let name = optional_name.as_ref().ok_or(
                crate::admin_rules_read_page_error::AdminRulesReadPageError::MissingName,
            )?;
            let optional_created_at = item.get_created_at();
            let created_at = optional_created_at.as_ref().ok_or(
                crate::admin_rules_read_page_error::AdminRulesReadPageError::MissingCreatedAt,
            )?;
            Ok(server_admin_contract::admin_rule_summary::AdminRuleSummary::new(
                server_admin_contract::admin_rule_id::AdminRuleId::try_from(
                    <pg_types_numeric::generate_pg_types_mod::I64AsNonNullBigSerialInitializationByPg as pg_crud_common::pg_type::PgType>::into_inner(id),
                )?,
                match server_admin_contract::admin_rule_value::AdminRuleValue::try_from(
                    <pg_types_text_misc::generate_pg_types_mod::StringAsNonNullText as pg_crud_common::pg_type::PgType>::into_inner(name.get_value().clone()),
                ) {
                    Ok(admin_rule_value) => admin_rule_value,
                    Err(_) => return Err(crate::admin_rules_read_page_error::AdminRulesReadPageError::StoredRule),
                },
                server_admin_contract::admin_rule_timestamp::AdminRuleTimestamp::from(
                    server_admin_contract::admin_role_timestamp::AdminRoleTimestamp::try_from(
                        <pg_types_chrono_net::generate_pg_types_mod::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNonNullTimestampTz as pg_crud_common::pg_type::PgType>::into_inner(
                            created_at.get_value().clone(),
                    )
                    .to_string(),
                )
                .map_err(crate::admin_rules_read_page_error::AdminRulesReadPageError::CreatedAt)?,
                ),
            ))
        })
        .collect::<Result<Vec<_>, crate::admin_rules_read_page_error::AdminRulesReadPageError>>()?;
    let Ok(total) = u64::try_from(i64::from(list_total)) else {
        return Err(crate::admin_rules_read_page_error::AdminRulesReadPageError::TotalConversion);
    };
    Ok(
        server_admin_contract::admin_rules_page::AdminRulesPage::new(
            server_admin_contract::admin_rule_summaries::AdminRuleSummaries::try_from(items)?,
            server_admin_contract::admin_page_total::AdminPageTotal::from(total),
        ),
    )
}
