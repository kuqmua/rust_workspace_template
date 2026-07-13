# План переиспользования кода по модулям

## 1. Цель

Сократить повторение типовой логики в workspace, сохранив строгие границы между доменными типами, генераторами, PostgreSQL, HTTP API, frontend и runtime. Переиспользуемая логика должна находиться у одного явного владельца, а прикладные crates должны передавать владельцу только доменную конфигурацию.

План рассчитан на постепенное внедрение без массовой смены публичного API. Каждый этап должен быть отдельным изменением с собственными тестами и измеримым уменьшением дублирования.

## Статус выполнения

| Этап | Статус | Подтверждение |
|---|---|---|
| 0. Baseline и inventory | Completed | Зафиксированы размеры генераторов и количество механических impl; обязательный baseline был зелёным до начала рефакторинга |
| 1. Wrapper generation | Completed | Добавлены owned unsized target, borrowed slice и secret policy; `app_state` и `pg_tbl` являются production consumers |
| 2. Table operation single source of truth | Completed | `OpDsc` владеет HTTP method, status, permission action, operation kind и capabilities; generated route/client/OpenAPI/negative tests проходят |
| 3. PostgreSQL type/filter model | Completed | `PgTypeSpec` покрывает PostgreSQL type capabilities; `FilterSpec` владеет equality/text/range SQL metadata, bind count и value shape; generated-crate tests проходят |
| 4. Runtime/API/frontend reuse | Completed | Общий success/problem interpreter имеет native/generated/browser consumers; runtime и admin domain разделены по private-модулям с сохранением facade API |
| 5. Cleanup и hardening | Completed | Удалены эквивалентные helpers, проверены public API/features/dependencies, обновлён README и пройден полный workspace gate |

### Baseline inventory

| Метрика | Исходное значение |
|---|---:|
| Строки в `gen_pg_tbl_src`, `gen_pg_types_src`, `gen_wh_flts_src` | 15 090 |
| Ручные `From` impl | 109 |
| Ручные `AsRef` impl | 30 |
| Ручные `Debug` impl | 17 |
| Ручные `Display` impl | 58 |
| Route literals в `gen_pg_tbl_src` | 18 |
| Упоминания `StatusCode` в `gen_pg_tbl_src` | 36 |
| Потребители `newtype::Newtype` | 257 |

Значения являются baseline для сравнения, а не самостоятельной целью. Уменьшение строк или impl считается улучшением только при сохранении доменной семантики, public API и проверяемого поведения.

## 2. Ограничения

- Не создавать новые crates без отдельного явного запроса. Общую логику размещать в существующих shared crates.
- Не объединять независимые crates и не переносить PostgreSQL-зависимости в общие contract crates.
- Не добавлять зависимости только ради сокращения нескольких строк. Сначала использовать `std`, `syn`, `quote`, SQLx, Serde и Utoipa, уже принадлежащие соответствующему слою.
- Не генерировать бизнес-правила, которые различаются между доменами. Генерировать только механические реализации и контракты.
- Не менять wire format, SQL semantics и публичные имена без отдельного миграционного этапа.
- Все crates.io зависимости объявлять только в `[workspace.dependencies]`, а workspace crates подключать через `crate_name = { workspace = true }`.
- Публичные границы должны использовать repository domain wrappers, а не primitive или внешние типы.
- Не создавать универсальный helper до появления как минимум двух реальных потребителей.

## 3. Критерии выбора логики для переиспользования

Код переносится к общему владельцу, если одновременно выполняются следующие условия:

1. Есть минимум два реальных потребителя или один генератор повторяет шаблон для нескольких категорий типов.
2. Поведение совпадает, включая ошибки, ограничения, Serde, SQL и OpenAPI.
3. Общая реализация не требует передавать прикладному слою детали другого домена.
4. После переноса публичная поверхность не становится шире без необходимости.
5. Общую реализацию можно проверить unit-тестами независимо от конкретного сервиса.

Не следует объединять код только из-за похожего синтаксиса. Например, `From<Owned>` и `From<&T>` можно генерировать общей механикой, но правила валидации `AdminLogin` и `PgTblIdempotencyKey` должны оставаться у своих доменных владельцев.

## 4. Целевая схема ответственности

| Слой | Владелец | Что переиспользуется | Что остаётся у потребителя |
|---|---|---|---|
| Простые newtype | `newtype` | механические impl, ограничения строки, Serde/OpenAPI parity | имя и доменная политика типа |
| Proc-macro utilities | `workspace_macro_helpers`, `macros_helpers`, `gen_quotes` | parsing, diagnostics, token construction, generated-test utilities | конкретная модель атрибутов macro |
| PostgreSQL CRUD primitives | `pg_crud_cmn` | pagination, bounded collections, bind counter, operators, query wrappers | таблица, поля и разрешённые операции |
| PostgreSQL типы | `gen_pg_types_src`, `pg_types_cmn` | единая type model и генерация повторяемых impl | категория SQL-типа и специальные invariants |
| PostgreSQL filters | `wh_flts`, `gen_wh_flts_src` | filter operators, escaped patterns, bind/SQL parity | список поддерживаемых фильтров типа |
| Табличный API | `pg_tbl`, `gen_pg_tbl_src` | route model, SQL assembly, handlers, clients, OpenAPI, frontend metadata | declarative table configuration |
| HTTP contracts | `frontend_contract`, `server_admin_contract` | route metadata, problem responses, transport contracts | административные request/response models |
| Runtime | `server_runtime` | lifecycle, limits, budgets, request metadata, security layers | service composition and configured values |
| Конфигурация | `config_lib`, `server_config` | env parsing and validated config types | composition of one executable config |
| Application state | `app_state`, `server_app_state` | pool wrappers and capability traits | concrete state fields required by server |
| Admin domain | `server_admin` | reusable admin API, auth, cleanup and migrations | executable mounting and environment |
| Frontend | `server_admin_frontend` | typed transport, authentication coordination, reusable UI primitives | page composition and domain-specific forms |
| Проверки | `macros_helpers`, `tests`, `workspace_test_runner` | JSON/OpenAPI/DB guards and policy gates | local fixtures and domain assertions |

