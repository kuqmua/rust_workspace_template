# Структуры-обёртки над типами

Этот документ содержит инвентаризацию одно-полевых tuple-структур (`struct Name(Type);`), объявленных в Rust-исходниках workspace. Именно такая форма считается здесь структурой-обёрткой над типом.

Включены production-, test-, bench- и example-модули всех workspace crates. Исключены `target/` и структуры, которые присутствуют только как токены внутри генераторов (`quote!`), поскольку они не являются объявленными items исходного crate.

Обозначения столбцов:

- **I** — только `From`/`TryFrom`/десериализация: найден разрешённый путь инициализации, поле закрыто и альтернативный фабричный путь не обнаружен.
- **D** — десериализация нужна: текущая реализация явно предусматривает `Deserialize`, ручной impl или serde-режим вспомогательного макроса.
- **B** — нужен ограничивающий `TryFrom`: непосредственно оборачиваемый сырой owned-тип может неограниченно наращивать содержимое. Ограничение ставится на границе входа сырого типа, а не повторяется поверх готового доменного типа.
- **DT** — нужен `TryFrom` в десериализации: одновременно выполняются **D** и **B**; десериализация должна получить сырой тип и вызвать проверяющий `TryFrom`.
- **FT** — поменять `From` на `TryFrom`: сейчас объявлен `From`/`FromInner`, но выполняется **B**, поэтому инициализация должна стать fallible и проверять верхнюю границу размера.

Borrowed-ссылки и массивы фиксированной длины не считаются неограниченно растущими. **Нет** означает, что соответствующее условие не доказано или не требуется.

Всего структур-обёрток: **930**. **I:** 483 Да / 447 Нет. **D:** 96 Да / 834 Нет. **B:** 270 Да / 660 Нет. **DT:** 57 Да / 873 Нет. **FT:** 91 Да / 839 Нет.

## Crate `app_state`

### Модуль `app_state`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `SqlxPgPool` | Да | Нет | Нет | Нет | Нет |
| `SqlxPgPoolRef` | Да | Нет | Нет | Нет | Нет |
## Crate `common_routes`

### Модуль `common_routes`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `AxumCommonRoutes` | Нет | Нет | Нет | Нет | Нет |
| `AxumHealthCheckStatus` | Нет | Нет | Нет | Нет | Нет |
| `AxumHttpUriRef` | Нет | Нет | Нет | Нет | Нет |
| `AxumJsonPayload` | Нет | Нет | Нет | Нет | Нет |
| `HealthCheckSucceeded` | Нет | Нет | Нет | Нет | Нет |
| `HealthComponents` | Да | Нет | Да | Нет | Да |
| `HealthDatabaseAvailable` | Да | Нет | Нет | Нет | Нет |
| `NoRouteMessageCapacity` | Нет | Нет | Нет | Нет | Нет |
| `NotFoundMessage` | Да | Нет | Да | Нет | Нет |
| `OpenApiSpecificationPath` | Нет | Нет | Нет | Нет | Нет |
| `StdArcCommonRoutesAppState` | Нет | Нет | Нет | Нет | Нет |
| `UriSuffixRef` | Нет | Нет | Нет | Нет | Нет |
| `UtoipaCommonRoutesOpenApiDocument` | Нет | Нет | Нет | Нет | Нет |
## Crate `config_lib`

### Модуль `config_lib`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `AdminAccessTokenTtlSeconds` | Нет | Нет | Нет | Нет | Нет |
| `AdminBoolParsingError` | Нет | Нет | Нет | Нет | Нет |
| `AdminCookieSecure` | Нет | Нет | Нет | Нет | Нет |
| `AdminJwtSecret` | Нет | Нет | Да | Нет | Нет |
| `AdminPasswordHashConcurrency` | Нет | Нет | Нет | Нет | Нет |
| `AdminPositiveU64ParsingError` | Нет | Нет | Нет | Нет | Нет |
| `AdminPositiveUsizeParsingError` | Нет | Нет | Нет | Нет | Нет |
| `AdminRefreshTokenTtlSeconds` | Нет | Нет | Нет | Нет | Нет |
| `AdminSessionLimit` | Нет | Нет | Нет | Нет | Нет |
| `AdminSignInRateLimit` | Нет | Нет | Нет | Нет | Нет |
| `AdminSwaggerEnabled` | Нет | Нет | Нет | Нет | Нет |
| `AdminTokenAudience` | Да | Да | Да | Да | Нет |
| `AdminTokenIssuer` | Да | Да | Да | Да | Нет |
| `ChronoEastFixedOffset` | Нет | Нет | Нет | Нет | Нет |
| `ChronoFixedOffsetError` | Нет | Нет | Нет | Нет | Нет |
| `ChronoTimezone` | Нет | Нет | Нет | Нет | Нет |
| `ConfigRustTypeName` | Да | Нет | Нет | Нет | Нет |
| `ContentSecurityPolicy` | Да | Нет | Да | Нет | Нет |
| `EnvVarName` | Да | Нет | Да | Нет | Да |
| `EnvVarNameRef` | Нет | Нет | Нет | Нет | Нет |
| `HttpGzipEnabled` | Нет | Нет | Нет | Нет | Нет |
| `MaximumSizeOfHttpBodyInBytes` | Да | Нет | Нет | Нет | Нет |
| `PgPoolAcquireTimeoutSeconds` | Нет | Нет | Нет | Нет | Нет |
| `PgPoolIdleTimeoutSeconds` | Нет | Нет | Нет | Нет | Нет |
| `PgPoolMaxConnections` | Да | Нет | Нет | Нет | Нет |
| `PgPoolMaxLifetimeSeconds` | Нет | Нет | Нет | Нет | Нет |
| `PgPoolMinConnections` | Нет | Нет | Нет | Нет | Нет |
| `RequestTimeoutSeconds` | Нет | Нет | Нет | Нет | Нет |
| `SecrecySecretBoxString` | Да | Нет | Да | Нет | Да |
| `StdEnvVarOk` | Нет | Нет | Да | Нет | Да |
| `StdEnvVarOkRef` | Нет | Нет | Нет | Нет | Нет |
| `StdI32ParsingError` | Нет | Нет | Нет | Нет | Нет |
| `StdNonZeroU64` | Да | Нет | Нет | Нет | Нет |
| `StdNonZeroUsize` | Да | Нет | Нет | Нет | Нет |
| `StdParseBoolError` | Да | Нет | Нет | Нет | Нет |
| `StdParseIntError` | Да | Нет | Нет | Нет | Нет |
| `StdU32ParsingError` | Нет | Нет | Нет | Нет | Нет |
| `StdUsizeParsingError` | Нет | Нет | Нет | Нет | Нет |
| `TimezoneSeconds` | Нет | Нет | Нет | Нет | Нет |
### Модуль `config_lib::types`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `EnvParseError` | Да | Нет | Да | Нет | Да |
| `EnvVarNameRef` | Нет | Нет | Нет | Нет | Нет |
| `EnvVarValueRef` | Нет | Нет | Нет | Нет | Нет |
| `ParseCtxRef` | Нет | Нет | Нет | Нет | Нет |
| `StdEnvVarResult` | Нет | Нет | Да | Нет | Нет |
| `TracingLevelName` | Нет | Нет | Нет | Нет | Нет |
## Crate `config_lib_macros`

### Модуль `config_lib_macros`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `ProcMacro2TryFromParseFixedErrorTy` | Нет | Нет | Да | Нет | Нет |
| `ProcMacro2TryFromParseInput` | Нет | Нет | Да | Нет | Нет |
| `ProcMacroTryFromParseTokenStream` | Нет | Нет | Да | Нет | Нет |
## Crate `development_data_bootstrap`

### Модуль `development_data_bootstrap`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `DevelopmentIdentityCount` | Нет | Нет | Нет | Нет | Нет |
| `DevelopmentIdentitySpecs` | Да | Нет | Да | Нет | Да |
## Crate `external_service_emulators`

### Модуль `external_service_emulators`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `RemoteSyncRequestCount` | Нет | Нет | Нет | Нет | Нет |
| `TokioMockNotificationReceiver` | Нет | Нет | Нет | Нет | Нет |
| `TokioMockNotificationSender` | Нет | Нет | Нет | Нет | Нет |
## Crate `file_storage`

### Модуль `file_storage`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `DiskCacheEvictionPlan` | Нет | Нет | Да | Нет | Нет |
| `StdDiskCacheModifiedAt` | Да | Нет | Нет | Нет | Нет |
| `StdDiskCacheSize` | Да | Нет | Нет | Нет | Нет |
| `StdFileBytes` | Да | Нет | Да | Нет | Нет |
| `StdFileStorageIoError` | Да | Нет | Нет | Нет | Нет |
| `StdFileStorageRoot` | Да | Нет | Да | Нет | Нет |
| `StdStaleBefore` | Да | Нет | Нет | Нет | Нет |
| `StdStaleStagingEntryCount` | Нет | Нет | Нет | Нет | Нет |
| `StdStaleStagingEntryLimit` | Да | Нет | Нет | Нет | Нет |
| `StdStorageOperationId` | Да | Нет | Да | Нет | Нет |
| `StdStoragePathRef` | Да | Нет | Нет | Нет | Нет |
| `StdStorageRelativePath` | Да | Нет | Да | Нет | Нет |
| `StorageDirectoryNameRef` | Нет | Нет | Нет | Нет | Нет |
## Crate `frontend_contract`

