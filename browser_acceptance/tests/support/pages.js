function page(name, path) {
  return Object.freeze({ name, path });
}

export const adminPages = Object.freeze({
  branding: page("branding", "/admin/branding"),
  health: page("health", "/admin/health"),
  metrics: page("metrics", "/admin/metrics"),
  rules: page("rules", "/admin/rules"),
  profile: page("profile", "/admin/profile"),
  roles: page("roles", "/admin/roles"),
  sessions: page("sessions", "/admin/sessions"),
  settings: page("settings", "/admin/settings"),
  users: page("users", "/admin/users"),
  version: page("version", "/admin/version")
});

export const tablePages = Object.freeze([
  Object.freeze({
    ...adminPages.users,
    headers: Object.freeze(["id", "login", "display_name", "must_change_password", "is_banned", "created_at", "updated_at", "actions"]),
    pagination: true,
    readOnly: true
  }),
  Object.freeze({
    ...adminPages.roles,
    headers: Object.freeze(["id", "name", "is_system", "created_at", "updated_at", "actions"]),
    pagination: true,
    readOnly: true
  }),
  Object.freeze({
    ...adminPages.rules,
    headers: Object.freeze(["id", "permission_resource_action_id", "basemap_id", "layer_group_id", "layer_id", "project_group_id", "project_id", "property_id", "role_id", "user_id", "feature_id", "value_item_id", "created_at", "actions"]),
    pagination: true,
    readOnly: true
  }),
  Object.freeze({
    ...adminPages.sessions,
    headers: Object.freeze(["id", "created", "expires", "current", "actions"]),
    pagination: true,
    readOnly: false
  })
]);

export const dataTablePages = Object.freeze(
  [
    "permission_actions",
    "permission_resource_actions",
    "permission_resources",
    "user_roles",
    "role_rules",
    "refresh_tokens",
    "access_sessions",
    "login_attempts",
    "audit_log",
    "system_settings",
    "rate_limits",
    "cleanup_status"
  ].map(name =>
    Object.freeze({
      name,
      path: `/admin/${name}`,
      snapshotName: name.replaceAll("_", "-")
    })
  )
);

export const dataTables = Object.freeze(
  Object.fromEntries(dataTablePages.map(value => [value.name, value]))
);

export const dataNavigationPaths = Object.freeze([
  adminPages.users.path,
  dataTables.user_roles.path,
  adminPages.roles.path,
  dataTables.role_rules.path,
  adminPages.rules.path,
  ...dataTablePages.filter(value => value.name !== "user_roles" && value.name !== "role_rules").map(value => value.path)
]);

export const navigationAdminPaths = Object.freeze([
  ...dataNavigationPaths,
  adminPages.branding.path,
  adminPages.health.path,
  "/admin/swagger_ui",
  adminPages.metrics.path,
  adminPages.profile.path,
  adminPages.sessions.path,
  adminPages.settings.path,
  adminPages.version.path
]);

export const serverRenderedPages = Object.freeze([
  Object.freeze({ ...adminPages.metrics, dynamic: true }),
  Object.freeze({ ...adminPages.version, dynamic: false })
]);

export const primaryAdminPaths = Object.freeze([
  adminPages.users.path,
  adminPages.roles.path,
  adminPages.rules.path,
  adminPages.sessions.path,
  adminPages.profile.path,
  adminPages.settings.path,
  adminPages.version.path
]);

export const diagnosticAdminPaths = Object.freeze([
  ...primaryAdminPaths.slice(0, 6),
  adminPages.metrics.path,
  adminPages.version.path
]);

export const mobileAdminPaths = Object.freeze([
  adminPages.users.path,
  adminPages.profile.path,
  adminPages.settings.path
]);
