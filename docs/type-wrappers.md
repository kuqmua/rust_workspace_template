# Структуры-обёртки над типами

Этот документ содержит инвентаризацию одно-полевых tuple-структур (`struct Name(Type);`), объявленных в Rust-исходниках workspace. Именно такая форма считается здесь структурой-обёрткой над типом.

Включены production-, test-, bench- и example-модули всех workspace crates. Исключены `target/` и структуры, которые присутствуют только как токены внутри генераторов (`quote!`), поскольку они не являются объявленными items исходного crate.

Обозначения столбцов:

- **I** — только `From`/`TryFrom`/десериализация: найден разрешённый путь инициализации, поле закрыто и альтернативный фабричный путь не обнаружен.
- **D** — десериализация нужна: текущая реализация явно предусматривает `Deserialize`, ручной impl или serde-режим вспомогательного макроса.
- **B** — нужен ограничивающий `TryFrom`: непосредственно оборачиваемый сырой owned-тип может неограниченно наращивать содержимое. Ограничение ставится на границе входа сырого типа, а не повторяется поверх готового доменного типа.
- **DT** — нужен `TryFrom` в десериализации: одновременно выполняются **D** и **B**; десериализация должна получить сырой тип и вызвать проверяющий `TryFrom`.
- **FT** — поменять `From` на `TryFrom`: сейчас объявлен `From`/`FromInner`, но выполняется **B**, поэтому инициализация должна стать fallible и проверять верхнюю границу размера.
- **Статус** — **Сделано** для проверенного и исправленного типа, **Не требуется** для проверенного типа без необходимых изменений, **Не проверено** для ещё не разобранного типа.

Borrowed-ссылки и массивы фиксированной длины не считаются неограниченно растущими. **Нет** означает, что соответствующее условие не доказано или не требуется.

Всего структур-обёрток: **930**. **I:** 483 Да / 447 Нет. **D:** 96 Да / 834 Нет. **B:** 270 Да / 660 Нет. **DT:** 57 Да / 873 Нет. **FT:** 91 Да / 839 Нет.

Проверено в коде: **930** типов; исправлено или уже было корректно ограничено: **191**; изменений не требуется: **739**; типов со статусом **Не проверено** нет.

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
| `AxumCommonRoutes` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `AxumHealthCheckStatus` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `AxumHttpUriRef` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `AxumJsonPayload` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `HealthCheckSucceeded` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `HealthComponents` | Да | Нет | Да | Нет | Да | Сделано |
| `HealthDatabaseAvailable` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `NoRouteMessageCapacity` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `NotFoundMessage` | Да | Нет | Да | Нет | Нет | Сделано |
| `OpenApiSpecificationPath` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `StdArcCommonRoutesAppState` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `UriSuffixRef` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `UtoipaCommonRoutesOpenApiDocument` | Нет | Нет | Нет | Нет | Нет | Не требуется |
## Crate `config_lib`

### Модуль `config_lib`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `AdminAccessTokenTtlSeconds` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `AdminBoolParsingError` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `AdminCookieSecure` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `AdminJwtSecret` | Нет | Нет | Да | Нет | Нет | Сделано |
| `AdminPasswordHashConcurrency` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `AdminPositiveU64ParsingError` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `AdminPositiveUsizeParsingError` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `AdminRefreshTokenTtlSeconds` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `AdminSessionLimit` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `AdminSignInRateLimit` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `AdminSwaggerEnabled` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `AdminTokenAudience` | Да | Да | Да | Да | Нет | Сделано |
| `AdminTokenIssuer` | Да | Да | Да | Да | Нет | Сделано |
| `ChronoEastFixedOffset` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `ChronoFixedOffsetError` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `ChronoTimezone` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `ConfigRustTypeName` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `ContentSecurityPolicy` | Да | Нет | Да | Нет | Нет | Сделано |
| `EnvVarName` | Да | Нет | Да | Нет | Да | Сделано |
| `EnvVarNameRef` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `HttpGzipEnabled` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `MaximumSizeOfHttpBodyInBytes` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `PgPoolAcquireTimeoutSeconds` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `PgPoolIdleTimeoutSeconds` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `PgPoolMaxConnections` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `PgPoolMaxLifetimeSeconds` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `PgPoolMinConnections` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `RequestTimeoutSeconds` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `SecrecySecretBoxString` | Да | Нет | Да | Нет | Да | Сделано |
| `StdEnvVarOk` | Нет | Нет | Да | Нет | Да | Сделано |
| `StdEnvVarOkRef` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `StdI32ParsingError` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `StdNonZeroU64` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdNonZeroUsize` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdParseBoolError` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdParseIntError` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdU32ParsingError` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `StdUsizeParsingError` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `TimezoneSeconds` | Нет | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `config_lib::types`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `EnvParseError` | Да | Нет | Да | Нет | Да | Сделано |
| `EnvVarNameRef` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `EnvVarValueRef` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `ParseCtxRef` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `StdEnvVarResult` | Нет | Нет | Да | Нет | Нет | Сделано |
| `TracingLevelName` | Нет | Нет | Нет | Нет | Нет | Не требуется |
## Crate `config_lib_macros`

### Модуль `config_lib_macros`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `ProcMacro2TryFromParseFixedErrorTy` | Нет | Нет | Да | Нет | Нет | Не требуется |
| `ProcMacro2TryFromParseInput` | Нет | Нет | Да | Нет | Нет | Не требуется |
| `ProcMacroTryFromParseTokenStream` | Нет | Нет | Да | Нет | Нет | Не требуется |
## Crate `development_data_bootstrap`

### Модуль `development_data_bootstrap`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `DevelopmentIdentityCount` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `DevelopmentIdentitySpecs` | Да | Нет | Да | Нет | Да | Сделано |
## Crate `external_service_emulators`

### Модуль `external_service_emulators`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `RemoteSyncRequestCount` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `TokioMockNotificationReceiver` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `TokioMockNotificationSender` | Нет | Нет | Нет | Нет | Нет | Не требуется |
## Crate `file_storage`

