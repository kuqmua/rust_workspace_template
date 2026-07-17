# Аудит оставшихся источников правды

## Цель документа

Документ описывает, что ещё нужно отрефакторить или переиспользовать, чтобы для каждого
изменяемого понятия в workspace существовал один авторитетный типизированный источник.

Аудит выполнен по текущему состоянию Rust-кода, SQL migrations, OpenAPI wiring,
Leptos frontend, Playwright fixtures, конфигурации и генераторов `pg_crud`. Уже выполненный
рефакторинг не предлагается повторять. Исторические migrations по-прежнему считаются
неизменяемым журналом и не должны генерироваться заново.

## Что уже централизовано

- admin login, display name, role name и новый пароль валидируются типами из
  `server_admin_contract`; переиспользуемая password/text policy находится в `text_policy`;
- admin permissions принадлежат `server_admin_contract::AdminPermission`, а база сверяется и
  восстанавливается через reconciliation;
- route path и базовый HTTP method объявлены в `TypedRoute`, handler bindings собраны одним
  `route_registry`;
- body limit принадлежит `RouteFamily` и используется Axum, OpenAPI и test fixture;
- Rust table descriptors сверяются с применёнными migrations по колонкам, nullability,
  server defaults и primary/foreign/unique keys;
- Playwright получает paths, permission catalog и успешные DTO из Rust-generated fixture;
- общие URL-safe, bounded/NUL-free, fixed hex и password validators вынесены в `text_policy`;
- production proc-macro entrypoints и `workspace_test_runner` вызывают одни и те же публичные
  `generate_*_src` функции;
- route, page и permission values больше не принадлежат `str_constants`.

Это хорошая основа, но наличие одного `TypedRoute` или одного enum само по себе ещё не
гарантирует, что все производные представления строятся из него.

## Сводка оставшихся проблем

| Приоритет | Область | Независимые представления сейчас | Целевой владелец |
|---|---|---:|---|
| P0 | Client route contract | `TypedRoute` + большой `AdminRoute::contract()` | `TypedRoute` metadata и typed parameter binding |
| P0 | OpenAPI errors/security | route access + 23 `route_openapi` response/security списка | typed endpoint policy рядом с route type |
| P0 | Admin system settings | общий DTO text + handler checks + SQL defaults/checks + frontend paths | отдельные setting types в `server_admin_contract` |
| P0 | Auth/security policy | config wrappers + runtime wrappers + magic thresholds | shared typed security policy |
| P1 | Полная DB conformance | migrations + частичный generated descriptor contract | migrations + обязательный полный catalog snapshot/test |
| P1 | SQL и sort identifiers | `str_constants`, SQL text, table fields, frontend sort arrays | repository/query types и field descriptors |
| P1 | Frontend page catalog | `AdminFrontendPath` + `AdminPage` + `AdminRoute` + UI match arms | typed `AdminPageSpec` catalog |
| P1 | Конфигурационный schema | `Config` fields + `.env.example` values + CI env | generated config descriptor |
| P2 | `pg_crud` typed IR | три source crate и локальные модели внутри emitter | общий non-proc-macro IR/pipeline |
| P2 | Общие operational policies | локальные rate/cleanup/retry числа | маленькие typed policy structs |

## P0. Удалить второй route contract из `AdminRoute`

### Факт

`server_admin_contract::AdminRoute::contract()` содержит большой `match`, который вручную
повторяет для клиентского enum:

- authentication/permission;
- HTTP method;
- mutation kind;
- success status;
- связь с route path.

Paths для большинства вариантов уже читаются через `typed_route_path`, но остальные поля
повторяют смысл `TypedRoute` и handler/OpenAPI contracts. Например, изменение method или success
status может скомпилироваться в server registry, оставив frontend transport со старым значением.
Параметризованные варианты (`UpdateUser(id)`, `DeleteRole(id)` и подобные) дополнительно вручную
форматируют path parameters.

### Целевое решение

Расширить route metadata минимальными отдельными контрактами:

- required permission как `Option<AdminPermission>` или типизированная access policy;
- success status;
- typed path parameter builder.

