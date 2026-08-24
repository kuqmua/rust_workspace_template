#[tokio::test]
#[ignore = "requires PostgreSQL; run through workspace_test_runner database"]
async fn postgresql_html_users_crud_covers_every_frontend_field_separately() {
    let fixture = admin_html_test_fixture().await;
    assert!(fixture.cookie.0.contains(fixture.csrf.0.as_str()));
    let login = "html_crud_user";
    let updated_login = "html_crud_user_updated";
    let display_name = "HTML CRUD User";
    let updated_display_name = "HTML CRUD User Updated";
    let password = "Html-crud-pass1";
    let updated_password = "Html-crud-pass2";
    let create_body = AdminHtmlTestFormBody::try_from(format!(
        "login={login}&display_name=HTML+CRUD+User&password={password}"
    ))
    .expect("801d9a43 postgresql_html_users_crud_covers_every_frontend_field_separately invariant must hold");
    let create_response = admin_html_response(
        &fixture,
        HttpAdminApiTestMethod::from(http::Method::POST),
        StdAdminApiTestStrRef::from(server_admin_contract::AdminHtmlAction::UserCreate.get()),
        StdAdminApiTestStrRef::from(create_body.0.as_str()),
    )
    .await;
    assert_eq!(create_response.status(), http::StatusCode::SEE_OTHER);
    let created = sqlx::query_as::<_, (i64, String, String, bool)>(
        "SELECT id, login, display_name, is_banned FROM users WHERE login = $1",
    )
    .bind(login)
    .fetch_one(&fixture.pool.0)
    .await
    .expect("5de4fc12 postgresql_html_users_crud_covers_every_frontend_field_separately invariant must hold");
    assert_eq!(created.1, login);
    assert_eq!(created.2, display_name);
    assert!(!created.3);
    let users_response = admin_html_response(
        &fixture,
        HttpAdminApiTestMethod::from(http::Method::GET),
        StdAdminApiTestStrRef::from(server_admin_contract::AdminFrontendPath::Users.get()),
        StdAdminApiTestStrRef::from(str_constants::PG_CRUD_EMPTY_SQL_SUFFIX),
    )
    .await;
    assert_eq!(users_response.status(), http::StatusCode::OK);
    let users_html = admin_html_body(users_response).await;
    assert_admin_csr_shell(&users_html);

    let login_update_body = AdminHtmlTestFormBody::try_from(format!(
        "user_id={}&login={updated_login}&display_name=HTML+CRUD+User",
        created.0
    ))
    .expect("b0714f29 postgresql_html_users_crud_covers_every_frontend_field_separately invariant must hold");
    let login_update_response = admin_html_response(
        &fixture,
        HttpAdminApiTestMethod::from(http::Method::POST),
        StdAdminApiTestStrRef::from(server_admin_contract::AdminHtmlAction::UserUpdate.get()),
        StdAdminApiTestStrRef::from(login_update_body.0.as_str()),
    )
    .await;
    assert_eq!(login_update_response.status(), http::StatusCode::SEE_OTHER);
    let login_update = sqlx::query_as::<_, (String, String)>(
        "SELECT login, display_name FROM users WHERE id = $1",
    )
    .bind(created.0)
    .fetch_one(&fixture.pool.0)
    .await
    .expect("68fae270 postgresql_html_users_crud_covers_every_frontend_field_separately invariant must hold");
    assert_eq!(
        login_update,
        (updated_login.to_owned(), display_name.to_owned())
    );

    let display_update_body = AdminHtmlTestFormBody::try_from(format!(
        "user_id={}&login={updated_login}&display_name=HTML+CRUD+User+Updated",
        created.0
    ))
    .expect("9a6eb324 postgresql_html_users_crud_covers_every_frontend_field_separately invariant must hold");
    let display_update_response = admin_html_response(
        &fixture,
        HttpAdminApiTestMethod::from(http::Method::POST),
        StdAdminApiTestStrRef::from(server_admin_contract::AdminHtmlAction::UserUpdate.get()),
        StdAdminApiTestStrRef::from(display_update_body.0.as_str()),
    )
    .await;
    assert_eq!(
        display_update_response.status(),
        http::StatusCode::SEE_OTHER
    );
    let display_update = sqlx::query_as::<_, (String, String)>(
        "SELECT login, display_name FROM users WHERE id = $1",
    )
    .bind(created.0)
    .fetch_one(&fixture.pool.0)
    .await
    .expect("10df386a postgresql_html_users_crud_covers_every_frontend_field_separately invariant must hold");
    assert_eq!(
        display_update,
        (updated_login.to_owned(), updated_display_name.to_owned())
    );

    let password_update_body = AdminHtmlTestFormBody::try_from(format!(
        "user_id={}&password={updated_password}",
        created.0
    ))
    .expect("cd82f375 postgresql_html_users_crud_covers_every_frontend_field_separately invariant must hold");
    let password_update_response = admin_html_response(
        &fixture,
        HttpAdminApiTestMethod::from(http::Method::POST),
        StdAdminApiTestStrRef::from(server_admin_contract::AdminHtmlAction::UserPassword.get()),
        StdAdminApiTestStrRef::from(password_update_body.0.as_str()),
    )
    .await;
    assert_eq!(
        password_update_response.status(),
        http::StatusCode::SEE_OTHER
    );
    let old_sign_in_body =
        AdminHtmlTestFormBody::try_from(format!("login={updated_login}&password={password}"))
            .expect("8c42d7e1 postgresql_html_users_crud_covers_every_frontend_field_separately invariant must hold");
    let old_sign_in_response = tower::ServiceExt::oneshot(
        fixture.router.0.clone(),
        html_request_with_peer(
            HttpAdminApiTestMethod::from(http::Method::POST),
            StdAdminApiTestStrRef::from(server_admin_contract::AdminHtmlAction::SignIn.get()),
            StdAdminApiTestStrRef::from(old_sign_in_body.0.as_str()),
            None,
        )
        .0,
    )
    .await
    .expect("26ab3584 postgresql_html_users_crud_covers_every_frontend_field_separately invariant must hold");
    assert_eq!(
        old_sign_in_response.status(),
        http::StatusCode::UNAUTHORIZED
    );
    let new_sign_in_body = AdminHtmlTestFormBody::try_from(format!(
        "login={updated_login}&password={updated_password}"
    ))
    .expect("ef05a691 postgresql_html_users_crud_covers_every_frontend_field_separately invariant must hold");
    let new_sign_in_response = tower::ServiceExt::oneshot(
        fixture.router.0.clone(),
        html_request_with_peer(
            HttpAdminApiTestMethod::from(http::Method::POST),
            StdAdminApiTestStrRef::from(server_admin_contract::AdminHtmlAction::SignIn.get()),
            StdAdminApiTestStrRef::from(new_sign_in_body.0.as_str()),
            None,
        )
        .0,
    )
    .await
    .expect("b9306c2e postgresql_html_users_crud_covers_every_frontend_field_separately invariant must hold");
    assert_eq!(new_sign_in_response.status(), http::StatusCode::SEE_OTHER);

    let role_id = sqlx::query_scalar::<_, i64>(str_constants::SERVER_ADMIN_READ_ADMIN_ROLE_ID_SQL)
        .fetch_one(&fixture.pool.0)
        .await
        .expect("f1674ab9 postgresql_html_users_crud_covers_every_frontend_field_separately invariant must hold");
    let roles_update_body = AdminHtmlTestFormBody::try_from(format!(
        "user_id={}&expected_role_ids=&role_{role_id}={role_id}",
        created.0
    ))
    .expect("410e6a8c postgresql_html_users_crud_covers_every_frontend_field_separately invariant must hold");
    let roles_update_response = admin_html_response(
        &fixture,
        HttpAdminApiTestMethod::from(http::Method::POST),
        StdAdminApiTestStrRef::from(server_admin_contract::AdminHtmlAction::UserRoles.get()),
        StdAdminApiTestStrRef::from(roles_update_body.0.as_str()),
    )
    .await;
    assert_eq!(roles_update_response.status(), http::StatusCode::SEE_OTHER);
    let assigned_roles =
        sqlx::query_scalar::<_, i64>("SELECT role_id FROM user_roles WHERE user_id = $1")
            .bind(created.0)
            .fetch_all(&fixture.pool.0)
            .await
            .expect("739cb4f5 postgresql_html_users_crud_covers_every_frontend_field_separately invariant must hold");
    assert_eq!(assigned_roles, vec![role_id]);

    let ban_body = AdminHtmlTestFormBody::try_from(format!("user_id={}&is_banned=true", created.0))
        .expect("a17fdc64 postgresql_html_users_crud_covers_every_frontend_field_separately invariant must hold");
    let ban_response = admin_html_response(
        &fixture,
        HttpAdminApiTestMethod::from(http::Method::POST),
        StdAdminApiTestStrRef::from(server_admin_contract::AdminHtmlAction::UserBan.get()),
        StdAdminApiTestStrRef::from(ban_body.0.as_str()),
    )
    .await;
    assert_eq!(ban_response.status(), http::StatusCode::SEE_OTHER);
    let final_users_response = admin_html_response(
        &fixture,
        HttpAdminApiTestMethod::from(http::Method::GET),
        StdAdminApiTestStrRef::from(server_admin_contract::AdminFrontendPath::Users.get()),
        StdAdminApiTestStrRef::from(str_constants::PG_CRUD_EMPTY_SQL_SUFFIX),
    )
    .await;
    let final_users_html = admin_html_body(final_users_response).await;
    assert_admin_csr_shell(&final_users_html);
    let unban_body =
        AdminHtmlTestFormBody::try_from(format!("user_id={}&is_banned=false", created.0))
            .expect("9d304db3 postgresql_html_users_crud_covers_every_frontend_field_separately invariant must hold");
    let unban_response = admin_html_response(
        &fixture,
        HttpAdminApiTestMethod::from(http::Method::POST),
        StdAdminApiTestStrRef::from(server_admin_contract::AdminHtmlAction::UserBan.get()),
        StdAdminApiTestStrRef::from(unban_body.0.as_str()),
    )
    .await;
    assert_eq!(unban_response.status(), http::StatusCode::SEE_OTHER);
    let is_banned = sqlx::query_scalar::<_, bool>("SELECT is_banned FROM users WHERE id = $1")
        .bind(created.0)
        .fetch_one(&fixture.pool.0)
        .await
        .expect("55208887 postgresql_html_users_crud_covers_every_frontend_field_separately invariant must hold");
    assert!(!is_banned);
    let roles_clear_body = AdminHtmlTestFormBody::try_from(format!(
        "user_id={}&expected_role_ids={role_id}",
        created.0
    ))
    .expect("04b638dc postgresql_html_users_crud_covers_every_frontend_field_separately invariant must hold");
    let roles_clear_response = admin_html_response(
        &fixture,
        HttpAdminApiTestMethod::from(http::Method::POST),
        StdAdminApiTestStrRef::from(server_admin_contract::AdminHtmlAction::UserRoles.get()),
        StdAdminApiTestStrRef::from(roles_clear_body.0.as_str()),
    )
    .await;
    assert_eq!(roles_clear_response.status(), http::StatusCode::SEE_OTHER);

    let delete_body =
        AdminHtmlTestFormBody::try_from(format!("user_id={}&confirmation=true", created.0))
            .expect("d4fe3069 postgresql_html_users_crud_covers_every_frontend_field_separately invariant must hold");
    let delete_response = admin_html_response(
        &fixture,
        HttpAdminApiTestMethod::from(http::Method::POST),
        StdAdminApiTestStrRef::from(server_admin_contract::AdminHtmlAction::UserDelete.get()),
        StdAdminApiTestStrRef::from(delete_body.0.as_str()),
    )
    .await;
    assert_eq!(delete_response.status(), http::StatusCode::SEE_OTHER);
    let deleted_count = sqlx::query_scalar::<_, i64>("SELECT count(*) FROM users WHERE id = $1")
        .bind(created.0)
        .fetch_one(&fixture.pool.0)
        .await
        .expect("72c950ea postgresql_html_users_crud_covers_every_frontend_field_separately invariant must hold");
    assert_eq!(deleted_count, i64_constants::ZERO);
    let deleted_users_response = admin_html_response(
        &fixture,
        HttpAdminApiTestMethod::from(http::Method::GET),
        StdAdminApiTestStrRef::from(server_admin_contract::AdminFrontendPath::Users.get()),
        StdAdminApiTestStrRef::from(str_constants::PG_CRUD_EMPTY_SQL_SUFFIX),
    )
    .await;
    let deleted_users_html = admin_html_body(deleted_users_response).await;
    assert_admin_csr_shell(&deleted_users_html);
    fixture.lock.0.rollback().await.expect("93db561a postgresql_html_users_crud_covers_every_frontend_field_separately invariant must hold");
}
#[tokio::test]
#[ignore = "requires PostgreSQL; run through workspace_test_runner database"]
async fn postgresql_html_roles_crud_covers_every_frontend_field_separately() {
    let fixture = admin_html_test_fixture().await;
    let role_name = "html_crud_role";
    let updated_role_name = "html_crud_role_updated";
    let create_body =
        AdminHtmlTestFormBody::try_from(format!("name={role_name}")).expect("c593e840 postgresql_html_roles_crud_covers_every_frontend_field_separately invariant must hold");
    let create_response = admin_html_response(
        &fixture,
        HttpAdminApiTestMethod::from(http::Method::POST),
        StdAdminApiTestStrRef::from(server_admin_contract::AdminHtmlAction::RoleCreate.get()),
        StdAdminApiTestStrRef::from(create_body.0.as_str()),
    )
    .await;
    assert_eq!(create_response.status(), http::StatusCode::SEE_OTHER);
    let created = sqlx::query_as::<_, (i64, String, bool)>(
        "SELECT id, name, is_system FROM roles WHERE name = $1",
    )
    .bind(role_name)
    .fetch_one(&fixture.pool.0)
    .await
    .expect("196fbd27 postgresql_html_roles_crud_covers_every_frontend_field_separately invariant must hold");
    assert_eq!(created.1, role_name);
    assert!(!created.2);
    let roles_response = admin_html_response(
        &fixture,
        HttpAdminApiTestMethod::from(http::Method::GET),
        StdAdminApiTestStrRef::from(server_admin_contract::AdminFrontendPath::Roles.get()),
        StdAdminApiTestStrRef::from(str_constants::PG_CRUD_EMPTY_SQL_SUFFIX),
    )
    .await;
    assert_eq!(roles_response.status(), http::StatusCode::OK);
    let roles_html = admin_html_body(roles_response).await;
    assert_admin_csr_shell(&roles_html);

    let update_body =
        AdminHtmlTestFormBody::try_from(format!("role_id={}&name={updated_role_name}", created.0))
            .expect("7ea84503 postgresql_html_roles_crud_covers_every_frontend_field_separately invariant must hold");
    let update_response = admin_html_response(
        &fixture,
        HttpAdminApiTestMethod::from(http::Method::POST),
        StdAdminApiTestStrRef::from(server_admin_contract::AdminHtmlAction::RoleUpdate.get()),
        StdAdminApiTestStrRef::from(update_body.0.as_str()),
    )
    .await;
    assert_eq!(update_response.status(), http::StatusCode::SEE_OTHER);
    let updated = sqlx::query_scalar::<_, String>("SELECT name FROM roles WHERE id = $1")
        .bind(created.0)
        .fetch_one(&fixture.pool.0)
        .await
        .expect("43f81d69 postgresql_html_roles_crud_covers_every_frontend_field_separately invariant must hold");
    assert_eq!(updated, updated_role_name);

    let permission =
        sqlx::query_as::<_, (i64, String)>("SELECT id, name FROM permissions ORDER BY id LIMIT 1")
            .fetch_one(&fixture.pool.0)
            .await
            .expect("ba920f54 postgresql_html_roles_crud_covers_every_frontend_field_separately invariant must hold");
    let permissions_body = AdminHtmlTestFormBody::try_from(format!(
        "role_id={}&expected_permission_ids=&permission_{}={}",
        created.0, permission.0, permission.0
    ))
    .expect("0d476c31 postgresql_html_roles_crud_covers_every_frontend_field_separately invariant must hold");
    let permissions_response = admin_html_response(
        &fixture,
        HttpAdminApiTestMethod::from(http::Method::POST),
        StdAdminApiTestStrRef::from(server_admin_contract::AdminHtmlAction::RolePermissions.get()),
        StdAdminApiTestStrRef::from(permissions_body.0.as_str()),
    )
    .await;
    assert_eq!(permissions_response.status(), http::StatusCode::SEE_OTHER);
    let assigned_permissions = sqlx::query_scalar::<_, i64>(
        "SELECT permission_id FROM role_permissions WHERE role_id = $1",
    )
    .bind(created.0)
    .fetch_all(&fixture.pool.0)
    .await
    .expect("82b0d9f3 postgresql_html_roles_crud_covers_every_frontend_field_separately invariant must hold");
    assert_eq!(assigned_permissions, vec![permission.0]);
    let final_roles_response = admin_html_response(
        &fixture,
        HttpAdminApiTestMethod::from(http::Method::GET),
        StdAdminApiTestStrRef::from(server_admin_contract::AdminFrontendPath::Roles.get()),
        StdAdminApiTestStrRef::from(str_constants::PG_CRUD_EMPTY_SQL_SUFFIX),
    )
    .await;
    let final_roles_html = admin_html_body(final_roles_response).await;
    assert_admin_csr_shell(&final_roles_html);

    let delete_body =
        AdminHtmlTestFormBody::try_from(format!("role_id={}&confirmation=true", created.0))
            .expect("e1547a60 postgresql_html_roles_crud_covers_every_frontend_field_separately invariant must hold");
    let delete_response = admin_html_response(
        &fixture,
        HttpAdminApiTestMethod::from(http::Method::POST),
        StdAdminApiTestStrRef::from(server_admin_contract::AdminHtmlAction::RoleDelete.get()),
        StdAdminApiTestStrRef::from(delete_body.0.as_str()),
    )
    .await;
    assert_eq!(delete_response.status(), http::StatusCode::SEE_OTHER);
    let deleted_count = sqlx::query_scalar::<_, i64>("SELECT count(*) FROM roles WHERE id = $1")
        .bind(created.0)
        .fetch_one(&fixture.pool.0)
        .await
        .expect("2db479f8 postgresql_html_roles_crud_covers_every_frontend_field_separately invariant must hold");
    assert_eq!(deleted_count, i64_constants::ZERO);
    let deleted_roles_response = admin_html_response(
        &fixture,
        HttpAdminApiTestMethod::from(http::Method::GET),
        StdAdminApiTestStrRef::from(server_admin_contract::AdminFrontendPath::Roles.get()),
        StdAdminApiTestStrRef::from(str_constants::PG_CRUD_EMPTY_SQL_SUFFIX),
    )
    .await;
    let deleted_roles_html = admin_html_body(deleted_roles_response).await;
    assert_admin_csr_shell(&deleted_roles_html);
    fixture.lock.0.rollback().await.expect("674dc2a9 postgresql_html_roles_crud_covers_every_frontend_field_separately invariant must hold");
}
#[tokio::test]
#[ignore = "requires PostgreSQL; run through workspace_test_runner database"]
async fn postgresql_html_settings_updates_and_reads_every_field_separately() {
    let fixture = admin_html_test_fixture().await;
    let site_name_a = StdAdminApiTestStrRef::from("HtmlSiteA");
    let site_name_b = StdAdminApiTestStrRef::from("HtmlSiteB");
    let route_a =
        StdAdminApiTestStrRef::from(server_admin_contract::AdminFrontendPath::Users.get());
    let route_b = StdAdminApiTestStrRef::from("/admin/roles");
    let tab_title_a = StdAdminApiTestStrRef::from("HtmlTabA");
    let tab_title_b = StdAdminApiTestStrRef::from("HtmlTabB");
    let main_logo_a = StdAdminApiTestStrRef::from("https://example.com/logo-a.png");
    let main_logo_b = StdAdminApiTestStrRef::from("https://example.com/logo-b.png");
    let primary_color_a = StdAdminApiTestStrRef::from("#112233");
    let primary_color_b = StdAdminApiTestStrRef::from("#445566");
    let organization_name_a = StdAdminApiTestStrRef::from("HtmlOrgA");
    let organization_name_b = StdAdminApiTestStrRef::from("HtmlOrgB");
    let organization_contacts_a = StdAdminApiTestStrRef::from("ops-a@example.com");
    let organization_contacts_b = StdAdminApiTestStrRef::from("ops-b@example.com");
    let support_url_a = StdAdminApiTestStrRef::from("https://example.com/support-a");
    let support_url_b = StdAdminApiTestStrRef::from("https://example.com/support-b");
    let states = [
        AdminHtmlSettingsTestValues {
            default_admin_route: route_a,
            main_logo: main_logo_a,
            organization_contacts: organization_contacts_a,
            organization_name: organization_name_a,
            primary_color: primary_color_a,
            site_name: site_name_a,
            support_url: support_url_a,
            tab_title: tab_title_a,
        },
        AdminHtmlSettingsTestValues {
            site_name: site_name_b,
            ..AdminHtmlSettingsTestValues {
                default_admin_route: route_a,
                main_logo: main_logo_a,
                organization_contacts: organization_contacts_a,
                organization_name: organization_name_a,
                primary_color: primary_color_a,
                site_name: site_name_a,
                support_url: support_url_a,
                tab_title: tab_title_a,
            }
        },
        AdminHtmlSettingsTestValues {
            default_admin_route: route_b,
            main_logo: main_logo_a,
            organization_contacts: organization_contacts_a,
            organization_name: organization_name_a,
            primary_color: primary_color_a,
            site_name: site_name_b,
            support_url: support_url_a,
            tab_title: tab_title_a,
        },
        AdminHtmlSettingsTestValues {
            tab_title: tab_title_b,
            ..AdminHtmlSettingsTestValues {
                default_admin_route: route_b,
                main_logo: main_logo_a,
                organization_contacts: organization_contacts_a,
                organization_name: organization_name_a,
                primary_color: primary_color_a,
                site_name: site_name_b,
                support_url: support_url_a,
                tab_title: tab_title_a,
            }
        },
        AdminHtmlSettingsTestValues {
            main_logo: main_logo_b,
            default_admin_route: route_b,
            organization_contacts: organization_contacts_a,
            organization_name: organization_name_a,
            primary_color: primary_color_a,
            site_name: site_name_b,
            support_url: support_url_a,
            tab_title: tab_title_b,
        },
        AdminHtmlSettingsTestValues {
            primary_color: primary_color_b,
            default_admin_route: route_b,
            main_logo: main_logo_b,
            organization_contacts: organization_contacts_a,
            organization_name: organization_name_a,
            site_name: site_name_b,
            support_url: support_url_a,
            tab_title: tab_title_b,
        },
        AdminHtmlSettingsTestValues {
            organization_name: organization_name_b,
            default_admin_route: route_b,
            main_logo: main_logo_b,
            organization_contacts: organization_contacts_a,
            primary_color: primary_color_b,
            site_name: site_name_b,
            support_url: support_url_a,
            tab_title: tab_title_b,
        },
        AdminHtmlSettingsTestValues {
            organization_contacts: organization_contacts_b,
            default_admin_route: route_b,
            main_logo: main_logo_b,
            organization_name: organization_name_b,
            primary_color: primary_color_b,
            site_name: site_name_b,
            support_url: support_url_a,
            tab_title: tab_title_b,
        },
        AdminHtmlSettingsTestValues {
            support_url: support_url_b,
            default_admin_route: route_b,
            main_logo: main_logo_b,
            organization_contacts: organization_contacts_b,
            organization_name: organization_name_b,
            primary_color: primary_color_b,
            site_name: site_name_b,
            tab_title: tab_title_b,
        },
    ];
    let fixture_ref = &fixture;
    futures::StreamExt::fold(futures::stream::iter(states), (), async |(), values| {
        let form_body = values.form_body();
        let update_response = admin_html_response(
            fixture_ref,
            HttpAdminApiTestMethod::from(http::Method::POST),
            StdAdminApiTestStrRef::from(
                server_admin_contract::AdminHtmlAction::SettingsUpdate.get(),
            ),
            StdAdminApiTestStrRef::from(form_body.0.as_str()),
        )
        .await;
        assert_eq!(update_response.status(), http::StatusCode::SEE_OTHER);
        let read_response = admin_html_response(
            fixture_ref,
            HttpAdminApiTestMethod::from(http::Method::GET),
            StdAdminApiTestStrRef::from(server_admin_contract::AdminFrontendPath::Settings.get()),
            StdAdminApiTestStrRef::from(str_constants::PG_CRUD_EMPTY_SQL_SUFFIX),
        )
        .await;
        assert_eq!(read_response.status(), http::StatusCode::OK);
        let read_html = admin_html_body(read_response).await;
        assert_admin_csr_shell(&read_html);
    })
    .await;
    let stored = sqlx::query_as::<
        _,
        (
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
        ),
    >(
        "SELECT site_name, default_admin_route, tab_title, main_logo, primary_color, organization_name, organization_contacts, support_url FROM system_settings WHERE id = 1",
    )
    .fetch_one(&fixture.pool.0)
    .await
    .expect("a8f201de postgresql_html_settings_updates_and_reads_every_field_separately invariant must hold");
    assert_eq!(stored.0, site_name_b.0);
    assert_eq!(stored.1, route_b.0);
    assert_eq!(stored.2, tab_title_b.0);
    assert_eq!(stored.3, main_logo_b.0);
    assert_eq!(stored.4, primary_color_b.0);
    assert_eq!(stored.5, organization_name_b.0);
    assert_eq!(stored.6, organization_contacts_b.0);
    assert_eq!(stored.7, support_url_b.0);
    let empty = StdAdminApiTestStrRef::from(str_constants::PG_CRUD_EMPTY_SQL_SUFFIX);
    let clear_states = [
        (
            AdminHtmlSettingsTestValues {
                default_admin_route: route_b,
                main_logo: main_logo_b,
                organization_contacts: organization_contacts_b,
                organization_name: organization_name_b,
                primary_color: primary_color_b,
                site_name: site_name_b,
                support_url: support_url_b,
                tab_title: empty,
            },
            usize_constants::ONE,
        ),
        (
            AdminHtmlSettingsTestValues {
                default_admin_route: route_b,
                main_logo: empty,
                organization_contacts: organization_contacts_b,
                organization_name: organization_name_b,
                primary_color: primary_color_b,
                site_name: site_name_b,
                support_url: support_url_b,
                tab_title: empty,
            },
            2usize,
        ),
        (
            AdminHtmlSettingsTestValues {
                default_admin_route: route_b,
                main_logo: empty,
                organization_contacts: organization_contacts_b,
                organization_name: organization_name_b,
                primary_color: empty,
                site_name: site_name_b,
                support_url: support_url_b,
                tab_title: empty,
            },
            3usize,
        ),
        (
            AdminHtmlSettingsTestValues {
                default_admin_route: route_b,
                main_logo: empty,
                organization_contacts: organization_contacts_b,
                organization_name: empty,
                primary_color: empty,
                site_name: site_name_b,
                support_url: support_url_b,
                tab_title: empty,
            },
            4usize,
        ),
        (
            AdminHtmlSettingsTestValues {
                default_admin_route: route_b,
                main_logo: empty,
                organization_contacts: empty,
                organization_name: empty,
                primary_color: empty,
                site_name: site_name_b,
                support_url: support_url_b,
                tab_title: empty,
            },
            5usize,
        ),
        (
            AdminHtmlSettingsTestValues {
                default_admin_route: route_b,
                main_logo: empty,
                organization_contacts: empty,
                organization_name: empty,
                primary_color: empty,
                site_name: site_name_b,
                support_url: empty,
                tab_title: empty,
            },
            6usize,
        ),
    ];
    futures::StreamExt::fold(
        futures::stream::iter(clear_states),
        (),
        async |(), (values, expected_cleared)| {
            let form_body = values.form_body();
            let clear_response = admin_html_response(
                fixture_ref,
                HttpAdminApiTestMethod::from(http::Method::POST),
                StdAdminApiTestStrRef::from(
                    server_admin_contract::AdminHtmlAction::SettingsUpdate.get(),
                ),
                StdAdminApiTestStrRef::from(form_body.0.as_str()),
            )
            .await;
            assert_eq!(clear_response.status(), http::StatusCode::SEE_OTHER);
            let optional_values = sqlx::query_as::<
                _,
                (
                    String,
                    String,
                    String,
                    String,
                    String,
                    String,
                ),
            >(
                "SELECT tab_title, main_logo, primary_color, organization_name, organization_contacts, support_url FROM system_settings WHERE id = 1",
            )
            .fetch_one(&fixture_ref.pool.0)
            .await
            .expect("d418f9c0 postgresql_html_settings_updates_and_reads_every_field_separately invariant must hold");
            assert_eq!(
                [
                    (
                        optional_values.0.as_str(),
                        str_constants::ADMIN,
                    ),
                    (
                        optional_values.1.as_str(),
                        str_constants::ADMIN_DEFAULT_MAIN_LOGO,
                    ),
                    (
                        optional_values.2.as_str(),
                        str_constants::PRIMARY_COLOR_DEFAULT,
                    ),
                    (
                        optional_values.3.as_str(),
                        str_constants::ADMIN,
                    ),
                    (
                        optional_values.4.as_str(),
                        str_constants::ADMIN_DEFAULT_ORGANIZATION_CONTACTS,
                    ),
                    (
                        optional_values.5.as_str(),
                        str_constants::ADMIN_DEFAULT_SUPPORT_URL,
                    ),
                ]
                .iter()
                .filter(|(value, default)| value == default)
                .count(),
                expected_cleared,
            );
        },
    )
    .await;
    fixture.lock.0.rollback().await.expect("c7659b40 postgresql_html_settings_updates_and_reads_every_field_separately invariant must hold");
}
#[tokio::test]
#[ignore = "requires PostgreSQL; run through workspace_test_runner database"]
async fn postgresql_bootstrap_password_must_change_before_admin_access() {
    let fixture =
        admin_html_test_fixture_with_password_change(server_admin_contract::AdminBool::from(true))
            .await;
    let users_response = admin_html_response(
        &fixture,
        HttpAdminApiTestMethod::from(http::Method::GET),
        StdAdminApiTestStrRef::from(server_admin_contract::AdminFrontendPath::Users.get()),
        StdAdminApiTestStrRef::from(str_constants::PG_CRUD_EMPTY_SQL_SUFFIX),
    )
    .await;
    assert_eq!(users_response.status(), http::StatusCode::SEE_OTHER);
    assert_eq!(
        users_response.headers().get(http::header::LOCATION),
        Some(&http::HeaderValue::from_static(
            server_admin_contract::AdminFrontendPath::Profile.get(),
        ))
    );
    let profile_response = admin_html_response(
        &fixture,
        HttpAdminApiTestMethod::from(http::Method::GET),
        StdAdminApiTestStrRef::from(server_admin_contract::AdminFrontendPath::Profile.get()),
        StdAdminApiTestStrRef::from(str_constants::PG_CRUD_EMPTY_SQL_SUFFIX),
    )
    .await;
    assert_eq!(profile_response.status(), http::StatusCode::OK);
    let correct_password =
        serde_json::from_str::<String>(str_constants::CORRECT_PASSWORD).expect("e20a72a8 postgresql_bootstrap_password_must_change_before_admin_access invariant must hold");
    let change_password_body = AdminHtmlTestFormBody::try_from(format!(
        "current_password={correct_password}&new_password=Bootstrap-changed-pass2",
    ))
    .expect("b42a390d postgresql_bootstrap_password_must_change_before_admin_access invariant must hold");
    let change_password_response = admin_html_response(
        &fixture,
        HttpAdminApiTestMethod::from(http::Method::POST),
        StdAdminApiTestStrRef::from(server_admin_contract::AdminHtmlAction::ProfilePassword.get()),
        StdAdminApiTestStrRef::from(change_password_body.0.as_str()),
    )
    .await;
    assert_eq!(
        change_password_response.status(),
        http::StatusCode::SEE_OTHER
    );
    let password_change_required = sqlx::query_scalar::<_, bool>(
        str_constants::SELECT_MUST_CHANGE_PASSWORD_FROM_ADMIN_USERS_WHERE_LOGIN_ADMIN,
    )
    .fetch_one(&fixture.pool.0)
    .await
    .expect("ea57fc2d postgresql_bootstrap_password_must_change_before_admin_access invariant must hold");
    assert!(!password_change_required);
    let post_change_users_response = admin_html_response(
        &fixture,
        HttpAdminApiTestMethod::from(http::Method::GET),
        StdAdminApiTestStrRef::from(server_admin_contract::AdminFrontendPath::Users.get()),
        StdAdminApiTestStrRef::from(str_constants::PG_CRUD_EMPTY_SQL_SUFFIX),
    )
    .await;
    assert_eq!(post_change_users_response.status(), http::StatusCode::OK);
    fixture.lock.0.rollback().await.expect("6a8ce0f3 postgresql_bootstrap_password_must_change_before_admin_access invariant must hold");
}