### Модуль `file_storage`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `DiskCacheEvictionPlan` | Нет | Нет | Да | Нет | Нет | Не требуется |
| `StdDiskCacheModifiedAt` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdDiskCacheSize` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdFileBytes` | Да | Нет | Да | Нет | Нет | Сделано |
| `StdFileStorageIoError` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdFileStorageRoot` | Да | Нет | Да | Нет | Нет | Сделано |
| `StdStaleBefore` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdStaleStagingEntryCount` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `StdStaleStagingEntryLimit` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdStorageOperationId` | Да | Нет | Да | Нет | Нет | Сделано |
| `StdStoragePathRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdStorageRelativePath` | Да | Нет | Да | Нет | Нет | Сделано |
| `StorageDirectoryNameRef` | Нет | Нет | Нет | Нет | Нет | Не требуется |
## Crate `frontend_contract`

### Модуль `frontend_contract`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `ActionContracts` | Да | Нет | Да | Нет | Да | Не требуется |
| `ContractI64` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `ContractStr` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `FieldContracts` | Да | Нет | Да | Нет | Да | Не требуется |
| `FieldOrder` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `FormValue` | Нет | Нет | Да | Нет | Нет | Сделано |
| `FormValueError` | Нет | Нет | Да | Нет | Нет | Сделано |
| `FormValueRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `RouteContracts` | Да | Нет | Да | Нет | Да | Не требуется |
| `TransportBody` | Да | Нет | Да | Нет | Да | Сделано |
| `TransportError` | Нет | Нет | Да | Нет | Нет | Сделано |
| `TransportIdempotencyKey` | Да | Нет | Да | Нет | Нет | Сделано |
| `TransportIfMatch` | Да | Нет | Да | Нет | Нет | Сделано |
| `TransportPath` | Нет | Нет | Да | Нет | Нет | Сделано |
| `TransportRetryAfter` | Да | Нет | Да | Нет | Нет | Сделано |
| `TransportStatus` | Нет | Нет | Нет | Нет | Нет | Не требуется |
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
| `OpenApiContractTextError` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `OpenApiResponseStatus` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `RuntimeRoutesRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SerdeJsonOpenApiSerializationError` | Нет | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `frontend_contract::problem`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `ApiProblemDetail` | Нет | Да | Да | Да | Нет | Сделано |
| `ApiProblemField` | Нет | Да | Да | Да | Нет | Сделано |
| `ApiProblemRequestId` | Нет | Да | Да | Да | Нет | Сделано |
| `ApiProblemStatus` | Да | Да | Нет | Нет | Нет | Не требуется |
| `ApiProblemViolations` | Нет | Да | Да | Да | Да | Сделано |
### Модуль `frontend_contract::route`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `OpenApiSecuritySchemeRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `ParameterizedRoutePath` | Нет | Нет | Да | Нет | Нет | Сделано |
| `RouteBodyLimit` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `RouteCoverageDescriptors` | Нет | Нет | Да | Нет | Да | Не требуется |
| `RouteMetadataList` | Нет | Нет | Да | Нет | Да | Не требуется |
| `RouteSchemaContracts` | Нет | Нет | Да | Нет | Да | Не требуется |
| `UtoipaOpenApiPathParameter` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `UtoipaOpenApiRouteSchema` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `frontend_contract::route::tests`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `Request` | Нет | Да | Нет | Нет | Нет | Не требуется |
| `Response` | Нет | Да | Нет | Нет | Нет | Не требуется |
### Модуль `frontend_contract::route_contract_validation`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `HttpContractBody` | Да | Нет | Да | Нет | Да | Сделано |
| `HttpContractStatus` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `RouteContractMismatches` | Нет | Нет | Да | Нет | Нет | Не требуется |
### Модуль `frontend_contract::route_coverage`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `RouteTestCategories` | Нет | Нет | Да | Нет | Да | Не требуется |
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
| `StdBool` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `SynExpr` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SynIdent` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SynRouteRegistryBindings` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `SynRouteRegistryHandler` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `SynRouteRegistryRoute` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `SynRouteRegistryState` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `SynType` | Да | Нет | Нет | Нет | Нет | Не требуется |
## Crate `generate_derive_token_stream_builder`

### Модуль `generate_derive_token_stream_builder`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `SnakeCaseString` | Да | Нет | Да | Нет | Нет | Сделано |
| `ToSnakeCaseInput` | Нет | Нет | Нет | Нет | Нет | Не требуется |
## Crate `generate_pg_table_src`

### Модуль `generate_pg_table_src::model`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `GeneratePgTableFieldCount` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `SynGeneratePgTableModelError` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SynGeneratePgTableModelInput` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `generate_pg_table_src::pipeline`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `SynBuiltGeneratePgTableInput` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `SynGeneratePgTablePipelineError` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `SynParsedGeneratePgTableInput` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `SynValidatedGeneratePgTableInput` | Нет | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `generate_pg_table_src::source`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `CompileErrorMessage` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `TableTestNames` | Да | Нет | Да | Нет | Да | Не требуется |
## Crate `generate_pg_types_src`

### Модуль `generate_pg_types_src::source`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `GeneratePgTypeRecords` | Да | Да | Да | Да | Нет | Сделано |
| `GeneratePgTypes` | Да | Да | Да | Да | Нет | Сделано |
| `GenerateSecretText` | Нет | Да | Нет | Нет | Нет | Не требуется |
| `ParsedGeneratePgTypesConfig` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `PgSqlName` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `PgTypesModelEntryCount` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `SerdeJsonGeneratePgTypesError` | Нет | Нет | Нет | Нет | Нет | Не требуется |
## Crate `generate_quotes`

### Модуль `generate_quotes`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `ProcMacro2QuotedLiteralTokenStream` | Да | Нет | Да | Нет | Да | Не требуется |
| `QuoteChar` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `QuotePanicId` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `QuotePrefix` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `QuotedLiteral` | Да | Нет | Да | Нет | Нет | Сделано |
## Crate `generate_where_filters_src`

### Модуль `generate_where_filters_src::bind`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `FilterPlaceholderCount` | Нет | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `generate_where_filters_src::model`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `BindCount` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `FilterSpecValid` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `FilterSqlOperator` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `FilterSqlSuffix` | Нет | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `generate_where_filters_src::source`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `ProcMacro2GenerateWhereFiltersInput` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `ProcMacro2GenerateWhereFiltersTokenStream` | Да | Нет | Да | Нет | Да | Не требуется |
| `SerdeJsonGenerateWhereFiltersError` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `ValidatedGenerateWhereFiltersConfig` | Нет | Нет | Нет | Нет | Нет | Не требуется |
## Crate `git_info`

### Модуль `git_info`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `GitCommitId` | Нет | Нет | Да | Нет | Да | Сделано |
| `GitCommitIdFallback` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `GitCommitIdRef` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `GitCommitLink` | Нет | Нет | Да | Нет | Да | Сделано |
| `GitCommitLinkCapacity` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `GitCommitLinkOutputRefMut` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `IsProjectCommit` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `ProjectGitCommitLinkRef` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `StdGitCommitIdCow` | Нет | Нет | Да | Нет | Да | Сделано |
| `StdGitCommitLinkCow` | Нет | Нет | Да | Нет | Да | Сделано |
| `ValidateProjectCommitError` | Нет | Нет | Нет | Нет | Нет | Не требуется |
## Crate `location`

