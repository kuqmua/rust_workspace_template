# План автоматической генерации frontend-кода через существующие proc-macro

## Цель

Сократить ручное дублирование PostgreSQL-, HTTP- и OpenAPI-контрактов в `server_admin_frontend`, используя уже существующие генераторы `gen_pg_types_src` и `gen_pg_tbl_src`. Генерировать следует контрактный и однотипный адаптерный код. Визуальную композицию страниц и бизнес-сценарии лучше оставить обычным Leptos-кодом.

## Текущее состояние

Сейчас `server_admin_frontend/src/app.rs` вручную объявляет:

- DTO `User`, `Role`, `Permission`, `AuditEntry`, `Settings` и `GitInfo`;
- request/response-типы авторизации;
- методы `AdminApiClient` с повторяющимися HTTP path, method, JSON decode и обработкой статусов;
- `Page` со связью «маршрут → permission → loader → view»;
- колонки таблиц и поля формы настроек;
- строки permission вроде `users:read`, `roles:update`;
- JSON payload через `serde_json::json!`, поэтому переименование поля может сломать frontend только во время выполнения.

При этом workspace уже содержит большую часть необходимой информации:

- `gen_pg_types_src` знает Rust/PostgreSQL/wire-типы, nullable-варианты, сериализацию, ограничения и OpenAPI schema;
- `gen_pg_tbl_src` знает поля таблицы, primary key, включённые CRUD-операции, payload/response-типы, HTTP method, path, success status и permission;
- `GenPgTbl` уже генерирует `*ApiClient`, но он использует `reqwest` и поэтому предназначен для native-кода, а не для текущего WASM-клиента на `gloo-net`;
- `GenPgTbl` уже генерирует `*RouteContract::ALL`, `path()`, `authentication()`, `http_method()`, `operation()`, `success_status()` и `mutates()`;
- `newtype::Newtype` уже умеет генерировать `Display`, `From`, `AsRef`, transparent `Debug`, getters и извлечение inner value;
- `newtype::BoundedString` уже умеет общую runtime/Serde-валидацию строк, включая min/max, trim и запрет NUL.

## Что можно использовать уже сейчас

### 1. `newtype::Newtype` для локальных frontend wrapper-типов

Ручные реализации для `Text` и подобных tuple struct можно заменить derive-опциями:

```rust
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, newtype::Newtype)]
#[newtype(display, from_inner)]
struct Text(String);
```

Для wrapper над ссылкой применим `as_ref_inner`, для owned inner type — `as_ref_owned`, `from_inner`, `into_inner`; для прозрачного форматирования — `debug_transparent`.

Ограничение: сейчас `newtype` подключён к `server_admin_frontend` только для native target. Для применения в `app.rs` зависимость потребуется сделать доступной и для `wasm32`, не добавляя новый crate.

### 2. Существующие generated CRUD DTO

Payload/response-типы, которые создаёт `GenPgTbl`, не следует повторно описывать во frontend. Их нужно поместить в доступный обоим target контрактный слой без Axum, SQLx и `reqwest` в публичной WASM-зависимости.

Это уберёт ручные `serde_json::json!` для CRUD и даст compile-time проверку названий и типов полей.

### 3. Существующий route contract

Навигация, permission guards и выбор HTTP method могут опираться на generated `*RouteContract`, а не на строковые литералы. Уже сгенерированная информация достаточна для:

- скрытия недоступных разделов и действий;
- выбора create/read/update/delete permission;
- определения mutating-запросов, которым нужен CSRF header;
- проверки ожидаемого success status;
- построения endpoint path без ручного `format!`/`concat!`.

## Что следует добавить в `gen_pg_types_src`

### 1. Описание frontend wire-типа

Для каждого PostgreSQL-типа генерировать статическую метаинформацию:

```rust
pub enum FrontendInputKind {
    Checkbox,
    Date,
    DateTime,
    Number,
    Text,
    Time,
    Uuid,
}

pub struct FrontendTypeContract {
    pub input_kind: FrontendInputKind,
    pub nullable: bool,
    pub step: Option<&'static str>,
}
```

