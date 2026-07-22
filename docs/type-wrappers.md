# Структуры-обёртки над типами

Этот документ содержит инвентаризацию одно-полевых tuple-структур (`struct Name(Type);`), объявленных в Rust-исходниках workspace. Именно такая форма считается здесь структурой-обёрткой над типом.

Включены production-, test-, bench- и example-модули всех workspace crates. Исключены `target/` и структуры, которые присутствуют только как токены внутри генераторов (`quote!`), поскольку они не являются объявленными items исходного crate.

Это не реестр всех Rust-структур workspace: именованные структуры (`struct Name { field: Type }`), unit-структуры и items, генерируемые только при раскрытии макросов, сюда не входят. Проверка «только через `From`/`TryFrom`» относится к доступной API-границе и фактически найденным путям создания в исходниках. Внутри модуля объявления Rust сохраняет техническую возможность вызвать приватный tuple-конструктор, поэтому абсолютный запрет на уровне языка невозможен без отдельного lint/code-style правила для мест использования.

Обозначения столбцов:

- **I** — только `From`/`TryFrom`: поле закрыто, альтернативный фабричный путь не обнаружен, а `Deserialize`, если он есть, получает сырой тип и передаёт его в `From`/`TryFrom`. Обычный `#[derive(Deserialize)]` непосредственно на wrapper-типе этому условию не соответствует.
- **D** — десериализация нужна: текущая реализация явно предусматривает `Deserialize`, ручной impl или serde-режим вспомогательного макроса.
- **B** — нужен ограничивающий `TryFrom`: непосредственно оборачиваемый сырой owned-тип может неограниченно наращивать содержимое. Ограничение ставится на границе входа сырого типа, а не повторяется поверх готового доменного типа.
- **DT** — нужен `TryFrom` в десериализации: одновременно выполняются **D** и **B**; десериализация должна получить сырой тип и вызвать проверяющий `TryFrom`.
- **FT** — поменять `From` на `TryFrom`: сейчас объявлен `From`/`FromInner`, но выполняется **B**, поэтому инициализация должна стать fallible и проверять верхнюю границу размера.
- **Статус** — **Сделано** для проверенного и исправленного типа, **Не требуется** для проверенного типа без необходимых изменений, **Не проверено** для ещё не разобранного типа.

Borrowed-ссылки и массивы фиксированной длины не считаются неограниченно растущими. **Нет** означает, что соответствующее условие не доказано или не требуется.

Всего структур-обёрток: **949**. **I:** 949 Да / 0 Нет. **D:** 100 Да / 849 Нет. **B:** 277 Да / 672 Нет. **DT:** 61 Да / 888 Нет. **FT:** 93 Да / 856 Нет.

Проверено в коде: **949** типов; исправлено или уже было корректно ограничено: **198**; изменений по ограничению размера не требуется: **751**; типов со статусом **Не проверено** нет. Столбец **Статус** относится к ограничению размера (`B`/`DT`/`FT`), а не к соблюдению правила **I**.

## Итог аудита инициализации

Все **949 из 949** объявленных в исходниках wrapper-структур теперь создаются только через `From`/`TryFrom`: внутренние tuple-поля закрыты, прямые вызовы `Type(value)` и `Self(value)` вне conversion-impl запрещены code-style тестом.

Десериализация также проходит через conversion trait: serde derive использует `#[serde(from = "RawType")]` или `#[serde(try_from = "RawType")]`, а ручные `Deserialize` завершаются вызовом `From`/`TryFrom`. Политика проверяется тестами `tuple_wrappers_initialize_only_through_from_or_try_from`, `tuple_wrappers_do_not_expose_inner_field` и `tuple_wrapper_deserialization_uses_from_or_try_from`.

## Crate `app_state`

### Модуль `app_state`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `SqlxPgPool` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SqlxPgPoolRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
## Crate `common_routes`

### Модуль `common_routes`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `AxumCommonRoutes` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `AxumHealthCheckStatus` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `AxumHttpUriRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `AxumJsonPayload` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `HealthCheckSucceeded` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `HealthComponents` | Да | Нет | Да | Нет | Да | Сделано |
| `HealthDatabaseAvailable` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `NoRouteMessageCapacity` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `NotFoundMessage` | Да | Нет | Да | Нет | Нет | Сделано |
| `OpenApiSpecificationPath` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdArcCommonRoutesAppState` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `UriSuffixRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `UtoipaCommonRoutesOpenApiDocument` | Да | Нет | Нет | Нет | Нет | Не требуется |
## Crate `config_lib`

### Модуль `config_lib`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `AdminAccessTokenTtlSeconds` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `AdminBoolParsingError` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `AdminCookieSecure` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `AdminJwtSecret` | Да | Нет | Да | Нет | Нет | Сделано |
| `AdminPasswordHashConcurrency` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `AdminPositiveU64ParsingError` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `AdminPositiveUsizeParsingError` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `AdminRefreshTokenTtlSeconds` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `AdminSessionLimit` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `AdminSignInRateLimit` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `AdminSwaggerEnabled` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `AdminTokenAudience` | Да | Да | Да | Да | Нет | Сделано |
| `AdminTokenIssuer` | Да | Да | Да | Да | Нет | Сделано |
| `ChronoEastFixedOffset` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `ChronoFixedOffsetError` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `ChronoTimezone` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `ConfigRustTypeName` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `ContentSecurityPolicy` | Да | Нет | Да | Нет | Нет | Сделано |
| `EnvVarName` | Да | Нет | Да | Нет | Да | Сделано |
| `EnvVarNameRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `HttpGzipEnabled` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `MaximumSizeOfHttpBodyInBytes` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `PgPoolAcquireTimeoutSeconds` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `PgPoolIdleTimeoutSeconds` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `PgPoolMaxConnections` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `PgPoolMaxLifetimeSeconds` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `PgPoolMinConnections` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `RequestTimeoutSeconds` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SecrecySecretBoxString` | Да | Нет | Да | Нет | Да | Сделано |
| `StdEnvVarOk` | Да | Нет | Да | Нет | Да | Сделано |
| `StdEnvVarOkRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdI32ParsingError` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdNonZeroU64` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdNonZeroUsize` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdParseBoolError` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdParseIntError` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdU32ParsingError` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdUsizeParsingError` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `TimezoneSeconds` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `TryFromStdEnvVarOkAdminCookieSecureError` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `config_lib::types`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `EnvParseError` | Да | Нет | Да | Нет | Да | Сделано |
| `EnvVarNameRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `EnvVarValueRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `ParseCtxRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdEnvVarResult` | Да | Нет | Да | Нет | Нет | Сделано |
| `TracingLevelName` | Да | Нет | Нет | Нет | Нет | Не требуется |
## Crate `config_lib_macros`

### Модуль `config_lib_macros`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `ProcMacro2TryFromParseFixedErrorTy` | Да | Нет | Да | Нет | Нет | Не требуется |
| `ProcMacro2TryFromParseInput` | Да | Нет | Да | Нет | Нет | Не требуется |
| `ProcMacroTryFromParseTokenStream` | Да | Нет | Да | Нет | Нет | Не требуется |
## Crate `development_data_bootstrap`

### Модуль `development_data_bootstrap`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `DevelopmentIdentityCount` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `DevelopmentIdentitySpecs` | Да | Нет | Да | Нет | Да | Сделано |
## Crate `external_service_emulators`

### Модуль `external_service_emulators`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `RemoteSyncRequestCount` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `TokioMockNotificationReceiver` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `TokioMockNotificationSender` | Да | Нет | Нет | Нет | Нет | Не требуется |
## Crate `file_storage`