### Модуль `frontend_contract`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `ActionContracts` | Да | Нет | Да | Нет | Да |
| `ContractI64` | Нет | Нет | Нет | Нет | Нет |
| `ContractStr` | Да | Нет | Нет | Нет | Нет |
| `FieldContracts` | Да | Нет | Да | Нет | Да |
| `FieldOrder` | Нет | Нет | Нет | Нет | Нет |
| `FormValue` | Нет | Нет | Да | Нет | Нет |
| `FormValueError` | Нет | Нет | Да | Нет | Нет |
| `FormValueRef` | Да | Нет | Нет | Нет | Нет |
| `RouteContracts` | Да | Нет | Да | Нет | Да |
| `TransportBody` | Да | Нет | Да | Нет | Да |
| `TransportError` | Нет | Нет | Да | Нет | Нет |
| `TransportIdempotencyKey` | Да | Нет | Да | Нет | Нет |
| `TransportIfMatch` | Да | Нет | Да | Нет | Нет |
| `TransportPath` | Нет | Нет | Да | Нет | Нет |
| `TransportRetryAfter` | Да | Нет | Да | Нет | Нет |
| `TransportStatus` | Нет | Нет | Нет | Нет | Нет |
### Модуль `frontend_contract::auth_session_keep_alive`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `StdAuthSessionInstant` | Да | Нет | Нет | Нет | Нет |
| `StdAuthSessionRefreshInterval` | Да | Нет | Нет | Нет | Нет |
### Модуль `frontend_contract::json_snapshot`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `JsonContractSnapshot` | Да | Нет | Да | Нет | Нет |
| `JsonSnapshotDynamicFieldRef` | Да | Нет | Нет | Нет | Нет |
### Модуль `frontend_contract::openapi_validation`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `OpenApiContractText` | Да | Нет | Да | Нет | Нет |
| `OpenApiContractTextError` | Нет | Нет | Нет | Нет | Нет |
| `OpenApiResponseStatus` | Да | Нет | Нет | Нет | Нет |
| `RuntimeRoutesRef` | Да | Нет | Нет | Нет | Нет |
| `SerdeJsonOpenApiSerializationError` | Нет | Нет | Нет | Нет | Нет |
### Модуль `frontend_contract::problem`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `ApiProblemDetail` | Нет | Да | Да | Да | Нет |
| `ApiProblemField` | Нет | Да | Да | Да | Нет |
| `ApiProblemRequestId` | Нет | Да | Да | Да | Нет |
| `ApiProblemStatus` | Да | Да | Нет | Нет | Нет |
| `ApiProblemViolations` | Нет | Да | Да | Да | Да |
### Модуль `frontend_contract::route`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `OpenApiSecuritySchemeRef` | Да | Нет | Нет | Нет | Нет |
| `ParameterizedRoutePath` | Нет | Нет | Да | Нет | Нет |
| `RouteBodyLimit` | Да | Нет | Нет | Нет | Нет |
| `RouteCoverageDescriptors` | Нет | Нет | Да | Нет | Да |
| `RouteMetadataList` | Нет | Нет | Да | Нет | Да |
| `RouteSchemaContracts` | Нет | Нет | Да | Нет | Да |
| `UtoipaOpenApiPathParameter` | Да | Нет | Нет | Нет | Нет |
| `UtoipaOpenApiRouteSchema` | Да | Нет | Нет | Нет | Нет |
### Модуль `frontend_contract::route::tests`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `Request` | Нет | Да | Нет | Нет | Нет |
| `Response` | Нет | Да | Нет | Нет | Нет |
### Модуль `frontend_contract::route_contract_validation`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `HttpContractBody` | Да | Нет | Да | Нет | Да |
| `HttpContractStatus` | Да | Нет | Нет | Нет | Нет |
| `RouteContractMismatches` | Нет | Нет | Да | Нет | Нет |
### Модуль `frontend_contract::route_coverage`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `RouteTestCategories` | Нет | Нет | Да | Нет | Да |
### Модуль `frontend_contract::url_builder`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `ApiUrl` | Да | Нет | Да | Нет | Нет |
| `ApiUrlPathSegmentRef` | Да | Нет | Нет | Нет | Нет |
| `ApiUrlQueryComponentRef` | Да | Нет | Нет | Нет | Нет |
## Crate `frontend_contract_macros`

### Модуль `frontend_contract_macros`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `StdBool` | Нет | Нет | Нет | Нет | Нет |
| `SynExpr` | Да | Нет | Нет | Нет | Нет |
| `SynIdent` | Да | Нет | Нет | Нет | Нет |
| `SynRouteRegistryBindings` | Нет | Нет | Нет | Нет | Нет |
| `SynRouteRegistryHandler` | Нет | Нет | Нет | Нет | Нет |
| `SynRouteRegistryRoute` | Нет | Нет | Нет | Нет | Нет |
| `SynRouteRegistryState` | Нет | Нет | Нет | Нет | Нет |
| `SynType` | Да | Нет | Нет | Нет | Нет |
## Crate `generate_derive_token_stream_builder`

### Модуль `generate_derive_token_stream_builder`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `SnakeCaseString` | Да | Нет | Да | Нет | Нет |
| `ToSnakeCaseInput` | Нет | Нет | Нет | Нет | Нет |
## Crate `generate_pg_table_src`

### Модуль `generate_pg_table_src::model`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `GeneratePgTableFieldCount` | Нет | Нет | Нет | Нет | Нет |
| `SynGeneratePgTableModelError` | Да | Нет | Нет | Нет | Нет |
| `SynGeneratePgTableModelInput` | Да | Нет | Нет | Нет | Нет |
### Модуль `generate_pg_table_src::pipeline`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `SynBuiltGeneratePgTableInput` | Нет | Нет | Нет | Нет | Нет |
| `SynGeneratePgTablePipelineError` | Нет | Нет | Нет | Нет | Нет |
| `SynParsedGeneratePgTableInput` | Нет | Нет | Нет | Нет | Нет |
| `SynValidatedGeneratePgTableInput` | Нет | Нет | Нет | Нет | Нет |
### Модуль `generate_pg_table_src::source`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `CompileErrorMessage` | Нет | Нет | Нет | Нет | Нет |
| `TableTestNames` | Да | Нет | Да | Нет | Да |
## Crate `generate_pg_types_src`

### Модуль `generate_pg_types_src::source`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `GeneratePgTypeRecords` | Да | Да | Да | Да | Нет |
| `GeneratePgTypes` | Да | Да | Да | Да | Нет |
| `GenerateSecretText` | Нет | Да | Нет | Нет | Нет |
| `ParsedGeneratePgTypesConfig` | Нет | Нет | Нет | Нет | Нет |
| `PgSqlName` | Нет | Нет | Нет | Нет | Нет |
| `PgTypesModelEntryCount` | Нет | Нет | Нет | Нет | Нет |
| `SerdeJsonGeneratePgTypesError` | Нет | Нет | Нет | Нет | Нет |
## Crate `generate_quotes`

### Модуль `generate_quotes`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `ProcMacro2QuotedLiteralTokenStream` | Да | Нет | Да | Нет | Да |
| `QuoteChar` | Нет | Нет | Нет | Нет | Нет |
| `QuotePanicId` | Нет | Нет | Нет | Нет | Нет |
| `QuotePrefix` | Нет | Нет | Нет | Нет | Нет |
| `QuotedLiteral` | Да | Нет | Да | Нет | Нет |
## Crate `generate_where_filters_src`

### Модуль `generate_where_filters_src::bind`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `FilterPlaceholderCount` | Нет | Нет | Нет | Нет | Нет |
### Модуль `generate_where_filters_src::model`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `BindCount` | Нет | Нет | Нет | Нет | Нет |
| `FilterSpecValid` | Нет | Нет | Нет | Нет | Нет |
| `FilterSqlOperator` | Нет | Нет | Нет | Нет | Нет |
| `FilterSqlSuffix` | Нет | Нет | Нет | Нет | Нет |
### Модуль `generate_where_filters_src::source`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `ProcMacro2GenerateWhereFiltersInput` | Да | Нет | Нет | Нет | Нет |
| `ProcMacro2GenerateWhereFiltersTokenStream` | Да | Нет | Да | Нет | Да |
| `SerdeJsonGenerateWhereFiltersError` | Нет | Нет | Нет | Нет | Нет |
| `ValidatedGenerateWhereFiltersConfig` | Нет | Нет | Нет | Нет | Нет |
## Crate `git_info`

### Модуль `git_info`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `GitCommitId` | Нет | Нет | Да | Нет | Да |
| `GitCommitIdFallback` | Нет | Нет | Нет | Нет | Нет |
| `GitCommitIdRef` | Нет | Нет | Нет | Нет | Нет |
| `GitCommitLink` | Нет | Нет | Да | Нет | Да |
| `GitCommitLinkCapacity` | Нет | Нет | Нет | Нет | Нет |
| `GitCommitLinkOutputRefMut` | Нет | Нет | Нет | Нет | Нет |
| `IsProjectCommit` | Нет | Нет | Нет | Нет | Нет |
| `ProjectGitCommitLinkRef` | Нет | Нет | Нет | Нет | Нет |
| `StdGitCommitIdCow` | Нет | Нет | Да | Нет | Да |
| `StdGitCommitLinkCow` | Нет | Нет | Да | Нет | Да |
| `ValidateProjectCommitError` | Нет | Нет | Нет | Нет | Нет |
## Crate `location`

### Модуль `location`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `SynItemEnumMutRef` | Да | Нет | Нет | Нет | Нет |
## Crate `location_lib`

### Модуль `location_lib::location`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `ChronoLocationDateTime` | Нет | Нет | Нет | Нет | Нет |
| `ChronoLocationDisplayTimezone` | Нет | Нет | Нет | Нет | Нет |
| `LocationColumn` | Нет | Да | Нет | Нет | Нет |
| `LocationCommit` | Да | Да | Да | Да | Нет |
| `LocationFile` | Да | Да | Да | Да | Нет |
| `LocationFileRef` | Нет | Нет | Нет | Нет | Нет |
| `LocationLine` | Нет | Да | Нет | Нет | Нет |
| `StdFmtRefMut` | Нет | Нет | Нет | Нет | Нет |
| `StdLocationDuration` | Нет | Да | Нет | Нет | Нет |
| `StdTimeDurationNanos` | Да | Нет | Нет | Нет | Нет |
| `StdTimeDurationSecs` | Да | Нет | Нет | Нет | Нет |
## Crate `location_test`

### Модуль `location_test`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `LocationTestCount` | Да | Да | Нет | Нет | Нет |
| `LocationTestFlag` | Да | Да | Нет | Нет | Нет |
| `LocationTestText` | Нет | Да | Да | Да | Нет |
## Crate `macro_clippy_check_common`

### Модуль `macro_clippy_check_common::tests`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `StdTmpDir` | Нет | Нет | Да | Нет | Нет |
## Crate `macros_helpers`