Она должна выводиться из уже существующих `WireKind`, `FltKind` и `CanBeNl`, а не из нового параллельного списка.

### 2. Парсинг и форматирование полей формы

Для generated wire-wrapper типов добавить единообразные методы:

- `parse_form_value(&str) -> Result<Self, ...>`;
- `format_form_value(&self) -> String`;
- отдельное представление пустого nullable-поля;
- человекочитаемую validation error без знания SQLx во frontend.

Это особенно полезно для UUID, date/time, interval, range, numeric и nullable-типов. Для простого `String` отдельная генерация почти ничего не даёт.

### 3. Ограничения для HTML-контролов

Из уже создаваемой OpenAPI schema переносить в frontend contract:

- `minimum`/`maximum`;
- формат (`date`, `uuid`, `int32`, `int64` и т. п.);
- nullable;
- пример значения;
- строковые min/max, если они известны доменному wrapper-типу.

OpenAPI JSON не нужно парсить в браузере во время выполнения: proc-macro может сгенерировать typed const metadata на этапе компиляции.

## Что следует добавить в `gen_pg_tbl_src`

### 1. WASM API client рядом с native client

На основе тех же `api_client_methods_ts` генерировать browser adapter под `cfg(target_arch = "wasm32")`:

- `gloo_net::http::RequestBuilder`;
- typed request payload;
- typed success response;
- общий decode generated error response;
- автоматический CSRF header для `route_contract.mutates()`;
- единый credential/cookie режим;
- path, method и success status из `RouteContract`.

Native `reqwest`-клиент и WASM-клиент должны строиться из одного внутреннего описания операции, чтобы контракт не расходился.

### 2. Метаданные сущности и полей

Для каждой таблицы генерировать структуры наподобие:

```rust
pub struct FrontendFieldContract {
    pub name: &'static str,
    pub label: &'static str,
    pub is_primary_key: bool,
    pub readable: bool,
    pub creatable: bool,
    pub updatable: bool,
    pub type_contract: FrontendTypeContract,
}
```

Минимальный набор (`name`, CRUD-доступность, тип, nullable) выводится автоматически. `label`, порядок, placeholder и видимость лучше разрешить переопределять атрибутами, например `#[gen_pg_tbl_frontend(label = "Display name")]`.

### 3. Typed form state и payload conversion

Можно генерировать:

- `*CreateForm` и `*UpdateForm` со строковым состоянием HTML input;
- `TryFrom<*CreateForm> for *CreatePayload`;
- `TryFrom<*UpdateForm> for *UpdatePayload`;
- структуру ошибок по полям;
- начальное update-state из read DTO;
- сброс формы после успешного create.

Это заменит ручное чтение сигналов и `serde_json::json!`, сохраняя доменную валидацию generated типов.

### 4. Table column contract

Для read DTO генерировать описание колонок и безопасное извлечение отображаемого значения:

- key/label;
- порядок;
- форматирование;
- sortable/filterable flags;
- тип фильтра;
- возможность скрыть технические поля.

Leptos-компонент таблицы остаётся общим и вручную написанным, а `GenPgTbl` предоставляет только данные колонок. Это лучше, чем генерировать большой `view!`: дизайн останется изменяемым без правки proc-macro.

### 5. CRUD action contract

Из `api_mode`, operation и permission генерировать список доступных действий:

- create/edit/delete;
- bulk и single-row операции;
- required permission;
- confirm requirement;
- success status;
- необходимость CSRF.

Тогда `users_view` и `roles_view` не будут вручную клонировать clients и повторять permission-строки для каждой кнопки.

### 6. Route/page descriptor

Для каждой generated сущности можно создать descriptor:

```rust
pub struct FrontendPageContract {
    pub path: &'static str,
    pub title: &'static str,
    pub read_permission: Option<&'static str>,
    pub fields: &'static [FrontendFieldContract],
    pub actions: &'static [FrontendActionContract],
}
```

