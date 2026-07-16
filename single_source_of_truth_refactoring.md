# План рефакторинга для единого источника правды

## Цель

Для каждого изменяемого понятия в workspace должен существовать один авторитетный источник. Остальные представления должны либо генерироваться из него, либо проверяться относительно него автоматическим тестом.

Под «источником правды» здесь понимается не общий файл строковых констант, а типизированная модель в crate, который владеет соответствующей семантикой. `str_constants` должен хранить переиспользуемый текст, но не владеть правилами API, базы данных или безопасности.

Аудит выполнен по текущим Rust-исходникам, SQL-миграциям, proc-macro crate-ам, frontend-коду и Playwright-фикстурам. Документ описывает целевое состояние; он не предлагает менять исторические миграции или публичный API без отдельного этапа совместимости.

## Краткий итог

| Приоритет | Область | Сейчас источников | Целевой источник |
|---|---|---:|---|
| P0 | Admin-доменные строки и их ограничения | 3 | типизированная policy-модель в общем contract/domain crate |
| P0 | HTTP-маршруты, методы и OpenAPI | 3 | `TypedRoute` + единый реестр `RouteFamily` |
| P0 | Каталог admin permissions | 3–4 | типизированный каталог permissions |
| P1 | PostgreSQL-схема и Rust-дескрипторы таблиц | 2 | миграции как runtime-источник + обязательная conformance-проверка |
| P1 | Общие правила строковой валидации | много | переиспользуемые policy-функции и derive `TryFrom`/`BoundedString` |
| P1 | Лимиты HTTP/API | несколько | типизированная конфигурация и route metadata |
| P1 | Frontend E2E-моки | 2 | сгенерированный contract snapshot/fixture |
| P2 | Модели генераторов `pg_crud` | несколько pipeline-представлений | общая typed IR в shared crate |
| P2 | Строковые константы | один большой технический реестр | семантические значения в owning crate, текстовые фрагменты в `str_constants` |

## Уже существующие основы, которые нужно переиспользовать

Следующие механизмы уже движут проект к единому источнику правды и не должны быть реализованы заново:

- `workspace.dependencies` и code-style tests контролируют версии и способ подключения зависимостей;
- `newtype::BoundedString` связывает базовую runtime-валидацию, serde и OpenAPI;
- отдельные derive из `newtype` устраняют ручные trait implementations без объединения независимых контрактов;
- `frontend_contract::TypedRoute` уже типизирует большую часть HTTP-контракта;
- `frontend_contract::RouteFamily` уже предоставляет общий список маршрутов для coverage validation;
- `pg_crud_common::db_schema_conformance` является подходящей основой для проверки Rust-дескрипторов относительно применённых migrations;
- тест `env_and_envexample_have_same_keys` уже доказывает синхронизацию `.env` и `.env.example`;
- `server_admin_contract` уже используется Rust frontend-ом и поэтому должен оставаться источником wire DTO.

Рефакторинг ниже должен расширять эти механизмы, а не создавать параллельные генераторы и форматы.

## P0. Объединить правила admin-доменных значений

### Проблема

Одни и те же понятия объявлены независимо в API-контракте и серверном домене:

- `AdminLogin`, `AdminDisplayName`, `AdminRoleName` в [`server_admin_contract/src/lib.rs`](server_admin_contract/src/lib.rs);
- одноимённые типы в [`server_admin/src/domain.rs`](server_admin/src/domain.rs);
- ограничения PostgreSQL в [`server_admin/migrations/0001_admin_schema.sql`](server_admin/migrations/0001_admin_schema.sql).

Максимумы совпадают (`128`, `256`, `128`), но SQL дополнительно задаёт минимальную длину, trim/lowercase и regex. Например, API-тип `AdminLogin` ограничивает только максимальное число символов, тогда как база требует длину `3..=128`, lowercase и набор `[a-z0-9_.-]`. Это означает, что валидное значение одного слоя может быть отвергнуто другим.

Пароль также описан отдельно: `AdminPassword` в contract crate, секретный `AdminPassword` на сервере, ручная serde-валидация и ручная OpenAPI-схема в [`server_admin/src/lib.rs`](server_admin/src/lib.rs).

### Целевое решение

Создать типизированные policy-функции в выделенном shared crate либо в `server_admin_contract`, если он остаётся допустимой зависимостью серверного домена:

