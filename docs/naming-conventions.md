# Naming conventions

Repository-owned identifiers use `snake_case` consistently:

- Rust modules, functions, methods, variables, and fields use `snake_case`.
- Environment variables use `SCREAMING_SNAKE_CASE`.
- Types and enum variants use Rust's conventional `UpperCamelCase`.
- HTTP path segments owned by this repository use `snake_case`, including
  `/swagger_ui`, `/git_info`, and `/admin/sign_in`.
- JSON field names and query parameter names owned by this repository use
  `snake_case`.

Names defined by an external protocol or an integrated third-party API keep
their required spelling. Compatibility boundaries must not be renamed merely
to satisfy the local convention. Examples include standard HTTP header names,
media types, OAuth parameter names, and third-party webhook payload fields.

Route and field spelling must come from the corresponding typed contract or
catalog. Consumers must not repeat path or field strings manually.
