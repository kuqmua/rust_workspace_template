#[test]
fn test_authenticated_admin_checks_owned_rules() {
    let admin = crate::authenticated_admin::AuthenticatedAdmin::new(
        crate::admin_display_name::AdminDisplayName::try_from(constants_str::ADMIN.to_owned())
            .expect(constants_str::DIAGNOSTIC_68E94B2F),
        crate::admin_user_id::AdminUserId::try_from(constants_i64::ONE)
            .expect(constants_str::DIAGNOSTIC_134F7A9C),
        crate::admin_login::AdminLogin::try_from(constants_str::ROOT.to_owned())
            .expect(constants_str::DIAGNOSTIC_971C5E42),
        crate::admin_rule_values::AdminRuleValues::try_from(vec![
            crate::admin_rule_value::AdminRuleValue::try_from(
                crate::admin_rule::AdminRule::UsersRead
                    .as_str()
                    .get()
                    .to_owned(),
            )
            .expect(constants_str::DIAGNOSTIC_8BF39D41),
            crate::admin_rule_value::AdminRuleValue::try_from(
                crate::admin_rule::AdminRule::RulesRead
                    .as_str()
                    .get()
                    .to_owned(),
            )
            .expect(constants_str::VALUE_785335E9),
        ])
        .expect(constants_str::DIAGNOSTIC_BD2806F1),
        crate::admin_role_names::AdminRoleNames::try_from(Vec::new())
            .expect(constants_str::DIAGNOSTIC_763AE20C),
    );
    assert!(bool::from(
        admin.has_rule(crate::admin_rule::AdminRule::UsersRead)
    ));
    assert!(!bool::from(
        admin.has_rule(crate::admin_rule::AdminRule::UsersUpdate)
    ));
    assert!(bool::from(
        admin.can_access(crate::admin_page::AdminPage::Users)
    ));
    assert!(!bool::from(
        admin.can_access(crate::admin_page::AdminPage::Roles)
    ));
    assert!(bool::from(
        admin.can_access(crate::admin_page::AdminPage::Rules)
    ));
    assert!(bool::from(
        admin.can_access(crate::admin_page::AdminPage::Profile)
    ));
}

#[test]
fn test_change_own_password_has_no_session_revocation_choice() {
    let request = crate::admin_change_own_password_request::AdminChangeOwnPasswordRequest::new(
        crate::admin_password::AdminPassword::try_from(String::from(constants_str::VALUE_A1AB879D))
            .expect(constants_str::DIAGNOSTIC_C10E4DB7),
        crate::admin_new_password::AdminNewPassword::try_from(String::from(
            constants_str::VALUE_05A7131F,
        ))
        .expect(constants_str::DIAGNOSTIC_5932A1FE),
    );
    let json = serde_json::to_value(request).expect(constants_str::DIAGNOSTIC_06BA3EF9);
    assert_eq!(
        json,
        serde_json::json!({
            "current_password": "Current-password1",
            "new_password": "New-password2",
        })
    );
    assert!(matches!(
        serde_json::from_value::<
            crate::admin_change_own_password_request::AdminChangeOwnPasswordRequest,
        >(json)
        .map(drop),
        Ok(()),
    ));
    let Err(serde_json_error) = serde_json::from_str::<
        crate::admin_change_own_password_request::AdminChangeOwnPasswordRequest,
    >(constants_str::VALUE_4A4AAF28) else {
        std::panic::panic_any(constants_str::PANIC_ABAA9CDF);
    };
    assert!(serde_json_error.is_data());
    assert!(
        serde_json_error
            .to_string()
            .contains(stringify!(revoke_other_sessions))
    );
}

#[test]
fn test_passwords_are_redacted_and_share_policy() {
    let password =
        crate::admin_password::AdminPassword::try_from(String::from(constants_str::SECRET))
            .expect(constants_str::DIAGNOSTIC_9F3F5164);
    assert!(!format!("{password:?}").contains(constants_str::SECRET));
    let _new_password = crate::admin_new_password::AdminNewPassword::try_from(
        constants_str::TEST_STRONG_PASSWORD.to_owned(),
    )
    .expect(constants_str::DIAGNOSTIC_DA19950B);
    let Err(_weak_password_error) =
        crate::admin_new_password::AdminNewPassword::try_from(constants_str::PASSWORD.to_owned())
    else {
        std::panic::panic_any(constants_str::PANIC_24900F2F);
    };
}