```rust
pub fn validate_admin_login(value: &str) -> Result<(), AdminLoginError>;
pub fn validate_admin_display_name(value: &str) -> Result<(), AdminDisplayNameError>;
pub fn validate_admin_role_name(value: &str) -> Result<(), AdminRoleNameError>;
```

Contract-типы и серверные типы могут оставаться разными по назначению, но должны вызывать одну policy-функцию через `newtype::TryFrom`. Секретный серверный пароль должен конвертироваться из проверенного contract-типа, а не повторять числовые ограничения.

SQL нельзя напрямую заменить Rust-валидацией: ограничения базы являются последней линией защиты. Необходимо добавить conformance-тест, который доказывает совпадение Rust-policy и SQL constraint на граничных и характерных значениях.

### Критерии готовности

- Числа `128`, `256`, `1024` и правила формата не повторяются в нескольких Rust-типах.
- Contract и server используют одну функцию проверки каждого понятия.
- OpenAPI min/max берутся из той же policy-модели.
- Тесты проверяют одинаковый результат Rust и PostgreSQL на границах и для неправильного регистра/формата.

## P0. Сделать `TypedRoute` единственным описанием HTTP-контракта

### Проблема

Маршрут сейчас описывается в трёх местах:

1. `#[typed_route(...)]` в [`server_admin_contract/src/lib.rs`](server_admin_contract/src/lib.rs) содержит path, method, request, response, access и operation id.
2. [`server_admin/src/auth/routes.rs`](server_admin/src/auth/routes.rs) вручную повторяет path-to-handler и HTTP method через `.route(..., axum::routing::get/post/...)`.
3. Там же `#[openapi(paths(...), components(...))]` вручную перечисляет handlers и схемы; handlers дополнительно содержат `#[utoipa::path(...)]`.

`RouteFamily` уже агрегирует typed routes, но генерирует только coverage descriptors. Он не является источником router/OpenAPI registration.

### Целевое решение

Расширить typed route infrastructure двумя независимыми контрактами:

- route descriptor предоставляет path, method, request/response schema и security metadata;
- server-side registration связывает конкретный route type с handler-функцией.

Реестр handler-ов должен объявляться один раз. Из него должны строиться:

- `axum::Router`;
- список OpenAPI operations/schemas;
- coverage validation;
- frontend transport metadata.

Не следует делать один derive, который скрыто генерирует всё сразу. Предпочтительны отдельные derive/proc-macro для метаданных и явная функция регистрации handler-а.

### Критерии готовности

- Метод и path каждого endpoint записаны только в `TypedRoute`.
- Добавление route требует одной записи handler binding, а не изменений в router, OpenAPI paths и coverage list.
- Compile-time тест отвергает handler с несовместимыми request/response типами.
- OpenAPI validation сравнивает документ с тем же реестром, из которого собран router.

## P0. Централизовать каталог admin permissions

### Проблема

Permissions повторяются в:

- SQL seed [`server_admin/migrations/0002_admin_permissions.sql`](server_admin/migrations/0002_admin_permissions.sql);
- enum `AdminPermission` и его строковое отображение в [`server_admin/src/lib.rs`](server_admin/src/lib.rs) и [`server_admin/src/rbac.rs`](server_admin/src/rbac.rs);
- авторизации handlers;
- строковых константах;
- JavaScript-моках [`server_admin_frontend/tests/admin-tables.spec.js`](server_admin_frontend/tests/admin-tables.spec.js).

SQL seed и Rust enum могут разойтись без ошибки компиляции. Frontend fixture уже вручную повторяет такие значения, как `users:read` и `permissions:read`.

### Целевое решение

Владелец — типизированный Rust-каталог в server admin domain/shared crate:

```rust
pub enum AdminPermission { /* ... */ }

impl AdminPermission {
    pub const ALL: &[Self];
    pub const fn as_str(self) -> &'static str;
}
```

Из каталога должны получаться:

- runtime parsing/serialization;
- bootstrap/upsert актуальных permissions при запуске или отдельной команде миграции данных;
- тестовый JSON fixture;
- OpenAPI enum, если permission публикуется как конечное множество.

Историческую SQL-миграцию изменять нельзя. Для уже созданных баз нужен идемпотентный reconciliation шаг и тест, сравнивающий множество строк базы с `AdminPermission::ALL`.

### Критерии готовности

- Новая permission добавляется в одном Rust enum.
- Тест падает, если PostgreSQL содержит недостающую или неизвестную permission.
- JS-тесты не содержат вручную набранный основной permission catalog.