### Модуль `file_storage`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `DiskCacheEvictionPlan` | Да | Нет | Да | Нет | Нет | Не требуется |
| `StdDiskCacheModifiedAt` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdDiskCacheSize` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdFileBytes` | Да | Нет | Да | Нет | Нет | Сделано |
| `StdFileStorageIoError` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdFileStorageRoot` | Да | Нет | Да | Нет | Нет | Сделано |
| `StdStaleBefore` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdStaleStagingEntryCount` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdStaleStagingEntryLimit` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdStorageOperationId` | Да | Нет | Да | Нет | Нет | Сделано |
| `StdStoragePathRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdStorageRelativePath` | Да | Нет | Да | Нет | Нет | Сделано |
| `StorageDirectoryNameRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
## Crate `frontend_contract`

### Модуль `frontend_contract`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `ActionContracts` | Да | Нет | Да | Нет | Да | Не требуется |
| `ContractI64` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `ContractStr` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `FieldContracts` | Да | Нет | Да | Нет | Да | Не требуется |
| `FieldOrder` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `FilterContracts` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `FilterWireJson` | Да | Нет | Да | Нет | Нет | Сделано |
| `FormFieldNameRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `FormValue` | Да | Нет | Да | Нет | Нет | Сделано |
| `FormValueError` | Да | Нет | Да | Нет | Нет | Сделано |
| `FormValueRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `RouteContracts` | Да | Нет | Да | Нет | Да | Не требуется |
| `TransportBody` | Да | Нет | Да | Нет | Да | Сделано |
| `TransportError` | Да | Нет | Да | Нет | Нет | Сделано |
| `TransportIdempotencyKey` | Да | Нет | Да | Нет | Нет | Сделано |
| `TransportIfMatch` | Да | Нет | Да | Нет | Нет | Сделано |
| `TransportPath` | Да | Нет | Да | Нет | Нет | Сделано |
| `TransportRetryAfter` | Да | Нет | Да | Нет | Нет | Сделано |
| `TransportStatus` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `frontend_contract::auth_session_keep_alive`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `StdAuthSessionInstant` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdAuthSessionRefreshInterval` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `frontend_contract::json_snapshot`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `JsonContractSnapshot` | Да | Нет | Да | Нет | Нет | Сделано |
| `JsonSnapshotDynamicFieldRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `frontend_contract::openapi_validation`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `OpenApiContractText` | Да | Нет | Да | Нет | Нет | Сделано |
| `OpenApiContractTextError` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `OpenApiResponseStatus` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `RuntimeRoutesRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SerdeJsonOpenApiSerializationError` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `frontend_contract::problem`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `ApiProblemDetail` | Да | Да | Да | Да | Нет | Сделано |
| `ApiProblemField` | Да | Да | Да | Да | Нет | Сделано |
| `ApiProblemRequestId` | Да | Да | Да | Да | Нет | Сделано |
| `ApiProblemStatus` | Да | Да | Нет | Нет | Нет | Не требуется |
| `ApiProblemViolations` | Да | Да | Да | Да | Да | Сделано |
### Модуль `frontend_contract::route`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `OpenApiSecuritySchemeRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `ParameterizedRoutePath` | Да | Нет | Да | Нет | Нет | Сделано |
| `RouteBodyLimit` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `RouteCoverageDescriptors` | Да | Нет | Да | Нет | Да | Не требуется |
| `RouteMetadataList` | Да | Нет | Да | Нет | Да | Не требуется |
| `RouteSchemaContracts` | Да | Нет | Да | Нет | Да | Не требуется |
| `UtoipaOpenApiPathParameter` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `UtoipaOpenApiRouteSchema` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `frontend_contract::route::tests`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `Request` | Да | Да | Нет | Нет | Нет | Не требуется |
| `Response` | Да | Да | Нет | Нет | Нет | Не требуется |
### Модуль `frontend_contract::route_contract_validation`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `HttpContractBody` | Да | Нет | Да | Нет | Да | Сделано |
| `HttpContractStatus` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `RouteContractMismatches` | Да | Нет | Да | Нет | Нет | Не требуется |
### Модуль `frontend_contract::route_coverage`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `RouteTestCategories` | Да | Нет | Да | Нет | Да | Не требуется |
### Модуль `frontend_contract::url_builder`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `ApiUrl` | Да | Нет | Да | Нет | Нет | Сделано |
| `ApiUrlPathSegmentRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `ApiUrlQueryComponentRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
## Crate `frontend_contract_macros`

### Модуль `frontend_contract_macros`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `StdBool` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SynExpr` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SynIdent` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SynRouteRegistryBindings` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SynRouteRegistryHandler` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SynRouteRegistryRoute` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SynRouteRegistryState` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SynType` | Да | Нет | Нет | Нет | Нет | Не требуется |
## Crate `generate_derive_token_stream_builder`

### Модуль `generate_derive_token_stream_builder`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `SnakeCaseString` | Да | Нет | Да | Нет | Нет | Сделано |
| `ToSnakeCaseInput` | Да | Нет | Нет | Нет | Нет | Не требуется |
## Crate `generate_pg_table_src`

### Модуль `generate_pg_table_src::model`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `GeneratePgTableFieldCount` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SynGeneratePgTableModelError` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SynGeneratePgTableModelInput` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `generate_pg_table_src::pipeline`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `SynBuiltGeneratePgTableInput` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SynGeneratePgTablePipelineError` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SynParsedGeneratePgTableInput` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SynValidatedGeneratePgTableInput` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `generate_pg_table_src::source`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `CompileErrorMessage` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `TableTestNames` | Да | Нет | Да | Нет | Да | Не требуется |
## Crate `generate_pg_types_src`

### Модуль `generate_pg_types_src::source`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `GeneratePgTypeRecords` | Да | Да | Да | Да | Нет | Сделано |
| `GeneratePgTypes` | Да | Да | Да | Да | Нет | Сделано |
| `GenerateSecretText` | Да | Да | Нет | Нет | Нет | Не требуется |
| `ParsedGeneratePgTypesConfig` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `PgSqlName` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `PgTypesModelEntryCount` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SerdeJsonGeneratePgTypesError` | Да | Нет | Нет | Нет | Нет | Не требуется |
## Crate `generate_quotes`

### Модуль `generate_quotes`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `ProcMacro2QuotedLiteralTokenStream` | Да | Нет | Да | Нет | Да | Не требуется |
| `QuoteChar` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `QuotePanicId` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `QuotePrefix` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `QuotedLiteral` | Да | Нет | Да | Нет | Нет | Сделано |
## Crate `generate_where_filters_src`

### Модуль `generate_where_filters_src::bind`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `FilterPlaceholderCount` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `generate_where_filters_src::model`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `BindCount` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `FilterSpecValid` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `FilterSqlOperator` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `FilterSqlSuffix` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `generate_where_filters_src::source`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `ProcMacro2GenerateWhereFiltersInput` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `ProcMacro2GenerateWhereFiltersTokenStream` | Да | Нет | Да | Нет | Да | Не требуется |
| `SerdeJsonGenerateWhereFiltersError` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `ValidatedGenerateWhereFiltersConfig` | Да | Нет | Нет | Нет | Нет | Не требуется |
## Crate `git_info`

### Модуль `git_info`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `GitCommitId` | Да | Нет | Да | Нет | Да | Сделано |
| `GitCommitIdFallback` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `GitCommitIdRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `GitCommitLink` | Да | Нет | Да | Нет | Да | Сделано |
| `GitCommitLinkCapacity` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `GitCommitLinkOutputRefMut` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `IsProjectCommit` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `ProjectGitCommitLinkRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdGitCommitIdCow` | Да | Нет | Да | Нет | Да | Сделано |
| `StdGitCommitLinkCow` | Да | Нет | Да | Нет | Да | Сделано |
| `ValidateProjectCommitError` | Да | Нет | Нет | Нет | Нет | Не требуется |
## Crate `location`