#[test]
fn test_sign_in_accepts_only_login_and_password() {
    let basic = serde_json::json!({
        "login": "admin",
        "password": "correct_password"
    });
    let Ok(_basic_request) =
        serde_json::from_value::<crate::admin_sign_in_request::AdminSignInRequest>(basic)
    else {
        std::panic::panic_any(constants_str::PANIC_AF47412D);
    };
    let legacy_mfa = serde_json::json!({
        "login": "admin",
        "mfa_proof": { "kind": "totp", "value": "123456" },
        "password": "correct_password"
    });
    let Err(_legacy_mfa_error) =
        serde_json::from_value::<crate::admin_sign_in_request::AdminSignInRequest>(legacy_mfa)
    else {
        std::panic::panic_any(constants_str::PANIC_89071E97);
    };
}

#[test]
fn test_domain_values_follow_database_compatible_policies() {
    let _valid_login =
        crate::admin_login::AdminLogin::try_from(constants_str::ADMIN_USER_1.to_owned())
            .expect(constants_str::DIAGNOSTIC_E1CDDEBC);
    let Err(_reserved_login_error) =
        crate::admin_login::AdminLogin::try_from(constants_str::ADMIN.to_owned())
    else {
        std::panic::panic_any(constants_str::PANIC_AB23C76E);
    };
    let Err(_short_login_error) =
        crate::admin_login::AdminLogin::try_from(constants_str::AB.to_owned())
    else {
        std::panic::panic_any(constants_str::PANIC_CE5B9E72);
    };
    let _valid_display_name =
        crate::admin_display_name::AdminDisplayName::try_from(constants_str::ADMIN.to_owned())
            .expect(constants_str::DIAGNOSTIC_D315B74F);
    let Err(_blank_display_name_error) =
        crate::admin_display_name::AdminDisplayName::try_from(constants_str::SPACE.to_owned())
    else {
        std::panic::panic_any(constants_str::PANIC_1CCD43AA);
    };
    let _valid_role_name =
        crate::admin_role_name::AdminRoleName::try_from(constants_str::ADMIN_ALT.to_owned())
            .expect(constants_str::DIAGNOSTIC_713890E9);
    let Err(_reserved_role_name_error) =
        crate::admin_role_name::AdminRoleName::try_from(constants_str::ADMIN.to_owned())
    else {
        std::panic::panic_any(constants_str::PANIC_147FE35A);
    };
}

#[test]
fn test_authenticated_admin_borrowed_permissions_preserve_order_and_duplicates() {
    assert!([false, true].into_iter().all(|populated| {
        let rules = if populated {
            serde_json::json!([
                crate::admin_rule::AdminRule::RulesRead,
                crate::admin_rule::AdminRule::UsersRead,
                crate::admin_rule::AdminRule::RulesRead
            ])
        } else {
            serde_json::json!([])
        };
        let roles = if populated {
            serde_json::json!([
                constants_str::LOGIN,
                constants_str::ROOT,
                constants_str::LOGIN
            ])
        } else {
            serde_json::json!([])
        };
        let administrator_result = serde_json::from_value::<
            crate::authenticated_admin::AuthenticatedAdmin,
        >(serde_json::json!({
            (stringify!(display_name)): constants_str::ADMIN,
            (stringify!(id)): 1i64,
            (stringify!(login)): constants_str::ROOT,
            (stringify!(rules)): rules,
            (stringify!(roles)): roles
        }));
        assert!(administrator_result.is_ok());
        let Ok(administrator) = administrator_result else {
            return false;
        };
        assert!(serde_json::to_value(administrator.rules()).is_ok_and(|wire| wire == rules));
        assert!(serde_json::to_value(administrator.roles()).is_ok_and(|wire| wire == roles));
        [
            crate::admin_rule::AdminRule::AccessSessionsDelete,
            crate::admin_rule::AdminRule::AccessSessionsRead,
            crate::admin_rule::AdminRule::AuditLogRead,
            crate::admin_rule::AdminRule::CleanupStatusRead,
            crate::admin_rule::AdminRule::LoginAttemptsRead,
            crate::admin_rule::AdminRule::MetricsRead,
            crate::admin_rule::AdminRule::OpenApiRead,
            crate::admin_rule::AdminRule::PermissionActionsRead,
            crate::admin_rule::AdminRule::PermissionResourceActionsRead,
            crate::admin_rule::AdminRule::PermissionResourcesRead,
            crate::admin_rule::AdminRule::RulesRead,
            crate::admin_rule::AdminRule::RateLimitsRead,
            crate::admin_rule::AdminRule::RefreshTokensRead,
            crate::admin_rule::AdminRule::RoleRulesCreate,
            crate::admin_rule::AdminRule::RoleRulesDelete,
            crate::admin_rule::AdminRule::RoleRulesRead,
            crate::admin_rule::AdminRule::RoleRulesUpdate,
            crate::admin_rule::AdminRule::RolesCreate,
            crate::admin_rule::AdminRule::RolesDelete,
            crate::admin_rule::AdminRule::RolesRead,
            crate::admin_rule::AdminRule::RolesUpdate,
            crate::admin_rule::AdminRule::SystemSettingsRead,
            crate::admin_rule::AdminRule::SystemSettingsUpdate,
            crate::admin_rule::AdminRule::TablesRead,
            crate::admin_rule::AdminRule::UserRolesCreate,
            crate::admin_rule::AdminRule::UserRolesDelete,
            crate::admin_rule::AdminRule::UserRolesRead,
            crate::admin_rule::AdminRule::UserRolesUpdate,
            crate::admin_rule::AdminRule::UsersCreate,
            crate::admin_rule::AdminRule::UsersDelete,
            crate::admin_rule::AdminRule::UsersRead,
            crate::admin_rule::AdminRule::UsersUpdate,
        ]
        .into_iter()
        .all(|admin_rule| {
            let expected = populated
                && matches!(
                    admin_rule,
                    crate::admin_rule::AdminRule::RulesRead
                        | crate::admin_rule::AdminRule::UsersRead
                );
            administrator.has_rule(admin_rule) == crate::admin_bool::AdminBool::from(expected)
        })
    }));
}