## 5. План по модулям

### 5.1. `newtype`: механическая генерация wrapper implementations

#### Текущее назначение

`newtype` уже генерирует bounded string types и связывает runtime validation с Serde и Utoipa. В workspace при этом остаются повторяемые ручные реализации для owned wrappers, borrowed wrappers и slice wrappers.

#### Что вынести в переиспользуемую генерацию

Добавить опции к существующим derive, не создавая новый macro crate:

- `From<Inner> for Wrapper` для owned tuple wrapper;
- `AsRef<Inner>` с возвратом `&self.0`;
- `From<&'a Inner> for WrapperRef<'a>`;
- `AsRef<Inner>` для reference wrapper с возвратом `self.0`;
- поддержка unsized inner types, прежде всего `[T]` и `str`;
- делегирующий `Debug`, когда нужен именно формат inner value;
- делегирующий `Display`, только если он является частью доменного контракта;
- опциональные `Borrow`, `AsMut` и `From<Wrapper> for Inner`, только по явному атрибуту.

Не генерировать `Deref` по умолчанию: он размывает доменную границу и позволяет обходить намеренно минимальный API wrapper.

#### Первые потребители

- `app_state::SqlxPgPool` и `SqlxPgPoolRef`;
- reference/slice wrappers в `pg_tbl`;
- generated wrappers в `gen_pg_types_src`;
- contract string wrappers в `frontend_contract` и `server_admin_contract` после проверки wire compatibility.

#### Критерии приёмки

- Compile-pass покрывает owned, borrowed, `str` и `[T]`.
- Compile-error покрывает несовместимые опции, отсутствующее tuple field и попытку `AsMut` для immutable reference.
- Сгенерированный `Debug` для secret wrapper запрещён или требует отдельной redaction policy.
- Публичные сигнатуры первых потребителей не меняются.
- После миграции удалены только эквивалентные ручные impl.

### 5.2. `workspace_macro_helpers`: общий parsing foundation

#### Текущее назначение

Crate должен владеть низкоуровневой логикой proc-macro, не зависящей от PostgreSQL, HTTP или конкретного derive.

#### Что переиспользовать

- извлечение named fields и tuple inner type;
- проверка дубликатов атрибутов и неизвестных ключей;
- typed span-aware diagnostics с постоянным восьмисимвольным идентификатором;
- helpers для optional/list/name-value meta;
- проверка lifetime для borrowed wrappers;
- построение compile-error token stream без строковой конкатенации в потребителях.

#### Потребители

`newtype`, `config_lib_macros`, `server_app_state_macros`, `token_patterns_macros`, `naming_macros`, `gen_pg_types_src`, `gen_wh_flts_src` и `gen_pg_tbl_src`.

#### Граница

Здесь не должно быть типов вроде `PgType`, `AdminRoute` или `UtoipaSchema`. Helper возвращает синтаксическую модель, а смысл атрибута интерпретирует consuming macro.

#### Критерии приёмки

- Два macro consumer используют одинаковый parser дубликатов и неизвестных опций.
- Ошибка сохраняет исходный `Span`.
- Snapshot/compile-error сообщения стабильны.
- Crate не получает runtime dependencies.

### 5.3. `macros_helpers`: общая модель генерации и test support

#### Что централизовать

- field inventory поверх `syn::Field`;
- reusable builders для derive/visibility/enum/struct tokens;
- Rust source formatting и запись generated fixtures;
- JSON contract round-trip helper;
- guarded test database URL;
- общие status-code и error-response descriptors;
- helper проверки generated source отдельным временным crate.

#### Что не переносить

Route inventory и SQL operation model принадлежат `gen_pg_tbl_src`. PostgreSQL type categories принадлежат `gen_pg_types_src`.

#### Рефакторинг модулей

Разделить публичность существующих модулей по потребителям:

- `ast` — field/type inspection;
- `tokens` — механические builders;
- `generated_fixture` — запись, format, cargo check/clippy;
- `json_contract` — только под `test-utils`;
- `test_database` — только под `test-utils`;
- `http_contract` — status/content-type descriptors без Axum handler logic.

Использовать внутренние модули существующего crate; новый crate не требуется.

#### Критерии приёмки

- Production build не включает test-only filesystem/database helpers.
- Все три generator test crates используют один generated-fixture runner.
- Phase-specific ошибки различают generation, formatting, compilation, Clippy и runtime test.

### 5.4. `gen_quotes`: единые безопасные token fragments

#### План

Оставить здесь только часто повторяемые, независимые от домена quote fragments:

- string/identifier literal construction;
- fully qualified standard paths;
- lifetime and generic fragments;
- common `From`/`AsRef` token templates, если они используются минимум двумя macro crates.

Не переносить сюда большие handler bodies. Большой `quote!` должен оставаться рядом с operation model, иначе связь между контрактом и generated code станет скрытой.

#### Критерии приёмки

- Каждый public helper имеет минимум два consumer.
- Helper принимает typed syntax/token input, а не невалидированные Rust-строки.
- Тесты сравнивают token semantics после парсинга, а не только пробелы строки.

### 5.5. `pg_crud_cmn`: общий PostgreSQL CRUD kernel

#### Что уже является общим

Pagination, operators, query bind counter, bounded collections, ordering, uniqueness и общие `PgType` traits.

#### Что дополнительно консолидировать

- единый `SqlFragment`/`QueryFragment` контракт вместо нескольких строковых wrappers с одинаковой политикой;
- helper атомарного увеличения placeholder index;
- общие типы результата bind operation;
- cardinality policies для one/many payload;
- общий operation category (`Create`, `Read`, `Update`, `Delete`) без table-specific route names;
- deterministic duplicate detection strategy с выбором hash/non-hash через trait bounds.

#### Что оставить раздельным