### Модуль `location`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `SynItemEnumMutRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
## Crate `location_lib`

### Модуль `location_lib::location`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `ChronoLocationDateTime` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `ChronoLocationDisplayTimezone` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `LocationColumn` | Да | Да | Нет | Нет | Нет | Не требуется |
| `LocationCommit` | Да | Да | Да | Да | Нет | Сделано |
| `LocationFile` | Да | Да | Да | Да | Нет | Сделано |
| `LocationFileRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `LocationLine` | Да | Да | Нет | Нет | Нет | Не требуется |
| `StdFmtRefMut` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdLocationDuration` | Да | Да | Нет | Нет | Нет | Не требуется |
| `StdTimeDurationNanos` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdTimeDurationSecs` | Да | Нет | Нет | Нет | Нет | Не требуется |
## Crate `location_test`

### Модуль `location_test`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `LocationTestCount` | Да | Да | Нет | Нет | Нет | Не требуется |
| `LocationTestFlag` | Да | Да | Нет | Нет | Нет | Не требуется |
| `LocationTestText` | Да | Да | Да | Да | Нет | Сделано |
## Crate `macro_clippy_check_common`

### Модуль `macro_clippy_check_common::tests`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `StdTmpDir` | Да | Нет | Да | Нет | Нет | Не требуется |
## Crate `macros_helpers`

### Модуль `macros_helpers::attr_identifier_str`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `AttrIdentifierName` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `macros_helpers::generate_field_location_new_token_stream`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `FieldLocationColumn` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `FieldLocationFile` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `FieldLocationLine` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `macros_helpers::generate_if_write_is_err_token_stream`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `ProcMacro2IfWriteIsErrTokenStream` | Да | Нет | Да | Нет | Да | Не требуется |
### Модуль `macros_helpers::generate_simple_syn_punct`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `SynPathSegment` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SynPathSegments` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `macros_helpers::generated_rust_token_stream`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `GeneratedRustTokenStream` | Да | Нет | Да | Нет | Да | Не требуется |
### Модуль `macros_helpers::get_macro_attr`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `AttrPathMatches` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `ProcMacro2MacroAttrMetaListTokenStreamRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SynMacroAttrRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `macros_helpers::json_contract`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `JsonFixtureRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SerdeJsonError` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `macros_helpers::location`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `CompileErrorMessage` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SynVariantRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `macros_helpers::location_syn_field`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `SynLocationField` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `macros_helpers::rs_file_path`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `StdRsFilePath` | Да | Нет | Да | Нет | Нет | Не требуется |
### Модуль `macros_helpers::status_code`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `SynStatusCodeVariantRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `macros_helpers::syn_field`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `SynFieldIdentifier` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SynFieldType` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SynFieldVis` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `macros_helpers::test_database`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `SanitizedDatabaseTarget` | Да | Нет | Да | Нет | Нет | Сделано |
| `UrlRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `macros_helpers::test_hlp`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `ExpectedFileContent` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `ExpectedFileContentRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdAssertFilePath` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdAssertFilePathRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `TestPathStem` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `TestPathStemRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `macros_helpers::tool_command`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `StdOsString` | Да | Нет | Да | Нет | Да | Не требуется |
| `StdPathRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdProcessCommand` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdProcessExitStatus` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdProcessOutput` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `ToolArgRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `ToolArgsRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `ToolEnvKeyRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `ToolEnvValueRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `ToolProgramRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `macros_helpers::wrap_derive`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `ProcMacro2DeriveTokensRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `macros_helpers::write_string_into_file`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `ShouldWriteString` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdWrittenFilePath` | Да | Нет | Да | Нет | Да | Не требуется |
| `StdWrittenFilePathRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StringFileContentRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `macros_helpers::write_token_stream_into_file`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `ProcMacro2TokenStreamRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `ShouldWriteTokenStreamFlag` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdRustfmtPath` | Да | Нет | Нет | Нет | Нет | Не требуется |
## Crate `naming`

### Модуль `naming`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `SwaggerUrlPathPrefix` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SwaggerUrlPathSelfQuotesStrValue` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SwaggerUrlPathSelfQuotesTokenStreamValue` | Да | Нет | Нет | Нет | Нет | Не требуется |
## Crate `naming_common`

### Модуль `naming_common`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `CaseString` | Да | Нет | Да | Нет | Нет | Сделано |
| `ConvertCaseKind` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `ProcMacro2CaseTokenStream` | Да | Нет | Да | Нет | Нет | Не требуется |
## Crate `naming_macros`

### Модуль `naming_macros`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `ProcMacro2GeneratedNamingTokenStream` | Да | Нет | Да | Нет | Нет | Не требуется |
| `ProcMacro2VariantMatchingTokensRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SynEnumIdentifierRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
## Crate `newtype`

### Модуль `newtype`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `NewtypeBool` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `ProcMacro2GeneratedTokenStream` | Да | Нет | Да | Нет | Нет | Не требуется |
| `ProcMacroInputTokenStream` | Да | Нет | Да | Нет | Да | Не требуется |
| `SnakeIdentifier` | Да | Нет | Да | Нет | Нет | Сделано |
| `SnakeIdentifierifierLen` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SnakeIdentifierifierTryFromStringError` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SynAttrsRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SynDeriveInputRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SynExpr` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SynIdentifier` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SynIdentifierRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SynType` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SynTypeRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `newtype::tests::newtype::tests`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `CheckedText` | Да | Нет | Да | Нет | Нет | Сделано |
| `DebugValue` | Да | Нет | Да | Нет | Нет | Не требуется |
| `DescribedValue` | Да | Нет | Да | Нет | Нет | Сделано |
| `ExplicitErrorCheckedText` | Да | Нет | Да | Нет | Нет | Сделано |
| `InnerValue` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `InnerVecValue` | Да | Нет | Да | Нет | Да | Не требуется |
| `MutableValueRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `OwnedSliceValue` | Да | Нет | Да | Нет | Да | Не требуется |
| `OwnedValue` | Да | Нет | Да | Нет | Да | Не требуется |
| `ProcMacro2TokenValue` | Да | Нет | Да | Нет | Да | Не требуется |
| `RedactedDebugValue` | Да | Нет | Да | Нет | Нет | Не требуется |
| `ReferentValueRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `RichValue` | Да | Да | Да | Да | Нет | Сделано |
| `SliceValueRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdTransparentErrorValue` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StringValue` | Да | Нет | Да | Нет | Нет | Не требуется |
| `TargetVecValue` | Да | Нет | Да | Нет | Нет | Не требуется |
| `TransparentDebugValue` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `UsizeValue` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `ValidatedValue` | Да | Нет | Да | Нет | Нет | Сделано |
| `VecValue` | Да | Нет | Да | Нет | Нет | Не требуется |
### Модуль `newtype::tests::newtype::tests::to_err_string`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `ToErrStringValue` | Да | Нет | Да | Нет | Да | Не требуется |
## Crate `notification_service`

### Модуль `notification_service`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `AxumNotificationJson` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `AxumNotificationResponse` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `AxumNotificationRouter` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `AxumNotificationState` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `HttpNotificationApiProblem` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `HttpNotificationStatusCode` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `MetricsExporterPrometheusHandle` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `MetricsExporterPrometheusNotificationBuildError` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `NotificationBodyMaximumBytes` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `NotificationConfigError` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `NotificationServeError` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SqlxNotificationDatabaseError` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SqlxNotificationMigrationError` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdNotificationExitCode` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdNotificationIoError` | Да | Нет | Нет | Нет | Нет | Не требуется |
## Crate `notification_service_contract`

### Модуль `notification_service_contract`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `NotificationMessage` | Да | Да | Да | Да | Нет | Сделано |
| `UuidNotificationId` | Да | Да | Нет | Нет | Нет | Не требуется |
## Crate `optml`