#[test]
fn test_sign_in_request_into_parts_preserves_login_and_password_ownership() {
    let expected = serde_json::json!({
        (stringify!(login)): constants_str::ROOT,
        (stringify!(password)): constants_str::VALUE_A1AB879D
    });
    let request_result = serde_json::from_value::<crate::admin_sign_in_request::AdminSignInRequest>(
        expected.clone(),
    );
    assert!(request_result.is_ok());
    let Ok(request) = request_result else {
        return;
    };
    let (admin_login, admin_password) = request.into_parts();
    let rebuilt_request =
        crate::admin_sign_in_request::AdminSignInRequest::new(admin_login, admin_password);
    assert!(serde_json::to_value(rebuilt_request).is_ok_and(|wire| wire == expected));
}

#[test]
fn test_user_summary_accepts_every_plain_and_selected_field_combination() {
    let expected = serde_json::json!({
        (stringify!(display_name)): constants_str::ADMIN,
        (stringify!(id)): 7i64,
        (stringify!(is_banned)): true,
        (stringify!(login)): constants_str::LOGIN,
        (stringify!(role_ids)): [],
    });
    assert!((0u8..16u8).all(|mask| {
        let fields = [
            (
                stringify!(display_name),
                serde_json::json!(constants_str::ADMIN),
            ),
            (stringify!(id), serde_json::json!(7i64)),
            (stringify!(is_banned), serde_json::json!(true)),
            (stringify!(login), serde_json::json!(constants_str::LOGIN)),
        ]
        .into_iter()
        .enumerate()
        .map(|(index, (name, field))| {
            let value = if mask & (1u8 << index) == 0u8 {
                field
            } else {
                serde_json::json!({(stringify!(value)): field})
            };
            (name.to_owned(), value)
        })
        .collect::<serde_json::Map<String, serde_json::Value>>();
        serde_json::from_value::<crate::admin_user_summary::AdminUserSummary>(
            serde_json::Value::Object(fields),
        )
        .is_ok_and(|admin_user_summary| {
            serde_json::to_value(admin_user_summary).is_ok_and(|actual| actual == expected)
        })
    }));
}

#[test]
fn test_user_summary_rejects_malformed_selected_field_values() {
    assert!([
        (stringify!(display_name), serde_json::json!({})),
        (stringify!(display_name), serde_json::json!({(stringify!(value)): constants_str::EMPTY})),
        (stringify!(id), serde_json::json!({(stringify!(value)): 0i64})),
        (stringify!(id), serde_json::json!({(stringify!(value)): null})),
        (stringify!(is_banned), serde_json::json!({(stringify!(value)): 1u8})),
        (stringify!(login), serde_json::json!({(stringify!(value)): constants_str::ADMIN})),
        (stringify!(login), serde_json::json!({(stringify!(value)): {(stringify!(value)): constants_str::LOGIN}})),
    ].into_iter().all(|(name, field)| {
        let mut input = serde_json::json!({
            (stringify!(display_name)): constants_str::ADMIN,
            (stringify!(id)): 7i64,
            (stringify!(is_banned)): true,
            (stringify!(login)): constants_str::LOGIN,
        });
        let Some(value) = input.get_mut(name) else {
            return false;
        };
        *value = field;
        serde_json::from_value::<crate::admin_user_summary::AdminUserSummary>(input)
            .is_err_and(|error| error.is_data())
    }));
}