### Модуль `location`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `SynItemEnumMutRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
## Crate `location_lib`

### Модуль `location_lib::location`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `ChronoLocationDateTime` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `ChronoLocationDisplayTimezone` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `LocationColumn` | Нет | Да | Нет | Нет | Нет | Не требуется |
| `LocationCommit` | Да | Да | Да | Да | Нет | Сделано |
| `LocationFile` | Да | Да | Да | Да | Нет | Сделано |
| `LocationFileRef` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `LocationLine` | Нет | Да | Нет | Нет | Нет | Не требуется |
| `StdFmtRefMut` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `StdLocationDuration` | Нет | Да | Нет | Нет | Нет | Не требуется |
| `StdTimeDurationNanos` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdTimeDurationSecs` | Да | Нет | Нет | Нет | Нет | Не требуется |
## Crate `location_test`

### Модуль `location_test`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `LocationTestCount` | Да | Да | Нет | Нет | Нет | Не требуется |
| `LocationTestFlag` | Да | Да | Нет | Нет | Нет | Не требуется |
| `LocationTestText` | Нет | Да | Да | Да | Нет | Сделано |
## Crate `macro_clippy_check_common`

### Модуль `macro_clippy_check_common::tests`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `StdTmpDir` | Нет | Нет | Да | Нет | Нет | Не требуется |
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
| `GeneratedRustTokenStream` | Нет | Нет | Да | Нет | Да | Не требуется |
### Модуль `macros_helpers::get_macro_attr`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `AttrPathMatches` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `ProcMacro2MacroAttrMetaListTokenStreamRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SynMacroAttrRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `macros_helpers::json_contract`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `JsonFixtureRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SerdeJsonError` | Нет | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `macros_helpers::location`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `CompileErrorMessage` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `SynVariantRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `macros_helpers::location_syn_field`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `SynLocationField` | Нет | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `macros_helpers::rs_file_path`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `StdRsFilePath` | Нет | Нет | Да | Нет | Нет | Не требуется |
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
| `ExpectedFileContent` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `ExpectedFileContentRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdAssertFilePath` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `StdAssertFilePathRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `TestPathStem` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `TestPathStemRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `macros_helpers::tool_command`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `StdOsString` | Да | Нет | Да | Нет | Да | Не требуется |
| `StdPathRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdProcessCommand` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `StdProcessExitStatus` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `StdProcessOutput` | Нет | Нет | Нет | Нет | Нет | Не требуется |
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
| `ShouldWriteTokenStreamFlag` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `StdRustfmtPath` | Нет | Нет | Нет | Нет | Нет | Не требуется |
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
| `ConvertCaseKind` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `ProcMacro2CaseTokenStream` | Нет | Нет | Да | Нет | Нет | Не требуется |
## Crate `naming_macros`

### Модуль `naming_macros`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `ProcMacro2GeneratedNamingTokenStream` | Нет | Нет | Да | Нет | Нет | Не требуется |
| `ProcMacro2VariantMatchingTokensRef` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `SynEnumIdentifierRef` | Нет | Нет | Нет | Нет | Нет | Не требуется |
## Crate `newtype`

### Модуль `newtype`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `NewtypeBool` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `ProcMacro2GeneratedTokenStream` | Нет | Нет | Да | Нет | Нет | Не требуется |
| `ProcMacroInputTokenStream` | Да | Нет | Да | Нет | Да | Не требуется |
| `SnakeIdentifier` | Да | Нет | Да | Нет | Нет | Сделано |
| `SnakeIdentifierifierLen` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SynAttrsRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SynDeriveInputRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SynExpr` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `SynIdentifier` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `SynIdentifierRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SynType` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `SynTypeRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `newtype::tests::newtype::tests`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `CheckedText` | Да | Нет | Да | Нет | Нет | Сделано |
| `DebugValue` | Нет | Нет | Да | Нет | Нет | Не требуется |
| `DescribedValue` | Да | Нет | Да | Нет | Нет | Сделано |
| `ExplicitErrorCheckedText` | Да | Нет | Да | Нет | Нет | Сделано |
| `InnerValue` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `InnerVecValue` | Да | Нет | Да | Нет | Да | Не требуется |
| `MutableValueRef` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `OwnedSliceValue` | Да | Нет | Да | Нет | Да | Не требуется |
| `OwnedValue` | Да | Нет | Да | Нет | Да | Не требуется |
| `ProcMacro2TokenValue` | Да | Нет | Да | Нет | Да | Не требуется |
| `RedactedDebugValue` | Нет | Нет | Да | Нет | Нет | Не требуется |
| `ReferentValueRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `RichValue` | Да | Да | Да | Да | Нет | Сделано |
| `SliceValueRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdTransparentErrorValue` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `StringValue` | Нет | Нет | Да | Нет | Нет | Не требуется |
| `TargetVecValue` | Нет | Нет | Да | Нет | Нет | Не требуется |
| `TransparentDebugValue` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `UsizeValue` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `ValidatedValue` | Да | Нет | Да | Нет | Нет | Сделано |
| `VecValue` | Нет | Нет | Да | Нет | Нет | Не требуется |
### Модуль `newtype::tests::newtype::tests::to_err_string`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `ToErrStringValue` | Да | Нет | Да | Нет | Да | Не требуется |
## Crate `notification_service`

### Модуль `notification_service`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `AxumNotificationJson` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `AxumNotificationResponse` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `AxumNotificationRouter` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `AxumNotificationState` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `HttpNotificationApiProblem` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `HttpNotificationStatusCode` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `MetricsExporterPrometheusHandle` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `MetricsExporterPrometheusNotificationBuildError` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `NotificationBodyMaximumBytes` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `NotificationConfigError` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `NotificationServeError` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `SqlxNotificationDatabaseError` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `SqlxNotificationMigrationError` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `StdNotificationExitCode` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `StdNotificationIoError` | Нет | Нет | Нет | Нет | Нет | Не требуется |
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
| `SynFieldTyWithStaticLts` | Нет | Нет | Нет | Нет | Нет | Не требуется |
## Crate `panic_location`

