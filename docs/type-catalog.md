# Каталог объявленных Rust-типов

Этот документ содержит аудит исходного снимка типов, явно объявленных как Rust items в исходниках workspace: `struct`, `enum`, `trait`, `trait alias`, `union` и `type alias`. Статусы относятся к этому зафиксированному набору; служебные error/helper-типы, добавленные при выполнении рекомендаций, не расширяют набор аудита.

Включены production-, test-, bench-, example- и build-модули всех workspace crates. Исключены `target/` и типы, существующие только как токены внутри macro/`quote!`; они не являются самостоятельными объявлениями исходного модуля до раскрытия макроса. Локальные items внутри функций и методов также исключены: каталог описывает именно типы модулей. Строка **Анализ** классифицирует форму и назначение объявления, а не доказывает его runtime-семантику.

Проанализированы все **74** workspace crates и **274** Rust-файла. Найдено **1823** объявления в **211** модулях: **1606** production- и **217** test-only объявлений. Типы есть в 57 crates; ещё 17 crates содержат только функции, константы, macro entrypoints или macro invocations без самостоятельных type items.

## Критерий рекомендации `TryFrom`

- **Да** — сырой вход способен представить недопустимое состояние типа: превысить длину/размер, нарушить диапазон, формат, уникальность или межполевой инвариант. Для одного естественного raw-представления предпочтителен `TryFrom`; для нескольких независимых аргументов эквивалентом остаётся именованный fallible-конструктор.
- **Нет** — отдельной проверки именно на границе этого типа не требуется: это trait/alias, закрытый enum, marker, borrowed/fixed-size wrapper либо композиция уже валидированных типов. Это не отменяет проверок внутри вложенных доменных типов.
- Наличие текущего `TryFrom`, validator-атрибута или fallible-конструктора считается прямым доказательством инварианта. Для одно-полевых wrappers также использован полный аудит растущих raw-типов из `docs/type-wrappers.md`.

Итог: **289** типов требуют проверяемой инициализации и имеют её, **1534** — не требуют отдельного `TryFrom` на собственной границе.

## Сводка

### По виду

| Вид | Количество |
|---|---:|
| `enum` | 331 |
| `struct` | 1447 |
| `trait` | 45 |

### По видимости

| Видимость | Количество |
|---|---:|
| `crate` | 42 |
| `private` | 525 |
| `public` | 1189 |
| `restricted` | 67 |

### Crates без самостоятельных объявлений типов

`generate_getter_traits_for_struct_fields`, `generate_pg_table`, `generate_pg_types`, `generate_where_filters`, `location_macros`, `naming_common_macros`, `pg_crud`, `pg_crud_common_macros`, `pg_crud_macros_common_macros`, `pg_types`, `pg_types_chrono_net`, `pg_types_numeric`, `pg_types_text_misc`, `server_app_state_macros`, `str_constants`, `to_err_string_macros`, `try_from_env`.

## Crate `app_state`

### Модуль `app_state`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `GetSqlxPgPool` | `trait` | `public` | behavior contract; 1 associated items; production | **Нет** | Не требуется | trait не инициализируется как значение | [`app_state/src/lib.rs:5`](../app_state/src/lib.rs#L5) |
| `SqlxPgPool` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`app_state/src/lib.rs:3`](../app_state/src/lib.rs#L3) |
| `SqlxPgPoolRef` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`app_state/src/lib.rs:1`](../app_state/src/lib.rs#L1) |

## Crate `common_routes`

### Модуль `common_routes`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `AxumCommonRoutes` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`common_routes/src/lib.rs:143`](../common_routes/src/lib.rs#L143) |
| `AxumHealthCheckStatus` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`common_routes/src/lib.rs:118`](../common_routes/src/lib.rs#L118) |
| `AxumHttpUriRef` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`common_routes/src/lib.rs:20`](../common_routes/src/lib.rs#L20) |
| `AxumJsonPayload` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`common_routes/src/lib.rs:125`](../common_routes/src/lib.rs#L125) |
| `CommonRoutesOpenApi` | `struct` | `public` | unit marker type; production | **Нет** | Не требуется | marker не содержит проверяемых данных | [`common_routes/src/lib.rs:147`](../common_routes/src/lib.rs#L147) |
| `CommonRoutesParameters` | `trait` | `public` | behavior contract; 0 associated items; production | **Нет** | Не требуется | trait не инициализируется как значение | [`common_routes/src/lib.rs:190`](../common_routes/src/lib.rs#L190) |
| `GitInfo` | `struct` | `public` | named-field data structure; 1 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`common_routes/src/lib.rs:8`](../common_routes/src/lib.rs#L8) |
| `HealthCheckSucceeded` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`common_routes/src/lib.rs:26`](../common_routes/src/lib.rs#L26) |
| `HealthComponent` | `struct` | `public` | named-field data structure; 2 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`common_routes/src/lib.rs:43`](../common_routes/src/lib.rs#L43) |
| `HealthComponentKind` | `enum` | `public` | closed variant set; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`common_routes/src/lib.rs:37`](../common_routes/src/lib.rs#L37) |
| `HealthComponents` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | внутреннее значение может расти; нужен предел длины/размера | [`common_routes/src/lib.rs:48`](../common_routes/src/lib.rs#L48) |
| `HealthComponentsError` | `struct` | `public` | unit marker type; production | **Нет** | Не требуется | marker не содержит проверяемых данных | [`common_routes/src/lib.rs:68`](../common_routes/src/lib.rs#L68) |
| `HealthDatabaseAvailable` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`common_routes/src/lib.rs:28`](../common_routes/src/lib.rs#L28) |
| `HealthReport` | `struct` | `public` | named-field data structure; 2 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`common_routes/src/lib.rs:71`](../common_routes/src/lib.rs#L71) |
| `HealthStatus` | `enum` | `public` | closed variant set; 3 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`common_routes/src/lib.rs:30`](../common_routes/src/lib.rs#L30) |
| `JsonRes` | `struct` | `private` | named-field data structure; 2 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`common_routes/src/lib.rs:120`](../common_routes/src/lib.rs#L120) |
| `NoRouteMessageCapacity` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`common_routes/src/lib.rs:24`](../common_routes/src/lib.rs#L24) |
| `NotFoundHandle` | `struct` | `private` | named-field data structure; 3 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`common_routes/src/lib.rs:12`](../common_routes/src/lib.rs#L12) |
| `OpenApiSpecificationPath` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`common_routes/src/lib.rs:18`](../common_routes/src/lib.rs#L18) |
| `StdArcCommonRoutesAppState` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`common_routes/src/lib.rs:145`](../common_routes/src/lib.rs#L145) |
| `UriSuffixRef` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`common_routes/src/lib.rs:22`](../common_routes/src/lib.rs#L22) |
| `UtoipaCommonRoutesOpenApiDocument` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`common_routes/src/lib.rs:159`](../common_routes/src/lib.rs#L159) |

### Модуль `common_routes::tests`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `TestState` | `struct` | `private` | named-field data structure; 1 fields; test-only | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`common_routes/src/lib.rs:397`](../common_routes/src/lib.rs#L397) |

## Crate `config_lib`

### Модуль `config_lib`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `AdminAccessTokenTtlSeconds` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`config_lib/src/lib.rs:262`](../config_lib/src/lib.rs#L262) |
| `AdminBoolParsingError` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`config_lib/src/lib.rs:423`](../config_lib/src/lib.rs#L423) |
| `AdminCookieSecure` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`config_lib/src/lib.rs:399`](../config_lib/src/lib.rs#L399) |
| `AdminJwtSecret` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | внутреннее значение может расти; нужен предел длины/размера | [`config_lib/src/lib.rs:158`](../config_lib/src/lib.rs#L158) |
| `AdminPasswordHashConcurrency` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`config_lib/src/lib.rs:361`](../config_lib/src/lib.rs#L361) |
| `AdminPositiveU64ParsingError` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`config_lib/src/lib.rs:306`](../config_lib/src/lib.rs#L306) |
| `AdminPositiveUsizeParsingError` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`config_lib/src/lib.rs:372`](../config_lib/src/lib.rs#L372) |
| `AdminRefreshTokenTtlSeconds` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`config_lib/src/lib.rs:273`](../config_lib/src/lib.rs#L273) |
| `AdminSessionLimit` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`config_lib/src/lib.rs:295`](../config_lib/src/lib.rs#L295) |
| `AdminSignInRateLimit` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`config_lib/src/lib.rs:284`](../config_lib/src/lib.rs#L284) |
| `AdminSwaggerEnabled` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`config_lib/src/lib.rs:410`](../config_lib/src/lib.rs#L410) |
| `AdminTokenAudience` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | атрибут валидатора задаёт инвариант типа | [`config_lib/src/lib.rs:473`](../config_lib/src/lib.rs#L473) |
| `AdminTokenIssuer` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | атрибут валидатора задаёт инвариант типа | [`config_lib/src/lib.rs:459`](../config_lib/src/lib.rs#L459) |
| `ChronoEastFixedOffset` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`config_lib/src/lib.rs:61`](../config_lib/src/lib.rs#L61) |
| `ChronoFixedOffsetError` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`config_lib/src/lib.rs:51`](../config_lib/src/lib.rs#L51) |
| `ChronoTimezone` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`config_lib/src/lib.rs:743`](../config_lib/src/lib.rs#L743) |
| `ConfigExampleValidity` | `enum` | `public` | closed variant set; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`config_lib/src/lib.rs:72`](../config_lib/src/lib.rs#L72) |
| `ConfigFieldDescriptor` | `struct` | `public` | named-field data structure; 4 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`config_lib/src/lib.rs:79`](../config_lib/src/lib.rs#L79) |
| `ConfigFieldSensitivity` | `enum` | `public` | closed variant set; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`config_lib/src/lib.rs:67`](../config_lib/src/lib.rs#L67) |
| `ConfigLibStringWrapperTryFromStringError` | `enum` | `public` | error enum; 1 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`config_lib/src/lib.rs:6`](../config_lib/src/lib.rs#L6) |
| `ConfigRustTypeName` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`config_lib/src/lib.rs:77`](../config_lib/src/lib.rs#L77) |
| `ContentSecurityPolicy` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`config_lib/src/lib.rs:653`](../config_lib/src/lib.rs#L653) |
| `ContentSecurityPolicyError` | `enum` | `public` | error enum; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`config_lib/src/lib.rs:655`](../config_lib/src/lib.rs#L655) |
| `EnvVarName` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`config_lib/src/lib.rs:32`](../config_lib/src/lib.rs#L32) |
| `EnvVarNameRef` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`config_lib/src/lib.rs:30`](../config_lib/src/lib.rs#L30) |
| `HttpGzipEnabled` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`config_lib/src/lib.rs:421`](../config_lib/src/lib.rs#L421) |
| `MaximumSizeOfHttpBodyInBytes` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`config_lib/src/lib.rs:534`](../config_lib/src/lib.rs#L534) |
| `MaximumSizeOfHttpBodyInBytesTryFromUsizeError` | `enum` | `public` | error enum; 1 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`config_lib/src/lib.rs:543`](../config_lib/src/lib.rs#L543) |
| `PgPoolAcquireTimeoutSeconds` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`config_lib/src/lib.rs:595`](../config_lib/src/lib.rs#L595) |
| `PgPoolConfigParseError` | `enum` | `public` | error enum; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`config_lib/src/lib.rs:603`](../config_lib/src/lib.rs#L603) |
| `PgPoolIdleTimeoutSeconds` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`config_lib/src/lib.rs:597`](../config_lib/src/lib.rs#L597) |
| `PgPoolMaxConnections` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`config_lib/src/lib.rs:584`](../config_lib/src/lib.rs#L584) |
| `PgPoolMaxConnectionsTryFromU32Error` | `enum` | `public` | error enum; 1 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`config_lib/src/lib.rs:681`](../config_lib/src/lib.rs#L681) |
| `PgPoolMaxLifetimeSeconds` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`config_lib/src/lib.rs:599`](../config_lib/src/lib.rs#L599) |
| `PgPoolMinConnections` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`config_lib/src/lib.rs:593`](../config_lib/src/lib.rs#L593) |
| `RequestTimeoutSeconds` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`config_lib/src/lib.rs:601`](../config_lib/src/lib.rs#L601) |
| `SecrecySecretBoxString` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`config_lib/src/lib.rs:129`](../config_lib/src/lib.rs#L129) |
| `StdEnvVarOk` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`config_lib/src/lib.rs:4`](../config_lib/src/lib.rs#L4) |
| `StdEnvVarOkRef` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`config_lib/src/lib.rs:28`](../config_lib/src/lib.rs#L28) |
| `StdI32ParsingError` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`config_lib/src/lib.rs:53`](../config_lib/src/lib.rs#L53) |
| `StdNonZeroU64` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`config_lib/src/lib.rs:150`](../config_lib/src/lib.rs#L150) |
| `StdNonZeroUsize` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`config_lib/src/lib.rs:152`](../config_lib/src/lib.rs#L152) |
| `StdParseBoolError` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`config_lib/src/lib.rs:156`](../config_lib/src/lib.rs#L156) |
| `StdParseIntError` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`config_lib/src/lib.rs:154`](../config_lib/src/lib.rs#L154) |
| `StdU32ParsingError` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`config_lib/src/lib.rs:55`](../config_lib/src/lib.rs#L55) |
| `StdUsizeParsingError` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`config_lib/src/lib.rs:57`](../config_lib/src/lib.rs#L57) |
| `TimezoneSeconds` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`config_lib/src/lib.rs:59`](../config_lib/src/lib.rs#L59) |
| `TryFromStdEnvVarOk` | `trait` | `public` | behavior contract; 2 associated items; production | **Нет** | Не требуется | trait не инициализируется как значение | [`config_lib/src/lib.rs:63`](../config_lib/src/lib.rs#L63) |
| `TryFromStdEnvVarOkAdminCookieSecureError` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`config_lib/src/lib.rs:425`](../config_lib/src/lib.rs#L425) |
| `TryFromStdEnvVarOkAdminJwtSecretError` | `enum` | `public` | error enum; 4 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`config_lib/src/lib.rs:176`](../config_lib/src/lib.rs#L176) |
| `TryFromStdEnvVarOkAdminPasswordHashConcurrencyError` | `enum` | `public` | error enum; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`config_lib/src/lib.rs:374`](../config_lib/src/lib.rs#L374) |
| `TryFromStdEnvVarOkAdminPositiveU64Error` | `enum` | `public` | error enum; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`config_lib/src/lib.rs:308`](../config_lib/src/lib.rs#L308) |
| `TryFromStdEnvVarOkAdminTokenTextError` | `enum` | `public` | error enum; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`config_lib/src/lib.rs:487`](../config_lib/src/lib.rs#L487) |
| `TryFromStdEnvVarOkMaximumSizeOfHttpBodyInBytesError` | `enum` | `public` | error enum; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`config_lib/src/lib.rs:558`](../config_lib/src/lib.rs#L558) |
| `TryFromStdEnvVarOkPgPoolMaxConnectionsError` | `enum` | `public` | error enum; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`config_lib/src/lib.rs:696`](../config_lib/src/lib.rs#L696) |
| `TryFromStdEnvVarOkTimezoneError` | `enum` | `public` | error enum; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`config_lib/src/lib.rs:763`](../config_lib/src/lib.rs#L763) |

### Модуль `config_lib::tests`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `ParseRequiredEnvVarTestError` | `enum` | `private` | error enum; 2 variants; test-only | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`config_lib/src/lib.rs:854`](../config_lib/src/lib.rs#L854) |

### Модуль `config_lib::types`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `EnvParseError` | `enum` | `private` | error enum; 3 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`config_lib/src/types.rs:25`](../config_lib/src/types.rs#L25) |
| `EnvVarNameRef` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`config_lib/src/types.rs:19`](../config_lib/src/types.rs#L19) |
| `EnvVarValueRef` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`config_lib/src/types.rs:21`](../config_lib/src/types.rs#L21) |
| `ParseCtxRef` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`config_lib/src/types.rs:23`](../config_lib/src/types.rs#L23) |
| `SrcPlaceType` | `enum` | `public` | closed variant set; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`config_lib/src/types.rs:94`](../config_lib/src/types.rs#L94) |
| `StdEnvVarError` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`config_lib/src/types.rs:5`](../config_lib/src/types.rs#L5) |
| `StdEnvVarResult` | `struct` | `private` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`config_lib/src/types.rs:3`](../config_lib/src/types.rs#L3) |
| `TracingFormat` | `enum` | `public` | closed variant set; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`config_lib/src/types.rs:72`](../config_lib/src/types.rs#L72) |
| `TracingLevel` | `enum` | `public` | closed variant set; 5 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`config_lib/src/types.rs:49`](../config_lib/src/types.rs#L49) |
| `TracingLevelName` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`config_lib/src/types.rs:1`](../config_lib/src/types.rs#L1) |

## Crate `config_lib_macros`

### Модуль `config_lib_macros`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `ProcMacro2TryFromParseFixedErrorTy` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | отдельный инвариант этого служебного типа не требуется; проверка принадлежит итоговой доменной границе | [`config_lib/config_lib_macros/src/lib.rs:4`](../config_lib/config_lib_macros/src/lib.rs#L4) |
| `ProcMacro2TryFromParseInput` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | отдельный инвариант этого служебного типа не требуется; проверка принадлежит итоговой доменной границе | [`config_lib/config_lib_macros/src/lib.rs:1`](../config_lib/config_lib_macros/src/lib.rs#L1) |
| `ProcMacroTryFromParseTokenStream` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | отдельный инвариант этого служебного типа не требуется; проверка принадлежит итоговой доменной границе | [`config_lib/config_lib_macros/src/lib.rs:7`](../config_lib/config_lib_macros/src/lib.rs#L7) |

## Crate `development_data_bootstrap`

### Модуль `development_data_bootstrap`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `DevelopmentBootstrapPlan` | `struct` | `public` | named-field data structure; 1 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`development_data_bootstrap/src/lib.rs:17`](../development_data_bootstrap/src/lib.rs#L17) |
| `DevelopmentBootstrapSummary` | `struct` | `public` | named-field data structure; 3 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`development_data_bootstrap/src/lib.rs:40`](../development_data_bootstrap/src/lib.rs#L40) |
| `DevelopmentIdentityCount` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`development_data_bootstrap/src/lib.rs:47`](../development_data_bootstrap/src/lib.rs#L47) |
| `DevelopmentIdentitySpecs` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | внутреннее значение может расти; нужен предел длины/размера | [`development_data_bootstrap/src/lib.rs:3`](../development_data_bootstrap/src/lib.rs#L3) |
| `DevelopmentIdentitySpecsError` | `struct` | `public` | unit marker type; production | **Нет** | Не требуется | marker не содержит проверяемых данных | [`development_data_bootstrap/src/lib.rs:14`](../development_data_bootstrap/src/lib.rs#L14) |

## Crate `external_service_emulators`

### Модуль `external_service_emulators`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `MockNotificationInbox` | `struct` | `public` | named-field data structure; 1 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`external_service_emulators/src/lib.rs:6`](../external_service_emulators/src/lib.rs#L6) |
| `MockNotificationProvider` | `struct` | `public` | named-field data structure; 1 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`external_service_emulators/src/lib.rs:1`](../external_service_emulators/src/lib.rs#L1) |
| `MockNotificationProviderClosed` | `struct` | `public` | unit marker type; production | **Нет** | Не требуется | marker не содержит проверяемых данных | [`external_service_emulators/src/lib.rs:21`](../external_service_emulators/src/lib.rs#L21) |
| `RemoteSyncRequestCount` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`external_service_emulators/src/lib.rs:47`](../external_service_emulators/src/lib.rs#L47) |
| `RemoteSyncSource` | `struct` | `public` | named-field data structure; 2 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`external_service_emulators/src/lib.rs:50`](../external_service_emulators/src/lib.rs#L50) |
| `TokioMockNotificationReceiver` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`external_service_emulators/src/lib.rs:16`](../external_service_emulators/src/lib.rs#L16) |
| `TokioMockNotificationSender` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`external_service_emulators/src/lib.rs:11`](../external_service_emulators/src/lib.rs#L11) |

## Crate `file_storage`

### Модуль `file_storage`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `AtomicReplaceDurability` | `enum` | `public` | closed variant set; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`file_storage/src/lib.rs:519`](../file_storage/src/lib.rs#L519) |
| `DiskCacheBudgetError` | `enum` | `public` | error enum; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`file_storage/src/lib.rs:555`](../file_storage/src/lib.rs#L555) |
| `DiskCacheEntry` | `struct` | `public` | named-field data structure; 3 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`file_storage/src/lib.rs:531`](../file_storage/src/lib.rs#L531) |
| `DiskCacheEvictionPlan` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | отдельный инвариант этого служебного типа не требуется; проверка принадлежит итоговой доменной границе | [`file_storage/src/lib.rs:552`](../file_storage/src/lib.rs#L552) |
| `FileStorageError` | `enum` | `public` | error enum; 6 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`file_storage/src/lib.rs:93`](../file_storage/src/lib.rs#L93) |
| `FileStoragePathError` | `enum` | `public` | error enum; 5 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`file_storage/src/lib.rs:79`](../file_storage/src/lib.rs#L79) |
| `FileStorageStagingArea` | `enum` | `public` | closed variant set; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`file_storage/src/lib.rs:117`](../file_storage/src/lib.rs#L117) |
| `SafeFileStorage` | `struct` | `public` | named-field data structure; 1 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`file_storage/src/lib.rs:112`](../file_storage/src/lib.rs#L112) |
| `StaleStagingCleanupCfg` | `struct` | `public` | named-field data structure; 3 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`file_storage/src/lib.rs:151`](../file_storage/src/lib.rs#L151) |
| `StaleStagingCleanupCfgError` | `struct` | `public` | unit marker type; production | **Нет** | Не требуется | marker не содержит проверяемых данных | [`file_storage/src/lib.rs:172`](../file_storage/src/lib.rs#L172) |
| `StaleStagingCleanupReport` | `struct` | `public` | named-field data structure; 2 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`file_storage/src/lib.rs:189`](../file_storage/src/lib.rs#L189) |
| `StdDiskCacheModifiedAt` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`file_storage/src/lib.rs:528`](../file_storage/src/lib.rs#L528) |
| `StdDiskCacheSize` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`file_storage/src/lib.rs:525`](../file_storage/src/lib.rs#L525) |
| `StdFileBytes` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`file_storage/src/lib.rs:66`](../file_storage/src/lib.rs#L66) |
| `StdFileStorageIoError` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`file_storage/src/lib.rs:5`](../file_storage/src/lib.rs#L5) |
| `StdFileStorageRoot` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`file_storage/src/lib.rs:13`](../file_storage/src/lib.rs#L13) |
| `StdStaleBefore` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`file_storage/src/lib.rs:148`](../file_storage/src/lib.rs#L148) |
| `StdStaleStagingEntryCount` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`file_storage/src/lib.rs:176`](../file_storage/src/lib.rs#L176) |
| `StdStaleStagingEntryLimit` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`file_storage/src/lib.rs:135`](../file_storage/src/lib.rs#L135) |
| `StdStorageOperationId` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`file_storage/src/lib.rs:49`](../file_storage/src/lib.rs#L49) |
| `StdStoragePathRef` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`file_storage/src/lib.rs:7`](../file_storage/src/lib.rs#L7) |
| `StdStorageRelativePath` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`file_storage/src/lib.rs:29`](../file_storage/src/lib.rs#L29) |
| `StorageDirectoryNameRef` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`file_storage/src/lib.rs:10`](../file_storage/src/lib.rs#L10) |

## Crate `frontend_contract`

### Модуль `frontend_contract`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `ActionContract` | `struct` | `public` | named-field data structure; 3 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`frontend_contract/src/lib.rs:680`](../frontend_contract/src/lib.rs#L680) |
| `ActionContracts` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | отдельный инвариант этого служебного типа не требуется; проверка принадлежит итоговой доменной границе | [`frontend_contract/src/lib.rs:686`](../frontend_contract/src/lib.rs#L686) |
| `AuthenticationRequirement` | `enum` | `public` | closed variant set; 3 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`frontend_contract/src/lib.rs:653`](../frontend_contract/src/lib.rs#L653) |
| `CapabilitySupport` | `enum` | `public` | closed variant set; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`frontend_contract/src/lib.rs:108`](../frontend_contract/src/lib.rs#L108) |
| `ClientError` | `enum` | `public` | error enum; 6 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`frontend_contract/src/lib.rs:913`](../frontend_contract/src/lib.rs#L913) |
| `ConfirmationRequirement` | `enum` | `public` | closed variant set; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`frontend_contract/src/lib.rs:675`](../frontend_contract/src/lib.rs#L675) |
| `ContractI64` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`frontend_contract/src/lib.rs:206`](../frontend_contract/src/lib.rs#L206) |
| `ContractStr` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`frontend_contract/src/lib.rs:63`](../frontend_contract/src/lib.rs#L63) |
| `FieldCapability` | `enum` | `public` | closed variant set; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`frontend_contract/src/lib.rs:384`](../frontend_contract/src/lib.rs#L384) |
| `FieldContract` | `struct` | `public` | named-field data structure; 13 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`frontend_contract/src/lib.rs:406`](../frontend_contract/src/lib.rs#L406) |
| `FieldContracts` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | отдельный инвариант этого служебного типа не требуется; проверка принадлежит итоговой доменной границе | [`frontend_contract/src/lib.rs:422`](../frontend_contract/src/lib.rs#L422) |
| `FieldOrder` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`frontend_contract/src/lib.rs:394`](../frontend_contract/src/lib.rs#L394) |
| `FieldPlaceholder` | `enum` | `public` | closed variant set; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`frontend_contract/src/lib.rs:401`](../frontend_contract/src/lib.rs#L401) |
| `FieldVisibility` | `enum` | `public` | closed variant set; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`frontend_contract/src/lib.rs:396`](../frontend_contract/src/lib.rs#L396) |
| `FilterContracts` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`frontend_contract/src/lib.rs:186`](../frontend_contract/src/lib.rs#L186) |
| `FilterFormValueContract` | `trait` | `public` | behavior contract; 1 associated items; production | **Нет** | Не требуется | trait не инициализируется как значение | [`frontend_contract/src/lib.rs:362`](../frontend_contract/src/lib.rs#L362) |
| `FilterOperation` | `enum` | `public` | closed variant set; 24 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`frontend_contract/src/lib.rs:113`](../frontend_contract/src/lib.rs#L113) |
| `FilterValueShape` | `enum` | `public` | closed variant set; 6 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`frontend_contract/src/lib.rs:143`](../frontend_contract/src/lib.rs#L143) |
| `FilterWireJson` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | атрибут валидатора задаёт инвариант типа | [`frontend_contract/src/lib.rs:355`](../frontend_contract/src/lib.rs#L355) |
| `FormFieldError` | `struct` | `public` | named-field data structure; 2 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`frontend_contract/src/lib.rs:365`](../frontend_contract/src/lib.rs#L365) |
| `FormFieldNameRef` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`frontend_contract/src/lib.rs:345`](../frontend_contract/src/lib.rs#L345) |
| `FormValue` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | атрибут валидатора задаёт инвариант типа | [`frontend_contract/src/lib.rs:340`](../frontend_contract/src/lib.rs#L340) |
| `FormValueContract` | `trait` | `public` | behavior contract; 2 associated items; production | **Нет** | Не требуется | trait не инициализируется как значение | [`frontend_contract/src/lib.rs:358`](../frontend_contract/src/lib.rs#L358) |
| `FormValueError` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`frontend_contract/src/lib.rs:347`](../frontend_contract/src/lib.rs#L347) |
| `FormValueRef` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`frontend_contract/src/lib.rs:343`](../frontend_contract/src/lib.rs#L343) |
| `FrontendContractBodyError` | `struct` | `public` | unit marker type; production | **Нет** | Не требуется | marker не содержит проверяемых данных | [`frontend_contract/src/lib.rs:3`](../frontend_contract/src/lib.rs#L3) |
| `HasFilterContracts` | `trait` | `public` | behavior contract; 2 associated items; production | **Нет** | Не требуется | trait не инициализируется как значение | [`frontend_contract/src/lib.rs:188`](../frontend_contract/src/lib.rs#L188) |
| `HasTypeContract` | `trait` | `public` | behavior contract; 1 associated items; production | **Нет** | Не требуется | trait не инициализируется как значение | [`frontend_contract/src/lib.rs:337`](../frontend_contract/src/lib.rs#L337) |
| `HttpMethod` | `enum` | `public` | closed variant set; 9 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`frontend_contract/src/lib.rs:547`](../frontend_contract/src/lib.rs#L547) |
| `InputKind` | `enum` | `public` | closed variant set; 7 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`frontend_contract/src/lib.rs:72`](../frontend_contract/src/lib.rs#L72) |
| `InputStep` | `enum` | `public` | closed variant set; 3 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`frontend_contract/src/lib.rs:195`](../frontend_contract/src/lib.rs#L195) |
| `MutationKind` | `enum` | `public` | closed variant set; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`frontend_contract/src/lib.rs:659`](../frontend_contract/src/lib.rs#L659) |
| `Nullability` | `enum` | `public` | closed variant set; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`frontend_contract/src/lib.rs:103`](../frontend_contract/src/lib.rs#L103) |
| `NumericBound` | `enum` | `public` | closed variant set; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`frontend_contract/src/lib.rs:201`](../frontend_contract/src/lib.rs#L201) |
| `OperationKind` | `enum` | `public` | closed variant set; 8 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`frontend_contract/src/lib.rs:664`](../frontend_contract/src/lib.rs#L664) |
| `PageContract` | `struct` | `public` | named-field data structure; 5 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`frontend_contract/src/lib.rs:763`](../frontend_contract/src/lib.rs#L763) |
| `PrimaryKeyKind` | `enum` | `public` | closed variant set; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`frontend_contract/src/lib.rs:389`](../frontend_contract/src/lib.rs#L389) |
| `RouteContract` | `struct` | `public` | named-field data structure; 5 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`frontend_contract/src/lib.rs:715`](../frontend_contract/src/lib.rs#L715) |
| `RouteContracts` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | отдельный инвариант этого служебного типа не требуется; проверка принадлежит итоговой доменной границе | [`frontend_contract/src/lib.rs:723`](../frontend_contract/src/lib.rs#L723) |
| `RouteErrorStatus` | `enum` | `public` | closed variant set; 6 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`frontend_contract/src/lib.rs:565`](../frontend_contract/src/lib.rs#L565) |
| `SuccessStatus` | `enum` | `public` | closed variant set; 3 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`frontend_contract/src/lib.rs:559`](../frontend_contract/src/lib.rs#L559) |
| `Transport` | `trait` | `public` | behavior contract; 1 associated items; production | **Нет** | Не требуется | trait не инициализируется как значение | [`frontend_contract/src/lib.rs:907`](../frontend_contract/src/lib.rs#L907) |
| `TransportBody` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`frontend_contract/src/lib.rs:771`](../frontend_contract/src/lib.rs#L771) |
| `TransportError` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`frontend_contract/src/lib.rs:899`](../frontend_contract/src/lib.rs#L899) |
| `TransportIdempotencyKey` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | атрибут валидатора задаёт инвариант типа | [`frontend_contract/src/lib.rs:833`](../frontend_contract/src/lib.rs#L833) |
| `TransportIfMatch` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | атрибут валидатора задаёт инвариант типа | [`frontend_contract/src/lib.rs:836`](../frontend_contract/src/lib.rs#L836) |
| `TransportPath` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | атрибут валидатора задаёт инвариант типа | [`frontend_contract/src/lib.rs:839`](../frontend_contract/src/lib.rs#L839) |
| `TransportRequest` | `struct` | `public` | named-field data structure; 5 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`frontend_contract/src/lib.rs:783`](../frontend_contract/src/lib.rs#L783) |
| `TransportResponse` | `struct` | `public` | named-field data structure; 3 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`frontend_contract/src/lib.rs:849`](../frontend_contract/src/lib.rs#L849) |
| `TransportRetryAfter` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | атрибут валидатора задаёт инвариант типа | [`frontend_contract/src/lib.rs:846`](../frontend_contract/src/lib.rs#L846) |
| `TransportStatus` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | сырой `u16` допускает невалидный HTTP status; нужен проверяемый conversion | [`frontend_contract/src/lib.rs:842`](../frontend_contract/src/lib.rs#L842) |
| `TypeContract` | `struct` | `public` | named-field data structure; 7 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`frontend_contract/src/lib.rs:246`](../frontend_contract/src/lib.rs#L246) |
| `ValueExample` | `enum` | `public` | closed variant set; 9 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`frontend_contract/src/lib.rs:234`](../frontend_contract/src/lib.rs#L234) |
| `ValueFormat` | `enum` | `public` | closed variant set; 18 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`frontend_contract/src/lib.rs:82`](../frontend_contract/src/lib.rs#L82) |

### Модуль `frontend_contract::auth_session_keep_alive`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `AuthSessionKeepAlive` | `struct` | `public` | named-field data structure; 3 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`frontend_contract/src/auth_session_keep_alive.rs:50`](../frontend_contract/src/auth_session_keep_alive.rs#L50) |
| `AuthSessionKeepAliveDecision` | `enum` | `public` | closed variant set; 4 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`frontend_contract/src/auth_session_keep_alive.rs:30`](../frontend_contract/src/auth_session_keep_alive.rs#L30) |
| `AuthSessionKeepAliveError` | `enum` | `public` | error enum; 1 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`frontend_contract/src/auth_session_keep_alive.rs:38`](../frontend_contract/src/auth_session_keep_alive.rs#L38) |
| `AuthSessionPresence` | `enum` | `public` | closed variant set; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`frontend_contract/src/auth_session_keep_alive.rs:17`](../frontend_contract/src/auth_session_keep_alive.rs#L17) |
| `AuthSessionRefreshOutcome` | `enum` | `public` | closed variant set; 3 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`frontend_contract/src/auth_session_keep_alive.rs:23`](../frontend_contract/src/auth_session_keep_alive.rs#L23) |
| `AuthSessionRefreshState` | `enum` | `private` | closed variant set; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`frontend_contract/src/auth_session_keep_alive.rs:44`](../frontend_contract/src/auth_session_keep_alive.rs#L44) |
| `StdAuthSessionInstant` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`frontend_contract/src/auth_session_keep_alive.rs:1`](../frontend_contract/src/auth_session_keep_alive.rs#L1) |
| `StdAuthSessionRefreshInterval` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`frontend_contract/src/auth_session_keep_alive.rs:4`](../frontend_contract/src/auth_session_keep_alive.rs#L4) |

### Модуль `frontend_contract::json_snapshot`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `JsonContractSnapshot` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | внутреннее значение может расти; нужен предел длины/размера | [`frontend_contract/src/json_snapshot.rs:3`](../frontend_contract/src/json_snapshot.rs#L3) |
| `JsonContractSnapshotError` | `enum` | `public` | error enum; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`frontend_contract/src/json_snapshot.rs:14`](../frontend_contract/src/json_snapshot.rs#L14) |
| `JsonSnapshotDynamicFieldRef` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`frontend_contract/src/json_snapshot.rs:11`](../frontend_contract/src/json_snapshot.rs#L11) |

### Модуль `frontend_contract::openapi_validation`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `OpenApiContractText` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | атрибут валидатора задаёт инвариант типа | [`frontend_contract/src/openapi_validation.rs:3`](../frontend_contract/src/openapi_validation.rs#L3) |
| `OpenApiContractTextError` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`frontend_contract/src/openapi_validation.rs:7`](../frontend_contract/src/openapi_validation.rs#L7) |
| `OpenApiOperationExpectation` | `struct` | `public` | named-field data structure; 4 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`frontend_contract/src/openapi_validation.rs:44`](../frontend_contract/src/openapi_validation.rs#L44) |
| `OpenApiOperationValidationError` | `enum` | `public` | error enum; 6 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`frontend_contract/src/openapi_validation.rs:68`](../frontend_contract/src/openapi_validation.rs#L68) |
| `OpenApiPayloadValidationError` | `enum` | `public` | error enum; 4 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`frontend_contract/src/openapi_validation.rs:90`](../frontend_contract/src/openapi_validation.rs#L90) |
| `OpenApiResponseStatus` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | сырой `u16` допускает невалидный HTTP status; нужен проверяемый conversion | [`frontend_contract/src/openapi_validation.rs:35`](../frontend_contract/src/openapi_validation.rs#L35) |
| `OpenApiSchemaMismatch` | `enum` | `public` | closed variant set; 8 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`frontend_contract/src/openapi_validation.rs:78`](../frontend_contract/src/openapi_validation.rs#L78) |
| `OpenApiSecurityExpectation` | `enum` | `public` | closed variant set; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`frontend_contract/src/openapi_validation.rs:38`](../frontend_contract/src/openapi_validation.rs#L38) |
| `OpenApiValidationError` | `enum` | `public` | error enum; 10 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`frontend_contract/src/openapi_validation.rs:16`](../frontend_contract/src/openapi_validation.rs#L16) |
| `RuntimeRoutesRef` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`frontend_contract/src/openapi_validation.rs:13`](../frontend_contract/src/openapi_validation.rs#L13) |
| `SerdeJsonOpenApiSerializationError` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`frontend_contract/src/openapi_validation.rs:10`](../frontend_contract/src/openapi_validation.rs#L10) |

### Модуль `frontend_contract::problem`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `ApiProblem` | `struct` | `public` | named-field data structure; 5 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`frontend_contract/src/problem.rs:105`](../frontend_contract/src/problem.rs#L105) |
| `ApiProblemDetail` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | атрибут валидатора задаёт инвариант типа | [`frontend_contract/src/problem.rs:35`](../frontend_contract/src/problem.rs#L35) |
| `ApiProblemField` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | атрибут валидатора задаёт инвариант типа | [`frontend_contract/src/problem.rs:64`](../frontend_contract/src/problem.rs#L64) |
| `ApiProblemKind` | `enum` | `public` | closed variant set; 14 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`frontend_contract/src/problem.rs:1`](../frontend_contract/src/problem.rs#L1) |
| `ApiProblemRequestId` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | атрибут валидатора задаёт инвариант типа | [`frontend_contract/src/problem.rs:50`](../frontend_contract/src/problem.rs#L50) |
| `ApiProblemStatus` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | сырой `u16` допускает невалидный HTTP status; нужен проверяемый conversion | [`frontend_contract/src/problem.rs:21`](../frontend_contract/src/problem.rs#L21) |
| `ApiProblemViolation` | `struct` | `public` | named-field data structure; 2 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`frontend_contract/src/problem.rs:78`](../frontend_contract/src/problem.rs#L78) |
| `ApiProblemViolations` | `struct` | `crate` | single-field tuple wrapper; production | **Да** | Выполнено | внутреннее значение может расти; нужен предел длины/размера | [`frontend_contract/src/problem.rs:83`](../frontend_contract/src/problem.rs#L83) |
| `ApiProblemViolationsError` | `struct` | `crate` | unit marker type; production | **Нет** | Не требуется | marker не содержит проверяемых данных | [`frontend_contract/src/problem.rs:103`](../frontend_contract/src/problem.rs#L103) |

### Модуль `frontend_contract::route`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `AuthenticatedTransport` | `struct` | `public` | unit marker type; production | **Нет** | Не требуется | marker не содержит проверяемых данных | [`frontend_contract/src/route.rs:4`](../frontend_contract/src/route.rs#L4) |
| `CoveredRoute` | `trait` | `public` | behavior contract; 1 associated items; production | **Нет** | Не требуется | trait не инициализируется как значение | [`frontend_contract/src/route.rs:228`](../frontend_contract/src/route.rs#L228) |
| `OpenApiSecuritySchemeRef` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`frontend_contract/src/route.rs:226`](../frontend_contract/src/route.rs#L226) |
| `ParameterizedRoute` | `trait` | `public` | behavior contract; 2 associated items; production | **Нет** | Не требуется | trait не инициализируется как значение | [`frontend_contract/src/route.rs:231`](../frontend_contract/src/route.rs#L231) |
| `ParameterizedRoutePath` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`frontend_contract/src/route.rs:212`](../frontend_contract/src/route.rs#L212) |
| `ParameterizedRoutePathTryFromStringError` | `struct` | `public` | unit marker type; production | **Нет** | Не требуется | marker не содержит проверяемых данных | [`frontend_contract/src/route.rs:214`](../frontend_contract/src/route.rs#L214) |
| `PublicTransport` | `struct` | `public` | unit marker type; production | **Нет** | Не требуется | marker не содержит проверяемых данных | [`frontend_contract/src/route.rs:2`](../frontend_contract/src/route.rs#L2) |
| `RouteBodyLimit` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`frontend_contract/src/route.rs:235`](../frontend_contract/src/route.rs#L235) |
| `RouteCoverageDescriptors` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | отдельный инвариант этого служебного типа не требуется; проверка принадлежит итоговой доменной границе | [`frontend_contract/src/route.rs:237`](../frontend_contract/src/route.rs#L237) |
| `RouteFamily` | `trait` | `public` | behavior contract; 4 associated items; production | **Нет** | Не требуется | trait не инициализируется как значение | [`frontend_contract/src/route.rs:269`](../frontend_contract/src/route.rs#L269) |
| `RouteMetadata` | `struct` | `public` | named-field data structure; 7 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`frontend_contract/src/route.rs:36`](../frontend_contract/src/route.rs#L36) |
| `RouteMetadataList` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | отдельный инвариант этого служебного типа не требуется; проверка принадлежит итоговой доменной границе | [`frontend_contract/src/route.rs:252`](../frontend_contract/src/route.rs#L252) |
| `RouteMethod` | `enum` | `public` | closed variant set; 9 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`frontend_contract/src/route.rs:8`](../frontend_contract/src/route.rs#L8) |
| `RouteRequest` | `struct` | `public` | named-field data structure; 1 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`frontend_contract/src/route.rs:287`](../frontend_contract/src/route.rs#L287) |
| `RouteResponse` | `struct` | `public` | named-field data structure; 1 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`frontend_contract/src/route.rs:307`](../frontend_contract/src/route.rs#L307) |
| `RouteSchemaContract` | `struct` | `public` | named-field data structure; 3 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`frontend_contract/src/route.rs:165`](../frontend_contract/src/route.rs#L165) |
| `RouteSchemaContracts` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | отдельный инвариант этого служебного типа не требуется; проверка принадлежит итоговой доменной границе | [`frontend_contract/src/route.rs:248`](../frontend_contract/src/route.rs#L248) |
| `RouteTransport` | `trait` | `public` | behavior contract; 0 associated items; production | **Нет** | Не требуется | trait не инициализируется как значение | [`frontend_contract/src/route.rs:1`](../frontend_contract/src/route.rs#L1) |
| `TypedRoute` | `trait` | `public` | behavior contract; 7 associated items; production | **Нет** | Не требуется | trait не инициализируется как значение | [`frontend_contract/src/route.rs:147`](../frontend_contract/src/route.rs#L147) |
| `UtoipaOpenApiPathParameter` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`frontend_contract/src/route.rs:204`](../frontend_contract/src/route.rs#L204) |
| `UtoipaOpenApiRouteSchema` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`frontend_contract/src/route.rs:196`](../frontend_contract/src/route.rs#L196) |

### Модуль `frontend_contract::route::tests`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `Request` | `struct` | `private` | single-field tuple wrapper; test-only | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`frontend_contract/src/route.rs:469`](../frontend_contract/src/route.rs#L469) |
| `Response` | `struct` | `private` | single-field tuple wrapper; test-only | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`frontend_contract/src/route.rs:474`](../frontend_contract/src/route.rs#L474) |
| `Route` | `struct` | `private` | unit marker type; test-only | **Нет** | Не требуется | marker не содержит проверяемых данных | [`frontend_contract/src/route.rs:479`](../frontend_contract/src/route.rs#L479) |

### Модуль `frontend_contract::route_contract_validation`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `HttpContractBody` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`frontend_contract/src/route_contract_validation.rs:23`](../frontend_contract/src/route_contract_validation.rs#L23) |
| `HttpContractBodyKind` | `enum` | `public` | closed variant set; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`frontend_contract/src/route_contract_validation.rs:36`](../frontend_contract/src/route_contract_validation.rs#L36) |
| `HttpContractExpectation` | `struct` | `public` | named-field data structure; 3 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`frontend_contract/src/route_contract_validation.rs:63`](../frontend_contract/src/route_contract_validation.rs#L63) |
| `HttpContractMismatch` | `enum` | `public` | closed variant set; 4 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`frontend_contract/src/route_contract_validation.rs:84`](../frontend_contract/src/route_contract_validation.rs#L84) |
| `HttpContractObservation` | `struct` | `public` | named-field data structure; 3 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`frontend_contract/src/route_contract_validation.rs:42`](../frontend_contract/src/route_contract_validation.rs#L42) |
| `HttpContractStatus` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | сырой `u16` допускает невалидный HTTP status; нужен проверяемый conversion | [`frontend_contract/src/route_contract_validation.rs:20`](../frontend_contract/src/route_contract_validation.rs#L20) |
| `RouteContractMismatch` | `enum` | `public` | closed variant set; 3 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`frontend_contract/src/route_contract_validation.rs:1`](../frontend_contract/src/route_contract_validation.rs#L1) |
| `RouteContractMismatches` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | отдельный инвариант этого служебного типа не требуется; проверка принадлежит итоговой доменной границе | [`frontend_contract/src/route_contract_validation.rs:17`](../frontend_contract/src/route_contract_validation.rs#L17) |

### Модуль `frontend_contract::route_contract_validation::tests`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `ReadRoute` | `struct` | `private` | unit marker type; test-only | **Нет** | Не требуется | marker не содержит проверяемых данных | [`frontend_contract/src/route_contract_validation.rs:166`](../frontend_contract/src/route_contract_validation.rs#L166) |

### Модуль `frontend_contract::route_coverage`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `RouteAccess` | `enum` | `public` | closed variant set; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`frontend_contract/src/route_coverage.rs:1`](../frontend_contract/src/route_coverage.rs#L1) |
| `RouteCoverageDescriptor` | `struct` | `public` | named-field data structure; 4 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`frontend_contract/src/route_coverage.rs:115`](../frontend_contract/src/route_coverage.rs#L115) |
| `RouteCoverageError` | `enum` | `public` | error enum; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`frontend_contract/src/route_coverage.rs:177`](../frontend_contract/src/route_coverage.rs#L177) |
| `RouteCoverageEvidence` | `struct` | `public` | named-field data structure; 1 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`frontend_contract/src/route_coverage.rs:103`](../frontend_contract/src/route_coverage.rs#L103) |
| `RouteCoverageObligation` | `enum` | `public` | closed variant set; 5 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`frontend_contract/src/route_coverage.rs:144`](../frontend_contract/src/route_coverage.rs#L144) |
| `RouteDatabaseUsage` | `enum` | `public` | closed variant set; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`frontend_contract/src/route_coverage.rs:13`](../frontend_contract/src/route_coverage.rs#L13) |
| `RouteJsonBodyUsage` | `enum` | `public` | closed variant set; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`frontend_contract/src/route_coverage.rs:19`](../frontend_contract/src/route_coverage.rs#L19) |
| `RouteMutation` | `enum` | `public` | closed variant set; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`frontend_contract/src/route_coverage.rs:7`](../frontend_contract/src/route_coverage.rs#L7) |
| `RouteResponseKind` | `enum` | `public` | closed variant set; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`frontend_contract/src/route_coverage.rs:25`](../frontend_contract/src/route_coverage.rs#L25) |
| `RouteTestCapabilities` | `struct` | `public` | named-field data structure; 3 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`frontend_contract/src/route_coverage.rs:31`](../frontend_contract/src/route_coverage.rs#L31) |
| `RouteTestCategories` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | отдельный инвариант этого служебного типа не требуется; проверка принадлежит итоговой доменной границе | [`frontend_contract/src/route_coverage.rs:61`](../frontend_contract/src/route_coverage.rs#L61) |
| `RouteTestCategory` | `enum` | `public` | closed variant set; 5 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`frontend_contract/src/route_coverage.rs:53`](../frontend_contract/src/route_coverage.rs#L53) |

### Модуль `frontend_contract::tests::typed_route::tests`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `TestCatalog` | `enum` | `private` | closed variant set; 2 variants; test-only | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`frontend_contract/tests/typed_route.rs:34`](../frontend_contract/tests/typed_route.rs#L34) |
| `TestRequest` | `struct` | `private` | unit marker type; test-only | **Нет** | Не требуется | marker не содержит проверяемых данных | [`frontend_contract/tests/typed_route.rs:5`](../frontend_contract/tests/typed_route.rs#L5) |
| `TestResponse` | `struct` | `private` | unit marker type; test-only | **Нет** | Не требуется | marker не содержит проверяемых данных | [`frontend_contract/tests/typed_route.rs:7`](../frontend_contract/tests/typed_route.rs#L7) |
| `TestRoute` | `struct` | `private` | unit marker type; test-only | **Нет** | Не требуется | marker не содержит проверяемых данных | [`frontend_contract/tests/typed_route.rs:10`](../frontend_contract/tests/typed_route.rs#L10) |
| `TestRouteFamily` | `struct` | `private` | unit marker type; test-only | **Нет** | Не требуется | marker не содержит проверяемых данных | [`frontend_contract/tests/typed_route.rs:30`](../frontend_contract/tests/typed_route.rs#L30) |

### Модуль `frontend_contract::url_builder`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `ApiUrl` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | атрибут валидатора задаёт инвариант типа | [`frontend_contract/src/url_builder.rs:29`](../frontend_contract/src/url_builder.rs#L29) |
| `ApiUrlBuildError` | `enum` | `public` | error enum; 1 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`frontend_contract/src/url_builder.rs:23`](../frontend_contract/src/url_builder.rs#L23) |
| `ApiUrlPathSegmentRef` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`frontend_contract/src/url_builder.rs:3`](../frontend_contract/src/url_builder.rs#L3) |
| `ApiUrlQueryComponentRef` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`frontend_contract/src/url_builder.rs:20`](../frontend_contract/src/url_builder.rs#L20) |

## Crate `frontend_contract_macros`

### Модуль `frontend_contract_macros`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `PageCatalogArgs` | `struct` | `private` | named-field data structure; 3 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`frontend_contract_macros/src/lib.rs:27`](../frontend_contract_macros/src/lib.rs#L27) |
| `PageCatalogPageArgs` | `struct` | `private` | named-field data structure; 4 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`frontend_contract_macros/src/lib.rs:32`](../frontend_contract_macros/src/lib.rs#L32) |
| `RouteCatalogArgs` | `struct` | `private` | named-field data structure; 2 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`frontend_contract_macros/src/lib.rs:17`](../frontend_contract_macros/src/lib.rs#L17) |
| `RouteCatalogRouteArgs` | `struct` | `private` | named-field data structure; 4 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`frontend_contract_macros/src/lib.rs:21`](../frontend_contract_macros/src/lib.rs#L21) |
| `RouteRegistryArgs` | `struct` | `private` | named-field data structure; 4 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`frontend_contract_macros/src/lib.rs:229`](../frontend_contract_macros/src/lib.rs#L229) |
| `RouteRegistryBinding` | `struct` | `private` | named-field data structure; 2 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`frontend_contract_macros/src/lib.rs:203`](../frontend_contract_macros/src/lib.rs#L203) |
| `StdBool` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`frontend_contract_macros/src/lib.rs:14`](../frontend_contract_macros/src/lib.rs#L14) |
| `SynExpr` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`frontend_contract_macros/src/lib.rs:5`](../frontend_contract_macros/src/lib.rs#L5) |
| `SynIdent` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`frontend_contract_macros/src/lib.rs:11`](../frontend_contract_macros/src/lib.rs#L11) |
| `SynRouteRegistryBindings` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`frontend_contract_macros/src/lib.rs:213`](../frontend_contract_macros/src/lib.rs#L213) |
| `SynRouteRegistryHandler` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`frontend_contract_macros/src/lib.rs:207`](../frontend_contract_macros/src/lib.rs#L207) |
| `SynRouteRegistryRoute` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`frontend_contract_macros/src/lib.rs:210`](../frontend_contract_macros/src/lib.rs#L210) |
| `SynRouteRegistryState` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`frontend_contract_macros/src/lib.rs:216`](../frontend_contract_macros/src/lib.rs#L216) |
| `SynType` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`frontend_contract_macros/src/lib.rs:8`](../frontend_contract_macros/src/lib.rs#L8) |
| `TypedRouteArgs` | `struct` | `private` | named-field data structure; 12 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`frontend_contract_macros/src/lib.rs:188`](../frontend_contract_macros/src/lib.rs#L188) |

## Crate `generate_derive_token_stream_builder`

### Модуль `generate_derive_token_stream_builder`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `SnakeCaseString` | `struct` | `private` | single-field tuple wrapper; production | **Да** | Выполнено | атрибут валидатора задаёт инвариант типа | [`macros_helpers/generate_derive_token_stream_builder/src/lib.rs:4`](../macros_helpers/generate_derive_token_stream_builder/src/lib.rs#L4) |
| `ToSnakeCaseInput` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`macros_helpers/generate_derive_token_stream_builder/src/lib.rs:2`](../macros_helpers/generate_derive_token_stream_builder/src/lib.rs#L2) |

## Crate `generate_pg_table_src`

### Модуль `generate_pg_table_src::model`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `GeneratePgTableFieldCount` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`pg_crud/pg_table/generate_pg_table_src/src/model.rs:2`](../pg_crud/pg_table/generate_pg_table_src/src/model.rs#L2) |
| `GeneratePgTableModel` | `struct` | `public` | named-field data structure; 2 fields; production | **Да** | Выполнено | fallible-конструктор показывает наличие инварианта | [`pg_crud/pg_table/generate_pg_table_src/src/model.rs:5`](../pg_crud/pg_table/generate_pg_table_src/src/model.rs#L5) |
| `OperationDsc` | `struct` | `restricted` | named-field data structure; 7 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`pg_crud/pg_table/generate_pg_table_src/src/model.rs:46`](../pg_crud/pg_table/generate_pg_table_src/src/model.rs#L46) |
| `SynGeneratePgTableModelError` | `struct` | `restricted` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`pg_crud/pg_table/generate_pg_table_src/src/model.rs:12`](../pg_crud/pg_table/generate_pg_table_src/src/model.rs#L12) |
| `SynGeneratePgTableModelInput` | `struct` | `restricted` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`pg_crud/pg_table/generate_pg_table_src/src/model.rs:10`](../pg_crud/pg_table/generate_pg_table_src/src/model.rs#L10) |

### Модуль `generate_pg_table_src::pipeline`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `GeneratePgTablePipelineError` | `enum` | `public` | error enum; 3 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`pg_crud/pg_table/generate_pg_table_src/src/pipeline.rs:26`](../pg_crud/pg_table/generate_pg_table_src/src/pipeline.rs#L26) |
| `SynBuiltGeneratePgTableInput` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`pg_crud/pg_table/generate_pg_table_src/src/pipeline.rs:4`](../pg_crud/pg_table/generate_pg_table_src/src/pipeline.rs#L4) |
| `SynGeneratePgTablePipelineError` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`pg_crud/pg_table/generate_pg_table_src/src/pipeline.rs:23`](../pg_crud/pg_table/generate_pg_table_src/src/pipeline.rs#L23) |
| `SynParsedGeneratePgTableInput` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`pg_crud/pg_table/generate_pg_table_src/src/pipeline.rs:1`](../pg_crud/pg_table/generate_pg_table_src/src/pipeline.rs#L1) |
| `SynValidatedGeneratePgTableInput` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`pg_crud/pg_table/generate_pg_table_src/src/pipeline.rs:14`](../pg_crud/pg_table/generate_pg_table_src/src/pipeline.rs#L14) |

### Модуль `generate_pg_table_src::source`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `CompileErrorMessage` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`pg_crud/pg_table/generate_pg_table_src/src/source.rs:1`](../pg_crud/pg_table/generate_pg_table_src/src/source.rs#L1) |
| `TableTestNames` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | отдельный инвариант этого служебного типа не требуется; проверка принадлежит итоговой доменной границе | [`pg_crud/pg_table/generate_pg_table_src/src/source.rs:9`](../pg_crud/pg_table/generate_pg_table_src/src/source.rs#L9) |

## Crate `generate_pg_table_test`

### Модуль `generate_pg_table_test::tests`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `JsonContractValue` | `struct` | `private` | named-field data structure; 1 fields; test-only | **Нет** | Не требуется | test fixture; отдельный production-инвариант не нужен | [`pg_crud/pg_table/generate_pg_table_test/src/lib.rs:3`](../pg_crud/pg_table/generate_pg_table_test/src/lib.rs#L3) |

## Crate `generate_pg_types_src`

### Модуль `generate_pg_types_src::model`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `PgTypeSpec` | `struct` | `restricted` | named-field data structure; 5 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`pg_crud/pg_types/generate_pg_types_src/src/model.rs:2`](../pg_crud/pg_types/generate_pg_types_src/src/model.rs#L2) |

### Модуль `generate_pg_types_src::source`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `BuiltGeneratePgTypesModel` | `struct` | `public` | named-field data structure; 2 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`pg_crud/pg_types/generate_pg_types_src/src/source.rs:745`](../pg_crud/pg_types/generate_pg_types_src/src/source.rs#L745) |
| `CanBeNullable` | `enum` | `private` | closed variant set; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`pg_crud/pg_types/generate_pg_types_src/src/source.rs:423`](../pg_crud/pg_types/generate_pg_types_src/src/source.rs#L423) |
| `CanBePrimaryKey` | `enum` | `private` | closed variant set; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`pg_crud/pg_types/generate_pg_types_src/src/source.rs:207`](../pg_crud/pg_types/generate_pg_types_src/src/source.rs#L207) |
| `FilterKind` | `enum` | `private` | closed variant set; 13 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`pg_crud/pg_types/generate_pg_types_src/src/source.rs:191`](../pg_crud/pg_types/generate_pg_types_src/src/source.rs#L191) |
| `GeneratePgTypeRecords` | `struct` | `private` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`pg_crud/pg_types/generate_pg_types_src/src/source.rs:569`](../pg_crud/pg_types/generate_pg_types_src/src/source.rs#L569) |
| `GeneratePgTypes` | `struct` | `private` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`pg_crud/pg_types/generate_pg_types_src/src/source.rs:572`](../pg_crud/pg_types/generate_pg_types_src/src/source.rs#L572) |
| `GeneratePgTypesConfig` | `struct` | `private` | named-field data structure; 4 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`pg_crud/pg_types/generate_pg_types_src/src/source.rs:610`](../pg_crud/pg_types/generate_pg_types_src/src/source.rs#L610) |
| `GeneratePgTypesConfigVariant` | `enum` | `private` | closed variant set; 3 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`pg_crud/pg_types/generate_pg_types_src/src/source.rs:599`](../pg_crud/pg_types/generate_pg_types_src/src/source.rs#L599) |
| `GeneratePgTypesLengthError` | `struct` | `private` | unit marker type; production | **Нет** | Не требуется | marker не содержит проверяемых данных | [`pg_crud/pg_types/generate_pg_types_src/src/source.rs:575`](../pg_crud/pg_types/generate_pg_types_src/src/source.rs#L575) |
| `GeneratePgTypesPipelineError` | `enum` | `public` | error enum; 1 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`pg_crud/pg_types/generate_pg_types_src/src/source.rs:766`](../pg_crud/pg_types/generate_pg_types_src/src/source.rs#L766) |
| `GenerateSecretText` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`pg_crud/pg_types/generate_pg_types_src/src/source.rs:605`](../pg_crud/pg_types/generate_pg_types_src/src/source.rs#L605) |
| `ParsedGeneratePgTypesConfig` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`pg_crud/pg_types/generate_pg_types_src/src/source.rs:742`](../pg_crud/pg_types/generate_pg_types_src/src/source.rs#L742) |
| `PgSqlName` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`pg_crud/pg_types/generate_pg_types_src/src/source.rs:212`](../pg_crud/pg_types/generate_pg_types_src/src/source.rs#L212) |
| `PgType` | `enum` | `private` | closed variant set; 27 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`pg_crud/pg_types/generate_pg_types_src/src/source.rs:122`](../pg_crud/pg_types/generate_pg_types_src/src/source.rs#L122) |
| `PgTypeDeserialize` | `enum` | `private` | closed variant set; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`pg_crud/pg_types/generate_pg_types_src/src/source.rs:704`](../pg_crud/pg_types/generate_pg_types_src/src/source.rs#L704) |
| `PgTypeImplNewForDeserializeOrTryNewForDe` | `enum` | `private` | closed variant set; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`pg_crud/pg_types/generate_pg_types_src/src/source.rs:699`](../pg_crud/pg_types/generate_pg_types_src/src/source.rs#L699) |
| `PgTypeImplTryNewForDe` | `enum` | `private` | closed variant set; 8 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`pg_crud/pg_types/generate_pg_types_src/src/source.rs:687`](../pg_crud/pg_types/generate_pg_types_src/src/source.rs#L687) |
| `PgTypeInitializationTryNew` | `enum` | `private` | closed variant set; 12 variants; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`pg_crud/pg_types/generate_pg_types_src/src/source.rs:619`](../pg_crud/pg_types/generate_pg_types_src/src/source.rs#L619) |
| `PgTypeName` | `enum` | `private` | closed variant set; 26 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`pg_crud/pg_types/generate_pg_types_src/src/source.rs:60`](../pg_crud/pg_types/generate_pg_types_src/src/source.rs#L60) |
| `PgTypePattern` | `enum` | `private` | closed variant set; 1 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`pg_crud/pg_types/generate_pg_types_src/src/source.rs:504`](../pg_crud/pg_types/generate_pg_types_src/src/source.rs#L504) |
| `PgTypeRecord` | `struct` | `private` | named-field data structure; 3 fields; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`pg_crud/pg_types/generate_pg_types_src/src/source.rs:521`](../pg_crud/pg_types/generate_pg_types_src/src/source.rs#L521) |
| `PgTypeRecordRaw` | `struct` | `private` | named-field data structure; 3 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`pg_crud/pg_types/generate_pg_types_src/src/source.rs:539`](../pg_crud/pg_types/generate_pg_types_src/src/source.rs#L539) |
| `PgTypesModelEntryCount` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`pg_crud/pg_types/generate_pg_types_src/src/source.rs:755`](../pg_crud/pg_types/generate_pg_types_src/src/source.rs#L755) |
| `Range` | `enum` | `private` | closed variant set; 5 variants; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`pg_crud/pg_types/generate_pg_types_src/src/source.rs:448`](../pg_crud/pg_types/generate_pg_types_src/src/source.rs#L448) |
| `RustTypeName` | `enum` | `private` | closed variant set; 23 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`pg_crud/pg_types/generate_pg_types_src/src/source.rs:4`](../pg_crud/pg_types/generate_pg_types_src/src/source.rs#L4) |
| `SerdeJsonGeneratePgTypesError` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`pg_crud/pg_types/generate_pg_types_src/src/source.rs:763`](../pg_crud/pg_types/generate_pg_types_src/src/source.rs#L763) |
| `ValidatedGeneratePgTypesConfig` | `struct` | `public` | named-field data structure; 2 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`pg_crud/pg_types/generate_pg_types_src/src/source.rs:750`](../pg_crud/pg_types/generate_pg_types_src/src/source.rs#L750) |
| `WireKind` | `enum` | `private` | closed variant set; 22 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`pg_crud/pg_types/generate_pg_types_src/src/source.rs:166`](../pg_crud/pg_types/generate_pg_types_src/src/source.rs#L166) |

## Crate `generate_pg_types_test`

### Модуль `generate_pg_types_test::tests`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `JsonContractValue` | `struct` | `private` | named-field data structure; 1 fields; test-only | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`pg_crud/pg_types/generate_pg_types_test/src/lib.rs:4`](../pg_crud/pg_types/generate_pg_types_test/src/lib.rs#L4) |

## Crate `generate_quotes`

### Модуль `generate_quotes`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `ProcMacro2QuotedLiteralTokenStream` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | отдельный инвариант этого служебного типа не требуется; проверка принадлежит итоговой доменной границе | [`generate_quotes/src/lib.rs:13`](../generate_quotes/src/lib.rs#L13) |
| `QuoteChar` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`generate_quotes/src/lib.rs:4`](../generate_quotes/src/lib.rs#L4) |
| `QuotePanicId` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`generate_quotes/src/lib.rs:6`](../generate_quotes/src/lib.rs#L6) |
| `QuotePrefix` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`generate_quotes/src/lib.rs:2`](../generate_quotes/src/lib.rs#L2) |
| `QuotedLiteral` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | атрибут валидатора задаёт инвариант типа | [`generate_quotes/src/lib.rs:8`](../generate_quotes/src/lib.rs#L8) |

## Crate `generate_where_filters_src`

### Модуль `generate_where_filters_src::bind`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `FilterPlaceholderCount` | `struct` | `restricted` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`pg_crud/where_filters/generate_where_filters_src/src/bind.rs:5`](../pg_crud/where_filters/generate_where_filters_src/src/bind.rs#L5) |

### Модуль `generate_where_filters_src::model`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `BindCount` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`pg_crud/where_filters/generate_where_filters_src/src/model.rs:1`](../pg_crud/where_filters/generate_where_filters_src/src/model.rs#L1) |
| `FilterSpec` | `struct` | `restricted` | named-field data structure; 4 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`pg_crud/where_filters/generate_where_filters_src/src/model.rs:21`](../pg_crud/where_filters/generate_where_filters_src/src/model.rs#L21) |
| `FilterSpecValid` | `struct` | `restricted` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`pg_crud/where_filters/generate_where_filters_src/src/model.rs:9`](../pg_crud/where_filters/generate_where_filters_src/src/model.rs#L9) |
| `FilterSqlOperator` | `struct` | `restricted` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`pg_crud/where_filters/generate_where_filters_src/src/model.rs:3`](../pg_crud/where_filters/generate_where_filters_src/src/model.rs#L3) |
| `FilterSqlSuffix` | `struct` | `restricted` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`pg_crud/where_filters/generate_where_filters_src/src/model.rs:7`](../pg_crud/where_filters/generate_where_filters_src/src/model.rs#L7) |
| `FilterValueShape` | `enum` | `private` | closed variant set; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`pg_crud/where_filters/generate_where_filters_src/src/model.rs:16`](../pg_crud/where_filters/generate_where_filters_src/src/model.rs#L16) |

### Модуль `generate_where_filters_src::source`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `BuiltGenerateWhereFiltersModel` | `struct` | `public` | named-field data structure; 2 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`pg_crud/where_filters/generate_where_filters_src/src/source.rs:12`](../pg_crud/where_filters/generate_where_filters_src/src/source.rs#L12) |
| `GenerateWhereFiltersPipelineError` | `enum` | `public` | error enum; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`pg_crud/where_filters/generate_where_filters_src/src/source.rs:21`](../pg_crud/where_filters/generate_where_filters_src/src/source.rs#L21) |
| `ParsedGenerateWhereFiltersConfig` | `struct` | `public` | named-field data structure; 2 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`pg_crud/where_filters/generate_where_filters_src/src/source.rs:7`](../pg_crud/where_filters/generate_where_filters_src/src/source.rs#L7) |
| `ProcMacro2GenerateWhereFiltersInput` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`pg_crud/where_filters/generate_where_filters_src/src/source.rs:1`](../pg_crud/where_filters/generate_where_filters_src/src/source.rs#L1) |
| `ProcMacro2GenerateWhereFiltersTokenStream` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | отдельный инвариант этого служебного типа не требуется; проверка принадлежит итоговой доменной границе | [`pg_crud/where_filters/generate_where_filters_src/src/source.rs:3`](../pg_crud/where_filters/generate_where_filters_src/src/source.rs#L3) |
| `SerdeJsonGenerateWhereFiltersError` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`pg_crud/where_filters/generate_where_filters_src/src/source.rs:19`](../pg_crud/where_filters/generate_where_filters_src/src/source.rs#L19) |
| `ValidatedGenerateWhereFiltersConfig` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`pg_crud/where_filters/generate_where_filters_src/src/source.rs:17`](../pg_crud/where_filters/generate_where_filters_src/src/source.rs#L17) |

## Crate `generate_where_filters_test`

### Модуль `generate_where_filters_test::tests`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `JsonContractValue` | `struct` | `private` | named-field data structure; 1 fields; test-only | **Нет** | Не требуется | test fixture; отдельный production-инвариант не нужен | [`pg_crud/where_filters/generate_where_filters_test/src/lib.rs:4`](../pg_crud/where_filters/generate_where_filters_test/src/lib.rs#L4) |

## Crate `git_info`

### Модуль `git_info`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `GetGitCommitId` | `trait` | `public` | behavior contract; 5 associated items; production | **Нет** | Не требуется | trait не инициализируется как значение | [`git_info/src/lib.rs:222`](../git_info/src/lib.rs#L222) |
| `GetGitCommitLink` | `trait` | `public` | behavior contract; 2 associated items; production | **Нет** | Не требуется | trait не инициализируется как значение | [`git_info/src/lib.rs:216`](../git_info/src/lib.rs#L216) |
| `GitCommitId` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | внутреннее значение может расти; нужен предел длины/размера | [`git_info/src/lib.rs:24`](../git_info/src/lib.rs#L24) |
| `GitCommitIdFallback` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`git_info/src/lib.rs:82`](../git_info/src/lib.rs#L82) |
| `GitCommitIdRef` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`git_info/src/lib.rs:4`](../git_info/src/lib.rs#L4) |
| `GitCommitLink` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | внутреннее значение может расти; нужен предел длины/размера | [`git_info/src/lib.rs:84`](../git_info/src/lib.rs#L84) |
| `GitCommitLinkCapacity` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`git_info/src/lib.rs:167`](../git_info/src/lib.rs#L167) |
| `GitCommitLinkOutputRefMut` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`git_info/src/lib.rs:179`](../git_info/src/lib.rs#L179) |
| `GitInfoStringTryFromStringError` | `enum` | `public` | error enum; 1 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`git_info/src/lib.rs:39`](../git_info/src/lib.rs#L39) |
| `IsProjectCommit` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`git_info/src/lib.rs:163`](../git_info/src/lib.rs#L163) |
| `ProjectGitCommitLinkRef` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`git_info/src/lib.rs:150`](../git_info/src/lib.rs#L150) |
| `ProjectGitInfo` | `struct` | `public` | named-field data structure; 1 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`git_info/src/lib.rs:194`](../git_info/src/lib.rs#L194) |
| `StdGitCommitIdCow` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`git_info/src/lib.rs:62`](../git_info/src/lib.rs#L62) |
| `StdGitCommitLinkCow` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`git_info/src/lib.rs:121`](../git_info/src/lib.rs#L121) |
| `ValidateProjectCommitError` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`git_info/src/lib.rs:182`](../git_info/src/lib.rs#L182) |

### Модуль `git_info::tests`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `TestGitCommit` | `struct` | `private` | named-field data structure; 3 fields; test-only | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`git_info/src/lib.rs:382`](../git_info/src/lib.rs#L382) |

## Crate `initialize_environment_files`

### Модуль `initialize_environment_files`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `EnvContent` | `struct` | `private` | single-field tuple wrapper; production | **Да** | Выполнено | есть сырой растущий field; нужен предел длины/размера | [`initialize_environment_files/src/main.rs:22`](../initialize_environment_files/src/main.rs#L22) |
| `EnvContentRef` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`initialize_environment_files/src/main.rs:36`](../initialize_environment_files/src/main.rs#L36) |
| `EnvKey` | `struct` | `private` | single-field tuple wrapper; production | **Да** | Выполнено | есть сырой растущий field; нужен предел длины/размера | [`initialize_environment_files/src/main.rs:38`](../initialize_environment_files/src/main.rs#L38) |
| `EnvKeys` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | отдельный инвариант этого служебного типа не требуется; проверка принадлежит итоговой доменной границе | [`initialize_environment_files/src/main.rs:49`](../initialize_environment_files/src/main.rs#L49) |
| `InitEntries` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | отдельный инвариант этого служебного типа не требуется; проверка принадлежит итоговой доменной границе | [`initialize_environment_files/src/main.rs:74`](../initialize_environment_files/src/main.rs#L74) |
| `InitMaxBytes` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`initialize_environment_files/src/main.rs:72`](../initialize_environment_files/src/main.rs#L72) |
| `InitStringError` | `struct` | `private` | unit marker type; production | **Нет** | Не требуется | marker не содержит проверяемых данных | [`initialize_environment_files/src/main.rs:82`](../initialize_environment_files/src/main.rs#L82) |
| `InitializationEntry` | `struct` | `private` | named-field data structure; 3 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`initialize_environment_files/src/main.rs:16`](../initialize_environment_files/src/main.rs#L16) |
| `InitializationStatus` | `enum` | `private` | closed variant set; 5 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`initialize_environment_files/src/main.rs:8`](../initialize_environment_files/src/main.rs#L8) |
| `InitializeError` | `enum` | `private` | error enum; 7 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`initialize_environment_files/src/main.rs:85`](../initialize_environment_files/src/main.rs#L85) |
| `MemberSafe` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`initialize_environment_files/src/main.rs:51`](../initialize_environment_files/src/main.rs#L51) |
| `RunMode` | `enum` | `private` | closed variant set; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`initialize_environment_files/src/main.rs:3`](../initialize_environment_files/src/main.rs#L3) |
| `ServerRuntimeBoundedReadError` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`initialize_environment_files/src/main.rs:78`](../initialize_environment_files/src/main.rs#L78) |
| `StdInitIoError` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`initialize_environment_files/src/main.rs:76`](../initialize_environment_files/src/main.rs#L76) |
| `StdInitPathRef` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`initialize_environment_files/src/main.rs:70`](../initialize_environment_files/src/main.rs#L70) |
| `StdWorkspaceRootRef` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`initialize_environment_files/src/main.rs:68`](../initialize_environment_files/src/main.rs#L68) |
| `TomlInitError` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`initialize_environment_files/src/main.rs:80`](../initialize_environment_files/src/main.rs#L80) |
| `WorkspaceMember` | `struct` | `private` | single-field tuple wrapper; production | **Да** | Выполнено | есть сырой растущий field; нужен предел длины/размера | [`initialize_environment_files/src/main.rs:53`](../initialize_environment_files/src/main.rs#L53) |
| `WorkspaceMemberRef` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`initialize_environment_files/src/main.rs:64`](../initialize_environment_files/src/main.rs#L64) |
| `WorkspaceMembers` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | отдельный инвариант этого служебного типа не требуется; проверка принадлежит итоговой доменной границе | [`initialize_environment_files/src/main.rs:66`](../initialize_environment_files/src/main.rs#L66) |

## Crate `location`

### Модуль `location`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `SynItemEnumMutRef` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`location_lib/location/src/lib.rs:1`](../location_lib/location/src/lib.rs#L1) |

## Crate `location_lib`

### Модуль `location_lib::location`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `ChronoLocationDateTime` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`location_lib/src/location.rs:123`](../location_lib/src/location.rs#L123) |
| `ChronoLocationDisplayTimezone` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`location_lib/src/location.rs:121`](../location_lib/src/location.rs#L121) |
| `Location` | `struct` | `public` | named-field data structure; 6 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`location_lib/src/location.rs:142`](../location_lib/src/location.rs#L142) |
| `LocationColumn` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | source column использует нумерацию с единицы | [`location_lib/src/location.rs:38`](../location_lib/src/location.rs#L38) |
| `LocationCommit` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | атрибут валидатора задаёт инвариант типа | [`location_lib/src/location.rs:54`](../location_lib/src/location.rs#L54) |
| `LocationFile` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | атрибут валидатора задаёт инвариант типа | [`location_lib/src/location.rs:5`](../location_lib/src/location.rs#L5) |
| `LocationFileRef` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`location_lib/src/location.rs:117`](../location_lib/src/location.rs#L117) |
| `LocationLine` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | source line использует нумерацию с единицы | [`location_lib/src/location.rs:22`](../location_lib/src/location.rs#L22) |
| `Occr` | `struct` | `public` | named-field data structure; 3 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`location_lib/src/location.rs:125`](../location_lib/src/location.rs#L125) |
| `StdFmtRefMut` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`location_lib/src/location.rs:119`](../location_lib/src/location.rs#L119) |
| `StdLocationDuration` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`location_lib/src/location.rs:70`](../location_lib/src/location.rs#L70) |
| `StdTimeDuration` | `struct` | `public` | named-field data structure; 2 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`location_lib/src/location.rs:287`](../location_lib/src/location.rs#L287) |
| `StdTimeDurationNanos` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | nanosecond fraction должна быть меньше `1_000_000_000` | [`location_lib/src/location.rs:297`](../location_lib/src/location.rs#L297) |
| `StdTimeDurationSecs` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`location_lib/src/location.rs:293`](../location_lib/src/location.rs#L293) |

### Модуль `location_lib::location::tests`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `DatetimeFmt` | `struct` | `private` | named-field data structure; 1 fields; test-only | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`location_lib/src/location.rs:314`](../location_lib/src/location.rs#L314) |
| `PlaceFmt` | `struct` | `private` | named-field data structure; 2 fields; test-only | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`location_lib/src/location.rs:322`](../location_lib/src/location.rs#L322) |

## Crate `location_test`

### Модуль `location_test`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `DisplayStruct` | `struct` | `public` | named-field data structure; 2 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`location_lib/location_test/src/main.rs:97`](../location_lib/location_test/src/main.rs#L97) |
| `ErrorOne` | `enum` | `public` | closed variant set; 1 variants; production | **Нет** | Не требуется | отдельный инвариант этого служебного типа не требуется; проверка принадлежит итоговой доменной границе | [`location_lib/location_test/src/main.rs:10`](../location_lib/location_test/src/main.rs#L10) |
| `ErrorTwo` | `enum` | `public` | closed variant set; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`location_lib/location_test/src/main.rs:80`](../location_lib/location_test/src/main.rs#L80) |
| `ErrorUnnamedOne` | `enum` | `public` | closed variant set; 1 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`location_lib/location_test/src/main.rs:93`](../location_lib/location_test/src/main.rs#L93) |
| `LocationTestCount` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`location_lib/location_test/src/main.rs:66`](../location_lib/location_test/src/main.rs#L66) |
| `LocationTestFlag` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`location_lib/location_test/src/main.rs:52`](../location_lib/location_test/src/main.rs#L52) |
| `LocationTestText` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | атрибут валидатора задаёт инвариант типа | [`location_lib/location_test/src/main.rs:37`](../location_lib/location_test/src/main.rs#L37) |
| `SerdeStruct` | `struct` | `public` | named-field data structure; 3 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`location_lib/location_test/src/main.rs:110`](../location_lib/location_test/src/main.rs#L110) |

## Crate `macro_clippy_check_common`

### Модуль `macro_clippy_check_common`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `GeneratedCratePhase` | `enum` | `private` | closed variant set; 4 variants; test-only | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`macro_clippy_check_common/src/lib.rs:20`](../macro_clippy_check_common/src/lib.rs#L20) |
| `GeneratedCrateStep` | `struct` | `private` | named-field data structure; 2 fields; test-only | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`macro_clippy_check_common/src/lib.rs:39`](../macro_clippy_check_common/src/lib.rs#L39) |
| `RemoveDirOnDrop` | `struct` | `private` | named-field data structure; 1 fields; test-only | **Нет** | Не требуется | RAII-state строится из уже выбранного path; проверка относится к источнику path | [`macro_clippy_check_common/src/lib.rs:44`](../macro_clippy_check_common/src/lib.rs#L44) |

### Модуль `macro_clippy_check_common::tests`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `StdTmpDir` | `struct` | `private` | single-field tuple wrapper; test-only | **Нет** | Не требуется | отдельный инвариант этого служебного типа не требуется; проверка принадлежит итоговой доменной границе | [`macro_clippy_check_common/src/lib.rs:240`](../macro_clippy_check_common/src/lib.rs#L240) |

## Crate `macros_helpers`

### Модуль `macros_helpers::attr_identifier_str`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `AttrIdentifierName` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`macros_helpers/src/attr_identifier_str.rs:1`](../macros_helpers/src/attr_identifier_str.rs#L1) |
| `AttrIdentifierStr` | `trait` | `public` | behavior contract; 1 associated items; production | **Нет** | Не требуется | trait не инициализируется как значение | [`macros_helpers/src/attr_identifier_str.rs:3`](../macros_helpers/src/attr_identifier_str.rs#L3) |

### Модуль `macros_helpers::generate_field_location_new_token_stream`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `FieldLocationColumn` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | source column использует нумерацию с единицы | [`macros_helpers/src/generate_field_location_new_token_stream.rs:6`](../macros_helpers/src/generate_field_location_new_token_stream.rs#L6) |
| `FieldLocationFile` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`macros_helpers/src/generate_field_location_new_token_stream.rs:1`](../macros_helpers/src/generate_field_location_new_token_stream.rs#L1) |
| `FieldLocationLine` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | source line использует нумерацию с единицы | [`macros_helpers/src/generate_field_location_new_token_stream.rs:4`](../macros_helpers/src/generate_field_location_new_token_stream.rs#L4) |

### Модуль `macros_helpers::generate_if_write_is_err_token_stream`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `ProcMacro2IfWriteIsErrTokenStream` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | отдельный инвариант этого служебного типа не требуется; проверка принадлежит итоговой доменной границе | [`macros_helpers/src/generate_if_write_is_err_token_stream.rs:1`](../macros_helpers/src/generate_if_write_is_err_token_stream.rs#L1) |

### Модуль `macros_helpers::generate_simple_syn_punct`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `SynPathSegment` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`macros_helpers/src/generate_simple_syn_punct.rs:1`](../macros_helpers/src/generate_simple_syn_punct.rs#L1) |
| `SynPathSegments` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`macros_helpers/src/generate_simple_syn_punct.rs:3`](../macros_helpers/src/generate_simple_syn_punct.rs#L3) |

### Модуль `macros_helpers::get_macro_attr`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `AttrPathMatches` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`macros_helpers/src/get_macro_attr.rs:5`](../macros_helpers/src/get_macro_attr.rs#L5) |
| `MacroAttrError` | `enum` | `public` | error enum; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`macros_helpers/src/get_macro_attr.rs:7`](../macros_helpers/src/get_macro_attr.rs#L7) |
| `ProcMacro2MacroAttrMetaListTokenStreamRef` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`macros_helpers/src/get_macro_attr.rs:3`](../macros_helpers/src/get_macro_attr.rs#L3) |
| `SynMacroAttrRef` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`macros_helpers/src/get_macro_attr.rs:1`](../macros_helpers/src/get_macro_attr.rs#L1) |

### Модуль `macros_helpers::json_contract`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `ContractError` | `enum` | `public` | error enum; 4 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`macros_helpers/src/json_contract.rs:5`](../macros_helpers/src/json_contract.rs#L5) |
| `JsonFixtureRef` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`macros_helpers/src/json_contract.rs:1`](../macros_helpers/src/json_contract.rs#L1) |
| `SerdeJsonError` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`macros_helpers/src/json_contract.rs:3`](../macros_helpers/src/json_contract.rs#L3) |

### Модуль `macros_helpers::json_contract::tests`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `ReparseFails` | `struct` | `private` | unit marker type; test-only | **Нет** | Не требуется | marker не содержит проверяемых данных | [`macros_helpers/src/json_contract.rs:55`](../macros_helpers/src/json_contract.rs#L55) |
| `SerializeFails` | `struct` | `private` | unit marker type; test-only | **Нет** | Не требуется | marker не содержит проверяемых данных | [`macros_helpers/src/json_contract.rs:40`](../macros_helpers/src/json_contract.rs#L40) |
| `TestValue` | `struct` | `private` | named-field data structure; 1 fields; test-only | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`macros_helpers/src/json_contract.rs:36`](../macros_helpers/src/json_contract.rs#L36) |

### Модуль `macros_helpers::location`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `CompileErrorMessage` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`macros_helpers/src/location.rs:89`](../macros_helpers/src/location.rs#L89) |
| `LocationFieldAttr` | `enum` | `public` | closed variant set; 9 variants; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`macros_helpers/src/location.rs:1`](../macros_helpers/src/location.rs#L1) |
| `SynVariantRef` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`macros_helpers/src/location.rs:96`](../macros_helpers/src/location.rs#L96) |

### Модуль `macros_helpers::location_syn_field`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `SynLocationField` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`macros_helpers/src/location_syn_field.rs:1`](../macros_helpers/src/location_syn_field.rs#L1) |

### Модуль `macros_helpers::proc_macro2_tokens`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `ProcMacro2GeneratedRustTokenStream` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`macros_helpers/src/proc_macro2_tokens.rs:1`](../macros_helpers/src/proc_macro2_tokens.rs#L1) |

### Модуль `macros_helpers::rs_file_path`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `StdRsFilePath` | `struct` | `crate` | single-field tuple wrapper; production | **Нет** | Не требуется | отдельный инвариант этого служебного типа не требуется; проверка принадлежит итоговой доменной границе | [`macros_helpers/src/rs_file_path.rs:1`](../macros_helpers/src/rs_file_path.rs#L1) |

### Модуль `macros_helpers::status_code`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `GetOnlyOneStatusCodeError` | `enum` | `public` | error enum; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`macros_helpers/src/status_code.rs:438`](../macros_helpers/src/status_code.rs#L438) |
| `StatusCode` | `enum` | `public` | closed variant set; 60 variants; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`macros_helpers/src/status_code.rs:1`](../macros_helpers/src/status_code.rs#L1) |
| `SynStatusCodeVariantRef` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`macros_helpers/src/status_code.rs:445`](../macros_helpers/src/status_code.rs#L445) |

### Модуль `macros_helpers::syn_field`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `SynField` | `struct` | `public` | named-field data structure; 3 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`macros_helpers/src/syn_field.rs:1`](../macros_helpers/src/syn_field.rs#L1) |
| `SynFieldIdentifier` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`macros_helpers/src/syn_field.rs:7`](../macros_helpers/src/syn_field.rs#L7) |
| `SynFieldType` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`macros_helpers/src/syn_field.rs:18`](../macros_helpers/src/syn_field.rs#L18) |
| `SynFieldVis` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`macros_helpers/src/syn_field.rs:22`](../macros_helpers/src/syn_field.rs#L22) |

### Модуль `macros_helpers::test_database`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `SanitizedDatabaseTarget` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | атрибут валидатора задаёт инвариант типа | [`macros_helpers/src/test_database.rs:4`](../macros_helpers/src/test_database.rs#L4) |
| `UrlError` | `enum` | `public` | error enum; 3 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`macros_helpers/src/test_database.rs:7`](../macros_helpers/src/test_database.rs#L7) |
| `UrlRef` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`macros_helpers/src/test_database.rs:2`](../macros_helpers/src/test_database.rs#L2) |

### Модуль `macros_helpers::test_hlp`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `ExpectedFileContent` | `struct` | `crate` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`macros_helpers/src/test_hlp.rs:38`](../macros_helpers/src/test_hlp.rs#L38) |
| `ExpectedFileContentRef` | `struct` | `crate` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`macros_helpers/src/test_hlp.rs:31`](../macros_helpers/src/test_hlp.rs#L31) |
| `StdAssertFilePath` | `struct` | `crate` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`macros_helpers/src/test_hlp.rs:21`](../macros_helpers/src/test_hlp.rs#L21) |
| `StdAssertFilePathRef` | `struct` | `crate` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`macros_helpers/src/test_hlp.rs:14`](../macros_helpers/src/test_hlp.rs#L14) |
| `TestPathStem` | `struct` | `crate` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`macros_helpers/src/test_hlp.rs:4`](../macros_helpers/src/test_hlp.rs#L4) |
| `TestPathStemRef` | `struct` | `crate` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`macros_helpers/src/test_hlp.rs:2`](../macros_helpers/src/test_hlp.rs#L2) |

### Модуль `macros_helpers::tool_command`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `StdOsString` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | отдельный инвариант этого служебного типа не требуется; проверка принадлежит итоговой доменной границе | [`macros_helpers/src/tool_command.rs:5`](../macros_helpers/src/tool_command.rs#L5) |
| `StdPathRef` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`macros_helpers/src/tool_command.rs:1`](../macros_helpers/src/tool_command.rs#L1) |
| `StdProcessCommand` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`macros_helpers/src/tool_command.rs:3`](../macros_helpers/src/tool_command.rs#L3) |
| `StdProcessExitStatus` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`macros_helpers/src/tool_command.rs:22`](../macros_helpers/src/tool_command.rs#L22) |
| `StdProcessOutput` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`macros_helpers/src/tool_command.rs:24`](../macros_helpers/src/tool_command.rs#L24) |
| `ToolArgRef` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`macros_helpers/src/tool_command.rs:14`](../macros_helpers/src/tool_command.rs#L14) |
| `ToolArgsRef` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`macros_helpers/src/tool_command.rs:16`](../macros_helpers/src/tool_command.rs#L16) |
| `ToolCommand` | `struct` | `public` | named-field data structure; 2 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`macros_helpers/src/tool_command.rs:26`](../macros_helpers/src/tool_command.rs#L26) |
| `ToolEnvKeyRef` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`macros_helpers/src/tool_command.rs:18`](../macros_helpers/src/tool_command.rs#L18) |
| `ToolEnvValueRef` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`macros_helpers/src/tool_command.rs:20`](../macros_helpers/src/tool_command.rs#L20) |
| `ToolProgramRef` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`macros_helpers/src/tool_command.rs:12`](../macros_helpers/src/tool_command.rs#L12) |

### Модуль `macros_helpers::wrap_derive`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `ProcMacro2DeriveTokensRef` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`macros_helpers/src/wrap_derive.rs:1`](../macros_helpers/src/wrap_derive.rs#L1) |

### Модуль `macros_helpers::write_string_into_file`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `GeneratedFileMaximumBytes` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`macros_helpers/src/write_string_into_file.rs:7`](../macros_helpers/src/write_string_into_file.rs#L7) |
| `ShouldWriteString` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`macros_helpers/src/write_string_into_file.rs:9`](../macros_helpers/src/write_string_into_file.rs#L9) |
| `StdWrittenFilePath` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | отдельный инвариант этого служебного типа не требуется; проверка принадлежит итоговой доменной границе | [`macros_helpers/src/write_string_into_file.rs:1`](../macros_helpers/src/write_string_into_file.rs#L1) |
| `StdWrittenFilePathRef` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`macros_helpers/src/write_string_into_file.rs:3`](../macros_helpers/src/write_string_into_file.rs#L3) |
| `StringFileContentRef` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`macros_helpers/src/write_string_into_file.rs:5`](../macros_helpers/src/write_string_into_file.rs#L5) |
| `WritePathOutcome` | `enum` | `public` | closed variant set; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`macros_helpers/src/write_string_into_file.rs:13`](../macros_helpers/src/write_string_into_file.rs#L13) |

### Модуль `macros_helpers::write_token_stream_into_file`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `FormatWithCargofmt` | `enum` | `public` | closed variant set; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`macros_helpers/src/write_token_stream_into_file.rs:1`](../macros_helpers/src/write_token_stream_into_file.rs#L1) |
| `ProcMacro2TokenStreamRef` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`macros_helpers/src/write_token_stream_into_file.rs:11`](../macros_helpers/src/write_token_stream_into_file.rs#L11) |
| `ShouldWriteTokenStreamFlag` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`macros_helpers/src/write_token_stream_into_file.rs:15`](../macros_helpers/src/write_token_stream_into_file.rs#L15) |
| `ShouldWriteTokenStreamIntoFile` | `enum` | `public` | closed variant set; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`macros_helpers/src/write_token_stream_into_file.rs:6`](../macros_helpers/src/write_token_stream_into_file.rs#L6) |
| `StdRustfmtPath` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`macros_helpers/src/write_token_stream_into_file.rs:13`](../macros_helpers/src/write_token_stream_into_file.rs#L13) |

## Crate `naming`

### Модуль `naming`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `DisplayPlusToTokens` | `trait` | `public` | behavior contract; 0 associated items; production | **Нет** | Не требуется | trait не инициализируется как значение | [`naming/src/lib.rs:505`](../naming/src/lib.rs#L505) |
| `HashMap` | `struct` | `public` | unit marker type; production | **Нет** | Не требуется | marker не содержит проверяемых данных | [`naming/src/lib.rs:479`](../naming/src/lib.rs#L479) |
| `HashMapSnakeCase` | `struct` | `public` | unit marker type; production | **Нет** | Не требуется | marker не содержит проверяемых данных | [`naming/src/lib.rs:493`](../naming/src/lib.rs#L493) |
| `HashMapUpperCamelCase` | `struct` | `public` | unit marker type; production | **Нет** | Не требуется | marker не содержит проверяемых данных | [`naming/src/lib.rs:481`](../naming/src/lib.rs#L481) |
| `SwaggerUrlPathPrefix` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`naming/src/lib.rs:507`](../naming/src/lib.rs#L507) |
| `SwaggerUrlPathSelfQuotesStr` | `trait` | `public` | behavior contract; 1 associated items; production | **Нет** | Не требуется | trait не инициализируется как значение | [`naming/src/lib.rs:515`](../naming/src/lib.rs#L515) |
| `SwaggerUrlPathSelfQuotesStrValue` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`naming/src/lib.rs:509`](../naming/src/lib.rs#L509) |
| `SwaggerUrlPathSelfQuotesTokenStream` | `trait` | `public` | behavior contract; 1 associated items; production | **Нет** | Не требуется | trait не инициализируется как значение | [`naming/src/lib.rs:536`](../naming/src/lib.rs#L536) |
| `SwaggerUrlPathSelfQuotesTokenStreamValue` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`naming/src/lib.rs:511`](../naming/src/lib.rs#L511) |

## Crate `naming_common`

### Модуль `naming_common`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `CaseString` | `struct` | `private` | single-field tuple wrapper; production | **Да** | Выполнено | атрибут валидатора задаёт инвариант типа | [`naming/naming_common/src/lib.rs:74`](../naming/naming_common/src/lib.rs#L74) |
| `ConvertCaseKind` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`naming/naming_common/src/lib.rs:72`](../naming/naming_common/src/lib.rs#L72) |
| `ProcMacro2CaseTokenStream` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | отдельный инвариант этого служебного типа не требуется; проверка принадлежит итоговой доменной границе | [`naming/naming_common/src/lib.rs:79`](../naming/naming_common/src/lib.rs#L79) |

## Crate `naming_macros`

### Модуль `naming_macros`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `ProcMacro2GeneratedNamingTokenStream` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | отдельный инвариант этого служебного типа не требуется; проверка принадлежит итоговой доменной границе | [`naming/naming_macros/src/lib.rs:1`](../naming/naming_macros/src/lib.rs#L1) |
| `ProcMacro2VariantMatchingTokensRef` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`naming/naming_macros/src/lib.rs:5`](../naming/naming_macros/src/lib.rs#L5) |
| `SynEnumIdentifierRef` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`naming/naming_macros/src/lib.rs:3`](../naming/naming_macros/src/lib.rs#L3) |

## Crate `newtype`

### Модуль `newtype`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `BoundedStringAttrs` | `struct` | `private` | named-field data structure; 5 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`newtype/src/lib.rs:23`](../newtype/src/lib.rs#L23) |
| `BoundedStringOption` | `enum` | `private` | closed variant set; 5 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`newtype/src/lib.rs:63`](../newtype/src/lib.rs#L63) |
| `NewtypeAttrs` | `struct` | `private` | named-field data structure; 3 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`newtype/src/lib.rs:12`](../newtype/src/lib.rs#L12) |
| `NewtypeBool` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`newtype/src/lib.rs:138`](../newtype/src/lib.rs#L138) |
| `NewtypeOption` | `enum` | `private` | closed variant set; 31 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`newtype/src/lib.rs:71`](../newtype/src/lib.rs#L71) |
| `NewtypeTryFromAttrs` | `struct` | `private` | named-field data structure; 2 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`newtype/src/lib.rs:18`](../newtype/src/lib.rs#L18) |
| `ProcMacro2GeneratedTokenStream` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | отдельный инвариант этого служебного типа не требуется; проверка принадлежит итоговой доменной границе | [`newtype/src/lib.rs:111`](../newtype/src/lib.rs#L111) |
| `ProcMacroInputTokenStream` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | отдельный инвариант этого служебного типа не требуется; проверка принадлежит итоговой доменной границе | [`newtype/src/lib.rs:117`](../newtype/src/lib.rs#L117) |
| `SnakeIdentifier` | `struct` | `private` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`newtype/src/lib.rs:149`](../newtype/src/lib.rs#L149) |
| `SnakeIdentifierifierLen` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`newtype/src/lib.rs:150`](../newtype/src/lib.rs#L150) |
| `SnakeIdentifierifierTryFromStringError` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`newtype/src/lib.rs:157`](../newtype/src/lib.rs#L157) |
| `SynAttrsRef` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`newtype/src/lib.rs:199`](../newtype/src/lib.rs#L199) |
| `SynDeriveInputRef` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`newtype/src/lib.rs:211`](../newtype/src/lib.rs#L211) |
| `SynExpr` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`newtype/src/lib.rs:260`](../newtype/src/lib.rs#L260) |
| `SynIdentifier` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`newtype/src/lib.rs:225`](../newtype/src/lib.rs#L225) |
| `SynIdentifierRef` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`newtype/src/lib.rs:223`](../newtype/src/lib.rs#L223) |
| `SynType` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`newtype/src/lib.rs:253`](../newtype/src/lib.rs#L253) |
| `SynTypeRef` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`newtype/src/lib.rs:241`](../newtype/src/lib.rs#L241) |
| `ToErrStringMode` | `enum` | `private` | closed variant set; 3 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`newtype/src/lib.rs:105`](../newtype/src/lib.rs#L105) |
| `WireEnumAttrs` | `struct` | `private` | named-field data structure; 2 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`newtype/src/lib.rs:30`](../newtype/src/lib.rs#L30) |

### Модуль `newtype::tests::newtype::tests`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `BoolValue` | `struct` | `private` | single-field tuple wrapper; test-only | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`newtype/tests/newtype.rs:150`](../newtype/tests/newtype.rs#L150) |
| `CheckedText` | `struct` | `private` | single-field tuple wrapper; test-only | **Да** | Выполнено | внутреннее значение может расти; нужен предел длины/размера | [`newtype/tests/newtype.rs:169`](../newtype/tests/newtype.rs#L169) |
| `CheckedTextError` | `enum` | `private` | error enum; 1 variants; test-only | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`newtype/tests/newtype.rs:165`](../newtype/tests/newtype.rs#L165) |
| `ConstDisplayError` | `struct` | `private` | unit marker type; test-only | **Нет** | Не требуется | marker не содержит проверяемых данных | [`newtype/tests/newtype.rs:156`](../newtype/tests/newtype.rs#L156) |
| `DebugDisplayError` | `enum` | `private` | error enum; 1 variants; test-only | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`newtype/tests/newtype.rs:152`](../newtype/tests/newtype.rs#L152) |
| `DebugValue` | `struct` | `private` | single-field tuple wrapper; test-only | **Нет** | Не требуется | отдельный инвариант этого служебного типа не требуется; проверка принадлежит итоговой доменной границе | [`newtype/tests/newtype.rs:64`](../newtype/tests/newtype.rs#L64) |
| `DescribedValue` | `struct` | `private` | single-field tuple wrapper; test-only | **Да** | Выполнено | атрибут валидатора задаёт инвариант типа | [`newtype/tests/newtype.rs:104`](../newtype/tests/newtype.rs#L104) |
| `ExplicitErrorCheckedText` | `struct` | `private` | single-field tuple wrapper; test-only | **Да** | Выполнено | внутреннее значение может расти; нужен предел длины/размера | [`newtype/tests/newtype.rs:172`](../newtype/tests/newtype.rs#L172) |
| `GenericVec` | `struct` | `private` | single-field tuple wrapper; test-only | **Нет** | Не требуется | отдельный инвариант этого служебного типа не требуется; проверка принадлежит итоговой доменной границе | [`newtype/tests/newtype.rs:161`](../newtype/tests/newtype.rs#L161) |
| `InnerValue` | `struct` | `private` | single-field tuple wrapper; test-only | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`newtype/tests/newtype.rs:66`](../newtype/tests/newtype.rs#L66) |
| `InnerVecValue` | `struct` | `private` | single-field tuple wrapper; test-only | **Нет** | Не требуется | отдельный инвариант этого служебного типа не требуется; проверка принадлежит итоговой доменной границе | [`newtype/tests/newtype.rs:82`](../newtype/tests/newtype.rs#L82) |
| `MarkerError` | `struct` | `private` | unit marker type; test-only | **Нет** | Не требуется | marker не содержит проверяемых данных | [`newtype/tests/newtype.rs:143`](../newtype/tests/newtype.rs#L143) |
| `MutableValueRef` | `struct` | `private` | single-field tuple wrapper; test-only | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`newtype/tests/newtype.rs:163`](../newtype/tests/newtype.rs#L163) |
| `OwnedSliceValue` | `struct` | `private` | single-field tuple wrapper; test-only | **Нет** | Не требуется | отдельный инвариант этого служебного типа не требуется; проверка принадлежит итоговой доменной границе | [`newtype/tests/newtype.rs:131`](../newtype/tests/newtype.rs#L131) |
| `OwnedValue` | `struct` | `private` | single-field tuple wrapper; test-only | **Нет** | Не требуется | отдельный инвариант этого служебного типа не требуется; проверка принадлежит итоговой доменной границе | [`newtype/tests/newtype.rs:121`](../newtype/tests/newtype.rs#L121) |
| `ProcMacro2TokenValue` | `struct` | `private` | single-field tuple wrapper; test-only | **Нет** | Не требуется | отдельный инвариант этого служебного типа не требуется; проверка принадлежит итоговой доменной границе | [`newtype/tests/newtype.rs:113`](../newtype/tests/newtype.rs#L113) |
| `RedactedDebugValue` | `struct` | `private` | single-field tuple wrapper; test-only | **Нет** | Не требуется | отдельный инвариант этого служебного типа не требуется; проверка принадлежит итоговой доменной границе | [`newtype/tests/newtype.rs:139`](../newtype/tests/newtype.rs#L139) |
| `ReferentValue` | `struct` | `private` | tuple data structure; 2 fields; test-only | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`newtype/tests/newtype.rs:117`](../newtype/tests/newtype.rs#L117) |
| `ReferentValueRef` | `struct` | `private` | single-field tuple wrapper; test-only | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`newtype/tests/newtype.rs:119`](../newtype/tests/newtype.rs#L119) |
| `RichValue` | `struct` | `private` | single-field tuple wrapper; test-only | **Да** | Выполнено | атрибут валидатора задаёт инвариант типа | [`newtype/tests/newtype.rs:107`](../newtype/tests/newtype.rs#L107) |
| `SampleEnum` | `enum` | `private` | closed variant set; 2 variants; test-only | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`newtype/tests/newtype.rs:175`](../newtype/tests/newtype.rs#L175) |
| `SliceValueRef` | `struct` | `private` | single-field tuple wrapper; test-only | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`newtype/tests/newtype.rs:133`](../newtype/tests/newtype.rs#L133) |
| `StdArcGenericValue` | `struct` | `private` | single-field tuple wrapper; test-only | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`newtype/tests/newtype.rs:159`](../newtype/tests/newtype.rs#L159) |
| `StdPathBuf` | `struct` | `private` | single-field tuple wrapper; test-only | **Нет** | Не требуется | отдельный инвариант этого служебного типа не требуется; проверка принадлежит итоговой доменной границе | [`newtype/tests/newtype.rs:135`](../newtype/tests/newtype.rs#L135) |
| `StdTransparentErrorValue` | `struct` | `private` | single-field tuple wrapper; test-only | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`newtype/tests/newtype.rs:141`](../newtype/tests/newtype.rs#L141) |
| `StringValue` | `struct` | `private` | single-field tuple wrapper; test-only | **Да** | Выполнено | атрибут валидатора задаёт инвариант типа | [`newtype/tests/newtype.rs:45`](../newtype/tests/newtype.rs#L45) |
| `TargetVecValue` | `struct` | `private` | single-field tuple wrapper; test-only | **Нет** | Не требуется | отдельный инвариант этого служебного типа не требуется; проверка принадлежит итоговой доменной границе | [`newtype/tests/newtype.rs:94`](../newtype/tests/newtype.rs#L94) |
| `TransparentDebugValue` | `struct` | `private` | single-field tuple wrapper; test-only | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`newtype/tests/newtype.rs:137`](../newtype/tests/newtype.rs#L137) |
| `UsizeValue` | `struct` | `private` | single-field tuple wrapper; test-only | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`newtype/tests/newtype.rs:60`](../newtype/tests/newtype.rs#L60) |
| `ValidatedValue` | `struct` | `private` | single-field tuple wrapper; test-only | **Да** | Выполнено | атрибут валидатора задаёт инвариант типа | [`newtype/tests/newtype.rs:110`](../newtype/tests/newtype.rs#L110) |
| `VecValue` | `struct` | `private` | single-field tuple wrapper; test-only | **Нет** | Не требуется | отдельный инвариант этого служебного типа не требуется; проверка принадлежит итоговой доменной границе | [`newtype/tests/newtype.rs:78`](../newtype/tests/newtype.rs#L78) |

### Модуль `newtype::tests::newtype::tests::to_err_string`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `ErrorText` | `struct` | `crate` | single-field tuple wrapper; test-only | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`newtype/tests/newtype.rs:5`](../newtype/tests/newtype.rs#L5) |
| `ErrorTextTryFromStringError` | `enum` | `crate` | error enum; 1 variants; test-only | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`newtype/tests/newtype.rs:7`](../newtype/tests/newtype.rs#L7) |
| `ToErrString` | `trait` | `crate` | behavior contract; 1 associated items; test-only | **Нет** | Не требуется | trait не инициализируется как значение | [`newtype/tests/newtype.rs:37`](../newtype/tests/newtype.rs#L37) |

## Crate `notification_service`

### Модуль `notification_service`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `AxumNotificationJson` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`notification_service/src/main.rs:13`](../notification_service/src/main.rs#L13) |
| `AxumNotificationResponse` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`notification_service/src/main.rs:16`](../notification_service/src/main.rs#L16) |
| `AxumNotificationRouter` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`notification_service/src/main.rs:19`](../notification_service/src/main.rs#L19) |
| `AxumNotificationState` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`notification_service/src/main.rs:10`](../notification_service/src/main.rs#L10) |
| `HttpNotificationApiProblem` | `enum` | `private` | error enum; 4 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`notification_service/src/main.rs:26`](../notification_service/src/main.rs#L26) |
| `HttpNotificationStatusCode` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`notification_service/src/main.rs:22`](../notification_service/src/main.rs#L22) |
| `MetricsExporterPrometheusHandle` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`notification_service/src/main.rs:28`](../notification_service/src/main.rs#L28) |
| `MetricsExporterPrometheusNotificationBuildError` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`notification_service/src/main.rs:117`](../notification_service/src/main.rs#L117) |
| `NotificationBodyMaximumBytes` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`notification_service/src/main.rs:31`](../notification_service/src/main.rs#L31) |
| `NotificationConfigError` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`notification_service/src/main.rs:102`](../notification_service/src/main.rs#L102) |
| `NotificationOpenApi` | `struct` | `private` | unit marker type; production | **Нет** | Не требуется | marker не содержит проверяемых данных | [`notification_service/src/main.rs:198`](../notification_service/src/main.rs#L198) |
| `NotificationServeError` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`notification_service/src/main.rs:114`](../notification_service/src/main.rs#L114) |
| `NotificationServiceError` | `enum` | `private` | error enum; 7 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`notification_service/src/main.rs:85`](../notification_service/src/main.rs#L85) |
| `NotificationState` | `struct` | `private` | named-field data structure; 2 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`notification_service/src/main.rs:5`](../notification_service/src/main.rs#L5) |
| `SqlxNotificationDatabaseError` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`notification_service/src/main.rs:105`](../notification_service/src/main.rs#L105) |
| `SqlxNotificationMigrationError` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`notification_service/src/main.rs:108`](../notification_service/src/main.rs#L108) |
| `StdNotificationExitCode` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`notification_service/src/main.rs:34`](../notification_service/src/main.rs#L34) |
| `StdNotificationIoError` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`notification_service/src/main.rs:111`](../notification_service/src/main.rs#L111) |

## Crate `notification_service_config`

### Модуль `notification_service_config`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `Config` | `struct` | `public` | named-field data structure; 6 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`notification_service_config/src/lib.rs:1`](../notification_service_config/src/lib.rs#L1) |

## Crate `notification_service_contract`

### Модуль `notification_service_contract`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `CreateNotificationReq` | `struct` | `public` | named-field data structure; 1 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`notification_service_contract/src/lib.rs:4`](../notification_service_contract/src/lib.rs#L4) |
| `CreateNotificationRes` | `struct` | `public` | named-field data structure; 1 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`notification_service_contract/src/lib.rs:20`](../notification_service_contract/src/lib.rs#L20) |
| `CreateNotificationRoute` | `struct` | `public` | unit marker type; production | **Нет** | Не требуется | marker не содержит проверяемых данных | [`notification_service_contract/src/lib.rs:64`](../notification_service_contract/src/lib.rs#L64) |
| `NotificationMessage` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`notification_service_contract/src/lib.rs:37`](../notification_service_contract/src/lib.rs#L37) |
| `NotificationMessageTryFromStringError` | `enum` | `public` | error enum; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`notification_service_contract/src/lib.rs:90`](../notification_service_contract/src/lib.rs#L90) |
| `NotificationRoute` | `enum` | `public` | closed variant set; 1 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`notification_service_contract/src/lib.rs:80`](../notification_service_contract/src/lib.rs#L80) |
| `UuidNotificationId` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`notification_service_contract/src/lib.rs:49`](../notification_service_contract/src/lib.rs#L49) |

## Crate `optml`

### Модуль `optml`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `ReplaceLts` | `struct` | `private` | unit marker type; production | **Нет** | Не требуется | marker не содержит проверяемых данных | [`optml/src/lib.rs:1`](../optml/src/lib.rs#L1) |
| `SynFieldTyWithStaticLts` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`optml/src/lib.rs:2`](../optml/src/lib.rs#L2) |

## Crate `panic_location`

### Модуль `panic_location`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `PanicColumn` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`panic_location/src/lib.rs:8`](../panic_location/src/lib.rs#L8) |
| `PanicFile` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`panic_location/src/lib.rs:4`](../panic_location/src/lib.rs#L4) |
| `PanicLine` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`panic_location/src/lib.rs:6`](../panic_location/src/lib.rs#L6) |

## Crate `pg_crud_common`

### Модуль `pg_crud_common`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `AddOperator` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`pg_crud/pg_crud_common/src/lib.rs:490`](../pg_crud/pg_crud_common/src/lib.rs#L490) |
| `AllEnumVariants` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | отдельный инвариант этого служебного типа не требуется; проверка принадлежит итоговой доменной границе | [`pg_crud/pg_crud_common/src/lib.rs:117`](../pg_crud/pg_crud_common/src/lib.rs#L117) |
| `AllEnumVariantsArrayDefaultSomeOneElement` | `trait` | `public` | behavior contract; 1 associated items; production | **Нет** | Не требуется | trait не инициализируется как значение | [`pg_crud/pg_crud_common/src/lib.rs:119`](../pg_crud/pg_crud_common/src/lib.rs#L119) |
| `AllEnumVariantsArrayDefaultSomeOneElementMaxPageSize` | `trait` | `public` | behavior contract; 1 associated items; production | **Нет** | Не требуется | trait не инициализируется как значение | [`pg_crud/pg_crud_common/src/lib.rs:122`](../pg_crud/pg_crud_common/src/lib.rs#L122) |
| `DefaultSomeOneElement` | `trait` | `public` | behavior contract; 1 associated items; production | **Нет** | Не требуется | trait не инициализируется как значение | [`pg_crud/pg_crud_common/src/lib.rs:125`](../pg_crud/pg_crud_common/src/lib.rs#L125) |
| `DefaultSomeOneElementMaxPageSize` | `trait` | `public` | behavior contract; 1 associated items; production | **Нет** | Не требуется | trait не инициализируется как значение | [`pg_crud/pg_crud_common/src/lib.rs:128`](../pg_crud/pg_crud_common/src/lib.rs#L128) |
| `EqOperator` | `enum` | `public` | closed variant set; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`pg_crud/pg_crud_common/src/lib.rs:1667`](../pg_crud/pg_crud_common/src/lib.rs#L1667) |
| `EqOperatorQueryStr` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`pg_crud/pg_crud_common/src/lib.rs:1681`](../pg_crud/pg_crud_common/src/lib.rs#L1681) |
| `IsPrimaryKey` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`pg_crud/pg_crud_common/src/lib.rs:494`](../pg_crud/pg_crud_common/src/lib.rs#L494) |
| `IsStringEmpty` | `trait` | `public` | behavior contract; 1 associated items; production | **Нет** | Не требуется | trait не инициализируется как значение | [`pg_crud/pg_crud_common/src/lib.rs:1273`](../pg_crud/pg_crud_common/src/lib.rs#L1273) |
| `IsStringEmptyRes` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`pg_crud/pg_crud_common/src/lib.rs:1269`](../pg_crud/pg_crud_common/src/lib.rs#L1269) |
| `NonPrimaryKeyPgTypeReadIds` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`pg_crud/pg_crud_common/src/lib.rs:1626`](../pg_crud/pg_crud_common/src/lib.rs#L1626) |
| `NotEmptyUniqueVec` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`pg_crud/pg_crud_common/src/lib.rs:1288`](../pg_crud/pg_crud_common/src/lib.rs#L1288) |
| `NotEmptyUniqueVecTryNewError` | `enum` | `public` | error enum; 3 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`pg_crud/pg_crud_common/src/lib.rs:1276`](../pg_crud/pg_crud_common/src/lib.rs#L1276) |
| `NotZeroUnsignedPartOfI32` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`pg_crud/pg_crud_common/src/lib.rs:1809`](../pg_crud/pg_crud_common/src/lib.rs#L1809) |
| `NotZeroUnsignedPartOfI32TryFromI32Error` | `enum` | `public` | error enum; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`pg_crud/pg_crud_common/src/lib.rs:1844`](../pg_crud/pg_crud_common/src/lib.rs#L1844) |
| `NullableJsonObjPgTypeWhereFilter` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`pg_crud/pg_crud_common/src/lib.rs:511`](../pg_crud/pg_crud_common/src/lib.rs#L511) |
| `Operator` | `enum` | `public` | closed variant set; 4 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`pg_crud/pg_crud_common/src/lib.rs:131`](../pg_crud/pg_crud_common/src/lib.rs#L131) |
| `Order` | `enum` | `public` | closed variant set; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`pg_crud/pg_crud_common/src/lib.rs:908`](../pg_crud/pg_crud_common/src/lib.rs#L908) |
| `OrderBy` | `struct` | `public` | named-field data structure; 2 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`pg_crud/pg_crud_common/src/lib.rs:992`](../pg_crud/pg_crud_common/src/lib.rs#L992) |
| `OrderSnakeCaseStr` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`pg_crud/pg_crud_common/src/lib.rs:942`](../pg_crud/pg_crud_common/src/lib.rs#L942) |
| `OrderUpperCamelCaseStr` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`pg_crud/pg_crud_common/src/lib.rs:961`](../pg_crud/pg_crud_common/src/lib.rs#L961) |
| `PaginationBase` | `struct` | `public` | named-field data structure; 2 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`pg_crud/pg_crud_common/src/lib.rs:1021`](../pg_crud/pg_crud_common/src/lib.rs#L1021) |
| `PaginationStartsWithZero` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`pg_crud/pg_crud_common/src/lib.rs:1113`](../pg_crud/pg_crud_common/src/lib.rs#L1113) |
| `PaginationStartsWithZeroRaw` | `struct` | `private` | named-field data structure; 2 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`pg_crud/pg_crud_common/src/lib.rs:1108`](../pg_crud/pg_crud_common/src/lib.rs#L1108) |
| `PaginationStartsWithZeroTryNewError` | `enum` | `public` | error enum; 3 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`pg_crud/pg_crud_common/src/lib.rs:1130`](../pg_crud/pg_crud_common/src/lib.rs#L1130) |
| `PgType` | `trait` | `public` | behavior contract; 20 associated items; production | **Нет** | Не требуется | trait не инициализируется как значение | [`pg_crud/pg_crud_common/src/lib.rs:300`](../pg_crud/pg_crud_common/src/lib.rs#L300) |
| `PgTypeEqOperator` | `trait` | `public` | behavior contract; 1 associated items; production | **Нет** | Не требуется | trait не инициализируется как значение | [`pg_crud/pg_crud_common/src/lib.rs:1693`](../pg_crud/pg_crud_common/src/lib.rs#L1693) |
| `PgTypeGreaterThanTest` | `struct` | `public` | named-field data structure; 3 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`pg_crud/pg_crud_common/src/lib.rs:450`](../pg_crud/pg_crud_common/src/lib.rs#L450) |
| `PgTypeGreaterThanVariant` | `enum` | `public` | closed variant set; 3 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`pg_crud/pg_crud_common/src/lib.rs:250`](../pg_crud/pg_crud_common/src/lib.rs#L250) |
| `PgTypeLenGreaterThanTest` | `struct` | `public` | named-field data structure; 3 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`pg_crud/pg_crud_common/src/lib.rs:457`](../pg_crud/pg_crud_common/src/lib.rs#L457) |
| `PgTypeNotPrimaryKey` | `trait` | `public` | behavior contract; 2 associated items; production | **Нет** | Не требуется | trait не инициализируется как значение | [`pg_crud/pg_crud_common/src/lib.rs:369`](../pg_crud/pg_crud_common/src/lib.rs#L369) |
| `PgTypePrimaryKey` | `trait` | `public` | behavior contract; 6 associated items; production | **Нет** | Не требуется | trait не инициализируется как значение | [`pg_crud/pg_crud_common/src/lib.rs:354`](../pg_crud/pg_crud_common/src/lib.rs#L354) |
| `PgTypeTestCases` | `trait` | `public` | behavior contract; 18 associated items; test-only | **Нет** | Не требуется | trait не инициализируется как значение | [`pg_crud/pg_crud_common/src/lib.rs:374`](../pg_crud/pg_crud_common/src/lib.rs#L374) |
| `PgTypeWhere` | `struct` | `public` | named-field data structure; 2 fields; production | **Да** | Выполнено | fallible-конструктор показывает наличие инварианта | [`pg_crud/pg_crud_common/src/lib.rs:615`](../pg_crud/pg_crud_common/src/lib.rs#L615) |
| `PgTypeWhereFilter` | `trait` | `public` | behavior contract; 2 associated items; production | **Нет** | Не требуется | trait не инициализируется как значение | [`pg_crud/pg_crud_common/src/lib.rs:498`](../pg_crud/pg_crud_common/src/lib.rs#L498) |
| `SingleOrMultiple` | `enum` | `public` | closed variant set; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`pg_crud/pg_crud_common/src/lib.rs:1916`](../pg_crud/pg_crud_common/src/lib.rs#L1916) |
| `SqlxPostgresQuery` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`pg_crud/pg_crud_common/src/lib.rs:464`](../pg_crud/pg_crud_common/src/lib.rs#L464) |
| `UnsignedPartOfI32` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`pg_crud/pg_crud_common/src/lib.rs:1696`](../pg_crud/pg_crud_common/src/lib.rs#L1696) |
| `UnsignedPartOfI32Raw` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`pg_crud/pg_crud_common/src/lib.rs:1735`](../pg_crud/pg_crud_common/src/lib.rs#L1735) |
| `UnsignedPartOfI32TryFromI32Error` | `enum` | `public` | error enum; 1 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`pg_crud/pg_crud_common/src/lib.rs:1716`](../pg_crud/pg_crud_common/src/lib.rs#L1716) |
| `UuidUuidTestCases` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`pg_crud/pg_crud_common/src/lib.rs:1931`](../pg_crud/pg_crud_common/src/lib.rs#L1931) |
| `V` | `struct` | `public` | named-field data structure; 1 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`pg_crud/pg_crud_common/src/lib.rs:1237`](../pg_crud/pg_crud_common/src/lib.rs#L1237) |

### Модуль `pg_crud_common::advisory_lock`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `PgRelationCapacityError` | `enum` | `public` | error enum; 3 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`pg_crud/pg_crud_common/src/advisory_lock.rs:19`](../pg_crud/pg_crud_common/src/advisory_lock.rs#L19) |
| `PgRelationCapacityMaximum` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`pg_crud/pg_crud_common/src/advisory_lock.rs:6`](../pg_crud/pg_crud_common/src/advisory_lock.rs#L6) |
| `PgRelationLockError` | `enum` | `public` | error enum; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`pg_crud/pg_crud_common/src/advisory_lock.rs:63`](../pg_crud/pg_crud_common/src/advisory_lock.rs#L63) |
| `PgRelationLockNamespace` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`pg_crud/pg_crud_common/src/advisory_lock.rs:32`](../pg_crud/pg_crud_common/src/advisory_lock.rs#L32) |
| `PgRelationResourceId` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`pg_crud/pg_crud_common/src/advisory_lock.rs:29`](../pg_crud/pg_crud_common/src/advisory_lock.rs#L29) |
| `PgRelationResourceIds` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`pg_crud/pg_crud_common/src/advisory_lock.rs:49`](../pg_crud/pg_crud_common/src/advisory_lock.rs#L49) |
| `PgRelationRowCount` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`pg_crud/pg_crud_common/src/advisory_lock.rs:3`](../pg_crud/pg_crud_common/src/advisory_lock.rs#L3) |
| `SqlxPgRelationLockConnectionRef` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`pg_crud/pg_crud_common/src/advisory_lock.rs:74`](../pg_crud/pg_crud_common/src/advisory_lock.rs#L74) |
| `SqlxPgRelationLockError` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`pg_crud/pg_crud_common/src/advisory_lock.rs:71`](../pg_crud/pg_crud_common/src/advisory_lock.rs#L71) |

### Модуль `pg_crud_common::batch_validation`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `BatchDuplicatePolicy` | `enum` | `public` | closed variant set; 3 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`pg_crud/pg_crud_common/src/batch_validation.rs:1`](../pg_crud/pg_crud_common/src/batch_validation.rs#L1) |
| `BatchInvalidItemCount` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`pg_crud/pg_crud_common/src/batch_validation.rs:18`](../pg_crud/pg_crud_common/src/batch_validation.rs#L18) |
| `BatchInvalidItems` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | отдельный инвариант этого служебного типа не требуется; проверка принадлежит итоговой доменной границе | [`pg_crud/pg_crud_common/src/batch_validation.rs:38`](../pg_crud/pg_crud_common/src/batch_validation.rs#L38) |
| `BatchProcessedItemCount` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`pg_crud/pg_crud_common/src/batch_validation.rs:8`](../pg_crud/pg_crud_common/src/batch_validation.rs#L8) |
| `BatchStoppedEarly` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`pg_crud/pg_crud_common/src/batch_validation.rs:28`](../pg_crud/pg_crud_common/src/batch_validation.rs#L28) |
| `BatchValidationReport` | `struct` | `public` | named-field data structure; 4 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`pg_crud/pg_crud_common/src/batch_validation.rs:43`](../pg_crud/pg_crud_common/src/batch_validation.rs#L43) |
| `StdBatchRecords` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | отдельный инвариант этого служебного типа не требуется; проверка принадлежит итоговой доменной границе | [`pg_crud/pg_crud_common/src/batch_validation.rs:40`](../pg_crud/pg_crud_common/src/batch_validation.rs#L40) |

### Модуль `pg_crud_common::bind_index`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `QueryPartIncrement` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`pg_crud/pg_crud_common/src/bind_index.rs:1`](../pg_crud/pg_crud_common/src/bind_index.rs#L1) |
| `QueryPartIncrementMut` | `trait` | `public` | behavior contract; 1 associated items; production | **Нет** | Не требуется | trait не инициализируется как значение | [`pg_crud/pg_crud_common/src/bind_index.rs:20`](../pg_crud/pg_crud_common/src/bind_index.rs#L20) |

### Модуль `pg_crud_common::bounded_btree_map`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `BoundedBTreeMapError` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`pg_crud/pg_crud_common/src/bounded_btree_map.rs:4`](../pg_crud/pg_crud_common/src/bounded_btree_map.rs#L4) |
| `StdBoundedBTreeMap` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`pg_crud/pg_crud_common/src/bounded_btree_map.rs:9`](../pg_crud/pg_crud_common/src/bounded_btree_map.rs#L9) |
| `StdBoundedBTreeMapLen` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`pg_crud/pg_crud_common/src/bounded_btree_map.rs:1`](../pg_crud/pg_crud_common/src/bounded_btree_map.rs#L1) |
| `StdBoundedBTreeMapVisitor` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`pg_crud/pg_crud_common/src/bounded_btree_map.rs:44`](../pg_crud/pg_crud_common/src/bounded_btree_map.rs#L44) |

### Модуль `pg_crud_common::bounded_unique_vec`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `BoundedUniqueVec` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`pg_crud/pg_crud_common/src/bounded_unique_vec.rs:22`](../pg_crud/pg_crud_common/src/bounded_unique_vec.rs#L22) |
| `StdBoundedUniqueVecVisitor` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`pg_crud/pg_crud_common/src/bounded_unique_vec.rs:51`](../pg_crud/pg_crud_common/src/bounded_unique_vec.rs#L51) |
| `UniqueVecError` | `enum` | `public` | error enum; 4 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`pg_crud/pg_crud_common/src/bounded_unique_vec.rs:4`](../pg_crud/pg_crud_common/src/bounded_unique_vec.rs#L4) |
| `UniqueVecLen` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`pg_crud/pg_crud_common/src/bounded_unique_vec.rs:1`](../pg_crud/pg_crud_common/src/bounded_unique_vec.rs#L1) |

### Модуль `pg_crud_common::bounded_vec`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `BoundedVec` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`pg_crud/pg_crud_common/src/bounded_vec.rs:31`](../pg_crud/pg_crud_common/src/bounded_vec.rs#L31) |
| `BoundedVecError` | `enum` | `public` | error enum; 3 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`pg_crud/pg_crud_common/src/bounded_vec.rs:12`](../pg_crud/pg_crud_common/src/bounded_vec.rs#L12) |
| `BoundedVecLen` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`pg_crud/pg_crud_common/src/bounded_vec.rs:1`](../pg_crud/pg_crud_common/src/bounded_vec.rs#L1) |
| `StdPhantomDataBoundedVecVisitor` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`pg_crud/pg_crud_common/src/bounded_vec.rs:81`](../pg_crud/pg_crud_common/src/bounded_vec.rs#L81) |

### Модуль `pg_crud_common::cardinality`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `DuplicateCandidates` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | отдельный инвариант этого служебного типа не требуется; проверка принадлежит итоговой доменной границе | [`pg_crud/pg_crud_common/src/cardinality.rs:3`](../pg_crud/pg_crud_common/src/cardinality.rs#L3) |
| `DuplicateIdx` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`pg_crud/pg_crud_common/src/cardinality.rs:1`](../pg_crud/pg_crud_common/src/cardinality.rs#L1) |

### Модуль `pg_crud_common::cursor`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `CursorCodec` | `struct` | `public` | named-field data structure; 2 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`pg_crud/pg_crud_common/src/cursor.rs:127`](../pg_crud/pg_crud_common/src/cursor.rs#L127) |
| `CursorCodecBuildError` | `enum` | `public` | error enum; 1 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`pg_crud/pg_crud_common/src/cursor.rs:198`](../pg_crud/pg_crud_common/src/cursor.rs#L198) |
| `CursorDecodeError` | `enum` | `public` | error enum; 5 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`pg_crud/pg_crud_common/src/cursor.rs:212`](../pg_crud/pg_crud_common/src/cursor.rs#L212) |
| `CursorEncodeError` | `enum` | `public` | error enum; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`pg_crud/pg_crud_common/src/cursor.rs:204`](../pg_crud/pg_crud_common/src/cursor.rs#L204) |
| `CursorMaximumLength` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`pg_crud/pg_crud_common/src/cursor.rs:3`](../pg_crud/pg_crud_common/src/cursor.rs#L3) |
| `CursorPaginationUsage` | `enum` | `public` | closed variant set; 4 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`pg_crud/pg_crud_common/src/cursor.rs:6`](../pg_crud/pg_crud_common/src/cursor.rs#L6) |
| `CursorPayload` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`pg_crud/pg_crud_common/src/cursor.rs:81`](../pg_crud/pg_crud_common/src/cursor.rs#L81) |
| `CursorPayloadError` | `struct` | `public` | unit marker type; production | **Нет** | Не требуется | marker не содержит проверяемых данных | [`pg_crud/pg_crud_common/src/cursor.rs:100`](../pg_crud/pg_crud_common/src/cursor.rs#L100) |
| `CursorSigningKey` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`pg_crud/pg_crud_common/src/cursor.rs:55`](../pg_crud/pg_crud_common/src/cursor.rs#L55) |
| `CursorSigningKeyError` | `struct` | `public` | unit marker type; production | **Нет** | Не требуется | marker не содержит проверяемых данных | [`pg_crud/pg_crud_common/src/cursor.rs:77`](../pg_crud/pg_crud_common/src/cursor.rs#L77) |
| `OffsetPaginationPresence` | `enum` | `public` | closed variant set; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`pg_crud/pg_crud_common/src/cursor.rs:33`](../pg_crud/pg_crud_common/src/cursor.rs#L33) |
| `SignedCursor` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`pg_crud/pg_crud_common/src/cursor.rs:104`](../pg_crud/pg_crud_common/src/cursor.rs#L104) |
| `SignedCursorError` | `struct` | `public` | unit marker type; production | **Нет** | Не требуется | marker не содержит проверяемых данных | [`pg_crud/pg_crud_common/src/cursor.rs:123`](../pg_crud/pg_crud_common/src/cursor.rs#L123) |
| `SignedCursorPresence` | `enum` | `public` | closed variant set; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`pg_crud/pg_crud_common/src/cursor.rs:39`](../pg_crud/pg_crud_common/src/cursor.rs#L39) |

### Модуль `pg_crud_common::date_sql_filter`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `ChronoUtcDateTimeRef` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`pg_crud/pg_crud_common/src/date_sql_filter.rs:1`](../pg_crud/pg_crud_common/src/date_sql_filter.rs#L1) |
| `ChronoUtcDateTimes` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | отдельный инвариант этого служебного типа не требуется; проверка принадлежит итоговой доменной границе | [`pg_crud/pg_crud_common/src/date_sql_filter.rs:31`](../pg_crud/pg_crud_common/src/date_sql_filter.rs#L31) |
| `DateFilterBounds` | `struct` | `public` | named-field data structure; 4 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`pg_crud/pg_crud_common/src/date_sql_filter.rs:4`](../pg_crud/pg_crud_common/src/date_sql_filter.rs#L4) |
| `DateSqlFilter` | `struct` | `public` | named-field data structure; 2 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`pg_crud/pg_crud_common/src/date_sql_filter.rs:34`](../pg_crud/pg_crud_common/src/date_sql_filter.rs#L34) |
| `DateSqlFilterError` | `enum` | `public` | error enum; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`pg_crud/pg_crud_common/src/date_sql_filter.rs:46`](../pg_crud/pg_crud_common/src/date_sql_filter.rs#L46) |
| `StdDateSqlBindStart` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`pg_crud/pg_crud_common/src/date_sql_filter.rs:28`](../pg_crud/pg_crud_common/src/date_sql_filter.rs#L28) |

### Модуль `pg_crud_common::db_schema_conformance`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `DbCatalogSnapshot` | `struct` | `public` | named-field data structure; 1 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`pg_crud/pg_crud_common/src/db_schema_conformance.rs:345`](../pg_crud/pg_crud_common/src/db_schema_conformance.rs#L345) |
| `DbColumnContractSnapshot` | `struct` | `public` | named-field data structure; 4 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`pg_crud/pg_crud_common/src/db_schema_conformance.rs:251`](../pg_crud/pg_crud_common/src/db_schema_conformance.rs#L251) |
| `DbColumnContractSnapshots` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | отдельный инвариант этого служебного типа не требуется; проверка принадлежит итоговой доменной границе | [`pg_crud/pg_crud_common/src/db_schema_conformance.rs:194`](../pg_crud/pg_crud_common/src/db_schema_conformance.rs#L194) |
| `DbColumnHasServerDefault` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`pg_crud/pg_crud_common/src/db_schema_conformance.rs:26`](../pg_crud/pg_crud_common/src/db_schema_conformance.rs#L26) |
| `DbColumnNullable` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`pg_crud/pg_crud_common/src/db_schema_conformance.rs:14`](../pg_crud/pg_crud_common/src/db_schema_conformance.rs#L14) |
| `DbColumnSnapshot` | `struct` | `public` | named-field data structure; 4 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`pg_crud/pg_crud_common/src/db_schema_conformance.rs:284`](../pg_crud/pg_crud_common/src/db_schema_conformance.rs#L284) |
| `DbColumnSnapshots` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | отдельный инвариант этого служебного типа не требуется; проверка принадлежит итоговой доменной границе | [`pg_crud/pg_crud_common/src/db_schema_conformance.rs:222`](../pg_crud/pg_crud_common/src/db_schema_conformance.rs#L222) |
| `DbColumnSpec` | `struct` | `public` | named-field data structure; 4 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`pg_crud/pg_crud_common/src/db_schema_conformance.rs:29`](../pg_crud/pg_crud_common/src/db_schema_conformance.rs#L29) |
| `DbColumnSpecs` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | отдельный инвариант этого служебного типа не требуется; проверка принадлежит итоговой доменной границе | [`pg_crud/pg_crud_common/src/db_schema_conformance.rs:36`](../pg_crud/pg_crud_common/src/db_schema_conformance.rs#L36) |
| `DbDefaultSpec` | `struct` | `public` | named-field data structure; 2 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`pg_crud/pg_crud_common/src/db_schema_conformance.rs:122`](../pg_crud/pg_crud_common/src/db_schema_conformance.rs#L122) |
| `DbDefaultSpecs` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | отдельный инвариант этого служебного типа не требуется; проверка принадлежит итоговой доменной границе | [`pg_crud/pg_crud_common/src/db_schema_conformance.rs:84`](../pg_crud/pg_crud_common/src/db_schema_conformance.rs#L84) |
| `DbExtendedTableSchema` | `trait` | `public` | behavior contract; 2 associated items; production | **Нет** | Не требуется | trait не инициализируется как значение | [`pg_crud/pg_crud_common/src/db_schema_conformance.rs:153`](../pg_crud/pg_crud_common/src/db_schema_conformance.rs#L153) |
| `DbKeyContractSnapshot` | `enum` | `public` | closed variant set; 3 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`pg_crud/pg_crud_common/src/db_schema_conformance.rs:169`](../pg_crud/pg_crud_common/src/db_schema_conformance.rs#L169) |
| `DbKeyContractSnapshots` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | отдельный инвариант этого служебного типа не требуется; проверка принадлежит итоговой доменной границе | [`pg_crud/pg_crud_common/src/db_schema_conformance.rs:208`](../pg_crud/pg_crud_common/src/db_schema_conformance.rs#L208) |
| `DbKeySpec` | `enum` | `public` | closed variant set; 3 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`pg_crud/pg_crud_common/src/db_schema_conformance.rs:158`](../pg_crud/pg_crud_common/src/db_schema_conformance.rs#L158) |
| `DbKeySpecs` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | отдельный инвариант этого служебного типа не требуется; проверка принадлежит итоговой доменной границе | [`pg_crud/pg_crud_common/src/db_schema_conformance.rs:60`](../pg_crud/pg_crud_common/src/db_schema_conformance.rs#L60) |
| `DbObjectKind` | `enum` | `public` | closed variant set; 10 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`pg_crud/pg_crud_common/src/db_schema_conformance.rs:308`](../pg_crud/pg_crud_common/src/db_schema_conformance.rs#L308) |
| `DbObjectSnapshot` | `struct` | `public` | named-field data structure; 3 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`pg_crud/pg_crud_common/src/db_schema_conformance.rs:322`](../pg_crud/pg_crud_common/src/db_schema_conformance.rs#L322) |
| `DbObjectSnapshots` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | отдельный инвариант этого служебного типа не требуется; проверка принадлежит итоговой доменной границе | [`pg_crud/pg_crud_common/src/db_schema_conformance.rs:236`](../pg_crud/pg_crud_common/src/db_schema_conformance.rs#L236) |
| `DbObjectSpec` | `struct` | `public` | named-field data structure; 3 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`pg_crud/pg_crud_common/src/db_schema_conformance.rs:133`](../pg_crud/pg_crud_common/src/db_schema_conformance.rs#L133) |
| `DbObjectSpecs` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | отдельный инвариант этого служебного типа не требуется; проверка принадлежит итоговой доменной границе | [`pg_crud/pg_crud_common/src/db_schema_conformance.rs:72`](../pg_crud/pg_crud_common/src/db_schema_conformance.rs#L72) |
| `DbSchemaConformanceError` | `enum` | `public` | error enum; 10 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`pg_crud/pg_crud_common/src/db_schema_conformance.rs:368`](../pg_crud/pg_crud_common/src/db_schema_conformance.rs#L368) |
| `DbSchemaNameRef` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`pg_crud/pg_crud_common/src/db_schema_conformance.rs:278`](../pg_crud/pg_crud_common/src/db_schema_conformance.rs#L278) |
| `DbSchemaText` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | атрибут валидатора задаёт инвариант типа | [`pg_crud/pg_crud_common/src/db_schema_conformance.rs:7`](../pg_crud/pg_crud_common/src/db_schema_conformance.rs#L7) |
| `DbSchemaTextError` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`pg_crud/pg_crud_common/src/db_schema_conformance.rs:11`](../pg_crud/pg_crud_common/src/db_schema_conformance.rs#L11) |
| `DbSchemaTexts` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | отдельный инвариант этого служебного типа не требуется; проверка принадлежит итоговой доменной границе | [`pg_crud/pg_crud_common/src/db_schema_conformance.rs:180`](../pg_crud/pg_crud_common/src/db_schema_conformance.rs#L180) |
| `DbStaticSchemaText` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`pg_crud/pg_crud_common/src/db_schema_conformance.rs:23`](../pg_crud/pg_crud_common/src/db_schema_conformance.rs#L23) |
| `DbStaticSchemaTexts` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | отдельный инвариант этого служебного типа не требуется; проверка принадлежит итоговой доменной границе | [`pg_crud/pg_crud_common/src/db_schema_conformance.rs:48`](../pg_crud/pg_crud_common/src/db_schema_conformance.rs#L48) |
| `DbTableNameRef` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`pg_crud/pg_crud_common/src/db_schema_conformance.rs:281`](../pg_crud/pg_crud_common/src/db_schema_conformance.rs#L281) |
| `DbTableSchema` | `trait` | `public` | behavior contract; 6 associated items; production | **Нет** | Не требуется | trait не инициализируется как значение | [`pg_crud/pg_crud_common/src/db_schema_conformance.rs:114`](../pg_crud/pg_crud_common/src/db_schema_conformance.rs#L114) |
| `DbTableSnapshot` | `struct` | `public` | named-field data structure; 2 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`pg_crud/pg_crud_common/src/db_schema_conformance.rs:339`](../pg_crud/pg_crud_common/src/db_schema_conformance.rs#L339) |
| `PgColumnSchema` | `trait` | `public` | behavior contract; 3 associated items; production | **Нет** | Не требуется | trait не инициализируется как значение | [`pg_crud/pg_crud_common/src/db_schema_conformance.rs:17`](../pg_crud/pg_crud_common/src/db_schema_conformance.rs#L17) |
| `SqlxDbSchemaInspectionError` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`pg_crud/pg_crud_common/src/db_schema_conformance.rs:365`](../pg_crud/pg_crud_common/src/db_schema_conformance.rs#L365) |
| `SqlxPgPoolRef` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`pg_crud/pg_crud_common/src/db_schema_conformance.rs:275`](../pg_crud/pg_crud_common/src/db_schema_conformance.rs#L275) |

### Модуль `pg_crud_common::errors`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `PgCrudStringWrapperTryFromStringError` | `enum` | `public` | error enum; 1 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`pg_crud/pg_crud_common/src/errors.rs:19`](../pg_crud/pg_crud_common/src/errors.rs#L19) |
| `QueryPartError` | `enum` | `public` | error enum; 3 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`pg_crud/pg_crud_common/src/errors.rs:47`](../pg_crud/pg_crud_common/src/errors.rs#L47) |
| `SqlxBoxDynError` | `struct` | `crate` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`pg_crud/pg_crud_common/src/errors.rs:1`](../pg_crud/pg_crud_common/src/errors.rs#L1) |
| `SqlxPostgresQueryBindError` | `struct` | `public` | named-field data structure; 1 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`pg_crud/pg_crud_common/src/errors.rs:13`](../pg_crud/pg_crud_common/src/errors.rs#L13) |

### Модуль `pg_crud_common::filter_bind_plan`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `FilterBindPlan` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | отдельный инвариант этого служебного типа не требуется; проверка принадлежит итоговой доменной границе | [`pg_crud/pg_crud_common/src/filter_bind_plan.rs:33`](../pg_crud/pg_crud_common/src/filter_bind_plan.rs#L33) |
| `PgFilterBindValue` | `enum` | `public` | closed variant set; 3 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`pg_crud/pg_crud_common/src/filter_bind_plan.rs:26`](../pg_crud/pg_crud_common/src/filter_bind_plan.rs#L26) |
| `PgFilterBool` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`pg_crud/pg_crud_common/src/filter_bind_plan.rs:3`](../pg_crud/pg_crud_common/src/filter_bind_plan.rs#L3) |
| `PgFilterI64` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`pg_crud/pg_crud_common/src/filter_bind_plan.rs:6`](../pg_crud/pg_crud_common/src/filter_bind_plan.rs#L6) |
| `PgFilterText` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`pg_crud/pg_crud_common/src/filter_bind_plan.rs:9`](../pg_crud/pg_crud_common/src/filter_bind_plan.rs#L9) |
| `PgFilterTextError` | `struct` | `public` | unit marker type; production | **Нет** | Не требуется | marker не содержит проверяемых данных | [`pg_crud/pg_crud_common/src/filter_bind_plan.rs:22`](../pg_crud/pg_crud_common/src/filter_bind_plan.rs#L22) |

### Модуль `pg_crud_common::finite_f64`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `FiniteF64` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`pg_crud/pg_crud_common/src/finite_f64.rs:7`](../pg_crud/pg_crud_common/src/finite_f64.rs#L7) |
| `FiniteF64Error` | `enum` | `public` | error enum; 1 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`pg_crud/pg_crud_common/src/finite_f64.rs:1`](../pg_crud/pg_crud_common/src/finite_f64.rs#L1) |
| `PositiveFiniteF64` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`pg_crud/pg_crud_common/src/finite_f64.rs:29`](../pg_crud/pg_crud_common/src/finite_f64.rs#L29) |
| `PositiveFiniteF64Error` | `enum` | `public` | error enum; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`pg_crud/pg_crud_common/src/finite_f64.rs:21`](../pg_crud/pg_crud_common/src/finite_f64.rs#L21) |
| `UnitIntervalF64` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`pg_crud/pg_crud_common/src/finite_f64.rs:54`](../pg_crud/pg_crud_common/src/finite_f64.rs#L54) |
| `UnitIntervalF64Error` | `enum` | `public` | error enum; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`pg_crud/pg_crud_common/src/finite_f64.rs:46`](../pg_crud/pg_crud_common/src/finite_f64.rs#L46) |

### Модуль `pg_crud_common::invariants`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `BulkMutationOutcome` | `enum` | `public` | closed variant set; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`pg_crud/pg_crud_common/src/invariants.rs:1`](../pg_crud/pg_crud_common/src/invariants.rs#L1) |
| `DataInvariantViolation` | `enum` | `public` | closed variant set; 5 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`pg_crud/pg_crud_common/src/invariants.rs:10`](../pg_crud/pg_crud_common/src/invariants.rs#L10) |
| `PaginationTotal` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`pg_crud/pg_crud_common/src/invariants.rs:7`](../pg_crud/pg_crud_common/src/invariants.rs#L7) |

### Модуль `pg_crud_common::list_total`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `ListItems` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | отдельный инвариант этого служебного типа не требуется; проверка принадлежит итоговой доменной границе | [`pg_crud/pg_crud_common/src/list_total.rs:51`](../pg_crud/pg_crud_common/src/list_total.rs#L51) |
| `ListOffset` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | отдельный инвариант этого служебного типа не требуется; проверка принадлежит итоговой доменной границе | [`pg_crud/pg_crud_common/src/list_total.rs:1`](../pg_crud/pg_crud_common/src/list_total.rs#L1) |
| `ListPage` | `struct` | `public` | named-field data structure; 2 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`pg_crud/pg_crud_common/src/list_total.rs:54`](../pg_crud/pg_crud_common/src/list_total.rs#L54) |
| `ListRows` | `struct` | `public` | named-field data structure; 2 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`pg_crud/pg_crud_common/src/list_total.rs:71`](../pg_crud/pg_crud_common/src/list_total.rs#L71) |
| `ListRowsPresence` | `enum` | `public` | closed variant set; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`pg_crud/pg_crud_common/src/list_total.rs:9`](../pg_crud/pg_crud_common/src/list_total.rs#L9) |
| `ListTotal` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`pg_crud/pg_crud_common/src/list_total.rs:28`](../pg_crud/pg_crud_common/src/list_total.rs#L28) |
| `ListTotalError` | `struct` | `public` | unit marker type; production | **Нет** | Не требуется | marker не содержит проверяемых данных | [`pg_crud/pg_crud_common/src/list_total.rs:31`](../pg_crud/pg_crud_common/src/list_total.rs#L31) |
| `ListTotalSource` | `enum` | `public` | closed variant set; 3 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`pg_crud/pg_crud_common/src/list_total.rs:21`](../pg_crud/pg_crud_common/src/list_total.rs#L21) |
| `WindowTotalPresence` | `enum` | `public` | closed variant set; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`pg_crud/pg_crud_common/src/list_total.rs:15`](../pg_crud/pg_crud_common/src/list_total.rs#L15) |

### Модуль `pg_crud_common::operation_budget`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `OperationBudget` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`pg_crud/pg_crud_common/src/operation_budget.rs:1`](../pg_crud/pg_crud_common/src/operation_budget.rs#L1) |
| `OperationBudgetExceeded` | `struct` | `public` | named-field data structure; 2 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`pg_crud/pg_crud_common/src/operation_budget.rs:7`](../pg_crud/pg_crud_common/src/operation_budget.rs#L7) |
| `OperationCount` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`pg_crud/pg_crud_common/src/operation_budget.rs:4`](../pg_crud/pg_crud_common/src/operation_budget.rs#L4) |

### Модуль `pg_crud_common::operational_invariants`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `PgCounterReconciliation` | `enum` | `public` | closed variant set; 3 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`pg_crud/pg_crud_common/src/operational_invariants.rs:88`](../pg_crud/pg_crud_common/src/operational_invariants.rs#L88) |
| `PgCounterValue` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`pg_crud/pg_crud_common/src/operational_invariants.rs:85`](../pg_crud/pg_crud_common/src/operational_invariants.rs#L85) |
| `PgDuplicateIdentifierPresence` | `enum` | `private` | closed variant set; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`pg_crud/pg_crud_common/src/operational_invariants.rs:30`](../pg_crud/pg_crud_common/src/operational_invariants.rs#L30) |
| `PgOperationalLimit` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`pg_crud/pg_crud_common/src/operational_invariants.rs:95`](../pg_crud/pg_crud_common/src/operational_invariants.rs#L95) |
| `PgOperationalLimitError` | `enum` | `public` | error enum; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`pg_crud/pg_crud_common/src/operational_invariants.rs:116`](../pg_crud/pg_crud_common/src/operational_invariants.rs#L116) |
| `PgOperationalLimitUpdateAuthority` | `enum` | `public` | closed variant set; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`pg_crud/pg_crud_common/src/operational_invariants.rs:110`](../pg_crud/pg_crud_common/src/operational_invariants.rs#L110) |
| `PgScopedForeignKey` | `struct` | `public` | named-field data structure; 4 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`pg_crud/pg_crud_common/src/operational_invariants.rs:22`](../pg_crud/pg_crud_common/src/operational_invariants.rs#L22) |
| `PgScopedForeignKeyClauseText` | `struct` | `private` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`pg_crud/pg_crud_common/src/operational_invariants.rs:36`](../pg_crud/pg_crud_common/src/operational_invariants.rs#L36) |
| `PgScopedForeignKeyError` | `enum` | `public` | error enum; 3 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`pg_crud/pg_crud_common/src/operational_invariants.rs:10`](../pg_crud/pg_crud_common/src/operational_invariants.rs#L10) |
| `PgScopedForeignKeyOnDelete` | `enum` | `public` | closed variant set; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`pg_crud/pg_crud_common/src/operational_invariants.rs:4`](../pg_crud/pg_crud_common/src/operational_invariants.rs#L4) |
| `PgSqlIdentifiers` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | отдельный инвариант этого служебного типа не требуется; проверка принадлежит итоговой доменной границе | [`pg_crud/pg_crud_common/src/operational_invariants.rs:19`](../pg_crud/pg_crud_common/src/operational_invariants.rs#L19) |

### Модуль `pg_crud_common::order_preserving_deduplication`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `OrderPreservingValues` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | отдельный инвариант этого служебного типа не требуется; проверка принадлежит итоговой доменной границе | [`pg_crud/pg_crud_common/src/order_preserving_deduplication.rs:7`](../pg_crud/pg_crud_common/src/order_preserving_deduplication.rs#L7) |
| `SliceOrdering` | `enum` | `public` | closed variant set; 3 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`pg_crud/pg_crud_common/src/order_preserving_deduplication.rs:1`](../pg_crud/pg_crud_common/src/order_preserving_deduplication.rs#L1) |

### Модуль `pg_crud_common::pagination`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `PaginationEnd` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | отдельный инвариант этого служебного типа не требуется; проверка принадлежит итоговой доменной границе | [`pg_crud/pg_crud_common/src/pagination.rs:98`](../pg_crud/pg_crud_common/src/pagination.rs#L98) |
| `PaginationLimit` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | отдельный инвариант этого служебного типа не требуется; проверка принадлежит итоговой доменной границе | [`pg_crud/pg_crud_common/src/pagination.rs:1`](../pg_crud/pg_crud_common/src/pagination.rs#L1) |
| `PaginationOffset` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | отдельный инвариант этого служебного типа не требуется; проверка принадлежит итоговой доменной границе | [`pg_crud/pg_crud_common/src/pagination.rs:58`](../pg_crud/pg_crud_common/src/pagination.rs#L58) |
| `PaginationPolicy` | `struct` | `public` | named-field data structure; 1 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`pg_crud/pg_crud_common/src/pagination.rs:31`](../pg_crud/pg_crud_common/src/pagination.rs#L31) |
| `PaginationStart` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | отдельный инвариант этого служебного типа не требуется; проверка принадлежит итоговой доменной границе | [`pg_crud/pg_crud_common/src/pagination.rs:88`](../pg_crud/pg_crud_common/src/pagination.rs#L88) |

### Модуль `pg_crud_common::patch_field`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `PatchField` | `enum` | `public` | closed variant set; 3 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`pg_crud/pg_crud_common/src/patch_field.rs:1`](../pg_crud/pg_crud_common/src/patch_field.rs#L1) |

### Модуль `pg_crud_common::patch_field::tests`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `Patch` | `struct` | `private` | named-field data structure; 1 fields; test-only | **Нет** | Не требуется | локальная serde-fixture; проверяется вложенный payload, не fixture-тип | [`pg_crud/pg_crud_common/src/patch_field.rs:52`](../pg_crud/pg_crud_common/src/patch_field.rs#L52) |

### Модуль `pg_crud_common::pg_error`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `PgErrorKind` | `enum` | `public` | closed variant set; 12 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`pg_crud/pg_crud_common/src/pg_error.rs:1`](../pg_crud/pg_crud_common/src/pg_error.rs#L1) |
| `SqlxPgErrorRef` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`pg_crud/pg_crud_common/src/pg_error.rs:17`](../pg_crud/pg_crud_common/src/pg_error.rs#L17) |

### Модуль `pg_crud_common::query_fragment`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `QueryPartFragment` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`pg_crud/pg_crud_common/src/query_fragment.rs:1`](../pg_crud/pg_crud_common/src/query_fragment.rs#L1) |
| `SqlColumnRef` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`pg_crud/pg_crud_common/src/query_fragment.rs:48`](../pg_crud/pg_crud_common/src/query_fragment.rs#L48) |

### Модуль `pg_crud_common::read_query_plan`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `QuerySortOrder` | `enum` | `public` | closed variant set; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`pg_crud/pg_crud_common/src/read_query_plan.rs:1`](../pg_crud/pg_crud_common/src/read_query_plan.rs#L1) |
| `ReadQueryPlan` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`pg_crud/pg_crud_common/src/read_query_plan.rs:21`](../pg_crud/pg_crud_common/src/read_query_plan.rs#L21) |
| `ReadQueryPlanError` | `struct` | `public` | unit marker type; production | **Нет** | Не требуется | marker не содержит проверяемых данных | [`pg_crud/pg_crud_common/src/read_query_plan.rs:24`](../pg_crud/pg_crud_common/src/read_query_plan.rs#L24) |
| `SqlSortOrderText` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`pg_crud/pg_crud_common/src/read_query_plan.rs:15`](../pg_crud/pg_crud_common/src/read_query_plan.rs#L15) |
| `StdReadQueryBindIndex` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`pg_crud/pg_crud_common/src/read_query_plan.rs:18`](../pg_crud/pg_crud_common/src/read_query_plan.rs#L18) |

### Модуль `pg_crud_common::rollback`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `TransactionFailure` | `enum` | `public` | closed variant set; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`pg_crud/pg_crud_common/src/rollback.rs:1`](../pg_crud/pg_crud_common/src/rollback.rs#L1) |

### Модуль `pg_crud_common::sql_identifier`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `SqlIdentifier` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | внутреннее значение может расти; нужен предел длины/размера | [`pg_crud/pg_crud_common/src/sql_identifier.rs:1`](../pg_crud/pg_crud_common/src/sql_identifier.rs#L1) |
| `SqlIdentifierError` | `enum` | `public` | error enum; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`pg_crud/pg_crud_common/src/sql_identifier.rs:16`](../pg_crud/pg_crud_common/src/sql_identifier.rs#L16) |
| `SqlIdentifiers` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | отдельный инвариант этого служебного типа не требуется; проверка принадлежит итоговой доменной границе | [`pg_crud/pg_crud_common/src/sql_identifier.rs:28`](../pg_crud/pg_crud_common/src/sql_identifier.rs#L28) |
| `SqlQualifiedIdentifier` | `struct` | `public` | named-field data structure; 2 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`pg_crud/pg_crud_common/src/sql_identifier.rs:23`](../pg_crud/pg_crud_common/src/sql_identifier.rs#L23) |
| `SqlQueryText` | `struct` | `private` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`pg_crud/pg_crud_common/src/sql_identifier.rs:30`](../pg_crud/pg_crud_common/src/sql_identifier.rs#L30) |
| `SqlSelectBuilder` | `struct` | `public` | named-field data structure; 2 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`pg_crud/pg_crud_common/src/sql_identifier.rs:68`](../pg_crud/pg_crud_common/src/sql_identifier.rs#L68) |

### Модуль `pg_crud_common::sql_like_pattern`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `SqlLikeInputRef` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`pg_crud/pg_crud_common/src/sql_like_pattern.rs:8`](../pg_crud/pg_crud_common/src/sql_like_pattern.rs#L8) |
| `SqlLikeMatchMode` | `enum` | `public` | closed variant set; 3 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`pg_crud/pg_crud_common/src/sql_like_pattern.rs:1`](../pg_crud/pg_crud_common/src/sql_like_pattern.rs#L1) |
| `SqlLikePattern` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | внутреннее значение может расти; нужен предел длины/размера | [`pg_crud/pg_crud_common/src/sql_like_pattern.rs:11`](../pg_crud/pg_crud_common/src/sql_like_pattern.rs#L11) |
| `SqlLikePatternError` | `struct` | `public` | unit marker type; production | **Нет** | Не требуется | marker не содержит проверяемых данных | [`pg_crud/pg_crud_common/src/sql_like_pattern.rs:28`](../pg_crud/pg_crud_common/src/sql_like_pattern.rs#L28) |

### Модуль `pg_crud_common::tests_not_empty_unique_vec`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `NonClone` | `struct` | `private` | single-field tuple wrapper; test-only | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`pg_crud/pg_crud_common/src/lib.rs:1484`](../pg_crud/pg_crud_common/src/lib.rs#L1484) |

## Crate `pg_crud_macros_common`

### Модуль `pg_crud_macros_common`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `DeLen` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`pg_crud/pg_crud_macros_common/src/lib.rs:217`](../pg_crud/pg_crud_macros_common/src/lib.rs#L217) |
| `DefaultSomeOneOrDefaultSomeOneWithMaxPageSize` | `enum` | `public` | closed variant set; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`pg_crud/pg_crud_macros_common/src/lib.rs:425`](../pg_crud/pg_crud_macros_common/src/lib.rs#L425) |
| `DeriveOrImpl` | `enum` | `public` | closed variant set; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`pg_crud/pg_crud_macros_common/src/lib.rs:164`](../pg_crud/pg_crud_macros_common/src/lib.rs#L164) |
| `Dimension` | `enum` | `public` | closed variant set; 4 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`pg_crud/pg_crud_macros_common/src/lib.rs:457`](../pg_crud/pg_crud_macros_common/src/lib.rs#L457) |
| `DimensionIndexNumber` | `enum` | `public` | closed variant set; 4 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`pg_crud/pg_crud_macros_common/src/lib.rs:465`](../pg_crud/pg_crud_macros_common/src/lib.rs#L465) |
| `DimensionNumber` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`pg_crud/pg_crud_macros_common/src/lib.rs:201`](../pg_crud/pg_crud_macros_common/src/lib.rs#L201) |
| `EqOperatorHandle` | `enum` | `public` | closed variant set; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`pg_crud/pg_crud_macros_common/src/lib.rs:435`](../pg_crud/pg_crud_macros_common/src/lib.rs#L435) |
| `EqOrEqUsingFields` | `enum` | `public` | closed variant set; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`pg_crud/pg_crud_macros_common/src/lib.rs:430`](../pg_crud/pg_crud_macros_common/src/lib.rs#L430) |
| `Import` | `enum` | `public` | closed variant set; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`pg_crud/pg_crud_macros_common/src/lib.rs:328`](../pg_crud/pg_crud_macros_common/src/lib.rs#L328) |
| `ImportPathStr` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`pg_crud/pg_crud_macros_common/src/lib.rs:199`](../pg_crud/pg_crud_macros_common/src/lib.rs#L199) |
| `ImportSnakeCaseStr` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`pg_crud/pg_crud_macros_common/src/lib.rs:197`](../pg_crud/pg_crud_macros_common/src/lib.rs#L197) |
| `IsNullable` | `enum` | `public` | closed variant set; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`pg_crud/pg_crud_macros_common/src/lib.rs:262`](../pg_crud/pg_crud_macros_common/src/lib.rs#L262) |
| `IsNullablePrefixStr` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | атрибут валидатора задаёт инвариант типа | [`pg_crud/pg_crud_macros_common/src/lib.rs:194`](../pg_crud/pg_crud_macros_common/src/lib.rs#L194) |
| `IsStandardNonNull` | `enum` | `public` | closed variant set; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`pg_crud/pg_crud_macros_common/src/lib.rs:257`](../pg_crud/pg_crud_macros_common/src/lib.rs#L257) |
| `NamesCtx` | `struct` | `private` | named-field data structure; 63 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`pg_crud/pg_crud_macros_common/src/lib.rs:5`](../pg_crud/pg_crud_macros_common/src/lib.rs#L5) |
| `NonNullOrNullableStr` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`pg_crud/pg_crud_macros_common/src/lib.rs:192`](../pg_crud/pg_crud_macros_common/src/lib.rs#L192) |
| `PanicUuidRef` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`pg_crud/pg_crud_macros_common/src/lib.rs:251`](../pg_crud/pg_crud_macros_common/src/lib.rs#L251) |
| `ParseErrorIdRef` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`pg_crud/pg_crud_macros_common/src/lib.rs:249`](../pg_crud/pg_crud_macros_common/src/lib.rs#L249) |
| `ParseTokenStreamStrings` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | отдельный инвариант этого служебного типа не требуется; проверка принадлежит итоговой доменной границе | [`pg_crud/pg_crud_macros_common/src/lib.rs:227`](../pg_crud/pg_crud_macros_common/src/lib.rs#L227) |
| `ProcMacro2GeneratedRustTokenStreamVec` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | отдельный инвариант этого служебного типа не требуется; проверка принадлежит итоговой доменной границе | [`pg_crud/pg_crud_macros_common/src/lib.rs:169`](../pg_crud/pg_crud_macros_common/src/lib.rs#L169) |
| `ReadOrUpdate` | `enum` | `public` | closed variant set; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`pg_crud/pg_crud_macros_common/src/lib.rs:410`](../pg_crud/pg_crud_macros_common/src/lib.rs#L410) |
| `StructElsLen` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`pg_crud/pg_crud_macros_common/src/lib.rs:209`](../pg_crud/pg_crud_macros_common/src/lib.rs#L209) |
| `SynFieldRefs` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`pg_crud/pg_crud_macros_common/src/lib.rs:255`](../pg_crud/pg_crud_macros_common/src/lib.rs#L255) |
| `SynIdentifierTypeRefs` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`pg_crud/pg_crud_macros_common/src/lib.rs:253`](../pg_crud/pg_crud_macros_common/src/lib.rs#L253) |
| `WrapIntoBraces` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`pg_crud/pg_crud_macros_common/src/lib.rs:225`](../pg_crud/pg_crud_macros_common/src/lib.rs#L225) |

### Модуль `pg_crud_macros_common::filters`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `PgFilter` | `trait` | `public` | behavior contract; 3 associated items; production | **Нет** | Не требуется | trait не инициализируется как значение | [`pg_crud/pg_crud_macros_common/src/filters.rs:138`](../pg_crud/pg_crud_macros_common/src/filters.rs#L138) |
| `PgTypeFilter` | `enum` | `public` | closed variant set; 24 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`pg_crud/pg_crud_macros_common/src/filters.rs:1`](../pg_crud/pg_crud_macros_common/src/filters.rs#L1) |

## Crate `pg_table`

### Модуль `pg_table`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `CombinationOfAppStateLogicTraits` | `trait` | `public` | behavior contract; 0 associated items; production | **Нет** | Не требуется | trait не инициализируется как значение | [`pg_crud/pg_table/src/lib.rs:3`](../pg_crud/pg_table/src/lib.rs#L3) |
| `InsertValuesFmt` | `enum` | `private` | closed variant set; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`pg_crud/pg_table/src/lib.rs:513`](../pg_crud/pg_table/src/lib.rs#L513) |
| `PgTableIdempotencyActor` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`pg_crud/pg_table/src/lib.rs:18`](../pg_crud/pg_table/src/lib.rs#L18) |
| `PgTableIdempotencyBegin` | `enum` | `public` | closed variant set; 4 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`pg_crud/pg_table/src/lib.rs:114`](../pg_crud/pg_table/src/lib.rs#L114) |
| `PgTableIdempotencyBody` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`pg_crud/pg_table/src/lib.rs:28`](../pg_crud/pg_table/src/lib.rs#L28) |
| `PgTableIdempotencyBodyError` | `struct` | `public` | unit marker type; production | **Нет** | Не требуется | marker не содержит проверяемых данных | [`pg_crud/pg_table/src/lib.rs:30`](../pg_crud/pg_table/src/lib.rs#L30) |
| `PgTableIdempotencyBodyRef` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`pg_crud/pg_table/src/lib.rs:44`](../pg_crud/pg_table/src/lib.rs#L44) |
| `PgTableIdempotencyCleanupBatchSize` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | cleanup batch size должен быть больше нуля | [`pg_crud/pg_table/src/lib.rs:52`](../pg_crud/pg_table/src/lib.rs#L52) |
| `PgTableIdempotencyCleanupRetentionSeconds` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | retention не должен быть отрицательным | [`pg_crud/pg_table/src/lib.rs:50`](../pg_crud/pg_table/src/lib.rs#L50) |
| `PgTableIdempotencyCleanupRows` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`pg_crud/pg_table/src/lib.rs:54`](../pg_crud/pg_table/src/lib.rs#L54) |
| `PgTableIdempotencyKey` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`pg_crud/pg_table/src/lib.rs:20`](../pg_crud/pg_table/src/lib.rs#L20) |
| `PgTableIdempotencyMethod` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`pg_crud/pg_table/src/lib.rs:22`](../pg_crud/pg_table/src/lib.rs#L22) |
| `PgTableIdempotencyReplay` | `struct` | `public` | named-field data structure; 2 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`pg_crud/pg_table/src/lib.rs:109`](../pg_crud/pg_table/src/lib.rs#L109) |
| `PgTableIdempotencyRequest` | `struct` | `public` | named-field data structure; 2 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`pg_crud/pg_table/src/lib.rs:104`](../pg_crud/pg_table/src/lib.rs#L104) |
| `PgTableIdempotencyRequestHash` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`pg_crud/pg_table/src/lib.rs:26`](../pg_crud/pg_table/src/lib.rs#L26) |
| `PgTableIdempotencyResponseStatus` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | сырой `u16` допускает невалидный HTTP status | [`pg_crud/pg_table/src/lib.rs:46`](../pg_crud/pg_table/src/lib.rs#L46) |
| `PgTableIdempotencyRoute` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`pg_crud/pg_table/src/lib.rs:24`](../pg_crud/pg_table/src/lib.rs#L24) |
| `PgTableIdempotencyScope` | `struct` | `public` | named-field data structure; 4 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`pg_crud/pg_table/src/lib.rs:97`](../pg_crud/pg_table/src/lib.rs#L97) |
| `PgTableIdempotencyTextBytes` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`pg_crud/pg_table/src/lib.rs:48`](../pg_crud/pg_table/src/lib.rs#L48) |
| `PgTableIdempotencyTextError` | `enum` | `public` | error enum; 4 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`pg_crud/pg_table/src/lib.rs:121`](../pg_crud/pg_table/src/lib.rs#L121) |
| `PgTableNameRef` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`pg_crud/pg_table/src/lib.rs:528`](../pg_crud/pg_table/src/lib.rs#L528) |
| `PgTableQueryPartFragment` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`pg_crud/pg_table/src/lib.rs:583`](../pg_crud/pg_table/src/lib.rs#L583) |
| `PgTableQueryString` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`pg_crud/pg_table/src/lib.rs:548`](../pg_crud/pg_table/src/lib.rs#L548) |
| `PgTableRevision` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`pg_crud/pg_table/src/lib.rs:58`](../pg_crud/pg_table/src/lib.rs#L58) |
| `PgTableRevisionTryFromStringError` | `enum` | `public` | error enum; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`pg_crud/pg_table/src/lib.rs:63`](../pg_crud/pg_table/src/lib.rs#L63) |
| `PgTableSqlFragmentRef` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`pg_crud/pg_table/src/lib.rs:538`](../pg_crud/pg_table/src/lib.rs#L538) |
| `PgTableStringWrapperTryFromStringError` | `enum` | `public` | error enum; 1 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`pg_crud/pg_table/src/lib.rs:550`](../pg_crud/pg_table/src/lib.rs#L550) |
| `SelectWhereFmt` | `enum` | `private` | closed variant set; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`pg_crud/pg_table/src/lib.rs:518`](../pg_crud/pg_table/src/lib.rs#L518) |
| `SqlxPgTableIdempotencyError` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`pg_crud/pg_table/src/lib.rs:151`](../pg_crud/pg_table/src/lib.rs#L151) |
| `SqlxPgTablePgConnectionRef` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`pg_crud/pg_table/src/lib.rs:56`](../pg_crud/pg_table/src/lib.rs#L56) |
| `StdPgTableRevisionParseIntError` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`pg_crud/pg_table/src/lib.rs:61`](../pg_crud/pg_table/src/lib.rs#L61) |
| `UpdateSelectorFmt` | `enum` | `private` | closed variant set; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`pg_crud/pg_table/src/lib.rs:523`](../pg_crud/pg_table/src/lib.rs#L523) |

## Crate `pg_types_common`

### Модуль `pg_types_common`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `IsPrimaryKey` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`pg_crud/pg_types/pg_types_common/src/lib.rs:31`](../pg_crud/pg_types/pg_types_common/src/lib.rs#L31) |
| `PaginationStartsWithOne` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`pg_crud/pg_types/pg_types_common/src/lib.rs:40`](../pg_crud/pg_types/pg_types_common/src/lib.rs#L40) |
| `PaginationStartsWithOneRaw` | `struct` | `private` | named-field data structure; 2 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`pg_crud/pg_types/pg_types_common/src/lib.rs:1`](../pg_crud/pg_types/pg_types_common/src/lib.rs#L1) |
| `PaginationStartsWithOneTryNewError` | `enum` | `public` | error enum; 3 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`pg_crud/pg_types/pg_types_common/src/lib.rs:57`](../pg_crud/pg_types/pg_types_common/src/lib.rs#L57) |
| `PaginationStartsWithOneValue` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`pg_crud/pg_types/pg_types_common/src/lib.rs:6`](../pg_crud/pg_types/pg_types_common/src/lib.rs#L6) |

## Crate `prepare_postgresql_databases`

### Модуль `prepare_postgresql_databases`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `DatabasePreparationSpec` | `struct` | `public` | named-field data structure; 2 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`prepare_postgresql_databases/src/lib.rs:23`](../prepare_postgresql_databases/src/lib.rs#L23) |
| `DatabaseUrl` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | внутреннее значение может расти; нужен предел длины/размера | [`prepare_postgresql_databases/src/lib.rs:1`](../prepare_postgresql_databases/src/lib.rs#L1) |
| `DatabaseUrlError` | `enum` | `public` | error enum; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`prepare_postgresql_databases/src/lib.rs:5`](../prepare_postgresql_databases/src/lib.rs#L5) |
| `MigrationsSource` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | внутреннее значение может расти; нужен предел длины/размера | [`prepare_postgresql_databases/src/lib.rs:13`](../prepare_postgresql_databases/src/lib.rs#L13) |
| `MigrationsSourceError` | `enum` | `public` | error enum; 1 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`prepare_postgresql_databases/src/lib.rs:17`](../prepare_postgresql_databases/src/lib.rs#L17) |
| `ProcessArgument` | `enum` | `public` | closed variant set; 3 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`prepare_postgresql_databases/src/lib.rs:45`](../prepare_postgresql_databases/src/lib.rs#L45) |
| `ProcessArguments` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | отдельный инвариант этого служебного типа не требуется; проверка принадлежит итоговой доменной границе | [`prepare_postgresql_databases/src/lib.rs:83`](../prepare_postgresql_databases/src/lib.rs#L83) |
| `ProcessCommand` | `struct` | `public` | named-field data structure; 2 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`prepare_postgresql_databases/src/lib.rs:39`](../prepare_postgresql_databases/src/lib.rs#L39) |
| `ProcessCommands` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | отдельный инвариант этого служебного типа не требуется; проверка принадлежит итоговой доменной границе | [`prepare_postgresql_databases/src/lib.rs:85`](../prepare_postgresql_databases/src/lib.rs#L85) |
| `ProcessProgram` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`prepare_postgresql_databases/src/lib.rs:88`](../prepare_postgresql_databases/src/lib.rs#L88) |
| `ProcessStaticArgument` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`prepare_postgresql_databases/src/lib.rs:52`](../prepare_postgresql_databases/src/lib.rs#L52) |

## Crate `route_validators`

### Модуль `route_validators`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `AxumHttpStatusCode` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`route_validators/src/lib.rs:7`](../route_validators/src/lib.rs#L7) |
| `GetAxumHttpStatusCode` | `trait` | `public` | behavior contract; 1 associated items; production | **Нет** | Не требуется | trait не инициализируется как значение | [`route_validators/src/lib.rs:27`](../route_validators/src/lib.rs#L27) |

### Модуль `route_validators::check_body_size`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `AxumBody` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`route_validators/src/check_body_size.rs:1`](../route_validators/src/check_body_size.rs#L1) |
| `AxumBodySizeError` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`route_validators/src/check_body_size.rs:17`](../route_validators/src/check_body_size.rs#L17) |
| `BodySizeError` | `enum` | `public` | error enum; 1 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`route_validators/src/check_body_size.rs:31`](../route_validators/src/check_body_size.rs#L31) |
| `BodySizeLimitBytes` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`route_validators/src/check_body_size.rs:3`](../route_validators/src/check_body_size.rs#L3) |
| `BytesBodyBytes` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | внутреннее значение может расти; нужен предел длины/размера | [`route_validators/src/check_body_size.rs:27`](../route_validators/src/check_body_size.rs#L27) |
| `HttpBodySizeHint` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`route_validators/src/check_body_size.rs:19`](../route_validators/src/check_body_size.rs#L19) |

### Модуль `route_validators::check_commit`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `AxumCommitToStrConversionError` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`route_validators/src/check_commit.rs:9`](../route_validators/src/check_commit.rs#L9) |
| `CommitError` | `enum` | `public` | error enum; 3 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`route_validators/src/check_commit.rs:13`](../route_validators/src/check_commit.rs#L13) |
| `CommitNotEqMessage` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`route_validators/src/check_commit.rs:3`](../route_validators/src/check_commit.rs#L3) |
| `CommitToUse` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`route_validators/src/check_commit.rs:5`](../route_validators/src/check_commit.rs#L5) |
| `EnableApiGitCommitCheck` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`route_validators/src/check_commit.rs:11`](../route_validators/src/check_commit.rs#L11) |
| `NoCommitHeaderMessage` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`route_validators/src/check_commit.rs:7`](../route_validators/src/check_commit.rs#L7) |

### Модуль `route_validators::hdr_val`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `AxumHeaderValueRef` | `struct` | `crate` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`route_validators/src/hdr_val.rs:11`](../route_validators/src/hdr_val.rs#L11) |
| `AxumHeadersRef` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`route_validators/src/hdr_val.rs:1`](../route_validators/src/hdr_val.rs#L1) |
| `HeaderStrRef` | `struct` | `crate` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`route_validators/src/hdr_val.rs:13`](../route_validators/src/hdr_val.rs#L13) |

### Модуль `route_validators::hdr_val::tests`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `TestError` | `enum` | `private` | error enum; 3 variants; test-only | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`route_validators/src/hdr_val.rs:92`](../route_validators/src/hdr_val.rs#L92) |

### Модуль `route_validators::test_hlp`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `AxumTestHeaderValue` | `struct` | `crate` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`route_validators/src/test_hlp.rs:18`](../route_validators/src/test_hlp.rs#L18) |
| `AxumTestHeaders` | `struct` | `crate` | single-field tuple wrapper; production | **Нет** | Не требуется | отдельный инвариант этого служебного типа не требуется; проверка принадлежит итоговой доменной границе | [`route_validators/src/test_hlp.rs:7`](../route_validators/src/test_hlp.rs#L7) |
| `AxumTestHeadersMutRef` | `struct` | `crate` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`route_validators/src/test_hlp.rs:10`](../route_validators/src/test_hlp.rs#L10) |
| `TestExpId` | `struct` | `crate` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`route_validators/src/test_hlp.rs:3`](../route_validators/src/test_hlp.rs#L3) |
| `TestPanicText` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`route_validators/src/test_hlp.rs:5`](../route_validators/src/test_hlp.rs#L5) |
| `TestPollCount` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`route_validators/src/test_hlp.rs:20`](../route_validators/src/test_hlp.rs#L20) |
| `TestPollLimitReached` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`route_validators/src/test_hlp.rs:22`](../route_validators/src/test_hlp.rs#L22) |

### Модуль `route_validators::tests`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `TestError` | `struct` | `private` | unit marker type; test-only | **Нет** | Не требуется | marker не содержит проверяемых данных | [`route_validators/src/lib.rs:32`](../route_validators/src/lib.rs#L32) |

## Crate `server`

### Модуль `server`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `AxumApiRoutes` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server/src/main.rs:34`](../server/src/main.rs#L34) |
| `MetricsExporterPrometheusBuildError` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server/src/main.rs:7`](../server/src/main.rs#L7) |
| `MetricsExporterPrometheusHandle` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server/src/main.rs:9`](../server/src/main.rs#L9) |
| `RunServerError` | `enum` | `private` | error enum; 15 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server/src/main.rs:52`](../server/src/main.rs#L52) |
| `ServerAdminAuthSvcStateBuildError` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server/src/main.rs:29`](../server/src/main.rs#L29) |
| `ServerAdminCleanupCfgError` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server/src/main.rs:17`](../server/src/main.rs#L17) |
| `ServerAdminMigrateError` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server/src/main.rs:26`](../server/src/main.rs#L26) |
| `ServerConfigError` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server/src/main.rs:20`](../server/src/main.rs#L20) |
| `ServerRuntimeBackgroundTaskShutdownError` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server/src/main.rs:15`](../server/src/main.rs#L15) |
| `ServerRuntimeContentSecurityPolicyError` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server/src/main.rs:32`](../server/src/main.rs#L32) |
| `ServerRuntimeRequestTimeoutError` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server/src/main.rs:11`](../server/src/main.rs#L11) |
| `ServerRuntimeRunIntervalError` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server/src/main.rs:13`](../server/src/main.rs#L13) |
| `ServerRuntimeServeError` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server/src/main.rs:5`](../server/src/main.rs#L5) |
| `SqlxServerPgConnectError` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server/src/main.rs:23`](../server/src/main.rs#L23) |
| `StdServerExitCode` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server/src/main.rs:45`](../server/src/main.rs#L45) |
| `StdServerIoError` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server/src/main.rs:2`](../server/src/main.rs#L2) |
| `StdSharedServerAppState` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server/src/main.rs:36`](../server/src/main.rs#L36) |
| `TokioServerRuntime` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server/src/main.rs:43`](../server/src/main.rs#L43) |

## Crate `server_admin`

### Модуль `server_admin`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `AdminAccessClaims` | `struct` | `public` | named-field data structure; 6 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_admin/src/lib.rs:292`](../server_admin/src/lib.rs#L292) |
| `AdminAccessTokenError` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_admin/src/lib.rs:344`](../server_admin/src/lib.rs#L344) |
| `AdminAuditAction` | `enum` | `public` | closed variant set; 6 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_admin/src/lib.rs:367`](../server_admin/src/lib.rs#L367) |
| `AdminAuditResource` | `enum` | `public` | closed variant set; 6 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_admin/src/lib.rs:387`](../server_admin/src/lib.rs#L387) |
| `AdminAuthCollectionError` | `struct` | `crate` | unit marker type; production | **Нет** | Не требуется | marker не содержит проверяемых данных | [`server_admin/src/lib.rs:35`](../server_admin/src/lib.rs#L35) |
| `AdminBootstrapError` | `enum` | `public` | error enum; 5 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_admin/src/lib.rs:556`](../server_admin/src/lib.rs#L556) |
| `AdminCleanupBatchSize` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`server_admin/src/lib.rs:423`](../server_admin/src/lib.rs#L423) |
| `AdminCleanupCfg` | `struct` | `public` | named-field data structure; 6 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_admin/src/lib.rs:427`](../server_admin/src/lib.rs#L427) |
| `AdminCleanupCfgError` | `enum` | `public` | error enum; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_admin/src/lib.rs:458`](../server_admin/src/lib.rs#L458) |
| `AdminCleanupError` | `enum` | `public` | error enum; 3 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_admin/src/lib.rs:475`](../server_admin/src/lib.rs#L475) |
| `AdminCleanupReport` | `struct` | `public` | named-field data structure; 6 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_admin/src/lib.rs:436`](../server_admin/src/lib.rs#L436) |
| `AdminCleanupRetentionSeconds` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`server_admin/src/lib.rs:425`](../server_admin/src/lib.rs#L425) |
| `AdminCleanupRows` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_admin/src/lib.rs:445`](../server_admin/src/lib.rs#L445) |
| `AdminCookieKind` | `enum` | `public` | closed variant set; 3 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_admin/src/lib.rs:203`](../server_admin/src/lib.rs#L203) |
| `AdminCookieMaxAgeSeconds` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_admin/src/lib.rs:194`](../server_admin/src/lib.rs#L194) |
| `AdminCookieSecure` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_admin/src/lib.rs:192`](../server_admin/src/lib.rs#L192) |
| `AdminGeneratedToken` | `struct` | `public` | named-field data structure; 2 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_admin/src/lib.rs:169`](../server_admin/src/lib.rs#L169) |
| `AdminJwtSecret` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_admin/src/lib.rs:125`](../server_admin/src/lib.rs#L125) |
| `AdminMigrateError` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_admin/src/lib.rs:416`](../server_admin/src/lib.rs#L416) |
| `AdminMigrateErrorInner` | `enum` | `private` | closed variant set; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_admin/src/lib.rs:409`](../server_admin/src/lib.rs#L409) |
| `AdminOpaqueToken` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_admin/src/lib.rs:133`](../server_admin/src/lib.rs#L133) |
| `AdminPassword` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`server_admin/src/lib.rs:68`](../server_admin/src/lib.rs#L68) |
| `AdminPasswordHash` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_admin/src/lib.rs:117`](../server_admin/src/lib.rs#L117) |
| `AdminPasswordHashConcurrency` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_admin/src/lib.rs:272`](../server_admin/src/lib.rs#L272) |
| `AdminPasswordHashError` | `enum` | `public` | error enum; 3 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_admin/src/lib.rs:329`](../server_admin/src/lib.rs#L329) |
| `AdminPasswordHasher` | `struct` | `public` | named-field data structure; 1 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_admin/src/lib.rs:338`](../server_admin/src/lib.rs#L338) |
| `AdminPasswordTryFromStringError` | `enum` | `public` | error enum; 1 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_admin/src/lib.rs:71`](../server_admin/src/lib.rs#L71) |
| `AdminPermissions` | `struct` | `crate` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`server_admin/src/lib.rs:25`](../server_admin/src/lib.rs#L25) |
| `AdminRefreshToken` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_admin/src/lib.rs:141`](../server_admin/src/lib.rs#L141) |
| `AdminRoleNames` | `struct` | `crate` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`server_admin/src/lib.rs:30`](../server_admin/src/lib.rs#L30) |
| `AdminSessionId` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_admin/src/lib.rs:279`](../server_admin/src/lib.rs#L279) |
| `AdminTokenHash` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_admin/src/lib.rs:153`](../server_admin/src/lib.rs#L153) |
| `AdminUnixTokenStream` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_admin/src/lib.rs:274`](../server_admin/src/lib.rs#L274) |
| `Argon2AdminPasswordHashError` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_admin/src/lib.rs:64`](../server_admin/src/lib.rs#L64) |
| `HttpAdminHeaderMapRef` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_admin/src/lib.rs:201`](../server_admin/src/lib.rs#L201) |
| `JsonwebtokenAdminError` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_admin/src/lib.rs:342`](../server_admin/src/lib.rs#L342) |
| `SqlxAdminError` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_admin/src/lib.rs:66`](../server_admin/src/lib.rs#L66) |
| `SqlxAdminMigrateError` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_admin/src/lib.rs:407`](../server_admin/src/lib.rs#L407) |
| `StdAdminAccessToken` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | атрибут валидатора задаёт инвариант типа | [`server_admin/src/lib.rs:348`](../server_admin/src/lib.rs#L348) |
| `StdAdminCookie` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | атрибут валидатора задаёт инвариант типа | [`server_admin/src/lib.rs:196`](../server_admin/src/lib.rs#L196) |
| `StdAdminSharedSemaphore` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_admin/src/lib.rs:58`](../server_admin/src/lib.rs#L58) |
| `TokioAdminAcquireError` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_admin/src/lib.rs:62`](../server_admin/src/lib.rs#L62) |
| `TokioAdminJoinError` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_admin/src/lib.rs:60`](../server_admin/src/lib.rs#L60) |

### Модуль `server_admin::auth`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `AdminApiError` | `enum` | `public` | error enum; 12 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_admin/src/auth.rs:538`](../server_admin/src/auth.rs#L538) |
| `AdminAuditQuery` | `struct` | `public` | named-field data structure; 12 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_admin/src/auth.rs:149`](../server_admin/src/auth.rs#L149) |
| `AdminAuditQueryParts` | `struct` | `crate` | named-field data structure; 12 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_admin/src/auth.rs:169`](../server_admin/src/auth.rs#L169) |
| `AdminAuditResourceId` | `enum` | `private` | closed variant set; 4 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_admin/src/auth.rs:640`](../server_admin/src/auth.rs#L640) |
| `AdminAuditSuccessRef` | `struct` | `private` | named-field data structure; 5 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_admin/src/auth.rs:632`](../server_admin/src/auth.rs#L632) |
| `AdminAuthPolicy` | `struct` | `public` | named-field data structure; 11 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_admin/src/auth.rs:23`](../server_admin/src/auth.rs#L23) |
| `AdminAuthReq` | `struct` | `public` | named-field data structure; 3 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_admin/src/auth.rs:207`](../server_admin/src/auth.rs#L207) |
| `AdminAuthSvcState` | `struct` | `public` | named-field data structure; 12 fields; production | **Да** | Выполнено | fallible-конструктор показывает наличие инварианта | [`server_admin/src/auth.rs:58`](../server_admin/src/auth.rs#L58) |
| `AdminAuthSvcStateBuildError` | `enum` | `public` | error enum; 3 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_admin/src/auth.rs:75`](../server_admin/src/auth.rs#L75) |
| `AdminHtmlSwaggerEnabled` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_admin/src/auth.rs:1043`](../server_admin/src/auth.rs#L1043) |
| `AdminPeerAddr` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_admin/src/auth.rs:213`](../server_admin/src/auth.rs#L213) |
| `AdminSessionBundle` | `struct` | `public` | named-field data structure; 4 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_admin/src/auth.rs:1115`](../server_admin/src/auth.rs#L1115) |
| `AdminSessionError` | `enum` | `public` | error enum; 3 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_admin/src/auth.rs:1140`](../server_admin/src/auth.rs#L1140) |
| `AdminSessionPath` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_admin/src/auth.rs:248`](../server_admin/src/auth.rs#L248) |
| `AdminSignInJson` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_admin/src/auth.rs:238`](../server_admin/src/auth.rs#L238) |
| `AuthenticatedAdmin` | `struct` | `public` | named-field data structure; 6 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_admin/src/auth.rs:99`](../server_admin/src/auth.rs#L99) |
| `AxumAdminAuthRouter` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_admin/src/auth.rs:1022`](../server_admin/src/auth.rs#L1022) |
| `AxumAdminForm` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_admin/src/auth.rs:242`](../server_admin/src/auth.rs#L242) |
| `AxumAdminJson` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_admin/src/auth.rs:240`](../server_admin/src/auth.rs#L240) |
| `AxumAdminPath` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_admin/src/auth.rs:244`](../server_admin/src/auth.rs#L244) |
| `AxumAdminQuery` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_admin/src/auth.rs:246`](../server_admin/src/auth.rs#L246) |
| `AxumAdminResponse` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_admin/src/auth.rs:575`](../server_admin/src/auth.rs#L575) |
| `HttpAdminHeaderMap` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | отдельный инвариант этого служебного типа не требуется; проверка принадлежит итоговой доменной границе | [`server_admin/src/auth.rs:205`](../server_admin/src/auth.rs#L205) |
| `HttpAdminHeaderValueError` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_admin/src/auth.rs:536`](../server_admin/src/auth.rs#L536) |
| `JsonwebtokenAdminDecodingKey` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_admin/src/auth.rs:5`](../server_admin/src/auth.rs#L5) |
| `JsonwebtokenAdminDecodingKeys` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | отдельный инвариант этого служебного типа не требуется; проверка принадлежит итоговой доменной границе | [`server_admin/src/auth.rs:7`](../server_admin/src/auth.rs#L7) |
| `JsonwebtokenAdminEncodingKey` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_admin/src/auth.rs:3`](../server_admin/src/auth.rs#L3) |
| `SqlxAdminPgConnectionRef` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_admin/src/auth.rs:663`](../server_admin/src/auth.rs#L663) |
| `StdAdminAccessTtlSeconds` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | TTL должен быть больше нуля; raw `u64` этого не гарантирует | [`server_admin/src/auth.rs:9`](../server_admin/src/auth.rs#L9) |
| `StdAdminFailureDelayMillis` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_admin/src/auth.rs:17`](../server_admin/src/auth.rs#L17) |
| `StdAdminFailureThreshold` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | failure threshold должен быть больше нуля | [`server_admin/src/auth.rs:15`](../server_admin/src/auth.rs#L15) |
| `StdAdminRateLimitCount` | `struct` | `crate` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_admin/src/auth.rs:19`](../server_admin/src/auth.rs#L19) |
| `StdAdminRateLimitWindowSeconds` | `struct` | `crate` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_admin/src/auth.rs:21`](../server_admin/src/auth.rs#L21) |
| `StdAdminRefreshTtlSeconds` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | TTL должен быть больше нуля; raw `u64` этого не гарантирует | [`server_admin/src/auth.rs:11`](../server_admin/src/auth.rs#L11) |
| `StdAdminSessionLimit` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | session limit должен быть больше нуля | [`server_admin/src/auth.rs:13`](../server_admin/src/auth.rs#L13) |
| `StdSharedAdminAuthSvcState` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_admin/src/auth.rs:73`](../server_admin/src/auth.rs#L73) |
| `UtoipaAdminAuthOpenApi` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_admin/src/auth.rs:1024`](../server_admin/src/auth.rs#L1024) |

### Модуль `server_admin::auth::html`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `AdminHtmlFormKey` | `struct` | `private` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`server_admin/src/auth/html.rs:116`](../server_admin/src/auth/html.rs#L116) |
| `AdminHtmlFormKeyError` | `struct` | `private` | unit marker type; production | **Нет** | Не требуется | marker не содержит проверяемых данных | [`server_admin/src/auth/html.rs:98`](../server_admin/src/auth/html.rs#L98) |
| `AdminHtmlFormText` | `struct` | `private` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`server_admin/src/auth/html.rs:105`](../server_admin/src/auth/html.rs#L105) |
| `AdminHtmlFormTextError` | `struct` | `private` | unit marker type; production | **Нет** | Не требуется | marker не содержит проверяемых данных | [`server_admin/src/auth/html.rs:95`](../server_admin/src/auth/html.rs#L95) |
| `ChangePasswordForm` | `struct` | `private` | named-field data structure; 2 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_admin/src/auth/html.rs:14`](../server_admin/src/auth/html.rs#L14) |
| `CreateRoleForm` | `struct` | `private` | named-field data structure; 1 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_admin/src/auth/html.rs:67`](../server_admin/src/auth/html.rs#L67) |
| `CreateUserForm` | `struct` | `private` | named-field data structure; 3 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_admin/src/auth/html.rs:28`](../server_admin/src/auth/html.rs#L28) |
| `RevokeSessionForm` | `struct` | `private` | named-field data structure; 2 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_admin/src/auth/html.rs:21`](../server_admin/src/auth/html.rs#L21) |
| `RoleIdForm` | `struct` | `private` | named-field data structure; 2 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_admin/src/auth/html.rs:78`](../server_admin/src/auth/html.rs#L78) |
| `RolePermissionsForm` | `struct` | `private` | named-field data structure; 3 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_admin/src/auth/html.rs:84`](../server_admin/src/auth/html.rs#L84) |
| `SettingsForm` | `struct` | `private` | named-field data structure; 8 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_admin/src/auth/html.rs:143`](../server_admin/src/auth/html.rs#L143) |
| `SignInForm` | `struct` | `private` | named-field data structure; 2 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_admin/src/auth/html.rs:7`](../server_admin/src/auth/html.rs#L7) |
| `StdAdminHtmlSelected` | `struct` | `private` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`server_admin/src/auth/html.rs:127`](../server_admin/src/auth/html.rs#L127) |
| `StdAdminHtmlSelectedError` | `struct` | `private` | unit marker type; production | **Нет** | Не требуется | marker не содержит проверяемых данных | [`server_admin/src/auth/html.rs:101`](../server_admin/src/auth/html.rs#L101) |
| `UpdateRoleForm` | `struct` | `private` | named-field data structure; 2 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_admin/src/auth/html.rs:72`](../server_admin/src/auth/html.rs#L72) |
| `UpdateUserForm` | `struct` | `private` | named-field data structure; 3 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_admin/src/auth/html.rs:35`](../server_admin/src/auth/html.rs#L35) |
| `UserBanForm` | `struct` | `private` | named-field data structure; 2 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_admin/src/auth/html.rs:48`](../server_admin/src/auth/html.rs#L48) |
| `UserIdForm` | `struct` | `private` | named-field data structure; 2 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_admin/src/auth/html.rs:54`](../server_admin/src/auth/html.rs#L54) |
| `UserPasswordForm` | `struct` | `private` | named-field data structure; 2 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_admin/src/auth/html.rs:42`](../server_admin/src/auth/html.rs#L42) |
| `UserRolesForm` | `struct` | `private` | named-field data structure; 3 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_admin/src/auth/html.rs:60`](../server_admin/src/auth/html.rs#L60) |

### Модуль `server_admin::auth::rate_limit`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `AdminRateLimitScope` | `enum` | `restricted` | closed variant set; 5 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_admin/src/auth/rate_limit.rs:2`](../server_admin/src/auth/rate_limit.rs#L2) |

### Модуль `server_admin::auth::routes`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `AdminAuthRouteRegistry` | `struct` | `private` | unit marker type; production | **Нет** | Не требуется | marker не содержит проверяемых данных | [`server_admin/src/auth/routes.rs:2`](../server_admin/src/auth/routes.rs#L2) |

### Модуль `server_admin::auth::session`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `SessionRefresh` | `enum` | `private` | closed variant set; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_admin/src/auth/session.rs:46`](../server_admin/src/auth/session.rs#L46) |

### Модуль `server_admin::domain`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `AdminAuditLogId` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | database identity должен быть положительным | [`server_admin/src/domain.rs:169`](../server_admin/src/domain.rs#L169) |
| `AdminAuditResourceValue` | `enum` | `restricted` | closed variant set; 4 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_admin/src/domain.rs:23`](../server_admin/src/domain.rs#L23) |
| `AdminPermissionId` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | database identity должен быть положительным | [`server_admin/src/domain.rs:147`](../server_admin/src/domain.rs#L147) |
| `AdminPermissionName` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_admin/src/domain.rs:179`](../server_admin/src/domain.rs#L179) |
| `AdminRoleId` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | database identity должен быть положительным | [`server_admin/src/domain.rs:125`](../server_admin/src/domain.rs#L125) |
| `AdminUserId` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | database identity должен быть положительным | [`server_admin/src/domain.rs:103`](../server_admin/src/domain.rs#L103) |
| `SecrecyAdminString` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | внутреннее значение может расти; нужен предел длины/размера | [`server_admin/src/domain.rs:2`](../server_admin/src/domain.rs#L2) |
| `StdAdminBool` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_admin/src/domain.rs:49`](../server_admin/src/domain.rs#L49) |
| `StdAdminNonZeroUsize` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_admin/src/domain.rs:60`](../server_admin/src/domain.rs#L60) |
| `StdAdminSocketAddr` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_admin/src/domain.rs:95`](../server_admin/src/domain.rs#L95) |
| `StdAdminStrRef` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_admin/src/domain.rs:39`](../server_admin/src/domain.rs#L39) |
| `StdAdminString` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | атрибут валидатора задаёт инвариант типа | [`server_admin/src/domain.rs:10`](../server_admin/src/domain.rs#L10) |
| `UuidAdminValue` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_admin/src/domain.rs:68`](../server_admin/src/domain.rs#L68) |

### Модуль `server_admin::generated_auth`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `AdminGeneratedAuthLayer` | `struct` | `public` | named-field data structure; 1 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_admin/src/generated_auth.rs:1`](../server_admin/src/generated_auth.rs#L1) |
| `AdminGeneratedAuthService` | `struct` | `public` | named-field data structure; 2 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_admin/src/generated_auth.rs:10`](../server_admin/src/generated_auth.rs#L10) |

### Модуль `server_admin::generated_tables`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `AdminPermissions` | `struct` | `public` | named-field data structure; 3 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_admin/src/generated_tables.rs:122`](../server_admin/src/generated_tables.rs#L122) |
| `AdminRolePermissions` | `struct` | `public` | named-field data structure; 4 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_admin/src/generated_tables.rs:72`](../server_admin/src/generated_tables.rs#L72) |
| `AdminRoles` | `struct` | `public` | named-field data structure; 5 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_admin/src/generated_tables.rs:97`](../server_admin/src/generated_tables.rs#L97) |
| `AdminSystemSettings` | `struct` | `public` | named-field data structure; 10 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_admin/src/generated_tables.rs:142`](../server_admin/src/generated_tables.rs#L142) |
| `AdminUserRoles` | `struct` | `public` | named-field data structure; 4 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_admin/src/generated_tables.rs:47`](../server_admin/src/generated_tables.rs#L47) |
| `AdminUsers` | `struct` | `public` | named-field data structure; 7 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_admin/src/generated_tables.rs:2`](../server_admin/src/generated_tables.rs#L2) |
| `UtoipaAdminOpenApi` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_admin/src/generated_tables.rs:170`](../server_admin/src/generated_tables.rs#L170) |

### Модуль `server_admin::repository`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `AdminAuthenticatedRecord` | `struct` | `crate` | named-field data structure; 4 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_admin/src/repository.rs:50`](../server_admin/src/repository.rs#L50) |
| `AdminCleanupRepositoryReport` | `struct` | `crate` | named-field data structure; 5 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_admin/src/repository.rs:57`](../server_admin/src/repository.rs#L57) |
| `AdminPageTotalCount` | `struct` | `crate` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_admin/src/repository.rs:111`](../server_admin/src/repository.rs#L111) |
| `AdminRateLimitOutcome` | `enum` | `crate` | closed variant set; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_admin/src/repository.rs:36`](../server_admin/src/repository.rs#L36) |
| `AdminRateLimitRepositoryError` | `enum` | `crate` | error enum; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_admin/src/repository.rs:41`](../server_admin/src/repository.rs#L41) |
| `AdminRecentLoginFailureCount` | `struct` | `crate` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_admin/src/repository.rs:109`](../server_admin/src/repository.rs#L109) |
| `AdminRepositoryDbRef` | `enum` | `crate` | closed variant set; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_admin/src/repository.rs:46`](../server_admin/src/repository.rs#L46) |
| `AdminRepositoryError` | `enum` | `crate` | error enum; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_admin/src/repository.rs:13`](../server_admin/src/repository.rs#L13) |
| `AdminSignInUser` | `struct` | `crate` | named-field data structure; 3 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_admin/src/repository.rs:133`](../server_admin/src/repository.rs#L133) |
| `ReplaceRolePermissionsOutcome` | `enum` | `crate` | closed variant set; 5 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_admin/src/repository.rs:20`](../server_admin/src/repository.rs#L20) |
| `ReplaceUserRolesOutcome` | `enum` | `crate` | closed variant set; 5 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_admin/src/repository.rs:28`](../server_admin/src/repository.rs#L28) |
| `SqlxAdminRepositoryConnectionMutRef` | `struct` | `crate` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_admin/src/repository.rs:102`](../server_admin/src/repository.rs#L102) |
| `SqlxAdminRepositoryPoolRef` | `struct` | `crate` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_admin/src/repository.rs:107`](../server_admin/src/repository.rs#L107) |

### Модуль `server_admin::repository::data_tables`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `DataFlt` | `enum` | `private` | closed variant set; 6 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_admin/src/repository/data_tables.rs:101`](../server_admin/src/repository/data_tables.rs#L101) |
| `DataFltJson` | `struct` | `private` | single-field tuple wrapper; production | **Да** | Выполнено | атрибут валидатора задаёт инвариант типа | [`server_admin/src/repository/data_tables.rs:97`](../server_admin/src/repository/data_tables.rs#L97) |
| `DataPermissionsFlt` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_admin/src/repository/data_tables.rs:81`](../server_admin/src/repository/data_tables.rs#L81) |
| `DataRolePermissionsFlt` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_admin/src/repository/data_tables.rs:83`](../server_admin/src/repository/data_tables.rs#L83) |
| `DataRolesFlt` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_admin/src/repository/data_tables.rs:87`](../server_admin/src/repository/data_tables.rs#L87) |
| `DataSystemSettingsFlt` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_admin/src/repository/data_tables.rs:89`](../server_admin/src/repository/data_tables.rs#L89) |
| `DataUserRolesFlt` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_admin/src/repository/data_tables.rs:93`](../server_admin/src/repository/data_tables.rs#L93) |
| `DataUsersFlt` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_admin/src/repository/data_tables.rs:95`](../server_admin/src/repository/data_tables.rs#L95) |

### Модуль `server_admin::repository::roles`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `AdminActiveAdministratorCount` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_admin/src/repository/roles.rs:3`](../server_admin/src/repository/roles.rs#L3) |
| `LastAdminState` | `struct` | `crate` | named-field data structure; 2 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_admin/src/repository/roles.rs:6`](../server_admin/src/repository/roles.rs#L6) |

### Модуль `server_admin::tests::admin_api`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `AdminHtmlSettingsTestValues` | `struct` | `private` | named-field data structure; 8 fields; test-only | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_admin/tests/admin_api.rs:37`](../server_admin/tests/admin_api.rs#L37) |
| `AdminHtmlTestBody` | `struct` | `private` | single-field tuple wrapper; test-only | **Да** | Выполнено | атрибут валидатора задаёт инвариант типа | [`server_admin/tests/admin_api.rs:24`](../server_admin/tests/admin_api.rs#L24) |
| `AdminHtmlTestFixture` | `struct` | `private` | named-field data structure; 5 fields; test-only | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_admin/tests/admin_api.rs:30`](../server_admin/tests/admin_api.rs#L30) |
| `AdminHtmlTestFormBody` | `struct` | `private` | single-field tuple wrapper; test-only | **Да** | Выполнено | атрибут валидатора задаёт инвариант типа | [`server_admin/tests/admin_api.rs:27`](../server_admin/tests/admin_api.rs#L27) |
| `AxumAdminApiTestRouter` | `struct` | `private` | single-field tuple wrapper; test-only | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_admin/tests/admin_api.rs:6`](../server_admin/tests/admin_api.rs#L6) |
| `HttpAdminApiTestMethod` | `struct` | `private` | single-field tuple wrapper; test-only | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_admin/tests/admin_api.rs:12`](../server_admin/tests/admin_api.rs#L12) |
| `HttpAdminApiTestRequest` | `struct` | `private` | single-field tuple wrapper; test-only | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_admin/tests/admin_api.rs:14`](../server_admin/tests/admin_api.rs#L14) |
| `HttpAdminApiTestResponseRef` | `struct` | `private` | single-field tuple wrapper; test-only | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_admin/tests/admin_api.rs:18`](../server_admin/tests/admin_api.rs#L18) |
| `HttpAdminHtmlTestResponse` | `struct` | `private` | single-field tuple wrapper; test-only | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_admin/tests/admin_api.rs:16`](../server_admin/tests/admin_api.rs#L16) |
| `SqlxAdminApiTestPool` | `struct` | `private` | single-field tuple wrapper; test-only | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_admin/tests/admin_api.rs:8`](../server_admin/tests/admin_api.rs#L8) |
| `SqlxAdminHtmlTestTransaction` | `struct` | `private` | single-field tuple wrapper; test-only | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_admin/tests/admin_api.rs:10`](../server_admin/tests/admin_api.rs#L10) |
| `StdAdminApiTestCookie` | `struct` | `private` | single-field tuple wrapper; test-only | **Да** | Выполнено | атрибут валидатора задаёт инвариант типа | [`server_admin/tests/admin_api.rs:20`](../server_admin/tests/admin_api.rs#L20) |
| `StdAdminApiTestStrRef` | `struct` | `private` | single-field tuple wrapper; test-only | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_admin/tests/admin_api.rs:4`](../server_admin/tests/admin_api.rs#L4) |

## Crate `server_admin_contract`

### Модуль `server_admin_contract`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `AdminApiBodyMaxBytes` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_admin_contract/src/lib.rs:11`](../server_admin_contract/src/lib.rs#L11) |
| `AdminApiErrorBody` | `struct` | `public` | named-field data structure; 1 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_admin_contract/src/lib.rs:2466`](../server_admin_contract/src/lib.rs#L2466) |
| `AdminApiErrorCode` | `enum` | `public` | closed variant set; 7 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_admin_contract/src/lib.rs:2440`](../server_admin_contract/src/lib.rs#L2440) |
| `AdminAuditCursor` | `struct` | `public` | named-field data structure; 2 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_admin_contract/src/lib.rs:1923`](../server_admin_contract/src/lib.rs#L1923) |
| `AdminAuditDetailsBytes` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_admin_contract/src/lib.rs:373`](../server_admin_contract/src/lib.rs#L373) |
| `AdminAuditDetailsTooLarge` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_admin_contract/src/lib.rs:375`](../server_admin_contract/src/lib.rs#L375) |
| `AdminAuditExport` | `struct` | `public` | named-field data structure; 1 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_admin_contract/src/lib.rs:2171`](../server_admin_contract/src/lib.rs#L2171) |
| `AdminAuditExportCsv` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | атрибут валидатора задаёт инвариант типа | [`server_admin_contract/src/lib.rs:2162`](../server_admin_contract/src/lib.rs#L2162) |
| `AdminAuditExportRoute` | `struct` | `public` | unit marker type; production | **Нет** | Не требуется | marker не содержит проверяемых данных | [`server_admin_contract/src/lib.rs:2696`](../server_admin_contract/src/lib.rs#L2696) |
| `AdminAuditLogId` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | wire database identity должен быть положительным | [`server_admin_contract/src/lib.rs:619`](../server_admin_contract/src/lib.rs#L619) |
| `AdminAuditLogRoute` | `struct` | `public` | unit marker type; production | **Нет** | Не требуется | marker не содержит проверяемых данных | [`server_admin_contract/src/lib.rs:2692`](../server_admin_contract/src/lib.rs#L2692) |
| `AdminAuditPage` | `struct` | `public` | named-field data structure; 3 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_admin_contract/src/lib.rs:1942`](../server_admin_contract/src/lib.rs#L1942) |
| `AdminAuditTimestamp` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | атрибут валидатора задаёт инвариант типа | [`server_admin_contract/src/lib.rs:354`](../server_admin_contract/src/lib.rs#L354) |
| `AdminAuditView` | `struct` | `public` | named-field data structure; 9 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_admin_contract/src/lib.rs:1849`](../server_admin_contract/src/lib.rs#L1849) |
| `AdminAuditViews` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`server_admin_contract/src/lib.rs:1222`](../server_admin_contract/src/lib.rs#L1222) |
| `AdminBool` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_admin_contract/src/lib.rs:634`](../server_admin_contract/src/lib.rs#L634) |
| `AdminBoundedVec` | `struct` | `private` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`server_admin_contract/src/lib.rs:1017`](../server_admin_contract/src/lib.rs#L1017) |
| `AdminBrandingRoute` | `struct` | `public` | unit marker type; production | **Нет** | Не требуется | marker не содержит проверяемых данных | [`server_admin_contract/src/lib.rs:2700`](../server_admin_contract/src/lib.rs#L2700) |
| `AdminBrandingView` | `struct` | `public` | named-field data structure; 6 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_admin_contract/src/lib.rs:2279`](../server_admin_contract/src/lib.rs#L2279) |
| `AdminChangeOwnPasswordReq` | `struct` | `public` | named-field data structure; 2 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_admin_contract/src/lib.rs:1530`](../server_admin_contract/src/lib.rs#L1530) |
| `AdminChangeOwnPasswordRoute` | `struct` | `public` | unit marker type; production | **Нет** | Не требуется | marker не содержит проверяемых данных | [`server_admin_contract/src/lib.rs:2620`](../server_admin_contract/src/lib.rs#L2620) |
| `AdminCollectionError` | `enum` | `public` | error enum; 1 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_admin_contract/src/lib.rs:1004`](../server_admin_contract/src/lib.rs#L1004) |
| `AdminCreateRoleReq` | `struct` | `public` | named-field data structure; 1 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_admin_contract/src/lib.rs:1564`](../server_admin_contract/src/lib.rs#L1564) |
| `AdminCreateRoleRes` | `struct` | `public` | named-field data structure; 1 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_admin_contract/src/lib.rs:1579`](../server_admin_contract/src/lib.rs#L1579) |
| `AdminCreateRoleRoute` | `struct` | `public` | unit marker type; production | **Нет** | Не требуется | marker не содержит проверяемых данных | [`server_admin_contract/src/lib.rs:2672`](../server_admin_contract/src/lib.rs#L2672) |
| `AdminCreateUserReq` | `struct` | `public` | named-field data structure; 3 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_admin_contract/src/lib.rs:1461`](../server_admin_contract/src/lib.rs#L1461) |
| `AdminCreateUserRes` | `struct` | `public` | named-field data structure; 1 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_admin_contract/src/lib.rs:1486`](../server_admin_contract/src/lib.rs#L1486) |
| `AdminCreateUserRoute` | `struct` | `public` | unit marker type; production | **Нет** | Не требуется | marker не содержит проверяемых данных | [`server_admin_contract/src/lib.rs:2644`](../server_admin_contract/src/lib.rs#L2644) |
| `AdminDataColumn` | `struct` | `public` | named-field data structure; 4 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_admin_contract/src/lib.rs:1950`](../server_admin_contract/src/lib.rs#L1950) |
| `AdminDataColumns` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`server_admin_contract/src/lib.rs:2074`](../server_admin_contract/src/lib.rs#L2074) |
| `AdminDataFilter` | `struct` | `public` | named-field data structure; 2 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_admin_contract/src/lib.rs:1989`](../server_admin_contract/src/lib.rs#L1989) |
| `AdminDataFilters` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`server_admin_contract/src/lib.rs:2028`](../server_admin_contract/src/lib.rs#L2028) |
| `AdminDataInputKind` | `enum` | `public` | closed variant set; 7 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_admin_contract/src/lib.rs:2048`](../server_admin_contract/src/lib.rs#L2048) |
| `AdminDataRow` | `struct` | `public` | named-field data structure; 1 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_admin_contract/src/lib.rs:2094`](../server_admin_contract/src/lib.rs#L2094) |
| `AdminDataRows` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`server_admin_contract/src/lib.rs:1266`](../server_admin_contract/src/lib.rs#L1266) |
| `AdminDataTable` | `enum` | `public` | closed variant set; 12 variants; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`server_admin_contract/src/lib.rs:247`](../server_admin_contract/src/lib.rs#L247) |
| `AdminDataTableCatalog` | `struct` | `public` | named-field data structure; 1 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_admin_contract/src/lib.rs:2148`](../server_admin_contract/src/lib.rs#L2148) |
| `AdminDataTableFilterQuery` | `struct` | `public` | named-field data structure; 4 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_admin_contract/src/lib.rs:889`](../server_admin_contract/src/lib.rs#L889) |
| `AdminDataTableFrontendPath` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_admin_contract/src/lib.rs:2819`](../server_admin_contract/src/lib.rs#L2819) |
| `AdminDataTableQuery` | `struct` | `public` | named-field data structure; 2 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_admin_contract/src/lib.rs:945`](../server_admin_contract/src/lib.rs#L945) |
| `AdminDataTableRoute` | `struct` | `public` | unit marker type; production | **Нет** | Не требуется | marker не содержит проверяемых данных | [`server_admin_contract/src/lib.rs:2708`](../server_admin_contract/src/lib.rs#L2708) |
| `AdminDataTableStrRef` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_admin_contract/src/lib.rs:281`](../server_admin_contract/src/lib.rs#L281) |
| `AdminDataTableView` | `struct` | `public` | named-field data structure; 4 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_admin_contract/src/lib.rs:2108`](../server_admin_contract/src/lib.rs#L2108) |
| `AdminDataTables` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`server_admin_contract/src/lib.rs:1288`](../server_admin_contract/src/lib.rs#L1288) |
| `AdminDataTablesRoute` | `struct` | `public` | unit marker type; production | **Нет** | Не требуется | marker не содержит проверяемых данных | [`server_admin_contract/src/lib.rs:2704`](../server_admin_contract/src/lib.rs#L2704) |
| `AdminDefaultPageLimit` | `struct` | `private` | unit marker type; production | **Нет** | Не требуется | marker не содержит проверяемых данных | [`server_admin_contract/src/lib.rs:681`](../server_admin_contract/src/lib.rs#L681) |
| `AdminDefaultRoute` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | атрибут валидатора задаёт инвариант типа | [`server_admin_contract/src/lib.rs:420`](../server_admin_contract/src/lib.rs#L420) |
| `AdminDeleteRoleRoute` | `struct` | `public` | unit marker type; production | **Нет** | Не требуется | marker не содержит проверяемых данных | [`server_admin_contract/src/lib.rs:2680`](../server_admin_contract/src/lib.rs#L2680) |
| `AdminDeleteUserRoute` | `struct` | `public` | unit marker type; production | **Нет** | Не требуется | marker не содержит проверяемых данных | [`server_admin_contract/src/lib.rs:2652`](../server_admin_contract/src/lib.rs#L2652) |
| `AdminDisplayName` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | атрибут валидатора задаёт инвариант типа | [`server_admin_contract/src/lib.rs:78`](../server_admin_contract/src/lib.rs#L78) |
| `AdminEmptyCollection` | `struct` | `private` | unit marker type; production | **Нет** | Не требуется | marker не содержит проверяемых данных | [`server_admin_contract/src/lib.rs:1372`](../server_admin_contract/src/lib.rs#L1372) |
| `AdminFilterField` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | атрибут валидатора задаёт инвариант типа | [`server_admin_contract/src/lib.rs:826`](../server_admin_contract/src/lib.rs#L826) |
| `AdminFilterOperationKey` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | атрибут валидатора задаёт инвариант типа | [`server_admin_contract/src/lib.rs:862`](../server_admin_contract/src/lib.rs#L862) |
| `AdminFilterValue` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | атрибут валидатора задаёт инвариант типа | [`server_admin_contract/src/lib.rs:844`](../server_admin_contract/src/lib.rs#L844) |
| `AdminFrontendPath` | `enum` | `public` | closed variant set; 14 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_admin_contract/src/lib.rs:2846`](../server_admin_contract/src/lib.rs#L2846) |
| `AdminHtmlAction` | `enum` | `public` | closed variant set; 15 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_admin_contract/src/lib.rs:2877`](../server_admin_contract/src/lib.rs#L2877) |
| `AdminListPermissionsRoute` | `struct` | `public` | unit marker type; production | **Нет** | Не требуется | marker не содержит проверяемых данных | [`server_admin_contract/src/lib.rs:2688`](../server_admin_contract/src/lib.rs#L2688) |
| `AdminListRolesRoute` | `struct` | `public` | unit marker type; production | **Нет** | Не требуется | marker не содержит проверяемых данных | [`server_admin_contract/src/lib.rs:2668`](../server_admin_contract/src/lib.rs#L2668) |
| `AdminListUsersRoute` | `struct` | `public` | unit marker type; production | **Нет** | Не требуется | marker не содержит проверяемых данных | [`server_admin_contract/src/lib.rs:2640`](../server_admin_contract/src/lib.rs#L2640) |
| `AdminLogin` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | атрибут валидатора задаёт инвариант типа | [`server_admin_contract/src/lib.rs:58`](../server_admin_contract/src/lib.rs#L58) |
| `AdminMainLogo` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | атрибут валидатора задаёт инвариант типа | [`server_admin_contract/src/lib.rs:428`](../server_admin_contract/src/lib.rs#L428) |
| `AdminMeRoute` | `struct` | `public` | unit marker type; production | **Нет** | Не требуется | marker не содержит проверяемых данных | [`server_admin_contract/src/lib.rs:2604`](../server_admin_contract/src/lib.rs#L2604) |
| `AdminNewPassword` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | атрибут валидатора задаёт инвариант типа | [`server_admin_contract/src/lib.rs:136`](../server_admin_contract/src/lib.rs#L136) |
| `AdminNoBody` | `struct` | `public` | unit marker type; production | **Нет** | Не требуется | marker не содержит проверяемых данных | [`server_admin_contract/src/lib.rs:2480`](../server_admin_contract/src/lib.rs#L2480) |
| `AdminOptionalSetting` | `enum` | `public` | closed variant set; 6 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_admin_contract/src/lib.rs:2339`](../server_admin_contract/src/lib.rs#L2339) |
| `AdminOptionalSettings` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`server_admin_contract/src/lib.rs:1310`](../server_admin_contract/src/lib.rs#L1310) |
| `AdminOrganizationContacts` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | атрибут валидатора задаёт инвариант типа | [`server_admin_contract/src/lib.rs:439`](../server_admin_contract/src/lib.rs#L439) |
| `AdminOrganizationName` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | атрибут валидатора задаёт инвариант типа | [`server_admin_contract/src/lib.rs:448`](../server_admin_contract/src/lib.rs#L448) |
| `AdminPage` | `enum` | `public` | closed variant set; 10 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_admin_contract/src/lib.rs:2962`](../server_admin_contract/src/lib.rs#L2962) |
| `AdminPageCapability` | `enum` | `public` | closed variant set; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_admin_contract/src/lib.rs:3040`](../server_admin_contract/src/lib.rs#L3040) |
| `AdminPageLimit` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`server_admin_contract/src/lib.rs:668`](../server_admin_contract/src/lib.rs#L668) |
| `AdminPageLimitError` | `struct` | `public` | unit marker type; production | **Нет** | Не требуется | marker не содержит проверяемых данных | [`server_admin_contract/src/lib.rs:702`](../server_admin_contract/src/lib.rs#L702) |
| `AdminPageOffset` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_admin_contract/src/lib.rs:651`](../server_admin_contract/src/lib.rs#L651) |
| `AdminPagePathRef` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_admin_contract/src/lib.rs:2844`](../server_admin_contract/src/lib.rs#L2844) |
| `AdminPageSpec` | `struct` | `public` | named-field data structure; 5 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_admin_contract/src/lib.rs:3058`](../server_admin_contract/src/lib.rs#L3058) |
| `AdminPageTitle` | `enum` | `private` | closed variant set; 10 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_admin_contract/src/lib.rs:3045`](../server_admin_contract/src/lib.rs#L3045) |
| `AdminPageTotal` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_admin_contract/src/lib.rs:706`](../server_admin_contract/src/lib.rs#L706) |
| `AdminPassword` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | атрибут валидатора задаёт инвариант типа | [`server_admin_contract/src/lib.rs:118`](../server_admin_contract/src/lib.rs#L118) |
| `AdminPermission` | `enum` | `public` | closed variant set; 29 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_admin_contract/src/lib.rs:181`](../server_admin_contract/src/lib.rs#L181) |
| `AdminPermissionId` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | wire database identity должен быть положительным | [`server_admin_contract/src/lib.rs:603`](../server_admin_contract/src/lib.rs#L603) |
| `AdminPermissionIds` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`server_admin_contract/src/lib.rs:1134`](../server_admin_contract/src/lib.rs#L1134) |
| `AdminPermissionStrRef` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_admin_contract/src/lib.rs:173`](../server_admin_contract/src/lib.rs#L173) |
| `AdminPermissionSummaries` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`server_admin_contract/src/lib.rs:1200`](../server_admin_contract/src/lib.rs#L1200) |
| `AdminPermissionSummary` | `struct` | `public` | named-field data structure; 2 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_admin_contract/src/lib.rs:1732`](../server_admin_contract/src/lib.rs#L1732) |
| `AdminPermissionValue` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | атрибут валидатора задаёт инвариант типа | [`server_admin_contract/src/lib.rs:155`](../server_admin_contract/src/lib.rs#L155) |
| `AdminPermissionValues` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`server_admin_contract/src/lib.rs:1068`](../server_admin_contract/src/lib.rs#L1068) |
| `AdminPermissionsPage` | `struct` | `public` | named-field data structure; 2 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_admin_contract/src/lib.rs:1825`](../server_admin_contract/src/lib.rs#L1825) |
| `AdminPrimaryColor` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | атрибут валидатора задаёт инвариант типа | [`server_admin_contract/src/lib.rs:457`](../server_admin_contract/src/lib.rs#L457) |
| `AdminRefreshRoute` | `struct` | `public` | unit marker type; production | **Нет** | Не требуется | marker не содержит проверяемых данных | [`server_admin_contract/src/lib.rs:2588`](../server_admin_contract/src/lib.rs#L2588) |
| `AdminRevokeAllSessionsRoute` | `struct` | `public` | unit marker type; production | **Нет** | Не требуется | marker не содержит проверяемых данных | [`server_admin_contract/src/lib.rs:2636`](../server_admin_contract/src/lib.rs#L2636) |
| `AdminRevokeSessionRoute` | `struct` | `public` | unit marker type; production | **Нет** | Не требуется | marker не содержит проверяемых данных | [`server_admin_contract/src/lib.rs:2632`](../server_admin_contract/src/lib.rs#L2632) |
| `AdminRoleId` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | wire database identity должен быть положительным | [`server_admin_contract/src/lib.rs:587`](../server_admin_contract/src/lib.rs#L587) |
| `AdminRoleIds` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`server_admin_contract/src/lib.rs:1112`](../server_admin_contract/src/lib.rs#L1112) |
| `AdminRoleName` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | атрибут валидатора задаёт инвариант типа | [`server_admin_contract/src/lib.rs:98`](../server_admin_contract/src/lib.rs#L98) |
| `AdminRoleNames` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`server_admin_contract/src/lib.rs:1090`](../server_admin_contract/src/lib.rs#L1090) |
| `AdminRoleSummaries` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`server_admin_contract/src/lib.rs:1178`](../server_admin_contract/src/lib.rs#L1178) |
| `AdminRoleSummary` | `struct` | `public` | named-field data structure; 4 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_admin_contract/src/lib.rs:1692`](../server_admin_contract/src/lib.rs#L1692) |
| `AdminRolesPage` | `struct` | `public` | named-field data structure; 3 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_admin_contract/src/lib.rs:1788`](../server_admin_contract/src/lib.rs#L1788) |
| `AdminRoute` | `enum` | `public` | closed variant set; 31 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_admin_contract/src/lib.rs:2720`](../server_admin_contract/src/lib.rs#L2720) |
| `AdminRoutePath` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`server_admin_contract/src/lib.rs:2821`](../server_admin_contract/src/lib.rs#L2821) |
| `AdminRoutePathError` | `enum` | `public` | error enum; 1 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_admin_contract/src/lib.rs:2823`](../server_admin_contract/src/lib.rs#L2823) |
| `AdminSessionIdentifier` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | атрибут валидатора задаёт инвариант типа | [`server_admin_contract/src/lib.rs:2483`](../server_admin_contract/src/lib.rs#L2483) |
| `AdminSessionTimestamp` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | атрибут валидатора задаёт инвариант типа | [`server_admin_contract/src/lib.rs:2493`](../server_admin_contract/src/lib.rs#L2493) |
| `AdminSessionView` | `struct` | `public` | named-field data structure; 4 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_admin_contract/src/lib.rs:2503`](../server_admin_contract/src/lib.rs#L2503) |
| `AdminSessionViews` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`server_admin_contract/src/lib.rs:1332`](../server_admin_contract/src/lib.rs#L1332) |
| `AdminSessionsPage` | `struct` | `public` | named-field data structure; 2 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_admin_contract/src/lib.rs:2512`](../server_admin_contract/src/lib.rs#L2512) |
| `AdminSessionsRoute` | `struct` | `public` | unit marker type; production | **Нет** | Не требуется | marker не содержит проверяемых данных | [`server_admin_contract/src/lib.rs:2628`](../server_admin_contract/src/lib.rs#L2628) |
| `AdminSetRolePermissionsReq` | `struct` | `public` | named-field data structure; 2 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_admin_contract/src/lib.rs:1623`](../server_admin_contract/src/lib.rs#L1623) |
| `AdminSetRolePermissionsRoute` | `struct` | `public` | unit marker type; production | **Нет** | Не требуется | marker не содержит проверяемых данных | [`server_admin_contract/src/lib.rs:2684`](../server_admin_contract/src/lib.rs#L2684) |
| `AdminSetUserBanReq` | `struct` | `public` | named-field data structure; 1 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_admin_contract/src/lib.rs:1549`](../server_admin_contract/src/lib.rs#L1549) |
| `AdminSetUserBanRoute` | `struct` | `public` | unit marker type; production | **Нет** | Не требуется | marker не содержит проверяемых данных | [`server_admin_contract/src/lib.rs:2660`](../server_admin_contract/src/lib.rs#L2660) |
| `AdminSetUserPasswordReq` | `struct` | `public` | named-field data structure; 1 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_admin_contract/src/lib.rs:1515`](../server_admin_contract/src/lib.rs#L1515) |
| `AdminSetUserPasswordRoute` | `struct` | `public` | unit marker type; production | **Нет** | Не требуется | marker не содержит проверяемых данных | [`server_admin_contract/src/lib.rs:2656`](../server_admin_contract/src/lib.rs#L2656) |
| `AdminSetUserRolesReq` | `struct` | `public` | named-field data structure; 2 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_admin_contract/src/lib.rs:1604`](../server_admin_contract/src/lib.rs#L1604) |
| `AdminSetUserRolesRoute` | `struct` | `public` | unit marker type; production | **Нет** | Не требуется | marker не содержит проверяемых данных | [`server_admin_contract/src/lib.rs:2664`](../server_admin_contract/src/lib.rs#L2664) |
| `AdminSettingsRoute` | `struct` | `public` | unit marker type; production | **Нет** | Не требуется | marker не содержит проверяемых данных | [`server_admin_contract/src/lib.rs:2712`](../server_admin_contract/src/lib.rs#L2712) |
| `AdminSettingsView` | `struct` | `public` | named-field data structure; 8 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_admin_contract/src/lib.rs:2212`](../server_admin_contract/src/lib.rs#L2212) |
| `AdminSignInReq` | `struct` | `public` | named-field data structure; 2 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_admin_contract/src/lib.rs:979`](../server_admin_contract/src/lib.rs#L979) |
| `AdminSignInRes` | `struct` | `public` | named-field data structure; 1 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_admin_contract/src/lib.rs:1447`](../server_admin_contract/src/lib.rs#L1447) |
| `AdminSignInRoute` | `struct` | `public` | unit marker type; production | **Нет** | Не требуется | marker не содержит проверяемых данных | [`server_admin_contract/src/lib.rs:2572`](../server_admin_contract/src/lib.rs#L2572) |
| `AdminSignOutRoute` | `struct` | `public` | unit marker type; production | **Нет** | Не требуется | marker не содержит проверяемых данных | [`server_admin_contract/src/lib.rs:2624`](../server_admin_contract/src/lib.rs#L2624) |
| `AdminSiteName` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | атрибут валидатора задаёт инвариант типа | [`server_admin_contract/src/lib.rs:423`](../server_admin_contract/src/lib.rs#L423) |
| `AdminSortDirection` | `enum` | `public` | closed variant set; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_admin_contract/src/lib.rs:743`](../server_admin_contract/src/lib.rs#L743) |
| `AdminSupportUrl` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | атрибут валидатора задаёт инвариант типа | [`server_admin_contract/src/lib.rs:468`](../server_admin_contract/src/lib.rs#L468) |
| `AdminTabTitle` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | атрибут валидатора задаёт инвариант типа | [`server_admin_contract/src/lib.rs:479`](../server_admin_contract/src/lib.rs#L479) |
| `AdminTableQuery` | `struct` | `public` | named-field data structure; 5 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_admin_contract/src/lib.rs:769`](../server_admin_contract/src/lib.rs#L769) |
| `AdminTableSearch` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | атрибут валидатора задаёт инвариант типа | [`server_admin_contract/src/lib.rs:723`](../server_admin_contract/src/lib.rs#L723) |
| `AdminTableSortField` | `enum` | `public` | closed variant set; 14 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_admin_contract/src/lib.rs:490`](../server_admin_contract/src/lib.rs#L490) |
| `AdminTableSortFieldTryFromKeyError` | `struct` | `public` | unit marker type; production | **Нет** | Не требуется | marker не содержит проверяемых данных | [`server_admin_contract/src/lib.rs:507`](../server_admin_contract/src/lib.rs#L507) |
| `AdminTableSortKey` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | атрибут валидатора задаёт инвариант типа | [`server_admin_contract/src/lib.rs:733`](../server_admin_contract/src/lib.rs#L733) |
| `AdminTableSortKeyRef` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_admin_contract/src/lib.rs:510`](../server_admin_contract/src/lib.rs#L510) |
| `AdminText` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | атрибут валидатора задаёт инвариант типа | [`server_admin_contract/src/lib.rs:40`](../server_admin_contract/src/lib.rs#L40) |
| `AdminTexts` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`server_admin_contract/src/lib.rs:1244`](../server_admin_contract/src/lib.rs#L1244) |
| `AdminUpdateRoleReq` | `struct` | `public` | named-field data structure; 1 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_admin_contract/src/lib.rs:1589`](../server_admin_contract/src/lib.rs#L1589) |
| `AdminUpdateRoleRoute` | `struct` | `public` | unit marker type; production | **Нет** | Не требуется | marker не содержит проверяемых данных | [`server_admin_contract/src/lib.rs:2676`](../server_admin_contract/src/lib.rs#L2676) |
| `AdminUpdateSettingsReq` | `struct` | `public` | named-field data structure; 9 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_admin_contract/src/lib.rs:2325`](../server_admin_contract/src/lib.rs#L2325) |
| `AdminUpdateSettingsRoute` | `struct` | `public` | unit marker type; production | **Нет** | Не требуется | marker не содержит проверяемых данных | [`server_admin_contract/src/lib.rs:2716`](../server_admin_contract/src/lib.rs#L2716) |
| `AdminUpdateUserReq` | `struct` | `public` | named-field data structure; 2 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_admin_contract/src/lib.rs:1496`](../server_admin_contract/src/lib.rs#L1496) |
| `AdminUpdateUserRoute` | `struct` | `public` | unit marker type; production | **Нет** | Не требуется | marker не содержит проверяемых данных | [`server_admin_contract/src/lib.rs:2648`](../server_admin_contract/src/lib.rs#L2648) |
| `AdminUserId` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | wire database identity должен быть положительным | [`server_admin_contract/src/lib.rs:571`](../server_admin_contract/src/lib.rs#L571) |
| `AdminUserSummaries` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`server_admin_contract/src/lib.rs:1156`](../server_admin_contract/src/lib.rs#L1156) |
| `AdminUserSummary` | `struct` | `public` | named-field data structure; 5 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_admin_contract/src/lib.rs:1645`](../server_admin_contract/src/lib.rs#L1645) |
| `AdminUsersPage` | `struct` | `public` | named-field data structure; 3 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_admin_contract/src/lib.rs:1751`](../server_admin_contract/src/lib.rs#L1751) |
| `AuthenticatedAdmin` | `struct` | `public` | named-field data structure; 5 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_admin_contract/src/lib.rs:1383`](../server_admin_contract/src/lib.rs#L1383) |
| `SerdeJsonAdminAuditDetails` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`server_admin_contract/src/lib.rs:396`](../server_admin_contract/src/lib.rs#L396) |
| `StdPhantomDataAdminBoundedVecVisitor` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_admin_contract/src/lib.rs:1028`](../server_admin_contract/src/lib.rs#L1028) |

## Crate `server_admin_frontend`

### Модуль `server_admin_frontend`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `AxumAdminFrontendRouter` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_admin_frontend/src/lib.rs:7`](../server_admin_frontend/src/lib.rs#L7) |

### Модуль `server_admin_frontend::app`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `AdminCsrApiUrl` | `struct` | `private` | single-field tuple wrapper; production | **Да** | Выполнено | атрибут валидатора задаёт инвариант типа | [`server_admin_frontend/src/app.rs:87`](../server_admin_frontend/src/app.rs#L87) |
| `AdminCsrPage` | `enum` | `private` | closed variant set; 7 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_admin_frontend/src/app.rs:45`](../server_admin_frontend/src/app.rs#L45) |
| `AdminCsrQuery` | `struct` | `private` | named-field data structure; 10 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_admin_frontend/src/app.rs:119`](../server_admin_frontend/src/app.rs#L119) |
| `AdminCsrfToken` | `struct` | `private` | single-field tuple wrapper; production | **Да** | Выполнено | атрибут валидатора задаёт инвариант типа | [`server_admin_frontend/src/app.rs:91`](../server_admin_frontend/src/app.rs#L91) |
| `AdminHttpStatus` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_admin_frontend/src/app.rs:84`](../server_admin_frontend/src/app.rs#L84) |
| `AdminLoadState` | `enum` | `private` | closed variant set; 10 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_admin_frontend/src/app.rs:13`](../server_admin_frontend/src/app.rs#L13) |
| `AdminMutationMethod` | `enum` | `private` | closed variant set; 4 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_admin_frontend/src/app.rs:95`](../server_admin_frontend/src/app.rs#L95) |
| `AdminTableLoadError` | `enum` | `private` | error enum; 4 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_admin_frontend/src/app.rs:72`](../server_admin_frontend/src/app.rs#L72) |
| `MutationConfirmationMessageRef` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_admin_frontend/src/app.rs:102`](../server_admin_frontend/src/app.rs#L102) |
| `MutationConfirmed` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_admin_frontend/src/app.rs:105`](../server_admin_frontend/src/app.rs#L105) |

### Модуль `server_admin_frontend::shared`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `AdminTableFilterDirection` | `enum` | `crate` | closed variant set; 3 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_admin_frontend/src/shared.rs:12`](../server_admin_frontend/src/shared.rs#L12) |
| `AdminTableFilterPresentation` | `enum` | `crate` | closed variant set; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_admin_frontend/src/shared.rs:20`](../server_admin_frontend/src/shared.rs#L20) |
| `AdminTableQueryDirection` | `enum` | `crate` | closed variant set; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_admin_frontend/src/shared.rs:48`](../server_admin_frontend/src/shared.rs#L48) |

### Модуль `server_admin_frontend::ssr`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `AdminSsrErrorMessage` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`server_admin_frontend/src/ssr.rs:44`](../server_admin_frontend/src/ssr.rs#L44) |
| `AdminSsrHtml` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`server_admin_frontend/src/ssr.rs:66`](../server_admin_frontend/src/ssr.rs#L66) |
| `AdminSsrHtmlTryFromStringError` | `struct` | `public` | unit marker type; production | **Нет** | Не требуется | marker не содержит проверяемых данных | [`server_admin_frontend/src/ssr.rs:27`](../server_admin_frontend/src/ssr.rs#L27) |
| `AdminSsrText` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`server_admin_frontend/src/ssr.rs:53`](../server_admin_frontend/src/ssr.rs#L53) |
| `AdminSsrTextTryFromStringError` | `struct` | `public` | unit marker type; production | **Нет** | Не требуется | marker не содержит проверяемых данных | [`server_admin_frontend/src/ssr.rs:35`](../server_admin_frontend/src/ssr.rs#L35) |
| `AdminSsrViewExt` | `trait` | `private` | behavior contract; 1 associated items; production | **Нет** | Не требуется | trait не инициализируется как значение | [`server_admin_frontend/src/ssr.rs:12`](../server_admin_frontend/src/ssr.rs#L12) |

## Crate `server_app_state`

### Модуль `server_app_state`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `ServerAppState` | `struct` | `public` | named-field data structure; 5 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_app_state/src/lib.rs:1`](../server_app_state/src/lib.rs#L1) |

## Crate `server_config`

### Модуль `server_config`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `Config` | `struct` | `public` | named-field data structure; 27 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_config/src/lib.rs:1`](../server_config/src/lib.rs#L1) |

## Crate `server_runtime`

### Модуль `server_runtime`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `AxumRouter` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/lib.rs:214`](../server_runtime/src/lib.rs#L214) |
| `ForwardedProtoTrust` | `enum` | `public` | closed variant set; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_runtime/src/lib.rs:469`](../server_runtime/src/lib.rs#L469) |
| `HttpContentSecurityPolicy` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`server_runtime/src/lib.rs:474`](../server_runtime/src/lib.rs#L474) |
| `HttpContentSecurityPolicyError` | `struct` | `public` | unit marker type; production | **Нет** | Не требуется | marker не содержит проверяемых данных | [`server_runtime/src/lib.rs:476`](../server_runtime/src/lib.rs#L476) |
| `RequestIdLayer` | `struct` | `public` | unit marker type; production | **Нет** | Не требуется | marker не содержит проверяемых данных | [`server_runtime/src/lib.rs:302`](../server_runtime/src/lib.rs#L302) |
| `RequestIdService` | `struct` | `private` | named-field data structure; 1 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_runtime/src/lib.rs:312`](../server_runtime/src/lib.rs#L312) |
| `RequestIdTowerLayer` | `struct` | `private` | unit marker type; production | **Нет** | Не требуется | marker не содержит проверяемых данных | [`server_runtime/src/lib.rs:310`](../server_runtime/src/lib.rs#L310) |
| `RequestTimeoutBody` | `struct` | `private` | named-field data structure; 1 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_runtime/src/lib.rs:399`](../server_runtime/src/lib.rs#L399) |
| `RequestTimeoutLayer` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/lib.rs:391`](../server_runtime/src/lib.rs#L391) |
| `RequestTimeoutService` | `struct` | `private` | named-field data structure; 2 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_runtime/src/lib.rs:412`](../server_runtime/src/lib.rs#L412) |
| `RequestTimeoutTowerLayer` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/lib.rs:409`](../server_runtime/src/lib.rs#L409) |
| `ReqwestClient` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | fallible-конструктор показывает наличие инварианта | [`server_runtime/src/lib.rs:216`](../server_runtime/src/lib.rs#L216) |
| `ReqwestClientBuildError` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/lib.rs:263`](../server_runtime/src/lib.rs#L263) |
| `ReqwestClientPolicy` | `struct` | `public` | named-field data structure; 2 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_runtime/src/lib.rs:246`](../server_runtime/src/lib.rs#L246) |
| `SecurityHeadersLayer` | `struct` | `public` | named-field data structure; 2 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_runtime/src/lib.rs:490`](../server_runtime/src/lib.rs#L490) |
| `SecurityHeadersService` | `struct` | `private` | named-field data structure; 3 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_runtime/src/lib.rs:522`](../server_runtime/src/lib.rs#L522) |
| `SecurityHeadersTowerLayer` | `struct` | `private` | named-field data structure; 2 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_runtime/src/lib.rs:517`](../server_runtime/src/lib.rs#L517) |
| `ServeWithGracefulShutdownError` | `enum` | `public` | error enum; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_runtime/src/lib.rs:623`](../server_runtime/src/lib.rs#L623) |
| `ServiceRuntime` | `struct` | `public` | named-field data structure; 2 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_runtime/src/lib.rs:281`](../server_runtime/src/lib.rs#L281) |
| `StdRequestTimeoutMessage` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/lib.rs:394`](../server_runtime/src/lib.rs#L394) |
| `StdReqwestConnectTimeout` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`server_runtime/src/lib.rs:219`](../server_runtime/src/lib.rs#L219) |
| `StdReqwestRequestTimeout` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`server_runtime/src/lib.rs:221`](../server_runtime/src/lib.rs#L221) |
| `StdReqwestTimeoutError` | `struct` | `public` | unit marker type; production | **Нет** | Не требуется | marker не содержит проверяемых данных | [`server_runtime/src/lib.rs:223`](../server_runtime/src/lib.rs#L223) |
| `StdServeIoError` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/lib.rs:621`](../server_runtime/src/lib.rs#L621) |
| `TokioTcpListener` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/lib.rs:299`](../server_runtime/src/lib.rs#L299) |

### Модуль `server_runtime::background_job`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `BackgroundJob` | `struct` | `public` | named-field data structure; 2 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_runtime/src/background_job.rs:1`](../server_runtime/src/background_job.rs#L1) |

### Модуль `server_runtime::batched_cleanup`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `CleanupBatchCount` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/batched_cleanup.rs:25`](../server_runtime/src/batched_cleanup.rs#L25) |
| `CleanupBatchSize` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`server_runtime/src/batched_cleanup.rs:1`](../server_runtime/src/batched_cleanup.rs#L1) |
| `CleanupBatchSizeError` | `struct` | `public` | unit marker type; production | **Нет** | Не требуется | marker не содержит проверяемых данных | [`server_runtime/src/batched_cleanup.rs:4`](../server_runtime/src/batched_cleanup.rs#L4) |
| `CleanupCompletion` | `enum` | `public` | closed variant set; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_runtime/src/batched_cleanup.rs:36`](../server_runtime/src/batched_cleanup.rs#L36) |
| `CleanupContinuation` | `enum` | `public` | closed variant set; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_runtime/src/batched_cleanup.rs:30`](../server_runtime/src/batched_cleanup.rs#L30) |
| `CleanupReport` | `struct` | `public` | named-field data structure; 3 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_runtime/src/batched_cleanup.rs:42`](../server_runtime/src/batched_cleanup.rs#L42) |
| `CleanupRows` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/batched_cleanup.rs:20`](../server_runtime/src/batched_cleanup.rs#L20) |

### Модуль `server_runtime::bounded_read`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `BoundedBytes` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | внутреннее значение может расти; нужен предел длины/размера | [`server_runtime/src/bounded_read.rs:7`](../server_runtime/src/bounded_read.rs#L7) |
| `BoundedJsonReadError` | `enum` | `public` | error enum; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_runtime/src/bounded_read.rs:95`](../server_runtime/src/bounded_read.rs#L95) |
| `BoundedJsonText` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`server_runtime/src/bounded_read.rs:61`](../server_runtime/src/bounded_read.rs#L61) |
| `BoundedReadError` | `enum` | `public` | error enum; 5 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_runtime/src/bounded_read.rs:102`](../server_runtime/src/bounded_read.rs#L102) |
| `BoundedReadMaximumBytes` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/bounded_read.rs:4`](../server_runtime/src/bounded_read.rs#L4) |
| `BoundedReadObservedBytes` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/bounded_read.rs:126`](../server_runtime/src/bounded_read.rs#L126) |
| `BoundedText` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`server_runtime/src/bounded_read.rs:9`](../server_runtime/src/bounded_read.rs#L9) |
| `IoErrorPresenceDisposition` | `enum` | `public` | closed variant set; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_runtime/src/bounded_read.rs:47`](../server_runtime/src/bounded_read.rs#L47) |
| `ReqwestError` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/bounded_read.rs:52`](../server_runtime/src/bounded_read.rs#L52) |
| `ReqwestResponse` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/bounded_read.rs:56`](../server_runtime/src/bounded_read.rs#L56) |
| `SerdeJsonError` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/bounded_read.rs:59`](../server_runtime/src/bounded_read.rs#L59) |
| `StdBoundedReadConcurrency` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/bounded_read.rs:31`](../server_runtime/src/bounded_read.rs#L31) |
| `StdBoundedReadConcurrencyMaximum` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/bounded_read.rs:34`](../server_runtime/src/bounded_read.rs#L34) |
| `StdFromUtf8Error` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/bounded_read.rs:54`](../server_runtime/src/bounded_read.rs#L54) |
| `StdIoError` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/bounded_read.rs:45`](../server_runtime/src/bounded_read.rs#L45) |
| `StdPathRef` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/bounded_read.rs:1`](../server_runtime/src/bounded_read.rs#L1) |

### Модуль `server_runtime::child_process`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `ChildDiagnostic` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | внутреннее значение может расти; нужен предел длины/размера | [`server_runtime/src/child_process.rs:79`](../server_runtime/src/child_process.rs#L79) |
| `ChildProcessCompletion` | `enum` | `public` | closed variant set; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_runtime/src/child_process.rs:82`](../server_runtime/src/child_process.rs#L82) |
| `ChildProcessError` | `enum` | `public` | error enum; 6 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_runtime/src/child_process.rs:206`](../server_runtime/src/child_process.rs#L206) |
| `ChildProcessId` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/child_process.rs:4`](../server_runtime/src/child_process.rs#L4) |
| `ChildProcessReport` | `struct` | `public` | named-field data structure; 3 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_runtime/src/child_process.rs:108`](../server_runtime/src/child_process.rs#L108) |
| `ChildProcessReports` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | внутреннее значение может расти; нужен предел длины/размера | [`server_runtime/src/child_process.rs:66`](../server_runtime/src/child_process.rs#L66) |
| `ChildProcessSet` | `struct` | `public` | named-field data structure; 3 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_runtime/src/child_process.rs:15`](../server_runtime/src/child_process.rs#L15) |
| `ChildProcessSetError` | `enum` | `public` | error enum; 3 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_runtime/src/child_process.rs:69`](../server_runtime/src/child_process.rs#L69) |
| `ChildProcessSucceeded` | `enum` | `public` | closed variant set; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_runtime/src/child_process.rs:102`](../server_runtime/src/child_process.rs#L102) |
| `ChildProcessSupervisor` | `struct` | `public` | named-field data structure; 2 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_runtime/src/child_process.rs:142`](../server_runtime/src/child_process.rs#L142) |
| `StdChildDiagnosticMaximum` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/child_process.rs:1`](../server_runtime/src/child_process.rs#L1) |
| `StdChildExitStatus` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/child_process.rs:88`](../server_runtime/src/child_process.rs#L88) |
| `StdChildProcessIoError` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/child_process.rs:222`](../server_runtime/src/child_process.rs#L222) |
| `StdChildProcessSetMaximum` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/child_process.rs:7`](../server_runtime/src/child_process.rs#L7) |
| `StdCollectionsChildProcessMap` | `struct` | `private` | single-field tuple wrapper; production | **Да** | Выполнено | внутреннее значение может расти; нужен предел длины/размера | [`server_runtime/src/child_process.rs:10`](../server_runtime/src/child_process.rs#L10) |
| `TokioChildDiagnosticTask` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/child_process.rs:137`](../server_runtime/src/child_process.rs#L137) |
| `TokioChildProcess` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/child_process.rs:134`](../server_runtime/src/child_process.rs#L134) |
| `TokioChildProcessJoinError` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/child_process.rs:227`](../server_runtime/src/child_process.rs#L227) |
| `TokioManagedChild` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/child_process.rs:131`](../server_runtime/src/child_process.rs#L131) |

### Модуль `server_runtime::client_ip`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `HttpHeaderMapRef` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/client_ip.rs:4`](../server_runtime/src/client_ip.rs#L4) |
| `StdAddrParseError` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/client_ip.rs:33`](../server_runtime/src/client_ip.rs#L33) |
| `StdIpAddr` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/client_ip.rs:19`](../server_runtime/src/client_ip.rs#L19) |
| `StdParseIntError` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/client_ip.rs:36`](../server_runtime/src/client_ip.rs#L36) |
| `StdRangeContains` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/client_ip.rs:22`](../server_runtime/src/client_ip.rs#L22) |
| `StdResolvedClientIp` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/client_ip.rs:10`](../server_runtime/src/client_ip.rs#L10) |
| `StdSocketAddr` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/client_ip.rs:7`](../server_runtime/src/client_ip.rs#L7) |
| `StdTrustedProxyPrefixBits` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/client_ip.rs:30`](../server_runtime/src/client_ip.rs#L30) |
| `TrustedProxyRange` | `struct` | `public` | named-field data structure; 2 fields; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`server_runtime/src/client_ip.rs:14`](../server_runtime/src/client_ip.rs#L14) |
| `TrustedProxyRangeParseError` | `enum` | `public` | error enum; 4 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_runtime/src/client_ip.rs:39`](../server_runtime/src/client_ip.rs#L39) |
| `TrustedProxyRanges` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`server_runtime/src/client_ip.rs:85`](../server_runtime/src/client_ip.rs#L85) |
| `TrustedProxyRangesError` | `struct` | `public` | unit marker type; production | **Нет** | Не требуется | marker не содержит проверяемых данных | [`server_runtime/src/client_ip.rs:87`](../server_runtime/src/client_ip.rs#L87) |

### Модуль `server_runtime::cors`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `HttpCorsAllowOriginHeaderValues` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | внутреннее значение может расти; нужен предел длины/размера | [`server_runtime/src/cors.rs:7`](../server_runtime/src/cors.rs#L7) |
| `HttpCorsAllowOriginHeaderValuesError` | `enum` | `public` | error enum; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_runtime/src/cors.rs:10`](../server_runtime/src/cors.rs#L10) |
| `HttpCorsAllowOriginTextRef` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/cors.rs:4`](../server_runtime/src/cors.rs#L4) |

### Модуль `server_runtime::csp`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `HttpCspBuilder` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`server_runtime/src/csp.rs:61`](../server_runtime/src/csp.rs#L61) |
| `HttpCspDirectiveName` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`server_runtime/src/csp.rs:5`](../server_runtime/src/csp.rs#L5) |
| `HttpCspDirectiveValue` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`server_runtime/src/csp.rs:8`](../server_runtime/src/csp.rs#L8) |
| `HttpCspMaximumBytesError` | `struct` | `public` | unit marker type; production | **Нет** | Не требуется | marker не содержит проверяемых данных | [`server_runtime/src/csp.rs:75`](../server_runtime/src/csp.rs#L75) |
| `HttpCspTokenError` | `enum` | `public` | error enum; 3 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_runtime/src/csp.rs:11`](../server_runtime/src/csp.rs#L11) |

### Модуль `server_runtime::deduplicating_queue`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `DeduplicatingQueue` | `struct` | `public` | named-field data structure; 3 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_runtime/src/deduplicating_queue.rs:11`](../server_runtime/src/deduplicating_queue.rs#L11) |
| `QueuePush` | `enum` | `public` | closed variant set; 3 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_runtime/src/deduplicating_queue.rs:4`](../server_runtime/src/deduplicating_queue.rs#L4) |
| `StdCollectionsHashSet` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | отдельный инвариант этого служебного типа не требуется; проверка принадлежит итоговой доменной границе | [`server_runtime/src/deduplicating_queue.rs:56`](../server_runtime/src/deduplicating_queue.rs#L56) |
| `StdCollectionsVecDeque` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | отдельный инвариант этого служебного типа не требуется; проверка принадлежит итоговой доменной границе | [`server_runtime/src/deduplicating_queue.rs:59`](../server_runtime/src/deduplicating_queue.rs#L59) |
| `StdQueueMaximum` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/deduplicating_queue.rs:1`](../server_runtime/src/deduplicating_queue.rs#L1) |

### Модуль `server_runtime::exclusive_run`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `ExclusiveRun` | `struct` | `public` | named-field data structure; 1 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_runtime/src/exclusive_run.rs:1`](../server_runtime/src/exclusive_run.rs#L1) |
| `ExclusiveRunAlreadyActive` | `struct` | `public` | unit marker type; production | **Нет** | Не требуется | marker не содержит проверяемых данных | [`server_runtime/src/exclusive_run.rs:34`](../server_runtime/src/exclusive_run.rs#L34) |
| `ExclusiveRunGuard` | `struct` | `public` | named-field data structure; 1 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_runtime/src/exclusive_run.rs:38`](../server_runtime/src/exclusive_run.rs#L38) |
| `StdExclusiveRunAtomicBool` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/exclusive_run.rs:51`](../server_runtime/src/exclusive_run.rs#L51) |

### Модуль `server_runtime::execution_plan`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `ExecutionMode` | `enum` | `public` | closed variant set; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_runtime/src/execution_plan.rs:1`](../server_runtime/src/execution_plan.rs#L1) |
| `ExecutionReport` | `enum` | `public` | closed variant set; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_runtime/src/execution_plan.rs:7`](../server_runtime/src/execution_plan.rs#L7) |

### Модуль `server_runtime::fallback`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `AcceptsApplicationJson` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/fallback.rs:31`](../server_runtime/src/fallback.rs#L31) |
| `FallbackResponseMode` | `enum` | `public` | closed variant set; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_runtime/src/fallback.rs:7`](../server_runtime/src/fallback.rs#L7) |
| `HttpAcceptHeaderMaximumBytes` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/fallback.rs:25`](../server_runtime/src/fallback.rs#L25) |
| `HttpFallbackApiPrefixRef` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/fallback.rs:16`](../server_runtime/src/fallback.rs#L16) |
| `HttpFallbackMetricsPathRef` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/fallback.rs:19`](../server_runtime/src/fallback.rs#L19) |
| `HttpFallbackRequestPathRef` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/fallback.rs:13`](../server_runtime/src/fallback.rs#L13) |
| `HttpMediaRangeRef` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/fallback.rs:28`](../server_runtime/src/fallback.rs#L28) |
| `HttpOptionalAcceptHeaderRef` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/fallback.rs:22`](../server_runtime/src/fallback.rs#L22) |

### Модуль `server_runtime::generation_gate`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `Generation` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/generation_gate.rs:1`](../server_runtime/src/generation_gate.rs#L1) |
| `GenerationCommit` | `enum` | `public` | closed variant set; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_runtime/src/generation_gate.rs:4`](../server_runtime/src/generation_gate.rs#L4) |
| `GenerationGate` | `struct` | `public` | named-field data structure; 1 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_runtime/src/generation_gate.rs:10`](../server_runtime/src/generation_gate.rs#L10) |
| `StdGenerationAtomicU64` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/generation_gate.rs:35`](../server_runtime/src/generation_gate.rs#L35) |

### Модуль `server_runtime::geojson`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `GeoJsonDocumentText` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`server_runtime/src/geojson.rs:3`](../server_runtime/src/geojson.rs#L3) |
| `GeoJsonValidationError` | `enum` | `public` | error enum; 5 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_runtime/src/geojson.rs:21`](../server_runtime/src/geojson.rs#L21) |
| `GeoJsonValueValidation` | `trait` | `private` | behavior contract; 5 associated items; production | **Нет** | Не требуется | trait не инициализируется как значение | [`server_runtime/src/geojson.rs:35`](../server_runtime/src/geojson.rs#L35) |
| `SerdeJsonGeoJsonError` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/geojson.rs:18`](../server_runtime/src/geojson.rs#L18) |

### Модуль `server_runtime::header_text`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `HttpHeaderName` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/header_text.rs:7`](../server_runtime/src/header_text.rs#L7) |
| `HttpHeaderTextBytes` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/header_text.rs:4`](../server_runtime/src/header_text.rs#L4) |
| `HttpHeaderTextMaximumBytes` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`server_runtime/src/header_text.rs:1`](../server_runtime/src/header_text.rs#L1) |
| `HttpHeaderTextMaximumBytesError` | `struct` | `public` | unit marker type; production | **Нет** | Не требуется | marker не содержит проверяемых данных | [`server_runtime/src/header_text.rs:13`](../server_runtime/src/header_text.rs#L13) |
| `HttpHeaderTextRef` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/header_text.rs:10`](../server_runtime/src/header_text.rs#L10) |
| `HttpHeaderTextResolution` | `enum` | `public` | closed variant set; 4 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_runtime/src/header_text.rs:28`](../server_runtime/src/header_text.rs#L28) |

### Модуль `server_runtime::health`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `HealthComponentStatus` | `enum` | `public` | closed variant set; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_runtime/src/health.rs:7`](../server_runtime/src/health.rs#L7) |
| `HealthProbeSucceeded` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/health.rs:4`](../server_runtime/src/health.rs#L4) |
| `HealthReadiness` | `struct` | `public` | named-field data structure; 1 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_runtime/src/health.rs:41`](../server_runtime/src/health.rs#L41) |
| `HealthSnapshot` | `struct` | `public` | named-field data structure; 2 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_runtime/src/health.rs:14`](../server_runtime/src/health.rs#L14) |
| `ServiceLivenessSnapshot` | `struct` | `public` | named-field data structure; 1 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_runtime/src/health.rs:20`](../server_runtime/src/health.rs#L20) |
| `StdHealthProbeTimeout` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/health.rs:1`](../server_runtime/src/health.rs#L1) |
| `StdHealthReadinessAtomicBool` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/health.rs:35`](../server_runtime/src/health.rs#L35) |
| `StdSharedHealthReadiness` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/health.rs:38`](../server_runtime/src/health.rs#L38) |

### Модуль `server_runtime::history`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `AsyncRunHistory` | `struct` | `public` | named-field data structure; 2 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_runtime/src/history.rs:9`](../server_runtime/src/history.rs#L9) |
| `AsyncRunHistorySnapshot` | `struct` | `public` | named-field data structure; 2 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_runtime/src/history.rs:30`](../server_runtime/src/history.rs#L30) |
| `StdArcSharedRunReports` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/history.rs:7`](../server_runtime/src/history.rs#L7) |
| `StdAsyncRunHistoryMaximumLen` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`server_runtime/src/history.rs:14`](../server_runtime/src/history.rs#L14) |
| `StdAsyncRunHistoryMaximumLenTryFromUsizeError` | `struct` | `public` | unit marker type; production | **Нет** | Не требуется | marker не содержит проверяемых данных | [`server_runtime/src/history.rs:24`](../server_runtime/src/history.rs#L24) |
| `StdAsyncRunHistoryReportCount` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/history.rs:27`](../server_runtime/src/history.rs#L27) |
| `StdVecDequeRunReports` | `struct` | `private` | single-field tuple wrapper; production | **Да** | Выполнено | внутреннее значение может расти; нужен предел длины/размера | [`server_runtime/src/history.rs:1`](../server_runtime/src/history.rs#L1) |
| `TokioRwLockRunReports` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/history.rs:4`](../server_runtime/src/history.rs#L4) |

### Модуль `server_runtime::http_header_policy`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `HttpAttachmentFileNameRef` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/http_header_policy.rs:7`](../server_runtime/src/http_header_policy.rs#L7) |
| `HttpContentDisposition` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/http_header_policy.rs:10`](../server_runtime/src/http_header_policy.rs#L10) |
| `HttpContentDispositionError` | `enum` | `public` | error enum; 3 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_runtime/src/http_header_policy.rs:13`](../server_runtime/src/http_header_policy.rs#L13) |
| `HttpContentLength` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`server_runtime/src/http_header_policy.rs:100`](../server_runtime/src/http_header_policy.rs#L100) |
| `HttpContentLengthError` | `enum` | `public` | error enum; 4 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_runtime/src/http_header_policy.rs:89`](../server_runtime/src/http_header_policy.rs#L89) |

### Модуль `server_runtime::http_policy`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `BearerAuthorizationResolution` | `enum` | `public` | closed variant set; 3 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_runtime/src/http_policy.rs:10`](../server_runtime/src/http_policy.rs#L10) |
| `CookieResolution` | `enum` | `public` | closed variant set; 3 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_runtime/src/http_policy.rs:56`](../server_runtime/src/http_policy.rs#L56) |
| `HttpAuthorizationHeaderTextRef` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/http_policy.rs:5`](../server_runtime/src/http_policy.rs#L5) |
| `HttpBearerTokenRef` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/http_policy.rs:8`](../server_runtime/src/http_policy.rs#L8) |
| `HttpContentTypeTextRef` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/http_policy.rs:106`](../server_runtime/src/http_policy.rs#L106) |
| `HttpCookieHeadersRef` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/http_policy.rs:39`](../server_runtime/src/http_policy.rs#L39) |
| `HttpCookieNameRef` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/http_policy.rs:42`](../server_runtime/src/http_policy.rs#L42) |
| `HttpCookieValueRef` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/http_policy.rs:45`](../server_runtime/src/http_policy.rs#L45) |
| `OptionalJsonBodyPresence` | `enum` | `public` | closed variant set; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_runtime/src/http_policy.rs:135`](../server_runtime/src/http_policy.rs#L135) |
| `OptionalJsonContentType` | `enum` | `public` | closed variant set; 3 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_runtime/src/http_policy.rs:109`](../server_runtime/src/http_policy.rs#L109) |
| `OptionalJsonContentTypeDecision` | `enum` | `public` | closed variant set; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_runtime/src/http_policy.rs:140`](../server_runtime/src/http_policy.rs#L140) |

### Модуль `server_runtime::http_status_error`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `HttpErrorClass` | `enum` | `public` | closed variant set; 11 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_runtime/src/http_status_error.rs:4`](../server_runtime/src/http_status_error.rs#L4) |
| `HttpErrorStatus` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/http_status_error.rs:1`](../server_runtime/src/http_status_error.rs#L1) |

### Модуль `server_runtime::identity_bootstrap`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `IdentityBootstrapDecision` | `enum` | `public` | closed variant set; 3 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_runtime/src/identity_bootstrap.rs:58`](../server_runtime/src/identity_bootstrap.rs#L58) |
| `IdentityPresence` | `enum` | `public` | closed variant set; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_runtime/src/identity_bootstrap.rs:46`](../server_runtime/src/identity_bootstrap.rs#L46) |
| `IdentityRolePresence` | `enum` | `public` | closed variant set; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_runtime/src/identity_bootstrap.rs:52`](../server_runtime/src/identity_bootstrap.rs#L52) |
| `IdentitySpec` | `struct` | `public` | named-field data structure; 4 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_runtime/src/identity_bootstrap.rs:1`](../server_runtime/src/identity_bootstrap.rs#L1) |

### Модуль `server_runtime::lease_registry`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `LeaseEntry` | `struct` | `private` | named-field data structure; 3 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_runtime/src/lease_registry.rs:80`](../server_runtime/src/lease_registry.rs#L80) |
| `LeaseHeartbeat` | `enum` | `public` | closed variant set; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_runtime/src/lease_registry.rs:71`](../server_runtime/src/lease_registry.rs#L71) |
| `LeaseId` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`server_runtime/src/lease_registry.rs:3`](../server_runtime/src/lease_registry.rs#L3) |
| `LeaseIds` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | внутреннее значение может расти; нужен предел длины/размера | [`server_runtime/src/lease_registry.rs:77`](../server_runtime/src/lease_registry.rs#L77) |
| `LeaseKey` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`server_runtime/src/lease_registry.rs:15`](../server_runtime/src/lease_registry.rs#L15) |
| `LeaseRegistry` | `struct` | `public` | named-field data structure; 1 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_runtime/src/lease_registry.rs:93`](../server_runtime/src/lease_registry.rs#L93) |
| `LeaseRegistryInner` | `struct` | `private` | named-field data structure; 2 fields; production | **Нет** | Не требуется | mutable registry-state; capacity проверяется при `reserve`, не при создании | [`server_runtime/src/lease_registry.rs:87`](../server_runtime/src/lease_registry.rs#L87) |
| `LeaseReservation` | `enum` | `public` | closed variant set; 3 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_runtime/src/lease_registry.rs:64`](../server_runtime/src/lease_registry.rs#L64) |
| `LeaseState` | `enum` | `public` | closed variant set; 3 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_runtime/src/lease_registry.rs:37`](../server_runtime/src/lease_registry.rs#L37) |
| `LeaseTextError` | `enum` | `public` | error enum; 3 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_runtime/src/lease_registry.rs:27`](../server_runtime/src/lease_registry.rs#L27) |
| `LeaseTextRef` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/lease_registry.rs:176`](../server_runtime/src/lease_registry.rs#L176) |
| `StdArcTokioLeaseRegistryRwLock` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/lease_registry.rs:179`](../server_runtime/src/lease_registry.rs#L179) |
| `StdLeaseRegistryMaximum` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/lease_registry.rs:44`](../server_runtime/src/lease_registry.rs#L44) |
| `StdLeaseStaleTimeout` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`server_runtime/src/lease_registry.rs:47`](../server_runtime/src/lease_registry.rs#L47) |
| `StdLeaseStaleTimeoutError` | `struct` | `public` | unit marker type; production | **Нет** | Не требуется | marker не содержит проверяемых данных | [`server_runtime/src/lease_registry.rs:60`](../server_runtime/src/lease_registry.rs#L60) |
| `TokioLeaseInstant` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/lease_registry.rs:182`](../server_runtime/src/lease_registry.rs#L182) |
| `TokioLeaseRegistryRwLock` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/lease_registry.rs:185`](../server_runtime/src/lease_registry.rs#L185) |

### Модуль `server_runtime::lifecycle`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `BackgroundTask` | `struct` | `public` | named-field data structure; 2 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_runtime/src/lifecycle.rs:18`](../server_runtime/src/lifecycle.rs#L18) |
| `BackgroundTaskOutcome` | `enum` | `public` | closed variant set; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_runtime/src/lifecycle.rs:1`](../server_runtime/src/lifecycle.rs#L1) |
| `BackgroundTaskShutdownError` | `enum` | `public` | error enum; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_runtime/src/lifecycle.rs:11`](../server_runtime/src/lifecycle.rs#L11) |
| `StdRequestTimeout` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`server_runtime/src/lifecycle.rs:90`](../server_runtime/src/lifecycle.rs#L90) |
| `StdRequestTimeoutTryFromDurationError` | `struct` | `public` | unit marker type; production | **Нет** | Не требуется | marker не содержит проверяемых данных | [`server_runtime/src/lifecycle.rs:107`](../server_runtime/src/lifecycle.rs#L107) |
| `StdRunInterval` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`server_runtime/src/lifecycle.rs:75`](../server_runtime/src/lifecycle.rs#L75) |
| `StdRunIntervalTryFromDurationError` | `struct` | `public` | unit marker type; production | **Нет** | Не требуется | marker не содержит проверяемых данных | [`server_runtime/src/lifecycle.rs:87`](../server_runtime/src/lifecycle.rs#L87) |
| `TokioAbortTask` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/lifecycle.rs:8`](../server_runtime/src/lifecycle.rs#L8) |
| `TokioBackgroundTaskJoinHandle` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/lifecycle.rs:24`](../server_runtime/src/lifecycle.rs#L24) |
| `TokioBackgroundTaskShutdownSender` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/lifecycle.rs:27`](../server_runtime/src/lifecycle.rs#L27) |
| `TokioTaskJoinError` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/lifecycle.rs:6`](../server_runtime/src/lifecycle.rs#L6) |

### Модуль `server_runtime::limits`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `AcquirePermitError` | `enum` | `public` | error enum; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_runtime/src/limits.rs:48`](../server_runtime/src/limits.rs#L48) |
| `RetryAfterSecs` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`server_runtime/src/limits.rs:4`](../server_runtime/src/limits.rs#L4) |
| `RetryAfterSecsTryFromU64Error` | `struct` | `public` | unit marker type; production | **Нет** | Не требуется | marker не содержит проверяемых данных | [`server_runtime/src/limits.rs:16`](../server_runtime/src/limits.rs#L16) |
| `StdArcTokioSemaphore` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/limits.rs:25`](../server_runtime/src/limits.rs#L25) |
| `StdPermitWaitTimeout` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/limits.rs:1`](../server_runtime/src/limits.rs#L1) |
| `StdSemaphorePermitCount` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/limits.rs:27`](../server_runtime/src/limits.rs#L27) |
| `TokioAcquireError` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/limits.rs:46`](../server_runtime/src/limits.rs#L46) |
| `TokioOwnedSemaphorePermit` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/limits.rs:55`](../server_runtime/src/limits.rs#L55) |

### Модуль `server_runtime::metrics_layer`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `HttpMetricsLayer` | `struct` | `public` | named-field data structure; 1 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_runtime/src/metrics_layer.rs:146`](../server_runtime/src/metrics_layer.rs#L146) |
| `HttpMetricsPathCache` | `struct` | `private` | named-field data structure; 3 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_runtime/src/metrics_layer.rs:51`](../server_runtime/src/metrics_layer.rs#L51) |
| `HttpMetricsPathCacheMaximum` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`server_runtime/src/metrics_layer.rs:28`](../server_runtime/src/metrics_layer.rs#L28) |
| `HttpMetricsPathCacheMaximumTryFromUsizeError` | `struct` | `public` | unit marker type; production | **Нет** | Не требуется | marker не содержит проверяемых данных | [`server_runtime/src/metrics_layer.rs:47`](../server_runtime/src/metrics_layer.rs#L47) |
| `HttpMetricsPathText` | `struct` | `private` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`server_runtime/src/metrics_layer.rs:66`](../server_runtime/src/metrics_layer.rs#L66) |
| `HttpMetricsPathTextError` | `struct` | `private` | unit marker type; production | **Нет** | Не требуется | marker не содержит проверяемых данных | [`server_runtime/src/metrics_layer.rs:81`](../server_runtime/src/metrics_layer.rs#L81) |
| `HttpMetricsPathTextRef` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/metrics_layer.rs:84`](../server_runtime/src/metrics_layer.rs#L84) |
| `HttpMetricsService` | `struct` | `private` | named-field data structure; 2 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_runtime/src/metrics_layer.rs:191`](../server_runtime/src/metrics_layer.rs#L191) |
| `HttpMetricsTowerLayer` | `struct` | `private` | named-field data structure; 1 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_runtime/src/metrics_layer.rs:175`](../server_runtime/src/metrics_layer.rs#L175) |
| `MetricsResponseBody` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`server_runtime/src/metrics_layer.rs:4`](../server_runtime/src/metrics_layer.rs#L4) |
| `MetricsResponseBodyError` | `struct` | `public` | unit marker type; production | **Нет** | Не требуется | marker не содержит проверяемых данных | [`server_runtime/src/metrics_layer.rs:24`](../server_runtime/src/metrics_layer.rs#L24) |
| `MetricsSharedString` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/metrics_layer.rs:63`](../server_runtime/src/metrics_layer.rs#L63) |
| `StdHttpMetricsPathEntries` | `struct` | `private` | single-field tuple wrapper; production | **Да** | Выполнено | внутреннее значение может расти; нужен предел длины/размера | [`server_runtime/src/metrics_layer.rs:58`](../server_runtime/src/metrics_layer.rs#L58) |
| `StdSharedHttpMetricsPathCache` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/metrics_layer.rs:87`](../server_runtime/src/metrics_layer.rs#L87) |

### Модуль `server_runtime::multipart`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `FileStagingAction` | `enum` | `public` | closed variant set; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_runtime/src/multipart.rs:237`](../server_runtime/src/multipart.rs#L237) |
| `FileStagingDirectoryName` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`server_runtime/src/multipart.rs:243`](../server_runtime/src/multipart.rs#L243) |
| `MultipartBytes` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`server_runtime/src/multipart.rs:86`](../server_runtime/src/multipart.rs#L86) |
| `MultipartBytesPart` | `struct` | `public` | named-field data structure; 3 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_runtime/src/multipart.rs:120`](../server_runtime/src/multipart.rs#L120) |
| `MultipartBytesParts` | `struct` | `private` | single-field tuple wrapper; production | **Да** | Выполнено | внутреннее значение может расти; нужен предел длины/размера | [`server_runtime/src/multipart.rs:126`](../server_runtime/src/multipart.rs#L126) |
| `MultipartFieldName` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`server_runtime/src/multipart.rs:25`](../server_runtime/src/multipart.rs#L25) |
| `MultipartFileName` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`server_runtime/src/multipart.rs:44`](../server_runtime/src/multipart.rs#L44) |
| `MultipartPayloadMaximum` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/multipart.rs:5`](../server_runtime/src/multipart.rs#L5) |
| `MultipartRequestError` | `enum` | `public` | error enum; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_runtime/src/multipart.rs:159`](../server_runtime/src/multipart.rs#L159) |
| `MultipartTextPart` | `struct` | `public` | named-field data structure; 2 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_runtime/src/multipart.rs:100`](../server_runtime/src/multipart.rs#L100) |
| `MultipartTextParts` | `struct` | `private` | single-field tuple wrapper; production | **Да** | Выполнено | внутреннее значение может расти; нужен предел длины/размера | [`server_runtime/src/multipart.rs:129`](../server_runtime/src/multipart.rs#L129) |
| `MultipartTextValue` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`server_runtime/src/multipart.rs:70`](../server_runtime/src/multipart.rs#L70) |
| `MultipartUploadRequest` | `struct` | `public` | named-field data structure; 3 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_runtime/src/multipart.rs:167`](../server_runtime/src/multipart.rs#L167) |
| `MultipartValueError` | `enum` | `public` | error enum; 5 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_runtime/src/multipart.rs:11`](../server_runtime/src/multipart.rs#L11) |
| `MultipartValueLength` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/multipart.rs:8`](../server_runtime/src/multipart.rs#L8) |
| `StdStorageRelativePath` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | внутреннее значение может расти; нужен предел длины/размера | [`server_runtime/src/multipart.rs:285`](../server_runtime/src/multipart.rs#L285) |
| `StoragePathSegment` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`server_runtime/src/multipart.rs:266`](../server_runtime/src/multipart.rs#L266) |
| `StoragePathSegmentError` | `struct` | `public` | unit marker type; production | **Нет** | Не требуется | marker не содержит проверяемых данных | [`server_runtime/src/multipart.rs:268`](../server_runtime/src/multipart.rs#L268) |

### Модуль `server_runtime::notification`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `AxumNotificationJson` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/notification.rs:149`](../server_runtime/src/notification.rs#L149) |
| `AxumNotificationRouter` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/notification.rs:124`](../server_runtime/src/notification.rs#L124) |
| `AxumNotificationState` | `struct` | `private` | named-field data structure; 2 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_runtime/src/notification.rs:129`](../server_runtime/src/notification.rs#L129) |
| `HttpNotificationHeaderMap` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | отдельный инвариант этого служебного типа не требуется; проверка принадлежит итоговой доменной границе | [`server_runtime/src/notification.rs:126`](../server_runtime/src/notification.rs#L126) |
| `NotificationApiToken` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`server_runtime/src/notification.rs:1`](../server_runtime/src/notification.rs#L1) |
| `NotificationApiTokenAuthorized` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/notification.rs:7`](../server_runtime/src/notification.rs#L7) |
| `NotificationApiTokenError` | `enum` | `public` | error enum; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_runtime/src/notification.rs:34`](../server_runtime/src/notification.rs#L34) |
| `NotificationApiTokenRef` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/notification.rs:4`](../server_runtime/src/notification.rs#L4) |
| `NotificationMessage` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`server_runtime/src/notification.rs:56`](../server_runtime/src/notification.rs#L56) |
| `NotificationMessageError` | `enum` | `public` | error enum; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_runtime/src/notification.rs:60`](../server_runtime/src/notification.rs#L60) |
| `NotificationRequest` | `struct` | `public` | named-field data structure; 1 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_runtime/src/notification.rs:91`](../server_runtime/src/notification.rs#L91) |
| `NotificationSender` | `trait` | `public` | behavior contract; 2 associated items; production | **Нет** | Не требуется | trait не инициализируется как значение | [`server_runtime/src/notification.rs:82`](../server_runtime/src/notification.rs#L82) |
| `NotificationServiceState` | `struct` | `public` | named-field data structure; 3 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_runtime/src/notification.rs:103`](../server_runtime/src/notification.rs#L103) |

### Модуль `server_runtime::notification::tests`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `TestSender` | `struct` | `private` | unit marker type; test-only | **Нет** | Не требуется | marker не содержит проверяемых данных | [`server_runtime/src/notification.rs:221`](../server_runtime/src/notification.rs#L221) |

### Модуль `server_runtime::origin`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `AllowOriginSuffix` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/origin.rs:91`](../server_runtime/src/origin.rs#L91) |
| `AllowedOrigin` | `struct` | `public` | named-field data structure; 2 fields; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`server_runtime/src/origin.rs:1`](../server_runtime/src/origin.rs#L1) |
| `AllowedOriginError` | `struct` | `public` | unit marker type; production | **Нет** | Не требуется | marker не содержит проверяемых данных | [`server_runtime/src/origin.rs:58`](../server_runtime/src/origin.rs#L58) |
| `AllowedOrigins` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`server_runtime/src/origin.rs:62`](../server_runtime/src/origin.rs#L62) |
| `AllowedOriginsError` | `struct` | `public` | unit marker type; production | **Нет** | Не требуется | marker не содержит проверяемых данных | [`server_runtime/src/origin.rs:81`](../server_runtime/src/origin.rs#L81) |
| `HttpOriginAuthorityText` | `struct` | `private` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`server_runtime/src/origin.rs:7`](../server_runtime/src/origin.rs#L7) |
| `HttpOriginHeadersRef` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/origin.rs:85`](../server_runtime/src/origin.rs#L85) |
| `HttpOriginSchemeText` | `struct` | `private` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`server_runtime/src/origin.rs:22`](../server_runtime/src/origin.rs#L22) |
| `HttpOriginTextRef` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/origin.rs:88`](../server_runtime/src/origin.rs#L88) |
| `ParsedHttpOriginRef` | `struct` | `private` | named-field data structure; 2 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_runtime/src/origin.rs:94`](../server_runtime/src/origin.rs#L94) |
| `RequestOriginAllowed` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/origin.rs:100`](../server_runtime/src/origin.rs#L100) |

### Модуль `server_runtime::outbound_url`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `OutboundAddressDisposition` | `enum` | `private` | closed variant set; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_runtime/src/outbound_url.rs:200`](../server_runtime/src/outbound_url.rs#L200) |
| `OutboundAllowedHost` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`server_runtime/src/outbound_url.rs:32`](../server_runtime/src/outbound_url.rs#L32) |
| `OutboundHostAllowlist` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`server_runtime/src/outbound_url.rs:49`](../server_runtime/src/outbound_url.rs#L49) |
| `OutboundHostAllowlistError` | `enum` | `public` | error enum; 4 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_runtime/src/outbound_url.rs:83`](../server_runtime/src/outbound_url.rs#L83) |
| `OutboundHostPolicy` | `enum` | `public` | closed variant set; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_runtime/src/outbound_url.rs:1`](../server_runtime/src/outbound_url.rs#L1) |
| `OutboundUrlError` | `enum` | `public` | error enum; 7 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_runtime/src/outbound_url.rs:182`](../server_runtime/src/outbound_url.rs#L182) |
| `OutboundUrlPolicy` | `struct` | `public` | named-field data structure; 2 fields; production | **Нет** | Не требуется | policy сама валидна конструктивно; её `validate` проверяет URL, а не policy | [`server_runtime/src/outbound_url.rs:105`](../server_runtime/src/outbound_url.rs#L105) |
| `OutboundUrlScheme` | `enum` | `public` | closed variant set; 4 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_runtime/src/outbound_url.rs:7`](../server_runtime/src/outbound_url.rs#L7) |
| `OutboundUrlTextRef` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/outbound_url.rs:14`](../server_runtime/src/outbound_url.rs#L14) |
| `ReqwestOutboundUrl` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/outbound_url.rs:17`](../server_runtime/src/outbound_url.rs#L17) |
| `StdOutboundIpAddr` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/outbound_url.rs:102`](../server_runtime/src/outbound_url.rs#L102) |

### Модуль `server_runtime::path_policy`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `HttpAllowedPathPrefixRef` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/path_policy.rs:66`](../server_runtime/src/path_policy.rs#L66) |
| `HttpNormalizedPath` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`server_runtime/src/path_policy.rs:88`](../server_runtime/src/path_policy.rs#L88) |
| `HttpNormalizedPathError` | `struct` | `public` | unit marker type; production | **Нет** | Не требуется | marker не содержит проверяемых данных | [`server_runtime/src/path_policy.rs:90`](../server_runtime/src/path_policy.rs#L90) |
| `HttpProxyPath` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`server_runtime/src/path_policy.rs:8`](../server_runtime/src/path_policy.rs#L8) |
| `HttpProxyPathError` | `enum` | `public` | error enum; 3 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_runtime/src/path_policy.rs:10`](../server_runtime/src/path_policy.rs#L10) |
| `HttpProxyPathPrefixMatch` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/path_policy.rs:69`](../server_runtime/src/path_policy.rs#L69) |
| `HttpProxyPathRef` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/path_policy.rs:5`](../server_runtime/src/path_policy.rs#L5) |
| `HttpRequestPathRef` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/path_policy.rs:85`](../server_runtime/src/path_policy.rs#L85) |

### Модуль `server_runtime::pg_rate_limit`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `PgRateLimitDecision` | `enum` | `public` | closed variant set; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_runtime/src/pg_rate_limit.rs:69`](../server_runtime/src/pg_rate_limit.rs#L69) |
| `PgRateLimitError` | `enum` | `public` | error enum; 1 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_runtime/src/pg_rate_limit.rs:90`](../server_runtime/src/pg_rate_limit.rs#L90) |
| `PgRateLimitMaximum` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`server_runtime/src/pg_rate_limit.rs:41`](../server_runtime/src/pg_rate_limit.rs#L41) |
| `PgRateLimitQueryRef` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/pg_rate_limit.rs:3`](../server_runtime/src/pg_rate_limit.rs#L3) |
| `PgRateLimitScopeRef` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`server_runtime/src/pg_rate_limit.rs:9`](../server_runtime/src/pg_rate_limit.rs#L9) |
| `PgRateLimitSubjectRef` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`server_runtime/src/pg_rate_limit.rs:25`](../server_runtime/src/pg_rate_limit.rs#L25) |
| `PgRateLimitValidationError` | `enum` | `public` | error enum; 3 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_runtime/src/pg_rate_limit.rs:75`](../server_runtime/src/pg_rate_limit.rs#L75) |
| `PgRateLimitWindowSeconds` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`server_runtime/src/pg_rate_limit.rs:55`](../server_runtime/src/pg_rate_limit.rs#L55) |
| `SqlxPgRateLimitError` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/pg_rate_limit.rs:85`](../server_runtime/src/pg_rate_limit.rs#L85) |
| `SqlxPgRateLimitPoolRef` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/pg_rate_limit.rs:6`](../server_runtime/src/pg_rate_limit.rs#L6) |

### Модуль `server_runtime::redacted_url`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `RedactedUrl` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/redacted_url.rs:1`](../server_runtime/src/redacted_url.rs#L1) |
| `RedactedUrlTextRef` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/redacted_url.rs:24`](../server_runtime/src/redacted_url.rs#L24) |

### Модуль `server_runtime::request_id`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `HttpHeaderToStrError` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/request_id.rs:17`](../server_runtime/src/request_id.rs#L17) |
| `RequestId` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`server_runtime/src/request_id.rs:1`](../server_runtime/src/request_id.rs#L1) |
| `RequestIdTryFromHttpHeaderValueError` | `enum` | `public` | error enum; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_runtime/src/request_id.rs:19`](../server_runtime/src/request_id.rs#L19) |
| `RequestIdTryFromStringError` | `struct` | `public` | unit marker type; production | **Нет** | Не требуется | marker не содержит проверяемых данных | [`server_runtime/src/request_id.rs:14`](../server_runtime/src/request_id.rs#L14) |

### Модуль `server_runtime::resource_budget`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `GetBulkItemResourceBudget` | `trait` | `public` | behavior contract; 1 associated items; production | **Нет** | Не требуется | trait не инициализируется как значение | [`server_runtime/src/resource_budget.rs:35`](../server_runtime/src/resource_budget.rs#L35) |
| `GetIdempotencyResponseResourceBudget` | `trait` | `public` | behavior contract; 1 associated items; production | **Нет** | Не требуется | trait не инициализируется как значение | [`server_runtime/src/resource_budget.rs:38`](../server_runtime/src/resource_budget.rs#L38) |
| `ResourceBudget` | `struct` | `public` | named-field data structure; 2 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_runtime/src/resource_budget.rs:30`](../server_runtime/src/resource_budget.rs#L30) |
| `ResourceBudgetAmount` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/resource_budget.rs:3`](../server_runtime/src/resource_budget.rs#L3) |
| `ResourceBudgetConfigError` | `struct` | `public` | unit marker type; production | **Нет** | Не требуется | marker не содержит проверяемых данных | [`server_runtime/src/resource_budget.rs:27`](../server_runtime/src/resource_budget.rs#L27) |
| `ResourceBudgetMaximum` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`server_runtime/src/resource_budget.rs:1`](../server_runtime/src/resource_budget.rs#L1) |
| `ResourceBudgetReservation` | `struct` | `public` | named-field data structure; 2 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_runtime/src/resource_budget.rs:54`](../server_runtime/src/resource_budget.rs#L54) |
| `ResourceBudgetReserveError` | `enum` | `public` | error enum; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_runtime/src/resource_budget.rs:41`](../server_runtime/src/resource_budget.rs#L41) |
| `StdAtomicUsize` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/resource_budget.rs:6`](../server_runtime/src/resource_budget.rs#L6) |
| `StdSharedAtomicUsize` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/resource_budget.rs:9`](../server_runtime/src/resource_budget.rs#L9) |

### Модуль `server_runtime::resource_utilization`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `ResourceAmount` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/resource_utilization.rs:5`](../server_runtime/src/resource_utilization.rs#L5) |
| `ResourceUtilization` | `struct` | `public` | named-field data structure; 4 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_runtime/src/resource_utilization.rs:35`](../server_runtime/src/resource_utilization.rs#L35) |
| `ResourceUtilizationError` | `enum` | `public` | error enum; 1 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_runtime/src/resource_utilization.rs:26`](../server_runtime/src/resource_utilization.rs#L26) |
| `ResourceUtilizationPercent` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | percentage должен находиться в диапазоне `0..=100` | [`server_runtime/src/resource_utilization.rs:8`](../server_runtime/src/resource_utilization.rs#L8) |
| `ResourceUtilizationStatus` | `enum` | `public` | closed variant set; 4 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_runtime/src/resource_utilization.rs:18`](../server_runtime/src/resource_utilization.rs#L18) |

### Модуль `server_runtime::retry`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `RetryOutcome` | `struct` | `public` | named-field data structure; 2 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_runtime/src/retry.rs:51`](../server_runtime/src/retry.rs#L51) |
| `RetryPolicy` | `struct` | `public` | named-field data structure; 2 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_runtime/src/retry.rs:28`](../server_runtime/src/retry.rs#L28) |
| `StdRetryAttempts` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`server_runtime/src/retry.rs:1`](../server_runtime/src/retry.rs#L1) |
| `StdRetryAttemptsError` | `struct` | `public` | unit marker type; production | **Нет** | Не требуется | marker не содержит проверяемых данных | [`server_runtime/src/retry.rs:21`](../server_runtime/src/retry.rs#L21) |
| `StdRetryDelay` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/retry.rs:25`](../server_runtime/src/retry.rs#L25) |

### Модуль `server_runtime::secret_text`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `BoundedSecretText` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`server_runtime/src/secret_text.rs:14`](../server_runtime/src/secret_text.rs#L14) |
| `BoundedSecretTextError` | `enum` | `public` | error enum; 3 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_runtime/src/secret_text.rs:4`](../server_runtime/src/secret_text.rs#L4) |
| `SecretTextMatch` | `enum` | `public` | closed variant set; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_runtime/src/secret_text.rs:50`](../server_runtime/src/secret_text.rs#L50) |
| `SecretTextRef` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/secret_text.rs:42`](../server_runtime/src/secret_text.rs#L42) |

### Модуль `server_runtime::secure_cookie`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `HttpCookieAccess` | `enum` | `public` | closed variant set; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_runtime/src/secure_cookie.rs:47`](../server_runtime/src/secure_cookie.rs#L47) |
| `HttpCookieName` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`server_runtime/src/secure_cookie.rs:3`](../server_runtime/src/secure_cookie.rs#L3) |
| `HttpCookieSecure` | `enum` | `public` | closed variant set; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_runtime/src/secure_cookie.rs:53`](../server_runtime/src/secure_cookie.rs#L53) |
| `HttpCookieValue` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`server_runtime/src/secure_cookie.rs:22`](../server_runtime/src/secure_cookie.rs#L22) |
| `HttpSecureCookieError` | `enum` | `public` | error enum; 3 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_runtime/src/secure_cookie.rs:62`](../server_runtime/src/secure_cookie.rs#L62) |
| `HttpSetCookieHeaderValue` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/secure_cookie.rs:59`](../server_runtime/src/secure_cookie.rs#L59) |
| `StdCookieMaxAgeSeconds` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/secure_cookie.rs:44`](../server_runtime/src/secure_cookie.rs#L44) |

### Модуль `server_runtime::service_bootstrap`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `ServiceTracingFormat` | `enum` | `public` | closed variant set; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_runtime/src/service_bootstrap.rs:1`](../server_runtime/src/service_bootstrap.rs#L1) |
| `StdServiceRuntimeIoError` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/service_bootstrap.rs:13`](../server_runtime/src/service_bootstrap.rs#L13) |
| `TokioServiceRuntime` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/service_bootstrap.rs:10`](../server_runtime/src/service_bootstrap.rs#L10) |
| `TracingSubscriberInitError` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/service_bootstrap.rs:7`](../server_runtime/src/service_bootstrap.rs#L7) |

### Модуль `server_runtime::single_flight`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `SingleFlight` | `struct` | `public` | named-field data structure; 2 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_runtime/src/single_flight.rs:34`](../server_runtime/src/single_flight.rs#L34) |
| `SingleFlightAcquire` | `enum` | `public` | closed variant set; 3 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_runtime/src/single_flight.rs:73`](../server_runtime/src/single_flight.rs#L73) |
| `SingleFlightInner` | `struct` | `private` | named-field data structure; 1 fields; production | **Нет** | Не требуется | mutable map-state; maximum проверяется при `acquire`, не при создании | [`server_runtime/src/single_flight.rs:114`](../server_runtime/src/single_flight.rs#L114) |
| `SingleFlightKey` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`server_runtime/src/single_flight.rs:3`](../server_runtime/src/single_flight.rs#L3) |
| `SingleFlightKeyError` | `enum` | `public` | error enum; 3 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_runtime/src/single_flight.rs:21`](../server_runtime/src/single_flight.rs#L21) |
| `SingleFlightOwner` | `struct` | `public` | named-field data structure; 2 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_runtime/src/single_flight.rs:80`](../server_runtime/src/single_flight.rs#L80) |
| `SingleFlightSignal` | `enum` | `private` | closed variant set; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_runtime/src/single_flight.rs:130`](../server_runtime/src/single_flight.rs#L130) |
| `SingleFlightWaitOutcome` | `enum` | `public` | closed variant set; 1 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_runtime/src/single_flight.rs:109`](../server_runtime/src/single_flight.rs#L109) |
| `SingleFlightWaiter` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/single_flight.rs:98`](../server_runtime/src/single_flight.rs#L98) |
| `StdArcStdSingleFlightRwLock` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/single_flight.rs:119`](../server_runtime/src/single_flight.rs#L119) |
| `StdSingleFlightMaximum` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/single_flight.rs:31`](../server_runtime/src/single_flight.rs#L31) |
| `StdSingleFlightRwLock` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/single_flight.rs:122`](../server_runtime/src/single_flight.rs#L122) |
| `StdSingleFlightWriteGuard` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/single_flight.rs:125`](../server_runtime/src/single_flight.rs#L125) |
| `TokioSingleFlightReceiver` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/single_flight.rs:136`](../server_runtime/src/single_flight.rs#L136) |
| `TokioSingleFlightSender` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/single_flight.rs:139`](../server_runtime/src/single_flight.rs#L139) |

### Модуль `server_runtime::source_selection`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `SourceSelection` | `enum` | `public` | closed variant set; 3 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_runtime/src/source_selection.rs:1`](../server_runtime/src/source_selection.rs#L1) |
| `SourceSelectionError` | `struct` | `public` | unit marker type; production | **Нет** | Не требуется | marker не содержит проверяемых данных | [`server_runtime/src/source_selection.rs:11`](../server_runtime/src/source_selection.rs#L11) |

### Модуль `server_runtime::trace_context`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `HttpTraceParent` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`server_runtime/src/trace_context.rs:4`](../server_runtime/src/trace_context.rs#L4) |
| `HttpTraceParentError` | `enum` | `public` | error enum; 3 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_runtime/src/trace_context.rs:7`](../server_runtime/src/trace_context.rs#L7) |
| `HttpTraceState` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`server_runtime/src/trace_context.rs:57`](../server_runtime/src/trace_context.rs#L57) |
| `HttpTraceStateError` | `struct` | `public` | unit marker type; production | **Нет** | Не требуется | marker не содержит проверяемых данных | [`server_runtime/src/trace_context.rs:60`](../server_runtime/src/trace_context.rs#L60) |
| `OutboundTraceContext` | `struct` | `public` | named-field data structure; 3 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`server_runtime/src/trace_context.rs:78`](../server_runtime/src/trace_context.rs#L78) |
| `ReqwestRequestBuilder` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`server_runtime/src/trace_context.rs:85`](../server_runtime/src/trace_context.rs#L85) |

### Модуль `server_runtime::wire_token`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `VersionedUrlSafeWireTokenText` | `struct` | `public` | named-field data structure; 3 fields; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`server_runtime/src/wire_token.rs:13`](../server_runtime/src/wire_token.rs#L13) |
| `VersionedUrlSafeWireTokenTextError` | `enum` | `public` | error enum; 3 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`server_runtime/src/wire_token.rs:3`](../server_runtime/src/wire_token.rs#L3) |

## Crate `str_constants_macros`

### Модуль `str_constants_macros`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `Constant` | `struct` | `private` | named-field data structure; 3 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`str_constants_macros/src/lib.rs:73`](../str_constants_macros/src/lib.rs#L73) |
| `ConstantPart` | `enum` | `private` | closed variant set; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`str_constants_macros/src/lib.rs:50`](../str_constants_macros/src/lib.rs#L50) |
| `ConstantParts` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | отдельный инвариант этого служебного типа не требуется; проверка принадлежит итоговой доменной границе | [`str_constants_macros/src/lib.rs:54`](../str_constants_macros/src/lib.rs#L54) |
| `Constants` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | отдельный инвариант этого служебного типа не требуется; проверка принадлежит итоговой доменной границе | [`str_constants_macros/src/lib.rs:60`](../str_constants_macros/src/lib.rs#L60) |
| `DefineStrConstantsInput` | `struct` | `private` | named-field data structure; 2 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`str_constants_macros/src/lib.rs:79`](../str_constants_macros/src/lib.rs#L79) |
| `Fragment` | `struct` | `private` | named-field data structure; 2 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`str_constants_macros/src/lib.rs:45`](../str_constants_macros/src/lib.rs#L45) |
| `Fragments` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | отдельный инвариант этого служебного типа не требуется; проверка принадлежит итоговой доменной границе | [`str_constants_macros/src/lib.rs:66`](../str_constants_macros/src/lib.rs#L66) |
| `SynIdent` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`str_constants_macros/src/lib.rs:6`](../str_constants_macros/src/lib.rs#L6) |
| `SynLitStr` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`str_constants_macros/src/lib.rs:19`](../str_constants_macros/src/lib.rs#L19) |
| `SynVisibility` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`str_constants_macros/src/lib.rs:32`](../str_constants_macros/src/lib.rs#L32) |

## Crate `synchronization_service_runtime`

### Модуль `synchronization_service_runtime`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `SynchronizationPayload` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`synchronization_service_runtime/src/lib.rs:18`](../synchronization_service_runtime/src/lib.rs#L18) |
| `SynchronizationPayloadTooLarge` | `struct` | `public` | unit marker type; production | **Нет** | Не требуется | marker не содержит проверяемых данных | [`synchronization_service_runtime/src/lib.rs:9`](../synchronization_service_runtime/src/lib.rs#L9) |
| `SynchronizationRuntimeConfiguration` | `struct` | `public` | named-field data structure; 2 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`synchronization_service_runtime/src/lib.rs:3`](../synchronization_service_runtime/src/lib.rs#L3) |
| `SynchronizationSource` | `trait` | `public` | behavior contract; 2 associated items; production | **Нет** | Не требуется | trait не инициализируется как значение | [`synchronization_service_runtime/src/lib.rs:33`](../synchronization_service_runtime/src/lib.rs#L33) |

## Crate `tests`

### Модуль `tests::code_style`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `AnalyzerStateRawContainerFieldVisitor` | `struct` | `private` | named-field data structure; 1 fields; test-only | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`tests/src/code_style/mod.rs:1833`](../tests/src/code_style/mod.rs#L1833) |
| `AsyncBlockingCallVisitor` | `struct` | `private` | named-field data structure; 2 fields; test-only | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`tests/src/code_style/mod.rs:315`](../tests/src/code_style/mod.rs#L315) |
| `ConstDisplayImplVisitor` | `struct` | `private` | named-field data structure; 1 fields; test-only | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`tests/src/code_style/mod.rs:786`](../tests/src/code_style/mod.rs#L786) |
| `ConstantAliasVisitor` | `struct` | `private` | named-field data structure; 1 fields; test-only | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`tests/src/code_style/mod.rs:605`](../tests/src/code_style/mod.rs#L605) |
| `DbgVisitor` | `struct` | `private` | named-field data structure; 1 fields; test-only | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`tests/src/code_style/mod.rs:41`](../tests/src/code_style/mod.rs#L41) |
| `DeclaredDomainTypeVisitor` | `struct` | `private` | named-field data structure; 1 fields; test-only | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`tests/src/code_style/mod.rs:1746`](../tests/src/code_style/mod.rs#L1746) |
| `DeserializeConversionCallVisitor` | `struct` | `private` | named-field data structure; 1 fields; test-only | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`tests/src/code_style/mod.rs:1606`](../tests/src/code_style/mod.rs#L1606) |
| `DirectDeserializeTupleWrapperVisitor` | `struct` | `private` | named-field data structure; 1 fields; test-only | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`tests/src/code_style/mod.rs:1603`](../tests/src/code_style/mod.rs#L1603) |
| `DirectPathCallVisitor` | `struct` | `private` | named-field data structure; 1 fields; test-only | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`tests/src/code_style/mod.rs:416`](../tests/src/code_style/mod.rs#L416) |
| `DirectTupleWrapperConstructorVisitor` | `struct` | `private` | named-field data structure; 4 fields; test-only | **Нет** | Не требуется | AST visitor accumulator; ограничения применяются при обходе, не через `TryFrom` | [`tests/src/code_style/mod.rs:1617`](../tests/src/code_style/mod.rs#L1617) |
| `DomainTypePolicyVisitor` | `struct` | `private` | named-field data structure; 5 fields; test-only | **Нет** | Не требуется | AST visitor accumulator; ограничения применяются при обходе, не через `TryFrom` | [`tests/src/code_style/mod.rs:1826`](../tests/src/code_style/mod.rs#L1826) |
| `EmptyErrorImplVisitor` | `struct` | `private` | named-field data structure; 1 fields; test-only | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`tests/src/code_style/mod.rs:780`](../tests/src/code_style/mod.rs#L780) |
| `ExpectOrPanic` | `enum` | `private` | closed variant set; 2 variants; test-only | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`tests/src/code_style/mod.rs:15`](../tests/src/code_style/mod.rs#L15) |
| `ExternalLeafWrapperNameException` | `struct` | `private` | named-field data structure; 2 fields; test-only | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`tests/src/code_style/mod.rs:11`](../tests/src/code_style/mod.rs#L11) |
| `ExternalLeafWrapperNameVisitor` | `struct` | `private` | named-field data structure; 2 fields; test-only | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`tests/src/code_style/mod.rs:1839`](../tests/src/code_style/mod.rs#L1839) |
| `ForLoopVisitor` | `struct` | `private` | named-field data structure; 1 fields; test-only | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`tests/src/code_style/mod.rs:85`](../tests/src/code_style/mod.rs#L85) |
| `ForwardingBorrowVisitor` | `struct` | `private` | named-field data structure; 1 fields; test-only | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`tests/src/code_style/mod.rs:638`](../tests/src/code_style/mod.rs#L638) |
| `ForwardingDerefVisitor` | `struct` | `private` | named-field data structure; 2 fields; test-only | **Нет** | Не требуется | AST visitor accumulator; ограничения применяются при обходе, не через `TryFrom` | [`tests/src/code_style/mod.rs:634`](../tests/src/code_style/mod.rs#L634) |
| `ForwardingDisplayVisitor` | `struct` | `private` | named-field data structure; 1 fields; test-only | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`tests/src/code_style/mod.rs:777`](../tests/src/code_style/mod.rs#L777) |
| `ForwardingIntoIteratorVisitor` | `struct` | `private` | named-field data structure; 1 fields; test-only | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`tests/src/code_style/mod.rs:916`](../tests/src/code_style/mod.rs#L916) |
| `HelperRawTextReturnVisitor` | `struct` | `private` | named-field data structure; 1 fields; test-only | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`tests/src/code_style/mod.rs:1836`](../tests/src/code_style/mod.rs#L1836) |
| `IncludeAssetMacroVisitor` | `struct` | `private` | named-field data structure; 1 fields; test-only | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`tests/src/code_style/mod.rs:402`](../tests/src/code_style/mod.rs#L402) |
| `LenCheckedFunctionNameVisitor` | `struct` | `private` | named-field data structure; 1 fields; test-only | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`tests/src/code_style/mod.rs:1562`](../tests/src/code_style/mod.rs#L1562) |
| `LenMethodCallVisitor` | `struct` | `private` | named-field data structure; 1 fields; test-only | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`tests/src/code_style/mod.rs:1589`](../tests/src/code_style/mod.rs#L1589) |
| `LostSpawnVisitor` | `struct` | `private` | named-field data structure; 1 fields; test-only | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`tests/src/code_style/mod.rs:459`](../tests/src/code_style/mod.rs#L459) |
| `ManualDeserializeTupleWrapperVisitor` | `struct` | `private` | named-field data structure; 2 fields; test-only | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`tests/src/code_style/mod.rs:1609`](../tests/src/code_style/mod.rs#L1609) |
| `ManualNotImplVisitor` | `struct` | `private` | named-field data structure; 1 fields; test-only | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`tests/src/code_style/mod.rs:783`](../tests/src/code_style/mod.rs#L783) |
| `NumericAsCastVisitor` | `struct` | `private` | named-field data structure; 1 fields; test-only | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`tests/src/code_style/mod.rs:106`](../tests/src/code_style/mod.rs#L106) |
| `PassthroughFromVisitor` | `struct` | `private` | named-field data structure; 2 fields; test-only | **Нет** | Не требуется | AST visitor accumulator; ограничения применяются при обходе, не через `TryFrom` | [`tests/src/code_style/mod.rs:1028`](../tests/src/code_style/mod.rs#L1028) |
| `PassthroughIntoInnerFromVisitor` | `struct` | `private` | named-field data structure; 2 fields; test-only | **Нет** | Не требуется | AST visitor accumulator; ограничения применяются при обходе, не через `TryFrom` | [`tests/src/code_style/mod.rs:956`](../tests/src/code_style/mod.rs#L956) |
| `ProductionStringLiteralVisitor` | `struct` | `private` | named-field data structure; 1 fields; test-only | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`tests/src/code_style/mod.rs:1130`](../tests/src/code_style/mod.rs#L1130) |
| `PublicStructFieldVisitor` | `struct` | `private` | named-field data structure; 1 fields; test-only | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`tests/src/code_style/mod.rs:140`](../tests/src/code_style/mod.rs#L140) |
| `PublicTupleWrapperFieldVisitor` | `struct` | `private` | named-field data structure; 1 fields; test-only | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`tests/src/code_style/mod.rs:1600`](../tests/src/code_style/mod.rs#L1600) |
| `RuntimeArcVisitor` | `struct` | `private` | named-field data structure; 2 fields; test-only | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`tests/src/code_style/mod.rs:271`](../tests/src/code_style/mod.rs#L271) |
| `RuntimeMutexVisitor` | `struct` | `private` | named-field data structure; 1 fields; test-only | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`tests/src/code_style/mod.rs:249`](../tests/src/code_style/mod.rs#L249) |
| `RuntimePanicExpectUnwrapVisitor` | `struct` | `private` | named-field data structure; 1 fields; test-only | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`tests/src/code_style/mod.rs:219`](../tests/src/code_style/mod.rs#L219) |
| `RustOrClippy` | `enum` | `private` | closed variant set; 2 variants; test-only | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`tests/src/code_style/mod.rs:28`](../tests/src/code_style/mod.rs#L28) |
| `SerdeJsonValueFieldVisitor` | `struct` | `private` | named-field data structure; 1 fields; test-only | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`tests/src/code_style/mod.rs:111`](../tests/src/code_style/mod.rs#L111) |
| `SerdeJsonValueTypeVisitor` | `struct` | `private` | named-field data structure; 1 fields; test-only | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`tests/src/code_style/mod.rs:135`](../tests/src/code_style/mod.rs#L135) |
| `SourceDroppingMapErrVisitor` | `struct` | `private` | named-field data structure; 1 fields; test-only | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`tests/src/code_style/mod.rs:89`](../tests/src/code_style/mod.rs#L89) |
| `StringConstantVisitor` | `struct` | `private` | named-field data structure; 1 fields; test-only | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`tests/src/code_style/mod.rs:1170`](../tests/src/code_style/mod.rs#L1170) |
| `StringWrapperFromVisitor` | `struct` | `private` | named-field data structure; 5 fields; test-only | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`tests/src/code_style/mod.rs:1398`](../tests/src/code_style/mod.rs#L1398) |
| `StringWrapperNameVisitor` | `struct` | `private` | named-field data structure; 1 fields; test-only | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`tests/src/code_style/mod.rs:1387`](../tests/src/code_style/mod.rs#L1387) |
| `TestNondeterminismVisitor` | `struct` | `private` | named-field data structure; 2 fields; test-only | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`tests/src/code_style/mod.rs:482`](../tests/src/code_style/mod.rs#L482) |
| `TestStringLiteralVisitor` | `struct` | `private` | named-field data structure; 1 fields; test-only | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`tests/src/code_style/mod.rs:1119`](../tests/src/code_style/mod.rs#L1119) |
| `TodoUnimplVisitor` | `struct` | `private` | named-field data structure; 2 fields; test-only | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`tests/src/code_style/mod.rs:55`](../tests/src/code_style/mod.rs#L55) |
| `TupleWrapperConversionCollector` | `struct` | `private` | named-field data structure; 2 fields; test-only | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`tests/src/code_style/mod.rs:1613`](../tests/src/code_style/mod.rs#L1613) |
| `TypeAliasVisitor` | `struct` | `private` | named-field data structure; 1 fields; test-only | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`tests/src/code_style/mod.rs:593`](../tests/src/code_style/mod.rs#L593) |
| `UnboundedReadVisitor` | `struct` | `private` | named-field data structure; 1 fields; test-only | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`tests/src/code_style/mod.rs:419`](../tests/src/code_style/mod.rs#L419) |
| `UnitTestExternalServiceVisitor` | `struct` | `private` | named-field data structure; 2 fields; test-only | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`tests/src/code_style/mod.rs:361`](../tests/src/code_style/mod.rs#L361) |
| `UnwrapVisitor` | `struct` | `private` | named-field data structure; 1 fields; test-only | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`tests/src/code_style/mod.rs:74`](../tests/src/code_style/mod.rs#L74) |
| `UseImportVisitor` | `struct` | `private` | named-field data structure; 4 fields; test-only | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`tests/src/code_style/mod.rs:530`](../tests/src/code_style/mod.rs#L530) |

### Модуль `tests::code_style::lint_sync`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `LintProbeDisposition` | `enum` | `private` | closed variant set; 3 variants; test-only | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`tests/src/code_style/lint_sync.rs:1`](../tests/src/code_style/lint_sync.rs#L1) |

### Модуль `tests::code_style::reuse_policy`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `FunctionBodyComplexity` | `struct` | `private` | named-field data structure; 1 fields; test-only | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`tests/src/code_style/reuse_policy.rs:7`](../tests/src/code_style/reuse_policy.rs#L7) |
| `FunctionBodyVisitor` | `struct` | `private` | named-field data structure; 3 fields; test-only | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`tests/src/code_style/reuse_policy.rs:1`](../tests/src/code_style/reuse_policy.rs#L1) |

### Модуль `tests::code_style::snapshot`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `CargoTomlSourceFile` | `struct` | `private` | named-field data structure; 3 fields; test-only | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`tests/src/code_style/snapshot.rs:10`](../tests/src/code_style/snapshot.rs#L10) |
| `CodebaseSnapshot` | `struct` | `restricted` | named-field data structure; 5 fields; test-only | **Нет** | Не требуется | внутренний агрегат анализа; входы ограничиваются при чтении файлов | [`tests/src/code_style/snapshot.rs:15`](../tests/src/code_style/snapshot.rs#L15) |
| `ProjectSourceFile` | `struct` | `restricted` | named-field data structure; 2 fields; test-only | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`tests/src/code_style/snapshot.rs:6`](../tests/src/code_style/snapshot.rs#L6) |
| `RsSourceFile` | `struct` | `restricted` | named-field data structure; 3 fields; test-only | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`tests/src/code_style/snapshot.rs:1`](../tests/src/code_style/snapshot.rs#L1) |

### Модуль `tests::code_style::source_policy`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `ReviewedPublicFields` | `struct` | `private` | named-field data structure; 4 fields; test-only | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`tests/src/code_style/source_policy.rs:132`](../tests/src/code_style/source_policy.rs#L132) |

### Модуль `tests::code_style::types`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `AnalyzerBool` | `struct` | `restricted` | single-field tuple wrapper; test-only | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`tests/src/code_style/types.rs:15`](../tests/src/code_style/types.rs#L15) |
| `AnalyzerChar` | `struct` | `restricted` | single-field tuple wrapper; test-only | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`tests/src/code_style/types.rs:32`](../tests/src/code_style/types.rs#L32) |
| `AnalyzerCount` | `struct` | `restricted` | single-field tuple wrapper; test-only | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`tests/src/code_style/types.rs:2`](../tests/src/code_style/types.rs#L2) |
| `CargoMetadata` | `struct` | `restricted` | single-field tuple wrapper; test-only | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`tests/src/code_style/types.rs:39`](../tests/src/code_style/types.rs#L39) |
| `CargoMetadataRef` | `struct` | `restricted` | single-field tuple wrapper; test-only | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`tests/src/code_style/types.rs:41`](../tests/src/code_style/types.rs#L41) |
| `CargoTomlFileIdx` | `struct` | `restricted` | single-field tuple wrapper; test-only | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`tests/src/code_style/types.rs:25`](../tests/src/code_style/types.rs#L25) |
| `DiagnosticMsgs` | `struct` | `restricted` | single-field tuple wrapper; test-only | **Нет** | Не требуется | отдельный инвариант этого служебного типа не требуется; проверка принадлежит итоговой доменной границе | [`tests/src/code_style/types.rs:82`](../tests/src/code_style/types.rs#L82) |
| `DiagnosticMsgsMutRef` | `struct` | `restricted` | single-field tuple wrapper; test-only | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`tests/src/code_style/types.rs:92`](../tests/src/code_style/types.rs#L92) |
| `FunctionBodyHash` | `struct` | `restricted` | single-field tuple wrapper; test-only | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`tests/src/code_style/types.rs:159`](../tests/src/code_style/types.rs#L159) |
| `RegexRegexRef` | `struct` | `restricted` | single-field tuple wrapper; test-only | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`tests/src/code_style/types.rs:161`](../tests/src/code_style/types.rs#L161) |
| `SourceText` | `struct` | `restricted` | single-field tuple wrapper; test-only | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`tests/src/code_style/types.rs:99`](../tests/src/code_style/types.rs#L99) |
| `SourceTextList` | `struct` | `restricted` | single-field tuple wrapper; test-only | **Нет** | Не требуется | отдельный инвариант этого служебного типа не требуется; проверка принадлежит итоговой доменной границе | [`tests/src/code_style/types.rs:131`](../tests/src/code_style/types.rs#L131) |
| `SourceTextListRef` | `struct` | `restricted` | single-field tuple wrapper; test-only | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`tests/src/code_style/types.rs:141`](../tests/src/code_style/types.rs#L141) |
| `SourceTextRef` | `struct` | `restricted` | single-field tuple wrapper; test-only | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`tests/src/code_style/types.rs:68`](../tests/src/code_style/types.rs#L68) |
| `SourceTextTryFromStringError` | `struct` | `restricted` | named-field data structure; 1 fields; test-only | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`tests/src/code_style/types.rs:101`](../tests/src/code_style/types.rs#L101) |
| `StaticStr` | `struct` | `restricted` | single-field tuple wrapper; test-only | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`tests/src/code_style/types.rs:54`](../tests/src/code_style/types.rs#L54) |
| `StaticStrSliceRef` | `struct` | `restricted` | single-field tuple wrapper; test-only | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`tests/src/code_style/types.rs:61`](../tests/src/code_style/types.rs#L61) |
| `StdCargoPackageIdRefSet` | `struct` | `restricted` | single-field tuple wrapper; test-only | **Нет** | Не требуется | отдельный инвариант этого служебного типа не требуется; проверка принадлежит итоговой доменной границе | [`tests/src/code_style/types.rs:48`](../tests/src/code_style/types.rs#L48) |
| `StdFunctionBodyLocationsMap` | `struct` | `restricted` | single-field tuple wrapper; test-only | **Нет** | Не требуется | отдельный инвариант этого служебного типа не требуется; проверка принадлежит итоговой доменной границе | [`tests/src/code_style/types.rs:163`](../tests/src/code_style/types.rs#L163) |
| `StdFunctionBodyLocationsMapMutRef` | `struct` | `restricted` | single-field tuple wrapper; test-only | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`tests/src/code_style/types.rs:167`](../tests/src/code_style/types.rs#L167) |
| `StdPathBuf` | `struct` | `restricted` | single-field tuple wrapper; test-only | **Нет** | Не требуется | отдельный инвариант этого служебного типа не требуется; проверка принадлежит итоговой доменной границе | [`tests/src/code_style/types.rs:180`](../tests/src/code_style/types.rs#L180) |
| `StdPathRef` | `struct` | `restricted` | single-field tuple wrapper; test-only | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`tests/src/code_style/types.rs:193`](../tests/src/code_style/types.rs#L193) |
| `StdProcessOutputRef` | `struct` | `restricted` | single-field tuple wrapper; test-only | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`tests/src/code_style/types.rs:52`](../tests/src/code_style/types.rs#L52) |
| `StdSourceTextHashSet` | `struct` | `restricted` | single-field tuple wrapper; test-only | **Нет** | Не требуется | отдельный инвариант этого служебного типа не требуется; проверка принадлежит итоговой доменной границе | [`tests/src/code_style/types.rs:78`](../tests/src/code_style/types.rs#L78) |
| `StdSourceTextRefSet` | `struct` | `restricted` | single-field tuple wrapper; test-only | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`tests/src/code_style/types.rs:76`](../tests/src/code_style/types.rs#L76) |
| `StdSourceTextSet` | `struct` | `restricted` | single-field tuple wrapper; test-only | **Нет** | Не требуется | отдельный инвариант этого служебного типа не требуется; проверка принадлежит итоговой доменной границе | [`tests/src/code_style/types.rs:148`](../tests/src/code_style/types.rs#L148) |
| `StdStdSourceTextSetRef` | `struct` | `restricted` | single-field tuple wrapper; test-only | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`tests/src/code_style/types.rs:178`](../tests/src/code_style/types.rs#L178) |
| `SynAttributeListRef` | `struct` | `restricted` | single-field tuple wrapper; test-only | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`tests/src/code_style/types.rs:201`](../tests/src/code_style/types.rs#L201) |
| `SynAttributeRef` | `struct` | `restricted` | single-field tuple wrapper; test-only | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`tests/src/code_style/types.rs:199`](../tests/src/code_style/types.rs#L199) |
| `SynBlockRef` | `struct` | `restricted` | single-field tuple wrapper; test-only | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`tests/src/code_style/types.rs:80`](../tests/src/code_style/types.rs#L80) |
| `SynExprCallRef` | `struct` | `restricted` | single-field tuple wrapper; test-only | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`tests/src/code_style/types.rs:203`](../tests/src/code_style/types.rs#L203) |
| `SynFieldsRef` | `struct` | `restricted` | single-field tuple wrapper; test-only | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`tests/src/code_style/types.rs:210`](../tests/src/code_style/types.rs#L210) |
| `SynFile` | `struct` | `restricted` | single-field tuple wrapper; test-only | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`tests/src/code_style/types.rs:195`](../tests/src/code_style/types.rs#L195) |
| `SynFileRef` | `struct` | `restricted` | single-field tuple wrapper; test-only | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`tests/src/code_style/types.rs:197`](../tests/src/code_style/types.rs#L197) |
| `SynGenericsRef` | `struct` | `restricted` | single-field tuple wrapper; test-only | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`tests/src/code_style/types.rs:212`](../tests/src/code_style/types.rs#L212) |
| `SynIdentifierRef` | `struct` | `restricted` | single-field tuple wrapper; test-only | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`tests/src/code_style/types.rs:251`](../tests/src/code_style/types.rs#L251) |
| `SynItemFnRef` | `struct` | `restricted` | single-field tuple wrapper; test-only | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`tests/src/code_style/types.rs:216`](../tests/src/code_style/types.rs#L216) |
| `SynItemImplRef` | `struct` | `restricted` | single-field tuple wrapper; test-only | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`tests/src/code_style/types.rs:214`](../tests/src/code_style/types.rs#L214) |
| `SynItemRef` | `struct` | `restricted` | single-field tuple wrapper; test-only | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`tests/src/code_style/types.rs:218`](../tests/src/code_style/types.rs#L218) |
| `SynItemStructRef` | `struct` | `restricted` | single-field tuple wrapper; test-only | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`tests/src/code_style/types.rs:220`](../tests/src/code_style/types.rs#L220) |
| `SynPathArgumentsRef` | `struct` | `restricted` | single-field tuple wrapper; test-only | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`tests/src/code_style/types.rs:222`](../tests/src/code_style/types.rs#L222) |
| `SynPathRef` | `struct` | `restricted` | single-field tuple wrapper; test-only | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`tests/src/code_style/types.rs:236`](../tests/src/code_style/types.rs#L236) |
| `SynPathSegmentRef` | `struct` | `restricted` | single-field tuple wrapper; test-only | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`tests/src/code_style/types.rs:224`](../tests/src/code_style/types.rs#L224) |
| `SynSignatureRef` | `struct` | `restricted` | single-field tuple wrapper; test-only | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`tests/src/code_style/types.rs:238`](../tests/src/code_style/types.rs#L238) |
| `SynTypePathRef` | `struct` | `restricted` | single-field tuple wrapper; test-only | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`tests/src/code_style/types.rs:240`](../tests/src/code_style/types.rs#L240) |
| `SynTypeRef` | `struct` | `restricted` | single-field tuple wrapper; test-only | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`tests/src/code_style/types.rs:247`](../tests/src/code_style/types.rs#L247) |
| `SynUseTreeRef` | `struct` | `restricted` | single-field tuple wrapper; test-only | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`tests/src/code_style/types.rs:249`](../tests/src/code_style/types.rs#L249) |
| `TomlTable` | `struct` | `restricted` | single-field tuple wrapper; test-only | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`tests/src/code_style/types.rs:258`](../tests/src/code_style/types.rs#L258) |
| `TomlTableRef` | `struct` | `restricted` | single-field tuple wrapper; test-only | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`tests/src/code_style/types.rs:260`](../tests/src/code_style/types.rs#L260) |
| `TomlValue` | `struct` | `restricted` | single-field tuple wrapper; test-only | **Нет** | Не требуется | отдельный инвариант этого служебного типа не требуется; проверка принадлежит итоговой доменной границе | [`tests/src/code_style/types.rs:274`](../tests/src/code_style/types.rs#L274) |
| `TomlValueRef` | `struct` | `restricted` | single-field tuple wrapper; test-only | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`tests/src/code_style/types.rs:267`](../tests/src/code_style/types.rs#L267) |
| `WalkdirWalkDir` | `struct` | `restricted` | single-field tuple wrapper; test-only | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`tests/src/code_style/types.rs:276`](../tests/src/code_style/types.rs#L276) |

### Модуль `tests::domain_type_policy_fixture`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `DomainEntity` | `struct` | `private` | named-field data structure; 2 fields; test-only | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`tests/src/domain_type_policy_fixture.rs:35`](../tests/src/domain_type_policy_fixture.rs#L35) |
| `DomainEvent` | `enum` | `private` | closed variant set; 1 variants; test-only | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`tests/src/domain_type_policy_fixture.rs:39`](../tests/src/domain_type_policy_fixture.rs#L39) |
| `DomainEvents` | `struct` | `private` | single-field tuple wrapper; test-only | **Нет** | Не требуется | отдельный инвариант этого служебного типа не требуется; проверка принадлежит итоговой доменной границе | [`tests/src/domain_type_policy_fixture.rs:42`](../tests/src/domain_type_policy_fixture.rs#L42) |
| `DomainId` | `struct` | `private` | single-field tuple wrapper; test-only | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`tests/src/domain_type_policy_fixture.rs:1`](../tests/src/domain_type_policy_fixture.rs#L1) |
| `DomainName` | `struct` | `private` | single-field tuple wrapper; test-only | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`tests/src/domain_type_policy_fixture.rs:3`](../tests/src/domain_type_policy_fixture.rs#L3) |
| `DomainNameTryFromStringError` | `enum` | `private` | error enum; 1 variants; test-only | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`tests/src/domain_type_policy_fixture.rs:5`](../tests/src/domain_type_policy_fixture.rs#L5) |

### Модуль `tests::trybuild::route_contract_catalog_missing_route`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `BrokenCatalog` | `enum` | `private` | closed variant set; 1 variants; test-only | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`tests/trybuild/route_contract_catalog_missing_route.rs:1`](../tests/trybuild/route_contract_catalog_missing_route.rs#L1) |

### Модуль `tests::trybuild::route_contract_page_catalog_non_unit`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `BrokenPages` | `enum` | `private` | closed variant set; 1 variants; test-only | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`tests/trybuild/route_contract_page_catalog_non_unit.rs:1`](../tests/trybuild/route_contract_page_catalog_non_unit.rs#L1) |

### Модуль `tests::trybuild::route_contract_wire_enum_duplicate`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `DuplicateWireValue` | `enum` | `private` | closed variant set; 2 variants; test-only | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`tests/trybuild/route_contract_wire_enum_duplicate.rs:1`](../tests/trybuild/route_contract_wire_enum_duplicate.rs#L1) |

### Модуль `tests::trybuild::route_contract_wire_enum_non_unit`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `NonUnitWireValue` | `enum` | `private` | closed variant set; 1 variants; test-only | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`tests/trybuild/route_contract_wire_enum_non_unit.rs:1`](../tests/trybuild/route_contract_wire_enum_non_unit.rs#L1) |

### Модуль `tests::trybuild::route_contract_wrong_family_empty`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `EmptyRouteFamily` | `struct` | `private` | unit marker type; test-only | **Нет** | Не требуется | marker не содержит проверяемых данных | [`tests/trybuild/route_contract_wrong_family_empty.rs:1`](../tests/trybuild/route_contract_wrong_family_empty.rs#L1) |

### Модуль `tests::trybuild::route_contract_wrong_family_missing_attribute`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `MissingAttributeRouteFamily` | `struct` | `private` | unit marker type; test-only | **Нет** | Не требуется | marker не содержит проверяемых данных | [`tests/trybuild/route_contract_wrong_family_missing_attribute.rs:1`](../tests/trybuild/route_contract_wrong_family_missing_attribute.rs#L1) |

### Модуль `tests::trybuild::route_contract_wrong_path_parameter`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `ParameterizedTestRoute` | `struct` | `private` | unit marker type; test-only | **Нет** | Не требуется | marker не содержит проверяемых данных | [`tests/trybuild/route_contract_wrong_path_parameter.rs:1`](../tests/trybuild/route_contract_wrong_path_parameter.rs#L1) |

### Модуль `tests::trybuild::route_contract_wrong_request`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `Request` | `struct` | `private` | unit marker type; test-only | **Нет** | Не требуется | marker не содержит проверяемых данных | [`tests/trybuild/route_contract_wrong_request.rs:1`](../tests/trybuild/route_contract_wrong_request.rs#L1) |
| `Response` | `struct` | `private` | unit marker type; test-only | **Нет** | Не требуется | marker не содержит проверяемых данных | [`tests/trybuild/route_contract_wrong_request.rs:3`](../tests/trybuild/route_contract_wrong_request.rs#L3) |
| `Route` | `struct` | `private` | unit marker type; test-only | **Нет** | Не требуется | marker не содержит проверяемых данных | [`tests/trybuild/route_contract_wrong_request.rs:5`](../tests/trybuild/route_contract_wrong_request.rs#L5) |

### Модуль `tests::trybuild::route_contract_wrong_response`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `Request` | `struct` | `private` | unit marker type; test-only | **Нет** | Не требуется | marker не содержит проверяемых данных | [`tests/trybuild/route_contract_wrong_response.rs:1`](../tests/trybuild/route_contract_wrong_response.rs#L1) |
| `Response` | `struct` | `private` | unit marker type; test-only | **Нет** | Не требуется | marker не содержит проверяемых данных | [`tests/trybuild/route_contract_wrong_response.rs:3`](../tests/trybuild/route_contract_wrong_response.rs#L3) |
| `Route` | `struct` | `private` | unit marker type; test-only | **Нет** | Не требуется | marker не содержит проверяемых данных | [`tests/trybuild/route_contract_wrong_response.rs:5`](../tests/trybuild/route_contract_wrong_response.rs#L5) |

### Модуль `tests::trybuild::route_contract_wrong_route`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `FirstRequest` | `struct` | `private` | unit marker type; test-only | **Нет** | Не требуется | marker не содержит проверяемых данных | [`tests/trybuild/route_contract_wrong_route.rs:1`](../tests/trybuild/route_contract_wrong_route.rs#L1) |
| `FirstRoute` | `struct` | `private` | unit marker type; test-only | **Нет** | Не требуется | marker не содержит проверяемых данных | [`tests/trybuild/route_contract_wrong_route.rs:7`](../tests/trybuild/route_contract_wrong_route.rs#L7) |
| `Response` | `struct` | `private` | unit marker type; test-only | **Нет** | Не требуется | marker не содержит проверяемых данных | [`tests/trybuild/route_contract_wrong_route.rs:5`](../tests/trybuild/route_contract_wrong_route.rs#L5) |
| `SecondRequest` | `struct` | `private` | unit marker type; test-only | **Нет** | Не требуется | marker не содержит проверяемых данных | [`tests/trybuild/route_contract_wrong_route.rs:3`](../tests/trybuild/route_contract_wrong_route.rs#L3) |
| `SecondRoute` | `struct` | `private` | unit marker type; test-only | **Нет** | Не требуется | marker не содержит проверяемых данных | [`tests/trybuild/route_contract_wrong_route.rs:8`](../tests/trybuild/route_contract_wrong_route.rs#L8) |

### Модуль `tests::trybuild::route_contract_wrong_transport`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `AuthenticatedRoute` | `struct` | `private` | unit marker type; test-only | **Нет** | Не требуется | marker не содержит проверяемых данных | [`tests/trybuild/route_contract_wrong_transport.rs:10`](../tests/trybuild/route_contract_wrong_transport.rs#L10) |
| `Request` | `struct` | `private` | unit marker type; test-only | **Нет** | Не требуется | marker не содержит проверяемых данных | [`tests/trybuild/route_contract_wrong_transport.rs:6`](../tests/trybuild/route_contract_wrong_transport.rs#L6) |
| `Response` | `struct` | `private` | unit marker type; test-only | **Нет** | Не требуется | marker не содержит проверяемых данных | [`tests/trybuild/route_contract_wrong_transport.rs:8`](../tests/trybuild/route_contract_wrong_transport.rs#L8) |

## Crate `text_policy`

### Модуль `text_policy`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `BoundedTextPolicyError` | `enum` | `public` | error enum; 3 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`text_policy/src/lib.rs:9`](../text_policy/src/lib.rs#L9) |
| `FixedLengthAsciiHexText` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`text_policy/src/lib.rs:63`](../text_policy/src/lib.rs#L63) |
| `FixedLengthAsciiHexTextError` | `enum` | `public` | error enum; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`text_policy/src/lib.rs:56`](../text_policy/src/lib.rs#L56) |
| `NonEmptyTrimmedText` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`text_policy/src/lib.rs:37`](../text_policy/src/lib.rs#L37) |
| `PasswordLength` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`text_policy/src/lib.rs:136`](../text_policy/src/lib.rs#L136) |
| `PasswordLengthRange` | `struct` | `public` | named-field data structure; 2 fields; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`text_policy/src/lib.rs:139`](../text_policy/src/lib.rs#L139) |
| `PasswordLengthRangeError` | `struct` | `public` | unit marker type; production | **Нет** | Не требуется | marker не содержит проверяемых данных | [`text_policy/src/lib.rs:150`](../text_policy/src/lib.rs#L150) |
| `PasswordPolicyViolation` | `enum` | `public` | closed variant set; 7 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`text_policy/src/lib.rs:166`](../text_policy/src/lib.rs#L166) |
| `PasswordTextRef` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`text_policy/src/lib.rs:133`](../text_policy/src/lib.rs#L133) |
| `RequiredNulFreeBoundedText` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`text_policy/src/lib.rs:19`](../text_policy/src/lib.rs#L19) |
| `UrlSafeTokenPartMaximumBytes` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`text_policy/src/lib.rs:81`](../text_policy/src/lib.rs#L81) |
| `UrlSafeTokenPartRef` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`text_policy/src/lib.rs:84`](../text_policy/src/lib.rs#L84) |
| `UrlSafeTokenPartText` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`text_policy/src/lib.rs:97`](../text_policy/src/lib.rs#L97) |
| `UrlSafeTokenPartTextError` | `enum` | `public` | error enum; 3 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`text_policy/src/lib.rs:87`](../text_policy/src/lib.rs#L87) |

## Crate `to_err_string`

### Модуль `to_err_string`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `ErrorText` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | атрибут валидатора задаёт инвариант типа | [`to_err_string/src/lib.rs:31`](../to_err_string/src/lib.rs#L31) |
| `StaticStrToOwnedInput` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`to_err_string/src/lib.rs:79`](../to_err_string/src/lib.rs#L79) |
| `ToErrString` | `trait` | `public` | behavior contract; 1 associated items; production | **Нет** | Не требуется | trait не инициализируется как значение | [`to_err_string/src/lib.rs:28`](../to_err_string/src/lib.rs#L28) |

## Crate `token_patterns`

### Модуль `token_patterns`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `ProcMacro2TokensMut` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`token_patterns/src/lib.rs:182`](../token_patterns/src/lib.rs#L182) |

## Crate `token_patterns_macros`

### Модуль `token_patterns_macros`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `ProcMacro2GenerateTpInput` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | отдельный инвариант этого служебного типа не требуется; проверка принадлежит итоговой доменной границе | [`token_patterns/token_patterns_macros/src/lib.rs:1`](../token_patterns/token_patterns_macros/src/lib.rs#L1) |
| `ProcMacro2GenerateTpOutput` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | отдельный инвариант этого служебного типа не требуется; проверка принадлежит итоговой доменной границе | [`token_patterns/token_patterns_macros/src/lib.rs:4`](../token_patterns/token_patterns_macros/src/lib.rs#L4) |

## Crate `where_filters`

### Модуль `where_filters`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `Between` | `struct` | `public` | named-field data structure; 2 fields; production | **Да** | Выполнено | fallible-конструктор показывает наличие инварианта | [`pg_crud/where_filters/src/lib.rs:164`](../pg_crud/where_filters/src/lib.rs#L164) |
| `BetweenTryNewError` | `enum` | `public` | error enum; 1 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`pg_crud/where_filters/src/lib.rs:201`](../pg_crud/where_filters/src/lib.rs#L201) |
| `BoundedVec` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`pg_crud/where_filters/src/lib.rs:627`](../pg_crud/where_filters/src/lib.rs#L627) |
| `BoundedVecLen` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`pg_crud/where_filters/src/lib.rs:669`](../pg_crud/where_filters/src/lib.rs#L669) |
| `BoundedVecTryNewError` | `enum` | `public` | error enum; 1 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`pg_crud/where_filters/src/lib.rs:648`](../pg_crud/where_filters/src/lib.rs#L648) |
| `DefaultRegexPattern` | `struct` | `private` | unit marker type; production | **Нет** | Не требуется | marker не содержит проверяемых данных | [`pg_crud/where_filters/src/lib.rs:53`](../pg_crud/where_filters/src/lib.rs#L53) |
| `EncodeFormat` | `enum` | `public` | closed variant set; 3 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`pg_crud/where_filters/src/lib.rs:6`](../pg_crud/where_filters/src/lib.rs#L6) |
| `PgTypeNotEmptyUniqueVec` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`pg_crud/where_filters/src/lib.rs:492`](../pg_crud/where_filters/src/lib.rs#L492) |
| `RegexCase` | `enum` | `public` | closed variant set; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`pg_crud/where_filters/src/lib.rs:122`](../pg_crud/where_filters/src/lib.rs#L122) |
| `RegexCasePostgreqlSyntax` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`pg_crud/where_filters/src/lib.rs:138`](../pg_crud/where_filters/src/lib.rs#L138) |
| `RegexError` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`pg_crud/where_filters/src/lib.rs:60`](../pg_crud/where_filters/src/lib.rs#L60) |
| `RegexRegex` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`pg_crud/where_filters/src/lib.rs:39`](../pg_crud/where_filters/src/lib.rs#L39) |
| `RegexRegexTryFromStringError` | `enum` | `public` | error enum; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`pg_crud/where_filters/src/lib.rs:62`](../pg_crud/where_filters/src/lib.rs#L62) |
| `Variant` | `enum` | `private` | closed variant set; 2 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`pg_crud/where_filters/src/lib.rs:696`](../pg_crud/where_filters/src/lib.rs#L696) |

### Модуль `where_filters::tests`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `NonClone` | `struct` | `private` | single-field tuple wrapper; test-only | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`pg_crud/where_filters/src/lib.rs:793`](../pg_crud/where_filters/src/lib.rs#L793) |

## Crate `workspace_macro_helpers`

### Модуль `workspace_macro_helpers`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `FirstCommaStripped` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`workspace_macro_helpers/src/lib.rs:328`](../workspace_macro_helpers/src/lib.rs#L328) |
| `FirstIdentifier` | `struct` | `public` | single-field tuple wrapper; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`workspace_macro_helpers/src/lib.rs:224`](../workspace_macro_helpers/src/lib.rs#L224) |
| `FirstIdentifierifierTryFromStringError` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`workspace_macro_helpers/src/lib.rs:227`](../workspace_macro_helpers/src/lib.rs#L227) |
| `PartIndex` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`workspace_macro_helpers/src/lib.rs:342`](../workspace_macro_helpers/src/lib.rs#L342) |
| `ProcMacro2MacroTokens` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | отдельный инвариант этого служебного типа не требуется; проверка принадлежит итоговой доменной границе | [`workspace_macro_helpers/src/lib.rs:75`](../workspace_macro_helpers/src/lib.rs#L75) |
| `ProcMacro2TopLevelCommaParts` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | отдельный инвариант этого служебного типа не требуется; проверка принадлежит итоговой доменной границе | [`workspace_macro_helpers/src/lib.rs:140`](../workspace_macro_helpers/src/lib.rs#L140) |
| `StdUniqueOptionSet` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | отдельный инвариант этого служебного типа не требуется; проверка принадлежит итоговой доменной границе | [`workspace_macro_helpers/src/lib.rs:262`](../workspace_macro_helpers/src/lib.rs#L262) |
| `StdUniqueOptionSetContains` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`workspace_macro_helpers/src/lib.rs:271`](../workspace_macro_helpers/src/lib.rs#L271) |
| `StdUniqueOptionSetIsEmpty` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`workspace_macro_helpers/src/lib.rs:284`](../workspace_macro_helpers/src/lib.rs#L284) |
| `SynDeriveInputRef` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`workspace_macro_helpers/src/lib.rs:2`](../workspace_macro_helpers/src/lib.rs#L2) |
| `SynFieldsNamedRef` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`workspace_macro_helpers/src/lib.rs:21`](../workspace_macro_helpers/src/lib.rs#L21) |
| `SynFieldsUnnamedRef` | `struct` | `public` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`workspace_macro_helpers/src/lib.rs:40`](../workspace_macro_helpers/src/lib.rs#L40) |
| `SynStructShapeRef` | `enum` | `public` | closed variant set; 3 variants; production | **Да** | Выполнено | существующий `TryFrom` уже фиксирует fallible-инвариант | [`workspace_macro_helpers/src/lib.rs:15`](../workspace_macro_helpers/src/lib.rs#L15) |
| `TopLevelCommaPart` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`workspace_macro_helpers/src/lib.rs:178`](../workspace_macro_helpers/src/lib.rs#L178) |

## Crate `workspace_scaffold`

### Модуль `workspace_scaffold`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `ProjectNameRef` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`workspace_scaffold/src/main.rs:2`](../workspace_scaffold/src/main.rs#L2) |
| `ReplacementsRef` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`workspace_scaffold/src/main.rs:17`](../workspace_scaffold/src/main.rs#L17) |
| `RepositoryUrlRef` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`workspace_scaffold/src/main.rs:5`](../workspace_scaffold/src/main.rs#L5) |
| `ScaffoldError` | `enum` | `private` | error enum; 8 variants; production | **Нет** | Не требуется | закрытые variants сами задают допустимые состояния | [`workspace_scaffold/src/main.rs:26`](../workspace_scaffold/src/main.rs#L26) |
| `ScaffoldText` | `struct` | `private` | single-field tuple wrapper; production | **Да** | Выполнено | атрибут валидатора задаёт инвариант типа | [`workspace_scaffold/src/main.rs:10`](../workspace_scaffold/src/main.rs#L10) |
| `ScaffoldTextRef` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`workspace_scaffold/src/main.rs:13`](../workspace_scaffold/src/main.rs#L13) |
| `ServerRuntimeBoundedReadError` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`workspace_scaffold/src/main.rs:23`](../workspace_scaffold/src/main.rs#L23) |
| `ServicePort` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`workspace_scaffold/src/main.rs:8`](../workspace_scaffold/src/main.rs#L8) |
| `ShouldSkip` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`workspace_scaffold/src/main.rs:19`](../workspace_scaffold/src/main.rs#L19) |
| `StdScaffoldIoError` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`workspace_scaffold/src/main.rs:21`](../workspace_scaffold/src/main.rs#L21) |
| `StdScaffoldPathRef` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`workspace_scaffold/src/main.rs:15`](../workspace_scaffold/src/main.rs#L15) |

## Crate `workspace_test_runner`

### Модуль `workspace_test_runner`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `AdminFixtureString` | `struct` | `private` | single-field tuple wrapper; production | **Да** | Выполнено | атрибут валидатора задаёт инвариант типа | [`workspace_test_runner/src/main.rs:182`](../workspace_test_runner/src/main.rs#L182) |
| `AllocationTool` | `struct` | `private` | named-field data structure; 2 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`workspace_test_runner/src/main.rs:185`](../workspace_test_runner/src/main.rs#L185) |
| `AnsiTextRef` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`workspace_test_runner/src/main.rs:78`](../workspace_test_runner/src/main.rs#L78) |
| `CargoArgs` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`workspace_test_runner/src/main.rs:59`](../workspace_test_runner/src/main.rs#L59) |
| `CleanAnsiText` | `struct` | `private` | single-field tuple wrapper; production | **Да** | Выполнено | атрибут валидатора задаёт инвариант типа | [`workspace_test_runner/src/main.rs:85`](../workspace_test_runner/src/main.rs#L85) |
| `MeasurementName` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`workspace_test_runner/src/main.rs:52`](../workspace_test_runner/src/main.rs#L52) |
| `MemusageColumnIdx` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`workspace_test_runner/src/main.rs:102`](../workspace_test_runner/src/main.rs#L102) |
| `MemusageKey` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`workspace_test_runner/src/main.rs:88`](../workspace_test_runner/src/main.rs#L88) |
| `MemusageProgNameRef` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`workspace_test_runner/src/main.rs:135`](../workspace_test_runner/src/main.rs#L135) |
| `MemusageRowName` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`workspace_test_runner/src/main.rs:95`](../workspace_test_runner/src/main.rs#L95) |
| `MemusageValueRef` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`workspace_test_runner/src/main.rs:109`](../workspace_test_runner/src/main.rs#L109) |
| `ProgramArgsRef` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`workspace_test_runner/src/main.rs:123`](../workspace_test_runner/src/main.rs#L123) |
| `ProgramPathRef` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`workspace_test_runner/src/main.rs:116`](../workspace_test_runner/src/main.rs#L116) |
| `QuoteTokenStreamGeneratePgTableMeasureInputTokenStream` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | отдельный инвариант этого служебного типа не требуется; проверка принадлежит итоговой доменной границе | [`workspace_test_runner/src/main.rs:142`](../workspace_test_runner/src/main.rs#L142) |
| `RunnerMode` | `struct` | `private` | single-field tuple wrapper; production | **Да** | Выполнено | атрибут валидатора задаёт инвариант типа | [`workspace_test_runner/src/main.rs:179`](../workspace_test_runner/src/main.rs#L179) |
| `StdRunnerIoErrorRef` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`workspace_test_runner/src/main.rs:165`](../workspace_test_runner/src/main.rs#L165) |
| `StdRunnerPathRef` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`workspace_test_runner/src/main.rs:172`](../workspace_test_runner/src/main.rs#L172) |
| `StderrTextRef` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`workspace_test_runner/src/main.rs:71`](../workspace_test_runner/src/main.rs#L71) |
| `ToolAvailable` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`workspace_test_runner/src/main.rs:158`](../workspace_test_runner/src/main.rs#L158) |
| `ToolName` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`workspace_test_runner/src/main.rs:144`](../workspace_test_runner/src/main.rs#L144) |
| `ToolPath` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`workspace_test_runner/src/main.rs:151`](../workspace_test_runner/src/main.rs#L151) |

### Модуль `workspace_test_runner::execution`

| Тип | Вид | Видимость | Анализ | TryFrom | Статус | Обоснование | Источник |
|---|---|---|---|:---:|---|---|---|
| `CommandArgsRef` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`workspace_test_runner/src/execution.rs:51`](../workspace_test_runner/src/execution.rs#L51) |
| `CommandDurationMillis` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`workspace_test_runner/src/execution.rs:25`](../workspace_test_runner/src/execution.rs#L25) |
| `CommandIdx` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`workspace_test_runner/src/execution.rs:4`](../workspace_test_runner/src/execution.rs#L4) |
| `CommandProgramRef` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`workspace_test_runner/src/execution.rs:49`](../workspace_test_runner/src/execution.rs#L49) |
| `CommandRun` | `struct` | `private` | named-field data structure; 5 fields; production | **Нет** | Не требуется | fields типизированы; отдельный инвариант этого типа не обнаружен | [`workspace_test_runner/src/execution.rs:86`](../workspace_test_runner/src/execution.rs#L86) |
| `CommandSucceeded` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`workspace_test_runner/src/execution.rs:27`](../workspace_test_runner/src/execution.rs#L27) |
| `CommandText` | `struct` | `private` | single-field tuple wrapper; production | **Да** | Выполнено | атрибут валидатора задаёт инвариант типа | [`workspace_test_runner/src/execution.rs:53`](../workspace_test_runner/src/execution.rs#L53) |
| `CommandTexts` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | отдельный инвариант этого служебного типа не требуется; проверка принадлежит итоговой доменной границе | [`workspace_test_runner/src/execution.rs:56`](../workspace_test_runner/src/execution.rs#L56) |
| `CommandsRef` | `struct` | `restricted` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`workspace_test_runner/src/execution.rs:34`](../workspace_test_runner/src/execution.rs#L34) |
| `StdCommandDuration` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`workspace_test_runner/src/execution.rs:18`](../workspace_test_runner/src/execution.rs#L18) |
| `StdCommandStartedAt` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`workspace_test_runner/src/execution.rs:11`](../workspace_test_runner/src/execution.rs#L11) |
| `StdExecutionIoError` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`workspace_test_runner/src/execution.rs:58`](../workspace_test_runner/src/execution.rs#L58) |
| `StdRunDir` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | отдельный инвариант этого служебного типа не требуется; проверка принадлежит итоговой доменной границе | [`workspace_test_runner/src/execution.rs:67`](../workspace_test_runner/src/execution.rs#L67) |
| `SummaryText` | `struct` | `private` | single-field tuple wrapper; production | **Да** | Выполнено | атрибут валидатора задаёт инвариант типа | [`workspace_test_runner/src/execution.rs:69`](../workspace_test_runner/src/execution.rs#L69) |
| `TextRef` | `struct` | `private` | single-field tuple wrapper; production | **Нет** | Не требуется | inner фиксирован, заимствован или уже валидирован своим типом | [`workspace_test_runner/src/execution.rs:60`](../workspace_test_runner/src/execution.rs#L60) |

## Проверка полноты

Каталог сформирован синтаксическим разбором каждого `.rs`-файла в workspace. Итоговое число строк таблиц должно совпадать с числом найденных module-level type items; повторяющиеся имена допустимы, если они объявлены в разных модулях.
