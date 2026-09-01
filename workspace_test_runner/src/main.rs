#![allow(
    clippy::exit,
    reason = "the workspace test runner owns immediate process termination for failed tool modes"
)]
#![allow(
    clippy::needless_for_each,
    reason = "repository policy forbids for loops"
)]
#![allow(
    clippy::wildcard_imports,
    reason = "root-owned runner modes retain the former domain facade vocabulary"
)]

pub mod admin_fixture;
pub mod admin_fixture_string;
pub mod allocation_tool;
pub mod allocation_tools;
pub mod ansi_text_ref;
pub mod cargo_args;
pub mod cargo_subcommand_available;
pub mod check_tool_available;
pub mod clean_ansi_text;
pub mod command_duration;
pub mod command_duration_millis;
pub mod command_idx;
pub mod command_run;
pub mod command_started_at_instant;
pub mod command_succeeded;
pub mod command_text;
pub mod command_texts;
pub mod commands_ref;
pub mod create_admin_fixture_string;
pub mod domain_types;
pub mod execution_io_error;
#[cfg(test)]
pub mod execution_tests;
pub mod failed_test_names;
pub mod generate_pg_table_measure_input_token_stream;
pub mod macro_generation_measurements;
pub mod measure_cargo_command;
pub mod measure_memusage_command;
pub mod measurement_name;
pub mod memusage_column_idx;
pub mod memusage_heap_value;
pub mod memusage_key;
pub mod memusage_prog_name_ref;
pub mod memusage_row_name;
pub mod memusage_table_value;
pub mod memusage_value_ref;
pub mod print_without_measurement_footer;
pub mod print_without_memusage_footer;
pub mod program_args_ref;
pub mod program_path_ref;
pub mod quote_token_stream_generate_pg_table_measure_input_token_stream;
pub mod run_commands;
pub mod run_counter;
pub mod run_workspace_tests;
pub mod runner_mode;
pub mod stderr_text_ref;
pub mod strip_ansi;
pub mod strip_ansi_codes;
pub mod summary_text;
pub mod text_ref;
pub mod tool_available;
pub mod tool_name;
pub mod tool_path;

#[cfg(test)]
pub mod test_tests;