`AdminRoute` должен стать только удобной runtime-проекцией route type + параметров. Его
`contract()` должен делегировать metadata конкретного route type, а не собирать новый
`RouteContract` вручную. Для routes без параметров допустима generated/declared binding table;
для routes с параметрами нужен отдельный typed parameter object, а не строковый `format!` в enum.

Не следует превращать `TypedRoute` в универсальный макрос, генерирующий router, UI и handlers.
Нужны отдельные contracts: route metadata, path binding и handler binding.

### Критерии готовности

- method, mutation, permission и success status каждого admin endpoint записаны один раз;
- `AdminRoute::contract()` не содержит ручной таблицы этих значений;
- compile-time test ломается при несовместимом route parameter type;
- mutation test одного `TypedRoute` доказывает, что frontend transport и server registry меняются
  вместе;
- search-test запрещает новые ручные `RouteContract::new` для уже типизированных admin routes.

## P0. Сделать endpoint policy источником OpenAPI responses и security

### Факт

Каждый handler в `server_admin/src/auth.rs` имеет `#[route_openapi(...)]` с независимо набранными
response statuses, response DTO и security schemes. `TypedRoute` уже знает request/response,
access и mutation, но OpenAPI error surface остаётся второй таблицей. В текущем файле таких
annotations 23.

Это оставляет возможными расхождения:

- success DTO route type и `route_openapi` body;
- permission/access policy и OpenAPI security;
- реальные `AdminApiError` варианты и объявленные 401/403/409/422/429/500;
- retry header и rate-limit capability.

### Целевое решение

Ввести небольшой typed `EndpointPolicy`, принадлежащий route type или route binding:

- success status берётся из route metadata;
- success body берётся из `TypedRoute::Response`;
- security выводится из access/CSRF policy;
- error cases задаются типизированным набором возможностей endpoint, например
  `Validation`, `Conflict`, `RateLimited`, а не сырыми status numbers;
- OpenAPI responses, runtime error mapping и coverage obligations строятся из этого набора.

Handler-specific описание/tag может остаться рядом с handler, если оно не влияет на wire
semantics.

### Критерии готовности

- в handler annotations нет ручного success response body/status;
- security scheme не повторяет route access/CSRF policy;
- тест сравнивает возможные runtime `AdminApiError` категории с OpenAPI responses;
- добавление `RateLimited` автоматически добавляет 429 и `Retry-After`;
- compile-time test отклоняет endpoint policy с response type, отличным от `TypedRoute::Response`.

## P0. Типизировать admin system settings

### Факт

`AdminSettingsView` и `AdminUpdateSettingsReq` используют общий `AdminSettingText` почти для всех
полей. Семантика затем проверяется отдельно:

- handler вручную проверяет trimmed/non-empty `site_name`;
- handler принимает `default_admin_route` по `starts_with("/admin")`;
- migration задаёт default `'/admin/users'`, `site_name = 'Admin'` и SQL CHECK constraints;
- frontend показывает `AdminFrontendPath::Users` как placeholder, но DTO допускает произвольный
  текст с подходящим префиксом.

SQL default route может перестать быть членом frontend catalog, а Rust/OpenAPI этого не заметят.

### Целевое решение

Создать отдельные contract types хотя бы для:

- `AdminSiteName`;
- `AdminDefaultRoute`;
- `AdminPrimaryColor`;
- `AdminSupportUrl`;
- nullable organization/logo/contact values.

`AdminDefaultRoute` должен строиться из разрешённого `AdminFrontendPath`/`AdminPage`, либо иметь
явный policy для расширяемых plugin paths. SQL остаётся последней линией защиты; conformance test
должен проверять Rust policy против SQL CHECK и проверять, что migration default парсится typed
моделью.

### Критерии готовности

- handler не содержит ручных `trim`/`starts_with` checks для setting fields;
- OpenAPI schema каждого setting отражает его реальную policy;
- migration defaults проходят typed parser;
- PostgreSQL и Rust дают одинаковый результат на boundary/invalid cases;
- frontend form работает с предметными типами, а не с одним `AdminSettingText`.

## P0. Объединить auth configuration и runtime security types

### Факт

