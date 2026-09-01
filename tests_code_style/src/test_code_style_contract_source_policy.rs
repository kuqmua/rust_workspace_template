fn publicly_forwards_crate_root(item: &syn::Item) -> bool {
    let syn::Item::Use(item_use) = item else {
        return false;
    };
    matches!(item_use.vis, syn::Visibility::Public(_))
        && matches!(
            &item_use.tree,
            syn::UseTree::Path(path) if path.ident == constants_str::CRATE
        )
}
#[test]
fn test_private_shared_modules_do_not_forward_crate_root_exports() {
    super::test_code_style_snapshot::with_codebase_snapshot(|snapshot| {
        let errors = snapshot
            .rs_files()
            .iter()
            .filter(|source_file| {
                source_file.path().as_ref().file_name()
                    == Some(std::ffi::OsStr::new(constants_str::VALUE_0544FC95))
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
                                .join(constants_str::VALUE_07642C44),
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
fn test_private_shared_module_forwarding_policy_distinguishes_public_visibility_and_owner() {
    let public_forward =
        syn::parse_file(constants_str::VALUE_5C907704).expect(constants_str::DIAGNOSTIC_B2D1E940);
    let crate_forward =
        syn::parse_file(constants_str::VALUE_9388C05D).expect(constants_str::DIAGNOSTIC_53F91AC7);
    let local_public =
        syn::parse_file(constants_str::VALUE_E40DBB0F).expect(constants_str::DIAGNOSTIC_9A47E2C6);
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
fn test_admin_frontend_api_urls_come_from_typed_routes() {
    super::test_code_style_snapshot::with_codebase_snapshot(|snapshot| {
        let source = snapshot
            .rs_files()
            .iter()
            .filter(|file| {
                file.path()
                    .as_ref()
                    .to_string_lossy()
                    .contains(constants_str::VALUE_BC9DA9CE)
            })
            .map(|file| file.content().as_ref())
            .collect::<String>();
        assert!(!source.contains("constants_str::V1"), "24e5ceeb");
        assert!(!source.contains("ADMIN_API_"), "72b66898");
    });
}
#[test]
#[allow(clippy::needless_for_each)] // iterator form is required by the workspace no-for-loop policy
fn test_service_route_endpoint_composition_uses_shared_registries() {
    super::test_code_style_snapshot::with_codebase_snapshot(|snapshot| {
        [
            (constants_str::VALUE_3EB7B056, constants_usize::EIGHT),
            (constants_str::VALUE_629EE5ED, constants_usize::ONE),
        ]
        .iter()
        .for_each(|(path_suffix, expected_registry_count)| {
            let source = snapshot
                .rs_files()
                .iter()
                .filter(|file| {
                    file.path().as_ref().ends_with(path_suffix)
                        || crate::code_style::declared_child_matches(
                            file.path().as_ref().to_string_lossy().as_ref(),
                            path_suffix,
                        )
                        || (*path_suffix == constants_str::VALUE_3EB7B056
                            && file
                                .path()
                                .as_ref()
                                .to_string_lossy()
                                .contains(constants_str::SERVER_ADMIN_HTML_MODULE_DIR))
                })
                .map(|file| file.content().as_ref())
                .collect::<String>();
            assert_eq!(
                source
                    .matches("frontend_contract_macros::endpoint_registry")
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
fn test_typed_route_registries_own_request_bodies_and_schema_catalogs() {
    super::test_code_style_snapshot::with_codebase_snapshot(|snapshot| {
        [constants_str::VALUE_7BF90B7C, constants_str::VALUE_629EE5ED]
            .iter()
            .for_each(|path_suffix| {
                let source = snapshot
                    .rs_files()
                    .iter()
                    .find(|file| file.path().as_ref().ends_with(path_suffix))
                    .expect(constants_str::DIAGNOSTIC_A63A8D31)
                    .content()
                    .as_ref();
                assert!(!source.contains("components(schemas"), "94cc9de1");
            });
        std::iter::once(&constants_str::VALUE_0690A45F).for_each(|path_suffix| {
            let source = snapshot
                .rs_files()
                .iter()
                .find(|file| file.path().as_ref().ends_with(path_suffix))
                .expect(constants_str::DIAGNOSTIC_5BDE3D5C)
                .content()
                .as_ref();
            assert!(!source.contains("request_body ="), "95cc867b");
        });
        [constants_str::VALUE_AA6C3BC8, constants_str::VALUE_4DE86380]
            .iter()
            .for_each(|path_suffix| {
                let source = snapshot
                    .rs_files()
                    .iter()
                    .find(|file| file.path().as_ref().ends_with(path_suffix))
                    .expect(constants_str::DIAGNOSTIC_D07BE29F)
                    .content()
                    .as_ref();
                assert!(!source.contains("error_statuses ="), "5a8ed6cf");
            });
    });
}
#[test]
#[allow(clippy::needless_for_each)] // iterator form is required by the workspace no-for-loop policy
fn test_generated_admin_table_consumers_use_the_shared_catalog() {
    super::test_code_style_snapshot::with_codebase_snapshot(|snapshot| {
        [
            (constants_str::VALUE_206B48D7, constants_str::VALUE_41EC3410),
            (constants_str::VALUE_8E182ED1, constants_str::VALUE_D6BB9F39),
            (constants_str::VALUE_8E182ED1, constants_str::VALUE_78CE6024),
        ]
        .iter()
        .for_each(|(path_suffix, forbidden)| {
            let source = snapshot
                .rs_files()
                .iter()
                .filter(|file| {
                    let path = file.path().as_ref().to_string_lossy();
                    path.ends_with(path_suffix)
                        || crate::code_style::declared_child_matches(path.as_ref(), path_suffix)
                })
                .map(|file| file.content().as_ref())
                .collect::<Vec<&str>>()
                .join(constants_str::NEWLINE);
            assert!(!source.is_empty(), "94a2f8c1");
            assert!(!source.contains(forbidden), "8b137dd2");
            assert!(source.contains("AdminGeneratedTable"), "e1c82f79");
        });
        let server_application = snapshot
            .rs_files()
            .iter()
            .find(|file| {
                file.path()
                    .as_ref()
                    .ends_with(constants_str::SERVER_SRC_APPLICATION_ADMIN_API_RS)
            })
            .expect(constants_str::DIAGNOSTIC_148223EC)
            .content()
            .as_ref();
        [
            constants_str::VALUE_D6456971,
            constants_str::VALUE_1788D397,
            constants_str::VALUE_F3C1108D,
            constants_str::VALUE_1A8BCD41,
            constants_str::VALUE_3C94AF87,
            constants_str::VALUE_639F76CD,
        ]
        .iter()
        .for_each(|forbidden| {
            assert!(!server_application.contains(forbidden), "d01c1dd0");
        });
        assert!(
            server_application.contains("server_admin::generated_routes::generated_routes"),
            "af786d19"
        );
        let admin_api = snapshot
            .rs_files()
            .iter()
            .filter(|file| {
                let path = file.path().as_ref().to_string_lossy();
                path.ends_with(constants_str::VALUE_88607159)
                    || path.contains(constants_str::VALUE_B51C3727)
            })
            .map(|file| file.content().as_ref())
            .collect::<Vec<&str>>()
            .join(constants_str::NEWLINE);
        assert!(!admin_api.is_empty(), "e26d929b");
        [constants_str::VALUE_CEB9FEF2, constants_str::VALUE_632E5011]
            .iter()
            .for_each(|forbidden| {
                assert!(!admin_api.contains(forbidden), "535813a1");
            });
        assert!(
            admin_api.contains("server_admin::validate_catalog_schema::validate_catalog_schema"),
            "de411cae"
        );
    });
}
#[test]
#[allow(clippy::needless_for_each)] // iterator form is required by the workspace no-for-loop policy
fn test_administrator_data_table_queries_come_from_the_typed_spec() {
    super::test_code_style_snapshot::with_codebase_snapshot(|snapshot| {
        let repository = snapshot
            .rs_files()
            .iter()
            .filter(|file| {
                let path = file.path().as_ref().to_string_lossy();
                path.ends_with(constants_str::VALUE_8E182ED1)
                    || crate::code_style::declared_child_matches(
                        path.as_ref(),
                        constants_str::VALUE_8E182ED1,
                    )
            })
            .map(|file| file.content().as_ref())
            .collect::<Vec<&str>>()
            .join(constants_str::NEWLINE);
        assert!(!repository.is_empty(), "3ac24886");
        let admin_api = snapshot
            .rs_files()
            .iter()
            .filter(|file| {
                let path = file.path().as_ref().to_string_lossy();
                path.ends_with(constants_str::VALUE_88607159)
                    || path.contains(constants_str::VALUE_B51C3727)
            })
            .map(|file| file.content().as_ref())
            .collect::<Vec<&str>>()
            .join(constants_str::NEWLINE);
        assert!(!admin_api.is_empty(), "1049d34b");
        [
            constants_str::VALUE_F7BEC314,
            constants_str::VALUE_73522C89,
            constants_str::VALUE_01C2291C,
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
fn test_administrator_csr_page_behavior_comes_from_the_page_catalog() {
    super::test_code_style_snapshot::with_codebase_snapshot(|snapshot| {
        let query = snapshot
            .rs_files()
            .iter()
            .find(|file| {
                file.path()
                    .as_ref()
                    .ends_with(constants_str::VALUE_BEBEC57E)
            })
            .expect(constants_str::DIAGNOSTIC_58E2110E)
            .content()
            .as_ref();
        let loader = snapshot
            .rs_files()
            .iter()
            .find(|file| {
                file.path()
                    .as_ref()
                    .ends_with(constants_str::VALUE_27AB06E9)
            })
            .expect(constants_str::DIAGNOSTIC_04BB78AF)
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
                    .ends_with(constants_str::SERVER_ADMIN_FRONTEND_SRC_APP_SETTINGS_RS)
            })
            .expect(constants_str::DIAGNOSTIC_2F3AFE52)
            .content()
            .as_ref();
        let ssr = snapshot
            .rs_files()
            .iter()
            .find(|file| {
                file.path()
                    .as_ref()
                    .ends_with(constants_str::SERVER_ADMIN_FRONTEND_SRC_SSR_SETTINGS_RS)
            })
            .expect(constants_str::DIAGNOSTIC_2C589B2B)
            .content()
            .as_ref();
        assert!(
            pages.contains("crate::admin_settings_form_values::AdminSettingsFormValues::from"),
            "3ca65c5b"
        );
        assert!(
            ssr.contains("crate::admin_settings_form_values::AdminSettingsFormValues::from"),
            "9f904035"
        );
        assert!(!pages.contains("page.main_logo()"), "67c3d270");
        assert!(!ssr.contains("view.main_logo()"), "6201410d");
    });
}
#[test]
fn test_config_reference_accessors_use_generated_forwarding() {
    super::test_code_style_snapshot::with_codebase_snapshot(|snapshot| {
        let source = snapshot
            .rs_files()
            .iter()
            .find(|file| {
                file.path()
                    .as_ref()
                    .ends_with(constants_str::VALUE_D31B3088)
            })
            .expect(constants_str::DIAGNOSTIC_E210FFD6)
            .content()
            .as_ref();
        assert!(!source.contains(" for &Config"), "c0f0354a");
    });
}