fn main() {
    let mode = std::env::args().nth(constants_usize::ONE).map(|value| {
        runner_mode::RunnerMode::try_from(value).unwrap_or_else(runner_mode::RunnerMode::from)
    });
    let result = match mode.as_ref().map(runner_mode::RunnerMode::as_ref) {
        None | Some(constants_str::STATIC) => run_commands::run_commands(
            commands_ref::CommandsRef::from(&constants_str::WORKSPACE_TEST_RUNNER_STATIC_COMMANDS),
        ),
        Some(constants_str::DATABASE) => {
            match std::env::var(constants_str::ENV_NAMES_DATABASE_URL) {
                Ok(database_url) => {
                    match macro_helpers::validate_test_database_url::validate_test_database_url(
                        macro_helpers::url_ref::UrlRef::from(database_url.as_str()),
                    ) {
                        Ok(_target) => {
                            run_commands::run_commands(commands_ref::CommandsRef::from(&[(
                                constants_str::WORKSPACE_TEST_RUNNER_CARGO,
                                &constants_str::WORKSPACE_TEST_RUNNER_CARGO_TEST_DATABASE_ARGS[..],
                            )]))
                        }
                        Err(error) => {
                            eprintln!("database test guard rejected DATABASE_URL: {error}");
                            Err(())
                        }
                    }
                }
                Err(error) => {
                    eprintln!("database test mode requires DATABASE_URL: {error}");
                    Err(())
                }
            }
        }
        Some(constants_str::WORKSPACE_TEST_RUNNER_GENERATE_PG_TABLE_WORKLOAD) => {
            let input = generate_pg_table_measure_input_token_stream::generate_pg_table_measure_input_token_stream(
                &quote::quote! {"False"},
            );
            let repeat_count = domain_types::DIRECT_GENERATION_REPEAT_COUNT;
            let output_bytes = (0..repeat_count).fold(constants_usize::ZERO, |accumulator, _| {
                let output = generate_pg_table_src::generate_pg_table::generate_pg_table(
                    macro_helpers::proc_macro2_token_stream_ref::ProcMacro2TokenStreamRef::from(
                        input.as_ref(),
                    ),
                );
                accumulator.saturating_add(output.to_string().len())
            });
            println!(
                "allocation_workload=generate_pg_table_src repeat_count={repeat_count} output_bytes={output_bytes}",
            );
            Ok(())
        }
        Some(constants_str::WORKSPACE_TEST_RUNNER_GENERATE_PG_TYPES_WORKLOAD) => {
            let input = quote::quote! {
                {
                    "pg_table_cols_write_into_file": "False",
                    "whole_write_into_file": "False",
                    "variant": "All"
                }
            };
            let repeat_count = domain_types::DIRECT_GENERATION_REPEAT_COUNT;
            let output_bytes = (0..repeat_count).fold(constants_usize::ZERO, |accumulator, _| {
                let output =
                    generate_pg_types_src::generate_pg_types_tokens::generate_pg_types_tokens(
                        macro_helpers::proc_macro2_token_stream_ref::ProcMacro2TokenStreamRef::from(
                            &input,
                        ),
                    );
                accumulator.saturating_add(output.to_string().len())
            });
            println!(
                "allocation_workload=generate_pg_types_src repeat_count={repeat_count} output_bytes={output_bytes}",
            );
            Ok(())
        }
        Some(constants_str::WORKSPACE_TEST_RUNNER_ADMIN_CONTRACT_FIXTURE) => (|| {
            let no_body_schema = serde_json::to_value(
                <server_admin_contract::admin_no_body::AdminNoBody as utoipa::PartialSchema>::schema(
                ),
            )
            .map_err(|error| eprintln!("{error}"))?;
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
                .map(|permission| {
                    serde_json::Value::String(permission.as_str().as_ref().to_owned())
                })
                .collect::<Vec<_>>();
            let permission_values = server_admin_contract::admin_permission::AdminPermission::ALL
                .into_iter()
                .map(|permission| {
                    create_admin_fixture_string::create_admin_fixture_string::<
                        server_admin_contract::admin_permission_value::AdminPermissionValue,
                    >(permission.as_str().as_ref().to_owned())
                })
                .collect::<Result<Vec<_>, ()>>()?;
            let authenticated_admin = server_admin_contract::authenticated_admin::AuthenticatedAdmin::new(
                create_admin_fixture_string::create_admin_fixture_string::<
                    server_admin_contract::admin_display_name::AdminDisplayName,
                >(String::from(constants_str::ADMIN))?,
                server_admin_contract::admin_user_id::AdminUserId::try_from(constants_i64::ONE)
                    .map_err(|error| eprintln!("{error}"))?,
                create_admin_fixture_string::create_admin_fixture_string::<
                    server_admin_contract::admin_login::AdminLogin,
                >(String::from(constants_str::ROOT))?,
                server_admin_contract::admin_permission_values::AdminPermissionValues::try_from(
                    permission_values.clone(),
                )
                .map_err(|error| eprintln!("{error}"))?,
                server_admin_contract::admin_role_names::AdminRoleNames::try_from(vec![
                    create_admin_fixture_string::create_admin_fixture_string::<
                        server_admin_contract::admin_role_name::AdminRoleName,
                    >(String::from(
                        constants_str::ADMIN_FIXTURE_ROLE_NAME,
                    ))?,
                ])
                .map_err(|error| eprintln!("{error}"))?,
            );
            let users = (constants_i64::ZERO..25i64)
                .map(|index| {
                    let number = index.checked_add(constants_i64::ONE).ok_or_else(|| {
                        eprintln!("administrator fixture user identifier overflow");
                    })?;
                    let is_alpha = index == 24i64;
                    let role_id = server_admin_contract::admin_role_id::AdminRoleId::try_from(
                        constants_i64::ONE,
                    )
                    .map_err(|error| eprintln!("{error}"))?;
                    Ok(
                        server_admin_contract::admin_user_summary::AdminUserSummary::new(
                            create_admin_fixture_string::create_admin_fixture_string::<
                                server_admin_contract::admin_display_name::AdminDisplayName,
                            >(if is_alpha {
                                String::from(constants_str::ADMIN_FIXTURE_ALPHA_DISPLAY_NAME)
                            } else {
                                format!("User {number:02}")
                            })?,
                            server_admin_contract::admin_user_id::AdminUserId::try_from(number)
                                .map_err(|error| eprintln!("{error}"))?,
                            server_admin_contract::admin_bool::AdminBool::from(
                                index & constants_i64::ONE == constants_i64::ZERO,
                            ),
                            create_admin_fixture_string::create_admin_fixture_string::<
                                server_admin_contract::admin_login::AdminLogin,
                            >(if is_alpha {
                                String::from(constants_str::ADMIN_FIXTURE_ALPHA_LOGIN)
                            } else {
                                format!("user_{number:02}")
                            })?,
                            server_admin_contract::admin_role_ids::AdminRoleIds::try_from(vec![
                                role_id,
                            ])
                            .map_err(|error| eprintln!("{error}"))?,
                        ),
                    )
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
                        server_admin_contract::admin_permission_summary::AdminPermissionSummary::new(
                            server_admin_contract::admin_permission_id::AdminPermissionId::try_from(
                                identifier,
                            )
                            .map_err(|error| eprintln!("{error}"))?,
                            permission,
                        ),
                    )
                })
                .collect::<Result<Vec<_>, ()>>()?;
            let role_summary = server_admin_contract::admin_role_summary::AdminRoleSummary::new(
                server_admin_contract::admin_role_id::AdminRoleId::try_from(constants_i64::ONE)
                    .map_err(|error| eprintln!("{error}"))?,
                server_admin_contract::admin_bool::AdminBool::from(false),
                create_admin_fixture_string::create_admin_fixture_string::<
                    server_admin_contract::admin_role_name::AdminRoleName,
                >(String::from(
                    constants_str::ADMIN_FIXTURE_ROLE_NAME,
                ))?,
                server_admin_contract::admin_permission_ids::AdminPermissionIds::try_from(
                    permission_summaries
                        .iter()
                        .map(server_admin_contract::admin_permission_summary::AdminPermissionSummary::id)
                        .collect::<Vec<_>>(),
                )
                .map_err(|error| eprintln!("{error}"))?,
            );
            let role_summaries = vec![role_summary];
            let audit_details =
                server_admin_contract::serde_json_admin_audit_details::SerdeJsonAdminAuditDetails::try_from(
                    serde_json::json!({
                        constants_str::FIELD: constants_str::DISPLAY_NAME
                    }),
                )
                .map_err(|error| eprintln!("{error}"))?;
            let audit_log_id =
                server_admin_contract::admin_audit_log_id::AdminAuditLogId::try_from(
                    constants_i64::ONE,
                )
                .map_err(|error| eprintln!("{error}"))?;
            let audit_user_id = server_admin_contract::admin_user_id::AdminUserId::try_from(25i64)
                .map_err(|error| eprintln!("{error}"))?;
            let audit = vec![
                server_admin_contract::admin_audit_view::AdminAuditView::new(
                    create_admin_fixture_string::create_admin_fixture_string::<
                        server_admin_contract::admin_text::AdminText,
                    >(String::from(constants_str::ADMIN_FIXTURE_AUDIT_ACTION))?,
                    create_admin_fixture_string::create_admin_fixture_string::<
                        server_admin_contract::admin_audit_timestamp::AdminAuditTimestamp,
                    >(String::from(
                        constants_str::ADMIN_FIXTURE_AUDIT_CREATED_AT,
                    ))?,
                    Some(audit_details),
                    audit_log_id,
                    create_admin_fixture_string::create_admin_fixture_string::<
                        server_admin_contract::admin_text::AdminText,
                    >(String::from(
                        constants_str::ADMIN_FIXTURE_AUDIT_RESOURCE,
                    ))?,
                    Some(create_admin_fixture_string::create_admin_fixture_string::<
                        server_admin_contract::admin_text::AdminText,
                    >(String::from(
                        constants_str::ADMIN_FIXTURE_AUDIT_RESOURCE_ID,
                    ))?),
                    server_admin_contract::admin_bool::AdminBool::from(true),
                    Some(audit_user_id),
                    Some(create_admin_fixture_string::create_admin_fixture_string::<
                        server_admin_contract::admin_login::AdminLogin,
                    >(String::from(
                        constants_str::ADMIN_FIXTURE_ALPHA_LOGIN,
                    ))?),
                ),
            ];
            let sessions = vec![
                server_admin_contract::admin_session_view::AdminSessionView::new(
                    create_admin_fixture_string::create_admin_fixture_string::<
                        server_admin_contract::admin_session_timestamp::AdminSessionTimestamp,
                    >(String::from(
                        constants_str::ADMIN_FIXTURE_SESSION_CREATED_AT,
                    ))?,
                    create_admin_fixture_string::create_admin_fixture_string::<
                        server_admin_contract::admin_session_timestamp::AdminSessionTimestamp,
                    >(String::from(
                        constants_str::ADMIN_FIXTURE_SESSION_EXPIRES_AT,
                    ))?,
                    create_admin_fixture_string::create_admin_fixture_string::<
                        server_admin_contract::admin_session_identifier::AdminSessionIdentifier,
                    >(String::from(constants_str::ADMIN_FIXTURE_SESSION_ID))?,
                    server_admin_contract::admin_bool::AdminBool::from(true),
                ),
                server_admin_contract::admin_session_view::AdminSessionView::new(
                    create_admin_fixture_string::create_admin_fixture_string::<
                        server_admin_contract::admin_session_timestamp::AdminSessionTimestamp,
                    >(String::from(
                        constants_str::ADMIN_FIXTURE_SESSION_CREATED_AT,
                    ))?,
                    create_admin_fixture_string::create_admin_fixture_string::<
                        server_admin_contract::admin_session_timestamp::AdminSessionTimestamp,
                    >(String::from(
                        constants_str::ADMIN_FIXTURE_SESSION_EXPIRES_AT,
                    ))?,
                    create_admin_fixture_string::create_admin_fixture_string::<
                        server_admin_contract::admin_session_identifier::AdminSessionIdentifier,
                    >(String::from(
                        constants_str::ADMIN_FIXTURE_SECOND_SESSION_ID,
                    ))?,
                    server_admin_contract::admin_bool::AdminBool::from(false),
                ),
            ];
            let authenticated_admin_json =
                serde_json::to_value(&authenticated_admin).map_err(|error| {
                    eprintln!("{error}");
                })?;
            let user_total = u64::try_from(users.len()).map_err(|error| eprintln!("{error}"))?;
            let users_page = server_admin_contract::admin_users_page::AdminUsersPage::new(
                server_admin_contract::admin_user_summaries::AdminUserSummaries::try_from(users)
                    .map_err(|error| eprintln!("{error}"))?,
                server_admin_contract::admin_role_summaries::AdminRoleSummaries::try_from(
                    role_summaries.clone(),
                )
                .map_err(|error| eprintln!("{error}"))?,
                server_admin_contract::admin_page_total::AdminPageTotal::from(user_total),
            );
            let role_total =
                u64::try_from(role_summaries.len()).map_err(|error| eprintln!("{error}"))?;
            let roles_page = server_admin_contract::admin_roles_page::AdminRolesPage::new(
                server_admin_contract::admin_role_summaries::AdminRoleSummaries::try_from(role_summaries)
                    .map_err(|error| eprintln!("{error}"))?,
                server_admin_contract::admin_permission_summaries::AdminPermissionSummaries::try_from(
                    permission_summaries.clone(),
                )
                .map_err(|error| eprintln!("{error}"))?,
                server_admin_contract::admin_page_total::AdminPageTotal::from(role_total),
            );
            let permission_total =
                u64::try_from(permission_summaries.len()).map_err(|error| eprintln!("{error}"))?;
            let permissions_page = server_admin_contract::admin_permissions_page::AdminPermissionsPage::new(
                server_admin_contract::admin_permission_summaries::AdminPermissionSummaries::try_from(
                    permission_summaries,
                )
                .map_err(|error| eprintln!("{error}"))?,
                server_admin_contract::admin_page_total::AdminPageTotal::from(permission_total),
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
            let audit_cursor = server_admin_contract::admin_audit_cursor::AdminAuditCursor::new(
                create_admin_fixture_string::create_admin_fixture_string::<
                    server_admin_contract::admin_audit_timestamp::AdminAuditTimestamp,
                >(String::from(constants_str::ADMIN_FIXTURE_AUDIT_CREATED_AT))?,
                server_admin_contract::admin_audit_log_id::AdminAuditLogId::try_from(
                    constants_i64::ONE,
                )
                .map_err(|error| eprintln!("{error}"))?,
            );
            let audit_page = server_admin_contract::admin_audit_page::AdminAuditPage::new(
                server_admin_contract::admin_audit_views::AdminAuditViews::try_from(audit)
                    .map_err(|error| eprintln!("{error}"))?,
                Some(audit_cursor),
                server_admin_contract::admin_page_total::AdminPageTotal::from(1u64),
            );
            let audit_json = serde_json::to_value(&audit_page).map_err(|error| {
                eprintln!("{error}");
            })?;
            let sessions_json = serde_json::to_value(&sessions).map_err(|error| {
                eprintln!("{error}");
            })?;
            let no_body_json = serde_json::to_value(
                server_admin_contract::admin_no_body::AdminNoBody,
            )
            .map_err(|error| {
                eprintln!("{error}");
            })?;
            let open_api_json = serde_json::to_value(utoipa::openapi::OpenApi::from(
                server_admin::generated_open_api::generated_open_api(),
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
                                        <server_admin_contract::admin_route::AdminAuthenticationRouteFamily as frontend_contract::route_family::RouteFamily>::body_limit()
                                            .map(frontend_contract::route_body_limit::RouteBodyLimit::get),
                                        open_api_json,
                                    ]))
                                    .map_err(|error| {
                                        eprintln!("{error}");
                                    })?;
            let Some(workspace_root) = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).parent()
            else {
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
        })(),
        Some(constants_str::WORKSPACE_TEST_RUNNER_PG_CRUD_COMMON_QUERY_PART_WORKLOAD) => (|| {
            let output_bytes = (0..domain_types::SQL_BUILDER_MEASURE_SERIES_COUNT)
                                .try_fold(constants_usize::ZERO, |series_accumulator, _| {
                                    (0..domain_types::MEASURE_REPEAT_COUNT).try_fold(
                                        series_accumulator,
                                        |accumulator, _| {
                                            let mut increment = constants_u64::ZERO;
                                            match pg_crud_common::pg_type_where_filter::PgTypeWhereFilter::query_part(
                                                &pg_crud_common::pagination_base::PaginationBase::default(),
                                                &mut increment,
                                                pg_crud_common::sql_column_ref::SqlColumnRef::from(
                                                    &constants_str::COLUMN,
                                                ),
                                                pg_crud_common::add_operator::AddOperator::from(false),
                                            ) {
                                                Ok(fragment) => {
                                                    Ok(accumulator.saturating_add(fragment.as_ref().len()))
                                                }
                                                Err(error) => {
                                                    eprintln!(
                                                        "allocation_workload=pg_crud_common_query_part status=failed error={error:?}"
                                                    );
                                                    Err(())
                                                }
                                            }
                                        },
                                    )
                                })?;
            println!(
                "allocation_workload=pg_crud_common_query_part series_count={series_count} repeat_count={repeat_count} output_bytes={output_bytes}",
                series_count = domain_types::SQL_BUILDER_MEASURE_SERIES_COUNT,
                repeat_count = domain_types::MEASURE_REPEAT_COUNT,
            );
            Ok(())
        })(
        ),
        Some(constants_str::WORKSPACE_TEST_RUNNER_WHERE_FILTERS_QUERY_PART_WORKLOAD) => (|| {
            let where_filters_values = (constants_i32::ZERO..64i32).collect::<Vec<i32>>();
            let where_filters_bounded_vec =
                match where_filters::pg_filter_vec::PgFilterVec::<i32, 64>::try_from(
                    where_filters_values,
                ) {
                    Ok(value) => value,
                    Err(error) => {
                        eprintln!(
                            "allocation_workload=where_filters_query_part status=setup_failed error={error:?}"
                        );
                        return Err(());
                    }
                };
            let output_bytes = (0..domain_types::SQL_BUILDER_MEASURE_SERIES_COUNT)
                                .try_fold(constants_usize::ZERO, |series_accumulator, _| {
                                    (0..domain_types::MEASURE_REPEAT_COUNT).try_fold(
                                        series_accumulator,
                                        |accumulator, _| {
                                            let mut increment = constants_u64::ZERO;
                                            match where_filters_bounded_vec.pg_type_query_part(
                                                &mut increment,
                                                pg_crud_common::sql_column_ref::SqlColumnRef::from(
                                                    &constants_str::COLUMN,
                                                ),
                                                pg_crud_common::add_operator::AddOperator::from(false),
                                            ) {
                                                Ok(fragment) => {
                                                    Ok(accumulator.saturating_add(fragment.as_ref().len()))
                                                }
                                                Err(error) => {
                                                    eprintln!(
                                                        "allocation_workload=where_filters_query_part status=failed error={error:?}"
                                                    );
                                                    Err(())
                                                }
                                            }
                                        },
                                    )
                                })?;
            println!(
                "allocation_workload=where_filters_query_part series_count={series_count} repeat_count={repeat_count} output_bytes={output_bytes}",
                series_count = domain_types::SQL_BUILDER_MEASURE_SERIES_COUNT,
                repeat_count = domain_types::MEASURE_REPEAT_COUNT,
            );
            Ok(())
        })(
        ),
        Some(constants_str::MACRO_GENERATION) => {
            macro_generation_measurements::macro_generation_measurements()
                .iter()
                .try_fold((), |(), (measurement_name, args)| {
                    measure_cargo_command::measure_cargo_command(*measurement_name, *args)
                })
        }
        Some(constants_str::TESTS_ALT) => run_workspace_tests::run_workspace_tests(),
        Some(constants_str::HEAVY_LOAD) => {
            if cargo_subcommand_available::cargo_subcommand_available(tool_name::ToolName::from(
                constants_str::NEXTEST,
            ))
            .get()
            {
                run_commands::run_commands(commands_ref::CommandsRef::from(&[(
                    constants_str::WORKSPACE_TEST_RUNNER_CARGO,
                    &constants_str::WORKSPACE_TEST_RUNNER_NEXTEST_HEAVY_ARGS[..],
                )]))
            } else {
                eprintln!("heavy-load mode requires cargo-nextest; optional tool is unavailable");
                Err(())
            }
        }
        Some(constants_str::RELEASE) => {
            [
                constants_str::WORKSPACE_TEST_RUNNER_AUDIT_SUBCOMMAND,
                constants_str::WORKSPACE_TEST_RUNNER_DENY_SUBCOMMAND,
                constants_str::WORKSPACE_TEST_RUNNER_HACK_SUBCOMMAND,
                constants_str::SEMVER_CHECKS,
                constants_str::UDEPS,
                constants_str::MACHETE,
                constants_str::LLVM_COV,
            ]
            .into_iter()
            .for_each(|tool| {
                println!(
                    "release_tool={tool} available={}",
                    cargo_subcommand_available::cargo_subcommand_available(
                        tool_name::ToolName::from(tool)
                    )
                    .get()
                );
            });
            let mut commands =
                Vec::<(&str, &[&str])>::from(constants_str::WORKSPACE_TEST_RUNNER_STATIC_COMMANDS);
            if cargo_subcommand_available::cargo_subcommand_available(tool_name::ToolName::from(
                constants_str::NEXTEST,
            ))
            .get()
            {
                commands.extend(constants_str::WORKSPACE_TEST_RUNNER_NEXTEST_COMMANDS);
            } else {
                commands.extend(constants_str::WORKSPACE_TEST_RUNNER_CARGO_TEST_COMMANDS);
            }
            [
                (
                    constants_str::WORKSPACE_TEST_RUNNER_AUDIT_SUBCOMMAND,
                    constants_str::WORKSPACE_TEST_RUNNER_CARGO_AUDIT_ARGS.as_slice(),
                ),
                (
                    constants_str::WORKSPACE_TEST_RUNNER_DENY_SUBCOMMAND,
                    constants_str::WORKSPACE_TEST_RUNNER_CARGO_DENY_ARGS.as_slice(),
                ),
                (
                    constants_str::WORKSPACE_TEST_RUNNER_HACK_SUBCOMMAND,
                    constants_str::WORKSPACE_TEST_RUNNER_CARGO_HACK_ARGS.as_slice(),
                ),
                (
                    constants_str::MACHETE,
                    constants_str::WORKSPACE_TEST_RUNNER_CARGO_MACHETE_ARGS.as_slice(),
                ),
                (
                    constants_str::SEMVER_CHECKS,
                    constants_str::WORKSPACE_TEST_RUNNER_CARGO_SEMVER_CHECKS_ARGS.as_slice(),
                ),
                (
                    constants_str::UDEPS,
                    constants_str::WORKSPACE_TEST_RUNNER_CARGO_UDEPS_ARGS.as_slice(),
                ),
            ]
            .into_iter()
            .filter(|(subcommand, _args)| {
                cargo_subcommand_available::cargo_subcommand_available(tool_name::ToolName::from(
                    *subcommand,
                ))
                .get()
            })
            .for_each(|(_subcommand, args)| {
                commands.push((constants_str::WORKSPACE_TEST_RUNNER_CARGO, args));
            });
            run_commands::run_commands(commands_ref::CommandsRef::from(commands.as_slice()))
        }
        Some(constants_str::MEASURE) => {
            let allocation_tools_printed: Result<(), std::convert::Infallible> =
                allocation_tools::allocation_tools()
                    .iter()
                    .try_fold((), |(), tool| {
                        let available =
                            check_tool_available::check_tool_available(*tool.get_path());
                        println!(
                            "measurement=allocation_tool_available tool={} path={} available={}",
                            tool.get_name().get(),
                            tool.get_path().get(),
                            available.get()
                        );
                        Ok(())
                    });
            match allocation_tools_printed {
                Ok(()) => {}
                Err(error) => match error {},
            }
            if std::path::Path::new(constants_str::WORKSPACE_TEST_RUNNER_MEMUSAGE_PATH).exists() {
                println!(
                    "measurement=exact_allocations status=available tool=libmemusage path={}",
                    constants_str::WORKSPACE_TEST_RUNNER_MEMUSAGE_PATH
                );
                measure_memusage_command::measure_memusage_command(
                    measurement_name::MeasurementName::from(constants_str::CODE_STYLE),
                    program_path_ref::ProgramPathRef::from(
                        constants_str::WORKSPACE_TEST_RUNNER_CARGO,
                    ),
                    program_args_ref::ProgramArgsRef::from(&[
                        constants_str::TEST_ALT_3,
                        constants_str::P,
                        constants_str::TESTS_ALT,
                        constants_str::CODE_STYLE,
                    ]),
                    memusage_prog_name_ref::MemusageProgNameRef::from(
                        constants_str::WORKSPACE_TEST_RUNNER_CARGO,
                    ),
                )
                .unwrap_or_else(|()| std::process::exit(1));
                let current_exe = match std::env::current_exe() {
                    Ok(value) => value,
                    Err(error) => {
                        eprintln!(
                            "measurement=exact_allocations status=current_exe_failed error={error}"
                        );
                        std::process::exit(1);
                    }
                };
                let current_exe_string = current_exe.to_string_lossy().to_string();
                let current_exe_prog_name = current_exe
                    .file_name()
                    .and_then(|value| value.to_str())
                    .unwrap_or(constants_str::WORKSPACE_TEST_RUNNER_ALT);
                [
                    (
                        measurement_name::MeasurementName::from(
                            constants_str::GENERATE_PG_TABLE_SRC,
                        ),
                        constants_str::WORKSPACE_TEST_RUNNER_GENERATE_PG_TABLE_WORKLOAD,
                    ),
                    (
                        measurement_name::MeasurementName::from(
                            constants_str::GENERATE_PG_TYPES_SRC,
                        ),
                        constants_str::WORKSPACE_TEST_RUNNER_GENERATE_PG_TYPES_WORKLOAD,
                    ),
                    (
                        measurement_name::MeasurementName::from(
                            constants_str::PG_CRUD_COMMON_QUERY_PART,
                        ),
                        constants_str::WORKSPACE_TEST_RUNNER_PG_CRUD_COMMON_QUERY_PART_WORKLOAD,
                    ),
                    (
                        measurement_name::MeasurementName::from(
                            constants_str::WHERE_FILTERS_QUERY_PART,
                        ),
                        constants_str::WORKSPACE_TEST_RUNNER_WHERE_FILTERS_QUERY_PART_WORKLOAD,
                    ),
                ]
                .into_iter()
                .try_fold((), |(), (measurement_name, workload_mode)| {
                    measure_memusage_command::measure_memusage_command(
                        measurement_name,
                        program_path_ref::ProgramPathRef::from(current_exe_string.as_str()),
                        program_args_ref::ProgramArgsRef::from(&[workload_mode]),
                        memusage_prog_name_ref::MemusageProgNameRef::from(current_exe_prog_name),
                    )
                })
                .unwrap_or_else(|()| std::process::exit(1));
            } else {
                println!(
                    "measurement=exact_allocations status=unavailable reason=no_safe_allocator_counter_or_external_allocation_profiler memory_proxy_fields=memory_proxy_peak_rss_kb,memory_proxy_minor_page_faults,memory_proxy_major_page_faults"
                );
            }
            measure_cargo_command::measure_cargo_command(
                measurement_name::MeasurementName::from(constants_str::CODE_STYLE),
                cargo_args::CargoArgs::from(&[
                    constants_str::TEST_ALT_3,
                    constants_str::P,
                    constants_str::TESTS_ALT,
                    constants_str::CODE_STYLE,
                ]),
            )
            .unwrap_or_else(|()| std::process::exit(1));
            measure_cargo_command::measure_cargo_command(
                measurement_name::MeasurementName::from(constants_str::CLIPPY),
                cargo_args::CargoArgs::from(
                    &constants_str::WORKSPACE_TEST_RUNNER_CARGO_CLIPPY_ARGS,
                ),
            )
            .unwrap_or_else(|()| std::process::exit(1));
            let generate_pg_table_input_token_stream =
                        generate_pg_table_measure_input_token_stream::generate_pg_table_measure_input_token_stream(
                            &quote::quote! {"False"},
                        );
            let generate_pg_table_input_with_tests_token_stream =
                        generate_pg_table_measure_input_token_stream::generate_pg_table_measure_input_token_stream(
                            &quote::quote! {"True"},
                        );
            let parse_started = std::time::Instant::now();
            let parsed = generate_pg_table_src::parse_generate_pg_table::parse_generate_pg_table(
                macro_helpers::proc_macro2_token_stream_ref::ProcMacro2TokenStreamRef::from(
                    generate_pg_table_input_token_stream.as_ref(),
                ),
            )
            .unwrap_or_else(|error| {
                std::panic::panic_any(constants_str::PANIC_D6399CBF.replacen(
                    constants_str::PANIC_PLACEHOLDER_81240055,
                    error.to_string().as_str(),
                    1usize,
                ))
            });
            let parse_us = parse_started.elapsed().as_micros();
            let build_started = std::time::Instant::now();
            let built =
                generate_pg_table_src::build_generate_pg_table::build_generate_pg_table(parsed)
                    .unwrap_or_else(|error| {
                        std::panic::panic_any(constants_str::PANIC_6ACB4E92.replacen(
                            constants_str::PANIC_PLACEHOLDER_81240055,
                            error.to_string().as_str(),
                            1usize,
                        ))
                    });
            let build_us = build_started.elapsed().as_micros();
            let validate_started = std::time::Instant::now();
            let validated =
                generate_pg_table_src::validate_generate_pg_table::validate_generate_pg_table(
                    built,
                )
                .unwrap_or_else(|error| {
                    std::panic::panic_any(constants_str::PANIC_4533A758.replacen(
                        constants_str::PANIC_PLACEHOLDER_81240055,
                        error.to_string().as_str(),
                        1usize,
                    ))
                });
            let validate_us = validate_started.elapsed().as_micros();
            let emit_started = std::time::Instant::now();
            let staged_output =
                generate_pg_table_src::emit_generate_pg_table::emit_generate_pg_table(validated);
            let emit_us = emit_started.elapsed().as_micros();
            println!(
                "measurement=generate_pg_table_typed_stages parse_us={parse_us} build_us={build_us} validate_us={validate_us} emit_us={emit_us} output_bytes={}",
                staged_output.to_string().len()
            );
            let generate_pg_table_measurement = (0..domain_types::DIRECT_GENERATION_REPEAT_COUNT)
                .fold(
                    (
                        u128::MAX,
                        constants_u128::ZERO,
                        constants_u128::ZERO,
                        constants_usize::ZERO,
                        constants_usize::ZERO,
                    ),
                    |(min_wall_us, max_wall_us, total_wall_us, _, _), _| {
                        let started = std::time::Instant::now();
                        let output = generate_pg_table_src::generate_pg_table::generate_pg_table(
                            macro_helpers::proc_macro2_token_stream_ref::ProcMacro2TokenStreamRef::from(
                                generate_pg_table_input_token_stream.as_ref(),
                            ),
                        );
                        let wall_us = started.elapsed().as_micros();
                        (
                            min_wall_us.min(wall_us),
                            max_wall_us.max(wall_us),
                            total_wall_us.saturating_add(wall_us),
                            output.to_string().len(),
                            output.as_ref().clone().into_iter().count(),
                        )
                    },
                );
            println!(
                "measurement=generate_pg_table_src repeat_count={} wall_min_us={} wall_total_us={} wall_max_us={} output_bytes={} output_token_trees={}",
                domain_types::DIRECT_GENERATION_REPEAT_COUNT,
                generate_pg_table_measurement.0,
                generate_pg_table_measurement.2,
                generate_pg_table_measurement.1,
                generate_pg_table_measurement.3,
                generate_pg_table_measurement.4
            );
            let generate_pg_table_with_tests_dir =
                std::path::Path::new(constants_str::TARGET_MEASURE_GENERATE_PG_TABLE_WITH_TESTS);
            if let Err(error) = std::fs::create_dir_all(generate_pg_table_with_tests_dir) {
                eprintln!(
                    "measurement=generate_pg_table_src_with_tests status=create_dir_failed error={error}"
                );
                std::process::exit(1);
            }
            if let Err(error) = std::fs::write(
                generate_pg_table_with_tests_dir.join(constants_str::RUSTFMT_TOML),
                constants_str::EDITION_2024_NEWLINE,
            ) {
                eprintln!(
                    "measurement=generate_pg_table_src_with_tests status=rustfmt_config_write_failed error={error}"
                );
                std::process::exit(1);
            }
            let current_dir = match std::env::current_dir() {
                Ok(value) => value,
                Err(error) => {
                    eprintln!(
                        "measurement=generate_pg_table_src_with_tests status=current_dir_failed error={error}"
                    );
                    std::process::exit(1);
                }
            };
            if let Err(error) = std::env::set_current_dir(generate_pg_table_with_tests_dir) {
                eprintln!(
                    "measurement=generate_pg_table_src_with_tests status=set_current_dir_failed error={error}"
                );
                std::process::exit(1);
            }
            let generate_pg_table_with_tests_measurement =
                (0..domain_types::DIRECT_GENERATION_REPEAT_COUNT).fold(
                    (
                        u128::MAX,
                        constants_u128::ZERO,
                        constants_u128::ZERO,
                        constants_usize::ZERO,
                        constants_usize::ZERO,
                    ),
                    |(min_wall_us, max_wall_us, total_wall_us, _, _), _| {
                        let started = std::time::Instant::now();
                        let output = generate_pg_table_src::generate_pg_table::generate_pg_table(
                            macro_helpers::proc_macro2_token_stream_ref::ProcMacro2TokenStreamRef::from(
                                generate_pg_table_input_with_tests_token_stream.as_ref(),
                            ),
                        );
                        let wall_us = started.elapsed().as_micros();
                        (
                            min_wall_us.min(wall_us),
                            max_wall_us.max(wall_us),
                            total_wall_us.saturating_add(wall_us),
                            output.to_string().len(),
                            output.as_ref().clone().into_iter().count(),
                        )
                    },
                );
            if let Err(error) = std::env::set_current_dir(current_dir) {
                eprintln!(
                    "measurement=generate_pg_table_src_with_tests status=restore_current_dir_failed error={error}"
                );
                std::process::exit(1);
            }
            let generate_pg_table_tests_stage_output_path =
                generate_pg_table_with_tests_dir.join(constants_str::GENERATE_PG_TABLE_TESTS_RS);
            let generate_pg_table_tests_stage_output =
                match server_runtime_http::read_bounded_file::read_bounded_file(
                    server_runtime_http::runtime_path_ref::RuntimePathRef::from(
                        generate_pg_table_tests_stage_output_path.as_path(),
                    ),
                    server_runtime_http::bounded_read_maximum_bytes::BoundedReadMaximumBytes::from(
                        constants_usize::VALUE_16_777_216,
                    ),
                )
                .and_then(server_runtime_http::bounded_text::BoundedText::try_from)
                {
                    Ok(content) => (content.as_ref().len(), content.as_ref().lines().count()),
                    Err(error) => {
                        eprintln!(
                            "measurement=generate_pg_table_tests_stage_output status=read_failed error={error}"
                        );
                        std::process::exit(1);
                    }
                };
            println!(
                "measurement=generate_pg_table_src_with_tests repeat_count={} wall_min_us={} wall_total_us={} wall_max_us={} output_bytes={} output_token_trees={}",
                domain_types::DIRECT_GENERATION_REPEAT_COUNT,
                generate_pg_table_with_tests_measurement.0,
                generate_pg_table_with_tests_measurement.2,
                generate_pg_table_with_tests_measurement.1,
                generate_pg_table_with_tests_measurement.3,
                generate_pg_table_with_tests_measurement.4
            );
            println!(
                "measurement=generate_pg_table_tests_stage_output bytes={} lines={}",
                generate_pg_table_tests_stage_output.0, generate_pg_table_tests_stage_output.1
            );
            println!(
                "measurement=generate_pg_table_tests_emit_delta repeat_count={} wall_total_delta_us={} wall_min_delta_us={} wall_max_delta_us={} output_bytes_delta={}",
                domain_types::DIRECT_GENERATION_REPEAT_COUNT,
                generate_pg_table_with_tests_measurement
                    .2
                    .saturating_sub(generate_pg_table_measurement.2),
                generate_pg_table_with_tests_measurement
                    .0
                    .saturating_sub(generate_pg_table_measurement.0),
                generate_pg_table_with_tests_measurement
                    .1
                    .saturating_sub(generate_pg_table_measurement.1),
                generate_pg_table_with_tests_measurement
                    .3
                    .saturating_sub(generate_pg_table_measurement.3)
            );
            let generate_pg_types_input_token_stream = quote::quote! {
                {
                    "pg_table_cols_write_into_file": "False",
                    "whole_write_into_file": "False",
                    "variant": "All"
                }
            };
            let pg_types_parse_started = std::time::Instant::now();
            let parsed_pg_types =
                generate_pg_types_src::parse_generate_pg_types::parse_generate_pg_types(
                    macro_helpers::proc_macro2_token_stream_ref::ProcMacro2TokenStreamRef::from(
                        &generate_pg_types_input_token_stream,
                    ),
                )
                .unwrap_or_else(|error| {
                    std::panic::panic_any(constants_str::PANIC_A19C725E.replacen(
                        constants_str::PANIC_PLACEHOLDER_81240055,
                        error.to_string().as_str(),
                        1usize,
                    ))
                });
            let pg_types_parse_us = pg_types_parse_started.elapsed().as_micros();
            let pg_types_build_started = std::time::Instant::now();
            let built_pg_types =
                generate_pg_types_src::build_generate_pg_types::build_generate_pg_types(
                    parsed_pg_types,
                )
                .unwrap_or_else(|error| {
                    std::panic::panic_any(constants_str::PANIC_C47612BD.replacen(
                        constants_str::PANIC_PLACEHOLDER_81240055,
                        error.to_string().as_str(),
                        1usize,
                    ))
                });
            let pg_types_build_us = pg_types_build_started.elapsed().as_micros();
            let pg_types_validate_started = std::time::Instant::now();
            let validated_pg_types =
                generate_pg_types_src::validate_generate_pg_types::validate_generate_pg_types(
                    built_pg_types,
                )
                .unwrap_or_else(|error| {
                    std::panic::panic_any(constants_str::PANIC_D3E581A4.replacen(
                        constants_str::PANIC_PLACEHOLDER_81240055,
                        error.to_string().as_str(),
                        1usize,
                    ))
                });
            let pg_types_validate_us = pg_types_validate_started.elapsed().as_micros();
            let pg_types_emit_started = std::time::Instant::now();
            let staged_pg_types =
                generate_pg_types_src::emit_generate_pg_types::emit_generate_pg_types(
                    validated_pg_types,
                );
            let pg_types_emit_us = pg_types_emit_started.elapsed().as_micros();
            println!(
                "measurement=generate_pg_types_typed_stages parse_us={pg_types_parse_us} build_us={pg_types_build_us} validate_us={pg_types_validate_us} emit_us={pg_types_emit_us} output_bytes={}",
                staged_pg_types.to_string().len()
            );
            let generate_pg_types_measurement = (0..domain_types::DIRECT_GENERATION_REPEAT_COUNT)
                .fold(
                    (
                        u128::MAX,
                        constants_u128::ZERO,
                        constants_u128::ZERO,
                        constants_usize::ZERO,
                        constants_usize::ZERO,
                    ),
                    |(min_wall_us, max_wall_us, total_wall_us, _, _), _| {
                        let started = std::time::Instant::now();
                        let output =
                        generate_pg_types_src::generate_pg_types_tokens::generate_pg_types_tokens(
                            macro_helpers::proc_macro2_token_stream_ref::ProcMacro2TokenStreamRef::from(
                                &generate_pg_types_input_token_stream,
                            ),
                        );
                        let wall_us = started.elapsed().as_micros();
                        (
                            min_wall_us.min(wall_us),
                            max_wall_us.max(wall_us),
                            total_wall_us.saturating_add(wall_us),
                            output.to_string().len(),
                            output.as_ref().clone().into_iter().count(),
                        )
                    },
                );
            println!(
                "measurement=generate_pg_types_src repeat_count={} wall_min_us={} wall_total_us={} wall_max_us={} output_bytes={} output_token_trees={}",
                domain_types::DIRECT_GENERATION_REPEAT_COUNT,
                generate_pg_types_measurement.0,
                generate_pg_types_measurement.2,
                generate_pg_types_measurement.1,
                generate_pg_types_measurement.3,
                generate_pg_types_measurement.4
            );
            let generate_where_filters_input_token_stream = quote::quote! {
                {
                    "pg_types_write_into_file": "False",
                    "whole_write_into_file": "False"
                }
            };
            let where_filters_parse_started = std::time::Instant::now();
            let parsed_where_filters = generate_where_filters_src::parse_generate_where_filters::parse_generate_where_filters(
                                generate_where_filters_src::proc_macro2_generate_where_filters_input::ProcMacro2GenerateWhereFiltersInput::from(
                                    &generate_where_filters_input_token_stream,
                                ),
                            )
                            .unwrap_or_else(|error| std::panic::panic_any(constants_str::PANIC_8F246DC1.replacen(constants_str::PANIC_PLACEHOLDER_81240055, error.to_string().as_str(), 1usize)));
            let where_filters_parse_us = where_filters_parse_started.elapsed().as_micros();
            let where_filters_build_started = std::time::Instant::now();
            let built_where_filters =
                generate_where_filters_src::build_generate_where_filters::build_generate_where_filters(
                    parsed_where_filters,
                )
                .unwrap_or_else(|error| std::panic::panic_any(constants_str::PANIC_912F6BCE.replacen(constants_str::PANIC_PLACEHOLDER_81240055, error.to_string().as_str(), 1usize)));
            let where_filters_build_us = where_filters_build_started.elapsed().as_micros();
            let where_filters_validate_started = std::time::Instant::now();
            let validated_where_filters =
                generate_where_filters_src::validate_generate_where_filters::validate_generate_where_filters(
                    built_where_filters,
                )
                .unwrap_or_else(|error| std::panic::panic_any(constants_str::PANIC_54B73A29.replacen(constants_str::PANIC_PLACEHOLDER_81240055, error.to_string().as_str(), 1usize)));
            let where_filters_validate_us = where_filters_validate_started.elapsed().as_micros();
            let where_filters_emit_started = std::time::Instant::now();
            let staged_where_filters =
                generate_where_filters_src::emit_generate_where_filters::emit_generate_where_filters(
                    validated_where_filters,
                );
            let where_filters_emit_us = where_filters_emit_started.elapsed().as_micros();
            println!(
                "measurement=generate_where_filters_typed_stages parse_us={where_filters_parse_us} build_us={where_filters_build_us} validate_us={where_filters_validate_us} emit_us={where_filters_emit_us} output_bytes={}",
                staged_where_filters.to_string().len()
            );
            let generate_where_filters_measurement = (0..domain_types::DIRECT_GENERATION_REPEAT_COUNT).fold(
                                (
                                    u128::MAX,
                                    constants_u128::ZERO,
                                    constants_u128::ZERO,
                                    constants_usize::ZERO,
                                    constants_usize::ZERO,
                                ),
                                |(min_wall_us, max_wall_us, total_wall_us, _, _), _| {
                                    let started = std::time::Instant::now();
                                    let output = generate_where_filters_src::generate_where_filters_source::generate_where_filters_source(
                                        generate_where_filters_src::proc_macro2_generate_where_filters_input::ProcMacro2GenerateWhereFiltersInput::from(
                                            &generate_where_filters_input_token_stream,
                                        ),
                                    );
                                    let wall_us = started.elapsed().as_micros();
                                    (
                                        min_wall_us.min(wall_us),
                                        max_wall_us.max(wall_us),
                                        total_wall_us.saturating_add(wall_us),
                                        output.to_string().len(),
                                        output.as_ref().clone().into_iter().count(),
                                    )
                                },
                            );
            println!(
                "measurement=generate_where_filters_src repeat_count={} wall_min_us={} wall_total_us={} wall_max_us={} output_bytes={} output_token_trees={}",
                domain_types::DIRECT_GENERATION_REPEAT_COUNT,
                generate_where_filters_measurement.0,
                generate_where_filters_measurement.2,
                generate_where_filters_measurement.1,
                generate_where_filters_measurement.3,
                generate_where_filters_measurement.4
            );
            let pg_crud_common_query_part: Result<
                (u128, u128, u128, usize),
                pg_crud_common::query_part_error::QueryPartError,
            > = (0..domain_types::SQL_BUILDER_MEASURE_SERIES_COUNT).try_fold(
                (
                    u128::MAX,
                    constants_u128::ZERO,
                    constants_u128::ZERO,
                    constants_usize::ZERO,
                ),
                |(min_wall_us, max_wall_us, total_wall_us, _), _| {
                    let started = std::time::Instant::now();
                    let output_bytes = (0..domain_types::MEASURE_REPEAT_COUNT).try_fold(
                        constants_usize::ZERO,
                        |accumulator, _| {
                            let mut increment = constants_u64::ZERO;
                            match pg_crud_common::pg_type_where_filter::PgTypeWhereFilter::query_part(
                                &pg_crud_common::pagination_base::PaginationBase::default(),
                                &mut increment,
                                pg_crud_common::sql_column_ref::SqlColumnRef::from(
                                    &constants_str::COLUMN,
                                ),
                                pg_crud_common::add_operator::AddOperator::from(false),
                            ) {
                                Ok(fragment) => {
                                    Ok(accumulator.saturating_add(fragment.as_ref().len()))
                                }
                                Err(error) => Err(error),
                            }
                        },
                    )?;
                    let wall_us = started.elapsed().as_micros();
                    Ok((
                        min_wall_us.min(wall_us),
                        max_wall_us.max(wall_us),
                        total_wall_us.saturating_add(wall_us),
                        output_bytes,
                    ))
                },
            );
            match pg_crud_common_query_part {
                Ok((min_wall_us, max_wall_us, total_wall_us, output_bytes)) => {
                    println!(
                        "measurement=pg_crud_common_query_part series_count={series_count} repeat_count={repeat_count} wall_min_us={min_wall_us} wall_total_us={total_wall_us} wall_max_us={max_wall_us} output_bytes={output_bytes}",
                        series_count = domain_types::SQL_BUILDER_MEASURE_SERIES_COUNT,
                        repeat_count = domain_types::MEASURE_REPEAT_COUNT,
                    );
                }
                Err(error) => {
                    eprintln!(
                        "measurement=pg_crud_common_query_part status=failed error={error:?}"
                    );
                    std::process::exit(1);
                }
            }
            let where_filters_values = (constants_i32::ZERO..64i32).collect::<Vec<i32>>();
            let where_filters_bounded_vec =
                match where_filters::pg_filter_vec::PgFilterVec::<i32, 64>::try_from(
                    where_filters_values,
                ) {
                    Ok(value) => value,
                    Err(error) => {
                        eprintln!(
                            "measurement=where_filters_query_part status=setup_failed error={error:?}"
                        );
                        std::process::exit(1);
                    }
                };
            let where_filters_query_part: Result<
                (u128, u128, u128, usize),
                pg_crud_common::query_part_error::QueryPartError,
            > = (0..domain_types::SQL_BUILDER_MEASURE_SERIES_COUNT).try_fold(
                (
                    u128::MAX,
                    constants_u128::ZERO,
                    constants_u128::ZERO,
                    constants_usize::ZERO,
                ),
                |(min_wall_us, max_wall_us, total_wall_us, _), _| {
                    let started = std::time::Instant::now();
                    let output_bytes = (0..domain_types::MEASURE_REPEAT_COUNT).try_fold(
                        constants_usize::ZERO,
                        |accumulator, _| {
                            let mut increment = constants_u64::ZERO;
                            match where_filters_bounded_vec.pg_type_query_part(
                                &mut increment,
                                pg_crud_common::sql_column_ref::SqlColumnRef::from(
                                    &constants_str::COLUMN,
                                ),
                                pg_crud_common::add_operator::AddOperator::from(false),
                            ) {
                                Ok(fragment) => {
                                    Ok(accumulator.saturating_add(fragment.as_ref().len()))
                                }
                                Err(error) => Err(error),
                            }
                        },
                    )?;
                    let wall_us = started.elapsed().as_micros();
                    Ok((
                        min_wall_us.min(wall_us),
                        max_wall_us.max(wall_us),
                        total_wall_us.saturating_add(wall_us),
                        output_bytes,
                    ))
                },
            );
            match where_filters_query_part {
                Ok((min_wall_us, max_wall_us, total_wall_us, output_bytes)) => {
                    println!(
                        "measurement=where_filters_query_part series_count={series_count} repeat_count={repeat_count} wall_min_us={min_wall_us} wall_total_us={total_wall_us} wall_max_us={max_wall_us} output_bytes={output_bytes}",
                        series_count = domain_types::SQL_BUILDER_MEASURE_SERIES_COUNT,
                        repeat_count = domain_types::MEASURE_REPEAT_COUNT,
                    );
                    Ok(())
                }
                Err(error) => {
                    eprintln!("measurement=where_filters_query_part status=failed error={error:?}");
                    Err(())
                }
            }
        }
        Some(constants_str::ALL_ALT) => run_commands::run_commands(
            commands_ref::CommandsRef::from(&constants_str::WORKSPACE_TEST_RUNNER_STATIC_COMMANDS),
        )
        .and_then(|()| run_workspace_tests::run_workspace_tests())
        .and_then(|()| {
            macro_generation_measurements::macro_generation_measurements()
                .iter()
                .try_fold((), |(), (measurement_name, args)| {
                    measure_cargo_command::measure_cargo_command(*measurement_name, *args)
                })
        }),
        Some(other) => {
            eprintln!(
                "unknown mode `{other}`; expected `static`, `database`, `test_tests`, `heavy-load`, `release`, `macro-generation`, `measure`, `all`, or `alloc-workload-*`"
            );
            Err(())
        }
    };
    if result.is_err() {
        std::process::exit(1);
    }
}