## P1. Зафиксировать отношение миграций и Rust-дескрипторов таблиц

### Проблема

Структура admin-таблиц повторяется в SQL migrations и в [`server_admin/src/generated_tables.rs`](server_admin/src/generated_tables.rs). Rust-дескрипторы повторяют имена и типы колонок, а proc macro строит из них CRUD API.

Генерировать исторические migrations из текущей структуры нельзя: миграции должны быть неизменяемым журналом эволюции. Обратная генерация Rust-кода из живой базы также сделает сборку зависимой от внешнего сервиса.

### Целевое решение

Считать применённые migrations runtime-источником структуры базы, а Rust-дескрипторы — compile-time ожиданием. Связать их обязательным schema conformance test на основе уже существующего `pg_crud_common::db_schema_conformance`.

Проверять как минимум:

- таблицы и порядок/имена колонок;
- PostgreSQL type и nullability;
- primary/foreign/unique keys;
- server defaults;
- CRUD exclude fields и чувствительные поля.

Для новых таблиц можно позднее ввести typed schema specification, генерирующую начальную migration и Rust descriptor. После публикации migration остаётся неизменяемой.

### Критерии готовности

- CI поднимает чистую PostgreSQL, применяет migrations и проверяет каждый CRUD descriptor.
- Несовпадение типа, nullability или отсутствующая колонка приводит к детерминированному падению.
- В документе crate зафиксировано, что migrations — runtime authority, descriptor — проверяемое ожидание.

## P1. Переиспользовать строковые validation policies

### Проблема

В workspace много ручных `TryFrom<String>` с повторяющимися проверками длины, ASCII, trim, NUL и разрешённых символов. Часть типов использует `newtype::BoundedString`, а часть повторяет те же базовые проверки в `server_runtime`, `pg_crud`, `git_info`, `config_lib` и других crate-ах.

`BoundedString` уже централизует min/max, char count, trim, NUL, serde и utoipa, но предметные проверки остаются разрозненными. Нельзя расширять `BoundedString` десятками скрытых флагов: это снова создаст агрегирующий макрос с неочевидной семантикой.

### Целевое решение

- Оставить `BoundedString` источником только базовой политики ограниченной строки.
- Вынести переиспользуемые предметные validators в shared crate: ASCII identifier, lowercase identifier, URL-safe token part, SQL identifier и подобные.
- Использовать отдельный `newtype::TryFrom` с явным `#[try_from(validator = ...)]`.
- Если validator влияет на OpenAPI, предоставить отдельное типизированное schema metadata, а не вручную повторять ограничения.

### Критерии готовности

- Одинаковая последовательность проверок не реализована более чем в одном crate.
- Каждый validator имеет table-driven unit tests.
- Тип ошибки принадлежит policy и одинаково используется всеми wrapper-типами.

## P1. Убрать локальные копии HTTP/API-лимитов

### Проблема

Глобальный body limit приходит из конфигурации в [`server/src/main.rs`](server/src/main.rs), но admin router отдельно задаёт `65_536` в [`server_admin/src/auth/routes.rs`](server_admin/src/auth/routes.rs). Дополнительно contract types имеют локальные максимумы `8192`, `4096` и другие, которые не связаны с route metadata.

Локальный более строгий лимит может быть намеренным, но сейчас он не является частью typed route contract и поэтому невидим клиенту/OpenAPI.

### Целевое решение

Ввести доменные wrapper-типы лимитов и добавить optional body limit в metadata маршрута или route family. Фактический Axum layer, body-size validator и OpenAPI requestBody constraints должны читать одно значение.

### Критерии готовности

- В router wiring нет сырых числовых body limits.
- Эффективный лимит маршрута доступен через typed metadata.
- Есть тест, доказывающий одинаковую границу в transport validation и Axum rejection.

## P1. Генерировать frontend E2E fixtures из Rust-контракта

### Проблема

Основной frontend написан на Rust/Leptos и использует `server_admin_contract`, но Playwright-тесты вручную задают URL и JSON shape в JavaScript. Эти моки не компилируются вместе с DTO и могут продолжить проходить после несовместимого изменения контракта.

### Целевое решение

Добавить Rust fixture generator/test utility, который сериализует валидные DTO и typed route paths в JSON-файл внутри `target/` перед Playwright. JS fixture server должен загружать этот файл вместо ручного воспроизведения основных DTO и путей.

Не следует коммитить сгенерированный runtime artifact или читать его через `include_str!`; генерация должна быть явным шагом тестового runner-а.