#[test]
fn test_authenticated_admin_page_access_preserves_exact_rule_requirements() {
    [
        None,
        Some(crate::admin_rule::AdminRule::UsersRead),
        Some(crate::admin_rule::AdminRule::RolesRead),
        Some(crate::admin_rule::AdminRule::RulesRead),
        Some(crate::admin_rule::AdminRule::SystemSettingsRead),
        Some(crate::admin_rule::AdminRule::TablesRead),
        Some(crate::admin_rule::AdminRule::MetricsRead),
        Some(crate::admin_rule::AdminRule::OpenApiRead),
        Some(crate::admin_rule::AdminRule::UsersUpdate),
    ]
    .into_iter()
    .fold((), |(), granted| {
        let rules = granted
            .into_iter()
            .flat_map(|rule| [rule, rule])
            .collect::<Vec<_>>();
        let result = serde_json::from_value::<crate::authenticated_admin::AuthenticatedAdmin>(
            serde_json::json!({
                (stringify!(display_name)): constants_str::ADMIN,
                (stringify!(id)): 1i64,
                (stringify!(login)): constants_str::ROOT,
                (stringify!(rules)): rules,
                (stringify!(roles)): [constants_str::LOGIN],
            }),
        );
        assert!(result.as_ref().err().is_none());
        if let Ok(administrator) = result {
            let expected = [
                (crate::admin_page::AdminPage::Health, None),
                (crate::admin_page::AdminPage::Branding, None),
                (
                    crate::admin_page::AdminPage::Users,
                    Some(crate::admin_rule::AdminRule::UsersRead),
                ),
                (
                    crate::admin_page::AdminPage::Roles,
                    Some(crate::admin_rule::AdminRule::RolesRead),
                ),
                (
                    crate::admin_page::AdminPage::Rules,
                    Some(crate::admin_rule::AdminRule::RulesRead),
                ),
                (
                    crate::admin_page::AdminPage::Settings,
                    Some(crate::admin_rule::AdminRule::SystemSettingsRead),
                ),
                (
                    crate::admin_page::AdminPage::Tables,
                    Some(crate::admin_rule::AdminRule::TablesRead),
                ),
                (crate::admin_page::AdminPage::Sessions, None),
                (
                    crate::admin_page::AdminPage::Metrics,
                    Some(crate::admin_rule::AdminRule::MetricsRead),
                ),
                (crate::admin_page::AdminPage::Version, None),
                (crate::admin_page::AdminPage::Profile, None),
                (
                    crate::admin_page::AdminPage::OpenApi,
                    Some(crate::admin_rule::AdminRule::OpenApiRead),
                ),
            ];
            assert_eq!(crate::admin_page::AdminPage::all().count(), expected.len());
            assert!(
                expected
                    .into_iter()
                    .all(|(page, required)| administrator.can_access(page)
                        == crate::admin_bool::AdminBool::from(
                            required.is_none_or(|rule| Some(rule) == granted)
                        ))
            );
            assert_eq!(
                administrator.rules().len(),
                if granted.is_some() { 2usize } else { 0usize }
            );
        }
    });
}

#[test]
fn test_sign_in_response_preserves_authenticated_user_and_wire_shape() {
    let user_value = serde_json::json!({
        (stringify!(display_name)): constants_str::ADMIN,
        (stringify!(id)): 7i64,
        (stringify!(login)): constants_str::ROOT,
        (stringify!(rules)): [crate::admin_rule::AdminRule::UsersRead, crate::admin_rule::AdminRule::RulesRead],
        (stringify!(roles)): [constants_str::ADMIN_ALT, constants_str::ROOT],
    });
    assert!(
        serde_json::from_value::<crate::authenticated_admin::AuthenticatedAdmin>(
            user_value.clone()
        )
        .is_ok_and(|user| {
            let response = crate::admin_sign_in_response::AdminSignInResponse::new(user);
            serde_json::to_value(response.user()).is_ok_and(|value| value == user_value)
                && serde_json::to_value(response).is_ok_and(|wire| {
                    let expected = serde_json::json!({(stringify!(user)): user_value});
                    wire == expected
                        && serde_json::from_value::<
                            crate::admin_sign_in_response::AdminSignInResponse,
                        >(wire)
                        .is_ok_and(|decoded| {
                            serde_json::to_value(decoded).is_ok_and(|value| value == expected)
                        })
                })
        })
    );
    assert!(
        [
            serde_json::json!({}),
            serde_json::json!({(stringify!(user)): null}),
            serde_json::json!({(stringify!(user)): true})
        ]
        .into_iter()
        .all(|value| serde_json::from_value::<
            crate::admin_sign_in_response::AdminSignInResponse,
        >(value)
        .is_err_and(|error| error.is_data()))
    );
}