### Модуль `optml`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `SynFieldTyWithStaticLts` | Да | Нет | Нет | Нет | Нет | Не требуется |
## Crate `panic_location`

### Модуль `panic_location`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `PanicColumn` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `PanicFile` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `PanicLine` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `PanicWithLocationMessage` | Да | Нет | Да | Нет | Нет | Сделано |
## Crate `pg_crud_common`

### Модуль `pg_crud_common`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `AddOperator` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `AllEnumVariants` | Да | Нет | Да | Нет | Да | Не требуется |
| `EqOperatorQueryStr` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `IsPrimaryKey` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `IsStringEmptyRes` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `NonPrimaryKeyPgTypeReadIds` | Да | Да | Нет | Нет | Нет | Не требуется |
| `NotEmptyUniqueVec` | Да | Нет | Да | Нет | Нет | Сделано |
| `NotZeroUnsignedPartOfI32` | Да | Да | Нет | Нет | Нет | Не требуется |
| `NullableJsonObjPgTypeWhereFilter` | Да | Да | Нет | Нет | Нет | Не требуется |
| `OrderSnakeCaseStr` | Да | Нет | Да | Нет | Да | Сделано |
| `OrderUpperCamelCaseStr` | Да | Нет | Да | Нет | Да | Сделано |
| `PaginationStartsWithZero` | Да | Да | Нет | Нет | Нет | Не требуется |
| `SqlxPostgresQuery` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `UnsignedPartOfI32` | Да | Да | Нет | Нет | Нет | Не требуется |
| `UnsignedPartOfI32Raw` | Да | Да | Нет | Нет | Нет | Не требуется |
| `UuidUuidTestCases` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `pg_crud_common::advisory_lock`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `PgRelationCapacityMaximum` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `PgRelationLockNamespace` | Да | Нет | Да | Нет | Нет | Сделано |
| `PgRelationResourceId` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `PgRelationResourceIds` | Да | Нет | Да | Нет | Нет | Сделано |
| `PgRelationRowCount` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SqlxPgRelationLockConnectionRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SqlxPgRelationLockError` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `pg_crud_common::batch_validation`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `BatchInvalidItemCount` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `BatchInvalidItems` | Да | Нет | Да | Нет | Да | Не требуется |
| `BatchProcessedItemCount` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `BatchStoppedEarly` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdBatchRecords` | Да | Нет | Да | Нет | Да | Не требуется |
### Модуль `pg_crud_common::bind_index`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `QueryPartIncrement` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `pg_crud_common::bounded_btree_map`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `BoundedBTreeMapError` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdBoundedBTreeMap` | Да | Да | Да | Да | Нет | Сделано |
| `StdBoundedBTreeMapLen` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdBoundedBTreeMapVisitor` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `pg_crud_common::bounded_unique_vec`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `BoundedUniqueVec` | Да | Да | Да | Да | Нет | Сделано |
| `StdBoundedUniqueVecVisitor` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `UniqueVecLen` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `pg_crud_common::bounded_vec`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `BoundedVec` | Да | Да | Да | Да | Нет | Сделано |
| `BoundedVecLen` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdPhantomDataBoundedVecVisitor` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `pg_crud_common::cardinality`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `DuplicateCandidates` | Да | Нет | Да | Нет | Да | Не требуется |
| `DuplicateIdx` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `pg_crud_common::cursor`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `CursorMaximumLength` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `CursorPayload` | Да | Нет | Да | Нет | Нет | Сделано |
| `CursorSigningKey` | Да | Нет | Да | Нет | Нет | Сделано |
| `SignedCursor` | Да | Нет | Да | Нет | Нет | Сделано |
### Модуль `pg_crud_common::date_sql_filter`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `ChronoUtcDateTimeRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `ChronoUtcDateTimes` | Да | Нет | Да | Нет | Нет | Не требуется |
| `StdDateSqlBindStart` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `pg_crud_common::db_schema_conformance`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `DbColumnContractSnapshots` | Да | Нет | Да | Нет | Да | Не требуется |
| `DbColumnHasServerDefault` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `DbColumnNullable` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `DbColumnSnapshots` | Да | Нет | Да | Нет | Да | Не требуется |
| `DbColumnSpecs` | Да | Нет | Да | Нет | Да | Не требуется |
| `DbDefaultSpecs` | Да | Нет | Да | Нет | Да | Не требуется |
| `DbKeyContractSnapshots` | Да | Нет | Да | Нет | Да | Не требуется |
| `DbKeySpecs` | Да | Нет | Да | Нет | Да | Не требуется |
| `DbObjectSnapshots` | Да | Нет | Да | Нет | Да | Не требуется |
| `DbObjectSpecs` | Да | Нет | Да | Нет | Да | Не требуется |
| `DbSchemaNameRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `DbSchemaText` | Да | Нет | Да | Нет | Нет | Сделано |
| `DbSchemaTextError` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `DbSchemaTexts` | Да | Нет | Да | Нет | Да | Не требуется |
| `DbStaticSchemaText` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `DbStaticSchemaTexts` | Да | Нет | Да | Нет | Да | Не требуется |
| `DbTableNameRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SqlxDbSchemaInspectionError` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SqlxPgPoolRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `pg_crud_common::errors`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `SqlxPostgresQueryBindError` | Да | Нет | Да | Нет | Да | Сделано |
### Модуль `pg_crud_common::filter_bind_plan`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `PgFilterBindValues` | Да | Нет | Да | Нет | Нет | Не требуется |
| `PgFilterBool` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `PgFilterI64` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `PgFilterText` | Да | Нет | Да | Нет | Нет | Сделано |
### Модуль `pg_crud_common::finite_f64`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `FiniteF64` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `PositiveFiniteF64` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `UnitIntervalF64` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `pg_crud_common::invariants`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `PaginationTotal` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `pg_crud_common::list_total`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `ListItems` | Да | Нет | Да | Нет | Да | Не требуется |
| `ListOffset` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `ListTotal` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `pg_crud_common::operation_budget`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `OperationBudget` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `OperationCount` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `pg_crud_common::operational_invariants`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `PgCounterValue` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `PgOperationalLimit` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `PgScopedForeignKeyClauseText` | Да | Нет | Да | Нет | Нет | Сделано |
| `PgSqlIdentifiers` | Да | Нет | Да | Нет | Да | Не требуется |
### Модуль `pg_crud_common::order_preserving_deduplication`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `OrderPreservingValues` | Да | Нет | Да | Нет | Да | Не требуется |
### Модуль `pg_crud_common::pagination`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `PaginationEnd` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `PaginationLimit` | Да | Да | Нет | Нет | Нет | Не требуется |
| `PaginationOffset` | Да | Да | Нет | Нет | Нет | Не требуется |
| `PaginationStart` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `pg_crud_common::pg_error`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `SqlxPgErrorRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `pg_crud_common::query_fragment`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `QueryPartFragment` | Да | Нет | Да | Нет | Да | Сделано |
| `SqlColumnRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `pg_crud_common::read_query_plan`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `ReadQueryPlan` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SqlSortOrderText` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdReadQueryBindIndex` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `pg_crud_common::sql_identifier`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `SqlIdentifier` | Да | Нет | Да | Нет | Нет | Сделано |
| `SqlIdentifiers` | Да | Нет | Да | Нет | Да | Не требуется |
| `SqlQueryText` | Да | Нет | Да | Нет | Нет | Сделано |
### Модуль `pg_crud_common::sql_like_pattern`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `SqlLikeInputRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SqlLikePattern` | Да | Да | Да | Да | Нет | Сделано |
### Модуль `pg_crud_common::tests_not_empty_unique_vec`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `NonClone` | Да | Нет | Нет | Нет | Нет | Не требуется |
## Crate `pg_crud_macros_common`

### Модуль `pg_crud_macros_common`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `DeLen` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `DimensionNumber` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `GeneratedRustTokenStreamVec` | Да | Нет | Да | Нет | Да | Не требуется |
| `ImportPathStr` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `ImportSnakeCaseStr` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `IsNullablePrefixStr` | Да | Нет | Да | Нет | Нет | Сделано |
| `NonNullOrNullableStr` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `PanicUuidRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `ParseErrorIdRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `ParseTokenStreamStrings` | Да | Нет | Да | Нет | Да | Не требуется |
| `StructElsLen` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SynFieldRefs` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SynIdentifierTypeRefs` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `WrapIntoBraces` | Да | Нет | Нет | Нет | Нет | Не требуется |
## Crate `pg_table`

### Модуль `pg_table`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `PgTableIdempotencyActor` | Да | Нет | Да | Нет | Нет | Сделано |
| `PgTableIdempotencyBody` | Да | Нет | Да | Нет | Да | Сделано |
| `PgTableIdempotencyBodyRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `PgTableIdempotencyCleanupBatchSize` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `PgTableIdempotencyCleanupRetentionSeconds` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `PgTableIdempotencyCleanupRows` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `PgTableIdempotencyKey` | Да | Нет | Да | Нет | Нет | Сделано |
| `PgTableIdempotencyMethod` | Да | Нет | Да | Нет | Нет | Сделано |
| `PgTableIdempotencyRequestHash` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `PgTableIdempotencyResponseStatus` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `PgTableIdempotencyRoute` | Да | Нет | Да | Нет | Нет | Сделано |
| `PgTableIdempotencyTextBytes` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `PgTableNameRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `PgTableQueryPartFragment` | Да | Нет | Да | Нет | Да | Сделано |
| `PgTableQueryString` | Да | Нет | Да | Нет | Да | Сделано |
| `PgTableRevision` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `PgTableSqlFragmentRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SqlxPgTableIdempotencyError` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SqlxPgTablePgConnectionRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdPgTableRevisionParseIntError` | Да | Нет | Нет | Нет | Нет | Не требуется |
## Crate `pg_types_common`