### Модуль `macros_helpers::attr_identifier_str`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `AttrIdentifierName` | Да | Нет | Нет | Нет | Нет |
### Модуль `macros_helpers::generate_field_location_new_token_stream`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `FieldLocationColumn` | Да | Нет | Нет | Нет | Нет |
| `FieldLocationFile` | Да | Нет | Нет | Нет | Нет |
| `FieldLocationLine` | Да | Нет | Нет | Нет | Нет |
### Модуль `macros_helpers::generate_if_write_is_err_token_stream`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `ProcMacro2IfWriteIsErrTokenStream` | Да | Нет | Да | Нет | Да |
### Модуль `macros_helpers::generate_simple_syn_punct`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `SynPathSegment` | Да | Нет | Нет | Нет | Нет |
| `SynPathSegments` | Да | Нет | Нет | Нет | Нет |
### Модуль `macros_helpers::generated_rust_token_stream`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `GeneratedRustTokenStream` | Нет | Нет | Да | Нет | Да |
### Модуль `macros_helpers::get_macro_attr`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `AttrPathMatches` | Нет | Нет | Нет | Нет | Нет |
| `ProcMacro2MacroAttrMetaListTokenStreamRef` | Да | Нет | Нет | Нет | Нет |
| `SynMacroAttrRef` | Да | Нет | Нет | Нет | Нет |
### Модуль `macros_helpers::json_contract`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `JsonFixtureRef` | Да | Нет | Нет | Нет | Нет |
| `SerdeJsonError` | Нет | Нет | Нет | Нет | Нет |
### Модуль `macros_helpers::location`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `CompileErrorMessage` | Нет | Нет | Нет | Нет | Нет |
| `SynVariantRef` | Да | Нет | Нет | Нет | Нет |
### Модуль `macros_helpers::location_syn_field`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `SynLocationField` | Нет | Нет | Нет | Нет | Нет |
### Модуль `macros_helpers::rs_file_path`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `StdRsFilePath` | Нет | Нет | Да | Нет | Нет |
### Модуль `macros_helpers::status_code`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `SynStatusCodeVariantRef` | Да | Нет | Нет | Нет | Нет |
### Модуль `macros_helpers::syn_field`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `SynFieldIdentifier` | Да | Нет | Нет | Нет | Нет |
| `SynFieldType` | Да | Нет | Нет | Нет | Нет |
| `SynFieldVis` | Да | Нет | Нет | Нет | Нет |
### Модуль `macros_helpers::test_database`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `SanitizedDatabaseTarget` | Да | Нет | Да | Нет | Нет |
| `UrlRef` | Да | Нет | Нет | Нет | Нет |
### Модуль `macros_helpers::test_hlp`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `ExpectedFileContent` | Нет | Нет | Нет | Нет | Нет |
| `ExpectedFileContentRef` | Да | Нет | Нет | Нет | Нет |
| `StdAssertFilePath` | Нет | Нет | Нет | Нет | Нет |
| `StdAssertFilePathRef` | Да | Нет | Нет | Нет | Нет |
| `TestPathStem` | Нет | Нет | Нет | Нет | Нет |
| `TestPathStemRef` | Да | Нет | Нет | Нет | Нет |
### Модуль `macros_helpers::tool_command`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `StdOsString` | Да | Нет | Да | Нет | Да |
| `StdPathRef` | Да | Нет | Нет | Нет | Нет |
| `StdProcessCommand` | Нет | Нет | Нет | Нет | Нет |
| `StdProcessExitStatus` | Нет | Нет | Нет | Нет | Нет |
| `StdProcessOutput` | Нет | Нет | Нет | Нет | Нет |
| `ToolArgRef` | Да | Нет | Нет | Нет | Нет |
| `ToolArgsRef` | Да | Нет | Нет | Нет | Нет |
| `ToolEnvKeyRef` | Да | Нет | Нет | Нет | Нет |
| `ToolEnvValueRef` | Да | Нет | Нет | Нет | Нет |
| `ToolProgramRef` | Да | Нет | Нет | Нет | Нет |
### Модуль `macros_helpers::wrap_derive`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `ProcMacro2DeriveTokensRef` | Да | Нет | Нет | Нет | Нет |
### Модуль `macros_helpers::write_string_into_file`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `ShouldWriteString` | Да | Нет | Нет | Нет | Нет |
| `StdWrittenFilePath` | Да | Нет | Да | Нет | Да |
| `StdWrittenFilePathRef` | Да | Нет | Нет | Нет | Нет |
| `StringFileContentRef` | Да | Нет | Нет | Нет | Нет |
### Модуль `macros_helpers::write_token_stream_into_file`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `ProcMacro2TokenStreamRef` | Да | Нет | Нет | Нет | Нет |
| `ShouldWriteTokenStreamFlag` | Нет | Нет | Нет | Нет | Нет |
| `StdRustfmtPath` | Нет | Нет | Нет | Нет | Нет |
## Crate `naming`

### Модуль `naming`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `SwaggerUrlPathPrefix` | Да | Нет | Нет | Нет | Нет |
| `SwaggerUrlPathSelfQuotesStrValue` | Да | Нет | Нет | Нет | Нет |
| `SwaggerUrlPathSelfQuotesTokenStreamValue` | Да | Нет | Нет | Нет | Нет |
## Crate `naming_common`

### Модуль `naming_common`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `CaseString` | Да | Нет | Да | Нет | Нет |
| `ConvertCaseKind` | Нет | Нет | Нет | Нет | Нет |
| `ProcMacro2CaseTokenStream` | Нет | Нет | Да | Нет | Нет |
## Crate `naming_macros`

### Модуль `naming_macros`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `ProcMacro2GeneratedNamingTokenStream` | Нет | Нет | Да | Нет | Нет |
| `ProcMacro2VariantMatchingTokensRef` | Нет | Нет | Нет | Нет | Нет |
| `SynEnumIdentifierRef` | Нет | Нет | Нет | Нет | Нет |
## Crate `newtype`

### Модуль `newtype`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `NewtypeBool` | Нет | Нет | Нет | Нет | Нет |
| `ProcMacro2GeneratedTokenStream` | Нет | Нет | Да | Нет | Нет |
| `ProcMacroInputTokenStream` | Да | Нет | Да | Нет | Да |
| `SnakeIdentifier` | Да | Нет | Да | Нет | Нет |
| `SnakeIdentifierifierLen` | Да | Нет | Нет | Нет | Нет |
| `SynAttrsRef` | Да | Нет | Нет | Нет | Нет |
| `SynDeriveInputRef` | Да | Нет | Нет | Нет | Нет |
| `SynExpr` | Нет | Нет | Нет | Нет | Нет |
| `SynIdentifier` | Нет | Нет | Нет | Нет | Нет |
| `SynIdentifierRef` | Да | Нет | Нет | Нет | Нет |
| `SynType` | Нет | Нет | Нет | Нет | Нет |
| `SynTypeRef` | Да | Нет | Нет | Нет | Нет |
### Модуль `newtype::tests::newtype::tests`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `CheckedText` | Да | Нет | Да | Нет | Нет |
| `DebugValue` | Нет | Нет | Да | Нет | Нет |
| `DescribedValue` | Да | Нет | Да | Нет | Нет |
| `ExplicitErrorCheckedText` | Да | Нет | Да | Нет | Нет |
| `InnerValue` | Нет | Нет | Нет | Нет | Нет |
| `InnerVecValue` | Да | Нет | Да | Нет | Да |
| `MutableValueRef` | Нет | Нет | Нет | Нет | Нет |
| `OwnedSliceValue` | Да | Нет | Да | Нет | Да |
| `OwnedValue` | Да | Нет | Да | Нет | Да |
| `ProcMacro2TokenValue` | Да | Нет | Да | Нет | Да |
| `RedactedDebugValue` | Нет | Нет | Да | Нет | Нет |
| `ReferentValueRef` | Да | Нет | Нет | Нет | Нет |
| `RichValue` | Да | Да | Да | Да | Нет |
| `SliceValueRef` | Да | Нет | Нет | Нет | Нет |
| `StdTransparentErrorValue` | Нет | Нет | Нет | Нет | Нет |
| `StringValue` | Нет | Нет | Да | Нет | Нет |
| `TargetVecValue` | Нет | Нет | Да | Нет | Нет |
| `TransparentDebugValue` | Нет | Нет | Нет | Нет | Нет |
| `UsizeValue` | Да | Нет | Нет | Нет | Нет |
| `ValidatedValue` | Да | Нет | Да | Нет | Нет |
| `VecValue` | Нет | Нет | Да | Нет | Нет |
### Модуль `newtype::tests::newtype::tests::to_err_string`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `ToErrStringValue` | Да | Нет | Да | Нет | Да |
## Crate `notification_service`

### Модуль `notification_service`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `AxumNotificationJson` | Нет | Нет | Нет | Нет | Нет |
| `AxumNotificationResponse` | Нет | Нет | Нет | Нет | Нет |
| `AxumNotificationRouter` | Нет | Нет | Нет | Нет | Нет |
| `AxumNotificationState` | Нет | Нет | Нет | Нет | Нет |
| `HttpNotificationApiProblem` | Нет | Нет | Нет | Нет | Нет |
| `HttpNotificationStatusCode` | Нет | Нет | Нет | Нет | Нет |
| `MetricsExporterPrometheusHandle` | Нет | Нет | Нет | Нет | Нет |
| `MetricsExporterPrometheusNotificationBuildError` | Нет | Нет | Нет | Нет | Нет |
| `NotificationBodyMaximumBytes` | Нет | Нет | Нет | Нет | Нет |
| `NotificationConfigError` | Нет | Нет | Нет | Нет | Нет |
| `NotificationServeError` | Нет | Нет | Нет | Нет | Нет |
| `SqlxNotificationDatabaseError` | Нет | Нет | Нет | Нет | Нет |
| `SqlxNotificationMigrationError` | Нет | Нет | Нет | Нет | Нет |
| `StdNotificationExitCode` | Нет | Нет | Нет | Нет | Нет |
| `StdNotificationIoError` | Нет | Нет | Нет | Нет | Нет |
## Crate `notification_service_contract`

### Модуль `notification_service_contract`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `NotificationMessage` | Да | Да | Да | Да | Нет |
| `UuidNotificationId` | Да | Да | Нет | Нет | Нет |
## Crate `optml`

### Модуль `optml`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `SynFieldTyWithStaticLts` | Нет | Нет | Нет | Нет | Нет |
## Crate `panic_location`

### Модуль `panic_location`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `PanicColumn` | Нет | Нет | Нет | Нет | Нет |
| `PanicFile` | Нет | Нет | Нет | Нет | Нет |
| `PanicLine` | Нет | Нет | Нет | Нет | Нет |
| `PanicWithLocationMessage` | Да | Нет | Да | Нет | Нет |
## Crate `pg_crud_common`

