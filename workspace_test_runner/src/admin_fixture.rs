const ADMIN_FIXTURE_STRING_MAX_LEN: usize = 1_048_576usize;
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::BoundedString)]
#[bounded_string(max = ADMIN_FIXTURE_STRING_MAX_LEN)]
struct AdminFixtureString(String);
fn admin_fixture_string<Value>(value: impl TryInto<AdminFixtureString>) -> Result<Value, ()>
where
    Value: TryFrom<String>,
    Value::Error: std::fmt::Display,
{
    let bounded_value = value.try_into().map_err(|_error| {
        eprintln!(
            "{}",
            str_constants::WORKSPACE_TEST_RUNNER_ADMIN_FIXTURE_STRING_INVALID
        );
    })?;
    Value::try_from(bounded_value.0).map_err(|error| {
        eprintln!("{error}");
    })
}
#[allow(
    clippy::single_call_fn,
    reason = "the command-mode facade keeps fixture generation out of main dispatch"
)]
pub(super) fn write_admin_contract_fixture() -> Result<(), ()> {
    let no_body_schema = serde_json::to_value(
        <server_admin_contract::AdminNoBody as utoipa::PartialSchema>::schema(),
    )
    .map_err(|error| eprintln!("{error}"))?;
    let routes = <server_admin_contract::AdminAuthenticationRouteFamily as frontend_contract::RouteFamily>::schema_contracts()
        .as_ref()
        .iter()
        .map(|contract| {
            let metadata = contract.metadata();
            let request_schema = contract
                .request_schema()
                .cloned()
                .map(|schema| {
                    let openapi_schema: utoipa::openapi::RefOr<utoipa::openapi::Schema> = schema.into();
                    serde_json::to_value(openapi_schema)
                })
                .transpose()
                .map_err(|error| eprintln!("{error}"))?
                .filter(|schema| schema != &no_body_schema);
            let response_schema = contract
                .response_schema()
                .cloned()
                .map(|schema| {
                    let openapi_schema: utoipa::openapi::RefOr<utoipa::openapi::Schema> = schema.into();
                    serde_json::to_value(openapi_schema)
                })
                .transpose()
                .map_err(|error| eprintln!("{error}"))?
                .filter(|_schema| metadata.success_status() != frontend_contract::SuccessStatus::Code204);
            Ok(serde_json::json!([
                metadata.openapi_operation_id().as_ref(),
                metadata.method().as_ref(),
                metadata.path().as_ref(),
                u16::from(metadata.success_status().transport_status()),
                request_schema,
                response_schema,
            ]))
        })
        .collect::<Result<Vec<_>, ()>>()?;
    let permissions = server_admin_contract::AdminPermission::ALL
        .into_iter()
        .map(|permission| serde_json::Value::String(permission.as_str().as_ref().to_owned()))
        .collect::<Vec<_>>();
    let permission_values = server_admin_contract::AdminPermission::ALL
        .into_iter()
        .map(|permission| {
            admin_fixture_string::<server_admin_contract::AdminPermissionValue>(
                permission.as_str().as_ref().to_owned(),
            )
        })
        .collect::<Result<Vec<_>, ()>>()?;
    let authenticated_admin = server_admin_contract::AuthenticatedAdmin::new(
        admin_fixture_string::<server_admin_contract::AdminDisplayName>(String::from(
            str_constants::ADMIN,
        ))?,
        server_admin_contract::AdminUserId::try_from(1i64).map_err(|error| eprintln!("{error}"))?,
        admin_fixture_string::<server_admin_contract::AdminLogin>(String::from(
            str_constants::ROOT,
        ))?,
        server_admin_contract::AdminPermissionValues::try_from(permission_values.clone())
            .map_err(|error| eprintln!("{error}"))?,
        server_admin_contract::AdminRoleNames::try_from(vec![admin_fixture_string::<
            server_admin_contract::AdminRoleName,
        >(String::from(
            str_constants::ADMIN_FIXTURE_ROLE_NAME,
        ))?])
        .map_err(|error| eprintln!("{error}"))?,
    );
    let users = (0i64..25i64)
        .map(|index| {
            let number = index.checked_add(1i64).ok_or_else(|| {
                eprintln!("administrator fixture user identifier overflow");
            })?;
            let is_alpha = index == 24i64;
            let role_id = server_admin_contract::AdminRoleId::try_from(1i64)
                .map_err(|error| eprintln!("{error}"))?;
            Ok(server_admin_contract::AdminUserSummary::new(
                admin_fixture_string::<server_admin_contract::AdminDisplayName>(if is_alpha {
                    String::from(str_constants::ADMIN_FIXTURE_ALPHA_DISPLAY_NAME)
                } else {
                    format!("User {number:02}")
                })?,
                server_admin_contract::AdminUserId::try_from(number)
                    .map_err(|error| eprintln!("{error}"))?,
                server_admin_contract::AdminBool::from(index & 1i64 == 0i64),
                admin_fixture_string::<server_admin_contract::AdminLogin>(if is_alpha {
                    String::from(str_constants::ADMIN_FIXTURE_ALPHA_LOGIN)
                } else {
                    format!("user_{number:02}")
                })?,
                server_admin_contract::AdminRoleIds::try_from(vec![role_id])
                    .map_err(|error| eprintln!("{error}"))?,
            ))
        })
        .collect::<Result<Vec<_>, ()>>()?;
    let permission_summaries = permission_values
        .into_iter()
        .enumerate()
        .map(|(index, permission)| {
            let value = i64::try_from(index).map_err(|error| {
                eprintln!("{error}");
            })?;
            let identifier = value.checked_add(1i64).ok_or_else(|| {
                eprintln!("administrator fixture permission identifier overflow");
            })?;
            Ok(server_admin_contract::AdminPermissionSummary::new(
                server_admin_contract::AdminPermissionId::try_from(identifier)
                    .map_err(|error| eprintln!("{error}"))?,
                permission,
            ))
        })
        .collect::<Result<Vec<_>, ()>>()?;
    let role_summary = server_admin_contract::AdminRoleSummary::new(
        server_admin_contract::AdminRoleId::try_from(1i64).map_err(|error| eprintln!("{error}"))?,
        server_admin_contract::AdminBool::from(false),
        admin_fixture_string::<server_admin_contract::AdminRoleName>(String::from(
            str_constants::ADMIN_FIXTURE_ROLE_NAME,
        ))?,
        server_admin_contract::AdminPermissionIds::try_from(
            permission_summaries
                .iter()
                .map(server_admin_contract::AdminPermissionSummary::id)
                .collect::<Vec<_>>(),
        )
        .map_err(|error| eprintln!("{error}"))?,
    );
    let role_summaries = vec![role_summary];
    let audit_details =
        server_admin_contract::SerdeJsonAdminAuditDetails::try_from(serde_json::json!({
            str_constants::FIELD: str_constants::DISPLAY_NAME
        }))
        .map_err(|error| eprintln!("{error}"))?;
    let audit_log_id = server_admin_contract::AdminAuditLogId::try_from(1i64)
        .map_err(|error| eprintln!("{error}"))?;
    let audit_user_id = server_admin_contract::AdminUserId::try_from(25i64)
        .map_err(|error| eprintln!("{error}"))?;
    let audit = vec![server_admin_contract::AdminAuditView::new(
        admin_fixture_string::<server_admin_contract::AdminText>(String::from(
            str_constants::ADMIN_FIXTURE_AUDIT_ACTION,
        ))?,
        admin_fixture_string::<server_admin_contract::AdminAuditTimestamp>(String::from(
            str_constants::ADMIN_FIXTURE_AUDIT_CREATED_AT,
        ))?,
        Some(audit_details),
        audit_log_id,
        admin_fixture_string::<server_admin_contract::AdminText>(String::from(
            str_constants::ADMIN_FIXTURE_AUDIT_RESOURCE,
        ))?,
        Some(admin_fixture_string::<server_admin_contract::AdminText>(
            String::from(str_constants::ADMIN_FIXTURE_AUDIT_RESOURCE_ID),
        )?),
        server_admin_contract::AdminBool::from(true),
        Some(audit_user_id),
        Some(admin_fixture_string::<server_admin_contract::AdminLogin>(
            String::from(str_constants::ADMIN_FIXTURE_ALPHA_LOGIN),
        )?),
    )];
    let sessions = vec![
        server_admin_contract::AdminSessionView::new(
            admin_fixture_string::<server_admin_contract::AdminSessionTimestamp>(String::from(
                str_constants::ADMIN_FIXTURE_SESSION_CREATED_AT,
            ))?,
            admin_fixture_string::<server_admin_contract::AdminSessionTimestamp>(String::from(
                str_constants::ADMIN_FIXTURE_SESSION_EXPIRES_AT,
            ))?,
            admin_fixture_string::<server_admin_contract::AdminSessionIdentifier>(String::from(
                str_constants::ADMIN_FIXTURE_SESSION_ID,
            ))?,
            server_admin_contract::AdminBool::from(true),
        ),
        server_admin_contract::AdminSessionView::new(
            admin_fixture_string::<server_admin_contract::AdminSessionTimestamp>(String::from(
                str_constants::ADMIN_FIXTURE_SESSION_CREATED_AT,
            ))?,
            admin_fixture_string::<server_admin_contract::AdminSessionTimestamp>(String::from(
                str_constants::ADMIN_FIXTURE_SESSION_EXPIRES_AT,
            ))?,
            admin_fixture_string::<server_admin_contract::AdminSessionIdentifier>(String::from(
                str_constants::ADMIN_FIXTURE_SECOND_SESSION_ID,
            ))?,
            server_admin_contract::AdminBool::from(false),
        ),
    ];
    let authenticated_admin_json = serde_json::to_value(&authenticated_admin).map_err(|error| {
        eprintln!("{error}");
    })?;
    let user_total = u64::try_from(users.len()).map_err(|error| eprintln!("{error}"))?;
    let users_page = server_admin_contract::AdminUsersPage::new(
        server_admin_contract::AdminUserSummaries::try_from(users)
            .map_err(|error| eprintln!("{error}"))?,
        server_admin_contract::AdminRoleSummaries::try_from(role_summaries.clone())
            .map_err(|error| eprintln!("{error}"))?,
        server_admin_contract::AdminPageTotal::from(user_total),
    );
    let role_total = u64::try_from(role_summaries.len()).map_err(|error| eprintln!("{error}"))?;
    let roles_page = server_admin_contract::AdminRolesPage::new(
        server_admin_contract::AdminRoleSummaries::try_from(role_summaries)
            .map_err(|error| eprintln!("{error}"))?,
        server_admin_contract::AdminPermissionSummaries::try_from(permission_summaries.clone())
            .map_err(|error| eprintln!("{error}"))?,
        server_admin_contract::AdminPageTotal::from(role_total),
    );
    let permission_total =
        u64::try_from(permission_summaries.len()).map_err(|error| eprintln!("{error}"))?;
    let permissions_page = server_admin_contract::AdminPermissionsPage::new(
        server_admin_contract::AdminPermissionSummaries::try_from(permission_summaries)
            .map_err(|error| eprintln!("{error}"))?,
        server_admin_contract::AdminPageTotal::from(permission_total),
    );
    let users_json = serde_json::to_value(&users_page).map_err(|error| {
        eprintln!("{error}");
    })?;
    let role_summaries_json = serde_json::to_value(&roles_page).map_err(|error| {
        eprintln!("{error}");
    })?;
    let permission_summaries_json = serde_json::to_value(&permissions_page).map_err(|error| {
        eprintln!("{error}");
    })?;
    let audit_cursor = server_admin_contract::AdminAuditCursor::new(
        admin_fixture_string::<server_admin_contract::AdminAuditTimestamp>(String::from(
            str_constants::ADMIN_FIXTURE_AUDIT_CREATED_AT,
        ))?,
        server_admin_contract::AdminAuditLogId::try_from(1i64)
            .map_err(|error| eprintln!("{error}"))?,
    );
    let audit_page = server_admin_contract::AdminAuditPage::new(
        server_admin_contract::AdminAuditViews::try_from(audit)
            .map_err(|error| eprintln!("{error}"))?,
        Some(audit_cursor),
        server_admin_contract::AdminPageTotal::from(1u64),
    );
    let audit_json = serde_json::to_value(&audit_page).map_err(|error| {
        eprintln!("{error}");
    })?;
    let sessions_json = serde_json::to_value(&sessions).map_err(|error| {
        eprintln!("{error}");
    })?;
    let no_body_json =
        serde_json::to_value(server_admin_contract::AdminNoBody).map_err(|error| {
            eprintln!("{error}");
        })?;
    let open_api_json = serde_json::to_value(utoipa::openapi::OpenApi::from(
        server_admin::generated_tables::generated_open_api(),
    ))
    .map_err(|error| {
        eprintln!("{error}");
    })?;
    let fixture = serde_json::to_vec_pretty(&serde_json::json!([
        routes,
        permissions,
        authenticated_admin_json,
        users_json,
        role_summaries_json,
        permission_summaries_json,
        audit_json,
        sessions_json,
        no_body_json,
        <server_admin_contract::AdminAuthenticationRouteFamily as frontend_contract::RouteFamily>::body_limit()
            .map(frontend_contract::RouteBodyLimit::get),
        open_api_json,
    ]))
    .map_err(|error| {
        eprintln!("{error}");
    })?;
    let Some(workspace_root) = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).parent() else {
        return Err(());
    };
    let target = workspace_root.join(str_constants::TARGET);
    std::fs::create_dir_all(target.as_path()).map_err(|error| {
        eprintln!("{error}");
    })?;
    std::fs::write(
        target.join(str_constants::WORKSPACE_TEST_RUNNER_ADMIN_CONTRACT_FIXTURE_FILE),
        fixture,
    )
    .map_err(|error| {
        eprintln!("{error}");
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn fixture_text_enforces_the_owned_bound() {
        let oversized =
            String::from("a").repeat(super::ADMIN_FIXTURE_STRING_MAX_LEN.saturating_add(1usize));
        assert!(matches!(
            super::AdminFixtureString::try_from(oversized),
            Err(super::AdminFixtureStringTryFromStringError::TooLong { .. })
        ));
    }
}