#[test]
fn test_create_user_response_preserves_identifier_and_wire_shape() {
    assert!([1i64, i64::MAX].into_iter().all(|value| {
        crate::admin_user_id::AdminUserId::try_from(value).is_ok_and(|id| {
            let response = crate::admin_create_user_response::AdminCreateUserResponse::new(id);
            serde_json::to_value(response).is_ok_and(|wire| {
                let expected = serde_json::json!({(stringify!(id)): value});
                wire == expected
                    && serde_json::from_value::<
                        crate::admin_create_user_response::AdminCreateUserResponse,
                    >(wire)
                    .is_ok_and(|decoded| {
                        serde_json::to_value(decoded).is_ok_and(|serialized| serialized == expected)
                    })
            })
        })
    }));
    assert!(
        [
            serde_json::json!({}),
            serde_json::json!({(stringify!(id)): null}),
            serde_json::json!({(stringify!(id)): 0i64}),
            serde_json::json!({(stringify!(id)): -1i64})
        ]
        .into_iter()
        .all(|value| serde_json::from_value::<
            crate::admin_create_user_response::AdminCreateUserResponse,
        >(value)
        .is_err_and(|error| error.is_data()))
    );
}

#[test]
fn test_users_and_roles_pages_preserve_distinct_items_roles_and_total() {
    let items_value = serde_json::json!([
        {(stringify!(display_name)): constants_str::ADMIN, (stringify!(id)): 7i64, (stringify!(is_banned)): true, (stringify!(login)): constants_str::LOGIN, (stringify!(role_ids)): [11i64, 13i64]},
        {(stringify!(display_name)): constants_str::ROOT, (stringify!(id)): 19i64, (stringify!(is_banned)): false, (stringify!(login)): constants_str::ROOT, (stringify!(role_ids)): [13i64]},
    ]);
    let roles_value = serde_json::json!([
        {(stringify!(id)): 13i64, (stringify!(is_system)): false, (stringify!(name)): constants_str::LOGIN, (stringify!(created_at)): constants_str::ADMIN_FIXTURE_AUDIT_CREATED_AT, (stringify!(updated_at)): constants_str::ADMIN_FIXTURE_SESSION_CREATED_AT},
        {(stringify!(id)): 11i64, (stringify!(is_system)): true, (stringify!(name)): constants_str::ROOT, (stringify!(created_at)): constants_str::ADMIN_FIXTURE_SESSION_CREATED_AT, (stringify!(updated_at)): constants_str::ADMIN_FIXTURE_AUDIT_CREATED_AT},
    ]);
    assert!(
        serde_json::from_value::<crate::admin_role_summaries::AdminRoleSummaries>(
            roles_value.clone()
        )
        .is_ok_and(|items| {
            let page = crate::admin_roles_page::AdminRolesPage::new(
                items,
                crate::admin_page_total::AdminPageTotal::from(71u64),
            );
            let expected =
                serde_json::json!({(stringify!(items)): roles_value, (stringify!(total)): 71u64});
            u64::from(page.total()) == 71u64
                && serde_json::to_value(page.items()).is_ok_and(|wire| wire == roles_value)
                && serde_json::to_value(&page).is_ok_and(|wire| {
                    wire == expected
                        && serde_json::from_value::<crate::admin_roles_page::AdminRolesPage>(wire)
                            .is_ok_and(|decoded| {
                                serde_json::to_value(decoded)
                                    .is_ok_and(|serialized| serialized == expected)
                            })
                })
                && serde_json::to_value(page.into_items()).is_ok_and(|wire| wire == roles_value)
        })
    );
    let collections = (
        serde_json::from_value::<crate::admin_user_summaries::AdminUserSummaries>(
            items_value.clone(),
        ),
        serde_json::from_value::<crate::admin_role_summaries::AdminRoleSummaries>(
            roles_value.clone(),
        ),
    );
    assert!(matches!(&collections, (Ok(_), Ok(_))));
    if let (Ok(items), Ok(roles)) = collections {
        let page = crate::admin_users_page::AdminUsersPage::new(
            items,
            roles,
            crate::admin_page_total::AdminPageTotal::from(71u64),
        );
        assert_eq!(u64::from(page.total()), 71u64);
        assert!(serde_json::to_value(page.items()).is_ok_and(|wire| wire == items_value));
        assert!(serde_json::to_value(page.roles()).is_ok_and(|wire| wire == roles_value));
        let expected = serde_json::json!({(stringify!(items)): items_value, (stringify!(roles)): roles_value, (stringify!(total)): 71u64});
        assert!(
            serde_json::to_value(&page).is_ok_and(|wire| wire == expected
                && serde_json::from_value::<crate::admin_users_page::AdminUsersPage>(wire)
                    .is_ok_and(|decoded| serde_json::to_value(decoded)
                        .is_ok_and(|serialized| serialized == expected)))
        );
        assert!(serde_json::to_value(page.into_items()).is_ok_and(|wire| wire == items_value));
    }
}