### Модуль `pg_crud_common`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `AddOperator` | Да | Нет | Нет | Нет | Нет |
| `AllEnumVariants` | Да | Нет | Да | Нет | Да |
| `EqOperatorQueryStr` | Да | Нет | Нет | Нет | Нет |
| `IsPrimaryKey` | Да | Нет | Нет | Нет | Нет |
| `IsStringEmptyRes` | Да | Нет | Нет | Нет | Нет |
| `NonPrimaryKeyPgTypeReadIds` | Нет | Да | Нет | Нет | Нет |
| `NotEmptyUniqueVec` | Нет | Нет | Да | Нет | Нет |
| `NotZeroUnsignedPartOfI32` | Нет | Да | Нет | Нет | Нет |
| `NullableJsonObjPgTypeWhereFilter` | Да | Да | Нет | Нет | Нет |
| `OrderSnakeCaseStr` | Да | Нет | Да | Нет | Да |
| `OrderUpperCamelCaseStr` | Да | Нет | Да | Нет | Да |
| `PaginationStartsWithZero` | Нет | Да | Нет | Нет | Нет |
| `SqlxPostgresQuery` | Да | Нет | Нет | Нет | Нет |
| `UnsignedPartOfI32` | Нет | Да | Нет | Нет | Нет |
| `UnsignedPartOfI32Raw` | Да | Да | Нет | Нет | Нет |
| `UuidUuidTestCases` | Да | Нет | Нет | Нет | Нет |
### Модуль `pg_crud_common::advisory_lock`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `PgRelationCapacityMaximum` | Да | Нет | Нет | Нет | Нет |
| `PgRelationLockNamespace` | Да | Нет | Да | Нет | Нет |
| `PgRelationResourceId` | Да | Нет | Нет | Нет | Нет |
| `PgRelationResourceIds` | Да | Нет | Да | Нет | Нет |
| `PgRelationRowCount` | Нет | Нет | Нет | Нет | Нет |
| `SqlxPgRelationLockConnectionRef` | Да | Нет | Нет | Нет | Нет |
| `SqlxPgRelationLockError` | Нет | Нет | Нет | Нет | Нет |
### Модуль `pg_crud_common::batch_validation`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `BatchInvalidItemCount` | Нет | Нет | Нет | Нет | Нет |
| `BatchInvalidItems` | Да | Нет | Да | Нет | Да |
| `BatchProcessedItemCount` | Да | Нет | Нет | Нет | Нет |
| `BatchStoppedEarly` | Да | Нет | Нет | Нет | Нет |
| `StdBatchRecords` | Да | Нет | Да | Нет | Да |
### Модуль `pg_crud_common::bind_index`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `QueryPartIncrement` | Да | Нет | Нет | Нет | Нет |
### Модуль `pg_crud_common::bounded_btree_map`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `StdBoundedBTreeMap` | Нет | Да | Да | Да | Нет |
| `StdBoundedBTreeMapLen` | Да | Нет | Нет | Нет | Нет |
| `StdBoundedBTreeMapVisitor` | Нет | Нет | Нет | Нет | Нет |
### Модуль `pg_crud_common::bounded_unique_vec`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `BoundedUniqueVec` | Нет | Да | Да | Да | Нет |
| `StdBoundedUniqueVecVisitor` | Нет | Нет | Нет | Нет | Нет |
| `UniqueVecLen` | Нет | Нет | Нет | Нет | Нет |
### Модуль `pg_crud_common::bounded_vec`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `BoundedVec` | Нет | Да | Да | Да | Нет |
| `BoundedVecLen` | Да | Нет | Нет | Нет | Нет |
| `StdPhantomDataBoundedVecVisitor` | Нет | Нет | Нет | Нет | Нет |
### Модуль `pg_crud_common::cardinality`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `DuplicateCandidates` | Да | Нет | Да | Нет | Да |
| `DuplicateIdx` | Да | Нет | Нет | Нет | Нет |
### Модуль `pg_crud_common::cursor`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `CursorMaximumLength` | Да | Нет | Нет | Нет | Нет |
| `CursorPayload` | Да | Нет | Да | Нет | Нет |
| `CursorSigningKey` | Да | Нет | Да | Нет | Нет |
| `SignedCursor` | Нет | Нет | Да | Нет | Нет |
### Модуль `pg_crud_common::date_sql_filter`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `ChronoUtcDateTimeRef` | Да | Нет | Нет | Нет | Нет |
| `ChronoUtcDateTimes` | Нет | Нет | Да | Нет | Нет |
| `StdDateSqlBindStart` | Да | Нет | Нет | Нет | Нет |
### Модуль `pg_crud_common::db_schema_conformance`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `DbColumnContractSnapshots` | Нет | Нет | Да | Нет | Да |
| `DbColumnHasServerDefault` | Да | Нет | Нет | Нет | Нет |
| `DbColumnNullable` | Да | Нет | Нет | Нет | Нет |
| `DbColumnSnapshots` | Нет | Нет | Да | Нет | Да |
| `DbColumnSpecs` | Нет | Нет | Да | Нет | Да |
| `DbDefaultSpecs` | Нет | Нет | Да | Нет | Да |
| `DbKeyContractSnapshots` | Нет | Нет | Да | Нет | Да |
| `DbKeySpecs` | Нет | Нет | Да | Нет | Да |
| `DbObjectSnapshots` | Нет | Нет | Да | Нет | Да |
| `DbObjectSpecs` | Нет | Нет | Да | Нет | Да |
| `DbSchemaNameRef` | Да | Нет | Нет | Нет | Нет |
| `DbSchemaText` | Да | Нет | Да | Нет | Нет |
| `DbSchemaTextError` | Нет | Нет | Нет | Нет | Нет |
| `DbSchemaTexts` | Нет | Нет | Да | Нет | Да |
| `DbStaticSchemaText` | Да | Нет | Нет | Нет | Нет |
| `DbStaticSchemaTexts` | Нет | Нет | Да | Нет | Да |
| `DbTableNameRef` | Да | Нет | Нет | Нет | Нет |
| `SqlxDbSchemaInspectionError` | Нет | Нет | Нет | Нет | Нет |
| `SqlxPgPoolRef` | Да | Нет | Нет | Нет | Нет |
### Модуль `pg_crud_common::errors`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `SqlxPostgresQueryBindError` | Да | Нет | Да | Нет | Да |
### Модуль `pg_crud_common::filter_bind_plan`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `PgFilterBindValues` | Нет | Нет | Да | Нет | Нет |
| `PgFilterBool` | Да | Нет | Нет | Нет | Нет |
| `PgFilterI64` | Да | Нет | Нет | Нет | Нет |
| `PgFilterText` | Да | Нет | Да | Нет | Нет |
### Модуль `pg_crud_common::finite_f64`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `FiniteF64` | Да | Нет | Нет | Нет | Нет |
| `PositiveFiniteF64` | Да | Нет | Нет | Нет | Нет |
| `UnitIntervalF64` | Да | Нет | Нет | Нет | Нет |
### Модуль `pg_crud_common::invariants`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `PaginationTotal` | Да | Нет | Нет | Нет | Нет |
### Модуль `pg_crud_common::list_total`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `ListItems` | Да | Нет | Да | Нет | Да |
| `ListOffset` | Да | Нет | Нет | Нет | Нет |
| `ListTotal` | Нет | Нет | Нет | Нет | Нет |
### Модуль `pg_crud_common::operation_budget`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `OperationBudget` | Да | Нет | Нет | Нет | Нет |
| `OperationCount` | Нет | Нет | Нет | Нет | Нет |
### Модуль `pg_crud_common::operational_invariants`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `PgCounterValue` | Нет | Нет | Нет | Нет | Нет |
| `PgOperationalLimit` | Да | Нет | Нет | Нет | Нет |
| `PgScopedForeignKeyClauseText` | Да | Нет | Да | Нет | Нет |
| `PgSqlIdentifiers` | Да | Нет | Да | Нет | Да |
### Модуль `pg_crud_common::order_preserving_deduplication`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `OrderPreservingValues` | Да | Нет | Да | Нет | Да |
### Модуль `pg_crud_common::pagination`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `PaginationEnd` | Нет | Нет | Нет | Нет | Нет |
| `PaginationLimit` | Нет | Да | Нет | Нет | Нет |
| `PaginationOffset` | Нет | Да | Нет | Нет | Нет |
| `PaginationStart` | Нет | Нет | Нет | Нет | Нет |
### Модуль `pg_crud_common::pg_error`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `SqlxPgErrorRef` | Да | Нет | Нет | Нет | Нет |
### Модуль `pg_crud_common::query_fragment`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `QueryPartFragment` | Нет | Нет | Да | Нет | Да |
| `SqlColumnRef` | Да | Нет | Нет | Нет | Нет |
### Модуль `pg_crud_common::read_query_plan`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `ReadQueryPlan` | Нет | Нет | Нет | Нет | Нет |
| `SqlSortOrderText` | Нет | Нет | Нет | Нет | Нет |
| `StdReadQueryBindIndex` | Да | Нет | Нет | Нет | Нет |
### Модуль `pg_crud_common::sql_identifier`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `SqlIdentifier` | Да | Нет | Да | Нет | Нет |
| `SqlIdentifiers` | Да | Нет | Да | Нет | Да |
| `SqlQueryText` | Нет | Нет | Да | Нет | Нет |
### Модуль `pg_crud_common::sql_like_pattern`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `SqlLikeInputRef` | Да | Нет | Нет | Нет | Нет |
| `SqlLikePattern` | Да | Да | Да | Да | Нет |
### Модуль `pg_crud_common::tests_not_empty_unique_vec`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `NonClone` | Нет | Нет | Нет | Нет | Нет |
## Crate `pg_crud_macros_common`

### Модуль `pg_crud_macros_common`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `DeLen` | Да | Нет | Нет | Нет | Нет |
| `DimensionNumber` | Да | Нет | Нет | Нет | Нет |
| `GeneratedRustTokenStreamVec` | Нет | Нет | Да | Нет | Да |
| `ImportPathStr` | Да | Нет | Нет | Нет | Нет |
| `ImportSnakeCaseStr` | Да | Нет | Нет | Нет | Нет |
| `IsNullablePrefixStr` | Да | Нет | Да | Нет | Нет |
| `NonNullOrNullableStr` | Да | Нет | Нет | Нет | Нет |
| `PanicUuidRef` | Да | Нет | Нет | Нет | Нет |
| `ParseErrorIdRef` | Да | Нет | Нет | Нет | Нет |
| `ParseTokenStreamStrings` | Да | Нет | Да | Нет | Да |
| `StructElsLen` | Да | Нет | Нет | Нет | Нет |
| `SynFieldRefs` | Да | Нет | Нет | Нет | Нет |
| `SynIdentifierTypeRefs` | Да | Нет | Нет | Нет | Нет |
| `WrapIntoBraces` | Да | Нет | Нет | Нет | Нет |
## Crate `pg_table`

### Модуль `pg_table`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `PgTableIdempotencyActor` | Да | Нет | Да | Нет | Нет |
| `PgTableIdempotencyBody` | Да | Нет | Да | Нет | Да |
| `PgTableIdempotencyBodyRef` | Да | Нет | Нет | Нет | Нет |
| `PgTableIdempotencyCleanupBatchSize` | Да | Нет | Нет | Нет | Нет |
| `PgTableIdempotencyCleanupRetentionSeconds` | Да | Нет | Нет | Нет | Нет |
| `PgTableIdempotencyCleanupRows` | Да | Нет | Нет | Нет | Нет |
| `PgTableIdempotencyKey` | Нет | Нет | Да | Нет | Нет |
| `PgTableIdempotencyMethod` | Да | Нет | Да | Нет | Нет |
| `PgTableIdempotencyRequestHash` | Нет | Нет | Нет | Нет | Нет |
| `PgTableIdempotencyResponseStatus` | Да | Нет | Нет | Нет | Нет |
| `PgTableIdempotencyRoute` | Да | Нет | Да | Нет | Нет |
| `PgTableIdempotencyTextBytes` | Да | Нет | Нет | Нет | Нет |
| `PgTableNameRef` | Да | Нет | Нет | Нет | Нет |
| `PgTableQueryPartFragment` | Да | Нет | Да | Нет | Да |
| `PgTableQueryString` | Да | Нет | Да | Нет | Да |
| `PgTableRevision` | Да | Нет | Нет | Нет | Нет |
| `PgTableSqlFragmentRef` | Да | Нет | Нет | Нет | Нет |
| `SqlxPgTableIdempotencyError` | Нет | Нет | Нет | Нет | Нет |
| `SqlxPgTablePgConnectionRef` | Да | Нет | Нет | Нет | Нет |
| `StdPgTableRevisionParseIntError` | Нет | Нет | Нет | Нет | Нет |
## Crate `pg_types_common`

### Модуль `pg_types_common`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `IsPrimaryKey` | Да | Нет | Нет | Нет | Нет |
| `PaginationStartsWithOne` | Нет | Да | Нет | Нет | Нет |
| `PaginationStartsWithOneValue` | Да | Да | Нет | Нет | Нет |
## Crate `prepare_postgresql_databases`

