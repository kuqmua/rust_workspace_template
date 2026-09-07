pub(crate) fn run_admin_fixture_cli() -> crate::runner_cli_outcome::RunnerCliOutcome {
    let result = (|| {
        let render_admin_fixture_conversion_error = |admin_fixture_conversion_error: crate::admin_fixture_conversion_error::AdminFixtureConversionError| {
            macro_helpers::tool_console_stream::ToolConsoleStream::write_or_exit(macro_helpers::tool_console_stream::ToolConsoleStream::StandardError, macro_helpers::std_fmt_arguments::StdFmtArguments::from(format_args!("{admin_fixture_conversion_error}{}", constants_str::NEWLINE)));
        };
        let no_body_schema = serde_json::to_value(
            <server_admin_contract::admin_no_body::AdminNoBody as utoipa::PartialSchema>::schema(),
        )
        .map_err(|error| {
            macro_helpers::tool_console_stream::ToolConsoleStream::write_or_exit(
                macro_helpers::tool_console_stream::ToolConsoleStream::StandardError,
                macro_helpers::std_fmt_arguments::StdFmtArguments::from(format_args!(
                    "{}{}",
                    format_args!("{error}"),
                    constants_str::NEWLINE
                )),
            )
        })?;
        let routes = <server_admin_contract::admin_route::AdminAuthenticationRouteFamily as frontend_contract::route_family::RouteFamily>::schema_contracts()
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
                                            .map_err(|error| macro_helpers::tool_console_stream::ToolConsoleStream::write_or_exit(macro_helpers::tool_console_stream::ToolConsoleStream::StandardError, macro_helpers::std_fmt_arguments::StdFmtArguments::from(format_args!("{}{}", format_args!("{error}"), constants_str::NEWLINE))))?
                                            .filter(|schema| schema != &no_body_schema);
                                        let response_schema = contract
                                            .response_schema()
                                            .cloned()
                                            .map(|schema| {
                                                let openapi_schema: utoipa::openapi::RefOr<utoipa::openapi::Schema> = schema.into();
                                                serde_json::to_value(openapi_schema)
                                            })
                                            .transpose()
                                            .map_err(|error| macro_helpers::tool_console_stream::ToolConsoleStream::write_or_exit(macro_helpers::tool_console_stream::ToolConsoleStream::StandardError, macro_helpers::std_fmt_arguments::StdFmtArguments::from(format_args!("{}{}", format_args!("{error}"), constants_str::NEWLINE))))?
                                            .filter(|_schema| metadata.success_status() != frontend_contract::success_status::SuccessStatus::Code204);
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
        let permissions = server_admin_contract::admin_permission::AdminPermission::ALL
            .into_iter()
            .map(|permission| serde_json::Value::String(permission.as_str().as_ref().to_owned()))
            .collect::<Vec<_>>();
        let permission_values = server_admin_contract::admin_permission::AdminPermission::ALL
            .into_iter()
            .map(|permission| {
                crate::create_admin_fixture_string::create_admin_fixture_string::<
                    server_admin_contract::admin_permission_value::AdminPermissionValue,
                >(permission.as_str().as_ref().to_owned())
                .map_err(render_admin_fixture_conversion_error)
            })
            .collect::<Result<Vec<_>, ()>>()?;
        let authenticated_admin =
            server_admin_contract::authenticated_admin::AuthenticatedAdmin::new(
                crate::create_admin_fixture_string::create_admin_fixture_string::<
                    server_admin_contract::admin_display_name::AdminDisplayName,
                >(String::from(constants_str::ADMIN))
                .map_err(render_admin_fixture_conversion_error)?,
                server_admin_contract::admin_user_id::AdminUserId::try_from(constants_i64::ONE)
                    .map_err(|error| {
                        macro_helpers::tool_console_stream::ToolConsoleStream::write_or_exit(
                            macro_helpers::tool_console_stream::ToolConsoleStream::StandardError,
                            macro_helpers::std_fmt_arguments::StdFmtArguments::from(format_args!(
                                "{}{}",
                                format_args!("{error}"),
                                constants_str::NEWLINE
                            )),
                        )
                    })?,
                crate::create_admin_fixture_string::create_admin_fixture_string::<
                    server_admin_contract::admin_login::AdminLogin,
                >(String::from(constants_str::ROOT))
                .map_err(render_admin_fixture_conversion_error)?,
                server_admin_contract::admin_permission_values::AdminPermissionValues::try_from(
                    permission_values.clone(),
                )
                .map_err(|error| {
                    macro_helpers::tool_console_stream::ToolConsoleStream::write_or_exit(
                        macro_helpers::tool_console_stream::ToolConsoleStream::StandardError,
                        macro_helpers::std_fmt_arguments::StdFmtArguments::from(format_args!(
                            "{}{}",
                            format_args!("{error}"),
                            constants_str::NEWLINE
                        )),
                    )
                })?,
                server_admin_contract::admin_role_names::AdminRoleNames::try_from(vec![
                    crate::create_admin_fixture_string::create_admin_fixture_string::<
                        server_admin_contract::admin_role_name::AdminRoleName,
                    >(String::from(constants_str::ADMIN_FIXTURE_ROLE_NAME))
                    .map_err(render_admin_fixture_conversion_error)?,
                ])
                .map_err(|error| {
                    macro_helpers::tool_console_stream::ToolConsoleStream::write_or_exit(
                        macro_helpers::tool_console_stream::ToolConsoleStream::StandardError,
                        macro_helpers::std_fmt_arguments::StdFmtArguments::from(format_args!(
                            "{}{}",
                            format_args!("{error}"),
                            constants_str::NEWLINE
                        )),
                    )
                })?,
            );
        let users = (constants_i64::ZERO..25i64)
            .map(|index| {
                let number = index.checked_add(constants_i64::ONE).ok_or_else(|| {
                    macro_helpers::tool_console_stream::ToolConsoleStream::write_or_exit(macro_helpers::tool_console_stream::ToolConsoleStream::StandardError, macro_helpers::std_fmt_arguments::StdFmtArguments::from(format_args!("{}{}", format_args!("{}", constants_str::RUNNER_CLI_TEXT_995B4527), constants_str::NEWLINE)));
                })?;
                let is_alpha = index == 24i64;
                let role_id = server_admin_contract::admin_role_id::AdminRoleId::try_from(
                    constants_i64::ONE,
                )
                .map_err(|error| macro_helpers::tool_console_stream::ToolConsoleStream::write_or_exit(macro_helpers::tool_console_stream::ToolConsoleStream::StandardError, macro_helpers::std_fmt_arguments::StdFmtArguments::from(format_args!("{}{}", format_args!("{error}"), constants_str::NEWLINE))))?;
                Ok(
                    server_admin_contract::admin_user_summary::AdminUserSummary::new(
                        crate::create_admin_fixture_string::create_admin_fixture_string::<
                            server_admin_contract::admin_display_name::AdminDisplayName,
                        >(if is_alpha {
                            String::from(constants_str::ADMIN_FIXTURE_ALPHA_DISPLAY_NAME)
                        } else {
                            format!("User {number:02}")
                        })
                        .map_err(render_admin_fixture_conversion_error)?,
                        server_admin_contract::admin_user_id::AdminUserId::try_from(number)
                            .map_err(|error| macro_helpers::tool_console_stream::ToolConsoleStream::write_or_exit(macro_helpers::tool_console_stream::ToolConsoleStream::StandardError, macro_helpers::std_fmt_arguments::StdFmtArguments::from(format_args!("{}{}", format_args!("{error}"), constants_str::NEWLINE))))?,
                        server_admin_contract::admin_bool::AdminBool::from(
                            index & constants_i64::ONE == constants_i64::ZERO,
                        ),
                        crate::create_admin_fixture_string::create_admin_fixture_string::<
                            server_admin_contract::admin_login::AdminLogin,
                        >(if is_alpha {
                            String::from(constants_str::ADMIN_FIXTURE_ALPHA_LOGIN)
                        } else {
                            format!("user_{number:02}")
                        })
                        .map_err(render_admin_fixture_conversion_error)?,
                        server_admin_contract::admin_role_ids::AdminRoleIds::try_from(vec![
                            role_id,
                        ])
                        .map_err(|error| macro_helpers::tool_console_stream::ToolConsoleStream::write_or_exit(macro_helpers::tool_console_stream::ToolConsoleStream::StandardError, macro_helpers::std_fmt_arguments::StdFmtArguments::from(format_args!("{}{}", format_args!("{error}"), constants_str::NEWLINE))))?,
                    ),
                )
            })
            .collect::<Result<Vec<_>, ()>>()?;
        let permission_summaries = permission_values
            .into_iter()
            .enumerate()
            .map(|(index, permission)| {
                let value = i64::try_from(index).map_err(|error| {
                    macro_helpers::tool_console_stream::ToolConsoleStream::write_or_exit(macro_helpers::tool_console_stream::ToolConsoleStream::StandardError, macro_helpers::std_fmt_arguments::StdFmtArguments::from(format_args!("{}{}", format_args!("{error}"), constants_str::NEWLINE)));
                })?;
                let identifier = value.checked_add(constants_i64::ONE).ok_or_else(|| {
                    macro_helpers::tool_console_stream::ToolConsoleStream::write_or_exit(macro_helpers::tool_console_stream::ToolConsoleStream::StandardError, macro_helpers::std_fmt_arguments::StdFmtArguments::from(format_args!("{}{}", format_args!("{}", constants_str::RUNNER_CLI_TEXT_E9108304), constants_str::NEWLINE)));
                })?;
                Ok(
                    server_admin_contract::admin_permission_summary::AdminPermissionSummary::new(
                        server_admin_contract::admin_permission_id::AdminPermissionId::try_from(
                            identifier,
                        )
                        .map_err(|error| macro_helpers::tool_console_stream::ToolConsoleStream::write_or_exit(macro_helpers::tool_console_stream::ToolConsoleStream::StandardError, macro_helpers::std_fmt_arguments::StdFmtArguments::from(format_args!("{}{}", format_args!("{error}"), constants_str::NEWLINE))))?,
                        permission,
                    ),
                )
            })
            .collect::<Result<Vec<_>, ()>>()?;
        let role_summary = server_admin_contract::admin_role_summary::AdminRoleSummary::new(
            server_admin_contract::admin_role_id::AdminRoleId::try_from(constants_i64::ONE)
                .map_err(|error| {
                    macro_helpers::tool_console_stream::ToolConsoleStream::write_or_exit(
                        macro_helpers::tool_console_stream::ToolConsoleStream::StandardError,
                        macro_helpers::std_fmt_arguments::StdFmtArguments::from(format_args!(
                            "{}{}",
                            format_args!("{error}"),
                            constants_str::NEWLINE
                        )),
                    )
                })?,
            server_admin_contract::admin_bool::AdminBool::from(false),
            crate::create_admin_fixture_string::create_admin_fixture_string::<
                server_admin_contract::admin_role_name::AdminRoleName,
            >(String::from(constants_str::ADMIN_FIXTURE_ROLE_NAME))
            .map_err(render_admin_fixture_conversion_error)?,
            server_admin_contract::admin_permission_ids::AdminPermissionIds::try_from(
                permission_summaries
                    .iter()
                    .map(
                        server_admin_contract::admin_permission_summary::AdminPermissionSummary::id,
                    )
                    .collect::<Vec<_>>(),
            )
            .map_err(|error| {
                macro_helpers::tool_console_stream::ToolConsoleStream::write_or_exit(
                    macro_helpers::tool_console_stream::ToolConsoleStream::StandardError,
                    macro_helpers::std_fmt_arguments::StdFmtArguments::from(format_args!(
                        "{}{}",
                        format_args!("{error}"),
                        constants_str::NEWLINE
                    )),
                )
            })?,
        );
        let role_summaries = vec![role_summary];
        let audit_details =
            server_admin_contract::serde_json_admin_audit_details::SerdeJsonAdminAuditDetails::try_from(
                serde_json::json!({
                    constants_str::FIELD: constants_str::DISPLAY_NAME
                }),
            )
            .map_err(|error| macro_helpers::tool_console_stream::ToolConsoleStream::write_or_exit(macro_helpers::tool_console_stream::ToolConsoleStream::StandardError, macro_helpers::std_fmt_arguments::StdFmtArguments::from(format_args!("{}{}", format_args!("{error}"), constants_str::NEWLINE))))?;
        let audit_log_id = server_admin_contract::admin_audit_log_id::AdminAuditLogId::try_from(
            constants_i64::ONE,
        )
        .map_err(|error| {
            macro_helpers::tool_console_stream::ToolConsoleStream::write_or_exit(
                macro_helpers::tool_console_stream::ToolConsoleStream::StandardError,
                macro_helpers::std_fmt_arguments::StdFmtArguments::from(format_args!(
                    "{}{}",
                    format_args!("{error}"),
                    constants_str::NEWLINE
                )),
            );
        })?;
        let audit_user_id = server_admin_contract::admin_user_id::AdminUserId::try_from(25i64)
            .map_err(|error| {
                macro_helpers::tool_console_stream::ToolConsoleStream::write_or_exit(
                    macro_helpers::tool_console_stream::ToolConsoleStream::StandardError,
                    macro_helpers::std_fmt_arguments::StdFmtArguments::from(format_args!(
                        "{}{}",
                        format_args!("{error}"),
                        constants_str::NEWLINE
                    )),
                );
            })?;
        let audit = vec![
            server_admin_contract::admin_audit_view::AdminAuditView::new(
                crate::create_admin_fixture_string::create_admin_fixture_string::<
                    server_admin_contract::admin_text::AdminText,
                >(String::from(constants_str::ADMIN_FIXTURE_AUDIT_ACTION))
                .map_err(render_admin_fixture_conversion_error)?,
                crate::create_admin_fixture_string::create_admin_fixture_string::<
                    server_admin_contract::admin_audit_timestamp::AdminAuditTimestamp,
                >(String::from(constants_str::ADMIN_FIXTURE_AUDIT_CREATED_AT))
                .map_err(render_admin_fixture_conversion_error)?,
                Some(audit_details),
                audit_log_id,
                crate::create_admin_fixture_string::create_admin_fixture_string::<
                    server_admin_contract::admin_text::AdminText,
                >(String::from(constants_str::ADMIN_FIXTURE_AUDIT_RESOURCE))
                .map_err(render_admin_fixture_conversion_error)?,
                Some(
                    crate::create_admin_fixture_string::create_admin_fixture_string::<
                        server_admin_contract::admin_text::AdminText,
                    >(String::from(
                        constants_str::ADMIN_FIXTURE_AUDIT_RESOURCE_ID,
                    ))
                    .map_err(render_admin_fixture_conversion_error)?,
                ),
                server_admin_contract::admin_bool::AdminBool::from(true),
                Some(audit_user_id),
                Some(
                    crate::create_admin_fixture_string::create_admin_fixture_string::<
                        server_admin_contract::admin_login::AdminLogin,
                    >(String::from(constants_str::ADMIN_FIXTURE_ALPHA_LOGIN))
                    .map_err(render_admin_fixture_conversion_error)?,
                ),
            ),
        ];
        let sessions = vec![
            server_admin_contract::admin_session_view::AdminSessionView::new(
                crate::create_admin_fixture_string::create_admin_fixture_string::<
                    server_admin_contract::admin_session_timestamp::AdminSessionTimestamp,
                >(String::from(
                    constants_str::ADMIN_FIXTURE_SESSION_CREATED_AT,
                ))
                .map_err(render_admin_fixture_conversion_error)?,
                crate::create_admin_fixture_string::create_admin_fixture_string::<
                    server_admin_contract::admin_session_timestamp::AdminSessionTimestamp,
                >(String::from(
                    constants_str::ADMIN_FIXTURE_SESSION_EXPIRES_AT,
                ))
                .map_err(render_admin_fixture_conversion_error)?,
                crate::create_admin_fixture_string::create_admin_fixture_string::<
                    server_admin_contract::admin_session_identifier::AdminSessionIdentifier,
                >(String::from(constants_str::ADMIN_FIXTURE_SESSION_ID))
                .map_err(render_admin_fixture_conversion_error)?,
                server_admin_contract::admin_bool::AdminBool::from(true),
            ),
            server_admin_contract::admin_session_view::AdminSessionView::new(
                crate::create_admin_fixture_string::create_admin_fixture_string::<
                    server_admin_contract::admin_session_timestamp::AdminSessionTimestamp,
                >(String::from(
                    constants_str::ADMIN_FIXTURE_SESSION_CREATED_AT,
                ))
                .map_err(render_admin_fixture_conversion_error)?,
                crate::create_admin_fixture_string::create_admin_fixture_string::<
                    server_admin_contract::admin_session_timestamp::AdminSessionTimestamp,
                >(String::from(
                    constants_str::ADMIN_FIXTURE_SESSION_EXPIRES_AT,
                ))
                .map_err(render_admin_fixture_conversion_error)?,
                crate::create_admin_fixture_string::create_admin_fixture_string::<
                    server_admin_contract::admin_session_identifier::AdminSessionIdentifier,
                >(String::from(constants_str::ADMIN_FIXTURE_SECOND_SESSION_ID))
                .map_err(render_admin_fixture_conversion_error)?,
                server_admin_contract::admin_bool::AdminBool::from(false),
            ),
        ];
        let authenticated_admin_json =
            serde_json::to_value(&authenticated_admin).map_err(|error| {
                macro_helpers::tool_console_stream::ToolConsoleStream::write_or_exit(
                    macro_helpers::tool_console_stream::ToolConsoleStream::StandardError,
                    macro_helpers::std_fmt_arguments::StdFmtArguments::from(format_args!(
                        "{}{}",
                        format_args!("{error}"),
                        constants_str::NEWLINE
                    )),
                );
            })?;
        let user_total = u64::try_from(users.len()).map_err(|error| {
            macro_helpers::tool_console_stream::ToolConsoleStream::write_or_exit(
                macro_helpers::tool_console_stream::ToolConsoleStream::StandardError,
                macro_helpers::std_fmt_arguments::StdFmtArguments::from(format_args!(
                    "{}{}",
                    format_args!("{error}"),
                    constants_str::NEWLINE
                )),
            );
        })?;
        let users_page = server_admin_contract::admin_users_page::AdminUsersPage::new(
            server_admin_contract::admin_user_summaries::AdminUserSummaries::try_from(users)
                .map_err(|error| {
                    macro_helpers::tool_console_stream::ToolConsoleStream::write_or_exit(
                        macro_helpers::tool_console_stream::ToolConsoleStream::StandardError,
                        macro_helpers::std_fmt_arguments::StdFmtArguments::from(format_args!(
                            "{}{}",
                            format_args!("{error}"),
                            constants_str::NEWLINE
                        )),
                    );
                })?,
            server_admin_contract::admin_role_summaries::AdminRoleSummaries::try_from(
                role_summaries.clone(),
            )
            .map_err(|error| {
                macro_helpers::tool_console_stream::ToolConsoleStream::write_or_exit(
                    macro_helpers::tool_console_stream::ToolConsoleStream::StandardError,
                    macro_helpers::std_fmt_arguments::StdFmtArguments::from(format_args!(
                        "{}{}",
                        format_args!("{error}"),
                        constants_str::NEWLINE
                    )),
                );
            })?,
            server_admin_contract::admin_page_total::AdminPageTotal::from(user_total),
        );
        let role_total = u64::try_from(role_summaries.len()).map_err(|error| {
            macro_helpers::tool_console_stream::ToolConsoleStream::write_or_exit(
                macro_helpers::tool_console_stream::ToolConsoleStream::StandardError,
                macro_helpers::std_fmt_arguments::StdFmtArguments::from(format_args!(
                    "{}{}",
                    format_args!("{error}"),
                    constants_str::NEWLINE
                )),
            );
        })?;
        let roles_page = server_admin_contract::admin_roles_page::AdminRolesPage::new(
            server_admin_contract::admin_role_summaries::AdminRoleSummaries::try_from(
                role_summaries,
            )
            .map_err(|error| {
                macro_helpers::tool_console_stream::ToolConsoleStream::write_or_exit(
                    macro_helpers::tool_console_stream::ToolConsoleStream::StandardError,
                    macro_helpers::std_fmt_arguments::StdFmtArguments::from(format_args!(
                        "{}{}",
                        format_args!("{error}"),
                        constants_str::NEWLINE
                    )),
                )
            })?,
            server_admin_contract::admin_permission_summaries::AdminPermissionSummaries::try_from(
                permission_summaries.clone(),
            )
            .map_err(|error| {
                macro_helpers::tool_console_stream::ToolConsoleStream::write_or_exit(
                    macro_helpers::tool_console_stream::ToolConsoleStream::StandardError,
                    macro_helpers::std_fmt_arguments::StdFmtArguments::from(format_args!(
                        "{}{}",
                        format_args!("{error}"),
                        constants_str::NEWLINE
                    )),
                )
            })?,
            server_admin_contract::admin_page_total::AdminPageTotal::from(role_total),
        );
        let permission_total = u64::try_from(permission_summaries.len()).map_err(|error| {
            macro_helpers::tool_console_stream::ToolConsoleStream::write_or_exit(
                macro_helpers::tool_console_stream::ToolConsoleStream::StandardError,
                macro_helpers::std_fmt_arguments::StdFmtArguments::from(format_args!(
                    "{}{}",
                    format_args!("{error}"),
                    constants_str::NEWLINE
                )),
            );
        })?;
        let permissions_page = server_admin_contract::admin_permissions_page::AdminPermissionsPage::new(
            server_admin_contract::admin_permission_summaries::AdminPermissionSummaries::try_from(
                permission_summaries,
            )
            .map_err(|error| macro_helpers::tool_console_stream::ToolConsoleStream::write_or_exit(macro_helpers::tool_console_stream::ToolConsoleStream::StandardError, macro_helpers::std_fmt_arguments::StdFmtArguments::from(format_args!("{}{}", format_args!("{error}"), constants_str::NEWLINE))))?,
            server_admin_contract::admin_page_total::AdminPageTotal::from(permission_total),
        );
        let users_json = serde_json::to_value(&users_page).map_err(|error| {
            macro_helpers::tool_console_stream::ToolConsoleStream::write_or_exit(
                macro_helpers::tool_console_stream::ToolConsoleStream::StandardError,
                macro_helpers::std_fmt_arguments::StdFmtArguments::from(format_args!(
                    "{}{}",
                    format_args!("{error}"),
                    constants_str::NEWLINE
                )),
            );
        })?;
        let role_summaries_json = serde_json::to_value(&roles_page).map_err(|error| {
            macro_helpers::tool_console_stream::ToolConsoleStream::write_or_exit(
                macro_helpers::tool_console_stream::ToolConsoleStream::StandardError,
                macro_helpers::std_fmt_arguments::StdFmtArguments::from(format_args!(
                    "{}{}",
                    format_args!("{error}"),
                    constants_str::NEWLINE
                )),
            );
        })?;
        let permission_summaries_json =
            serde_json::to_value(&permissions_page).map_err(|error| {
                macro_helpers::tool_console_stream::ToolConsoleStream::write_or_exit(
                    macro_helpers::tool_console_stream::ToolConsoleStream::StandardError,
                    macro_helpers::std_fmt_arguments::StdFmtArguments::from(format_args!(
                        "{}{}",
                        format_args!("{error}"),
                        constants_str::NEWLINE
                    )),
                );
            })?;
        let audit_cursor = server_admin_contract::admin_audit_cursor::AdminAuditCursor::new(
            crate::create_admin_fixture_string::create_admin_fixture_string::<
                server_admin_contract::admin_audit_timestamp::AdminAuditTimestamp,
            >(String::from(constants_str::ADMIN_FIXTURE_AUDIT_CREATED_AT))
            .map_err(render_admin_fixture_conversion_error)?,
            server_admin_contract::admin_audit_log_id::AdminAuditLogId::try_from(
                constants_i64::ONE,
            )
            .map_err(|error| {
                macro_helpers::tool_console_stream::ToolConsoleStream::write_or_exit(
                    macro_helpers::tool_console_stream::ToolConsoleStream::StandardError,
                    macro_helpers::std_fmt_arguments::StdFmtArguments::from(format_args!(
                        "{}{}",
                        format_args!("{error}"),
                        constants_str::NEWLINE
                    )),
                );
            })?,
        );
        let audit_page = server_admin_contract::admin_audit_page::AdminAuditPage::new(
            server_admin_contract::admin_audit_views::AdminAuditViews::try_from(audit).map_err(
                |error| {
                    macro_helpers::tool_console_stream::ToolConsoleStream::write_or_exit(
                        macro_helpers::tool_console_stream::ToolConsoleStream::StandardError,
                        macro_helpers::std_fmt_arguments::StdFmtArguments::from(format_args!(
                            "{}{}",
                            format_args!("{error}"),
                            constants_str::NEWLINE
                        )),
                    );
                },
            )?,
            Some(audit_cursor),
            server_admin_contract::admin_page_total::AdminPageTotal::from(1u64),
        );
        let audit_json = serde_json::to_value(&audit_page).map_err(|error| {
            macro_helpers::tool_console_stream::ToolConsoleStream::write_or_exit(
                macro_helpers::tool_console_stream::ToolConsoleStream::StandardError,
                macro_helpers::std_fmt_arguments::StdFmtArguments::from(format_args!(
                    "{}{}",
                    format_args!("{error}"),
                    constants_str::NEWLINE
                )),
            );
        })?;
        let sessions_json = serde_json::to_value(&sessions).map_err(|error| {
            macro_helpers::tool_console_stream::ToolConsoleStream::write_or_exit(
                macro_helpers::tool_console_stream::ToolConsoleStream::StandardError,
                macro_helpers::std_fmt_arguments::StdFmtArguments::from(format_args!(
                    "{}{}",
                    format_args!("{error}"),
                    constants_str::NEWLINE
                )),
            );
        })?;
        let no_body_json = serde_json::to_value(server_admin_contract::admin_no_body::AdminNoBody)
            .map_err(|error| {
                macro_helpers::tool_console_stream::ToolConsoleStream::write_or_exit(
                    macro_helpers::tool_console_stream::ToolConsoleStream::StandardError,
                    macro_helpers::std_fmt_arguments::StdFmtArguments::from(format_args!(
                        "{}{}",
                        format_args!("{error}"),
                        constants_str::NEWLINE
                    )),
                );
            })?;
        let open_api_json = serde_json::to_value(utoipa::openapi::OpenApi::from(
            server_admin::generated_open_api::generated_open_api(),
        ))
        .map_err(|error| {
            macro_helpers::tool_console_stream::ToolConsoleStream::write_or_exit(
                macro_helpers::tool_console_stream::ToolConsoleStream::StandardError,
                macro_helpers::std_fmt_arguments::StdFmtArguments::from(format_args!(
                    "{}{}",
                    format_args!("{error}"),
                    constants_str::NEWLINE
                )),
            );
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
                                    <server_admin_contract::admin_route::AdminAuthenticationRouteFamily as frontend_contract::route_family::RouteFamily>::body_limit()
                                        .map(frontend_contract::route_body_limit::RouteBodyLimit::get),
                                    open_api_json,
                                ]))
                                .map_err(|error| {
                                    macro_helpers::tool_console_stream::ToolConsoleStream::write_or_exit(macro_helpers::tool_console_stream::ToolConsoleStream::StandardError, macro_helpers::std_fmt_arguments::StdFmtArguments::from(format_args!("{}{}", format_args!("{error}"), constants_str::NEWLINE)));
                                })?;
        let Some(workspace_root) = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).parent() else {
            return Err(());
        };
        let target = workspace_root.join(constants_str::TARGET);
        std::fs::create_dir_all(target.as_path()).map_err(|error| {
            macro_helpers::tool_console_stream::ToolConsoleStream::write_or_exit(
                macro_helpers::tool_console_stream::ToolConsoleStream::StandardError,
                macro_helpers::std_fmt_arguments::StdFmtArguments::from(format_args!(
                    "{}{}",
                    format_args!("{error}"),
                    constants_str::NEWLINE
                )),
            );
        })?;
        std::fs::write(
            target.join(constants_str::WORKSPACE_TEST_RUNNER_ADMIN_CONTRACT_FIXTURE_FILE),
            fixture,
        )
        .map_err(|error| {
            macro_helpers::tool_console_stream::ToolConsoleStream::write_or_exit(
                macro_helpers::tool_console_stream::ToolConsoleStream::StandardError,
                macro_helpers::std_fmt_arguments::StdFmtArguments::from(format_args!(
                    "{}{}",
                    format_args!("{error}"),
                    constants_str::NEWLINE
                )),
            );
        })
    })();
    match result {
        Ok(()) => crate::runner_cli_outcome::RunnerCliOutcome::Completed,
        Err(()) => crate::runner_cli_outcome::RunnerCliOutcome::Failed,
    }
}