### Модуль `pg_types_common`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `IsPrimaryKey` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `PaginationStartsWithOne` | Да | Да | Нет | Нет | Нет | Не требуется |
| `PaginationStartsWithOneValue` | Да | Да | Нет | Нет | Нет | Не требуется |
## Crate `prepare_postgresql_databases`

### Модуль `prepare_postgresql_databases`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `DatabaseUrl` | Да | Нет | Да | Нет | Нет | Сделано |
| `MigrationsSource` | Да | Нет | Да | Нет | Нет | Сделано |
| `ProcessArguments` | Да | Нет | Да | Нет | Нет | Не требуется |
| `ProcessCommands` | Да | Нет | Да | Нет | Да | Не требуется |
| `ProcessProgram` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `ProcessStaticArgument` | Да | Нет | Нет | Нет | Нет | Не требуется |
## Crate `route_validators`

### Модуль `route_validators`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `AxumHttpStatusCode` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `route_validators::check_body_size`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `AxumBody` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `AxumBodySizeError` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `BodySizeLimitBytes` | Да | Да | Нет | Нет | Нет | Не требуется |
| `BytesBodyBytes` | Да | Нет | Да | Нет | Нет | Сделано |
| `HttpBodySizeHint` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `route_validators::check_commit`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `AxumCommitToStrConversionError` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `CommitNotEqMessage` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `CommitToUse` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `EnableApiGitCommitCheck` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `NoCommitHeaderMessage` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `route_validators::hdr_val`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `AxumHeaderValueRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `AxumHeadersRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `HeaderStrRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `route_validators::test_hlp`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `AxumTestHeaderValue` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `AxumTestHeaders` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `AxumTestHeadersMutRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `TestExpId` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `TestPanicText` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `TestPollCount` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `TestPollLimitReached` | Да | Нет | Нет | Нет | Нет | Не требуется |
## Crate `server`

### Модуль `server`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `AxumApiRoutes` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `MetricsExporterPrometheusBuildError` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `MetricsExporterPrometheusHandle` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `ServerAdminAuthSvcStateBuildError` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `ServerAdminCleanupCfgError` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `ServerAdminMigrateError` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `ServerConfigError` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `ServerRuntimeBackgroundTaskShutdownError` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `ServerRuntimeContentSecurityPolicyError` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `ServerRuntimeRequestTimeoutError` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `ServerRuntimeRunIntervalError` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `ServerRuntimeServeError` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SqlxServerPgConnectError` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdServerExitCode` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdServerIoError` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdSharedServerAppState` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `TokioServerRuntime` | Да | Нет | Нет | Нет | Нет | Не требуется |
## Crate `server_admin`

