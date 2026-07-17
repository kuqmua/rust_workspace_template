const NOW: &str = "now()";
const FALSE: &str = "false";

pub fn admin_catalog_snapshot(
    schema: super::StdAdminStrRef<'_>,
) -> Result<pg_crud_common::DbCatalogSnapshot, pg_crud_common::DbSchemaTextTryFromStringError> {
    fn snapshot(
        name: &str,
        kind: pg_crud_common::DbObjectKind,
        definition: String,
    ) -> Result<pg_crud_common::DbObjectSnapshot, pg_crud_common::DbSchemaTextTryFromStringError>
    {
        Ok(pg_crud_common::DbObjectSnapshot::new(
            pg_crud_common::DbSchemaText::try_from(name.to_owned())?,
            kind,
            pg_crud_common::DbSchemaText::try_from(definition)?,
        ))
    }
    let updated_at_function = format!(
        "CREATE OR REPLACE FUNCTION {}.admin_set_updated_at()\n RETURNS trigger\n LANGUAGE plpgsql\nAS $function$\nBEGIN\n    NEW.updated_at = NOW();\n    RETURN NEW;\nEND;\n$function$\n",
        schema.0
    );
    let audit_function = format!(
        "CREATE OR REPLACE FUNCTION {}.admin_audit_log_append_only()\n RETURNS trigger\n LANGUAGE plpgsql\nAS $function$\nBEGIN\n    IF TG_OP = 'DELETE' AND current_setting('app.admin_audit_cleanup', TRUE) = 'on' THEN\n        RETURN OLD;\n    END IF;\n    RAISE EXCEPTION 'admin_audit_log is append-only';\nEND;\n$function$\n",
        schema.0
    );
    Ok(pg_crud_common::DbCatalogSnapshot::new(vec![
        snapshot(
            "admin_audit_log_append_only",
            pg_crud_common::DbObjectKind::Function,
            audit_function,
        )?,
        snapshot(
            "admin_set_updated_at",
            pg_crud_common::DbObjectKind::Function,
            updated_at_function,
        )?,
        snapshot(
            "admin_audit_log_append_only_guard",
            pg_crud_common::DbObjectKind::Trigger,
            "admin_audit_log:BEFORE:DELETE:EXECUTE FUNCTION admin_audit_log_append_only()"
                .to_owned(),
        )?,
        snapshot(
            "admin_audit_log_append_only_guard",
            pg_crud_common::DbObjectKind::Trigger,
            "admin_audit_log:BEFORE:UPDATE:EXECUTE FUNCTION admin_audit_log_append_only()"
                .to_owned(),
        )?,
        snapshot(
            "admin_roles_set_updated_at",
            pg_crud_common::DbObjectKind::Trigger,
            "admin_roles:BEFORE:UPDATE:EXECUTE FUNCTION admin_set_updated_at()".to_owned(),
        )?,
        snapshot(
            "admin_system_settings_set_updated_at",
            pg_crud_common::DbObjectKind::Trigger,
            "admin_system_settings:BEFORE:UPDATE:EXECUTE FUNCTION admin_set_updated_at()"
                .to_owned(),
        )?,
        snapshot(
            "admin_users_set_updated_at",
            pg_crud_common::DbObjectKind::Trigger,
            "admin_users:BEFORE:UPDATE:EXECUTE FUNCTION admin_set_updated_at()".to_owned(),
        )?,
    ]))
}

fn default<Column, Expression>(
    column: Column,
    expression: Expression,
) -> pg_crud_common::DbDefaultSpec
where
    Column: Into<pg_crud_common::DbStaticSchemaText>,
    Expression: Into<pg_crud_common::DbStaticSchemaText>,
{
    pg_crud_common::DbDefaultSpec::new(column.into(), expression.into())
}
fn object<Name, Definition>(
    name: Name,
    kind: pg_crud_common::DbObjectKind,
    definition: Definition,
) -> pg_crud_common::DbObjectSpec
where
    Name: Into<pg_crud_common::DbStaticSchemaText>,
    Definition: Into<pg_crud_common::DbStaticSchemaText>,
{
    pg_crud_common::DbObjectSpec::new(name.into(), kind, definition.into())
}