`config_lib::AdminTokenIssuer`/`AdminTokenAudience` и
`server_admin::AdminTokenIssuer`/`AdminTokenAudience` независимо повторяют `max = 256`. При
построении auth state config value снова конвертируется через `String`.

Рядом остаются локальные operational числа:

- sign-in failure threshold `10`;
- rate-limit windows/counts `60` и `300`;
- multiplier `5`;
- artificial failure delay `200 ms`;
- session/access/refresh conversions и fallback limits.

Часть значений приходит из `Config`, часть зашита в handlers/audit/rate-limit code, поэтому
эффективную security policy нельзя получить одним typed snapshot.

### Целевое решение

- Перенести issuer/audience validation в shared admin security/contract type; `Config` должен
  хранить этот тип напрямую или тонкий secret/config wrapper вокруг него.
- Ввести immutable `AdminAuthPolicy`, состоящий из предметных wrappers для thresholds, windows,
  delay, TTL и session limit.
- Создавать policy один раз из `server_config::Config`, затем передавать её в auth state.
- Не помещать секрет JWT в общий публичный contract crate; для secret material оставить отдельный
  security crate или server-only wrapper.

Отдельно `token_context_hash` сейчас связан с SHA-256 implementation и SQL regex
`^[0-9a-f]{64}$`. Следует ввести secret/redacted fixed digest type, переиспользующий fixed-hex
policy, и conformance test с migration constraint.

### Критерии готовности

- issuer/audience limits объявлены один раз;
- runtime code не конвертирует валидированный config type обратно через raw `String`;
- все auth thresholds доступны из одного `AdminAuthPolicy` snapshot;
- `AdminTokenHash` гарантирует алгоритм/encoding/length на уровне типа;
- DB constraint для context hash проверяется тестом относительно того же digest type.

## P1. Расширить schema conformance до CHECK, index и точных defaults

### Факт

`validate_generated_postgres_table` проверяет generated descriptor по колонкам, nullability,
наличию server default и PK/FK/unique keys. В `pg_crud_common::db_schema_conformance` уже есть более
полные `inspect_postgres_table` и `inspect_postgres_catalog`, умеющие читать CHECK constraints,
indexes, triggers, functions и views, но admin descriptor test их не использует.

Из-за этого тест не заметит, например:

- удаление login/display-name CHECK;
- изменение singleton/default-route constraint;
- изменение partial/session index;
- изменение конкретного default expression при сохранении факта наличия default;
- расхождение trigger/function contract.

### Целевое решение

Не копировать полные SQL expressions в `str_constants`. Добавить к schema expectation
типизированные `DbCheckSpec`, `DbIndexSpec` и normalized default expectation. Для сложных
объектов, которые нельзя разумно выразить typed model, хранить reviewed normalized catalog
snapshot рядом с owning migration/domain crate и сравнивать с применённой чистой БД.

### Критерии готовности

- обязательный admin DB test покрывает CHECK constraints и indexes;
- default проверяется по нормализованному значению, а не только как boolean;
- test mutation каждого object kind демонстрирует детерминированное падение;
- CI запускает проверку на чистой PostgreSQL после всех migrations;
- historical migrations не переписываются.

## P1. Убрать SQL и frontend field catalogs из `str_constants`

### Факт

В `server_admin` около 74 `sqlx::query*` sites; значительная часть использует большие
`SERVER_ADMIN_*_SQL` constants из `str_constants`. Это делает глобальный текстовый crate
неявным repository API. Один и тот же query используется несколькими handlers, но у него нет
владельца, result/bind contract или предметного имени типа.

Там же остаются `ADMIN_TABLE_USER_SORTS`, `ADMIN_TABLE_ROLE_SORTS`,
`ADMIN_TABLE_PERMISSION_SORTS` и `ADMIN_TABLE_AUDIT_SORTS`. Они повторяют поля DTO/table и labels,
а frontend принимает sort key как свободный text wrapper.

### Целевое решение

- Переместить SQL в owning repository modules (`users`, `sessions`, `roles`, `audit`,
  `permissions`) как private typed query functions.