- Idempotency storage и optimistic revision остаются в `pg_tbl`, потому что они принадлежат HTTP/table mutation lifecycle.
- LIKE/regex/filter semantics остаются в `wh_flts`.
- Конкретные SQLx Rust/PostgreSQL mappings остаются в `pg_types_*`.

#### Критерии приёмки

- `gen_pg_types_src`, `gen_wh_flts_src` и `gen_pg_tbl_src` используют один placeholder/bind contract.
- Ошибка overflow не превращается в saturating behavior.
- SQL fragments нельзя создать из пустой или структурно недопустимой строки без проверки владельца.
- Hot loops не получают дополнительных allocations.

### 5.6. `gen_pg_types_src`: единая модель PostgreSQL-типа

#### Проблема

Это второй по размеру генератор. Числовые, текстовые, temporal/network и nullable варианты повторяют одинаковые impl для construction, read, bind, Serde, schema, examples и tests.

#### Целевая внутренняя декомпозиция

Без создания новых crates разделить `src/lib.rs` на private modules:

- `model` — `PgTypeSpec`, nullability, initialization, Rust/SQL type category;
- `parse` — macro/config parsing и validation;
- `impls` — `From`, `TryFrom`, `AsRef`, `Debug`, SQLx encode/decode/type;
- `serde_schema` — Serde, Schemars/Utoipa constraints;
- `crud_traits` — `PgType`, PK/non-PK и query binding;
- `test_gen` — deterministic generated tests;
- `emit` — orchestration и final token stream.

#### Единый `PgTypeSpec`

Модель должна хранить:

- Rust inner type и PostgreSQL type name;
- nullable/non-nullable;
- initialized by client/PostgreSQL;
- owned/reference/slice construction policy;
- validation strategy;
- supported comparison/filter operations;
- serialization and OpenAPI format;
- safe example/test values;
- secret/redacted debug policy.

Категорийные различия задаются data/config callbacks, а не копированием полного impl block.

#### Порядок миграции

1. Перевести два простых числовых типа на `PgTypeSpec`.
2. Сравнить generated public surface и source fixtures.
3. Перевести nullable пары.
4. Перевести text/UUID types.
5. Последними перевести chrono/network и secret types.

#### Критерии приёмки

- Для каждого типа сохраняются SQLx type compatibility, Serde JSON и OpenAPI schema.
- Nullable и non-nullable варианты различаются только заданной policy.
- Secret types никогда не получают делегирующий raw `Debug`.
- Generated compile/check tests проходят для каждой категории.
- Размер повторяющихся emission branches уменьшается без роста публичного API.

### 5.7. `pg_types_cmn` и crates категорий типов

#### Целевая роль

- `pg_types_cmn` владеет runtime traits и общими error/domain wrappers.
- `pg_types_numeric` содержит только сгенерированные numeric types и категории, специфичные для чисел.
- `pg_types_text_misc` содержит text/UUID/secret types и их специальные invariants.
- `pg_types_chrono_net` содержит temporal/network types и их специальные conversions.
- `pg_types` остаётся минимальным facade только для стабильных намеренных re-exports; новые wildcard re-exports не добавлять.

#### План переиспользования

- Удалить локальные impl только после генерации эквивалента из `PgTypeSpec`.
- Общие test cases хранить в `pg_types_cmn`, специальные boundary cases — рядом с категорией.
- Не перемещать concrete external types в workspace root dependencies потребителей: concrete versions остаются в workspace `Cargo.toml`, а использование — через workspace dependencies.

### 5.8. `wh_flts` и `gen_wh_flts_src`: reusable filter algebra

#### Целевая модель

Ввести private declarative `FilterSpec` внутри генератора:

- operation kind;
- accepted value shape;
- validation rule;
- SQL operator/template;
- bind count;
- null semantics;
- supported PostgreSQL type categories;
- Serde/OpenAPI discriminator;
- deterministic examples.

#### Переиспользуемые части

- equality/in/range/between operator mechanics;
- escaped contains/starts-with/ends-with text search;
- regex case policy;
- empty/duplicate validation;
- placeholder progression and bind order;
- common JSON representation and schema.

#### Граница

Raw SQL pattern, literal text search и regex должны оставаться разными доменными типами. Нельзя объединять их общим `String` API.

#### Критерии приёмки