### Модуль `prepare_postgresql_databases`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `DatabaseUrl` | Да | Нет | Да | Нет | Нет |
| `MigrationsSource` | Да | Нет | Да | Нет | Нет |
| `ProcessArguments` | Нет | Нет | Да | Нет | Нет |
| `ProcessCommands` | Да | Нет | Да | Нет | Да |
| `ProcessProgram` | Нет | Нет | Нет | Нет | Нет |
| `ProcessStaticArgument` | Нет | Нет | Нет | Нет | Нет |
## Crate `route_validators`

### Модуль `route_validators`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `AxumHttpStatusCode` | Нет | Нет | Нет | Нет | Нет |
### Модуль `route_validators::check_body_size`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `AxumBody` | Да | Нет | Нет | Нет | Нет |
| `AxumBodySizeError` | Нет | Нет | Нет | Нет | Нет |
| `BodySizeLimitBytes` | Да | Да | Нет | Нет | Нет |
| `BytesBodyBytes` | Нет | Нет | Да | Нет | Нет |
| `HttpBodySizeHint` | Нет | Нет | Нет | Нет | Нет |
### Модуль `route_validators::check_commit`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `AxumCommitToStrConversionError` | Нет | Нет | Нет | Нет | Нет |
| `CommitNotEqMessage` | Нет | Нет | Нет | Нет | Нет |
| `CommitToUse` | Нет | Нет | Нет | Нет | Нет |
| `EnableApiGitCommitCheck` | Да | Нет | Нет | Нет | Нет |
| `NoCommitHeaderMessage` | Нет | Нет | Нет | Нет | Нет |
### Модуль `route_validators::hdr_val`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `AxumHeaderValueRef` | Нет | Нет | Нет | Нет | Нет |
| `AxumHeadersRef` | Да | Нет | Нет | Нет | Нет |
| `HeaderStrRef` | Нет | Нет | Нет | Нет | Нет |
### Модуль `route_validators::test_hlp`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `AxumTestHeaderValue` | Нет | Нет | Нет | Нет | Нет |
| `AxumTestHeaders` | Нет | Нет | Нет | Нет | Нет |
| `AxumTestHeadersMutRef` | Нет | Нет | Нет | Нет | Нет |
| `TestExpId` | Нет | Нет | Нет | Нет | Нет |
| `TestPanicText` | Нет | Нет | Нет | Нет | Нет |
| `TestPollCount` | Нет | Нет | Нет | Нет | Нет |
| `TestPollLimitReached` | Нет | Нет | Нет | Нет | Нет |
## Crate `server`

### Модуль `server`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `AxumApiRoutes` | Нет | Нет | Нет | Нет | Нет |
| `MetricsExporterPrometheusBuildError` | Нет | Нет | Нет | Нет | Нет |
| `MetricsExporterPrometheusHandle` | Нет | Нет | Нет | Нет | Нет |
| `ServerAdminAuthSvcStateBuildError` | Нет | Нет | Нет | Нет | Нет |
| `ServerAdminCleanupCfgError` | Нет | Нет | Нет | Нет | Нет |
| `ServerAdminMigrateError` | Нет | Нет | Нет | Нет | Нет |
| `ServerConfigError` | Нет | Нет | Нет | Нет | Нет |
| `ServerRuntimeBackgroundTaskShutdownError` | Нет | Нет | Нет | Нет | Нет |
| `ServerRuntimeContentSecurityPolicyError` | Нет | Нет | Нет | Нет | Нет |
| `ServerRuntimeRequestTimeoutError` | Нет | Нет | Нет | Нет | Нет |
| `ServerRuntimeRunIntervalError` | Нет | Нет | Нет | Нет | Нет |
| `ServerRuntimeServeError` | Нет | Нет | Нет | Нет | Нет |
| `SqlxServerPgConnectError` | Нет | Нет | Нет | Нет | Нет |
| `StdServerExitCode` | Нет | Нет | Нет | Нет | Нет |
| `StdServerIoError` | Нет | Нет | Нет | Нет | Нет |
| `StdSharedServerAppState` | Нет | Нет | Нет | Нет | Нет |
| `TokioServerRuntime` | Нет | Нет | Нет | Нет | Нет |
## Crate `server_admin`

### Модуль `server_admin`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `AdminAccessTokenError` | Нет | Нет | Нет | Нет | Нет |
| `AdminCleanupBatchSize` | Да | Нет | Нет | Нет | Нет |
| `AdminCleanupRetentionSeconds` | Да | Нет | Нет | Нет | Нет |
| `AdminCleanupRows` | Нет | Нет | Нет | Нет | Нет |
| `AdminCookieMaxAgeSeconds` | Да | Нет | Нет | Нет | Нет |
| `AdminCookieSecure` | Да | Нет | Нет | Нет | Нет |
| `AdminJwtSecret` | Нет | Нет | Нет | Нет | Нет |
| `AdminMigrateError` | Нет | Нет | Нет | Нет | Нет |
| `AdminOpaqueToken` | Нет | Нет | Нет | Нет | Нет |
| `AdminPassword` | Нет | Да | Нет | Нет | Нет |
| `AdminPasswordHash` | Нет | Нет | Нет | Нет | Нет |
| `AdminPasswordHashConcurrency` | Да | Нет | Нет | Нет | Нет |
| `AdminPermissions` | Да | Нет | Да | Нет | Да |
| `AdminRefreshToken` | Нет | Нет | Нет | Нет | Нет |
| `AdminRoleNames` | Да | Нет | Да | Нет | Да |
| `AdminSessionId` | Да | Да | Нет | Нет | Нет |
| `AdminTokenHash` | Нет | Нет | Нет | Нет | Нет |
| `AdminUnixTokenStream` | Да | Да | Нет | Нет | Нет |
| `Argon2AdminPasswordHashError` | Да | Нет | Нет | Нет | Нет |
| `HttpAdminHeaderMapRef` | Да | Нет | Нет | Нет | Нет |
| `JsonwebtokenAdminError` | Да | Нет | Нет | Нет | Нет |
| `SqlxAdminError` | Да | Нет | Нет | Нет | Нет |
| `SqlxAdminMigrateError` | Да | Нет | Нет | Нет | Нет |
| `StdAdminAccessToken` | Да | Нет | Да | Нет | Нет |
| `StdAdminCookie` | Нет | Нет | Да | Нет | Нет |
| `StdAdminSharedSemaphore` | Нет | Нет | Нет | Нет | Нет |
| `TokioAdminAcquireError` | Да | Нет | Нет | Нет | Нет |
| `TokioAdminJoinError` | Да | Нет | Нет | Нет | Нет |
### Модуль `server_admin::auth`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `AdminHtmlSwaggerEnabled` | Да | Нет | Нет | Нет | Нет |
| `AdminPeerAddr` | Нет | Нет | Нет | Нет | Нет |
| `AdminSessionPath` | Нет | Нет | Нет | Нет | Нет |
| `AdminSignInJson` | Нет | Нет | Нет | Нет | Нет |
| `AxumAdminAuthRouter` | Нет | Нет | Нет | Нет | Нет |
| `AxumAdminForm` | Нет | Нет | Нет | Нет | Нет |
| `AxumAdminJson` | Нет | Нет | Нет | Нет | Нет |
| `AxumAdminPath` | Нет | Нет | Нет | Нет | Нет |
| `AxumAdminQuery` | Нет | Нет | Нет | Нет | Нет |
| `AxumAdminResponse` | Нет | Нет | Нет | Нет | Нет |
| `HttpAdminHeaderMap` | Нет | Нет | Нет | Нет | Нет |
| `HttpAdminHeaderValueError` | Да | Нет | Нет | Нет | Нет |
| `JsonwebtokenAdminDecodingKey` | Нет | Нет | Нет | Нет | Нет |
| `JsonwebtokenAdminDecodingKeys` | Да | Нет | Да | Нет | Да |
| `JsonwebtokenAdminEncodingKey` | Нет | Нет | Нет | Нет | Нет |
| `SqlxAdminPgConnectionRef` | Да | Нет | Нет | Нет | Нет |
| `StdAdminAccessTtlSeconds` | Да | Нет | Нет | Нет | Нет |
| `StdAdminFailureDelayMillis` | Да | Нет | Нет | Нет | Нет |
| `StdAdminFailureThreshold` | Да | Нет | Нет | Нет | Нет |
| `StdAdminRateLimitCount` | Да | Нет | Нет | Нет | Нет |
| `StdAdminRateLimitWindowSeconds` | Да | Нет | Нет | Нет | Нет |
| `StdAdminRefreshTtlSeconds` | Да | Нет | Нет | Нет | Нет |
| `StdAdminSessionLimit` | Да | Нет | Нет | Нет | Нет |
| `StdSharedAdminAuthSvcState` | Да | Нет | Нет | Нет | Нет |
| `UtoipaAdminAuthOpenApi` | Нет | Нет | Нет | Нет | Нет |
### Модуль `server_admin::auth::html`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `AdminHtmlFormKey` | Да | Да | Да | Да | Нет |
| `AdminHtmlFormText` | Да | Да | Да | Да | Нет |
| `StdAdminHtmlSelected` | Да | Да | Да | Да | Нет |
### Модуль `server_admin::domain`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `AdminAuditLogId` | Нет | Нет | Нет | Нет | Нет |
| `AdminPermissionId` | Нет | Да | Нет | Нет | Нет |
| `AdminPermissionName` | Нет | Нет | Нет | Нет | Нет |
| `AdminRoleId` | Нет | Да | Нет | Нет | Нет |
| `AdminUserId` | Нет | Да | Нет | Нет | Нет |
| `SecrecyAdminString` | Нет | Нет | Да | Нет | Да |
| `StdAdminBool` | Нет | Да | Нет | Нет | Нет |
| `StdAdminNonZeroUsize` | Нет | Нет | Нет | Нет | Нет |
| `StdAdminSocketAddr` | Нет | Нет | Нет | Нет | Нет |
| `StdAdminStrRef` | Нет | Нет | Нет | Нет | Нет |
| `StdAdminString` | Нет | Нет | Да | Нет | Нет |
| `UuidAdminValue` | Нет | Да | Нет | Нет | Нет |
### Модуль `server_admin::generated_tables`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `UtoipaAdminOpenApi` | Нет | Нет | Нет | Нет | Нет |
### Модуль `server_admin::repository`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `AdminPageTotalCount` | Да | Нет | Нет | Нет | Нет |
| `AdminRecentLoginFailureCount` | Да | Нет | Нет | Нет | Нет |
| `SqlxAdminRepositoryConnectionMutRef` | Да | Нет | Нет | Нет | Нет |
| `SqlxAdminRepositoryPoolRef` | Да | Нет | Нет | Нет | Нет |
### Модуль `server_admin::repository::roles`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `AdminActiveAdministratorCount` | Да | Нет | Нет | Нет | Нет |
### Модуль `server_admin::tests::admin_api`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `AdminHtmlTestBody` | Да | Нет | Да | Нет | Нет |
| `AdminHtmlTestFormBody` | Да | Нет | Да | Нет | Нет |
| `AxumAdminApiTestRouter` | Нет | Нет | Нет | Нет | Нет |
| `HttpAdminApiTestMethod` | Нет | Нет | Нет | Нет | Нет |
| `HttpAdminApiTestRequest` | Нет | Нет | Нет | Нет | Нет |
| `HttpAdminApiTestResponseRef` | Нет | Нет | Нет | Нет | Нет |
| `HttpAdminHtmlTestResponse` | Да | Нет | Нет | Нет | Нет |
| `SqlxAdminApiTestPool` | Нет | Нет | Нет | Нет | Нет |
| `SqlxAdminHtmlTestTransaction` | Нет | Нет | Нет | Нет | Нет |
| `StdAdminApiTestCookie` | Да | Нет | Да | Нет | Нет |
| `StdAdminApiTestStrRef` | Нет | Нет | Нет | Нет | Нет |
## Crate `server_admin_contract`