### Модуль `server_admin`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `AdminAccessTokenError` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `AdminCleanupBatchSize` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `AdminCleanupRetentionSeconds` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `AdminCleanupRows` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `AdminCookieMaxAgeSeconds` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `AdminCookieSecure` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `AdminJwtSecret` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `AdminMigrateError` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `AdminOpaqueToken` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `AdminPassword` | Да | Да | Нет | Нет | Нет | Сделано |
| `AdminPasswordHash` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `AdminPasswordHashConcurrency` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `AdminPermissions` | Да | Нет | Да | Нет | Да | Сделано |
| `AdminRefreshToken` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `AdminRoleNames` | Да | Нет | Да | Нет | Да | Сделано |
| `AdminSessionId` | Да | Да | Нет | Нет | Нет | Не требуется |
| `AdminTokenHash` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `AdminUnixTokenStream` | Да | Да | Нет | Нет | Нет | Не требуется |
| `Argon2AdminPasswordHashError` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `HttpAdminHeaderMapRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `JsonwebtokenAdminError` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SqlxAdminError` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SqlxAdminMigrateError` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdAdminAccessToken` | Да | Нет | Да | Нет | Нет | Сделано |
| `StdAdminCookie` | Да | Нет | Да | Нет | Нет | Сделано |
| `StdAdminSharedSemaphore` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `TokioAdminAcquireError` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `TokioAdminJoinError` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `server_admin::auth`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `AdminHtmlSwaggerEnabled` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `AdminPeerAddr` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `AdminSessionPath` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `AdminSignInJson` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `AxumAdminAuthRouter` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `AxumAdminForm` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `AxumAdminJson` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `AxumAdminPath` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `AxumAdminQuery` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `AxumAdminResponse` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `HttpAdminHeaderMap` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `HttpAdminHeaderValueError` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `JsonwebtokenAdminDecodingKey` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `JsonwebtokenAdminDecodingKeys` | Да | Нет | Да | Нет | Да | Не требуется |
| `JsonwebtokenAdminEncodingKey` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SqlxAdminPgConnectionRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdAdminAccessTtlSeconds` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdAdminFailureDelayMillis` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdAdminFailureThreshold` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdAdminRateLimitCount` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdAdminRateLimitWindowSeconds` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdAdminRefreshTtlSeconds` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdAdminSessionLimit` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdSharedAdminAuthSvcState` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `UtoipaAdminAuthOpenApi` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `server_admin::auth::html`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `AdminHtmlFormKey` | Да | Да | Да | Да | Нет | Сделано |
| `AdminHtmlFormText` | Да | Да | Да | Да | Нет | Сделано |
| `StdAdminHtmlSelected` | Да | Да | Да | Да | Нет | Сделано |
### Модуль `server_admin::domain`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `AdminAuditLogId` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `AdminPermissionId` | Да | Да | Нет | Нет | Нет | Не требуется |
| `AdminPermissionName` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `AdminRoleId` | Да | Да | Нет | Нет | Нет | Не требуется |
| `AdminUserId` | Да | Да | Нет | Нет | Нет | Не требуется |
| `SecrecyAdminString` | Да | Нет | Да | Нет | Да | Сделано |
| `StdAdminBool` | Да | Да | Нет | Нет | Нет | Не требуется |
| `StdAdminNonZeroUsize` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdAdminSocketAddr` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdAdminStrRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdAdminString` | Да | Нет | Да | Нет | Нет | Сделано |
| `UuidAdminValue` | Да | Да | Нет | Нет | Нет | Не требуется |
### Модуль `server_admin::generated_tables`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `UtoipaAdminOpenApi` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `server_admin::repository`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `AdminPageTotalCount` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `AdminRecentLoginFailureCount` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SqlxAdminRepositoryConnectionMutRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SqlxAdminRepositoryPoolRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `server_admin::repository::data_tables`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `DataFltJson` | Да | Нет | Да | Нет | Нет | Сделано |
| `DataPermissionsFlt` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `DataRolePermissionsFlt` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `DataRolesFlt` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `DataSystemSettingsFlt` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `DataUserRolesFlt` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `DataUsersFlt` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `server_admin::repository::roles`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `AdminActiveAdministratorCount` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `server_admin::tests::admin_api`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `AdminHtmlTestBody` | Да | Нет | Да | Нет | Нет | Сделано |
| `AdminHtmlTestFormBody` | Да | Нет | Да | Нет | Нет | Сделано |
| `AxumAdminApiTestRouter` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `HttpAdminApiTestMethod` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `HttpAdminApiTestRequest` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `HttpAdminApiTestResponseRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `HttpAdminHtmlTestResponse` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SqlxAdminApiTestPool` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SqlxAdminHtmlTestTransaction` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdAdminApiTestCookie` | Да | Нет | Да | Нет | Нет | Сделано |
| `StdAdminApiTestStrRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
## Crate `server_admin_contract`

### Модуль `server_admin_contract`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `AdminApiBodyMaxBytes` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `AdminAuditDetailsBytes` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `AdminAuditDetailsTooLarge` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `AdminAuditExportCsv` | Да | Да | Да | Да | Нет | Сделано |
| `AdminAuditLogId` | Да | Да | Нет | Нет | Нет | Не требуется |
| `AdminAuditTimestamp` | Да | Да | Да | Да | Нет | Сделано |
| `AdminAuditViews` | Да | Да | Да | Да | Да | Сделано |
| `AdminBool` | Да | Да | Нет | Нет | Нет | Не требуется |
| `AdminDataColumns` | Да | Да | Да | Да | Да | Сделано |
| `AdminDataFilters` | Да | Да | Да | Да | Да | Сделано |
| `AdminDataRows` | Да | Да | Да | Да | Да | Сделано |
| `AdminDataTableStrRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `AdminDataTables` | Да | Да | Да | Да | Да | Сделано |
| `AdminDefaultRoute` | Да | Да | Да | Да | Нет | Сделано |
| `AdminDisplayName` | Да | Да | Да | Да | Нет | Сделано |
| `AdminFilterField` | Да | Да | Да | Да | Нет | Сделано |
| `AdminFilterOperationKey` | Да | Нет | Да | Нет | Нет | Сделано |
| `AdminFilterValue` | Да | Да | Да | Да | Нет | Сделано |
| `AdminLogin` | Да | Да | Да | Да | Нет | Сделано |
| `AdminMainLogo` | Да | Да | Да | Да | Нет | Сделано |
| `AdminNewPassword` | Да | Да | Да | Да | Нет | Сделано |
| `AdminOptionalSettings` | Да | Да | Да | Да | Да | Сделано |
| `AdminOrganizationContacts` | Да | Да | Да | Да | Нет | Сделано |
| `AdminOrganizationName` | Да | Да | Да | Да | Нет | Сделано |
| `AdminPageLimit` | Да | Да | Нет | Нет | Нет | Не требуется |
| `AdminPageOffset` | Да | Да | Нет | Нет | Нет | Не требуется |
| `AdminPagePathRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `AdminPageTotal` | Да | Да | Нет | Нет | Нет | Не требуется |
| `AdminPassword` | Да | Да | Да | Да | Нет | Сделано |
| `AdminPermissionId` | Да | Да | Нет | Нет | Нет | Не требуется |
| `AdminPermissionIds` | Да | Да | Да | Да | Да | Сделано |
| `AdminPermissionStrRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `AdminPermissionSummaries` | Да | Да | Да | Да | Да | Сделано |
| `AdminPermissionValue` | Да | Да | Да | Да | Нет | Сделано |
| `AdminPermissionValues` | Да | Да | Да | Да | Да | Сделано |
| `AdminPrimaryColor` | Да | Да | Да | Да | Нет | Сделано |
| `AdminRoleId` | Да | Да | Нет | Нет | Нет | Не требуется |
| `AdminRoleIds` | Да | Да | Да | Да | Да | Сделано |
| `AdminRoleName` | Да | Да | Да | Да | Нет | Сделано |
| `AdminRoleNames` | Да | Да | Да | Да | Да | Сделано |
| `AdminRoleSummaries` | Да | Да | Да | Да | Да | Сделано |
| `AdminRoutePath` | Да | Нет | Да | Нет | Нет | Сделано |
| `AdminSessionIdentifier` | Да | Да | Да | Да | Нет | Сделано |
| `AdminSessionTimestamp` | Да | Да | Да | Да | Нет | Сделано |
| `AdminSessionViews` | Да | Да | Да | Да | Да | Сделано |
| `AdminSiteName` | Да | Да | Да | Да | Нет | Сделано |
| `AdminSupportUrl` | Да | Да | Да | Да | Нет | Сделано |
| `AdminTabTitle` | Да | Да | Да | Да | Нет | Сделано |
| `AdminTableSearch` | Да | Да | Да | Да | Нет | Сделано |
| `AdminTableSortKey` | Да | Да | Да | Да | Нет | Сделано |
| `AdminTableSortKeyRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `AdminText` | Да | Да | Да | Да | Нет | Сделано |
| `AdminTexts` | Да | Да | Да | Да | Да | Сделано |
| `AdminUserId` | Да | Да | Нет | Нет | Нет | Не требуется |
| `AdminUserSummaries` | Да | Да | Да | Да | Да | Сделано |
| `SerdeJsonAdminAuditDetails` | Да | Да | Да | Да | Нет | Сделано |
## Crate `server_admin_frontend`

### Модуль `server_admin_frontend`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `AxumAdminFrontendRouter` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `server_admin_frontend::ssr`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `AdminSsrErrorMessage` | Да | Нет | Да | Нет | Нет | Сделано |
| `AdminSsrHtml` | Да | Нет | Да | Нет | Нет | Сделано |
| `AdminSsrText` | Да | Нет | Да | Нет | Нет | Сделано |
## Crate `server_runtime`

### Модуль `server_runtime`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `AxumRouter` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `HttpContentSecurityPolicy` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `RequestTimeoutLayer` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `RequestTimeoutTowerLayer` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `ReqwestClient` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `ReqwestClientBuildError` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdRequestTimeoutMessage` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdReqwestConnectTimeout` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdReqwestRequestTimeout` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdServeIoError` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `TokioTcpListener` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `server_runtime::batched_cleanup`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `CleanupBatchCount` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `CleanupBatchSize` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `CleanupRows` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `server_runtime::bounded_read`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `BoundedBytes` | Да | Нет | Да | Нет | Нет | Сделано |
| `BoundedJsonText` | Да | Нет | Да | Нет | Нет | Сделано |
| `BoundedReadMaximumBytes` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `BoundedReadObservedBytes` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `BoundedText` | Да | Нет | Да | Нет | Нет | Сделано |
| `ReqwestError` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `ReqwestResponse` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SerdeJsonError` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdBoundedReadConcurrency` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdBoundedReadConcurrencyMaximum` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdFromUtf8Error` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdIoError` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdPathRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `server_runtime::child_process`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `ChildDiagnostic` | Да | Нет | Да | Нет | Нет | Сделано |
| `ChildProcessId` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `ChildProcessReports` | Да | Нет | Да | Нет | Нет | Сделано |
| `StdChildDiagnosticMaximum` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdChildExitStatus` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdChildProcessIoError` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdChildProcessSetMaximum` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdCollectionsChildProcessMap` | Да | Нет | Да | Нет | Нет | Сделано |
| `TokioChildDiagnosticTask` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `TokioChildProcess` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `TokioChildProcessJoinError` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `TokioManagedChild` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `server_runtime::client_ip`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `HttpHeaderMapRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdAddrParseError` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdIpAddr` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdParseIntError` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdRangeContains` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdResolvedClientIp` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdSocketAddr` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdTrustedProxyPrefixBits` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `TrustedProxyRanges` | Да | Нет | Да | Нет | Да | Сделано |
### Модуль `server_runtime::cors`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `HttpCorsAllowOriginHeaderValues` | Да | Нет | Да | Нет | Нет | Сделано |
| `HttpCorsAllowOriginTextRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `server_runtime::csp`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `HttpCspBuilder` | Да | Нет | Да | Нет | Нет | Сделано |
| `HttpCspDirectiveName` | Да | Нет | Да | Нет | Нет | Сделано |
| `HttpCspDirectiveValue` | Да | Нет | Да | Нет | Нет | Сделано |
### Модуль `server_runtime::deduplicating_queue`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `StdCollectionsHashSet` | Да | Нет | Да | Нет | Да | Не требуется |
| `StdCollectionsVecDeque` | Да | Нет | Да | Нет | Да | Не требуется |
| `StdQueueMaximum` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `server_runtime::exclusive_run`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `StdExclusiveRunAtomicBool` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `server_runtime::fallback`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `AcceptsApplicationJson` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `HttpAcceptHeaderMaximumBytes` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `HttpFallbackApiPrefixRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `HttpFallbackMetricsPathRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `HttpFallbackRequestPathRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `HttpMediaRangeRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `HttpOptionalAcceptHeaderRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `server_runtime::generation_gate`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `Generation` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdGenerationAtomicU64` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `server_runtime::geojson`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `GeoJsonDocumentText` | Да | Нет | Да | Нет | Нет | Сделано |
| `SerdeJsonGeoJsonError` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `server_runtime::header_text`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `HttpHeaderName` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `HttpHeaderTextBytes` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `HttpHeaderTextMaximumBytes` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `HttpHeaderTextRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `server_runtime::health`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `HealthProbeSucceeded` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdHealthProbeTimeout` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdHealthReadinessAtomicBool` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdSharedHealthReadiness` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `server_runtime::history`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `StdArcSharedRunReports` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdAsyncRunHistoryMaximumLen` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdAsyncRunHistoryReportCount` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdVecDequeRunReports` | Да | Нет | Да | Нет | Нет | Сделано |
| `TokioRwLockRunReports` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `server_runtime::http_header_policy`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `HttpAttachmentFileNameRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `HttpContentDisposition` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `HttpContentLength` | Да | Нет | Да | Нет | Нет | Сделано |
### Модуль `server_runtime::http_policy`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `HttpAuthorizationHeaderTextRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `HttpBearerTokenRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `HttpContentTypeTextRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `HttpCookieHeadersRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `HttpCookieNameRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `HttpCookieValueRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `server_runtime::http_status_error`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `HttpErrorStatus` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `server_runtime::lease_registry`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `LeaseId` | Да | Нет | Да | Нет | Нет | Сделано |
| `LeaseIds` | Да | Нет | Да | Нет | Нет | Сделано |
| `LeaseKey` | Да | Нет | Да | Нет | Нет | Сделано |
| `LeaseTextRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdArcTokioLeaseRegistryRwLock` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdLeaseRegistryMaximum` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdLeaseStaleTimeout` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `TokioLeaseInstant` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `TokioLeaseRegistryRwLock` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `server_runtime::lifecycle`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `StdRequestTimeout` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdRunInterval` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `TokioAbortTask` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `TokioBackgroundTaskJoinHandle` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `TokioBackgroundTaskShutdownSender` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `TokioTaskJoinError` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `server_runtime::limits`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `RetryAfterSecs` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdArcTokioSemaphore` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdPermitWaitTimeout` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdSemaphorePermitCount` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `TokioAcquireError` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `TokioOwnedSemaphorePermit` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `server_runtime::metrics_layer`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `HttpMetricsPathCacheMaximum` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `HttpMetricsPathText` | Да | Нет | Да | Нет | Нет | Сделано |
| `HttpMetricsPathTextRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `MetricsResponseBody` | Да | Нет | Да | Нет | Нет | Сделано |
| `MetricsSharedString` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdHttpMetricsPathEntries` | Да | Нет | Да | Нет | Нет | Сделано |
| `StdSharedHttpMetricsPathCache` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `server_runtime::multipart`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `FileStagingDirectoryName` | Да | Нет | Да | Нет | Нет | Сделано |
| `MultipartBytes` | Да | Нет | Да | Нет | Нет | Сделано |
| `MultipartBytesParts` | Да | Нет | Да | Нет | Нет | Сделано |
| `MultipartFieldName` | Да | Нет | Да | Нет | Нет | Сделано |
| `MultipartFileName` | Да | Нет | Да | Нет | Нет | Сделано |
| `MultipartPayloadMaximum` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `MultipartTextParts` | Да | Нет | Да | Нет | Нет | Сделано |
| `MultipartTextValue` | Да | Нет | Да | Нет | Нет | Сделано |
| `MultipartValueLength` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdStorageRelativePath` | Да | Нет | Да | Нет | Нет | Сделано |
| `StoragePathSegment` | Да | Нет | Да | Нет | Нет | Сделано |
### Модуль `server_runtime::notification`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `AxumNotificationJson` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `AxumNotificationRouter` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `HttpNotificationHeaderMap` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `NotificationApiToken` | Да | Нет | Да | Нет | Нет | Сделано |
| `NotificationApiTokenAuthorized` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `NotificationApiTokenRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `NotificationMessage` | Да | Да | Да | Да | Нет | Сделано |
### Модуль `server_runtime::origin`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `AllowOriginSuffix` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `AllowedOrigins` | Да | Нет | Да | Нет | Нет | Сделано |
| `HttpOriginAuthorityText` | Да | Нет | Да | Нет | Нет | Сделано |
| `HttpOriginHeadersRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `HttpOriginSchemeText` | Да | Нет | Да | Нет | Нет | Сделано |
| `HttpOriginTextRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `RequestOriginAllowed` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `server_runtime::outbound_url`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `OutboundAllowedHost` | Да | Нет | Да | Нет | Нет | Сделано |
| `OutboundHostAllowlist` | Да | Нет | Да | Нет | Нет | Сделано |
| `OutboundUrlTextRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `ReqwestOutboundUrl` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdOutboundIpAddr` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `server_runtime::path_policy`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `HttpAllowedPathPrefixRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `HttpNormalizedPath` | Да | Нет | Да | Нет | Нет | Сделано |
| `HttpProxyPath` | Да | Нет | Да | Нет | Нет | Сделано |
| `HttpProxyPathPrefixMatch` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `HttpProxyPathRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `HttpRequestPathRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `server_runtime::pg_rate_limit`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `PgRateLimitMaximum` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `PgRateLimitQueryRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `PgRateLimitScopeRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `PgRateLimitSubjectRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `PgRateLimitWindowSeconds` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SqlxPgRateLimitError` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SqlxPgRateLimitPoolRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `server_runtime::redacted_url`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `RedactedUrl` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `RedactedUrlTextRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `server_runtime::request_id`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `HttpHeaderToStrError` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `RequestId` | Да | Нет | Да | Нет | Нет | Сделано |
### Модуль `server_runtime::resource_budget`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `ResourceBudgetAmount` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `ResourceBudgetMaximum` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdAtomicUsize` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdSharedAtomicUsize` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `server_runtime::resource_utilization`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `ResourceAmount` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `ResourceUtilizationPercent` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `server_runtime::retry`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `StdRetryAttempts` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdRetryDelay` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `server_runtime::secret_text`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `BoundedSecretText` | Да | Нет | Да | Нет | Нет | Сделано |
| `SecretTextRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `server_runtime::secure_cookie`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `HttpCookieName` | Да | Нет | Да | Нет | Нет | Сделано |
| `HttpCookieValue` | Да | Нет | Да | Нет | Нет | Сделано |
| `HttpSetCookieHeaderValue` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdCookieMaxAgeSeconds` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `server_runtime::service_bootstrap`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `StdServiceRuntimeIoError` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `TokioServiceRuntime` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `TracingSubscriberInitError` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `server_runtime::single_flight`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `SingleFlightKey` | Да | Нет | Да | Нет | Нет | Сделано |
| `SingleFlightWaiter` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdArcStdSingleFlightRwLock` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdSingleFlightMaximum` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdSingleFlightRwLock` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdSingleFlightWriteGuard` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `TokioSingleFlightReceiver` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `TokioSingleFlightSender` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `server_runtime::trace_context`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `HttpTraceParent` | Да | Нет | Да | Нет | Нет | Сделано |
| `HttpTraceState` | Да | Нет | Да | Нет | Нет | Сделано |
| `ReqwestRequestBuilder` | Да | Нет | Нет | Нет | Нет | Не требуется |
## Crate `str_constants_macros`

