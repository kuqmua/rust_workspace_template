fn publicly_forwards_crate_root(item: &syn::Item) -> bool {
    let syn::Item::Use(item_use) = item else {
        return false;
    };
    matches!(item_use.vis, syn::Visibility::Public(_))
        && matches!(
            &item_use.tree,
            syn::UseTree::Path(path) if path.ident == "crate"
        )
}
#[test]
fn private_shared_modules_do_not_forward_crate_root_exports() {
    super::snapshot::with_codebase_snapshot(|snapshot| {
        let errors = snapshot
            .rs_files()
            .iter()
            .filter(|source_file| {
                source_file.path().as_ref().file_name() == Some(std::ffi::OsStr::new("lib.rs"))
            })
            .flat_map(|crate_root| {
                crate_root
                    .ast()
                    .as_ref()
                    .items
                    .iter()
                    .filter_map(|item| {
                        let syn::Item::Mod(module) = item else {
                            return None;
                        };
                        matches!(module.vis, syn::Visibility::Inherited)
                            .then_some(module)
                            .filter(|module_ref| module_ref.content.is_none())
                    })
                    .filter_map(|module| {
                        let source_directory = crate_root.path().as_ref().parent()?;
                        [
                            source_directory.join(format!("{}.rs", module.ident)),
                            source_directory
                                .join(module.ident.to_string())
                                .join("mod.rs"),
                        ]
                        .into_iter()
                        .find_map(|module_path| {
                            snapshot
                                .rs_files()
                                .iter()
                                .find(|source_file| source_file.path().as_ref() == module_path)
                        })
                    })
                    .filter(|module_file| {
                        module_file
                            .ast()
                            .as_ref()
                            .items
                            .iter()
                            .any(publicly_forwards_crate_root)
                    })
                    .map(|module_file| module_file.path().as_ref().display().to_string())
            })
            .collect::<Vec<_>>();
        assert!(
            errors.is_empty(),
            "c8f4a271 private crate-root modules publicly forward `crate::` exports:\n{}",
            errors.join("\n")
        );
    });
}
#[test]
fn private_shared_module_forwarding_policy_distinguishes_public_visibility_and_owner() {
    let public_forward = syn::parse_file("pub use crate::owner::Item;").expect("b2d1e940 private_shared_module_forwarding_policy_distinguishes_public_visibility_and_owner invariant must hold");
    let crate_forward = syn::parse_file("pub(crate) use crate::owner::Item;").expect("53f91ac7 private_shared_module_forwarding_policy_distinguishes_public_visibility_and_owner invariant must hold");
    let local_public = syn::parse_file("pub use self::owner::Item;").expect("9a47e2c6 private_shared_module_forwarding_policy_distinguishes_public_visibility_and_owner invariant must hold");
    assert!(
        public_forward
            .items
            .iter()
            .any(publicly_forwards_crate_root)
    );
    assert!(!crate_forward.items.iter().any(publicly_forwards_crate_root));
    assert!(!local_public.items.iter().any(publicly_forwards_crate_root));
}
#[test]
fn admin_frontend_api_urls_come_from_typed_routes() {
    super::snapshot::with_codebase_snapshot(|snapshot| {
        let source = snapshot
            .rs_files()
            .iter()
            .filter(|file| {
                file.path()
                    .as_ref()
                    .to_string_lossy()
                    .contains("server_admin_frontend/src/app/")
            })
            .map(|file| file.content().as_ref())
            .collect::<String>();
        assert!(!source.contains("str_constants::V1"), "24e5ceeb");
        assert!(!source.contains("ADMIN_API_"), "72b66898");
    });
}
#[test]
#[allow(clippy::needless_for_each)] // iterator form is required by the workspace no-for-loop policy
fn service_route_handler_composition_uses_shared_registries() {
    super::snapshot::with_codebase_snapshot(|snapshot| {
        [
            ("server_admin/src/auth/html.rs", 2usize),
            ("notification_service/src/routes.rs", 1usize),
        ]
        .iter()
        .for_each(|(path_suffix, expected_registry_count)| {
            let source = snapshot
                .rs_files()
                .iter()
                .find(|file| file.path().as_ref().ends_with(path_suffix))
                .expect("249edc4a service_route_handler_composition_uses_shared_registries invariant must hold")
                .content()
                .as_ref();
            assert_eq!(
                source
                    .matches("frontend_contract::handler_registry")
                    .count(),
                *expected_registry_count,
                "26aa4162"
            );
            assert!(!source.contains(".route("), "71f23fd6");
        });
    });
}
#[test]
#[allow(clippy::needless_for_each)] // iterator form is required by the workspace no-for-loop policy
fn typed_route_registries_own_request_bodies_and_schema_catalogs() {
    super::snapshot::with_codebase_snapshot(|snapshot| {
        [
            "server_admin/src/auth/routes.rs",
            "notification_service/src/routes.rs",
        ]
        .iter()
        .for_each(|path_suffix| {
            let source = snapshot
                .rs_files()
                .iter()
                .find(|file| file.path().as_ref().ends_with(path_suffix))
                .expect("a63a8d31 typed_route_registries_own_request_bodies_and_schema_catalogs invariant must hold")
                .content()
                .as_ref();
            assert!(!source.contains("components(schemas"), "94cc9de1");
        });
        [
            "server_admin/src/auth.rs",
            "notification_service/src/main.rs",
        ]
        .iter()
        .for_each(|path_suffix| {
            let source = snapshot
                .rs_files()
                .iter()
                .find(|file| file.path().as_ref().ends_with(path_suffix))
                .expect("5bde3d5c typed_route_registries_own_request_bodies_and_schema_catalogs invariant must hold")
                .content()
                .as_ref();
            assert!(!source.contains("request_body ="), "95cc867b");
        });
        [
            "server_admin_contract/src/lib.rs",
            "notification_service_contract/src/lib.rs",
        ]
        .iter()
        .for_each(|path_suffix| {
            let source = snapshot
                .rs_files()
                .iter()
                .find(|file| file.path().as_ref().ends_with(path_suffix))
                .expect("d07be29f typed_route_registries_own_request_bodies_and_schema_catalogs invariant must hold")
                .content()
                .as_ref();
            assert!(!source.contains("error_statuses ="), "5a8ed6cf");
        });
    });
}
#[test]
#[allow(clippy::needless_for_each)] // iterator form is required by the workspace no-for-loop policy
fn generated_admin_table_consumers_use_the_shared_catalog() {
    super::snapshot::with_codebase_snapshot(|snapshot| {
        [
            (
                "server_admin/src/generated_auth.rs",
                "AdminRolesRouteContract",
            ),
            (
                "server_admin/src/repository/data_tables.rs",
                "AdminRoles::frontend_fields",
            ),
            (
                "server_admin/src/repository/data_tables.rs",
                "AdminRoles::frontend_filter_value",
            ),
        ]
        .iter()
        .for_each(|(path_suffix, forbidden)| {
            let source = snapshot
                .rs_files()
                .iter()
                .find(|file| file.path().as_ref().ends_with(path_suffix))
                .expect("94a2f8c1 generated_admin_table_consumers_use_the_shared_catalog invariant must hold")
                .content()
                .as_ref();
            assert!(!source.contains(forbidden), "8b137dd2");
            assert!(source.contains("AdminGeneratedTable"), "e1c82f79");
        });
        let server_routing = snapshot
            .rs_files()
            .iter()
            .find(|file| file.path().as_ref().ends_with("server/src/routing.rs"))
            .expect("148223ec generated_admin_table_consumers_use_the_shared_catalog invariant must hold")
            .content()
            .as_ref();
        [
            "AdminRoles::routes",
            "AdminRolePermissions::routes",
            "AdminUsers::routes",
            "AdminPermissions::routes",
            "AdminSystemSettings::routes",
            "AdminUserRoles::routes",
        ]
        .iter()
        .for_each(|forbidden| {
            assert!(!server_routing.contains(forbidden), "d01c1dd0");
        });
        assert!(
            server_routing.contains("generated_tables::generated_routes"),
            "af786d19"
        );
        let admin_api = snapshot
            .rs_files()
            .iter()
            .filter(|file| {
                let path = file.path().as_ref().to_string_lossy();
                path.ends_with("server_admin/tests/admin_api.rs")
                    || path.contains("server_admin/tests/admin_api/")
            })
            .map(|file| file.content().as_ref())
            .collect::<Vec<&str>>()
            .join("\n");
        assert!(!admin_api.is_empty(), "e26d929b");
        [
            "validate_generated_admin_table::<",
            "validate_generated_admin_table_in_schema::<",
        ]
        .iter()
        .for_each(|forbidden| {
            assert!(!admin_api.contains(forbidden), "535813a1");
        });
        assert!(
            admin_api.contains("generated_tables::validate_catalog_schema"),
            "de411cae"
        );
    });
}
#[test]
#[allow(clippy::needless_for_each)] // iterator form is required by the workspace no-for-loop policy
fn administrator_data_table_queries_come_from_the_typed_spec() {
    super::snapshot::with_codebase_snapshot(|snapshot| {
        let repository = snapshot
            .rs_files()
            .iter()
            .find(|file| {
                file.path()
                    .as_ref()
                    .ends_with("server_admin/src/repository/data_tables.rs")
            })
            .expect("3ac24886 administrator_data_table_queries_come_from_the_typed_spec invariant must hold")
            .content()
            .as_ref();
        let admin_api = snapshot
            .rs_files()
            .iter()
            .filter(|file| {
                let path = file.path().as_ref().to_string_lossy();
                path.ends_with("server_admin/tests/admin_api.rs")
                    || path.contains("server_admin/tests/admin_api/")
            })
            .map(|file| file.content().as_ref())
            .collect::<Vec<&str>>()
            .join("\n");
        assert!(!admin_api.is_empty(), "1049d34b");
        [
            "SERVER_ADMIN_DATA_USERS_SQL",
            "SERVER_ADMIN_DATA_COUNT_ACCESS_SESSIONS_SQL",
            "SERVER_ADMIN_DATA_USERS_COLUMNS",
        ]
        .iter()
        .for_each(|legacy_source| {
            assert!(!repository.contains(legacy_source), "7012056f");
            assert!(!admin_api.contains(legacy_source), "36154b24");
        });
        assert!(repository.contains("let spec = table.spec()"), "67b8279d");
        assert!(admin_api.contains("table.spec().columns()"), "92c41cb0");
    });
}
#[test]
fn administrator_csr_page_behavior_comes_from_the_page_catalog() {
    super::snapshot::with_codebase_snapshot(|snapshot| {
        let query = snapshot
            .rs_files()
            .iter()
            .find(|file| {
                file.path()
                    .as_ref()
                    .ends_with("server_admin_frontend/src/app/query/page.rs")
            })
            .expect("58e2110e administrator_csr_page_behavior_comes_from_the_page_catalog invariant must hold")
            .content()
            .as_ref();
        let loader = snapshot
            .rs_files()
            .iter()
            .find(|file| {
                file.path()
                    .as_ref()
                    .ends_with("server_admin_frontend/src/app/loader.rs")
            })
            .expect("04bb78af administrator_csr_page_behavior_comes_from_the_page_catalog invariant must hold")
            .content()
            .as_ref();
        assert!(!query.contains("AdminCsrPage"), "438888fd");
        assert!(query.contains("page.supports_csr()"), "d3ec99c6");
        assert!(loader.contains("page.uses_table_query()"), "256ac244");
        assert!(loader.contains("page.spec().route()"), "fe0906a9");
        let pages = snapshot
            .rs_files()
            .iter()
            .find(|file| {
                file.path()
                    .as_ref()
                    .ends_with(str_constants::SERVER_ADMIN_FRONTEND_SRC_APP_SETTINGS_RS)
            })
            .expect("2f3afe52 administrator_csr_page_behavior_comes_from_the_page_catalog invariant must hold")
            .content()
            .as_ref();
        let ssr = snapshot
            .rs_files()
            .iter()
            .find(|file| {
                file.path()
                    .as_ref()
                    .ends_with(str_constants::SERVER_ADMIN_FRONTEND_SRC_SSR_SETTINGS_RS)
            })
            .expect("2c589b2b administrator_csr_page_behavior_comes_from_the_page_catalog invariant must hold")
            .content()
            .as_ref();
        assert!(
            pages.contains("shared::settings::values::AdminSettingsFormValues::from"),
            "3ca65c5b"
        );
        assert!(
            ssr.contains("shared::settings::values::AdminSettingsFormValues::from"),
            "9f904035"
        );
        assert!(!pages.contains("page.main_logo()"), "67c3d270");
        assert!(!ssr.contains("view.main_logo()"), "6201410d");
    });
}
#[test]
fn config_reference_getters_use_generated_forwarding() {
    super::snapshot::with_codebase_snapshot(|snapshot| {
        let source = snapshot
            .rs_files()
            .iter()
            .find(|file| file.path().as_ref().ends_with("server_config/src/lib.rs"))
            .expect(
                "e210ffd6 config_reference_getters_use_generated_forwarding invariant must hold",
            )
            .content()
            .as_ref();
        assert!(!source.contains(" for &Config"), "c0f0354a");
    });
}