### Модуль `server_admin_contract`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `AdminApiBodyMaxBytes` | Нет | Нет | Нет | Нет | Нет |
| `AdminAuditDetailsBytes` | Да | Нет | Нет | Нет | Нет |
| `AdminAuditExportCsv` | Да | Да | Да | Да | Нет |
| `AdminAuditLogId` | Да | Да | Нет | Нет | Нет |
| `AdminAuditTimestamp` | Да | Да | Да | Да | Нет |
| `AdminAuditViews` | Да | Да | Да | Да | Да |
| `AdminBool` | Нет | Да | Нет | Нет | Нет |
| `AdminDataRows` | Да | Да | Да | Да | Да |
| `AdminDataTableStrRef` | Да | Нет | Нет | Нет | Нет |
| `AdminDataTables` | Да | Да | Да | Да | Да |
| `AdminDefaultRoute` | Да | Да | Да | Да | Нет |
| `AdminDisplayName` | Да | Да | Да | Да | Нет |
| `AdminLogin` | Да | Да | Да | Да | Нет |
| `AdminMainLogo` | Да | Да | Да | Да | Нет |
| `AdminNewPassword` | Да | Да | Да | Да | Нет |
| `AdminOptionalSettings` | Да | Да | Да | Да | Да |
| `AdminOrganizationContacts` | Да | Да | Да | Да | Нет |
| `AdminOrganizationName` | Да | Да | Да | Да | Нет |
| `AdminPageLimit` | Нет | Да | Нет | Нет | Нет |
| `AdminPageOffset` | Нет | Да | Нет | Нет | Нет |
| `AdminPagePathRef` | Да | Нет | Нет | Нет | Нет |
| `AdminPageTotal` | Нет | Да | Нет | Нет | Нет |
| `AdminPassword` | Да | Да | Да | Да | Нет |
| `AdminPermissionId` | Да | Да | Нет | Нет | Нет |
| `AdminPermissionIds` | Нет | Да | Да | Да | Да |
| `AdminPermissionStrRef` | Да | Нет | Нет | Нет | Нет |
| `AdminPermissionSummaries` | Да | Да | Да | Да | Да |
| `AdminPermissionValue` | Да | Да | Да | Да | Нет |
| `AdminPermissionValues` | Да | Да | Да | Да | Да |
| `AdminPrimaryColor` | Да | Да | Да | Да | Нет |
| `AdminRoleId` | Да | Да | Нет | Нет | Нет |
| `AdminRoleIds` | Нет | Да | Да | Да | Да |
| `AdminRoleName` | Да | Да | Да | Да | Нет |
| `AdminRoleNames` | Да | Да | Да | Да | Да |
| `AdminRoleSummaries` | Да | Да | Да | Да | Да |
| `AdminRoutePath` | Нет | Нет | Да | Нет | Нет |
| `AdminSessionIdentifier` | Да | Да | Да | Да | Нет |
| `AdminSessionTimestamp` | Да | Да | Да | Да | Нет |
| `AdminSessionViews` | Да | Да | Да | Да | Да |
| `AdminSiteName` | Да | Да | Да | Да | Нет |
| `AdminSupportUrl` | Да | Да | Да | Да | Нет |
| `AdminTabTitle` | Да | Да | Да | Да | Нет |
| `AdminTableSearch` | Нет | Да | Да | Да | Нет |
| `AdminTableSortKey` | Нет | Да | Да | Да | Нет |
| `AdminTableSortKeyRef` | Да | Нет | Нет | Нет | Нет |
| `AdminText` | Да | Да | Да | Да | Нет |
| `AdminTexts` | Да | Да | Да | Да | Да |
| `AdminUserId` | Да | Да | Нет | Нет | Нет |
| `AdminUserSummaries` | Да | Да | Да | Да | Да |
| `SerdeJsonAdminAuditDetails` | Да | Да | Да | Да | Нет |
## Crate `server_admin_frontend`

### Модуль `server_admin_frontend`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `AxumAdminFrontendRouter` | Нет | Нет | Нет | Нет | Нет |
### Модуль `server_admin_frontend::ssr`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `AdminSsrErrorMessage` | Да | Нет | Да | Нет | Нет |
| `AdminSsrHtml` | Нет | Нет | Да | Нет | Нет |
| `AdminSsrText` | Нет | Нет | Да | Нет | Нет |
## Crate `server_runtime`