- На boundary принимать domain wrappers и возвращать domain records, скрывая bind order/raw
  tuples.
- Общие transactional operations, например revoke-session или last-admin guard, сделать одной
  shared repository function вместо переиспользования строки.
- Генерировать/объявлять frontend sortable fields рядом с DTO/table field descriptor. Использовать
  enum `AdminUserSortField` и аналоги, содержащий wire key и label metadata.
- В `str_constants` оставить только reusable wording/labels, но не query или field catalog.

### Критерии готовности

- production handler не импортирует `*_SQL` из `str_constants`;
- bind/result shape находится в одной repository function;
- sort key нельзя создать из неизвестного raw string без `TryFrom` error;
- table/DTO rename ломает compile/conformance test, а не silently оставляет старый sort key;
- code-style gate запрещает новые production SQL constants и `ADMIN_TABLE_*_SORTS` в
  `str_constants`.

## P1. Свести frontend page metadata в один catalog

### Факт

Сейчас связаны ручными match/array mappings:

- `AdminFrontendPath`;
- `AdminPage` и `AdminPage::ALL`;
- `AdminPage::path`, `route`, `title`;
- `AdminRoute` permission;
- frontend load match arms и navigation rendering;
- swagger-enabled filtering.

Типы полезны, но добавление страницы всё ещё требует синхронного изменения нескольких таблиц.

### Целевое решение

В `server_admin_contract` определить небольшой `AdminPageSpec` catalog:

- page identity;
- frontend path;
- data route или local page kind;
- title key;
- required permission;
- visibility capability (`swagger`, `metrics`, всегда доступна);
- navigation order.

Frontend router, navigation, authorization и test fixture должны читать этот catalog. Rendering
function может оставаться явным match по page identity: UI component не нужно скрывать в макросе.

### Критерии готовности

- page path/title/permission/navigation order объявлены один раз;
- добавление page требует одной catalog entry и одного renderer;
- swagger-disabled filtering следует capability, а не сравнению path;
- exhaustive test доказывает, что каждая visible page имеет renderer и корректный route;
- SQL `default_admin_route` валидируется относительно этого catalog.

## P1. Создать typed config descriptor

### Факт

`TryFromEnv` уже выводит env key из имени поля `server_config::Config`, а code-style test сверяет
набор ключей `.env` и `.env.example`. Однако constraints, sensitivity, examples/recommended
defaults и CI values остаются независимыми представлениями. Например admin TTL/session/rate
limits и pool timeouts видны только через набор wrapper types и текстовые env files.

### Целевое решение

Расширить отдельный config metadata contract, не сам parser-макрос:

- env name;
- value type/parser;
- secret flag;
- required/optional;
- safe development example;
- documentation/constraints.

Из descriptor генерировать или проверять `.env.example` и CI test environment. Secret examples
должны быть явно test-only и никогда не попадать в production logs.

### Критерии готовности

- `.env.example` проверяется не только по keys, но и по parseability всех non-secret examples;
- missing descriptor для нового `Config` field является compile/test error;
- CI values проходят тот же parser до запуска server tests;
- документация конфигурации строится из descriptor;
- runtime `Config` остаётся единственным владельцем набора полей.

## P2. Завершить общий typed IR для `pg_crud`

### Факт

Production proc macros и benchmark runner теперь вызывают одни `generate_*_src` entrypoints, что
устраняет прежнее дублирование test parser. Но целевой typed IR ещё не полностью достигнут:

- `generate_pg_table_src::source` содержит локальные `GeneratePgTableInputModel`, field/variant
  models и stage functions внутри большого emitter scope;
- `generate_pg_types_src` и `generate_where_filters_src` имеют собственные private spec models;
- общие concepts (field capability, route operation, wire/schema shape, diagnostics) не имеют
  одного shared representation;
- public pipeline всё ещё в основном возвращает `TokenStream`, поэтому stage tests не могут
  независимо проверять validated model.

### Целевое решение

В shared non-proc-macro crate выделить явные стадии:

```text
parse tokens -> ParsedConfig -> build domain model -> ValidatedModel -> emit tokens
```