#[tokio::test]
#[ignore = "requires PostgreSQL; run through workspace_test_runner database"]
async fn postgresql_html_profile_reads_every_field_and_changes_own_password() {
    let fixture = admin_html_test_fixture().await;
    let profile_response = admin_html_response(
        &fixture,
        HttpAdminApiTestMethod::from(http::Method::GET),
        StdAdminApiTestStrRef::from(server_admin_contract::AdminFrontendPath::Profile.get()),
        StdAdminApiTestStrRef::from(str_constants::PG_CRUD_EMPTY_SQL_SUFFIX),
    )
    .await;
    assert_eq!(profile_response.status(), http::StatusCode::OK);
    let profile_html = admin_html_body(profile_response).await;
    assert_admin_csr_shell(&profile_html);

    let original_password_hash = sqlx::query_scalar::<_, String>(
        str_constants::SELECT_PASSWORD_HASH_FROM_ADMIN_USERS_WHERE_LOGIN_ADMIN,
    )
    .fetch_one(&fixture.pool.0)
    .await
    .expect("c09b5e4e postgresql_html_profile_reads_every_field_and_changes_own_password invariant must hold");
    let (current_session_id, user_id) = sqlx::query_as::<_, (uuid::Uuid, i64)>(
        "SELECT id, user_id FROM access_sessions WHERE revoked_at IS NULL",
    )
    .fetch_one(&fixture.pool.0)
    .await
    .expect("ae46b7c1 postgresql_html_profile_reads_every_field_and_changes_own_password invariant must hold");
    let other_session_id = uuid::Uuid::from_u128(2u128);
    let _inserted_other_session = sqlx::query(
        "INSERT INTO access_sessions (id, user_id, token_identifier_hash, csrf_token_hash, token_context_hash, expires_at) VALUES ($1, $2, 'other-token-hash', 'other-csrf-hash', repeat('a', 64), NOW() + INTERVAL '1 hour')",
    )
    .bind(other_session_id)
    .bind(user_id)
    .execute(&fixture.pool.0)
    .await
    .expect("3e216ecd postgresql_html_profile_reads_every_field_and_changes_own_password invariant must hold");
    let _inserted_other_refresh_token = sqlx::query(
        "INSERT INTO refresh_tokens (id, user_id, token_hash, expires_at) VALUES ($1, $2, 'other-refresh-hash', NOW() + INTERVAL '1 hour')",
    )
    .bind(uuid::Uuid::from_u128(3u128))
    .bind(user_id)
    .execute(&fixture.pool.0)
    .await
    .expect("d61fc342 postgresql_html_profile_reads_every_field_and_changes_own_password invariant must hold");
    let correct_password =
        serde_json::from_str::<String>(str_constants::CORRECT_PASSWORD).expect("c59b011a postgresql_html_profile_reads_every_field_and_changes_own_password invariant must hold");
    let change_password_body = AdminHtmlTestFormBody::try_from(format!(
        "current_password={correct_password}&new_password=Html-profile-pass2",
    ))
    .expect("c93d69e3 postgresql_html_profile_reads_every_field_and_changes_own_password invariant must hold");
    let change_password_response = admin_html_response(
        &fixture,
        HttpAdminApiTestMethod::from(http::Method::POST),
        StdAdminApiTestStrRef::from(server_admin_contract::AdminHtmlAction::ProfilePassword.get()),
        StdAdminApiTestStrRef::from(change_password_body.0.as_str()),
    )
    .await;
    assert_eq!(
        change_password_response.status(),
        http::StatusCode::SEE_OTHER
    );
    let changed_password_hash = sqlx::query_scalar::<_, String>(
        str_constants::SELECT_PASSWORD_HASH_FROM_ADMIN_USERS_WHERE_LOGIN_ADMIN,
    )
    .fetch_one(&fixture.pool.0)
    .await
    .expect("696330ca postgresql_html_profile_reads_every_field_and_changes_own_password invariant must hold");
    assert_ne!(changed_password_hash, original_password_hash);
    let current_session_revoked = sqlx::query_scalar::<_, bool>(
        "SELECT revoked_at IS NOT NULL FROM access_sessions WHERE id = $1",
    )
    .bind(current_session_id)
    .fetch_one(&fixture.pool.0)
    .await
    .expect("38923e84 postgresql_html_profile_reads_every_field_and_changes_own_password invariant must hold");
    assert!(!current_session_revoked);
    let other_session_revoked = sqlx::query_scalar::<_, bool>(
        "SELECT revoked_at IS NOT NULL FROM access_sessions WHERE id = $1",
    )
    .bind(other_session_id)
    .fetch_one(&fixture.pool.0)
    .await
    .expect("f0168dc5 postgresql_html_profile_reads_every_field_and_changes_own_password invariant must hold");
    assert!(other_session_revoked);
    let active_refresh_token_count = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM refresh_tokens WHERE revoked_at IS NULL",
    )
    .fetch_one(&fixture.pool.0)
    .await
    .expect("740d6dc9 postgresql_html_profile_reads_every_field_and_changes_own_password invariant must hold");
    assert_eq!(active_refresh_token_count, i64_constants::ZERO);
    let authenticated_response = admin_html_response(
        &fixture,
        HttpAdminApiTestMethod::from(http::Method::GET),
        StdAdminApiTestStrRef::from(server_admin_contract::AdminFrontendPath::Profile.get()),
        StdAdminApiTestStrRef::from(str_constants::PG_CRUD_EMPTY_SQL_SUFFIX),
    )
    .await;
    assert_eq!(authenticated_response.status(), http::StatusCode::OK);
    fixture.lock.0.rollback().await.expect("737bbbe6 postgresql_html_profile_reads_every_field_and_changes_own_password invariant must hold");
}
#[tokio::test]
#[ignore = "requires PostgreSQL; run through workspace_test_runner database"]
async fn postgresql_html_sessions_reads_every_field_and_revokes_session() {
    let fixture = admin_html_test_fixture().await;
    let admin_id =
        sqlx::query_scalar::<_, i64>(str_constants::SELECT_ID_FROM_ADMIN_USERS_WHERE_LOGIN_ADMIN)
            .fetch_one(&fixture.pool.0)
            .await
            .expect("7f0a7c64 postgresql_html_sessions_reads_every_field_and_revokes_session invariant must hold");
    let (session_id, _created_at, _expires_at) = sqlx::query_as::<_, (uuid::Uuid, String, String)>(
        str_constants::SERVER_ADMIN_LIST_ACTIVE_SESSIONS_SQL,
    )
    .bind(admin_id)
    .bind(100i64)
    .bind(i64_constants::ZERO)
    .fetch_one(&fixture.pool.0)
    .await
    .expect("32e44a86 postgresql_html_sessions_reads_every_field_and_revokes_session invariant must hold");
    let sessions_response = admin_html_response(
        &fixture,
        HttpAdminApiTestMethod::from(http::Method::GET),
        StdAdminApiTestStrRef::from(server_admin_contract::AdminFrontendPath::Sessions.get()),
        StdAdminApiTestStrRef::from(str_constants::PG_CRUD_EMPTY_SQL_SUFFIX),
    )
    .await;
    assert_eq!(sessions_response.status(), http::StatusCode::OK);
    let sessions_html = admin_html_body(sessions_response).await;
    assert_admin_csr_shell(&sessions_html);

    let revoke_body =
        AdminHtmlTestFormBody::try_from(format!("session_id={session_id}&confirmation=true"))
            .expect("2f8bea59 postgresql_html_sessions_reads_every_field_and_revokes_session invariant must hold");
    let revoke_response = admin_html_response(
        &fixture,
        HttpAdminApiTestMethod::from(http::Method::POST),
        StdAdminApiTestStrRef::from(server_admin_contract::AdminHtmlAction::SessionRevoke.get()),
        StdAdminApiTestStrRef::from(revoke_body.0.as_str()),
    )
    .await;
    assert_eq!(revoke_response.status(), http::StatusCode::SEE_OTHER);
    let revoked = sqlx::query_scalar::<_, bool>(
        "SELECT revoked_at IS NOT NULL FROM access_sessions WHERE id = $1",
    )
    .bind(session_id)
    .fetch_one(&fixture.pool.0)
    .await
    .expect("e443902e postgresql_html_sessions_reads_every_field_and_revokes_session invariant must hold");
    assert!(revoked);
    let rejected_response = admin_html_response(
        &fixture,
        HttpAdminApiTestMethod::from(http::Method::GET),
        StdAdminApiTestStrRef::from(server_admin_contract::AdminFrontendPath::Sessions.get()),
        StdAdminApiTestStrRef::from(str_constants::PG_CRUD_EMPTY_SQL_SUFFIX),
    )
    .await;
    assert_eq!(rejected_response.status(), http::StatusCode::SEE_OTHER);
    fixture.lock.0.rollback().await.expect("9f41b8bd postgresql_html_sessions_reads_every_field_and_revokes_session invariant must hold");
}
#[tokio::test]
#[ignore = "requires PostgreSQL; run through workspace_test_runner database"]
async fn postgresql_html_router_registers_every_owned_page_and_action() {
    let fixture = admin_html_test_fixture().await;
    let fixture_ref = &fixture;
    futures::StreamExt::fold(
        futures::StreamExt::filter(
            futures::stream::iter(server_admin_contract::AdminFrontendPath::all_pages()),
            |path| {
                std::future::ready(!matches!(
                    path,
                    server_admin_contract::AdminFrontendPath::Metrics
                        | server_admin_contract::AdminFrontendPath::Permissions
                        | server_admin_contract::AdminFrontendPath::Roles
                        | server_admin_contract::AdminFrontendPath::Tables
                        | server_admin_contract::AdminFrontendPath::Users
                ))
            },
        ),
        (),
        async |(), path| {
            let response = admin_html_response(
                fixture_ref,
                HttpAdminApiTestMethod::from(http::Method::GET),
                StdAdminApiTestStrRef::from(path.get()),
                StdAdminApiTestStrRef::from(str_constants::PG_CRUD_EMPTY_SQL_SUFFIX),
            )
            .await;
            assert!(
                !matches!(
                    response.status(),
                    http::StatusCode::NOT_FOUND
                        | http::StatusCode::METHOD_NOT_ALLOWED
                        | http::StatusCode::INTERNAL_SERVER_ERROR
                ),
                "frontend page {} returned {}",
                path.get(),
                response.status()
            );
            if matches!(
                path,
                server_admin_contract::AdminFrontendPath::Profile
                    | server_admin_contract::AdminFrontendPath::Sessions
                    | server_admin_contract::AdminFrontendPath::Settings
            ) {
                let html = admin_html_body(response).await;
                assert_admin_csr_shell(&html);
            }
        },
    )
    .await;
    futures::StreamExt::fold(
        futures::stream::iter(server_admin_contract::AdminDataTable::ALL),
        (),
        async |(), table| {
            let uri = table.frontend_path();
            let response = admin_html_response(
                fixture_ref,
                HttpAdminApiTestMethod::from(http::Method::GET),
                StdAdminApiTestStrRef::from(uri.as_ref()),
                StdAdminApiTestStrRef::from(str_constants::PG_CRUD_EMPTY_SQL_SUFFIX),
            )
            .await;
            assert_eq!(
                response.status(),
                http::StatusCode::OK,
                "table view {table} failed"
            );
            let html = admin_html_body(response).await;
            assert_admin_csr_shell(&html);
        },
    )
    .await;
    futures::StreamExt::fold(
        futures::stream::iter(server_admin_contract::AdminHtmlAction::ALL),
        (),
        async |(), action| {
            let response = tower::ServiceExt::oneshot(
                fixture_ref.router.0.clone(),
                html_request_with_peer(
                    HttpAdminApiTestMethod::from(http::Method::POST),
                    StdAdminApiTestStrRef::from(action.get()),
                    StdAdminApiTestStrRef::from(str_constants::PG_CRUD_EMPTY_SQL_SUFFIX),
                    None,
                )
                .0,
            )
            .await
            .expect("d9567273 postgresql_html_router_registers_every_owned_page_and_action invariant must hold");
            assert!(
                !matches!(
                    response.status(),
                    http::StatusCode::NOT_FOUND
                        | http::StatusCode::METHOD_NOT_ALLOWED
                        | http::StatusCode::INTERNAL_SERVER_ERROR
                ),
                "HTML action {} returned {}",
                action.get(),
                response.status()
            );
        },
    )
    .await;
    fixture.lock.0.rollback().await.expect(
        "c0c53cdc postgresql_html_router_registers_every_owned_page_and_action invariant must hold",
    );
}
#[tokio::test]
#[ignore = "requires PostgreSQL; run through workspace_test_runner database"]
async fn postgresql_html_crud_forms_enforce_auth_csrf_validation_conflict_and_filtering() {
    let fixture = admin_html_test_fixture().await;
    let unauthenticated_response = tower::ServiceExt::oneshot(
        fixture.router.0.clone(),
        html_request_with_peer(
            HttpAdminApiTestMethod::from(http::Method::GET),
            StdAdminApiTestStrRef::from(server_admin_contract::AdminFrontendPath::Users.get()),
            StdAdminApiTestStrRef::from(str_constants::PG_CRUD_EMPTY_SQL_SUFFIX),
            None,
        )
        .0,
    )
    .await
    .expect("184ec7b2 postgresql_html_crud_forms_enforce_auth_csrf_validation_conflict_and_filtering invariant must hold");
    assert_eq!(
        unauthenticated_response.status(),
        http::StatusCode::SEE_OTHER
    );
    assert_eq!(
        unauthenticated_response
            .headers()
            .get(http::header::LOCATION),
        Some(&http::HeaderValue::from_static(
            server_admin_contract::AdminFrontendPath::SignIn.get(),
        )),
    );

    let login = "html_form_contract_user";
    let valid_body = AdminHtmlTestFormBody::try_from(format!(
        "login={login}&display_name=HTML+Form+Contract+User&password=Html-form-pass1"
    ))
    .expect("94b36ec1 postgresql_html_crud_forms_enforce_auth_csrf_validation_conflict_and_filtering invariant must hold");
    let missing_csrf_response = tower::ServiceExt::oneshot(
        fixture.router.0.clone(),
        html_request_with_peer(
            HttpAdminApiTestMethod::from(http::Method::POST),
            StdAdminApiTestStrRef::from(server_admin_contract::AdminHtmlAction::UserCreate.get()),
            StdAdminApiTestStrRef::from(valid_body.0.as_str()),
            None,
        )
        .0,
    )
    .await
    .expect("e6013d7a postgresql_html_crud_forms_enforce_auth_csrf_validation_conflict_and_filtering invariant must hold");
    assert_eq!(missing_csrf_response.status(), http::StatusCode::FORBIDDEN);
    let unknown_field_body =
        AdminHtmlTestFormBody::try_from(format!("{}&unknown_field=true", valid_body.0))
            .expect("af2948d3 postgresql_html_crud_forms_enforce_auth_csrf_validation_conflict_and_filtering invariant must hold");
    let unknown_field_response = admin_html_response(
        &fixture,
        HttpAdminApiTestMethod::from(http::Method::POST),
        StdAdminApiTestStrRef::from(server_admin_contract::AdminHtmlAction::UserCreate.get()),
        StdAdminApiTestStrRef::from(unknown_field_body.0.as_str()),
    )
    .await;
    assert_eq!(
        unknown_field_response.status(),
        http::StatusCode::UNPROCESSABLE_ENTITY
    );
    let create_response = admin_html_response(
        &fixture,
        HttpAdminApiTestMethod::from(http::Method::POST),
        StdAdminApiTestStrRef::from(server_admin_contract::AdminHtmlAction::UserCreate.get()),
        StdAdminApiTestStrRef::from(valid_body.0.as_str()),
    )
    .await;
    assert_eq!(create_response.status(), http::StatusCode::SEE_OTHER);
    let duplicate_response = admin_html_response(
        &fixture,
        HttpAdminApiTestMethod::from(http::Method::POST),
        StdAdminApiTestStrRef::from(server_admin_contract::AdminHtmlAction::UserCreate.get()),
        StdAdminApiTestStrRef::from(valid_body.0.as_str()),
    )
    .await;
    assert_eq!(duplicate_response.status(), http::StatusCode::CONFLICT);
    let created_id = sqlx::query_scalar::<_, i64>("SELECT id FROM users WHERE login = $1")
        .bind(login)
        .fetch_one(&fixture.pool.0)
        .await
        .expect("378a4e50 postgresql_html_crud_forms_enforce_auth_csrf_validation_conflict_and_filtering invariant must hold");
    let filtered_path = AdminHtmlTestFormBody::try_from(format!(
        "{}?search={login}",
        server_admin_contract::AdminFrontendPath::Users.get()
    ))
    .expect("60bf2c91 postgresql_html_crud_forms_enforce_auth_csrf_validation_conflict_and_filtering invariant must hold");
    let filtered_response = admin_html_response(
        &fixture,
        HttpAdminApiTestMethod::from(http::Method::GET),
        StdAdminApiTestStrRef::from(filtered_path.0.as_str()),
        StdAdminApiTestStrRef::from(str_constants::PG_CRUD_EMPTY_SQL_SUFFIX),
    )
    .await;
    assert_eq!(filtered_response.status(), http::StatusCode::OK);
    let filtered_html = admin_html_body(filtered_response).await;
    assert_admin_csr_shell(&filtered_html);

    let role_id = sqlx::query_scalar::<_, i64>(str_constants::SERVER_ADMIN_READ_ADMIN_ROLE_ID_SQL)
        .fetch_one(&fixture.pool.0)
        .await
        .expect("bc10a764 postgresql_html_crud_forms_enforce_auth_csrf_validation_conflict_and_filtering invariant must hold");
    let stale_roles_body = AdminHtmlTestFormBody::try_from(format!(
        "user_id={created_id}&expected_role_ids={role_id}"
    ))
    .expect("1934ad6f postgresql_html_crud_forms_enforce_auth_csrf_validation_conflict_and_filtering invariant must hold");
    let stale_roles_response = admin_html_response(
        &fixture,
        HttpAdminApiTestMethod::from(http::Method::POST),
        StdAdminApiTestStrRef::from(server_admin_contract::AdminHtmlAction::UserRoles.get()),
        StdAdminApiTestStrRef::from(stale_roles_body.0.as_str()),
    )
    .await;
    assert_eq!(stale_roles_response.status(), http::StatusCode::CONFLICT);

    let role_name = "html_form_contract_role";
    let create_role_body =
        AdminHtmlTestFormBody::try_from(format!("name={role_name}")).expect("8cf4260d postgresql_html_crud_forms_enforce_auth_csrf_validation_conflict_and_filtering invariant must hold");
    let create_role_response = admin_html_response(
        &fixture,
        HttpAdminApiTestMethod::from(http::Method::POST),
        StdAdminApiTestStrRef::from(server_admin_contract::AdminHtmlAction::RoleCreate.get()),
        StdAdminApiTestStrRef::from(create_role_body.0.as_str()),
    )
    .await;
    assert_eq!(create_role_response.status(), http::StatusCode::SEE_OTHER);
    let duplicate_role_response = admin_html_response(
        &fixture,
        HttpAdminApiTestMethod::from(http::Method::POST),
        StdAdminApiTestStrRef::from(server_admin_contract::AdminHtmlAction::RoleCreate.get()),
        StdAdminApiTestStrRef::from(create_role_body.0.as_str()),
    )
    .await;
    assert_eq!(duplicate_role_response.status(), http::StatusCode::CONFLICT);
    let created_role_id = sqlx::query_scalar::<_, i64>("SELECT id FROM roles WHERE name = $1")
        .bind(role_name)
        .fetch_one(&fixture.pool.0)
        .await
        .expect("2643be19 postgresql_html_crud_forms_enforce_auth_csrf_validation_conflict_and_filtering invariant must hold");
    let permission_id =
        sqlx::query_scalar::<_, i64>("SELECT id FROM permissions ORDER BY id LIMIT 1")
            .fetch_one(&fixture.pool.0)
            .await
            .expect("d8134c5b postgresql_html_crud_forms_enforce_auth_csrf_validation_conflict_and_filtering invariant must hold");
    let stale_permissions_body = AdminHtmlTestFormBody::try_from(format!(
        "role_id={created_role_id}&expected_permission_ids={permission_id}"
    ))
    .expect("49fac702 postgresql_html_crud_forms_enforce_auth_csrf_validation_conflict_and_filtering invariant must hold");
    let stale_permissions_response = admin_html_response(
        &fixture,
        HttpAdminApiTestMethod::from(http::Method::POST),
        StdAdminApiTestStrRef::from(server_admin_contract::AdminHtmlAction::RolePermissions.get()),
        StdAdminApiTestStrRef::from(stale_permissions_body.0.as_str()),
    )
    .await;
    assert_eq!(
        stale_permissions_response.status(),
        http::StatusCode::CONFLICT
    );
    let delete_role_body =
        AdminHtmlTestFormBody::try_from(format!("role_id={created_role_id}&confirmation=true"))
            .expect("f1c637d8 postgresql_html_crud_forms_enforce_auth_csrf_validation_conflict_and_filtering invariant must hold");
    let delete_role_response = admin_html_response(
        &fixture,
        HttpAdminApiTestMethod::from(http::Method::POST),
        StdAdminApiTestStrRef::from(server_admin_contract::AdminHtmlAction::RoleDelete.get()),
        StdAdminApiTestStrRef::from(delete_role_body.0.as_str()),
    )
    .await;
    assert_eq!(delete_role_response.status(), http::StatusCode::SEE_OTHER);

    let unknown_delete_body = AdminHtmlTestFormBody::try_from(String::from(
        "user_id=9223372036854775807&confirmation=true",
    ))
    .expect("d96b20e4 postgresql_html_crud_forms_enforce_auth_csrf_validation_conflict_and_filtering invariant must hold");
    let unknown_delete_response = admin_html_response(
        &fixture,
        HttpAdminApiTestMethod::from(http::Method::POST),
        StdAdminApiTestStrRef::from(server_admin_contract::AdminHtmlAction::UserDelete.get()),
        StdAdminApiTestStrRef::from(unknown_delete_body.0.as_str()),
    )
    .await;
    assert_eq!(unknown_delete_response.status(), http::StatusCode::CONFLICT);

    let delete_body =
        AdminHtmlTestFormBody::try_from(format!("user_id={created_id}&confirmation=true"))
            .expect("4cf9072d postgresql_html_crud_forms_enforce_auth_csrf_validation_conflict_and_filtering invariant must hold");
    let delete_response = admin_html_response(
        &fixture,
        HttpAdminApiTestMethod::from(http::Method::POST),
        StdAdminApiTestStrRef::from(server_admin_contract::AdminHtmlAction::UserDelete.get()),
        StdAdminApiTestStrRef::from(delete_body.0.as_str()),
    )
    .await;
    assert_eq!(delete_response.status(), http::StatusCode::SEE_OTHER);
    fixture.lock.0.rollback().await.expect("7361eb5c postgresql_html_crud_forms_enforce_auth_csrf_validation_conflict_and_filtering invariant must hold");
}
#[cfg(test)]
use super::{
    AdminHtmlSettingsTestValues, AdminHtmlTestFormBody, HttpAdminApiTestMethod,
    StdAdminApiTestStrRef, admin_html_body, admin_html_response, admin_html_test_fixture,
    admin_html_test_fixture_with_password_change, assert_admin_csr_shell, html_request_with_peer,
};