- SQL text и bind sequence генерируются из одного `FilterSpec`.
- `%`, `_` и `\` сохраняют literal semantics.
- Каждый operator имеет type capability allowlist.
- Unknown fields и несколько operators в одном variant отклоняются одинаково runtime и schema tests.

### 5.9. `gen_pg_tbl_src`: декомпозиция главного генератора

#### Проблема

Один файл одновременно парсит config, строит SQL, генерирует Rust models, handlers, Reqwest client, frontend client, OpenAPI, metrics и tests. Это затрудняет повторное использование operation model и создаёт риск расхождения контрактов.

#### Целевая внутренняя структура

Разделить существующий crate на private modules:

- `config` — `GenPgTblConfig` parsing и validation;
- `table_model` — fields, PK, excluded fields, revision capability;
- `operation` — восемь CRUD descriptors и capability flags;
- `route_contract` — method/path/auth/status/header/content type;
- `sql` — query shape и bind plan;
- `rust_model` — payload/response/error types;
- `handler` — Axum extraction, validation, transaction and response;
- `client_reqwest` — backend/native client;
- `client_frontend` — transport-based WASM client;
- `openapi` — paths, schemas and responses;
- `frontend_meta` — pages, fields, actions and permissions;
- `metrics` — finite label descriptors;
- `test_gen` — parity, negative and JSON tests;
- `emit` — orchestration.

#### Главное правило переиспользования

`OperationSpec` и `RouteContractSpec` являются единственными источниками method/path/auth/status/header semantics. SQL, handlers, оба клиента, OpenAPI, frontend metadata и tests только читают их.

#### Общие capabilities

Оформить capabilities как композицию descriptor data:

- bulk cardinality limit;
- idempotency requirement;
- optimistic revision;
- authentication/permission;
- response status;
- resource budget kind;
- generated metrics labels;
- error status set.

Не разбрасывать проверки `if idempotency_enabled` по независимым генераторам. Сначала capability входит в `OperationSpec`, затем каждый emitter исчерпывающе обрабатывает её.

#### Порядок миграции

1. Извлечь private operation/route model без изменения output.
2. Перевести runtime route registration и OpenAPI.
3. Перевести Reqwest и frontend clients.
4. Перевести handlers и transaction policies.
5. Перевести metrics и generated tests.
6. Разделить файл физически после стабилизации model API.

#### Критерии приёмки

- Все enabled operations присутствуют одновременно в runtime, clients, OpenAPI и test descriptors.
- Operation ID уникальны.
- Status/header/content-type parity проверяется автоматически.
- Idempotency completion и mutation остаются в одной транзакции.
- Optimistic revision использует один typed `If-Match` contract.
- Existing table declarations и generated public signatures не меняются на первом этапе.

### 5.10. `pg_tbl`: runtime table-operation primitives

#### Что должно находиться здесь

- typed SQL fragments и query builders;
- idempotency storage lifecycle;
- optimistic revision type and SQL predicate;
- transaction-safe completion helpers;
- table-operation errors, не зависящие от конкретной таблицы.

#### Что улучшить для переиспользования

- одна внутренняя функция bind idempotency scope fields для begin/complete/release;
- единый executor-oriented helper, чтобы pool и transaction variants не дублировали SQL bindings;
- typed result для affected row count;
- schema ownership перевести из runtime `CREATE TABLE IF NOT EXISTS` к migration owner, оставив проверку совместимости при startup;
- query builders принимать validated fragments, а не произвольный `&str`.

#### Критерии приёмки

- Pool и connection paths используют один SQL/bind implementation.
- Нулевой affected-row result отличается от SQLx transport error.
- Cleanup имеет batch bound и отдельные retention policies.
- PostgreSQL integration tests доказывают replay, conflict, concurrency, rollback и cleanup.

### 5.11. `frontend_contract`: общий transport и HTTP metadata contract

#### Что переиспользовать

- `RouteContract`, method, mutation kind, authentication and success status;
- transport request/response wrappers;
- idempotency and concurrency header values;
- common `ApiProblem`;
- action/field/page metadata для generated UI;
- helper определения expected success и problem decoding, независимый от browser/Reqwest.

#### Новые внутренние модули

- `route`;
- `transport`;
- `problem`;
- `frontend_meta`;
- `headers`.

Это внутреннее разбиение существующего crate, не новые crates.

#### Граница

Crate не должен зависеть от Axum, SQLx, Gloo, Leptos или server admin implementation. Он описывает контракт, но не выполняет I/O.

#### Критерии приёмки

- Browser и generated/native clients используют одинаковую success/problem интерпретацию.
- Problem detail ограничен и не принимает internal source error.
- Route metadata сериализуется детерминированно для contract snapshot.

### 5.12. `server_admin_contract`: административный bounded context

#### План

- Оставить здесь admin request/response domain wrappers, routes, permissions and pages.
- Использовать `frontend_contract::RouteContract` вместо повторного хранения method/path/auth semantics.
- Перевести повторяемые string/newtype impl на расширенный `newtype` derive после стабилизации.
- Сгруппировать source по private modules `auth`, `users`, `roles`, `audit`, `settings`, `routes`, сохранив текущие публичные paths через точечные re-exports только при необходимости совместимости.

#### Критерии приёмки

- Contract crate не зависит от SQLx, Axum или Leptos.
- Все входные строки bounded и валидируются при десериализации.
- Route path tests покрывают parameterized routes.
- Старый `AdminApiErBody` удаляется только отдельным breaking-change этапом после полной миграции на `ApiProblem`.

### 5.13. `server_runtime`: общие operational primitives

#### Что переиспользовать

- supervised background task lifecycle;
- graceful shutdown;
- request ID;
- timeout and security layers;
- bounded semaphore acquisition and `Retry-After`;
- aggregate resource budgets;
- bounded async run history;
- health dependency probe primitives без конкретной БД-схемы.

#### Предлагаемая декомпозиция

- `lifecycle`;
- `http`;
- `request_id`;
- `limits`;
- `resource_budget`;
- `health`;
- `history`.

#### Новые кандидаты только после второго consumer

- общий periodic cleanup runner;
- common retry scheduler;
- bounded cache;
- file staging.

#### Критерии приёмки

- Runtime crate не знает admin table names и generated CRUD payload types.
- Cancellation и panic outcomes наблюдаемы владельцем.
- Guards освобождают ресурсы на success, error, unwind и future cancellation.
- Никакая блокировка не удерживается через `.await`.

### 5.14. `cmn_routes`: общие service endpoints

#### План

- Сохранить liveness, readiness, health summary и version/status endpoints общими.
- Ввести небольшой dependency-probe trait только при появлении второго типа dependency кроме PostgreSQL.
- Response contracts использовать из отдельного private `contract` module этого же crate.
- Probe execution/timeout делегировать `server_runtime`, а HTTP mapping оставить в `cmn_routes`.

#### Критерии приёмки

- Liveness не делает external I/O.
- Readiness имеет bounded timeout.
- Error source не попадает в response body.
- Один health contract используется runtime и OpenAPI.

### 5.15. `config_lib` и macro crates конфигурации

#### Что переиспользовать

- required env extraction;
- parse/map error skeleton;
- positive/non-zero numeric policies;
- boolean/text/secret config wrappers;
- generated getter traits;
- redacted diagnostics.

#### План

- Описать private `ConfigFieldSpec` в macro layer: env name, target wrapper, parser, secrecy and default policy.
- Генерировать однообразные `TryFromStdEnvVarOk` и getter implementations.
- Сохранять domain wrapper для каждой config field; не заменять всё универсальным `ConfigValue<T>`.
- Секреты всегда хранить в `SecretBox` wrapper и не включать raw значение в `Debug`/error.

#### Критерии приёмки

- `.env` и `.envexample` parity остаётся policy test.
- Missing, malformed and out-of-range errors различимы.
- Добавление одного config field требует declaration, workspace state wiring и tests, но не копирования parser boilerplate.

### 5.16. `server_config`: composition root конфигурации

#### План

- Оставить concrete `Config`, который агрегирует types из `config_lib`.
- Getter trait generation использовать для state capabilities.
- Не переносить env reading в runtime или domain crates.
- Добавить source-level test, что каждое поле `Config` имеет getter и участвует в construction.

### 5.17. `app_state`, `server_app_state` и `server_app_state_macros`

#### Разделение ответственности

- `app_state` — минимальные общие infrastructure wrappers, сейчас PostgreSQL pool capability.
- `server_app_state` — concrete cross-thread state текущего server.
- `server_app_state_macros` — механические getter trait impl для полей state.

#### Что переиспользовать

- owned/reference wrapper impl через `newtype`;
- capability getter generation;
- test state builder под `test-utils`;
- compile-time проверку, что generated handlers требуют только trait combination, а не concrete state.

#### Критерии приёмки

- Generated table API можно тестировать с минимальным fake/test state.
- Новый state field не становится публичным автоматически.
- `Arc` создаётся только в composition root для cross-thread sharing.
- Test builder не подключается в production feature set.

### 5.18. `server_admin`: reusable admin domain service

#### Внутренняя декомпозиция

Разделить крупные `lib.rs`/`auth.rs` по private modules:

- `domain` — admin-specific wrappers;
- `password` — hashing and verification;
- `token` — access/refresh/CSRF generation and validation;
- `session` — session storage and rotation;
- `rbac` — roles and permissions;
- `audit` — append-only write and bounded cleanup;
- `rate_limit`;
- `handlers`;
- `openapi`;
- `migrations`;
- `cleanup`.

#### Переиспользуемые зависимости

- `ApiProblem` из `frontend_contract`;
- lifecycle/budgets из `server_runtime`;
- pool wrapper из `app_state`;
- request/response types из `server_admin_contract`;
- generated table API из `gen_pg_tbl_src`.

#### Что не обобщать

Password policy, JWT claims, RBAC permission names и audit actions остаются admin domain types. Их не следует переносить в `server_runtime`.

#### Критерии приёмки

- Authentication, CSRF, RBAC и audit tests сохраняют поведение.
- Internal crypto/SQLx error не сериализуется.
- Cleanup остаётся batch-bounded и supervised.
- Migration/seed tests покрывают fresh, upgrade and repeated bootstrap.

### 5.19. `server_admin_frontend`: reusable browser client and UI mechanics

#### Что переиспользовать внутри crate

- typed transport adapter;
- JSON/problem decoding;
- authentication keep-alive coordinator;
- action runner and common loading/error state;
- form field rendering из `frontend_contract::FieldContract`;
- navigation/permission filtering;
- generated table list/create/update/delete components.

#### Внутренние модули

- `transport`;
- `auth_keep_alive`;
- `client`;
- `state`;
- `components`;
- `generated_table`;
- `admin_pages`;
- `router`.

#### Граница

Leptos/Gloo остаются здесь. `frontend_contract` не должен получать browser dependencies. Admin-specific labels/pages остаются в `admin_pages`; generic rendering читает typed metadata.

#### Критерии приёмки

- Одновременные 401 создают один refresh request.
- Original request повторяется максимум один раз.
- Generic table component не содержит имён конкретных admin таблиц.
- Host unit tests проверяют state machines, WASM check — browser integration.

### 5.20. `server`: composition root

#### План

Оставить в executable только:

- загрузку config;
- создание pool/state/budgets;
- mounting routers;
- создание supervised tasks;
- HTTP listener and graceful shutdown;
- mapping startup errors.

Вынести из `main.rs` в owning crates любую reusable реализацию, которая имеет второго consumer: CORS parsing — в config/runtime HTTP module, cleanup behavior — в `server_admin`, lifecycle — в `server_runtime`.

Не переносить composition-specific значения интервалов и лимитов в shared constants без конфигурационного контракта.

#### Критерии приёмки

- `main.rs` не содержит SQL.
- `main.rs` не реализует auth, CRUD или JSON contracts.
- Background task owner хранится до shutdown и явно завершается.

### 5.21. `tests`, generator test crates и `workspace_test_runner`

#### Общая стратегия

- `tests` владеет workspace policy и архитектурными проверками.
- `workspace_test_runner` владеет режимами запуска, DB guard и orchestration.
- `gen_*_test` crates владеют только fixtures и assertions своего генератора.
- `macros_helpers` предоставляет test-only mechanics.

#### Что переиспользовать

- generated crate check runner;
- JSON round-trip phases;
- normalized OpenAPI comparison;
- route/runtime/OpenAPI parity;
- guarded database target;
- deterministic schema isolation;
- common assertions для unknown fields, duplicate headers и wrong content type.

#### Критерии приёмки

- Static mode не требует PostgreSQL.
- Database mode отклоняет production-like URL до подключения.
- Generated negative matrix строится из route descriptors.
- Один и тот же fixture не копируется между несколькими generator test crates.

### 5.22. Малые utility и facade crates

#### `naming`, `naming_cmn` и macro crates

- Оставить здесь единственный источник преобразований `snake_case`, `UpperCamelCase` и workspace abbreviation policy.
- `naming_cmn` владеет runtime naming primitives, macro crates — compile-time parsing/emission, `naming` — намеренным facade.
- `gen_pg_types_src`, `gen_wh_flts_src`, `gen_pg_tbl_src` и config macros не должны реализовывать собственные case converters.
- Добавить exhaustive small-alphabet tests для совпадения runtime и macro-time преобразований.

#### `token_patterns` и `token_patterns_macros`

- Владеть повторяемыми token-pattern declarations, но не domain-specific Rust AST models.
- Использовать только там, где одинаковый token grammar встречается минимум в двух macro crates.
- Не скрывать через token pattern большие handler/query bodies.

#### `loc_lib`, `loc_macros`, `location` и `panic_loc`

- Сохранить единый механизм location-aware diagnostics.
- Генераторы должны получать location/error context через этот механизм, а не копировать `file!`, `line!` и format templates.
- Runtime error bodies не должны раскрывать source location; location предназначен для internal diagnostics.
- `loc_test` остаётся владельцем cross-crate contract fixtures.

#### `route_validators`

- Владеть HTTP pre-handler validation mechanics: body-size limit, commit/version header и другие transport-level checks.
- Validation result должен преобразовываться в общий `ApiProblem` на HTTP boundary, но crate не должен знать admin permissions или table SQL.
- Дублируемые header singleton checks переносить сюда только после второго non-generated consumer; table-specific `Idempotency-Key`/`If-Match` policy остаётся в operation descriptor.

#### `contract_constants`

- Хранить только действительно общие protocol constants, используемые несколькими contract/runtime crates.
- Не создавать каталоги строковых имён полей, routes или SQL columns: typed descriptors являются источником истины.
- Каждый public constant должен иметь source-level consumer count минимум два.

#### `git_info`

- Оставить единый typed build/version metadata contract и его generated values.
- `server`, common routes, validators и frontend должны читать одну структуру, а не независимо обращаться к environment или Git.
- Секреты, local paths и dirty diff content не включать в public metadata.

#### `to_err_string` и `to_err_string_macros`

- Использовать для механического безопасного делегирования error text только там, где не теряется typed source.
- Не заменять конкретные error enums строкой.
- Не применять к secret-bearing errors без redaction contract.

#### `optml`

- Сохранить узкую ownership для optional/multiple helper mechanics.
- Перед переносом похожей логики из CRUD проверить, совпадают ли cardinality и error semantics; синтаксического сходства недостаточно.
- Не расширять crate generic collection abstractions, уже принадлежащими `pg_crud_cmn::bounded_vec`.

#### `macro_clippy_check_cmn`

- Оставить общую orchestration model для проверки generated macro output.
- Свести создание временного crate, Cargo invocation и diagnostics к `macros_helpers::generated_fixture`, если оба crates сейчас владеют одной механикой; после миграции оставить одному crate orchestration, другому typed helpers.
- Не включать этот crate в production dependency graph.

#### Facade crates `pg_crud`, `pg_types`, `gen_pg_tbl`, `gen_pg_types`, `gen_wh_flts`

- Facade crates должны оставаться минимальными стабильными entry points.
- Re-export допускается только для намеренно поддерживаемого public API; wildcard re-export и alias через `as` не использовать.
- Реальная parsing/emission/runtime логика остаётся в `*_src` или owning runtime crate.
- Feature forwarding документировать и проверять feature-matrix тестом.

#### `pg_crud_macros_cmn`, `pg_crud_macros_cmn_macros` и `pg_crud_cmn_macros`

- Владеть только общей compile-time механикой CRUD derives и их runtime support traits.
- Parsing `syn` и diagnostics делегировать `workspace_macro_helpers`; field/query semantics делегировать `pg_crud_cmn`.
- Test-case generation не смешивать с production trait definitions: fixtures остаются под test feature или в generator test crates.
- Не дублировать `PgTypeSpec`, `FilterSpec` или `OperationSpec` в macro-common crates; эти модели имеют отдельных владельцев.

#### `try_from_env` и `gen_getter_traits_for_struct_fields`

- Оставить специализированными proc-macro entry points поверх общей parsing механики `workspace_macro_helpers`.
- `try_from_env` генерирует только переход из env representation в domain wrapper; конкретные defaults и composition остаются в `server_config`.
- `gen_getter_traits_for_struct_fields` генерирует capability getters без автоматического расширения visibility полей.
- Их README и compile-error tests должны фиксировать поддерживаемую attribute grammar.

#### `server_tbl_example`

- Использовать как reference consumer и executable contract fixture для всех generated capabilities.
- Не превращать пример в новый shared crate.
- Каждая новая capability `gen_pg_tbl_src` должна иметь здесь representative declaration, runtime/OpenAPI parity test и DB-independent negative case; DB semantics проверяются в guarded integration suite.

#### Критерии приёмки малых crates

- У каждого crate есть одно предложение об ответственности и явный список запрещённых зависимостей.
- Нет двух реализаций case conversion, location formatting, build metadata или generated fixture orchestration.
- Facade crates не содержат business/runtime logic.
- Utility helper имеет минимум два consumer либо остаётся private у первого consumer.

## 6. Сквозные инициативы

### 6.1. Owned/ref/slice wrapper family

Для повторяемой тройки `Owned(T)`, `Ref<'a>(&'a T)` и `SliceRef<'a>(&'a [T])` использовать одну декларацию только когда типы имеют одинаковую visibility и policy. Генерация должна поддерживать:

- lifetime propagation;
- sized/unsized inner;
- `From` и `AsRef`;
- optional `AsMut`;
- safe delegated formatting;
- запрет raw formatting для secrets.

### 6.2. Единый route contract

Все HTTP-представления operation должны выводиться из одного descriptor:

```text
OperationSpec
  -> Axum route
  -> handler extraction and status mapping
  -> Reqwest client
  -> WASM transport client
  -> OpenAPI operation
  -> frontend action metadata
  -> positive/negative contract tests
