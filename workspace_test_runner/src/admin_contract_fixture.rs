// The owner module retains lint-sensitive semantics from the original implementation.
#[allow(clippy::single_call_fn)] // the mode dispatcher owns the single fixture-generation entry point
pub(crate) fn admin_contract_fixture() -> Result<(), ()> {
    (|| {
        let no_body_schema = serde_json::to_value(
            <server_admin_contract::domain_types::AdminNoBody as utoipa::PartialSchema>::schema(),
        )
        .map_err(|error| eprintln!("{error}"))?;
        let routes = <server_admin_contract::domain_types::AdminAuthenticationRouteFamily as frontend_contract::domain_types::RouteFamily>::schema_contracts()
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
                                    .filter(|_schema| metadata.success_status() != frontend_contract::domain_types::SuccessStatus::Code204);
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
        let permissions = server_admin_contract::domain_types::AdminPermission::ALL
            .into_iter()
            .map(|permission| serde_json::Value::String(permission.as_str().as_ref().to_owned()))
            .collect::<Vec<_>>();
        let permission_values = server_admin_contract::domain_types::AdminPermission::ALL
            .into_iter()
            .map(|permission| {
                crate::admin_fixture::create_admin_fixture_string::<
                    server_admin_contract::domain_types::AdminPermissionValue,
                >(permission.as_str().as_ref().to_owned())
            })
            .collect::<Result<Vec<_>, ()>>()?;
        let authenticated_admin = server_admin_contract::domain_types::AuthenticatedAdmin::new(
            crate::admin_fixture::create_admin_fixture_string::<
                server_admin_contract::domain_types::AdminDisplayName,
            >(String::from(constants_str::ADMIN))?,
            server_admin_contract::domain_types::AdminUserId::try_from(constants_i64::ONE)
                .map_err(|error| eprintln!("{error}"))?,
            crate::admin_fixture::create_admin_fixture_string::<
                server_admin_contract::domain_types::AdminLogin,
            >(String::from(constants_str::ROOT))?,
            server_admin_contract::domain_types::AdminPermissionValues::try_from(
                permission_values.clone(),
            )
            .map_err(|error| eprintln!("{error}"))?,
            server_admin_contract::domain_types::AdminRoleNames::try_from(vec![
                crate::admin_fixture::create_admin_fixture_string::<
                    server_admin_contract::domain_types::AdminRoleName,
                >(String::from(constants_str::ADMIN_FIXTURE_ROLE_NAME))?,
            ])
            .map_err(|error| eprintln!("{error}"))?,
        );
        let users = (constants_i64::ZERO..25i64)
            .map(|index| {
                let number = index.checked_add(constants_i64::ONE).ok_or_else(|| {
                    eprintln!("administrator fixture user identifier overflow");
                })?;
                let is_alpha = index == 24i64;
                let role_id =
                    server_admin_contract::domain_types::AdminRoleId::try_from(constants_i64::ONE)
                        .map_err(|error| eprintln!("{error}"))?;
                Ok(server_admin_contract::domain_types::AdminUserSummary::new(
                    crate::admin_fixture::create_admin_fixture_string::<
                        server_admin_contract::domain_types::AdminDisplayName,
                    >(if is_alpha {
                        String::from(constants_str::ADMIN_FIXTURE_ALPHA_DISPLAY_NAME)
                    } else {
                        format!("User {number:02}")
                    })?,
                    server_admin_contract::domain_types::AdminUserId::try_from(number)
                        .map_err(|error| eprintln!("{error}"))?,
                    server_admin_contract::domain_types::AdminBool::from(
                        index & constants_i64::ONE == constants_i64::ZERO,
                    ),
                    crate::admin_fixture::create_admin_fixture_string::<
                        server_admin_contract::domain_types::AdminLogin,
                    >(if is_alpha {
                        String::from(constants_str::ADMIN_FIXTURE_ALPHA_LOGIN)
                    } else {
                        format!("user_{number:02}")
                    })?,
                    server_admin_contract::domain_types::AdminRoleIds::try_from(vec![role_id])
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
                let identifier = value.checked_add(constants_i64::ONE).ok_or_else(|| {
                    eprintln!("administrator fixture permission identifier overflow");
                })?;
                Ok(
                    server_admin_contract::domain_types::AdminPermissionSummary::new(
                        server_admin_contract::domain_types::AdminPermissionId::try_from(
                            identifier,
                        )
                        .map_err(|error| eprintln!("{error}"))?,
                        permission,
                    ),
                )
            })
            .collect::<Result<Vec<_>, ()>>()?;
        let role_summary = server_admin_contract::domain_types::AdminRoleSummary::new(
            server_admin_contract::domain_types::AdminRoleId::try_from(constants_i64::ONE)
                .map_err(|error| eprintln!("{error}"))?,
            server_admin_contract::domain_types::AdminBool::from(false),
            crate::admin_fixture::create_admin_fixture_string::<
                server_admin_contract::domain_types::AdminRoleName,
            >(String::from(constants_str::ADMIN_FIXTURE_ROLE_NAME))?,
            server_admin_contract::domain_types::AdminPermissionIds::try_from(
                permission_summaries
                    .iter()
                    .map(server_admin_contract::domain_types::AdminPermissionSummary::id)
                    .collect::<Vec<_>>(),
            )
            .map_err(|error| eprintln!("{error}"))?,
        );
        let role_summaries = vec![role_summary];
        let audit_details =
            server_admin_contract::domain_types::SerdeJsonAdminAuditDetails::try_from(
                serde_json::json!({
                    constants_str::FIELD: constants_str::DISPLAY_NAME
                }),
            )
            .map_err(|error| eprintln!("{error}"))?;
        let audit_log_id =
            server_admin_contract::domain_types::AdminAuditLogId::try_from(constants_i64::ONE)
                .map_err(|error| eprintln!("{error}"))?;
        let audit_user_id = server_admin_contract::domain_types::AdminUserId::try_from(25i64)
            .map_err(|error| eprintln!("{error}"))?;
        let audit = vec![server_admin_contract::domain_types::AdminAuditView::new(
            crate::admin_fixture::create_admin_fixture_string::<
                server_admin_contract::domain_types::AdminText,
            >(String::from(constants_str::ADMIN_FIXTURE_AUDIT_ACTION))?,
            crate::admin_fixture::create_admin_fixture_string::<
                server_admin_contract::domain_types::AdminAuditTimestamp,
            >(String::from(constants_str::ADMIN_FIXTURE_AUDIT_CREATED_AT))?,
            Some(audit_details),
            audit_log_id,
            crate::admin_fixture::create_admin_fixture_string::<
                server_admin_contract::domain_types::AdminText,
            >(String::from(constants_str::ADMIN_FIXTURE_AUDIT_RESOURCE))?,
            Some(crate::admin_fixture::create_admin_fixture_string::<
                server_admin_contract::domain_types::AdminText,
            >(String::from(
                constants_str::ADMIN_FIXTURE_AUDIT_RESOURCE_ID,
            ))?),
            server_admin_contract::domain_types::AdminBool::from(true),
            Some(audit_user_id),
            Some(crate::admin_fixture::create_admin_fixture_string::<
                server_admin_contract::domain_types::AdminLogin,
            >(String::from(
                constants_str::ADMIN_FIXTURE_ALPHA_LOGIN,
            ))?),
        )];
        let sessions = vec![
            server_admin_contract::domain_types::AdminSessionView::new(
                crate::admin_fixture::create_admin_fixture_string::<
                    server_admin_contract::domain_types::AdminSessionTimestamp,
                >(String::from(
                    constants_str::ADMIN_FIXTURE_SESSION_CREATED_AT,
                ))?,
                crate::admin_fixture::create_admin_fixture_string::<
                    server_admin_contract::domain_types::AdminSessionTimestamp,
                >(String::from(
                    constants_str::ADMIN_FIXTURE_SESSION_EXPIRES_AT,
                ))?,
                crate::admin_fixture::create_admin_fixture_string::<
                    server_admin_contract::domain_types::AdminSessionIdentifier,
                >(String::from(constants_str::ADMIN_FIXTURE_SESSION_ID))?,
                server_admin_contract::domain_types::AdminBool::from(true),
            ),
            server_admin_contract::domain_types::AdminSessionView::new(
                crate::admin_fixture::create_admin_fixture_string::<
                    server_admin_contract::domain_types::AdminSessionTimestamp,
                >(String::from(
                    constants_str::ADMIN_FIXTURE_SESSION_CREATED_AT,
                ))?,
                crate::admin_fixture::create_admin_fixture_string::<
                    server_admin_contract::domain_types::AdminSessionTimestamp,
                >(String::from(
                    constants_str::ADMIN_FIXTURE_SESSION_EXPIRES_AT,
                ))?,
                crate::admin_fixture::create_admin_fixture_string::<
                    server_admin_contract::domain_types::AdminSessionIdentifier,
                >(String::from(constants_str::ADMIN_FIXTURE_SECOND_SESSION_ID))?,
                server_admin_contract::domain_types::AdminBool::from(false),
            ),
        ];
        let authenticated_admin_json =
            serde_json::to_value(&authenticated_admin).map_err(|error| {
                eprintln!("{error}");
            })?;
        let user_total = u64::try_from(users.len()).map_err(|error| eprintln!("{error}"))?;
        let users_page = server_admin_contract::domain_types::AdminUsersPage::new(
            server_admin_contract::domain_types::AdminUserSummaries::try_from(users)
                .map_err(|error| eprintln!("{error}"))?,
            server_admin_contract::domain_types::AdminRoleSummaries::try_from(
                role_summaries.clone(),
            )
            .map_err(|error| eprintln!("{error}"))?,
            server_admin_contract::domain_types::AdminPageTotal::from(user_total),
        );
        let role_total =
            u64::try_from(role_summaries.len()).map_err(|error| eprintln!("{error}"))?;
        let roles_page = server_admin_contract::domain_types::AdminRolesPage::new(
            server_admin_contract::domain_types::AdminRoleSummaries::try_from(role_summaries)
                .map_err(|error| eprintln!("{error}"))?,
            server_admin_contract::domain_types::AdminPermissionSummaries::try_from(
                permission_summaries.clone(),
            )
            .map_err(|error| eprintln!("{error}"))?,
            server_admin_contract::domain_types::AdminPageTotal::from(role_total),
        );
        let permission_total =
            u64::try_from(permission_summaries.len()).map_err(|error| eprintln!("{error}"))?;
        let permissions_page = server_admin_contract::domain_types::AdminPermissionsPage::new(
            server_admin_contract::domain_types::AdminPermissionSummaries::try_from(
                permission_summaries,
            )
            .map_err(|error| eprintln!("{error}"))?,
            server_admin_contract::domain_types::AdminPageTotal::from(permission_total),
        );
        let users_json = serde_json::to_value(&users_page).map_err(|error| {
            eprintln!("{error}");
        })?;
        let role_summaries_json = serde_json::to_value(&roles_page).map_err(|error| {
            eprintln!("{error}");
        })?;
        let permission_summaries_json =
            serde_json::to_value(&permissions_page).map_err(|error| {
                eprintln!("{error}");
            })?;
        let audit_cursor = server_admin_contract::domain_types::AdminAuditCursor::new(
            crate::admin_fixture::create_admin_fixture_string::<
                server_admin_contract::domain_types::AdminAuditTimestamp,
            >(String::from(constants_str::ADMIN_FIXTURE_AUDIT_CREATED_AT))?,
            server_admin_contract::domain_types::AdminAuditLogId::try_from(constants_i64::ONE)
                .map_err(|error| eprintln!("{error}"))?,
        );
        let audit_page = server_admin_contract::domain_types::AdminAuditPage::new(
            server_admin_contract::domain_types::AdminAuditViews::try_from(audit)
                .map_err(|error| eprintln!("{error}"))?,
            Some(audit_cursor),
            server_admin_contract::domain_types::AdminPageTotal::from(1u64),
        );
        let audit_json = serde_json::to_value(&audit_page).map_err(|error| {
            eprintln!("{error}");
        })?;
        let sessions_json = serde_json::to_value(&sessions).map_err(|error| {
            eprintln!("{error}");
        })?;
        let no_body_json = serde_json::to_value(server_admin_contract::domain_types::AdminNoBody)
            .map_err(|error| {
            eprintln!("{error}");
        })?;
        let open_api_json = serde_json::to_value(utoipa::openapi::OpenApi::from(
            server_admin::domain_types::generated_tables::generated_open_api(),
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
                            <server_admin_contract::domain_types::AdminAuthenticationRouteFamily as frontend_contract::domain_types::RouteFamily>::body_limit()
                                .map(frontend_contract::domain_types::RouteBodyLimit::get),
                            open_api_json,
                        ]))
                        .map_err(|error| {
                            eprintln!("{error}");
                        })?;
        let Some(workspace_root) = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).parent() else {
            return Err(());
        };
        let target = workspace_root.join(constants_str::TARGET);
        std::fs::create_dir_all(target.as_path()).map_err(|error| {
            eprintln!("{error}");
        })?;
        std::fs::write(
            target.join(constants_str::WORKSPACE_TEST_RUNNER_ADMIN_CONTRACT_FIXTURE_FILE),
            fixture,
        )
        .map_err(|error| {
            eprintln!("{error}");
        })
    })()
}