### Модуль `panic_location`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `PanicColumn` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `PanicFile` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `PanicLine` | Нет | Нет | Нет | Нет | Нет | Не требуется |
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
| `NonPrimaryKeyPgTypeReadIds` | Нет | Да | Нет | Нет | Нет | Не требуется |
| `NotEmptyUniqueVec` | Нет | Нет | Да | Нет | Нет | Сделано |
| `NotZeroUnsignedPartOfI32` | Нет | Да | Нет | Нет | Нет | Не требуется |
| `NullableJsonObjPgTypeWhereFilter` | Да | Да | Нет | Нет | Нет | Не требуется |
| `OrderSnakeCaseStr` | Да | Нет | Да | Нет | Да | Сделано |
| `OrderUpperCamelCaseStr` | Да | Нет | Да | Нет | Да | Сделано |
| `PaginationStartsWithZero` | Нет | Да | Нет | Нет | Нет | Не требуется |
| `SqlxPostgresQuery` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `UnsignedPartOfI32` | Нет | Да | Нет | Нет | Нет | Не требуется |
| `UnsignedPartOfI32Raw` | Да | Да | Нет | Нет | Нет | Не требуется |
| `UuidUuidTestCases` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `pg_crud_common::advisory_lock`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `PgRelationCapacityMaximum` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `PgRelationLockNamespace` | Да | Нет | Да | Нет | Нет | Сделано |
| `PgRelationResourceId` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `PgRelationResourceIds` | Да | Нет | Да | Нет | Нет | Сделано |
| `PgRelationRowCount` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `SqlxPgRelationLockConnectionRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SqlxPgRelationLockError` | Нет | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `pg_crud_common::batch_validation`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `BatchInvalidItemCount` | Нет | Нет | Нет | Нет | Нет | Не требуется |
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
| `StdBoundedBTreeMap` | Нет | Да | Да | Да | Нет | Сделано |
| `StdBoundedBTreeMapLen` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdBoundedBTreeMapVisitor` | Нет | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `pg_crud_common::bounded_unique_vec`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `BoundedUniqueVec` | Нет | Да | Да | Да | Нет | Сделано |
| `StdBoundedUniqueVecVisitor` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `UniqueVecLen` | Нет | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `pg_crud_common::bounded_vec`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `BoundedVec` | Нет | Да | Да | Да | Нет | Сделано |
| `BoundedVecLen` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdPhantomDataBoundedVecVisitor` | Нет | Нет | Нет | Нет | Нет | Не требуется |
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
| `SignedCursor` | Нет | Нет | Да | Нет | Нет | Сделано |
### Модуль `pg_crud_common::date_sql_filter`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `ChronoUtcDateTimeRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `ChronoUtcDateTimes` | Нет | Нет | Да | Нет | Нет | Не требуется |
| `StdDateSqlBindStart` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `pg_crud_common::db_schema_conformance`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `DbColumnContractSnapshots` | Нет | Нет | Да | Нет | Да | Не требуется |
| `DbColumnHasServerDefault` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `DbColumnNullable` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `DbColumnSnapshots` | Нет | Нет | Да | Нет | Да | Не требуется |
| `DbColumnSpecs` | Нет | Нет | Да | Нет | Да | Не требуется |
| `DbDefaultSpecs` | Нет | Нет | Да | Нет | Да | Не требуется |
| `DbKeyContractSnapshots` | Нет | Нет | Да | Нет | Да | Не требуется |
| `DbKeySpecs` | Нет | Нет | Да | Нет | Да | Не требуется |
| `DbObjectSnapshots` | Нет | Нет | Да | Нет | Да | Не требуется |
| `DbObjectSpecs` | Нет | Нет | Да | Нет | Да | Не требуется |
| `DbSchemaNameRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `DbSchemaText` | Да | Нет | Да | Нет | Нет | Сделано |
| `DbSchemaTextError` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `DbSchemaTexts` | Нет | Нет | Да | Нет | Да | Не требуется |
| `DbStaticSchemaText` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `DbStaticSchemaTexts` | Нет | Нет | Да | Нет | Да | Не требуется |
| `DbTableNameRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SqlxDbSchemaInspectionError` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `SqlxPgPoolRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `pg_crud_common::errors`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `SqlxPostgresQueryBindError` | Да | Нет | Да | Нет | Да | Сделано |
### Модуль `pg_crud_common::filter_bind_plan`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `PgFilterBindValues` | Нет | Нет | Да | Нет | Нет | Не требуется |
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
| `ListTotal` | Нет | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `pg_crud_common::operation_budget`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `OperationBudget` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `OperationCount` | Нет | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `pg_crud_common::operational_invariants`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `PgCounterValue` | Нет | Нет | Нет | Нет | Нет | Не требуется |
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
| `PaginationEnd` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `PaginationLimit` | Нет | Да | Нет | Нет | Нет | Не требуется |
| `PaginationOffset` | Нет | Да | Нет | Нет | Нет | Не требуется |
| `PaginationStart` | Нет | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `pg_crud_common::pg_error`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `SqlxPgErrorRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `pg_crud_common::query_fragment`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `QueryPartFragment` | Нет | Нет | Да | Нет | Да | Сделано |
| `SqlColumnRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `pg_crud_common::read_query_plan`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `ReadQueryPlan` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `SqlSortOrderText` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `StdReadQueryBindIndex` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `pg_crud_common::sql_identifier`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `SqlIdentifier` | Да | Нет | Да | Нет | Нет | Сделано |
| `SqlIdentifiers` | Да | Нет | Да | Нет | Да | Не требуется |
| `SqlQueryText` | Нет | Нет | Да | Нет | Нет | Сделано |
### Модуль `pg_crud_common::sql_like_pattern`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `SqlLikeInputRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SqlLikePattern` | Да | Да | Да | Да | Нет | Сделано |
### Модуль `pg_crud_common::tests_not_empty_unique_vec`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `NonClone` | Нет | Нет | Нет | Нет | Нет | Не требуется |
## Crate `pg_crud_macros_common`