```

Любая новая capability считается завершённой только после обработки во всех семи consumers.

### 6.3. Единая error policy

- Domain validation возвращает конкретный enum error.
- Infrastructure error остаётся source.
- HTTP boundary преобразует ошибку в bounded `ApiProblem`.
- Frontend декодирует тот же contract.
- OpenAPI ссылается на тот же schema.
- Debug secret wrappers остаётся redacted.

### 6.4. Единая generated-test policy

Для каждого генератора нужны четыре уровня:

1. Unit tests внутренней модели.
2. Token/source assertions для ключевых ветвей.
3. Compile/check/Clippy временного generated crate.
4. Runtime/DB tests только там, где compile evidence недостаточно.

## 7. Приоритеты внедрения

| Этап | Приоритет | Изменение | Риск | Основное доказательство |
|---|---|---|---|---|
| 0 | P0 | Зафиксировать inventory повторяемых impl и generated API snapshots | Низкий | deterministic report и green baseline |
| 1 | P0 | Расширить `newtype` для owned/ref/slice impl | Средний | compile-pass/error и unchanged public API |
| 2 | P0 | Извлечь `OperationSpec`/`RouteContractSpec` в `gen_pg_tbl_src` | Высокий | runtime/client/OpenAPI parity |
| 3 | P0 | Разделить idempotency pool/transaction SQL через один executor helper | Средний | PostgreSQL rollback/replay tests |
| 4 | P1 | Ввести `PgTypeSpec` и перевести numeric pilot | Высокий | generated fixture diff и SQLx tests |
| 5 | P1 | Ввести `FilterSpec` и перевести equality/text search | Средний | SQL/bind/Serde/OpenAPI parity |
| 6 | P1 | Разделить `frontend_contract` и общий problem decoding | Средний | native/WASM contract tests |
| 7 | P1 | Декомпозировать `server_runtime` и `server_admin` на private modules | Средний | unchanged public paths and integration tests |
| 8 | P2 | Consolidate config field parsing/getters | Средний | env parity and error tests |
| 9 | P2 | Consolidate test runners and fixtures | Низкий | identical gates with less fixture code |
| 10 | P2 | Удалить оставшийся эквивалентный boilerplate | Средний | full workspace audit |

## 8. Детальный порядок работ

### Этап 0. Baseline и inventory

- Сохранить normalized generated Rust/OpenAPI fixtures для representative table и PostgreSQL types.
- Посчитать ручные `From`, `AsRef`, `Debug`, `Display`, JSON round-trip helpers и route literals.
- Зафиксировать публичные items через rustdoc/semver gate.
- Запустить обязательный baseline: format, Clippy, code-style, static tests, OpenAPI suite и guarded DB tests.

### Этап 1. Wrapper generation

- Добавить macro model/options.
- Реализовать owned и borrowed sized wrappers.
- Добавить unsized `str`/slice.
- Добавить formatting policy и secret prohibition.
- Перевести `app_state` как pilot.
- Перевести два generated PostgreSQL wrapper family.
- Удалить только доказанно эквивалентные impl.

### Этап 2. Table operation single source of truth

- [x] Зафиксировать все восемь CRUD operation descriptors.
- [x] Перевести route and OpenAPI emitters.
- [x] Перевести clients.
- [x] Перевести handler headers/status/errors.
- [x] Генерировать negative matrix из descriptors.
- [x] Проверить disabled operations и capability combinations.

### Этап 3. PostgreSQL type/filter model

- [x] Создать private `PgTypeSpec` и `FilterSpec`.
- [x] Перевести малые representative категории: PK-capability типов и comparison/range SQL operators.
- [x] Сравнить JSON, schema, SQLx encode/decode and generated source для пилотных категорий.
- [x] Расширять по одной категории, не смешивая с публичными rename: equality и text search добавлены после comparison/range pilot.

Подтверждение: `cargo check -p gen_pg_types_src -p gen_wh_flts_src` и `cargo test -p gen_pg_types_test -p gen_wh_flts_test` проходят; generated temporary crates также проходят собственный clippy/test harness. Text search сохраняет literal semantics `%`, `_`, `\`, а SQL operator/suffix и equality operator поступают из validated `FilterSpec`.

### Этап 4. Runtime/API/frontend reuse

- [x] Разделить common problem decode.
- [x] Разделить browser transport и auth state machine.
- [x] Разделить runtime lifecycle/limits/health modules.
- [x] Уменьшить composition logic в `server`.
- [x] Вынести OpenAPI/router wiring `server_admin` в private `auth/routes`.
- [x] Вынести session persistence/rotation `server_admin` в private `auth/session`.
- [x] Вынести rate-limit types и enforcement `server_admin` в private `auth/rate_limit`.
- [x] Вынести password hashing/verification `server_admin` в private `password`.
- [x] Вынести token generation/hash/JWT validation `server_admin` в private `token`.
- [x] Вынести schema migration/bootstrap `server_admin` в private `migrations`.
- [x] Вынести batch-bounded table cleanup `server_admin` в private `cleanup`.
- [x] Вынести append-only audit writer `server_admin` в private `auth/audit`.
- [x] Вынести bounded audit query `server_admin` в private `auth/audit`.
- [x] Вынести permission/audit mappings и permission serialization в private `rbac`.
- [x] Вынести core admin wrappers/identifiers/names в private `domain` с сохранением root reexports.
- [x] Вынести read-only users/roles/permissions/settings implementations в private `auth/handlers`.
- [x] Вынести `me`, session list/revoke и settings mutation implementations в private `auth/handlers`.
- [x] Вынести create-user implementation в private `auth/handlers`.
- [x] Вынести update-user/password и create/update/delete-role implementations в private `auth/handlers`.
- [x] Вынести role-permission и user-role assignment implementations в private `auth/handlers`.
- [x] Вынести user ban/delete implementations в private `auth/handlers` с last-active-admin guard.
- [x] Завершить перенос sign-in/refresh/sign-out implementations в `auth/handlers`.

Выполнено для composition root: generated-route authentication middleware перенесён из `server/main.rs` в private `server_admin::generated_auth`; CORS parsing и typed header collection перенесены в `server_runtime::cors`; executable только применяет готовые primitives. После переноса удалена неиспользуемая зависимость `server -> pg_tbl`.

Browser transport вынесен из `app.rs` в private `server_admin_frontend::transport`; refresh coordination остаётся в отдельном `auth_keep_alive`. CSRF token и Gloo HTTP method представлены доменными wrappers, а не raw boundary types.

Admin-domain decomposition выполнена по стабильным границам: core wrappers/identifiers/names находятся в private `domain.rs` и reexported из прежнего root; generated-route middleware находится в `server_admin::generated_auth`; OpenAPI inventory и Axum router wiring вынесены в private `auth/routes.rs`; session persistence/rotation — в `auth/session.rs`; rate-limit scope/value types и SQL enforcement — в `auth/rate_limit.rs`; append-only writer и bounded query — в `auth/audit.rs`; все route implementations — в `auth/handlers.rs`; permission/audit mappings и permission serialization — в `rbac.rs`; password hashing/verification — в `password.rs`; token generation/hash/JWT validation — в `token.rs`; migration/bootstrap — в `migrations.rs`; bounded cleanup — в `cleanup.rs`. Публичные paths и OpenAPI handler inventory сохранены forwarding-функциями либо прежними inherent methods.

Подтверждение текущего этапа: `cargo test -p frontend_contract`, `cargo check -p server_admin_frontend -p gen_pg_tbl_src`, `cargo test -p server_runtime -p cmn_routes`, `cargo test -p server_runtime -p server`, `cargo check -p server_admin -p server`, `cargo test -p server_admin --lib`, `cargo test -p server_admin_frontend`, `cargo check -p server_admin_frontend --target wasm32-unknown-unknown` и `cargo test -p tests code_style` проходят. `SuccessStatus::transport_status`, `TransportResponse::success_body` и `decode_api_problem` используются общим контрактом, generated client и browser admin client. PostgreSQL readiness сохраняет SQL/HTTP mapping в `cmn_routes`, но выполняет timeout через `server_runtime::health`. После текущей декомпозиции все 113 admin unit tests и 42 code-style tests остаются зелёными; integration coverage содержит repeated bootstrap, bounded cleanup и fresh/supported-baseline upgrade migrations.

### Этап 5. Cleanup и hardening

- [x] Удалить устаревшие ручные helpers.
- [x] Проверить public API minimality.
- [x] Проверить feature combinations и unused dependencies.
- [x] Обновить архитектурную документацию.

Cleanup audit удалил эквивалентный ручной `Debug` у `SnakeIdentLen`; оставшиеся secret-aware, query-aware и marker-type реализации имеют отличающуюся семантику. Borrowed syntax wrappers внутри самого `newtype` остаются ручными, потому что proc-macro crate не может применить собственный derive к своим типам. Новые implementation-модули закрыты, а прежние facade paths сохранены; добавленные публичные элементы имеют production consumers в runtime, frontend или generated authentication.

Полный all-features Clippy проверяет unused crate dependencies. Дополнительно проходят default workspace build и `--no-default-features` для всех crates с feature graph; единственная workspace feature family `test-utils` не попадает в production build без явного включения. Фактическое владение общей логикой зафиксировано в корневом `README.md`.

Финальное подтверждение: `cargo fmt`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test -p tests code_style`, targeted runtime/contract/admin/frontend suites, `cargo check -p server_admin_frontend --target wasm32-unknown-unknown` и generated temporary-crate suites `gen_pg_tbl_test`, `gen_pg_types_test`, `gen_wh_flts_test` проходят. Guarded PostgreSQL integration tests присутствуют в admin suite и корректно завершаются без обязательного внешнего сервиса.