### Критерии готовности

- API paths в Playwright не набираются строковыми литералами.
- Успешные response bodies сериализуются Rust DTO.
- Отдельные намеренно неправильные payload остаются ручными и помечены как negative fixtures.

## P2. Выделить общую typed IR для генераторов `pg_crud`

### Проблема

Генерация разделена между `generate_pg_table_src`, `generate_pg_types_src`, `generate_where_filters_src`, proc-macro entrypoint crate-ами и тестовым runner-ом. Одни и те же сведения проходят через token streams, JSON-подобные attributes и повторный parsing. `workspace_test_runner` отдельно воспроизводит pipeline и его stage metrics.

Это повышает риск, что production entrypoint, тестовый entrypoint и benchmark интерпретируют конфигурацию по-разному.

### Целевое решение

В shared non-proc-macro crate определить типизированные модели:

- parsed configuration;
- table/field model;
- validated generation model;
- route/operation model;
- diagnostics.

Все proc-macro entrypoints и `workspace_test_runner` должны вызывать один pipeline `parse -> build -> validate -> emit`. Token generation остаётся последней стадией, а не транспортом между стадиями.

### Критерии готовности

- Config parsing и validation существуют в одной реализации.
- Benchmark вызывает тот же публичный pipeline, что proc macro.
- Stage tests работают с typed IR, не ищут признаки реализации через `TokenStream::to_string()`.

## P2. Ограничить роль `str_constants`

### Проблема

`str_constants/src/lib.rs` объединяет пользовательские сообщения, SQL/API names, имена файлов, тестовые шаблоны, proc-macro diagnostics и семантические значения. Формально строка хранится один раз, но изменение доменного значения всё равно требует знания не владеющего им crate-а.

Централизация всех строк не равна единому источнику правды: например, route path должен принадлежать typed route, permission value — permission enum, а имя SQL-колонки — schema model.

### Целевое решение

- Оставить в `str_constants` общие текстовые фрагменты, сообщения и технически переиспользуемые литералы.
- Переместить семантические значения в owning typed APIs.
- Разрешить генераторам получать строку из типа (`as_str`, associated const, metadata), а не импортировать глобальную константу напрямую.
- Добавить code-style policy, запрещающую новые domain constants в `str_constants` без owning type или документированного исключения.

### Критерии готовности

- Для route, permission, config key и SQL identifier можно назвать owning crate и тип.
- Удаление глобальной строки не требует копирования литерала в потребители.
- `str_constants` больше не используется как неявный cross-domain API.

## Рекомендуемый порядок выполнения

1. Ввести shared admin validation policies и conformance tests без изменения публичных DTO.
2. Сделать permission catalog типизированным и добавить DB reconciliation test.
3. Расширить `RouteFamily` до полного contract registry; затем удалить ручное повторение router/OpenAPI metadata.
4. Добавить обязательный PostgreSQL schema conformance для generated table descriptors.
5. Генерировать Rust-owned fixtures для Playwright.
6. Последовательно мигрировать общие string validators.
7. После стабилизации контрактов выделить typed IR генераторов `pg_crud`.
8. В конце сократить `str_constants`, когда у каждого значения уже появился явный владелец.

Такой порядок сначала добавляет проверки против расхождения, затем удаляет дублирование. Это позволяет сохранять поведение на каждом этапе и не требует одномоментного изменения API, базы и frontend.

## Общие правила для каждого этапа

- Сначала определить owning crate и typed model.
- Не генерировать исторические migrations заново.
- Не делать сборку зависимой от работающей PostgreSQL или внешнего сервиса.
- Не объединять независимые контракты в один «универсальный» derive.
- Сохранять отдельные derive для отдельных trait contracts.
- Добавлять compile-time или conformance test до удаления второго представления.
- Не хранить сгенерированные файлы в source tree, если они могут детерминированно создаваться в `target/`.
- Публичные типы мигрировать через совместимый переходный слой, а не переименовывать массово.

## Проверка достижения единого источника правды

Для каждой области итоговый аудит должен отвечать на четыре вопроса:

1. Где находится единственное авторитетное типизированное значение?
2. Какие представления из него генерируются?
3. Какие внешние представления нельзя генерировать и каким conformance-тестом они проверяются?
4. Какой тест падает при намеренном расхождении?

Если на любой вопрос нет конкретного ответа, область ещё не имеет доказанного единого источника правды.