### Модуль `str_constants_macros`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `SynIdent` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SynLitStr` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SynVisibility` | Да | Нет | Нет | Нет | Нет | Не требуется |
## Crate `synchronization_service_runtime`

### Модуль `synchronization_service_runtime`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `SynchronizationPayload` | Да | Нет | Да | Нет | Да | Сделано |
## Crate `tests`

### Модуль `tests::code_style::types`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `AnalyzerBool` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `AnalyzerChar` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `AnalyzerCount` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `CargoMetadata` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `CargoMetadataRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `CargoTomlFileIdx` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `DiagnosticMsgs` | Да | Нет | Да | Нет | Нет | Не требуется |
| `DiagnosticMsgsMutRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SourceText` | Да | Нет | Да | Нет | Нет | Не требуется |
| `SourceTextList` | Да | Нет | Да | Нет | Да | Не требуется |
| `SourceTextListRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SourceTextRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StaticStr` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StaticStrSliceRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdCargoPackageIdRefSet` | Да | Нет | Да | Нет | Да | Не требуется |
| `StdPathBuf` | Да | Нет | Да | Нет | Да | Не требуется |
| `StdPathRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdProcessOutputRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdSourceTextHashSet` | Да | Нет | Да | Нет | Да | Не требуется |
| `StdSourceTextRefSet` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdSourceTextSet` | Да | Нет | Да | Нет | Да | Не требуется |
| `StdStdSourceTextSetRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SynAttributeListRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SynAttributeRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SynBlockRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SynExprCallRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SynFieldsRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SynFile` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SynFileRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SynGenericsRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SynIdentifierRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SynItemFnRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SynItemImplRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SynItemRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SynItemStructRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SynPathArgumentsRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SynPathRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SynPathSegmentRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SynSignatureRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SynTypePathRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SynTypeRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SynUseTreeRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `TomlTable` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `TomlTableRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `TomlValue` | Да | Нет | Да | Нет | Да | Не требуется |
| `TomlValueRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `WalkdirWalkDir` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `tests::domain_type_policy_fixture`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `DomainEvents` | Да | Нет | Да | Нет | Нет | Не требуется |
| `DomainId` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `DomainName` | Да | Нет | Да | Нет | Да | Не требуется |
## Crate `text_policy`

