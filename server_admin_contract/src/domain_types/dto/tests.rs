#[test]
fn authenticated_admin_checks_owned_permissions() {
    let admin = super::AuthenticatedAdmin::new(
        crate::domain_types::AdminDisplayName::try_from(constants_str::ADMIN.to_owned())
            .expect("68e94b2f authenticated_admin_checks_owned_permissions invariant must hold"),
        crate::domain_types::AdminUserId::try_from(constants_i64::ONE)
            .expect("134f7a9c authenticated_admin_checks_owned_permissions invariant must hold"),
        crate::domain_types::AdminLogin::try_from(constants_str::ROOT.to_owned())
            .expect("971c5e42 authenticated_admin_checks_owned_permissions invariant must hold"),
        crate::domain_types::AdminPermissionValues::try_from(vec![
            crate::domain_types::AdminPermissionValue::try_from(
                crate::domain_types::AdminPermission::UsersRead
                    .as_str()
                    .get()
                    .to_owned(),
            )
            .expect(constants_str::VALUE_785335E9),
        ])
        .expect("bd2806f1 authenticated_admin_checks_owned_permissions invariant must hold"),
        crate::domain_types::AdminRoleNames::try_from(Vec::new())
            .expect("763ae20c authenticated_admin_checks_owned_permissions invariant must hold"),
    );
    assert!(bool::from(admin.has_permission(
        crate::domain_types::AdminPermission::UsersRead
    )));
    assert!(!bool::from(admin.has_permission(
        crate::domain_types::AdminPermission::UsersUpdate
    )));
    assert!(bool::from(
        admin.can_access(crate::domain_types::AdminPage::Users)
    ));
    assert!(!bool::from(
        admin.can_access(crate::domain_types::AdminPage::Roles)
    ));
    assert!(bool::from(
        admin.can_access(crate::domain_types::AdminPage::Profile)
    ));
}

#[test]
fn change_own_password_has_no_session_revocation_choice() {
    let request = crate::domain_types::AdminChangeOwnPasswordReq::new(
        crate::domain_types::AdminPassword::try_from(String::from(constants_str::VALUE_A1AB879D))
            .expect(
                "c10e4db7 change_own_password_has_no_session_revocation_choice invariant must hold",
            ),
        crate::domain_types::AdminNewPassword::try_from(String::from(
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
        crate::domain_types::AdminChangeOwnPasswordReq,
    >(constants_str::VALUE_4A4AAF28) else {
        panic!("abaa9cdf");
    };
}

#[test]
fn passwords_are_redacted_and_share_policy() {
    let password =
        crate::domain_types::AdminPassword::try_from(String::from(constants_str::SECRET))
            .expect("9f3f5164 passwords_are_redacted_and_share_policy invariant must hold");
    assert!(!format!("{password:?}").contains("secret"));
    let _new_password = crate::domain_types::AdminNewPassword::try_from(
        constants_str::TEST_STRONG_PASSWORD.to_owned(),
    )
    .expect("da19950b passwords_are_redacted_and_share_policy invariant must hold");
    let Err(_weak_password_error) =
        crate::domain_types::AdminNewPassword::try_from(constants_str::PASSWORD.to_owned())
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
    let Ok(_basic_request) = serde_json::from_value::<crate::domain_types::AdminSignInReq>(basic)
    else {
        panic!("af47412d");
    };
    let legacy_mfa = serde_json::json!({
        "login": "admin",
        "mfa_proof": { "kind": "totp", "value": "123456" },
        "password": "correct_password"
    });
    let Err(_legacy_mfa_error) =
        serde_json::from_value::<crate::domain_types::AdminSignInReq>(legacy_mfa)
    else {
        panic!("89071e97");
    };
}

#[test]
fn domain_values_follow_database_compatible_policies() {
    let _valid_login = crate::domain_types::AdminLogin::try_from(
        constants_str::ADMIN_USER_1.to_owned(),
    )
    .expect("e1cddebc domain_values_follow_database_compatible_policies invariant must hold");
    let Err(_reserved_login_error) =
        crate::domain_types::AdminLogin::try_from(constants_str::ADMIN.to_owned())
    else {
        panic!("ab23c76e");
    };
    let Err(_short_login_error) =
        crate::domain_types::AdminLogin::try_from(constants_str::AB.to_owned())
    else {
        panic!("ce5b9e72");
    };
    let _valid_display_name = crate::domain_types::AdminDisplayName::try_from(
        constants_str::ADMIN.to_owned(),
    )
    .expect("d315b74f domain_values_follow_database_compatible_policies invariant must hold");
    let Err(_blank_display_name_error) =
        crate::domain_types::AdminDisplayName::try_from(constants_str::SPACE.to_owned())
    else {
        panic!("1ccd43aa");
    };
    let _valid_role_name = crate::domain_types::AdminRoleName::try_from(
        constants_str::ADMIN_ALT.to_owned(),
    )
    .expect("713890e9 domain_values_follow_database_compatible_policies invariant must hold");
    let Err(_reserved_role_name_error) =
        crate::domain_types::AdminRoleName::try_from(constants_str::ADMIN.to_owned())
    else {
        panic!("147fe35a");
    };
}