### Модуль `pg_crud_macros_common`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `DeLen` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `DimensionNumber` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `GeneratedRustTokenStreamVec` | Нет | Нет | Да | Нет | Да | Не требуется |
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
| `PgTableIdempotencyKey` | Нет | Нет | Да | Нет | Нет | Сделано |
| `PgTableIdempotencyMethod` | Да | Нет | Да | Нет | Нет | Сделано |
| `PgTableIdempotencyRequestHash` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `PgTableIdempotencyResponseStatus` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `PgTableIdempotencyRoute` | Да | Нет | Да | Нет | Нет | Сделано |
| `PgTableIdempotencyTextBytes` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `PgTableNameRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `PgTableQueryPartFragment` | Да | Нет | Да | Нет | Да | Сделано |
| `PgTableQueryString` | Да | Нет | Да | Нет | Да | Сделано |
| `PgTableRevision` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `PgTableSqlFragmentRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SqlxPgTableIdempotencyError` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `SqlxPgTablePgConnectionRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdPgTableRevisionParseIntError` | Нет | Нет | Нет | Нет | Нет | Не требуется |
## Crate `pg_types_common`

### Модуль `pg_types_common`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `IsPrimaryKey` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `PaginationStartsWithOne` | Нет | Да | Нет | Нет | Нет | Не требуется |
| `PaginationStartsWithOneValue` | Да | Да | Нет | Нет | Нет | Не требуется |
## Crate `prepare_postgresql_databases`

### Модуль `prepare_postgresql_databases`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `DatabaseUrl` | Да | Нет | Да | Нет | Нет | Сделано |
| `MigrationsSource` | Да | Нет | Да | Нет | Нет | Сделано |
| `ProcessArguments` | Нет | Нет | Да | Нет | Нет | Не требуется |
| `ProcessCommands` | Да | Нет | Да | Нет | Да | Не требуется |
| `ProcessProgram` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `ProcessStaticArgument` | Нет | Нет | Нет | Нет | Нет | Не требуется |
## Crate `route_validators`

### Модуль `route_validators`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `AxumHttpStatusCode` | Нет | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `route_validators::check_body_size`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `AxumBody` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `AxumBodySizeError` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `BodySizeLimitBytes` | Да | Да | Нет | Нет | Нет | Не требуется |
| `BytesBodyBytes` | Нет | Нет | Да | Нет | Нет | Сделано |
| `HttpBodySizeHint` | Нет | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `route_validators::check_commit`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `AxumCommitToStrConversionError` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `CommitNotEqMessage` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `CommitToUse` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `EnableApiGitCommitCheck` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `NoCommitHeaderMessage` | Нет | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `route_validators::hdr_val`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `AxumHeaderValueRef` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `AxumHeadersRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `HeaderStrRef` | Нет | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `route_validators::test_hlp`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `AxumTestHeaderValue` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `AxumTestHeaders` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `AxumTestHeadersMutRef` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `TestExpId` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `TestPanicText` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `TestPollCount` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `TestPollLimitReached` | Нет | Нет | Нет | Нет | Нет | Не требуется |
## Crate `server`

### Модуль `server`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `AxumApiRoutes` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `MetricsExporterPrometheusBuildError` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `MetricsExporterPrometheusHandle` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `ServerAdminAuthSvcStateBuildError` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `ServerAdminCleanupCfgError` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `ServerAdminMigrateError` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `ServerConfigError` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `ServerRuntimeBackgroundTaskShutdownError` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `ServerRuntimeContentSecurityPolicyError` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `ServerRuntimeRequestTimeoutError` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `ServerRuntimeRunIntervalError` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `ServerRuntimeServeError` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `SqlxServerPgConnectError` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `StdServerExitCode` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `StdServerIoError` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `StdSharedServerAppState` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `TokioServerRuntime` | Нет | Нет | Нет | Нет | Нет | Не требуется |
## Crate `server_admin`

### Модуль `server_admin`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `AdminAccessTokenError` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `AdminCleanupBatchSize` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `AdminCleanupRetentionSeconds` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `AdminCleanupRows` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `AdminCookieMaxAgeSeconds` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `AdminCookieSecure` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `AdminJwtSecret` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `AdminMigrateError` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `AdminOpaqueToken` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `AdminPassword` | Нет | Да | Нет | Нет | Нет | Сделано |
| `AdminPasswordHash` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `AdminPasswordHashConcurrency` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `AdminPermissions` | Да | Нет | Да | Нет | Да | Сделано |
| `AdminRefreshToken` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `AdminRoleNames` | Да | Нет | Да | Нет | Да | Сделано |
| `AdminSessionId` | Да | Да | Нет | Нет | Нет | Не требуется |
| `AdminTokenHash` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `AdminUnixTokenStream` | Да | Да | Нет | Нет | Нет | Не требуется |
| `Argon2AdminPasswordHashError` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `HttpAdminHeaderMapRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `JsonwebtokenAdminError` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SqlxAdminError` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SqlxAdminMigrateError` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdAdminAccessToken` | Да | Нет | Да | Нет | Нет | Сделано |
| `StdAdminCookie` | Нет | Нет | Да | Нет | Нет | Сделано |
| `StdAdminSharedSemaphore` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `TokioAdminAcquireError` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `TokioAdminJoinError` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `server_admin::auth`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `AdminHtmlSwaggerEnabled` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `AdminPeerAddr` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `AdminSessionPath` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `AdminSignInJson` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `AxumAdminAuthRouter` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `AxumAdminForm` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `AxumAdminJson` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `AxumAdminPath` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `AxumAdminQuery` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `AxumAdminResponse` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `HttpAdminHeaderMap` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `HttpAdminHeaderValueError` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `JsonwebtokenAdminDecodingKey` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `JsonwebtokenAdminDecodingKeys` | Да | Нет | Да | Нет | Да | Не требуется |
| `JsonwebtokenAdminEncodingKey` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `SqlxAdminPgConnectionRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdAdminAccessTtlSeconds` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdAdminFailureDelayMillis` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdAdminFailureThreshold` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdAdminRateLimitCount` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdAdminRateLimitWindowSeconds` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdAdminRefreshTtlSeconds` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdAdminSessionLimit` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdSharedAdminAuthSvcState` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `UtoipaAdminAuthOpenApi` | Нет | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `server_admin::auth::html`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `AdminHtmlFormKey` | Да | Да | Да | Да | Нет | Сделано |
| `AdminHtmlFormText` | Да | Да | Да | Да | Нет | Сделано |
| `StdAdminHtmlSelected` | Да | Да | Да | Да | Нет | Сделано |
### Модуль `server_admin::domain`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `AdminAuditLogId` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `AdminPermissionId` | Нет | Да | Нет | Нет | Нет | Не требуется |
| `AdminPermissionName` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `AdminRoleId` | Нет | Да | Нет | Нет | Нет | Не требуется |
| `AdminUserId` | Нет | Да | Нет | Нет | Нет | Не требуется |
| `SecrecyAdminString` | Нет | Нет | Да | Нет | Да | Сделано |
| `StdAdminBool` | Нет | Да | Нет | Нет | Нет | Не требуется |
| `StdAdminNonZeroUsize` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `StdAdminSocketAddr` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `StdAdminStrRef` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `StdAdminString` | Нет | Нет | Да | Нет | Нет | Сделано |
| `UuidAdminValue` | Нет | Да | Нет | Нет | Нет | Не требуется |
### Модуль `server_admin::generated_tables`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `UtoipaAdminOpenApi` | Нет | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `server_admin::repository`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `AdminPageTotalCount` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `AdminRecentLoginFailureCount` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SqlxAdminRepositoryConnectionMutRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SqlxAdminRepositoryPoolRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `server_admin::repository::roles`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `AdminActiveAdministratorCount` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `server_admin::tests::admin_api`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `AdminHtmlTestBody` | Да | Нет | Да | Нет | Нет | Сделано |
| `AdminHtmlTestFormBody` | Да | Нет | Да | Нет | Нет | Сделано |
| `AxumAdminApiTestRouter` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `HttpAdminApiTestMethod` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `HttpAdminApiTestRequest` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `HttpAdminApiTestResponseRef` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `HttpAdminHtmlTestResponse` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SqlxAdminApiTestPool` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `SqlxAdminHtmlTestTransaction` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `StdAdminApiTestCookie` | Да | Нет | Да | Нет | Нет | Сделано |
| `StdAdminApiTestStrRef` | Нет | Нет | Нет | Нет | Нет | Не требуется |
## Crate `server_admin_contract`

