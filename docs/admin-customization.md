# Administrator customization model

Customization has four layers. A later layer does not silently override an earlier layer unless
the setting is explicitly designed to do so.

| Layer | Examples | Applied | Restart |
|---|---|---|---|
| Compile-time identity | crate names, repository URL, image and service identifiers | scaffold/project replacement | rebuild |
| Deployment configuration | public origin, cookies, JWT keys, database, telemetry, capabilities | validated environment | restart |
| Runtime branding | site and tab titles, logo, color, organization, support URL, default page | administrator settings | no restart |
| Application extensions | permissions, routes, resources, navigation, dashboard components | application modules and migrations | rebuild and migrate |

Deployment security policy always wins over runtime branding. Runtime values cannot enable
Swagger, weaken cookies, widen CORS, or change authentication policy. Logo values must be
same-origin asset paths; support links must pass the repository outbound URL policy. Empty values
are accepted only for optional settings.

Resetting branding means submitting the documented template defaults through the normal settings
mutation, not deleting rows. The operation must pass validation and create the same audit record
as any other settings change. Required values such as site name and default route cannot be
cleared.