### Модуль `text_policy`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `FixedLengthAsciiHexText` | Да | Нет | Да | Нет | Нет | Сделано |
| `NonEmptyTrimmedText` | Да | Нет | Да | Нет | Нет | Сделано |
| `PasswordLength` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `PasswordTextRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `RequiredNulFreeBoundedText` | Да | Нет | Да | Нет | Нет | Сделано |
| `UrlSafeTokenPartMaximumBytes` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `UrlSafeTokenPartRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `UrlSafeTokenPartText` | Да | Нет | Да | Нет | Нет | Сделано |
## Crate `to_err_string`

### Модуль `to_err_string`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `StaticStrToOwnedInput` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `ToErrStringValue` | Да | Нет | Да | Нет | Нет | Сделано |
## Crate `token_patterns`

### Модуль `token_patterns`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `ProcMacro2TokensMut` | Да | Нет | Нет | Нет | Нет | Не требуется |
## Crate `token_patterns_macros`

### Модуль `token_patterns_macros`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `ProcMacro2GenerateTpInput` | Да | Нет | Да | Нет | Нет | Не требуется |
| `ProcMacro2GenerateTpOutput` | Да | Нет | Да | Нет | Нет | Не требуется |
## Crate `where_filters`

### Модуль `where_filters`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `BoundedVec` | Да | Да | Да | Да | Нет | Сделано |
| `BoundedVecLen` | Да | Да | Нет | Нет | Нет | Не требуется |
| `PgTypeNotEmptyUniqueVec` | Да | Нет | Да | Нет | Нет | Сделано |
| `RegexCasePostgreqlSyntax` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `RegexRegex` | Да | Да | Нет | Нет | Нет | Не требуется |
### Модуль `where_filters::tests`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `NonClone` | Да | Нет | Нет | Нет | Нет | Не требуется |
## Crate `workspace_macro_helpers`

### Модуль `workspace_macro_helpers`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `FirstCommaStripped` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `FirstIdentifier` | Да | Нет | Да | Нет | Да | Сделано |
| `FirstIdentifierifierTryFromStringError` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `PartIndex` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `ProcMacro2MacroTokens` | Да | Нет | Да | Нет | Да | Не требуется |
| `ProcMacro2TopLevelCommaParts` | Да | Нет | Да | Нет | Нет | Не требуется |
| `StdUniqueOptionSet` | Да | Нет | Да | Нет | Нет | Не требуется |
| `StdUniqueOptionSetContains` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdUniqueOptionSetIsEmpty` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SynDeriveInputRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SynFieldsNamedRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SynFieldsUnnamedRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `TopLevelCommaPart` | Да | Нет | Нет | Нет | Нет | Не требуется |
## Crate `workspace_scaffold`

### Модуль `workspace_scaffold`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `ProjectNameRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `RepositoryUrlRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `ServicePort` | Да | Нет | Нет | Нет | Нет | Не требуется |
## Crate `workspace_test_runner`

### Модуль `workspace_test_runner`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `AnsiTextRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `CargoArgs` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `CleanAnsiText` | Да | Нет | Да | Нет | Нет | Сделано |
| `MeasurementName` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `MemusageColumnIdx` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `MemusageKey` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `MemusageProgNameRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `MemusageRowName` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `MemusageValueRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `ProgramArgsRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `ProgramPathRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `QuoteTokenStreamGeneratePgTableMeasureInputTokenStream` | Да | Нет | Да | Нет | Нет | Не требуется |
| `StderrTextRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `ToolName` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `ToolPath` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `workspace_test_runner::execution`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `CommandIdx` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `CommandStartedAt` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `RunDir` | Да | Нет | Да | Нет | Нет | Не требуется |
| `SummaryText` | Да | Нет | Да | Нет | Нет | Не требуется |