### Модуль `server_admin_contract`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `AdminApiBodyMaxBytes` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `AdminAuditDetailsBytes` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `AdminAuditExportCsv` | Да | Да | Да | Да | Нет | Сделано |
| `AdminAuditLogId` | Да | Да | Нет | Нет | Нет | Не требуется |
| `AdminAuditTimestamp` | Да | Да | Да | Да | Нет | Сделано |
| `AdminAuditViews` | Да | Да | Да | Да | Да | Сделано |
| `AdminBool` | Нет | Да | Нет | Нет | Нет | Не требуется |
| `AdminDataRows` | Да | Да | Да | Да | Да | Сделано |
| `AdminDataTableStrRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `AdminDataTables` | Да | Да | Да | Да | Да | Сделано |
| `AdminDefaultRoute` | Да | Да | Да | Да | Нет | Сделано |
| `AdminDisplayName` | Да | Да | Да | Да | Нет | Сделано |
| `AdminLogin` | Да | Да | Да | Да | Нет | Сделано |
| `AdminMainLogo` | Да | Да | Да | Да | Нет | Сделано |
| `AdminNewPassword` | Да | Да | Да | Да | Нет | Сделано |
| `AdminOptionalSettings` | Да | Да | Да | Да | Да | Сделано |
| `AdminOrganizationContacts` | Да | Да | Да | Да | Нет | Сделано |
| `AdminOrganizationName` | Да | Да | Да | Да | Нет | Сделано |
| `AdminPageLimit` | Нет | Да | Нет | Нет | Нет | Не требуется |
| `AdminPageOffset` | Нет | Да | Нет | Нет | Нет | Не требуется |
| `AdminPagePathRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `AdminPageTotal` | Нет | Да | Нет | Нет | Нет | Не требуется |
| `AdminPassword` | Да | Да | Да | Да | Нет | Сделано |
| `AdminPermissionId` | Да | Да | Нет | Нет | Нет | Не требуется |
| `AdminPermissionIds` | Нет | Да | Да | Да | Да | Сделано |
| `AdminPermissionStrRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `AdminPermissionSummaries` | Да | Да | Да | Да | Да | Сделано |
| `AdminPermissionValue` | Да | Да | Да | Да | Нет | Сделано |
| `AdminPermissionValues` | Да | Да | Да | Да | Да | Сделано |
| `AdminPrimaryColor` | Да | Да | Да | Да | Нет | Сделано |
| `AdminRoleId` | Да | Да | Нет | Нет | Нет | Не требуется |
| `AdminRoleIds` | Нет | Да | Да | Да | Да | Сделано |
| `AdminRoleName` | Да | Да | Да | Да | Нет | Сделано |
| `AdminRoleNames` | Да | Да | Да | Да | Да | Сделано |
| `AdminRoleSummaries` | Да | Да | Да | Да | Да | Сделано |
| `AdminRoutePath` | Нет | Нет | Да | Нет | Нет | Сделано |
| `AdminSessionIdentifier` | Да | Да | Да | Да | Нет | Сделано |
| `AdminSessionTimestamp` | Да | Да | Да | Да | Нет | Сделано |
| `AdminSessionViews` | Да | Да | Да | Да | Да | Сделано |
| `AdminSiteName` | Да | Да | Да | Да | Нет | Сделано |
| `AdminSupportUrl` | Да | Да | Да | Да | Нет | Сделано |
| `AdminTabTitle` | Да | Да | Да | Да | Нет | Сделано |
| `AdminTableSearch` | Нет | Да | Да | Да | Нет | Сделано |
| `AdminTableSortKey` | Нет | Да | Да | Да | Нет | Сделано |
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
| `AxumAdminFrontendRouter` | Нет | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `server_admin_frontend::ssr`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `AdminSsrErrorMessage` | Да | Нет | Да | Нет | Нет | Сделано |
| `AdminSsrHtml` | Нет | Нет | Да | Нет | Нет | Сделано |
| `AdminSsrText` | Нет | Нет | Да | Нет | Нет | Сделано |
## Crate `server_runtime`

### Модуль `server_runtime`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `AxumRouter` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `HttpContentSecurityPolicy` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `RequestTimeoutLayer` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `RequestTimeoutTowerLayer` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `ReqwestClient` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `ReqwestClientBuildError` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `StdRequestTimeoutMessage` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `StdReqwestConnectTimeout` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdReqwestRequestTimeout` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdServeIoError` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `TokioTcpListener` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `server_runtime::batched_cleanup`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `CleanupBatchCount` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `CleanupBatchSize` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `CleanupRows` | Нет | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `server_runtime::bounded_read`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `BoundedBytes` | Нет | Нет | Да | Нет | Нет | Сделано |
| `BoundedJsonText` | Да | Нет | Да | Нет | Нет | Сделано |
| `BoundedReadMaximumBytes` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `BoundedReadObservedBytes` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `BoundedText` | Да | Нет | Да | Нет | Нет | Сделано |
| `ReqwestError` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `ReqwestResponse` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SerdeJsonError` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `StdBoundedReadConcurrency` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `StdBoundedReadConcurrencyMaximum` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdFromUtf8Error` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `StdIoError` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `StdPathRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `server_runtime::child_process`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `ChildDiagnostic` | Нет | Нет | Да | Нет | Нет | Сделано |
| `ChildProcessId` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `ChildProcessReports` | Нет | Нет | Да | Нет | Нет | Сделано |
| `StdChildDiagnosticMaximum` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdChildExitStatus` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `StdChildProcessIoError` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdChildProcessSetMaximum` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdCollectionsChildProcessMap` | Нет | Нет | Да | Нет | Нет | Сделано |
| `TokioChildDiagnosticTask` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `TokioChildProcess` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `TokioChildProcessJoinError` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `TokioManagedChild` | Нет | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `server_runtime::client_ip`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `HttpHeaderMapRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdAddrParseError` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdIpAddr` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdParseIntError` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdRangeContains` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `StdResolvedClientIp` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `StdSocketAddr` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdTrustedProxyPrefixBits` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `TrustedProxyRanges` | Нет | Нет | Да | Нет | Да | Сделано |
### Модуль `server_runtime::cors`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `HttpCorsAllowOriginHeaderValues` | Нет | Нет | Да | Нет | Нет | Сделано |
| `HttpCorsAllowOriginTextRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `server_runtime::csp`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `HttpCspBuilder` | Нет | Нет | Да | Нет | Нет | Сделано |
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
| `StdExclusiveRunAtomicBool` | Нет | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `server_runtime::fallback`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `AcceptsApplicationJson` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `HttpAcceptHeaderMaximumBytes` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `HttpFallbackApiPrefixRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `HttpFallbackMetricsPathRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `HttpFallbackRequestPathRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `HttpMediaRangeRef` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `HttpOptionalAcceptHeaderRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `server_runtime::generation_gate`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `Generation` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `StdGenerationAtomicU64` | Нет | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `server_runtime::geojson`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `GeoJsonDocumentText` | Да | Нет | Да | Нет | Нет | Сделано |
| `SerdeJsonGeoJsonError` | Нет | Нет | Нет | Нет | Нет | Не требуется |
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
| `HealthProbeSucceeded` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `StdHealthProbeTimeout` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdHealthReadinessAtomicBool` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `StdSharedHealthReadiness` | Нет | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `server_runtime::history`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `StdArcSharedRunReports` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `StdAsyncRunHistoryMaximumLen` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdAsyncRunHistoryReportCount` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `StdVecDequeRunReports` | Нет | Нет | Да | Нет | Нет | Сделано |
| `TokioRwLockRunReports` | Нет | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `server_runtime::http_header_policy`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `HttpAttachmentFileNameRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `HttpContentDisposition` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `HttpContentLength` | Да | Нет | Да | Нет | Нет | Сделано |
### Модуль `server_runtime::http_policy`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `HttpAuthorizationHeaderTextRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `HttpBearerTokenRef` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `HttpContentTypeTextRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `HttpCookieHeadersRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `HttpCookieNameRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `HttpCookieValueRef` | Нет | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `server_runtime::http_status_error`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `HttpErrorStatus` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `server_runtime::lease_registry`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `LeaseId` | Да | Нет | Да | Нет | Нет | Сделано |
| `LeaseIds` | Нет | Нет | Да | Нет | Нет | Сделано |
| `LeaseKey` | Да | Нет | Да | Нет | Нет | Сделано |
| `LeaseTextRef` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `StdArcTokioLeaseRegistryRwLock` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `StdLeaseRegistryMaximum` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdLeaseStaleTimeout` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `TokioLeaseInstant` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `TokioLeaseRegistryRwLock` | Нет | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `server_runtime::lifecycle`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `StdRequestTimeout` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdRunInterval` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `TokioAbortTask` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `TokioBackgroundTaskJoinHandle` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `TokioBackgroundTaskShutdownSender` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `TokioTaskJoinError` | Нет | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `server_runtime::limits`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `RetryAfterSecs` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdArcTokioSemaphore` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `StdPermitWaitTimeout` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdSemaphorePermitCount` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `TokioAcquireError` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `TokioOwnedSemaphorePermit` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `server_runtime::metrics_layer`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `HttpMetricsPathCacheMaximum` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `HttpMetricsPathText` | Да | Нет | Да | Нет | Нет | Сделано |
| `HttpMetricsPathTextRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `MetricsResponseBody` | Да | Нет | Да | Нет | Нет | Сделано |
| `MetricsSharedString` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `StdHttpMetricsPathEntries` | Нет | Нет | Да | Нет | Нет | Сделано |
| `StdSharedHttpMetricsPathCache` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `server_runtime::multipart`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `FileStagingDirectoryName` | Да | Нет | Да | Нет | Нет | Сделано |
| `MultipartBytes` | Да | Нет | Да | Нет | Нет | Сделано |
| `MultipartBytesParts` | Нет | Нет | Да | Нет | Нет | Сделано |
| `MultipartFieldName` | Да | Нет | Да | Нет | Нет | Сделано |
| `MultipartFileName` | Да | Нет | Да | Нет | Нет | Сделано |
| `MultipartPayloadMaximum` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `MultipartTextParts` | Нет | Нет | Да | Нет | Нет | Сделано |
| `MultipartTextValue` | Да | Нет | Да | Нет | Нет | Сделано |
| `MultipartValueLength` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `StdStorageRelativePath` | Нет | Нет | Да | Нет | Нет | Сделано |
| `StoragePathSegment` | Да | Нет | Да | Нет | Нет | Сделано |
### Модуль `server_runtime::notification`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `AxumNotificationJson` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `AxumNotificationRouter` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `HttpNotificationHeaderMap` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `NotificationApiToken` | Да | Нет | Да | Нет | Нет | Сделано |
| `NotificationApiTokenAuthorized` | Нет | Нет | Нет | Нет | Нет | Не требуется |
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
| `RequestOriginAllowed` | Нет | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `server_runtime::outbound_url`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `OutboundAllowedHost` | Да | Нет | Да | Нет | Нет | Сделано |
| `OutboundHostAllowlist` | Да | Нет | Да | Нет | Нет | Сделано |
| `OutboundUrlTextRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `ReqwestOutboundUrl` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `StdOutboundIpAddr` | Нет | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `server_runtime::path_policy`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `HttpAllowedPathPrefixRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `HttpNormalizedPath` | Да | Нет | Да | Нет | Нет | Сделано |
| `HttpProxyPath` | Да | Нет | Да | Нет | Нет | Сделано |
| `HttpProxyPathPrefixMatch` | Нет | Нет | Нет | Нет | Нет | Не требуется |
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
| `SqlxPgRateLimitError` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `SqlxPgRateLimitPoolRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `server_runtime::redacted_url`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `RedactedUrl` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `RedactedUrlTextRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `server_runtime::request_id`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `HttpHeaderToStrError` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `RequestId` | Да | Нет | Да | Нет | Нет | Сделано |
### Модуль `server_runtime::resource_budget`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `ResourceBudgetAmount` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `ResourceBudgetMaximum` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdAtomicUsize` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `StdSharedAtomicUsize` | Нет | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `server_runtime::resource_utilization`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `ResourceAmount` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `ResourceUtilizationPercent` | Нет | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `server_runtime::retry`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `StdRetryAttempts` | Нет | Нет | Нет | Нет | Нет | Не требуется |
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
| `HttpSetCookieHeaderValue` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `StdCookieMaxAgeSeconds` | Да | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `server_runtime::service_bootstrap`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `StdServiceRuntimeIoError` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `TokioServiceRuntime` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `TracingSubscriberInitError` | Нет | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `server_runtime::single_flight`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `SingleFlightKey` | Да | Нет | Да | Нет | Нет | Сделано |
| `SingleFlightWaiter` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `StdArcStdSingleFlightRwLock` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `StdSingleFlightMaximum` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdSingleFlightRwLock` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `StdSingleFlightWriteGuard` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `TokioSingleFlightReceiver` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `TokioSingleFlightSender` | Нет | Нет | Нет | Нет | Нет | Не требуется |
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
| `SynIdent` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `SynLitStr` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `SynVisibility` | Нет | Нет | Нет | Нет | Нет | Не требуется |
## Crate `synchronization_service_runtime`

### Модуль `synchronization_service_runtime`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `SynchronizationPayload` | Да | Нет | Да | Нет | Да | Сделано |
## Crate `tests`

### Модуль `tests::code_style::types`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `AnalyzerBool` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `AnalyzerChar` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `AnalyzerCount` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `CargoMetadata` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `CargoMetadataRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `CargoTomlFileIdx` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `DiagnosticMsgs` | Нет | Нет | Да | Нет | Нет | Не требуется |
| `DiagnosticMsgsMutRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SourceText` | Да | Нет | Да | Нет | Нет | Не требуется |
| `SourceTextList` | Нет | Нет | Да | Нет | Да | Не требуется |
| `SourceTextListRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SourceTextRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StaticStr` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `StaticStrSliceRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdCargoPackageIdRefSet` | Да | Нет | Да | Нет | Да | Не требуется |
| `StdPathBuf` | Да | Нет | Да | Нет | Да | Не требуется |
| `StdPathRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdProcessOutputRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdSourceTextHashSet` | Да | Нет | Да | Нет | Да | Не требуется |
| `StdSourceTextRefSet` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `StdSourceTextSet` | Нет | Нет | Да | Нет | Да | Не требуется |
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
| `DomainEvents` | Нет | Нет | Да | Нет | Нет | Не требуется |
| `DomainId` | Нет | Нет | Нет | Нет | Нет | Не требуется |
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
| `StaticStrToOwnedInput` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `ToErrStringValue` | Да | Нет | Да | Нет | Нет | Сделано |
## Crate `token_patterns`