## 9. Проверки для каждого этапа

Минимальный набор:

```bash
cargo fmt
cargo clippy --all-targets --all-features -- -D warnings
cargo test -p tests code_style
```

Дополнительно по области:

- generator: generated temporary crate check, Clippy and tests;
- contracts: JSON round trip, OpenAPI parity and normalized snapshot;
- frontend: host tests и `cargo check -p server_admin_frontend --target wasm32-unknown-unknown`;
- PostgreSQL: guarded test database, fresh schema and concurrent transaction tests;
- public API: semver check against выбранный baseline;
- features: feature-matrix check после изменения feature graph.

## 10. Метрики результата

Переиспользование считается полезным не по числу созданных abstractions, а по следующим метрикам:

- количество удалённых эквивалентных ручных impl;
- количество независимых literals method/path/status/header до и после;
- число emitters, читающих один operation descriptor;
- число type categories, использующих один `PgTypeSpec` pipeline;
- число filter operations, использующих один `FilterSpec` pipeline;
- доля generated crates, проверяемых одним runner;
- отсутствие роста public API без реального consumer;
- отсутствие новых dependencies для механики, реализуемой через `std` и текущий macro stack;
- неизменность wire/SQL behavior, подтверждённая contract и DB tests.

## 11. Что не следует переиспользовать

