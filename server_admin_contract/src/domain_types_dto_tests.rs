#[test]
fn authenticated_admin_checks_owned_permissions() {
    let admin = crate::authenticated_admin::AuthenticatedAdmin::new(
        crate::admin_display_name::AdminDisplayName::try_from(constants_str::ADMIN.to_owned())
            .expect("68e94b2f authenticated_admin_checks_owned_permissions invariant must hold"),
        crate::admin_user_id::AdminUserId::try_from(constants_i64::ONE)
            .expect("134f7a9c authenticated_admin_checks_owned_permissions invariant must hold"),
        crate::admin_login::AdminLogin::try_from(constants_str::ROOT.to_owned())
            .expect("971c5e42 authenticated_admin_checks_owned_permissions invariant must hold"),
        crate::admin_permission_values::AdminPermissionValues::try_from(vec![
            crate::admin_permission_value::AdminPermissionValue::try_from(
                crate::admin_permission::AdminPermission::UsersRead
                    .as_str()
                    .get()
                    .to_owned(),
            )
            .expect(constants_str::VALUE_785335E9),
        ])
        .expect("bd2806f1 authenticated_admin_checks_owned_permissions invariant must hold"),
        crate::admin_role_names::AdminRoleNames::try_from(Vec::new())
            .expect("763ae20c authenticated_admin_checks_owned_permissions invariant must hold"),
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
fn change_own_password_has_no_session_revocation_choice() {
    let request = crate::admin_change_own_password_req::AdminChangeOwnPasswordReq::new(
        crate::admin_password::AdminPassword::try_from(String::from(constants_str::VALUE_A1AB879D))
            .expect(
                "c10e4db7 change_own_password_has_no_session_revocation_choice invariant must hold",
            ),
        crate::admin_new_password::AdminNewPassword::try_from(String::from(
            constants_str::VALUE_05A7131F,
        ))
        .expect(
            "5932a1fe change_own_password_has_no_session_revocation_choice invariant must hold",
        ),
    );
    let json = serde_json::to_value(request).expect(
        "06ba3ef9 change_own_password_has_no_session_revocation_choice invariant must hold",
    );
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
        panic!("abaa9cdf");
    };
}

#[test]
fn passwords_are_redacted_and_share_policy() {
    let password =
        crate::admin_password::AdminPassword::try_from(String::from(constants_str::SECRET))
            .expect("9f3f5164 passwords_are_redacted_and_share_policy invariant must hold");
    assert!(!format!("{password:?}").contains("secret"));
    let _new_password = crate::admin_new_password::AdminNewPassword::try_from(
        constants_str::TEST_STRONG_PASSWORD.to_owned(),
    )
    .expect("da19950b passwords_are_redacted_and_share_policy invariant must hold");
    let Err(_weak_password_error) =
        crate::admin_new_password::AdminNewPassword::try_from(constants_str::PASSWORD.to_owned())
    else {
        panic!("24900f2f");
    };
}

#[test]
fn sign_in_accepts_only_login_and_password() {
    let basic = serde_json::json!({
        "login": "admin",
        "password": "correct_password"
    });
    let Ok(_basic_request) =
        serde_json::from_value::<crate::admin_sign_in_req::AdminSignInReq>(basic)
    else {
        panic!("af47412d");
    };
    let legacy_mfa = serde_json::json!({
        "login": "admin",
        "mfa_proof": { "kind": "totp", "value": "123456" },
        "password": "correct_password"
    });
    let Err(_legacy_mfa_error) =
        serde_json::from_value::<crate::admin_sign_in_req::AdminSignInReq>(legacy_mfa)
    else {
        panic!("89071e97");
    };
}

#[test]
fn domain_values_follow_database_compatible_policies() {
    let _valid_login = crate::admin_login::AdminLogin::try_from(
        constants_str::ADMIN_USER_1.to_owned(),
    )
    .expect("e1cddebc domain_values_follow_database_compatible_policies invariant must hold");
    let Err(_reserved_login_error) =
        crate::admin_login::AdminLogin::try_from(constants_str::ADMIN.to_owned())
    else {
        panic!("ab23c76e");
    };
    let Err(_short_login_error) =
        crate::admin_login::AdminLogin::try_from(constants_str::AB.to_owned())
    else {
        panic!("ce5b9e72");
    };
    let _valid_display_name =
        crate::admin_display_name::AdminDisplayName::try_from(constants_str::ADMIN.to_owned())
            .expect(
                "d315b74f domain_values_follow_database_compatible_policies invariant must hold",
            );
    let Err(_blank_display_name_error) =
        crate::admin_display_name::AdminDisplayName::try_from(constants_str::SPACE.to_owned())
    else {
        panic!("1ccd43aa");
    };
    let _valid_role_name =
        crate::admin_role_name::AdminRoleName::try_from(constants_str::ADMIN_ALT.to_owned())
            .expect(
                "713890e9 domain_values_follow_database_compatible_policies invariant must hold",
            );
    let Err(_reserved_role_name_error) =
        crate::admin_role_name::AdminRoleName::try_from(constants_str::ADMIN.to_owned())
    else {
        panic!("147fe35a");
    };
}