### Модуль `server_runtime`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `AxumRouter` | Нет | Нет | Нет | Нет | Нет |
| `HttpContentSecurityPolicy` | Да | Нет | Нет | Нет | Нет |
| `RequestTimeoutLayer` | Да | Нет | Нет | Нет | Нет |
| `RequestTimeoutTowerLayer` | Нет | Нет | Нет | Нет | Нет |
| `ReqwestClient` | Нет | Нет | Нет | Нет | Нет |
| `ReqwestClientBuildError` | Нет | Нет | Нет | Нет | Нет |
| `StdRequestTimeoutMessage` | Нет | Нет | Нет | Нет | Нет |
| `StdReqwestConnectTimeout` | Да | Нет | Нет | Нет | Нет |
| `StdReqwestRequestTimeout` | Да | Нет | Нет | Нет | Нет |
| `StdServeIoError` | Нет | Нет | Нет | Нет | Нет |
| `TokioTcpListener` | Да | Нет | Нет | Нет | Нет |
### Модуль `server_runtime::batched_cleanup`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `CleanupBatchCount` | Нет | Нет | Нет | Нет | Нет |
| `CleanupBatchSize` | Да | Нет | Нет | Нет | Нет |
| `CleanupRows` | Нет | Нет | Нет | Нет | Нет |
### Модуль `server_runtime::bounded_read`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `BoundedBytes` | Нет | Нет | Да | Нет | Нет |
| `BoundedJsonText` | Да | Нет | Да | Нет | Нет |
| `BoundedReadMaximumBytes` | Да | Нет | Нет | Нет | Нет |
| `BoundedReadObservedBytes` | Нет | Нет | Нет | Нет | Нет |
| `BoundedText` | Да | Нет | Да | Нет | Нет |
| `ReqwestError` | Нет | Нет | Нет | Нет | Нет |
| `ReqwestResponse` | Да | Нет | Нет | Нет | Нет |
| `SerdeJsonError` | Нет | Нет | Нет | Нет | Нет |
| `StdBoundedReadConcurrency` | Нет | Нет | Нет | Нет | Нет |
| `StdBoundedReadConcurrencyMaximum` | Да | Нет | Нет | Нет | Нет |
| `StdFromUtf8Error` | Нет | Нет | Нет | Нет | Нет |
| `StdIoError` | Нет | Нет | Нет | Нет | Нет |
| `StdPathRef` | Да | Нет | Нет | Нет | Нет |
### Модуль `server_runtime::child_process`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `ChildDiagnostic` | Нет | Нет | Да | Нет | Нет |
| `ChildProcessId` | Нет | Нет | Нет | Нет | Нет |
| `ChildProcessReports` | Нет | Нет | Да | Нет | Нет |
| `StdChildDiagnosticMaximum` | Да | Нет | Нет | Нет | Нет |
| `StdChildExitStatus` | Нет | Нет | Нет | Нет | Нет |
| `StdChildProcessIoError` | Да | Нет | Нет | Нет | Нет |
| `StdChildProcessSetMaximum` | Да | Нет | Нет | Нет | Нет |
| `StdCollectionsChildProcessMap` | Нет | Нет | Да | Нет | Нет |
| `TokioChildDiagnosticTask` | Нет | Нет | Нет | Нет | Нет |
| `TokioChildProcess` | Да | Нет | Нет | Нет | Нет |
| `TokioChildProcessJoinError` | Да | Нет | Нет | Нет | Нет |
| `TokioManagedChild` | Нет | Нет | Нет | Нет | Нет |
### Модуль `server_runtime::client_ip`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `HttpHeaderMapRef` | Да | Нет | Нет | Нет | Нет |
| `StdAddrParseError` | Да | Нет | Нет | Нет | Нет |
| `StdIpAddr` | Да | Нет | Нет | Нет | Нет |
| `StdParseIntError` | Да | Нет | Нет | Нет | Нет |
| `StdRangeContains` | Нет | Нет | Нет | Нет | Нет |
| `StdResolvedClientIp` | Нет | Нет | Нет | Нет | Нет |
| `StdSocketAddr` | Да | Нет | Нет | Нет | Нет |
| `StdTrustedProxyPrefixBits` | Да | Нет | Нет | Нет | Нет |
| `TrustedProxyRanges` | Нет | Нет | Да | Нет | Да |
### Модуль `server_runtime::cors`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `HttpCorsAllowOriginHeaderValues` | Нет | Нет | Да | Нет | Нет |
| `HttpCorsAllowOriginTextRef` | Да | Нет | Нет | Нет | Нет |
### Модуль `server_runtime::csp`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `HttpCspBuilder` | Нет | Нет | Да | Нет | Нет |
| `HttpCspDirectiveName` | Да | Нет | Да | Нет | Нет |
| `HttpCspDirectiveValue` | Да | Нет | Да | Нет | Нет |
### Модуль `server_runtime::deduplicating_queue`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `StdCollectionsHashSet` | Да | Нет | Да | Нет | Да |
| `StdCollectionsVecDeque` | Да | Нет | Да | Нет | Да |
| `StdQueueMaximum` | Да | Нет | Нет | Нет | Нет |
### Модуль `server_runtime::exclusive_run`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `StdExclusiveRunAtomicBool` | Нет | Нет | Нет | Нет | Нет |
### Модуль `server_runtime::fallback`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `AcceptsApplicationJson` | Нет | Нет | Нет | Нет | Нет |
| `HttpAcceptHeaderMaximumBytes` | Да | Нет | Нет | Нет | Нет |
| `HttpFallbackApiPrefixRef` | Да | Нет | Нет | Нет | Нет |
| `HttpFallbackMetricsPathRef` | Да | Нет | Нет | Нет | Нет |
| `HttpFallbackRequestPathRef` | Да | Нет | Нет | Нет | Нет |
| `HttpMediaRangeRef` | Нет | Нет | Нет | Нет | Нет |
| `HttpOptionalAcceptHeaderRef` | Да | Нет | Нет | Нет | Нет |
### Модуль `server_runtime::generation_gate`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `Generation` | Нет | Нет | Нет | Нет | Нет |
| `StdGenerationAtomicU64` | Нет | Нет | Нет | Нет | Нет |
### Модуль `server_runtime::geojson`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `GeoJsonDocumentText` | Да | Нет | Да | Нет | Нет |
| `SerdeJsonGeoJsonError` | Нет | Нет | Нет | Нет | Нет |
### Модуль `server_runtime::header_text`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `HttpHeaderName` | Да | Нет | Нет | Нет | Нет |
| `HttpHeaderTextBytes` | Да | Нет | Нет | Нет | Нет |
| `HttpHeaderTextMaximumBytes` | Да | Нет | Нет | Нет | Нет |
| `HttpHeaderTextRef` | Да | Нет | Нет | Нет | Нет |
### Модуль `server_runtime::health`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `HealthProbeSucceeded` | Нет | Нет | Нет | Нет | Нет |
| `StdHealthProbeTimeout` | Да | Нет | Нет | Нет | Нет |
| `StdHealthReadinessAtomicBool` | Нет | Нет | Нет | Нет | Нет |
| `StdSharedHealthReadiness` | Нет | Нет | Нет | Нет | Нет |
### Модуль `server_runtime::history`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `StdArcSharedRunReports` | Нет | Нет | Нет | Нет | Нет |
| `StdAsyncRunHistoryMaximumLen` | Да | Нет | Нет | Нет | Нет |
| `StdAsyncRunHistoryReportCount` | Нет | Нет | Нет | Нет | Нет |
| `StdVecDequeRunReports` | Нет | Нет | Да | Нет | Нет |
| `TokioRwLockRunReports` | Нет | Нет | Нет | Нет | Нет |
### Модуль `server_runtime::http_header_policy`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `HttpAttachmentFileNameRef` | Да | Нет | Нет | Нет | Нет |
| `HttpContentDisposition` | Нет | Нет | Нет | Нет | Нет |
| `HttpContentLength` | Да | Нет | Да | Нет | Нет |
### Модуль `server_runtime::http_policy`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `HttpAuthorizationHeaderTextRef` | Да | Нет | Нет | Нет | Нет |
| `HttpBearerTokenRef` | Нет | Нет | Нет | Нет | Нет |
| `HttpContentTypeTextRef` | Да | Нет | Нет | Нет | Нет |
| `HttpCookieHeadersRef` | Да | Нет | Нет | Нет | Нет |
| `HttpCookieNameRef` | Да | Нет | Нет | Нет | Нет |
| `HttpCookieValueRef` | Нет | Нет | Нет | Нет | Нет |
### Модуль `server_runtime::http_status_error`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `HttpErrorStatus` | Да | Нет | Нет | Нет | Нет |
### Модуль `server_runtime::lease_registry`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `LeaseId` | Да | Нет | Да | Нет | Нет |
| `LeaseIds` | Нет | Нет | Да | Нет | Нет |
| `LeaseKey` | Да | Нет | Да | Нет | Нет |
| `LeaseTextRef` | Нет | Нет | Нет | Нет | Нет |
| `StdArcTokioLeaseRegistryRwLock` | Нет | Нет | Нет | Нет | Нет |
| `StdLeaseRegistryMaximum` | Да | Нет | Нет | Нет | Нет |
| `StdLeaseStaleTimeout` | Да | Нет | Нет | Нет | Нет |
| `TokioLeaseInstant` | Нет | Нет | Нет | Нет | Нет |
| `TokioLeaseRegistryRwLock` | Нет | Нет | Нет | Нет | Нет |
### Модуль `server_runtime::lifecycle`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `StdRequestTimeout` | Да | Нет | Нет | Нет | Нет |
| `StdRunInterval` | Да | Нет | Нет | Нет | Нет |
| `TokioAbortTask` | Да | Нет | Нет | Нет | Нет |
| `TokioBackgroundTaskJoinHandle` | Да | Нет | Нет | Нет | Нет |
| `TokioBackgroundTaskShutdownSender` | Да | Нет | Нет | Нет | Нет |
| `TokioTaskJoinError` | Нет | Нет | Нет | Нет | Нет |
### Модуль `server_runtime::limits`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `RetryAfterSecs` | Да | Нет | Нет | Нет | Нет |
| `StdArcTokioSemaphore` | Нет | Нет | Нет | Нет | Нет |
| `StdPermitWaitTimeout` | Да | Нет | Нет | Нет | Нет |
| `StdSemaphorePermitCount` | Да | Нет | Нет | Нет | Нет |
| `TokioAcquireError` | Нет | Нет | Нет | Нет | Нет |
| `TokioOwnedSemaphorePermit` | Да | Нет | Нет | Нет | Нет |
### Модуль `server_runtime::metrics_layer`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `HttpMetricsPathCacheMaximum` | Нет | Нет | Нет | Нет | Нет |
| `HttpMetricsPathText` | Да | Нет | Да | Нет | Нет |
| `HttpMetricsPathTextRef` | Да | Нет | Нет | Нет | Нет |
| `MetricsResponseBody` | Да | Нет | Да | Нет | Нет |
| `MetricsSharedString` | Нет | Нет | Нет | Нет | Нет |
| `StdHttpMetricsPathEntries` | Нет | Нет | Да | Нет | Нет |
| `StdSharedHttpMetricsPathCache` | Да | Нет | Нет | Нет | Нет |
### Модуль `server_runtime::multipart`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `FileStagingDirectoryName` | Да | Нет | Да | Нет | Нет |
| `MultipartBytes` | Да | Нет | Да | Нет | Нет |
| `MultipartBytesParts` | Нет | Нет | Да | Нет | Нет |
| `MultipartFieldName` | Да | Нет | Да | Нет | Нет |
| `MultipartFileName` | Да | Нет | Да | Нет | Нет |
| `MultipartPayloadMaximum` | Да | Нет | Нет | Нет | Нет |
| `MultipartTextParts` | Нет | Нет | Да | Нет | Нет |
| `MultipartTextValue` | Да | Нет | Да | Нет | Нет |
| `MultipartValueLength` | Нет | Нет | Нет | Нет | Нет |
| `StdStorageRelativePath` | Нет | Нет | Да | Нет | Нет |
| `StoragePathSegment` | Да | Нет | Да | Нет | Нет |
### Модуль `server_runtime::notification`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `AxumNotificationJson` | Нет | Нет | Нет | Нет | Нет |
| `AxumNotificationRouter` | Нет | Нет | Нет | Нет | Нет |
| `HttpNotificationHeaderMap` | Нет | Нет | Нет | Нет | Нет |
| `NotificationApiToken` | Да | Нет | Да | Нет | Нет |
| `NotificationApiTokenAuthorized` | Нет | Нет | Нет | Нет | Нет |
| `NotificationApiTokenRef` | Да | Нет | Нет | Нет | Нет |
| `NotificationMessage` | Да | Да | Да | Да | Нет |
### Модуль `server_runtime::origin`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `AllowOriginSuffix` | Да | Нет | Нет | Нет | Нет |
| `AllowedOrigins` | Да | Нет | Да | Нет | Нет |
| `HttpOriginAuthorityText` | Да | Нет | Да | Нет | Нет |
| `HttpOriginHeadersRef` | Да | Нет | Нет | Нет | Нет |
| `HttpOriginSchemeText` | Да | Нет | Да | Нет | Нет |
| `HttpOriginTextRef` | Да | Нет | Нет | Нет | Нет |
| `RequestOriginAllowed` | Нет | Нет | Нет | Нет | Нет |
### Модуль `server_runtime::outbound_url`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `OutboundAllowedHost` | Да | Нет | Да | Нет | Нет |
| `OutboundHostAllowlist` | Да | Нет | Да | Нет | Нет |
| `OutboundUrlTextRef` | Да | Нет | Нет | Нет | Нет |
| `ReqwestOutboundUrl` | Нет | Нет | Нет | Нет | Нет |
| `StdOutboundIpAddr` | Нет | Нет | Нет | Нет | Нет |
### Модуль `server_runtime::path_policy`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `HttpAllowedPathPrefixRef` | Да | Нет | Нет | Нет | Нет |
| `HttpNormalizedPath` | Да | Нет | Да | Нет | Нет |
| `HttpProxyPath` | Да | Нет | Да | Нет | Нет |
| `HttpProxyPathPrefixMatch` | Нет | Нет | Нет | Нет | Нет |
| `HttpProxyPathRef` | Да | Нет | Нет | Нет | Нет |
| `HttpRequestPathRef` | Да | Нет | Нет | Нет | Нет |
### Модуль `server_runtime::pg_rate_limit`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `PgRateLimitMaximum` | Да | Нет | Нет | Нет | Нет |
| `PgRateLimitQueryRef` | Да | Нет | Нет | Нет | Нет |
| `PgRateLimitScopeRef` | Да | Нет | Нет | Нет | Нет |
| `PgRateLimitSubjectRef` | Да | Нет | Нет | Нет | Нет |
| `PgRateLimitWindowSeconds` | Да | Нет | Нет | Нет | Нет |
| `SqlxPgRateLimitError` | Нет | Нет | Нет | Нет | Нет |
| `SqlxPgRateLimitPoolRef` | Да | Нет | Нет | Нет | Нет |
### Модуль `server_runtime::redacted_url`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `RedactedUrl` | Нет | Нет | Нет | Нет | Нет |
| `RedactedUrlTextRef` | Да | Нет | Нет | Нет | Нет |
### Модуль `server_runtime::request_id`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `HttpHeaderToStrError` | Нет | Нет | Нет | Нет | Нет |
| `RequestId` | Да | Нет | Да | Нет | Нет |
### Модуль `server_runtime::resource_budget`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `ResourceBudgetAmount` | Да | Нет | Нет | Нет | Нет |
| `ResourceBudgetMaximum` | Да | Нет | Нет | Нет | Нет |
| `StdAtomicUsize` | Нет | Нет | Нет | Нет | Нет |
| `StdSharedAtomicUsize` | Нет | Нет | Нет | Нет | Нет |
### Модуль `server_runtime::resource_utilization`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `ResourceAmount` | Да | Нет | Нет | Нет | Нет |
| `ResourceUtilizationPercent` | Нет | Нет | Нет | Нет | Нет |
### Модуль `server_runtime::retry`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `StdRetryAttempts` | Нет | Нет | Нет | Нет | Нет |
| `StdRetryDelay` | Да | Нет | Нет | Нет | Нет |
### Модуль `server_runtime::secret_text`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `BoundedSecretText` | Да | Нет | Да | Нет | Нет |
| `SecretTextRef` | Да | Нет | Нет | Нет | Нет |
### Модуль `server_runtime::secure_cookie`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `HttpCookieName` | Да | Нет | Да | Нет | Нет |
| `HttpCookieValue` | Да | Нет | Да | Нет | Нет |
| `HttpSetCookieHeaderValue` | Нет | Нет | Нет | Нет | Нет |
| `StdCookieMaxAgeSeconds` | Да | Нет | Нет | Нет | Нет |
### Модуль `server_runtime::service_bootstrap`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `StdServiceRuntimeIoError` | Нет | Нет | Нет | Нет | Нет |
| `TokioServiceRuntime` | Нет | Нет | Нет | Нет | Нет |
| `TracingSubscriberInitError` | Нет | Нет | Нет | Нет | Нет |
### Модуль `server_runtime::single_flight`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `SingleFlightKey` | Да | Нет | Да | Нет | Нет |
| `SingleFlightWaiter` | Нет | Нет | Нет | Нет | Нет |
| `StdArcStdSingleFlightRwLock` | Нет | Нет | Нет | Нет | Нет |
| `StdSingleFlightMaximum` | Да | Нет | Нет | Нет | Нет |
| `StdSingleFlightRwLock` | Нет | Нет | Нет | Нет | Нет |
| `StdSingleFlightWriteGuard` | Нет | Нет | Нет | Нет | Нет |
| `TokioSingleFlightReceiver` | Нет | Нет | Нет | Нет | Нет |
| `TokioSingleFlightSender` | Нет | Нет | Нет | Нет | Нет |
### Модуль `server_runtime::trace_context`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `HttpTraceParent` | Да | Нет | Да | Нет | Нет |
| `HttpTraceState` | Да | Нет | Да | Нет | Нет |
| `ReqwestRequestBuilder` | Да | Нет | Нет | Нет | Нет |
## Crate `str_constants_macros`