Общий crate должен содержать только действительно общие concepts и diagnostics. Специфичные
table/type/filter models могут оставаться отдельными, но должны быть public внутри workspace и
оборачивать общий stage contract. Нельзя создавать один огромный enum всех generator variants.

`workspace_test_runner` должен benchmark-ить `parse/build/validate/emit` того же pipeline, а stage
tests — сравнивать typed models, не искать текстовые признаки в `TokenStream::to_string()`.

### Критерии готовности

- parsing и validation каждой config выполняются ровно одной функцией;
- proc macro и benchmark принимают один validated model API;
- stage timings измеряют реальные typed stages;
- diagnostics являются typed errors до финального `compile_error!` rendering;
- model tests могут выполняться без генерации Rust source;
- нет повторного parsing generated/intermediate token stream между стадиями.

## P2. Выделить остальные operational policies

После P0 security policy следует проверить оставшиеся локальные числа и правила:

- cleanup batch `1..=10_000` и retention `> 0`;
- pagination default `5`;
- text-search maximum `1024`;
- frontend transport/table text limits `4096`/`8192`;
- retry/backoff/timeouts в runtime и handlers.

Не каждое одинаковое число означает одинаковую семантику. Объединять следует только правила с
одной причиной изменения. Для каждого подтверждённого случая нужен маленький policy type в
owning crate и table-driven tests; случайные совпадения размеров объединять нельзя.

## Рекомендуемый порядок выполнения

1. Удалить ручной `AdminRoute::contract()` и связать client projection с `TypedRoute`.
2. Поверх той же metadata модели вывести OpenAPI success/security/error policies.
3. Типизировать admin settings и добавить conformance для defaults/CHECK.
4. Объединить issuer/audience, digest и auth thresholds в shared security policy.
5. Подключить полную schema/catalog conformance для admin migrations.
6. Перенести SQL в repository modules и заменить frontend sort strings typed enums.
7. Свести page/path/title/permission metadata в `AdminPageSpec`.
8. Добавить config descriptor и проверку `.env.example` values.
9. Завершить typed IR/stage API генераторов `pg_crud`.
10. После миграции усилить code-style gates для `str_constants` и raw policy numbers.

Первые два шага нужно выполнять вместе: иначе удаление client duplication может оставить второй
источник в OpenAPI или наоборот.

## Обязательная схема проверки каждого этапа

Для каждой изменяемой области итоговый review должен явно отвечать:

1. Какой crate и тип владеют значением?
2. Какие runtime/frontend/OpenAPI представления из него строятся?
3. Какое внешнее представление нельзя сгенерировать и каким conformance test оно проверяется?
4. Какой конкретный test падает при намеренном расхождении?

Дополнительно перед завершением каждого этапа:

- выполнить `cargo fmt`;
- выполнить `cargo clippy --all-targets --all-features -- -D warnings`;
- выполнить `cargo test -p tests code_style`;
- запустить затронутые DB tests на чистой PostgreSQL;
- для frontend contract changes пересобрать Trunk artifact и запустить Playwright;
- проверить `git diff --check` и отсутствие новых domain constants в `str_constants`.

## Что не следует делать

- не генерировать и не переписывать исторические migrations;
- не переносить все строки, числа или SQL в один новый «универсальный» crate;
- не считать одинаковые литералы одинаковой policy без общей причины изменения;
- не скрывать router, UI rendering, repository и OpenAPI в одном derive;
- не делать compile-time build зависимым от живой PostgreSQL;
- не заменять typed model JSON/token-string transport между стадиями;
- не расширять public API генераторов generics, которые нужны только внутреннему pipeline;
- не использовать `str_constants` как замену owning domain type.

## Итог

Наиболее серьёзный оставшийся риск — не сами строковые литералы, а параллельные typed и
полутипизированные таблицы вокруг уже существующего `TypedRoute`: client `AdminRoute` contract и
OpenAPI policy. После их устранения следующий максимальный выигрыш дадут предметные system
settings/security types и полная DB conformance. Перенос SQL и sort/page catalogs затем позволит
реально ограничить `str_constants` ролью reusable текста, а завершение `pg_crud` IR устранит
последнее крупное дублирование pipeline semantics.