#[test]
fn test_rules_page_preserves_distinct_items_and_total() {
    let items_value = serde_json::json!([
        {(stringify!(id)): 13i64, (stringify!(name)): crate::admin_rule::AdminRule::UsersRead, (stringify!(created_at)): constants_str::ADMIN_FIXTURE_AUDIT_CREATED_AT},
        {(stringify!(id)): 11i64, (stringify!(name)): crate::admin_rule::AdminRule::RolesRead, (stringify!(created_at)): constants_str::ADMIN_FIXTURE_SESSION_CREATED_AT},
    ]);
    assert!(
        serde_json::from_value::<crate::admin_rule_summaries::AdminRuleSummaries>(
            items_value.clone()
        )
        .is_ok_and(|items| {
            let page = crate::admin_rules_page::AdminRulesPage::new(
                items,
                crate::admin_page_total::AdminPageTotal::from(71u64),
            );
            let expected =
                serde_json::json!({(stringify!(items)): items_value, (stringify!(total)): 71u64});
            u64::from(page.total()) == 71u64
                && serde_json::to_value(page.items()).is_ok_and(|wire| wire == items_value)
                && serde_json::to_value(&page).is_ok_and(|wire| {
                    wire == expected
                        && serde_json::from_value::<crate::admin_rules_page::AdminRulesPage>(wire)
                            .is_ok_and(|decoded| {
                                serde_json::to_value(decoded)
                                    .is_ok_and(|serialized| serialized == expected)
                            })
                })
                && serde_json::to_value(page.into_items()).is_ok_and(|wire| wire == items_value)
        })
    );
}

#[test]
fn test_page_total_preserves_default_and_unsigned_wire_boundaries() {
    assert_eq!(
        u64::from(crate::admin_page_total::AdminPageTotal::default()),
        0u64
    );
    assert!([0u64, 71u64, u64::MAX].into_iter().all(|count| {
        let total = crate::admin_page_total::AdminPageTotal::from(count);
        serde_json::to_value(total).is_ok_and(|wire| {
            wire == serde_json::json!(count)
                && serde_json::from_value::<crate::admin_page_total::AdminPageTotal>(wire)
                    .is_ok_and(|decoded| u64::from(decoded) == count)
        })
    }));
    assert!(
        [
            serde_json::json!(-1.0f64),
            serde_json::json!(0.0f64),
            serde_json::json!(1.0f64),
            serde_json::json!(1.5f64),
            serde_json::json!(100.0f64),
            serde_json::json!([]),
            serde_json::json!({}),
            serde_json::json!(-1i64),
            serde_json::json!(null),
            serde_json::json!(true),
            serde_json::json!(constants_str::X)
        ]
        .into_iter()
        .all(
            |wire| serde_json::from_value::<crate::admin_page_total::AdminPageTotal>(wire)
                .is_err_and(|error| error.is_data())
        )
    );
}