### Модуль `str_constants_macros`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `SynIdent` | Нет | Нет | Нет | Нет | Нет |
| `SynLitStr` | Нет | Нет | Нет | Нет | Нет |
| `SynVisibility` | Нет | Нет | Нет | Нет | Нет |
## Crate `synchronization_service_runtime`

### Модуль `synchronization_service_runtime`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `SynchronizationPayload` | Да | Нет | Да | Нет | Да |
## Crate `tests`

### Модуль `tests::code_style::types`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `AnalyzerBool` | Нет | Нет | Нет | Нет | Нет |
| `AnalyzerChar` | Да | Нет | Нет | Нет | Нет |
| `AnalyzerCount` | Нет | Нет | Нет | Нет | Нет |
| `CargoMetadata` | Да | Нет | Нет | Нет | Нет |
| `CargoMetadataRef` | Да | Нет | Нет | Нет | Нет |
| `CargoTomlFileIdx` | Да | Нет | Нет | Нет | Нет |
| `DiagnosticMsgs` | Нет | Нет | Да | Нет | Нет |
| `DiagnosticMsgsMutRef` | Да | Нет | Нет | Нет | Нет |
| `SourceText` | Да | Нет | Да | Нет | Нет |
| `SourceTextList` | Нет | Нет | Да | Нет | Да |
| `SourceTextListRef` | Да | Нет | Нет | Нет | Нет |
| `SourceTextRef` | Да | Нет | Нет | Нет | Нет |
| `StaticStr` | Нет | Нет | Нет | Нет | Нет |
| `StaticStrSliceRef` | Да | Нет | Нет | Нет | Нет |
| `StdCargoPackageIdRefSet` | Да | Нет | Да | Нет | Да |
| `StdPathBuf` | Да | Нет | Да | Нет | Да |
| `StdPathRef` | Да | Нет | Нет | Нет | Нет |
| `StdProcessOutputRef` | Да | Нет | Нет | Нет | Нет |
| `StdSourceTextHashSet` | Да | Нет | Да | Нет | Да |
| `StdSourceTextRefSet` | Да | Нет | Нет | Нет | Нет |
| `StdSourceTextSet` | Нет | Нет | Да | Нет | Да |
| `StdStdSourceTextSetRef` | Да | Нет | Нет | Нет | Нет |
| `SynAttributeListRef` | Да | Нет | Нет | Нет | Нет |
| `SynAttributeRef` | Да | Нет | Нет | Нет | Нет |
| `SynBlockRef` | Да | Нет | Нет | Нет | Нет |
| `SynExprCallRef` | Да | Нет | Нет | Нет | Нет |
| `SynFieldsRef` | Да | Нет | Нет | Нет | Нет |
| `SynFile` | Да | Нет | Нет | Нет | Нет |
| `SynFileRef` | Да | Нет | Нет | Нет | Нет |
| `SynGenericsRef` | Да | Нет | Нет | Нет | Нет |
| `SynIdentifierRef` | Да | Нет | Нет | Нет | Нет |
| `SynItemFnRef` | Да | Нет | Нет | Нет | Нет |
| `SynItemImplRef` | Да | Нет | Нет | Нет | Нет |
| `SynItemRef` | Да | Нет | Нет | Нет | Нет |
| `SynItemStructRef` | Да | Нет | Нет | Нет | Нет |
| `SynPathArgumentsRef` | Да | Нет | Нет | Нет | Нет |
| `SynPathRef` | Да | Нет | Нет | Нет | Нет |
| `SynPathSegmentRef` | Да | Нет | Нет | Нет | Нет |
| `SynSignatureRef` | Да | Нет | Нет | Нет | Нет |
| `SynTypePathRef` | Да | Нет | Нет | Нет | Нет |
| `SynTypeRef` | Да | Нет | Нет | Нет | Нет |
| `SynUseTreeRef` | Да | Нет | Нет | Нет | Нет |
| `TomlTable` | Да | Нет | Нет | Нет | Нет |
| `TomlTableRef` | Да | Нет | Нет | Нет | Нет |
| `TomlValue` | Да | Нет | Да | Нет | Да |
| `TomlValueRef` | Да | Нет | Нет | Нет | Нет |
| `WalkdirWalkDir` | Да | Нет | Нет | Нет | Нет |
### Модуль `tests::domain_type_policy_fixture`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `DomainEvents` | Нет | Нет | Да | Нет | Нет |
| `DomainId` | Нет | Нет | Нет | Нет | Нет |
| `DomainName` | Да | Нет | Да | Нет | Да |
## Crate `text_policy`

### Модуль `text_policy`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `FixedLengthAsciiHexText` | Да | Нет | Да | Нет | Нет |
| `NonEmptyTrimmedText` | Да | Нет | Да | Нет | Нет |
| `PasswordLength` | Да | Нет | Нет | Нет | Нет |
| `PasswordTextRef` | Да | Нет | Нет | Нет | Нет |
| `RequiredNulFreeBoundedText` | Да | Нет | Да | Нет | Нет |
| `UrlSafeTokenPartMaximumBytes` | Да | Нет | Нет | Нет | Нет |
| `UrlSafeTokenPartRef` | Да | Нет | Нет | Нет | Нет |
| `UrlSafeTokenPartText` | Да | Нет | Да | Нет | Нет |
## Crate `to_err_string`

### Модуль `to_err_string`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `StaticStrToOwnedInput` | Нет | Нет | Нет | Нет | Нет |
| `ToErrStringValue` | Да | Нет | Да | Нет | Нет |
## Crate `token_patterns`

### Модуль `token_patterns`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `ProcMacro2TokensMut` | Нет | Нет | Нет | Нет | Нет |
## Crate `token_patterns_macros`

### Модуль `token_patterns_macros`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `ProcMacro2GenerateTpInput` | Нет | Нет | Да | Нет | Нет |
| `ProcMacro2GenerateTpOutput` | Нет | Нет | Да | Нет | Нет |
## Crate `where_filters`

### Модуль `where_filters`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `BoundedVec` | Нет | Да | Да | Да | Нет |
| `BoundedVecLen` | Да | Да | Нет | Нет | Нет |
| `PgTypeNotEmptyUniqueVec` | Нет | Нет | Да | Нет | Нет |
| `RegexCasePostgreqlSyntax` | Да | Нет | Нет | Нет | Нет |
| `RegexRegex` | Нет | Да | Нет | Нет | Нет |
### Модуль `where_filters::tests`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `NonClone` | Нет | Нет | Нет | Нет | Нет |
## Crate `workspace_macro_helpers`

### Модуль `workspace_macro_helpers`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `FirstCommaStripped` | Нет | Нет | Нет | Нет | Нет |
| `FirstIdentifier` | Да | Нет | Да | Нет | Да |
| `FirstIdentifierifierTryFromStringError` | Нет | Нет | Нет | Нет | Нет |
| `PartIndex` | Да | Нет | Нет | Нет | Нет |
| `ProcMacro2MacroTokens` | Нет | Нет | Да | Нет | Да |
| `ProcMacro2TopLevelCommaParts` | Нет | Нет | Да | Нет | Нет |
| `StdUniqueOptionSet` | Нет | Нет | Да | Нет | Нет |
| `StdUniqueOptionSetContains` | Нет | Нет | Нет | Нет | Нет |
| `StdUniqueOptionSetIsEmpty` | Нет | Нет | Нет | Нет | Нет |
| `SynDeriveInputRef` | Да | Нет | Нет | Нет | Нет |
| `SynFieldsNamedRef` | Нет | Нет | Нет | Нет | Нет |
| `SynFieldsUnnamedRef` | Нет | Нет | Нет | Нет | Нет |
| `TopLevelCommaPart` | Нет | Нет | Нет | Нет | Нет |
## Crate `workspace_scaffold`

### Модуль `workspace_scaffold`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `ProjectNameRef` | Нет | Нет | Нет | Нет | Нет |
| `RepositoryUrlRef` | Нет | Нет | Нет | Нет | Нет |
| `ServicePort` | Нет | Нет | Нет | Нет | Нет |
## Crate `workspace_test_runner`

### Модуль `workspace_test_runner`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `AnsiTextRef` | Нет | Нет | Нет | Нет | Нет |
| `CargoArgs` | Нет | Нет | Нет | Нет | Нет |
| `CleanAnsiText` | Нет | Нет | Да | Нет | Нет |
| `MeasurementName` | Нет | Нет | Нет | Нет | Нет |
| `MemusageColumnIdx` | Нет | Нет | Нет | Нет | Нет |
| `MemusageKey` | Нет | Нет | Нет | Нет | Нет |
| `MemusageProgNameRef` | Нет | Нет | Нет | Нет | Нет |
| `MemusageRowName` | Нет | Нет | Нет | Нет | Нет |
| `MemusageValueRef` | Нет | Нет | Нет | Нет | Нет |
| `ProgramArgsRef` | Нет | Нет | Нет | Нет | Нет |
| `ProgramPathRef` | Нет | Нет | Нет | Нет | Нет |
| `QuoteTokenStreamGeneratePgTableMeasureInputTokenStream` | Нет | Нет | Да | Нет | Нет |
| `StderrTextRef` | Да | Нет | Нет | Нет | Нет |
| `ToolName` | Нет | Нет | Нет | Нет | Нет |
| `ToolPath` | Нет | Нет | Нет | Нет | Нет |
### Модуль `workspace_test_runner::execution`

| Тип | I | D | B | DT | FT |
|---|:---:|:---:|:---:|:---:|:---:|
| `CommandIdx` | Нет | Нет | Нет | Нет | Нет |
| `CommandStartedAt` | Нет | Нет | Нет | Нет | Нет |
| `RunDir` | Нет | Нет | Да | Нет | Нет |
| `SummaryText` | Нет | Нет | Да | Нет | Нет |