По нему общий Leptos CRUD page сможет построить загрузку, toolbar, таблицу, форму и guards. Ручными останутся страницы, не являющиеся табличным CRUD: sign-in, metrics, version и сложные dashboard-сценарии.

### 7. Compile-time проверки frontend-конфигурации

Proc-macro должен выдавать compile error, если:

- editable поле не имеет подходящего frontend parser/input kind;
- frontend action включён, а соответствующая CRUD operation выключена `api_mode`;
- указана сортировка/фильтрация для неподдерживаемого типа;
- два поля получили одинаковый frontend key/order;
- route или permission переопределены несовместимо с generated contract.

## Что не стоит генерировать

- Полный Leptos `view!` для каждой страницы: он жёстко свяжет proc-macro с текущим дизайном и усложнит диагностику ошибок компиляции.
- CSS-классы и визуальную сетку: это ответственность общей библиотеки UI-компонентов.
- Логику sign-in/session redirect: она не выводится из схемы PostgreSQL-таблицы.
- Специальные действия `ban`, `set password`, `assign roles`: они не являются стандартными CRUD-операциями и требуют явного endpoint contract. Их можно генерировать позднее только после появления декларативного описания custom action.
- UI напрямую из OpenAPI во время выполнения: это добавит размер WASM, runtime-ошибки и потерю статической типизации.
- DTO одновременно из структуры БД и отдельно из OpenAPI: должен существовать один источник контракта, иначе дублирование сохранится внутри генераторов.

## Предлагаемая архитектурная граница

Общую generated логику следует разместить в выделенном shared crate, доступном серверу и WASM:

```text
gen_pg_types_src + gen_pg_tbl_src
                 |
                 v
shared generated contract
  - wire DTO
  - operation/route/permission metadata
  - field/type metadata
  - validation and form conversion
        |                         |
        v                         v
server adapter               WASM adapter
Axum/SQLx/reqwest             gloo-net/Leptos
```

Shared contract не должен зависеть от Axum, SQLx, `reqwest`, `gloo-net`, Leptos или `web-sys`. Адаптеры зависят от контракта, но контракт не зависит от адаптеров.

## Порядок внедрения

1. Выделить из `GenPgTbl` внутреннюю модель операции, уже используемую для routes, OpenAPI и native client, и покрыть её snapshot/unit-тестами.
2. Сделать generated DTO и `RouteContract` доступными shared crate без server-only зависимостей.
3. Перевести frontend CRUD-вызовы с `serde_json::json!` на generated typed payload/response.
4. Добавить WASM transport adapter и заменить повторяющиеся методы `AdminApiClient`.
5. Добавить `FrontendTypeContract` в `gen_pg_types_src` и typed form conversion.
6. Добавить field/table/action/page descriptors в `gen_pg_tbl_src`.
7. Создать один общий Leptos CRUD page, принимающий generated descriptor; первым перевести `Roles`, затем `Users`.
8. Только после стабилизации стандартного CRUD добавить декларативные custom actions для ban/password/role assignment.

## Минимальный полезный этап

Первый этап не должен пытаться генерировать весь интерфейс. Достаточно получить:

- shared generated CRUD DTO;
- typed WASM client;
- generated route/method/permission constants;
- typed create/update payload вместо `serde_json::json!`.

Это устранит наиболее опасное дублирование контракта и runtime-ошибки при переименовании полей, при этом внешний вид Leptos-приложения останется независимым от proc-macro.

## Критерии готовности генерации

- Во frontend нет ручных копий generated CRUD DTO.
- Для generated CRUD endpoint нет строковых path, method и permission в `app.rs`.
- Create/update запросы не строятся через `serde_json::json!`.
- Изменение имени или типа поля таблицы либо автоматически обновляет frontend, либо даёт compile error.
- Native client, Axum route, OpenAPI и WASM client используют одну модель операции.
- Generated shared contract компилируется для native и `wasm32-unknown-unknown`.
- Визуальные Leptos-компоненты можно менять без модификации proc-macro.
