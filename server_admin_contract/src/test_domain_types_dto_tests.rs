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
    let Err(_unknown_field_error) = serde_json::from_str::<
        crate::admin_change_own_password_request::AdminChangeOwnPasswordRequest,
    >(constants_str::VALUE_4A4AAF28) else {
        std::panic::panic_any(constants_str::PANIC_ABAA9CDF);
    };
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
