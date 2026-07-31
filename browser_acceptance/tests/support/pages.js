function page(name, path) {
  return Object.freeze({ name, path });
}

export const adminPages = Object.freeze({
  metrics: page("metrics", "/admin/metrics"),
  permissions: page("permissions", "/admin/permissions"),
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
    headers: Object.freeze(["id", "login", "display_name", "banned", "roles"]),
    pagination: true,
    readOnly: true
  }),
  Object.freeze({
    ...adminPages.roles,
    headers: Object.freeze(["id", "name", "system", "permissions"]),
    pagination: true,
    readOnly: true
  }),
  Object.freeze({
    ...adminPages.permissions,
    headers: Object.freeze(["id", "permission"]),
    pagination: true,
    readOnly: true
  }),
  Object.freeze({
    ...adminPages.sessions,
    headers: Object.freeze(["session", "created", "expires", "current", "actions"]),
    pagination: false,
    readOnly: false
  })
]);

export const dataTablePages = Object.freeze(
  [
    "user_roles",
    "role_permissions",
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
  adminPages.roles.path,
  adminPages.permissions.path,
  ...dataTablePages.map(value => value.path)
]);

export const navigationAdminPaths = Object.freeze([
  ...dataNavigationPaths,
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
  adminPages.permissions.path,
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