- Не объединять admin domain types с generic generated table types.
- Не переносить SQLx в `frontend_contract` или `server_admin_contract`.
- Не переносить Leptos/Gloo в shared contract crates.
- Не делать один универсальный error enum для всего workspace.
- Не делать один `StringWrapper<TPolicy>` вместо именованных доменных типов.
- Не превращать `server_runtime` в storage/domain crate.
- Не создавать универсальный query builder поверх всей SQLx API.
- Не генерировать `Deref`, `Clone`, `Default`, `Debug` или `Display` без явной семантической причины.
- Не создавать generic file staging, cache или retry abstraction до двух production consumers.
- Не разносить одну capability между независимыми конфигурационными флагами нескольких emitters.

## 12. Definition of Done

План переиспользования считается реализованным, когда:

1. Каждый повторяемый механизм имеет одного указанного владельца.
2. `newtype` покрывает выбранные owned/ref/slice families без раскрытия secrets.
3. Все представления CRUD operation выводятся из `OperationSpec`/`RouteContractSpec`.
4. Representative PostgreSQL type categories используют общий `PgTypeSpec` pipeline.
5. Filters используют `FilterSpec` с SQL/bind/schema parity.
6. Runtime, contract, admin и frontend crates физически разделены на private modules с сохранением архитектурных границ.
7. Ручной boilerplate удалён только после доказательства эквивалентности.
8. Публичный API не расширен speculative helpers.
9. Обязательные workspace gates, WASM build, OpenAPI suite и guarded PostgreSQL tests проходят.
10. Документация отражает фактическое, а не предполагаемое владение общей логикой.