impl pg_crud_common::DbExtendedTableSchema for super::generated_tables::AdminUsers {
    fn exact_defaults() -> Vec<pg_crud_common::DbDefaultSpec> {
        vec![
            default("is_banned", FALSE),
            default("created_at", NOW),
            default("updated_at", NOW),
        ]
    }
    fn checks_and_indexes() -> Vec<pg_crud_common::DbObjectSpec> {
        vec![
            object(
                "admin_users_display_name_length",
                pg_crud_common::DbObjectKind::Check,
                "CHECK (char_length(display_name) >= 1 AND char_length(display_name) <= 256)",
            ),
            object(
                "admin_users_display_name_trimmed",
                pg_crud_common::DbObjectKind::Check,
                "CHECK (display_name = btrim(display_name))",
            ),
            object(
                "admin_users_login_format",
                pg_crud_common::DbObjectKind::Check,
                "CHECK (login = lower(login) AND login ~ '^[a-z0-9_.-]+$'::text)",
            ),
            object(
                "admin_users_login_length",
                pg_crud_common::DbObjectKind::Check,
                "CHECK (char_length(login) >= 3 AND char_length(login) <= 128)",
            ),
            object(
                "admin_users_password_hash_not_empty",
                pg_crud_common::DbObjectKind::Check,
                "CHECK (char_length(password_hash) > 0)",
            ),
            object(
                "admin_users_login_lower_unq",
                pg_crud_common::DbObjectKind::Index,
                "CREATE UNIQUE INDEX admin_users_login_lower_unq ON public.admin_users USING btree (lower(login))",
            ),
        ]
    }
}
impl pg_crud_common::DbExtendedTableSchema for super::generated_tables::AdminRoles {
    fn exact_defaults() -> Vec<pg_crud_common::DbDefaultSpec> {
        vec![
            default("is_system", FALSE),
            default("created_at", NOW),
            default("updated_at", NOW),
        ]
    }
    fn checks_and_indexes() -> Vec<pg_crud_common::DbObjectSpec> {
        vec![
            object(
                "admin_roles_name_format",
                pg_crud_common::DbObjectKind::Check,
                "CHECK (name = lower(name) AND name ~ '^[a-z0-9_.-]+$'::text)",
            ),
            object(
                "admin_roles_name_length",
                pg_crud_common::DbObjectKind::Check,
                "CHECK (char_length(name) >= 1 AND char_length(name) <= 128)",
            ),
        ]
    }
}
impl pg_crud_common::DbExtendedTableSchema for super::generated_tables::AdminPermissions {
    fn exact_defaults() -> Vec<pg_crud_common::DbDefaultSpec> {
        vec![default("created_at", NOW)]
    }
    fn checks_and_indexes() -> Vec<pg_crud_common::DbObjectSpec> {
        vec![
            object(
                "admin_permissions_name_format",
                pg_crud_common::DbObjectKind::Check,
                "CHECK (name = lower(name) AND name ~ '^[a-z0-9_]+:[a-z0-9_]+$'::text)",
            ),
            object(
                "admin_permissions_name_length",
                pg_crud_common::DbObjectKind::Check,
                "CHECK (char_length(name) >= 3 AND char_length(name) <= 128)",
            ),
        ]
    }
}
impl pg_crud_common::DbExtendedTableSchema for super::generated_tables::AdminUserRoles {
    fn exact_defaults() -> Vec<pg_crud_common::DbDefaultSpec> {
        vec![default("created_at", NOW)]
    }
    fn checks_and_indexes() -> Vec<pg_crud_common::DbObjectSpec> {
        vec![object(
            "admin_user_roles_role_id_idx",
            pg_crud_common::DbObjectKind::Index,
            "CREATE INDEX admin_user_roles_role_id_idx ON public.admin_user_roles USING btree (role_id)",
        )]
    }
}
impl pg_crud_common::DbExtendedTableSchema for super::generated_tables::AdminRolePermissions {
    fn exact_defaults() -> Vec<pg_crud_common::DbDefaultSpec> {
        vec![default("created_at", NOW)]
    }
    fn checks_and_indexes() -> Vec<pg_crud_common::DbObjectSpec> {
        vec![object(
            "admin_role_permissions_permission_id_idx",
            pg_crud_common::DbObjectKind::Index,
            "CREATE INDEX admin_role_permissions_permission_id_idx ON public.admin_role_permissions USING btree (permission_id)",
        )]
    }
}
impl pg_crud_common::DbExtendedTableSchema for super::generated_tables::AdminSystemSettings {
    fn exact_defaults() -> Vec<pg_crud_common::DbDefaultSpec> {
        vec![
            default("id", "1"),
            default("site_name", "'Admin'::text"),
            default("default_admin_route", "'/admin/users'::text"),
            default("updated_at", NOW),
        ]
    }
    fn checks_and_indexes() -> Vec<pg_crud_common::DbObjectSpec> {
        vec![
            object(
                "admin_system_settings_default_route_format",
                pg_crud_common::DbObjectKind::Check,
                "CHECK (default_admin_route ~~ '/admin%'::text)",
            ),
            object(
                "admin_system_settings_singleton",
                pg_crud_common::DbObjectKind::Check,
                "CHECK (id = 1)",
            ),
            object(
                "admin_system_settings_site_name_not_empty",
                pg_crud_common::DbObjectKind::Check,
                "CHECK (char_length(btrim(site_name)) > 0)",
            ),
        ]
    }
}