#[test]
fn test_settings_view_preserves_distinct_optional_values_and_wire_shape() {
    assert!([false, true].into_iter().all(|populated| {
        let expected = serde_json::json!({
            (stringify!(default_admin_route)): constants_str::VALUE_074B6E5E,
            (stringify!(main_logo)): populated.then_some(constants_str::VALUE_A24910BB),
            (stringify!(organization_contacts)): populated.then_some(constants_str::VALUE_6F4C18D3),
            (stringify!(organization_name)): populated.then_some(constants_str::VALUE_D029F87E),
            (stringify!(primary_color)): populated.then_some(constants_str::VALUE_55F98A52),
            (stringify!(site_name)): constants_str::VALUE_AD710C1F,
            (stringify!(support_url)): populated.then_some(constants_str::VALUE_FE4E2333),
            (stringify!(tab_title)): populated.then_some(constants_str::VALUE_46BB10C9),
        });
        serde_json::from_value::<crate::admin_settings_view::AdminSettingsView>(expected.clone())
            .is_ok_and(|view| {
                let projection = serde_json::json!({
                    (stringify!(default_admin_route)): view.default_admin_route(),
                    (stringify!(main_logo)): view.main_logo(),
                    (stringify!(organization_contacts)): view.organization_contacts(),
                    (stringify!(organization_name)): view.organization_name(),
                    (stringify!(primary_color)): view.primary_color(),
                    (stringify!(site_name)): view.site_name(),
                    (stringify!(support_url)): view.support_url(),
                    (stringify!(tab_title)): view.tab_title(),
                });
                let rebuilt = crate::admin_settings_view::AdminSettingsView::new(
                    view.default_admin_route().clone(),
                    view.main_logo().cloned(),
                    view.organization_contacts().cloned(),
                    view.organization_name().cloned(),
                    view.primary_color().cloned(),
                    view.site_name().clone(),
                    view.support_url().cloned(),
                    view.tab_title().cloned(),
                );
                projection == expected
                    && serde_json::to_value(rebuilt).is_ok_and(|wire| wire == expected)
            })
    }));
}

#[test]
fn test_audit_page_preserves_distinct_records_optional_fields_and_cursor() {
    let records = serde_json::json!([
        {(stringify!(action)): constants_str::ADMIN_ALT, (stringify!(created_at)): constants_str::ADMIN_FIXTURE_AUDIT_CREATED_AT, (stringify!(details)): {(stringify!(value)): constants_str::X}, (stringify!(id)): 13i64, (stringify!(resource)): constants_str::LOGIN, (stringify!(resource_id)): constants_str::ROOT, (stringify!(succeeded)): true, (stringify!(user_id)): 7i64, (stringify!(user_login)): constants_str::ROOT},
        {(stringify!(action)): constants_str::LOGIN, (stringify!(created_at)): constants_str::ADMIN_FIXTURE_SESSION_CREATED_AT, (stringify!(details)): null, (stringify!(id)): 11i64, (stringify!(resource)): constants_str::ROOT, (stringify!(resource_id)): null, (stringify!(succeeded)): false, (stringify!(user_id)): null, (stringify!(user_login)): null},
    ]);
    assert!([false, true].into_iter().all(|has_cursor| {
        let cursor_value = has_cursor.then(|| serde_json::json!({(stringify!(created_at)): constants_str::ADMIN_FIXTURE_SESSION_CREATED_AT, (stringify!(id)): 11i64}));
        let expected = serde_json::json!({(stringify!(items)): records, (stringify!(next_cursor)): cursor_value, (stringify!(total)): 71u64});
        serde_json::from_value::<crate::admin_audit_page::AdminAuditPage>(expected.clone()).is_ok_and(|page| {
            let projected = serde_json::json!({(stringify!(items)): page.items(), (stringify!(next_cursor)): page.next_cursor(), (stringify!(total)): page.total()});
            let field_values_match = page.items().iter().zip([0usize, 1usize]).all(|(view, position)| {
                records.get(position).is_some_and(|record| serde_json::json!({
                    (stringify!(action)): view.action(),
                    (stringify!(created_at)): view.created_at(),
                    (stringify!(details)): view.details(),
                    (stringify!(id)): view.id(),
                    (stringify!(resource)): view.resource(),
                    (stringify!(resource_id)): view.resource_id(),
                    (stringify!(succeeded)): view.succeeded(),
                    (stringify!(user_id)): view.user_id(),
                    (stringify!(user_login)): view.user_login(),
                }) == *record)
            });
            let cursor = page.next_cursor().map(|original| crate::admin_audit_cursor::AdminAuditCursor::new(original.created_at().clone(), original.id()));
            let cursor_matches = serde_json::to_value(&cursor).is_ok_and(|wire| wire == serde_json::json!(cursor_value));
            let cursor_fields_match = page.next_cursor().is_none_or(|original| serde_json::json!({(stringify!(created_at)): original.created_at(), (stringify!(id)): original.id()}) == serde_json::json!(cursor_value));
            projected == expected && field_values_match && cursor_matches && cursor_fields_match
                && serde_json::from_value::<crate::admin_audit_views::AdminAuditViews>(records.clone()).is_ok_and(|items| {
                    let rebuilt = crate::admin_audit_page::AdminAuditPage::new(items, cursor, page.total());
                    serde_json::to_value(rebuilt).is_ok_and(|wire| wire == expected)
                })
        })
    }));
}