### Модуль `token_patterns`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `ProcMacro2TokensMut` | Нет | Нет | Нет | Нет | Нет | Не требуется |
## Crate `token_patterns_macros`

### Модуль `token_patterns_macros`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `ProcMacro2GenerateTpInput` | Нет | Нет | Да | Нет | Нет | Не требуется |
| `ProcMacro2GenerateTpOutput` | Нет | Нет | Да | Нет | Нет | Не требуется |
## Crate `where_filters`

### Модуль `where_filters`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `BoundedVec` | Нет | Да | Да | Да | Нет | Сделано |
| `BoundedVecLen` | Да | Да | Нет | Нет | Нет | Не требуется |
| `PgTypeNotEmptyUniqueVec` | Нет | Нет | Да | Нет | Нет | Сделано |
| `RegexCasePostgreqlSyntax` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `RegexRegex` | Нет | Да | Нет | Нет | Нет | Не требуется |
### Модуль `where_filters::tests`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `NonClone` | Нет | Нет | Нет | Нет | Нет | Не требуется |
## Crate `workspace_macro_helpers`

### Модуль `workspace_macro_helpers`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `FirstCommaStripped` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `FirstIdentifier` | Да | Нет | Да | Нет | Да | Сделано |
| `FirstIdentifierifierTryFromStringError` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `PartIndex` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `ProcMacro2MacroTokens` | Нет | Нет | Да | Нет | Да | Не требуется |
| `ProcMacro2TopLevelCommaParts` | Нет | Нет | Да | Нет | Нет | Не требуется |
| `StdUniqueOptionSet` | Нет | Нет | Да | Нет | Нет | Не требуется |
| `StdUniqueOptionSetContains` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `StdUniqueOptionSetIsEmpty` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `SynDeriveInputRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `SynFieldsNamedRef` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `SynFieldsUnnamedRef` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `TopLevelCommaPart` | Нет | Нет | Нет | Нет | Нет | Не требуется |
## Crate `workspace_scaffold`

### Модуль `workspace_scaffold`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `ProjectNameRef` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `RepositoryUrlRef` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `ServicePort` | Нет | Нет | Нет | Нет | Нет | Не требуется |
## Crate `workspace_test_runner`

### Модуль `workspace_test_runner`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `AnsiTextRef` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `CargoArgs` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `CleanAnsiText` | Нет | Нет | Да | Нет | Нет | Сделано |
| `MeasurementName` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `MemusageColumnIdx` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `MemusageKey` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `MemusageProgNameRef` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `MemusageRowName` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `MemusageValueRef` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `ProgramArgsRef` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `ProgramPathRef` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `QuoteTokenStreamGeneratePgTableMeasureInputTokenStream` | Нет | Нет | Да | Нет | Нет | Не требуется |
| `StderrTextRef` | Да | Нет | Нет | Нет | Нет | Не требуется |
| `ToolName` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `ToolPath` | Нет | Нет | Нет | Нет | Нет | Не требуется |
### Модуль `workspace_test_runner::execution`

| Тип | I | D | B | DT | FT | Статус |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `CommandIdx` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `CommandStartedAt` | Нет | Нет | Нет | Нет | Нет | Не требуется |
| `RunDir` | Нет | Нет | Да | Нет | Нет | Не требуется |
| `SummaryText` | Нет | Нет | Да | Нет | Нет | Не требуется |
