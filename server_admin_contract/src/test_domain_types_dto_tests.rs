#[test]
fn test_authenticated_admin_checks_owned_permissions() {
    let admin = crate::authenticated_admin::AuthenticatedAdmin::new(
        crate::admin_display_name::AdminDisplayName::try_from(constants_str::ADMIN.to_owned())
            .expect(constants_str::DIAGNOSTIC_68E94B2F),
        crate::admin_user_id::AdminUserId::try_from(constants_i64::ONE)
            .expect(constants_str::DIAGNOSTIC_134F7A9C),
        crate::admin_login::AdminLogin::try_from(constants_str::ROOT.to_owned())
            .expect(constants_str::DIAGNOSTIC_971C5E42),
        crate::admin_permission_values::AdminPermissionValues::try_from(vec![
            crate::admin_permission_value::AdminPermissionValue::try_from(
                crate::admin_permission::AdminPermission::UsersRead
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
    assert!(bool::from(admin.has_permission(
        crate::admin_permission::AdminPermission::UsersRead
    )));
    assert!(!bool::from(admin.has_permission(
        crate::admin_permission::AdminPermission::UsersUpdate
    )));
    assert!(bool::from(
        admin.can_access(crate::admin_page::AdminPage::Users)
    ));
    assert!(!bool::from(
        admin.can_access(crate::admin_page::AdminPage::Roles)
    ));
    assert!(bool::from(
        admin.can_access(crate::admin_page::AdminPage::Profile)
    ));
}

#[test]
fn test_change_own_password_has_no_session_revocation_choice() {
    let request = crate::admin_change_own_password_req::AdminChangeOwnPasswordReq::new(
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
        crate::admin_change_own_password_req::AdminChangeOwnPasswordReq,
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
        serde_json::from_value::<crate::admin_sign_in_req::AdminSignInReq>(basic)
    else {
        std::panic::panic_any(constants_str::PANIC_AF47412D);
    };
    let legacy_mfa = serde_json::json!({
        "login": "admin",
        "mfa_proof": { "kind": "totp", "value": "123456" },
        "password": "correct_password"
    });
    let Err(_legacy_mfa_error) =
        serde_json::from_value::<crate::admin_sign_in_req::AdminSignInReq>(legacy_mfa)
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