#[test]
fn test_settings_text_wrappers_preserve_character_limits_through_validation_and_serde() {
    fn setting_character_boundary_matches<Setting>(
        admin_bool: crate::admin_bool::AdminBool,
    ) -> crate::admin_bool::AdminBool
    where
        Setting: TryFrom<String> + AsRef<str> + serde::Serialize + serde::de::DeserializeOwned,
    {
        let maximum = constants_usize::VALUE_8_192;
        let base = constants_str::VALUE_FE4E2333;
        crate::admin_bool::AdminBool::from(maximum.checked_sub(base.chars().count()).is_some_and(
            |remaining| {
                let unicode = char::from(233u8).to_string();
                let valid = [
                    base.to_owned(),
                    format!("{}{}", base, constants_str::X.repeat(remaining)),
                    format!("{}{}", base, unicode.repeat(remaining)),
                ]
                .into_iter()
                .all(|input| {
                    Setting::try_from(input.clone()).is_ok_and(|setting| {
                        setting.as_ref() == input
                            && serde_json::to_value(setting).is_ok_and(|wire| {
                                wire == serde_json::json!(input)
                                    && serde_json::from_value::<Setting>(wire)
                                        .is_ok_and(|decoded| decoded.as_ref() == input)
                            })
                    })
                });
                let invalid = [
                    String::new(),
                    constants_str::SPACE.to_owned(),
                    format!("{}{}", base, constants_str::X.repeat(remaining + 1usize)),
                    format!("{}{}", base, unicode.repeat(remaining + 1usize)),
                ]
                .into_iter()
                .all(|input| {
                    Setting::try_from(input.clone()).err().is_some()
                        && serde_json::from_value::<Setting>(serde_json::json!(input))
                            .is_err_and(|error| error.is_data())
                });
                let plain_matches = if bool::from(admin_bool) {
                    Setting::try_from(constants_str::X.to_owned())
                        .err()
                        .is_some()
                        && serde_json::from_value::<Setting>(serde_json::json!(constants_str::X))
                            .is_err_and(|error| error.is_data())
                } else {
                    Setting::try_from(constants_str::X.to_owned())
                        .is_ok_and(|setting| setting.as_ref() == constants_str::X)
                        && serde_json::from_value::<Setting>(serde_json::json!(constants_str::X))
                            .is_ok_and(|decoded| decoded.as_ref() == constants_str::X)
                };
                valid && invalid && plain_matches
            },
        ))
    }
    assert_eq!(
        setting_character_boundary_matches::<crate::admin_site_name::AdminSiteName>(
            crate::admin_bool::AdminBool::from(false)
        ),
        crate::admin_bool::AdminBool::from(true)
    );
    assert_eq!(
        setting_character_boundary_matches::<crate::admin_tab_title::AdminTabTitle>(
            crate::admin_bool::AdminBool::from(false)
        ),
        crate::admin_bool::AdminBool::from(true)
    );
    assert_eq!(
        setting_character_boundary_matches::<crate::admin_main_logo::AdminMainLogo>(
            crate::admin_bool::AdminBool::from(true)
        ),
        crate::admin_bool::AdminBool::from(true)
    );
    assert_eq!(
        setting_character_boundary_matches::<crate::admin_support_url::AdminSupportUrl>(
            crate::admin_bool::AdminBool::from(true)
        ),
        crate::admin_bool::AdminBool::from(true)
    );
}

#[test]
fn test_password_entropy_and_generated_password_preserve_hex_and_debug_redaction() {
    assert!([0u8, 10u8, 176u8, 255u8].into_iter().all(|byte| {
        let entropy = crate::admin_password_entropy::AdminPasswordEntropy::from([byte; 32usize]);
        let entropy_debug = format!("{entropy:?}");
        entropy.into_inner() == [byte; 32usize]
            && entropy_debug
                == format!(
                    "{}({:?})",
                    stringify!(AdminPasswordEntropy),
                    constants_str::REDACTED_ALT_3
                )
            && crate::admin_new_password::AdminNewPassword::try_from(entropy).is_ok_and(
                |password| {
                    password.as_ref().as_bytes().get(..4usize)
                        == Some(concat!(stringify!(Aa1), '!').as_bytes())
                        && password
                            .as_ref()
                            .get(4usize..)
                            .is_some_and(|hex| hex == format!("{byte:02x}").repeat(32usize))
                        && format!("{password:?}")
                            == format!(
                                "{}({:?})",
                                stringify!(AdminNewPassword),
                                constants_str::REDACTED_ALT_3
                            )
                },
            )
    }));
}
