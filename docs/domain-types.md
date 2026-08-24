# Domain type declarations

This generated catalog lists every non-test-only struct, enum, trait, and union declaration recognized by the repository domain-type policy. It also includes domain types declared by the macros explicitly recognized by that policy.

Source files remain authoritative. Regenerate this catalog whenever domain declarations change.

- Direct declarations: 2061
- Macro-generated declarations: 116
- Total declarations: 2177

## `admin_bootstrap/src/main.rs`

- [`struct BootstrapPathBuf`](../admin_bootstrap/src/main.rs#L13)
- [`struct SqlxBootstrapError`](../admin_bootstrap/src/main.rs#L18)
- [`struct BootstrapStatus`](../admin_bootstrap/src/main.rs#L28)
- [`struct BootstrapArgs`](../admin_bootstrap/src/main.rs#L30)
- [`struct PasswordResetArgs`](../admin_bootstrap/src/main.rs#L36)
- [`enum AdminCommand`](../admin_bootstrap/src/main.rs#L41)
- [`enum BootstrapArgsError`](../admin_bootstrap/src/main.rs#L46)
- [`enum BootstrapCommandError`](../admin_bootstrap/src/main.rs#L57)
- [`struct BootstrapExitCode`](../admin_bootstrap/src/main.rs#L78)

## `app_state/src/domain_types.rs`

- [`struct SqlxPgPoolRef`](../app_state/src/domain_types.rs#L9)
- [`struct SqlxPgPool`](../app_state/src/domain_types.rs#L18)
- [`trait GetSqlxPgPool`](../app_state/src/domain_types.rs#L20)

## `bounded_types/src/domain_types.rs`

- [`struct BoundedLen`](../bounded_types/src/domain_types.rs#L24)
- [`enum BoundedValueError`](../bounded_types/src/domain_types.rs#L34)

## `bounded_types/src/domain_types/btree.rs`

- [`struct BoundedBTreeMap`](../bounded_types/src/domain_types/btree.rs#L5)
- [`struct BoundedBTreeMapVisitorPhantomData`](../bounded_types/src/domain_types/btree.rs#L123)

## `bounded_types/src/domain_types/hash.rs`

- [`struct BoundedHashMap`](../bounded_types/src/domain_types/hash.rs#L5)
- [`struct BoundedHashMapVisitorPhantomData`](../bounded_types/src/domain_types/hash.rs#L124)

## `bounded_types/src/domain_types/text.rs`

- [`struct BoundedString`](../bounded_types/src/domain_types/text.rs#L12)

## `bounded_types/src/domain_types/vector.rs`

- [`struct BoundedVec`](../bounded_types/src/domain_types/vector.rs#L10)
- [`struct BoundedVecVisitorPhantomData`](../bounded_types/src/domain_types/vector.rs#L111)

## `common_routes/src/lib.rs`

- [`struct GitInfo`](../common_routes/src/lib.rs#L15)
- [`struct NotFoundHandle`](../common_routes/src/lib.rs#L20)
- [`struct OpenApiSpecificationPath`](../common_routes/src/lib.rs#L33)
- [`struct AxumHttpUriRef`](../common_routes/src/lib.rs#L35)
- [`struct UriSuffixRef`](../common_routes/src/lib.rs#L37)
- [`struct NoRouteMessageCapacity`](../common_routes/src/lib.rs#L47)
- [`struct HealthCheckSucceeded`](../common_routes/src/lib.rs#L49)
- [`struct HealthDatabaseAvailable`](../common_routes/src/lib.rs#L59)
- [`enum HealthStatus`](../common_routes/src/lib.rs#L72)
- [`enum HealthComponentKind`](../common_routes/src/lib.rs#L89)
- [`struct HealthComponent`](../common_routes/src/lib.rs#L104)
- [`struct HealthComponents`](../common_routes/src/lib.rs#L111)
- [`struct HealthComponentsError`](../common_routes/src/lib.rs#L164)
- [`struct HealthReport`](../common_routes/src/lib.rs#L175)
- [`struct AxumHealthCheckStatus`](../common_routes/src/lib.rs#L230)
- [`struct JsonRes`](../common_routes/src/lib.rs#L237)
- [`enum CommonNotFoundError`](../common_routes/src/lib.rs#L241)
- [`enum HealthCheckError`](../common_routes/src/lib.rs#L246)
- [`enum HealthError`](../common_routes/src/lib.rs#L251)
- [`enum HealthLiveError`](../common_routes/src/lib.rs#L256)
- [`enum HealthReadyError`](../common_routes/src/lib.rs#L261)
- [`struct AxumJsonPayload`](../common_routes/src/lib.rs#L266)
- [`struct CommonNoBody`](../common_routes/src/lib.rs#L276)
- [`struct HealthLiveRoute`](../common_routes/src/lib.rs#L294)
- [`struct HealthReadyRoute`](../common_routes/src/lib.rs#L313)
- [`struct HealthRoute`](../common_routes/src/lib.rs#L332)
- [`struct HealthCheckRoute`](../common_routes/src/lib.rs#L351)
- [`struct GitInfoRoute`](../common_routes/src/lib.rs#L369)
- [`enum CommonRoute`](../common_routes/src/lib.rs#L381)
- [`struct AxumCommonRoutes`](../common_routes/src/lib.rs#L467)
- [`struct ArcCommonRoutesAppState`](../common_routes/src/lib.rs#L469)
- [`struct CommonRoutesOpenApi`](../common_routes/src/lib.rs#L471)
- [`struct UtoipaCommonRoutesOpenApiDocument`](../common_routes/src/lib.rs#L475)
- [`trait CommonRoutesParameters`](../common_routes/src/lib.rs#L512)
- [`struct CommonRouteRegistry`](../common_routes/src/lib.rs#L636)

## `common_routes/src/tests.rs`

- [`struct TestState`](../common_routes/src/tests.rs#L2)

## `common_routes/src/tests/route_contract.rs`

- [`struct ClientTransport`](../common_routes/src/tests/route_contract.rs#L2)

## `config_lib/src/admin.rs`

- [`struct AdminAccessTokenTtlSeconds`](../config_lib/src/admin.rs#L12)
- [`struct AdminRefreshTokenTtlSeconds`](../config_lib/src/admin.rs#L24)
- [`struct AdminLoginFailureLimit`](../config_lib/src/admin.rs#L36)
- [`struct AdminSignInRateLimit`](../config_lib/src/admin.rs#L48)
- [`struct AdminSessionLimit`](../config_lib/src/admin.rs#L60)
- [`struct AdminPositiveU64ParsingError`](../config_lib/src/admin.rs#L64)
- [`enum TryFromStdEnvVarOkAdminPositiveU64Error`](../config_lib/src/admin.rs#L66)
- [`struct AdminPasswordHashConcurrency`](../config_lib/src/admin.rs#L135)
- [`struct AdminPositiveUsizeParsingError`](../config_lib/src/admin.rs#L139)
- [`enum TryFromStdEnvVarOkAdminPasswordHashConcurrencyError`](../config_lib/src/admin.rs#L141)
- [`struct AdminTokenIssuer`](../config_lib/src/admin.rs#L179)
- [`struct AdminTokenAudience`](../config_lib/src/admin.rs#L194)
- [`enum TryFromStdEnvVarOkAdminTokenTextError`](../config_lib/src/admin.rs#L198)

## `config_lib/src/admin_jwt.rs`

- [`struct AdminJwtSecret`](../config_lib/src/admin_jwt.rs#L11)
- [`enum TryFromStdEnvVarOkAdminJwtSecretError`](../config_lib/src/admin_jwt.rs#L34)

## `config_lib/src/bool_flags.rs`

- [`struct AdminCookieSecure`](../config_lib/src/bool_flags.rs#L12)
- [`struct AdminSwaggerEnabled`](../config_lib/src/bool_flags.rs#L24)
- [`struct HttpGzipEnabled`](../config_lib/src/bool_flags.rs#L35)
- [`struct ProductionMode`](../config_lib/src/bool_flags.rs#L46)
- [`struct AdminBoolParsingError`](../config_lib/src/bool_flags.rs#L50)
- [`struct TryFromStdEnvVarOkAdminCookieSecureError`](../config_lib/src/bool_flags.rs#L54)

## `config_lib/src/http.rs`

- [`struct MaximumSizeOfHttpBodyInBytes`](../config_lib/src/http.rs#L9)
- [`enum MaximumSizeOfHttpBodyInBytesTryFromUsizeError`](../config_lib/src/http.rs#L13)
- [`enum TryFromStdEnvVarOkMaximumSizeOfHttpBodyInBytesError`](../config_lib/src/http.rs#L28)
- [`struct ContentSecurityPolicy`](../config_lib/src/http.rs#L57)
- [`enum ContentSecurityPolicyError`](../config_lib/src/http.rs#L61)

## `config_lib/src/lib.rs`

- [`struct StdEnvVarOk`](../config_lib/src/lib.rs#L38)
- [`enum ConfigLibStringWrapperTryFromStringError`](../config_lib/src/lib.rs#L42)
- [`struct StdEnvVarOkRef`](../config_lib/src/lib.rs#L64)
- [`struct EnvVarNameRef`](../config_lib/src/lib.rs#L73)
- [`struct EnvVarName`](../config_lib/src/lib.rs#L77)
- [`struct ChronoFixedOffsetError`](../config_lib/src/lib.rs#L104)
- [`struct I32ParseIntError`](../config_lib/src/lib.rs#L108)
- [`struct U32ParseIntError`](../config_lib/src/lib.rs#L112)
- [`struct UsizeParseIntError`](../config_lib/src/lib.rs#L116)
- [`struct TimezoneSeconds`](../config_lib/src/lib.rs#L126)
- [`struct ChronoEastFixedOffset`](../config_lib/src/lib.rs#L136)
- [`trait TryFromStdEnvVarOk`](../config_lib/src/lib.rs#L137)
- [`enum ConfigFieldSensitivity`](../config_lib/src/lib.rs#L142)
- [`enum ConfigFieldRequirement`](../config_lib/src/lib.rs#L147)
- [`enum ConfigExampleValidity`](../config_lib/src/lib.rs#L151)
- [`struct ConfigFieldExampleRef`](../config_lib/src/lib.rs#L163)
- [`struct ConfigRustTypeName`](../config_lib/src/lib.rs#L172)
- [`struct ConfigFieldDescriptor`](../config_lib/src/lib.rs#L174)
- [`struct StdConfigSecretString`](../config_lib/src/lib.rs#L248)
- [`struct SecrecySecretBoxString`](../config_lib/src/lib.rs#L255)
- [`struct ConfigNonZeroU64`](../config_lib/src/lib.rs#L278)
- [`struct ConfigNonZeroUsize`](../config_lib/src/lib.rs#L289)
- [`struct ParseIntError`](../config_lib/src/lib.rs#L293)
- [`struct ParseBoolError`](../config_lib/src/lib.rs#L297)
- [`macro-generated type CorsAllowOrigin`](../config_lib/src/lib.rs#L298)
- [`macro-generated type TrustedProxyRangesText`](../config_lib/src/lib.rs#L302)
- [`macro-generated type DatabaseUrl`](../config_lib/src/lib.rs#L306)
- [`macro-generated type EnableApiGitCommitCheck`](../config_lib/src/lib.rs#L307)
- [`macro-generated type MongoUrl`](../config_lib/src/lib.rs#L317)
- [`macro-generated type RedisUrl`](../config_lib/src/lib.rs#L319)
- [`macro-generated type ServiceSocketAddress`](../config_lib/src/lib.rs#L320)
- [`macro-generated type SrcPlaceType`](../config_lib/src/lib.rs#L330)
- [`macro-generated type StartingCheckLink`](../config_lib/src/lib.rs#L337)
- [`struct ChronoTimezone`](../config_lib/src/lib.rs#L349)
- [`enum TryFromStdEnvVarOkTimezoneError`](../config_lib/src/lib.rs#L362)
- [`enum TryFromStdEnvVarOkSvcModeError`](../config_lib/src/lib.rs#L399)
- [`macro-generated type TracingLevel`](../config_lib/src/lib.rs#L413)

## `config_lib/src/pg_pool.rs`

- [`struct PgPoolMaxConnections`](../config_lib/src/pg_pool.rs#L9)
- [`struct PgPoolMinConnections`](../config_lib/src/pg_pool.rs#L18)
- [`struct PgPoolAcquireTimeoutSeconds`](../config_lib/src/pg_pool.rs#L27)
- [`struct PgPoolIdleTimeoutSeconds`](../config_lib/src/pg_pool.rs#L36)
- [`struct PgPoolMaxLifetimeSeconds`](../config_lib/src/pg_pool.rs#L45)
- [`struct RequestTimeoutSeconds`](../config_lib/src/pg_pool.rs#L54)
- [`enum PgPoolConfigParseError`](../config_lib/src/pg_pool.rs#L58)
- [`enum PgPoolMaxConnectionsTryFromU32Error`](../config_lib/src/pg_pool.rs#L109)
- [`enum TryFromStdEnvVarOkPgPoolMaxConnectionsError`](../config_lib/src/pg_pool.rs#L124)

## `config_lib/src/types.rs`

- [`struct TracingLevelName`](../config_lib/src/types.rs#L11)
- [`struct EnvVarResultVarError`](../config_lib/src/types.rs#L13)
- [`struct EnvVarError`](../config_lib/src/types.rs#L23)
- [`struct EnvVarNameRef`](../config_lib/src/types.rs#L37)
- [`struct EnvVarValueRef`](../config_lib/src/types.rs#L39)
- [`struct ParseCtxRef`](../config_lib/src/types.rs#L50)
- [`enum EnvParseError`](../config_lib/src/types.rs#L52)
- [`enum TracingLevel`](../config_lib/src/types.rs#L90)
- [`enum TracingFormat`](../config_lib/src/types.rs#L101)
- [`enum SvcMode`](../config_lib/src/types.rs#L109)
- [`enum SrcPlaceType`](../config_lib/src/types.rs#L145)

## `config_lib_config_lib_macros/src/domain_types.rs`

- [`struct ProcMacro2TryFromParseInput`](../config_lib_config_lib_macros/src/domain_types.rs#L4)
- [`struct ProcMacro2TryFromParseFixedErrorTy`](../config_lib_config_lib_macros/src/domain_types.rs#L9)
- [`struct ProcMacroTryFromParseTokenStream`](../config_lib_config_lib_macros/src/domain_types.rs#L14)

## `constants_str_macros/src/lib.rs`

- [`struct SynIdent`](../constants_str_macros/src/lib.rs#L8)
- [`struct SynLitStr`](../constants_str_macros/src/lib.rs#L22)
- [`struct SynVisibility`](../constants_str_macros/src/lib.rs#L36)
- [`struct Fragment`](../constants_str_macros/src/lib.rs#L50)
- [`enum ConstantPart`](../constants_str_macros/src/lib.rs#L56)
- [`struct ConstantParts`](../constants_str_macros/src/lib.rs#L61)
- [`struct Constants`](../constants_str_macros/src/lib.rs#L76)
- [`struct Fragments`](../constants_str_macros/src/lib.rs#L91)
- [`struct Constant`](../constants_str_macros/src/lib.rs#L107)
- [`struct DefineStrConstantsInput`](../constants_str_macros/src/lib.rs#L114)

## `development_data_bootstrap/src/domain_types.rs`

- [`struct DevelopmentIdentitySpecs`](../development_data_bootstrap/src/domain_types.rs#L15)
- [`struct DevelopmentIdentitySpecsError`](../development_data_bootstrap/src/domain_types.rs#L38)
- [`struct DevelopmentBootstrapPlan`](../development_data_bootstrap/src/domain_types.rs#L41)
- [`struct DevelopmentBootstrapSummary`](../development_data_bootstrap/src/domain_types.rs#L66)
- [`struct DevelopmentIdentityCount`](../development_data_bootstrap/src/domain_types.rs#L83)

## `external_service_emulators/src/domain_types.rs`

- [`struct MockNotificationProvider`](../external_service_emulators/src/domain_types.rs#L2)
- [`struct MockNotificationInbox`](../external_service_emulators/src/domain_types.rs#L7)
- [`struct TokioMockNotificationSender`](../external_service_emulators/src/domain_types.rs#L12)
- [`struct TokioMockNotificationReceiver`](../external_service_emulators/src/domain_types.rs#L17)
- [`struct MockNotificationProviderClosed`](../external_service_emulators/src/domain_types.rs#L25)
- [`struct RemoteSyncRequestCount`](../external_service_emulators/src/domain_types.rs#L59)
- [`struct RemoteSyncSource`](../external_service_emulators/src/domain_types.rs#L62)

## `file_storage/src/domain_types.rs`

- [`struct FileStorageIoError`](../file_storage/src/domain_types.rs#L9)
- [`struct StoragePathRef`](../file_storage/src/domain_types.rs#L11)
- [`struct StorageDirectoryNameRef`](../file_storage/src/domain_types.rs#L14)
- [`struct FileStorageRootPathBuf`](../file_storage/src/domain_types.rs#L17)
- [`struct StorageRelativePathBuf`](../file_storage/src/domain_types.rs#L33)
- [`struct StdStorageOperationId`](../file_storage/src/domain_types.rs#L53)
- [`struct StdFileBytes`](../file_storage/src/domain_types.rs#L72)
- [`enum FileStoragePathError`](../file_storage/src/domain_types.rs#L86)
- [`enum FileStorageError`](../file_storage/src/domain_types.rs#L100)
- [`struct SafeFileStorage`](../file_storage/src/domain_types.rs#L119)
- [`enum FileStorageStagingArea`](../file_storage/src/domain_types.rs#L124)
- [`struct StdStaleStagingEntryLimit`](../file_storage/src/domain_types.rs#L142)
- [`struct StaleBeforeSystemTime`](../file_storage/src/domain_types.rs#L163)
- [`struct StaleStagingCleanupCfg`](../file_storage/src/domain_types.rs#L166)
- [`struct StaleStagingCleanupCfgError`](../file_storage/src/domain_types.rs#L190)
- [`struct StdStaleStagingEntryCount`](../file_storage/src/domain_types.rs#L204)
- [`struct StaleStagingCleanupReport`](../file_storage/src/domain_types.rs#L209)
- [`enum AtomicReplaceDurability`](../file_storage/src/domain_types.rs#L536)
- [`struct StdDiskCacheSize`](../file_storage/src/domain_types.rs#L550)
- [`struct DiskCacheModifiedAtSystemTime`](../file_storage/src/domain_types.rs#L561)
- [`struct DiskCacheEntry`](../file_storage/src/domain_types.rs#L564)
- [`struct DiskCacheEvictionPlan`](../file_storage/src/domain_types.rs#L594)
- [`enum DiskCacheBudgetError`](../file_storage/src/domain_types.rs#L601)

## `frontend_contract/src/auth_session_keep_alive.rs`

- [`struct AuthSessionInstant`](../frontend_contract/src/auth_session_keep_alive.rs#L10)
- [`struct AuthSessionRefreshIntervalDuration`](../frontend_contract/src/auth_session_keep_alive.rs#L13)
- [`enum AuthSessionPresence`](../frontend_contract/src/auth_session_keep_alive.rs#L26)
- [`enum AuthSessionRefreshOutcome`](../frontend_contract/src/auth_session_keep_alive.rs#L32)
- [`enum AuthSessionKeepAliveDecision`](../frontend_contract/src/auth_session_keep_alive.rs#L39)
- [`enum AuthSessionKeepAliveError`](../frontend_contract/src/auth_session_keep_alive.rs#L49)
- [`enum AuthSessionRefreshState`](../frontend_contract/src/auth_session_keep_alive.rs#L55)
- [`struct AuthSessionKeepAlive`](../frontend_contract/src/auth_session_keep_alive.rs#L61)

## `frontend_contract/src/client.rs`

- [`struct TypedClient`](../frontend_contract/src/client.rs#L2)

## `frontend_contract/src/handler_contract.rs`

- [`struct HandlerPath`](../frontend_contract/src/handler_contract.rs#L11)
- [`trait HandlerContract`](../frontend_contract/src/handler_contract.rs#L13)
- [`struct AxumHandlerMethodRouter`](../frontend_contract/src/handler_contract.rs#L21)

## `frontend_contract/src/lib.rs`

- [`struct FrontendContractBodyError`](../frontend_contract/src/lib.rs#L7)
- [`struct HttpStatusTryFromU16Error`](../frontend_contract/src/lib.rs#L17)
- [`enum KnownHttpStatus`](../frontend_contract/src/lib.rs#L19)
- [`struct ContractStr`](../frontend_contract/src/lib.rs#L123)
- [`enum InputKind`](../frontend_contract/src/lib.rs#L130)
- [`enum ValueFormat`](../frontend_contract/src/lib.rs#L140)
- [`enum Nullability`](../frontend_contract/src/lib.rs#L161)
- [`enum CapabilitySupport`](../frontend_contract/src/lib.rs#L166)
- [`enum FilterOperation`](../frontend_contract/src/lib.rs#L182)
- [`enum FilterValueShape`](../frontend_contract/src/lib.rs#L220)
- [`struct FilterContracts`](../frontend_contract/src/lib.rs#L269)
- [`trait HasFilterContracts`](../frontend_contract/src/lib.rs#L270)
- [`enum InputStep`](../frontend_contract/src/lib.rs#L278)
- [`enum NumericBound`](../frontend_contract/src/lib.rs#L284)
- [`struct ContractI64`](../frontend_contract/src/lib.rs#L297)
- [`enum ValueExample`](../frontend_contract/src/lib.rs#L325)
- [`struct TypeContract`](../frontend_contract/src/lib.rs#L337)
- [`trait HasTypeContract`](../frontend_contract/src/lib.rs#L427)
- [`struct FormValue`](../frontend_contract/src/lib.rs#L441)
- [`struct FormValueRef`](../frontend_contract/src/lib.rs#L452)
- [`struct FormFieldNameRef`](../frontend_contract/src/lib.rs#L463)
- [`struct FormValueError`](../frontend_contract/src/lib.rs#L474)
- [`struct FilterWireJson`](../frontend_contract/src/lib.rs#L491)
- [`trait FormValueContract`](../frontend_contract/src/lib.rs#L492)
- [`trait FilterFormValueContract`](../frontend_contract/src/lib.rs#L496)
- [`struct FormFieldError`](../frontend_contract/src/lib.rs#L500)
- [`enum FieldCapability`](../frontend_contract/src/lib.rs#L519)
- [`enum PrimaryKeyKind`](../frontend_contract/src/lib.rs#L524)
- [`struct FieldOrder`](../frontend_contract/src/lib.rs#L537)
- [`enum FieldVisibility`](../frontend_contract/src/lib.rs#L539)
- [`enum FieldPlaceholder`](../frontend_contract/src/lib.rs#L544)
- [`struct FieldContract`](../frontend_contract/src/lib.rs#L549)
- [`struct FieldContracts`](../frontend_contract/src/lib.rs#L573)
- [`enum HttpMethod`](../frontend_contract/src/lib.rs#L716)
- [`enum SuccessStatus`](../frontend_contract/src/lib.rs#L728)
- [`enum RouteErrorStatus`](../frontend_contract/src/lib.rs#L734)
- [`enum RouteErrorPolicy`](../frontend_contract/src/lib.rs#L830)
- [`enum AuthenticationRequirement`](../frontend_contract/src/lib.rs#L881)
- [`enum MutationKind`](../frontend_contract/src/lib.rs#L887)
- [`enum OperationKind`](../frontend_contract/src/lib.rs#L892)
- [`enum ConfirmationRequirement`](../frontend_contract/src/lib.rs#L903)
- [`struct ActionContract`](../frontend_contract/src/lib.rs#L908)
- [`struct ActionContracts`](../frontend_contract/src/lib.rs#L922)
- [`struct RouteContract`](../frontend_contract/src/lib.rs#L969)
- [`struct RouteContracts`](../frontend_contract/src/lib.rs#L985)
- [`struct PageContract`](../frontend_contract/src/lib.rs#L1043)
- [`struct TransportBody`](../frontend_contract/src/lib.rs#L1053)
- [`struct TransportRequest`](../frontend_contract/src/lib.rs#L1065)
- [`struct TransportIdempotencyKey`](../frontend_contract/src/lib.rs#L1124)
- [`struct TransportIfMatch`](../frontend_contract/src/lib.rs#L1135)
- [`struct TransportPath`](../frontend_contract/src/lib.rs#L1147)
- [`struct TransportStatus`](../frontend_contract/src/lib.rs#L1163)
- [`struct TransportRetryAfter`](../frontend_contract/src/lib.rs#L1189)
- [`struct TransportResponse`](../frontend_contract/src/lib.rs#L1191)
- [`struct TransportError`](../frontend_contract/src/lib.rs#L1250)
- [`trait Transport`](../frontend_contract/src/lib.rs#L1257)
- [`enum ClientError`](../frontend_contract/src/lib.rs#L1264)

## `frontend_contract/src/problem.rs`

- [`enum ApiProblemKind`](../frontend_contract/src/problem.rs#L13)
- [`enum ApiProblemError`](../frontend_contract/src/problem.rs#L32)
- [`struct ApiProblemStatus`](../frontend_contract/src/problem.rs#L82)
- [`struct ApiProblemDetail`](../frontend_contract/src/problem.rs#L164)
- [`struct ApiProblemRequestId`](../frontend_contract/src/problem.rs#L179)
- [`struct ApiProblemField`](../frontend_contract/src/problem.rs#L194)
- [`struct ApiProblemViolation`](../frontend_contract/src/problem.rs#L205)
- [`struct ApiProblemViolations`](../frontend_contract/src/problem.rs#L223)
- [`struct ApiProblem`](../frontend_contract/src/problem.rs#L250)

## `frontend_contract/src/route.rs`

- [`trait RouteTransport`](../frontend_contract/src/route.rs#L1)
- [`struct PublicTransport`](../frontend_contract/src/route.rs#L3)
- [`struct AuthenticatedTransport`](../frontend_contract/src/route.rs#L5)
- [`enum RouteMethod`](../frontend_contract/src/route.rs#L9)
- [`struct AxumMethodFilter`](../frontend_contract/src/route.rs#L39)
- [`struct RouteMetadata`](../frontend_contract/src/route.rs#L56)
- [`struct UtoipaOpenApiComponentsRefMut`](../frontend_contract/src/route.rs#L167)
- [`struct UtoipaOpenApiRefMut`](../frontend_contract/src/route.rs#L177)
- [`trait TypedRoute`](../frontend_contract/src/route.rs#L184)
- [`enum RouteRequestBody`](../frontend_contract/src/route.rs#L220)
- [`struct RouteSchemaContract`](../frontend_contract/src/route.rs#L225)
- [`struct UtoipaOpenApiRouteSchema`](../frontend_contract/src/route.rs#L258)
- [`struct UtoipaOpenApiPathParameter`](../frontend_contract/src/route.rs#L268)
- [`struct ParameterizedRoutePath`](../frontend_contract/src/route.rs#L284)
- [`struct ParameterizedRoutePathTryFromStringError`](../frontend_contract/src/route.rs#L286)
- [`struct OpenApiSecuritySchemeRef`](../frontend_contract/src/route.rs#L306)
- [`trait CoveredRoute`](../frontend_contract/src/route.rs#L307)
- [`trait ParameterizedRoute`](../frontend_contract/src/route.rs#L310)
- [`struct RouteBodyLimit`](../frontend_contract/src/route.rs#L324)
- [`struct RouteCoverageDescriptors`](../frontend_contract/src/route.rs#L336)
- [`struct RouteSchemaContracts`](../frontend_contract/src/route.rs#L352)
- [`struct RouteMetadataList`](../frontend_contract/src/route.rs#L366)
- [`trait RouteFamily`](../frontend_contract/src/route.rs#L408)
- [`trait RouteInFamily`](../frontend_contract/src/route.rs#L429)
- [`struct RouteRequest`](../frontend_contract/src/route.rs#L435)
- [`struct RouteResponse`](../frontend_contract/src/route.rs#L455)

## `frontend_contract/src/route_coverage.rs`

- [`enum RouteAccess`](../frontend_contract/src/route_coverage.rs#L2)
- [`enum RouteMutation`](../frontend_contract/src/route_coverage.rs#L8)
- [`enum RouteDatabaseUsage`](../frontend_contract/src/route_coverage.rs#L14)
- [`enum RouteJsonBodyUsage`](../frontend_contract/src/route_coverage.rs#L20)
- [`enum RouteResponseKind`](../frontend_contract/src/route_coverage.rs#L26)
- [`struct RouteTestCapabilities`](../frontend_contract/src/route_coverage.rs#L32)
- [`enum RouteTestCategory`](../frontend_contract/src/route_coverage.rs#L54)
- [`struct RouteTestCategories`](../frontend_contract/src/route_coverage.rs#L72)
- [`struct RouteCoverageEvidence`](../frontend_contract/src/route_coverage.rs#L119)
- [`struct RouteCoverageDescriptor`](../frontend_contract/src/route_coverage.rs#L131)
- [`enum RouteCoverageObligation`](../frontend_contract/src/route_coverage.rs#L160)
- [`enum RouteCoverageError`](../frontend_contract/src/route_coverage.rs#L193)

## `frontend_contract/src/url_builder.rs`

- [`struct ApiUrlPathSegmentRef`](../frontend_contract/src/url_builder.rs#L32)
- [`struct ApiUrlQueryComponentRef`](../frontend_contract/src/url_builder.rs#L57)
- [`enum ApiUrlBuildError`](../frontend_contract/src/url_builder.rs#L63)
- [`struct ApiUrl`](../frontend_contract/src/url_builder.rs#L78)

## `frontend_contract_macros/src/lib.rs`

- [`struct SynExpr`](../frontend_contract_macros/src/lib.rs#L6)
- [`struct SynType`](../frontend_contract_macros/src/lib.rs#L9)
- [`struct SynIdent`](../frontend_contract_macros/src/lib.rs#L12)
- [`struct StdBool`](../frontend_contract_macros/src/lib.rs#L22)
- [`struct SynAttributesRef`](../frontend_contract_macros/src/lib.rs#L24)
- [`struct ContractStructApiArgs`](../frontend_contract_macros/src/lib.rs#L27)
- [`struct ContractStructApiFieldArgs`](../frontend_contract_macros/src/lib.rs#L36)
- [`struct RouteCatalogArgs`](../frontend_contract_macros/src/lib.rs#L144)
- [`struct RouteCatalogRouteArgs`](../frontend_contract_macros/src/lib.rs#L149)
- [`struct PageCatalogArgs`](../frontend_contract_macros/src/lib.rs#L156)
- [`struct PageCatalogPageArgs`](../frontend_contract_macros/src/lib.rs#L162)
- [`struct TypedRouteArgs`](../frontend_contract_macros/src/lib.rs#L325)
- [`enum SynTypedRouteErrors`](../frontend_contract_macros/src/lib.rs#L342)
- [`struct RouteRegistryBinding`](../frontend_contract_macros/src/lib.rs#L348)
- [`struct SynRouteRegistryHandler`](../frontend_contract_macros/src/lib.rs#L353)
- [`struct SynRouteRegistryRoute`](../frontend_contract_macros/src/lib.rs#L356)
- [`struct SynRouteRegistryBindings`](../frontend_contract_macros/src/lib.rs#L359)
- [`struct SynRouteRegistrySchemas`](../frontend_contract_macros/src/lib.rs#L362)
- [`struct SynRouteRegistryState`](../frontend_contract_macros/src/lib.rs#L365)
- [`struct SynRouteRegistryFamily`](../frontend_contract_macros/src/lib.rs#L368)
- [`struct HandlerRegistryBinding`](../frontend_contract_macros/src/lib.rs#L371)
- [`struct SynHandlerRegistryContract`](../frontend_contract_macros/src/lib.rs#L376)
- [`struct SynHandlerRegistryHandler`](../frontend_contract_macros/src/lib.rs#L378)
- [`struct SynHandlerRegistryBindings`](../frontend_contract_macros/src/lib.rs#L381)
- [`struct SynHandlerRegistryState`](../frontend_contract_macros/src/lib.rs#L386)
- [`struct HandlerRegistryArgs`](../frontend_contract_macros/src/lib.rs#L400)
- [`struct RouteRegistryArgs`](../frontend_contract_macros/src/lib.rs#L442)

## `frontend_contract_validation/src/json_snapshot.rs`

- [`struct JsonContractSnapshot`](../frontend_contract_validation/src/json_snapshot.rs#L11)
- [`struct JsonSnapshotDynamicFieldRef`](../frontend_contract_validation/src/json_snapshot.rs#L24)
- [`enum JsonContractSnapshotError`](../frontend_contract_validation/src/json_snapshot.rs#L29)

## `frontend_contract_validation/src/openapi_validation.rs`

- [`struct OpenApiContractText`](../frontend_contract_validation/src/openapi_validation.rs#L13)
- [`struct OpenApiContractTextError`](../frontend_contract_validation/src/openapi_validation.rs#L23)
- [`struct SerdeJsonOpenApiSerializationError`](../frontend_contract_validation/src/openapi_validation.rs#L29)
- [`struct RuntimeRoutesRef`](../frontend_contract_validation/src/openapi_validation.rs#L32)
- [`struct OpenApiSchemaReferencesBTreeSet`](../frontend_contract_validation/src/openapi_validation.rs#L35)
- [`enum OpenApiValidationError`](../frontend_contract_validation/src/openapi_validation.rs#L65)
- [`struct OpenApiResponseStatus`](../frontend_contract_validation/src/openapi_validation.rs#L90)
- [`enum OpenApiSecurityExpectation`](../frontend_contract_validation/src/openapi_validation.rs#L103)
- [`struct OpenApiOperationExpectation`](../frontend_contract_validation/src/openapi_validation.rs#L109)
- [`enum OpenApiOperationValidationError`](../frontend_contract_validation/src/openapi_validation.rs#L135)
- [`enum OpenApiSchemaMismatch`](../frontend_contract_validation/src/openapi_validation.rs#L145)
- [`enum OpenApiPayloadValidationError`](../frontend_contract_validation/src/openapi_validation.rs#L159)

## `frontend_contract_validation/src/route_contract_validation.rs`

- [`enum RouteContractMismatch`](../frontend_contract_validation/src/route_contract_validation.rs#L2)
- [`struct RouteContractMismatches`](../frontend_contract_validation/src/route_contract_validation.rs#L26)
- [`struct HttpContractStatus`](../frontend_contract_validation/src/route_contract_validation.rs#L37)
- [`struct HttpContractBody`](../frontend_contract_validation/src/route_contract_validation.rs#L50)
- [`enum HttpContractBodyKind`](../frontend_contract_validation/src/route_contract_validation.rs#L67)
- [`struct HttpContractObservation`](../frontend_contract_validation/src/route_contract_validation.rs#L73)
- [`struct HttpContractExpectation`](../frontend_contract_validation/src/route_contract_validation.rs#L95)
- [`enum HttpContractMismatch`](../frontend_contract_validation/src/route_contract_validation.rs#L116)

## `generate_quotes/src/domain_types.rs`

- [`struct QuotePrefix`](../generate_quotes/src/domain_types.rs#L3)
- [`struct QuoteChar`](../generate_quotes/src/domain_types.rs#L5)
- [`struct QuotePanicId`](../generate_quotes/src/domain_types.rs#L7)
- [`struct QuoteStyle`](../generate_quotes/src/domain_types.rs#L9)
- [`struct QuotedLiteral`](../generate_quotes/src/domain_types.rs#L25)
- [`struct ProcMacro2QuotedLiteralTokenStream`](../generate_quotes/src/domain_types.rs#L36)

## `git_info/src/domain_types.rs`

- [`struct GitCommitIdRef`](../git_info/src/domain_types.rs#L18)
- [`struct GitCommitId`](../git_info/src/domain_types.rs#L39)
- [`enum GitInfoStringTryFromStringError`](../git_info/src/domain_types.rs#L54)
- [`struct GitCommitIdCow`](../git_info/src/domain_types.rs#L79)
- [`struct GitCommitIdFallback`](../git_info/src/domain_types.rs#L101)
- [`struct GitCommitLink`](../git_info/src/domain_types.rs#L116)
- [`struct GitCommitLinkCow`](../git_info/src/domain_types.rs#L157)
- [`struct ProjectGitCommitLinkRef`](../git_info/src/domain_types.rs#L194)
- [`struct IsProjectCommit`](../git_info/src/domain_types.rs#L205)
- [`struct GitCommitLinkCapacity`](../git_info/src/domain_types.rs#L217)
- [`struct GitCommitLinkOutputRefMut`](../git_info/src/domain_types.rs#L219)
- [`struct ValidateProjectCommitError`](../git_info/src/domain_types.rs#L232)
- [`struct ProjectGitInfo`](../git_info/src/domain_types.rs#L244)
- [`trait GetGitCommitLink`](../git_info/src/domain_types.rs#L263)
- [`trait GetGitCommitId`](../git_info/src/domain_types.rs#L269)

## `initialize_environment_files/src/main.rs`

- [`enum RunMode`](../initialize_environment_files/src/main.rs#L2)
- [`enum InitializationStatus`](../initialize_environment_files/src/main.rs#L7)
- [`struct InitializationEntry`](../initialize_environment_files/src/main.rs#L15)
- [`struct EnvContent`](../initialize_environment_files/src/main.rs#L29)
- [`struct EnvContentRef`](../initialize_environment_files/src/main.rs#L48)
- [`struct EnvKey`](../initialize_environment_files/src/main.rs#L61)
- [`struct EnvKeys`](../initialize_environment_files/src/main.rs#L73)
- [`struct MemberSafe`](../initialize_environment_files/src/main.rs#L81)
- [`struct WorkspaceMember`](../initialize_environment_files/src/main.rs#L94)
- [`struct WorkspaceMemberRef`](../initialize_environment_files/src/main.rs#L108)
- [`struct WorkspaceMembers`](../initialize_environment_files/src/main.rs#L110)
- [`struct WorkspaceRootPathRef`](../initialize_environment_files/src/main.rs#L120)
- [`struct InitPathRef`](../initialize_environment_files/src/main.rs#L122)
- [`struct InitMaxBytes`](../initialize_environment_files/src/main.rs#L124)
- [`struct InitEntries`](../initialize_environment_files/src/main.rs#L126)
- [`struct InitIoError`](../initialize_environment_files/src/main.rs#L133)
- [`struct ServerRuntimeBoundedReadError`](../initialize_environment_files/src/main.rs#L138)
- [`struct TomlInitError`](../initialize_environment_files/src/main.rs#L143)
- [`struct InitStringError`](../initialize_environment_files/src/main.rs#L146)
- [`enum InitializeError`](../initialize_environment_files/src/main.rs#L148)

## `location_lib/src/location.rs`

- [`struct LocationFile`](../location_lib/src/location.rs#L21)
- [`struct LocationLine`](../location_lib/src/location.rs#L41)
- [`struct LocationColumn`](../location_lib/src/location.rs#L80)
- [`struct LocationCoordinateTryFromU32Error`](../location_lib/src/location.rs#L104)
- [`struct LocationCommit`](../location_lib/src/location.rs#L120)
- [`struct LocationDuration`](../location_lib/src/location.rs#L134)
- [`struct LocationFileRef`](../location_lib/src/location.rs#L168)
- [`struct FormatterRefMut`](../location_lib/src/location.rs#L170)
- [`struct ChronoLocationDisplayTimezone`](../location_lib/src/location.rs#L172)
- [`struct ChronoLocationDateTime`](../location_lib/src/location.rs#L174)
- [`struct Occr`](../location_lib/src/location.rs#L187)
- [`struct Location`](../location_lib/src/location.rs#L204)
- [`struct StdTimeDuration`](../location_lib/src/location.rs#L339)
- [`struct StdTimeDurationSecs`](../location_lib/src/location.rs#L352)
- [`struct StdTimeDurationNanos`](../location_lib/src/location.rs#L366)
- [`struct StdTimeDurationNanosTryFromU32Error`](../location_lib/src/location.rs#L381)

## `location_lib_location/src/domain_types.rs`

- [`struct SynItemEnumMutRef`](../location_lib_location/src/domain_types.rs#L2)

## `location_lib_location/src/lib.rs`

- [`enum SuportedEnumVariant`](../location_lib_location/src/lib.rs#L68)

## `location_lib_location_test/src/main.rs`

- [`enum ErrorOne`](../location_lib_location_test/src/main.rs#L13)
- [`struct LocationTestText`](../location_lib_location_test/src/main.rs#L53)
- [`struct LocationTestFlag`](../location_lib_location_test/src/main.rs#L67)
- [`struct LocationTestCount`](../location_lib_location_test/src/main.rs#L81)
- [`enum ErrorTwo`](../location_lib_location_test/src/main.rs#L85)
- [`enum ErrorUnnamedOne`](../location_lib_location_test/src/main.rs#L100)
- [`struct DisplayStruct`](../location_lib_location_test/src/main.rs#L104)
- [`struct SerdeStruct`](../location_lib_location_test/src/main.rs#L120)

## `macros_helpers/src/attr_identifier_str.rs`

- [`struct AttrIdentifierName`](../macros_helpers/src/attr_identifier_str.rs#L4)
- [`trait AttrIdentifierStr`](../macros_helpers/src/attr_identifier_str.rs#L5)

## `macros_helpers/src/derive_token_stream_builder.rs`

- [`macro-generated struct DTokenStreamBuilder`](../macros_helpers/src/derive_token_stream_builder.rs#L1)

## `macros_helpers/src/generate_field_location_new_token_stream.rs`

- [`struct FieldLocationFile`](../macros_helpers/src/generate_field_location_new_token_stream.rs#L4)
- [`struct FieldLocationLine`](../macros_helpers/src/generate_field_location_new_token_stream.rs#L10)
- [`struct FieldLocationColumn`](../macros_helpers/src/generate_field_location_new_token_stream.rs#L35)
- [`struct FieldLocationCoordinateTryFromU32Error`](../macros_helpers/src/generate_field_location_new_token_stream.rs#L59)

## `macros_helpers/src/generate_if_write_is_err_token_stream.rs`

- [`struct ProcMacro2IfWriteIsErrTokenStream`](../macros_helpers/src/generate_if_write_is_err_token_stream.rs#L4)

## `macros_helpers/src/generate_simple_syn_punct.rs`

- [`struct SynPathSegment`](../macros_helpers/src/generate_simple_syn_punct.rs#L8)
- [`struct SynPathSegments`](../macros_helpers/src/generate_simple_syn_punct.rs#L17)

## `macros_helpers/src/get_macro_attr.rs`

- [`struct SynMacroAttrRef`](../macros_helpers/src/get_macro_attr.rs#L11)
- [`struct ProcMacro2MacroAttrMetaListTokenStreamRef`](../macros_helpers/src/get_macro_attr.rs#L21)
- [`struct AttrPathMatches`](../macros_helpers/src/get_macro_attr.rs#L31)
- [`enum MacroAttrError`](../macros_helpers/src/get_macro_attr.rs#L35)

## `macros_helpers/src/json_contract.rs`

- [`struct JsonFixtureRef`](../macros_helpers/src/json_contract.rs#L2)
- [`struct SerdeJsonError`](../macros_helpers/src/json_contract.rs#L7)
- [`enum ContractError`](../macros_helpers/src/json_contract.rs#L9)

## `macros_helpers/src/location.rs`

- [`enum LocationFieldAttr`](../macros_helpers/src/location.rs#L3)
- [`struct CompileErrorMessage`](../macros_helpers/src/location.rs#L90)
- [`struct SynVariantRef`](../macros_helpers/src/location.rs#L97)

## `macros_helpers/src/location_syn_field.rs`

- [`struct SynLocationField`](../macros_helpers/src/location_syn_field.rs#L4)

## `macros_helpers/src/proc_macro2_tokens.rs`

- [`struct ProcMacro2GeneratedRustTokenStream`](../macros_helpers/src/proc_macro2_tokens.rs#L13)

## `macros_helpers/src/rs_file_path.rs`

- [`struct RsFilePathBuf`](../macros_helpers/src/rs_file_path.rs#L10)

## `macros_helpers/src/status_code.rs`

- [`enum StatusCode`](../macros_helpers/src/status_code.rs#L14)
- [`enum GetOnlyOneStatusCodeError`](../macros_helpers/src/status_code.rs#L441)
- [`struct SynStatusCodeVariantRef`](../macros_helpers/src/status_code.rs#L448)

## `macros_helpers/src/syn_field.rs`

- [`struct SynField`](../macros_helpers/src/syn_field.rs#L2)
- [`struct SynFieldIdentifier`](../macros_helpers/src/syn_field.rs#L18)
- [`struct SynFieldType`](../macros_helpers/src/syn_field.rs#L28)
- [`struct SynFieldVis`](../macros_helpers/src/syn_field.rs#L38)

## `macros_helpers/src/test_database.rs`

- [`struct UrlRef`](../macros_helpers/src/test_database.rs#L3)
- [`struct SanitizedDatabaseTarget`](../macros_helpers/src/test_database.rs#L14)
- [`enum UrlError`](../macros_helpers/src/test_database.rs#L18)

## `macros_helpers/src/test_hlp.rs`

- [`struct TestPathStemRef`](../macros_helpers/src/test_hlp.rs#L3)
- [`struct TestPathStem`](../macros_helpers/src/test_hlp.rs#L5)
- [`struct AssertFilePathRef`](../macros_helpers/src/test_hlp.rs#L15)
- [`struct StdAssertFilePath`](../macros_helpers/src/test_hlp.rs#L22)
- [`struct ExpectedFileContentRef`](../macros_helpers/src/test_hlp.rs#L32)
- [`struct ExpectedFileContent`](../macros_helpers/src/test_hlp.rs#L39)

## `macros_helpers/src/tool_command.rs`

- [`struct PathRef`](../macros_helpers/src/tool_command.rs#L2)
- [`struct ProcessCommand`](../macros_helpers/src/tool_command.rs#L4)
- [`struct OsStringValue`](../macros_helpers/src/tool_command.rs#L6)
- [`struct ToolProgramRef`](../macros_helpers/src/tool_command.rs#L13)
- [`struct ToolArgRef`](../macros_helpers/src/tool_command.rs#L15)
- [`struct ToolArgsRef`](../macros_helpers/src/tool_command.rs#L17)
- [`struct ToolEnvKeyRef`](../macros_helpers/src/tool_command.rs#L19)
- [`struct ToolEnvValueRef`](../macros_helpers/src/tool_command.rs#L21)
- [`struct ProcessExitStatus`](../macros_helpers/src/tool_command.rs#L31)
- [`struct ProcessOutput`](../macros_helpers/src/tool_command.rs#L39)
- [`struct ToolCommand`](../macros_helpers/src/tool_command.rs#L41)

## `macros_helpers/src/wrap_derive.rs`

- [`struct ProcMacro2DeriveTokensRef`](../macros_helpers/src/wrap_derive.rs#L2)

## `macros_helpers/src/write_string_into_file.rs`

- [`struct WrittenFilePathBuf`](../macros_helpers/src/write_string_into_file.rs#L10)
- [`struct WrittenFilePathRef`](../macros_helpers/src/write_string_into_file.rs#L19)
- [`struct StringFileContentRef`](../macros_helpers/src/write_string_into_file.rs#L28)
- [`struct GeneratedFileMaximumBytes`](../macros_helpers/src/write_string_into_file.rs#L38)
- [`struct ShouldWriteString`](../macros_helpers/src/write_string_into_file.rs#L50)
- [`enum WritePathOutcome`](../macros_helpers/src/write_string_into_file.rs#L52)

## `macros_helpers/src/write_token_stream_into_file.rs`

- [`enum FormatWithCargofmt`](../macros_helpers/src/write_token_stream_into_file.rs#L2)
- [`enum ShouldWriteTokenStreamIntoFile`](../macros_helpers/src/write_token_stream_into_file.rs#L7)
- [`struct ProcMacro2TokenStreamRef`](../macros_helpers/src/write_token_stream_into_file.rs#L19)
- [`struct StdRustfmtPath`](../macros_helpers/src/write_token_stream_into_file.rs#L21)
- [`struct ShouldWriteTokenStreamFlag`](../macros_helpers/src/write_token_stream_into_file.rs#L23)

## `macros_helpers_generate_derive_token_stream_builder/src/domain_types.rs`

- [`struct ToSnakeCaseInput`](../macros_helpers_generate_derive_token_stream_builder/src/domain_types.rs#L6)
- [`struct SnakeCaseString`](../macros_helpers_generate_derive_token_stream_builder/src/domain_types.rs#L10)

## `macros_helpers_generate_derive_token_stream_builder/src/lib.rs`

- [`struct Element`](../macros_helpers_generate_derive_token_stream_builder/src/lib.rs#L30)

## `naming/src/domain_types.rs`

- [`struct HashMap`](../naming/src/domain_types.rs#L480)
- [`struct HashMapUpperCamelCase`](../naming/src/domain_types.rs#L482)
- [`struct HashMapSnakeCase`](../naming/src/domain_types.rs#L494)
- [`trait DisplayPlusToTokens`](../naming/src/domain_types.rs#L505)
- [`struct SwaggerUrlPathPrefix`](../naming/src/domain_types.rs#L515)
- [`struct SwaggerUrlPathSelfQuotesStrValue`](../naming/src/domain_types.rs#L519)
- [`struct SwaggerUrlPathSelfQuotesTokenStreamValue`](../naming/src/domain_types.rs#L523)
- [`trait SwaggerUrlPathSelfQuotesStr`](../naming/src/domain_types.rs#L526)
- [`trait SwaggerUrlPathSelfQuotesTokenStream`](../naming/src/domain_types.rs#L547)

## `naming_naming_common/src/domain_types.rs`

- [`struct ConvertCaseKind`](../naming_naming_common/src/domain_types.rs#L73)
- [`struct CaseString`](../naming_naming_common/src/domain_types.rs#L85)
- [`struct ProcMacro2CaseTokenStream`](../naming_naming_common/src/domain_types.rs#L87)

## `naming_naming_macros/src/domain_types.rs`

- [`struct ProcMacro2GeneratedNamingTokenStream`](../naming_naming_macros/src/domain_types.rs#L7)
- [`struct SynEnumIdentifierRef`](../naming_naming_macros/src/domain_types.rs#L12)
- [`struct ProcMacro2VariantMatchingTokensRef`](../naming_naming_macros/src/domain_types.rs#L21)

## `newtype/src/lib.rs`

- [`struct NewtypeAttrs`](../newtype/src/lib.rs#L14)
- [`struct NewtypeTryFromAttrs`](../newtype/src/lib.rs#L20)
- [`struct BoundedStringAttrs`](../newtype/src/lib.rs#L25)
- [`struct WireEnumAttrs`](../newtype/src/lib.rs#L33)
- [`enum BoundedStringOption`](../newtype/src/lib.rs#L69)
- [`enum NewtypeOption`](../newtype/src/lib.rs#L80)
- [`enum ToErrStringMode`](../newtype/src/lib.rs#L114)
- [`struct ProcMacro2GeneratedTokenStream`](../newtype/src/lib.rs#L120)
- [`struct ProcMacroInputTokenStream`](../newtype/src/lib.rs#L127)
- [`struct NewtypeBool`](../newtype/src/lib.rs#L149)
- [`struct SnakeIdentifier`](../newtype/src/lib.rs#L161)
- [`struct SnakeIdentifierifierLen`](../newtype/src/lib.rs#L163)
- [`struct SnakeIdentifierifierTryFromStringError`](../newtype/src/lib.rs#L170)
- [`struct SynAttrsRef`](../newtype/src/lib.rs#L212)
- [`struct SynDeriveInputRef`](../newtype/src/lib.rs#L224)
- [`struct SynIdentifierRef`](../newtype/src/lib.rs#L236)
- [`struct SynIdentifier`](../newtype/src/lib.rs#L238)
- [`struct SynTypeRef`](../newtype/src/lib.rs#L255)
- [`struct SynType`](../newtype/src/lib.rs#L267)
- [`struct SynExpr`](../newtype/src/lib.rs#L274)

## `notification_service/src/main.rs`

- [`struct NotificationState`](../notification_service/src/main.rs#L9)
- [`struct AxumNotificationState`](../notification_service/src/main.rs#L15)
- [`struct AxumNotificationJson`](../notification_service/src/main.rs#L18)
- [`struct AxumNotificationResponse`](../notification_service/src/main.rs#L21)
- [`struct AxumNotificationRouter`](../notification_service/src/main.rs#L24)
- [`struct HttpNotificationStatusCode`](../notification_service/src/main.rs#L27)
- [`enum CreateNotificationError`](../notification_service/src/main.rs#L30)
- [`enum MetricsError`](../notification_service/src/main.rs#L37)
- [`struct MetricsExporterPrometheusHandle`](../notification_service/src/main.rs#L44)
- [`struct NotificationBodyMaximumBytes`](../notification_service/src/main.rs#L47)
- [`struct NotificationExitCode`](../notification_service/src/main.rs#L50)
- [`enum NotificationServiceError`](../notification_service/src/main.rs#L158)
- [`struct NotificationConfigError`](../notification_service/src/main.rs#L181)
- [`struct SqlxNotificationDatabaseError`](../notification_service/src/main.rs#L187)
- [`struct SqlxNotificationMigrationError`](../notification_service/src/main.rs#L192)
- [`struct NotificationIoError`](../notification_service/src/main.rs#L197)
- [`struct NotificationServeError`](../notification_service/src/main.rs#L202)
- [`struct MetricsExporterPrometheusNotificationBuildError`](../notification_service/src/main.rs#L207)
- [`struct NotificationObservabilityInitError`](../notification_service/src/main.rs#L212)
- [`struct NotificationObservabilityShutdownError`](../notification_service/src/main.rs#L217)
- [`enum NotificationErrorCode`](../notification_service/src/main.rs#L221)

## `notification_service/src/routes.rs`

- [`struct NotificationApiRouteRegistry`](../notification_service/src/routes.rs#L78)
- [`struct NotificationRouteRegistry`](../notification_service/src/routes.rs#L97)

## `notification_service_config/src/domain_types.rs`

- [`struct Config`](../notification_service_config/src/domain_types.rs#L7)

## `notification_service_contract/src/domain_types.rs`

- [`struct CreateNotificationReq`](../notification_service_contract/src/domain_types.rs#L15)
- [`struct CreateNotificationRes`](../notification_service_contract/src/domain_types.rs#L40)
- [`struct NotificationMessage`](../notification_service_contract/src/domain_types.rs#L66)
- [`struct UuidNotificationId`](../notification_service_contract/src/domain_types.rs#L82)
- [`struct CreateNotificationRoute`](../notification_service_contract/src/domain_types.rs#L101)
- [`enum NotificationRoute`](../notification_service_contract/src/domain_types.rs#L116)
- [`enum NotificationOperationalRoute`](../notification_service_contract/src/domain_types.rs#L134)
- [`enum NotificationMessageTryFromStringError`](../notification_service_contract/src/domain_types.rs#L175)

## `panic_location/src/domain_types.rs`

- [`struct PanicFile`](../panic_location/src/domain_types.rs#L2)
- [`struct PanicLine`](../panic_location/src/domain_types.rs#L5)
- [`struct PanicColumn`](../panic_location/src/domain_types.rs#L8)

## `pg_crud_common/src/advisory_lock.rs`

- [`struct PgRelationRowCount`](../pg_crud_common/src/advisory_lock.rs#L13)
- [`struct PgRelationCapacityMaximum`](../pg_crud_common/src/advisory_lock.rs#L16)
- [`enum PgRelationCapacityError`](../pg_crud_common/src/advisory_lock.rs#L31)
- [`struct PgRelationResourceId`](../pg_crud_common/src/advisory_lock.rs#L51)
- [`struct PgRelationLockNamespace`](../pg_crud_common/src/advisory_lock.rs#L54)
- [`struct PgRelationResourceIds`](../pg_crud_common/src/advisory_lock.rs#L71)
- [`enum PgRelationLockError`](../pg_crud_common/src/advisory_lock.rs#L99)
- [`struct SqlxPgRelationLockError`](../pg_crud_common/src/advisory_lock.rs#L110)
- [`struct SqlxPgRelationLockConnectionRef`](../pg_crud_common/src/advisory_lock.rs#L113)

## `pg_crud_common/src/batch_validation.rs`

- [`enum BatchDuplicatePolicy`](../pg_crud_common/src/batch_validation.rs#L2)
- [`struct BatchProcessedItemCount`](../pg_crud_common/src/batch_validation.rs#L18)
- [`struct BatchInvalidItemCount`](../pg_crud_common/src/batch_validation.rs#L30)
- [`struct BatchStoppedEarly`](../pg_crud_common/src/batch_validation.rs#L42)
- [`struct BatchInvalidItems`](../pg_crud_common/src/batch_validation.rs#L47)
- [`struct BatchRecordsBTreeMap`](../pg_crud_common/src/batch_validation.rs#L57)
- [`struct BatchValidationReport`](../pg_crud_common/src/batch_validation.rs#L60)

## `pg_crud_common/src/bind_index.rs`

- [`struct QueryPartIncrement`](../pg_crud_common/src/bind_index.rs#L14)
- [`trait QueryPartIncrementMut`](../pg_crud_common/src/bind_index.rs#L15)

## `pg_crud_common/src/bounded_btree_map.rs`

- [`struct StdBoundedBTreeMapLen`](../pg_crud_common/src/bounded_btree_map.rs#L11)
- [`struct BoundedBTreeMapError`](../pg_crud_common/src/bounded_btree_map.rs#L18)
- [`struct BoundedBTreeMap`](../pg_crud_common/src/bounded_btree_map.rs#L21)

## `pg_crud_common/src/bounded_unique_vec.rs`

- [`struct UniqueVecLen`](../pg_crud_common/src/bounded_unique_vec.rs#L13)
- [`enum UniqueVecError`](../pg_crud_common/src/bounded_unique_vec.rs#L18)
- [`struct BoundedUniqueVec`](../pg_crud_common/src/bounded_unique_vec.rs#L45)
- [`struct BoundedUniqueVecVisitorPhantomData`](../pg_crud_common/src/bounded_unique_vec.rs#L67)

## `pg_crud_common/src/bounded_vec.rs`

- [`struct BoundedVecLen`](../pg_crud_common/src/bounded_vec.rs#L15)
- [`enum BoundedVecError`](../pg_crud_common/src/bounded_vec.rs#L20)
- [`struct BoundedVec`](../pg_crud_common/src/bounded_vec.rs#L46)

## `pg_crud_common/src/cardinality.rs`

- [`struct DuplicateIdx`](../pg_crud_common/src/cardinality.rs#L13)
- [`struct DuplicateCandidates`](../pg_crud_common/src/cardinality.rs#L23)

## `pg_crud_common/src/cursor.rs`

- [`struct CursorMaximumLength`](../pg_crud_common/src/cursor.rs#L4)
- [`enum CursorPaginationUsage`](../pg_crud_common/src/cursor.rs#L7)
- [`enum OffsetPaginationPresence`](../pg_crud_common/src/cursor.rs#L34)
- [`enum SignedCursorPresence`](../pg_crud_common/src/cursor.rs#L40)
- [`struct CursorSigningKey`](../pg_crud_common/src/cursor.rs#L56)
- [`struct CursorSigningKeyError`](../pg_crud_common/src/cursor.rs#L85)
- [`struct CursorPayload`](../pg_crud_common/src/cursor.rs#L90)
- [`struct CursorPayloadError`](../pg_crud_common/src/cursor.rs#L112)
- [`struct SignedCursor`](../pg_crud_common/src/cursor.rs#L117)
- [`struct SignedCursorError`](../pg_crud_common/src/cursor.rs#L139)
- [`struct CursorCodec`](../pg_crud_common/src/cursor.rs#L142)
- [`enum CursorCodecBuildError`](../pg_crud_common/src/cursor.rs#L215)
- [`enum CursorEncodeError`](../pg_crud_common/src/cursor.rs#L223)
- [`enum CursorDecodeError`](../pg_crud_common/src/cursor.rs#L233)

## `pg_crud_common/src/date_sql_filter.rs`

- [`struct ChronoUtcDateTimeRef`](../pg_crud_common/src/date_sql_filter.rs#L2)
- [`struct DateFilterBounds`](../pg_crud_common/src/date_sql_filter.rs#L5)
- [`struct DateSqlBindStartNonZeroU32`](../pg_crud_common/src/date_sql_filter.rs#L37)
- [`struct ChronoUtcDateTimes`](../pg_crud_common/src/date_sql_filter.rs#L48)
- [`struct DateSqlFilter`](../pg_crud_common/src/date_sql_filter.rs#L51)
- [`enum DateSqlFilterError`](../pg_crud_common/src/date_sql_filter.rs#L65)

## `pg_crud_common/src/db_schema_conformance.rs`

- [`struct DbSchemaText`](../pg_crud_common/src/db_schema_conformance.rs#L16)
- [`struct DbSchemaTextError`](../pg_crud_common/src/db_schema_conformance.rs#L26)
- [`struct DbColumnNullable`](../pg_crud_common/src/db_schema_conformance.rs#L39)
- [`trait PgColumnSchema`](../pg_crud_common/src/db_schema_conformance.rs#L41)
- [`struct DbStaticSchemaText`](../pg_crud_common/src/db_schema_conformance.rs#L57)
- [`struct DbColumnHasServerDefault`](../pg_crud_common/src/db_schema_conformance.rs#L70)
- [`struct DbColumnSpec`](../pg_crud_common/src/db_schema_conformance.rs#L74)
- [`struct DbColumnSpecs`](../pg_crud_common/src/db_schema_conformance.rs#L92)
- [`struct DbStaticSchemaTexts`](../pg_crud_common/src/db_schema_conformance.rs#L105)
- [`struct DbKeySpecs`](../pg_crud_common/src/db_schema_conformance.rs#L118)
- [`struct DbObjectSpecs`](../pg_crud_common/src/db_schema_conformance.rs#L131)
- [`struct DbDefaultSpecs`](../pg_crud_common/src/db_schema_conformance.rs#L144)
- [`trait DbTableSchema`](../pg_crud_common/src/db_schema_conformance.rs#L163)
- [`struct DbDefaultSpec`](../pg_crud_common/src/db_schema_conformance.rs#L172)
- [`struct DbObjectSpec`](../pg_crud_common/src/db_schema_conformance.rs#L184)
- [`trait DbExtendedTableSchema`](../pg_crud_common/src/db_schema_conformance.rs#L203)
- [`enum DbKeySpec`](../pg_crud_common/src/db_schema_conformance.rs#L209)
- [`enum DbKeyContractSnapshot`](../pg_crud_common/src/db_schema_conformance.rs#L222)
- [`struct DbSchemaTexts`](../pg_crud_common/src/db_schema_conformance.rs#L246)
- [`struct DbColumnContractSnapshots`](../pg_crud_common/src/db_schema_conformance.rs#L261)
- [`struct DbKeyContractSnapshots`](../pg_crud_common/src/db_schema_conformance.rs#L276)
- [`struct DbColumnSnapshots`](../pg_crud_common/src/db_schema_conformance.rs#L291)
- [`struct DbObjectSnapshots`](../pg_crud_common/src/db_schema_conformance.rs#L306)
- [`struct DbColumnContractSnapshot`](../pg_crud_common/src/db_schema_conformance.rs#L312)
- [`struct SqlxPgPoolRef`](../pg_crud_common/src/db_schema_conformance.rs#L336)
- [`struct DbSchemaNameRef`](../pg_crud_common/src/db_schema_conformance.rs#L339)
- [`struct DbTableNameRef`](../pg_crud_common/src/db_schema_conformance.rs#L342)
- [`struct DbColumnSnapshot`](../pg_crud_common/src/db_schema_conformance.rs#L347)
- [`enum DbObjectKind`](../pg_crud_common/src/db_schema_conformance.rs#L373)
- [`struct DbObjectSnapshot`](../pg_crud_common/src/db_schema_conformance.rs#L390)
- [`struct DbTableSnapshot`](../pg_crud_common/src/db_schema_conformance.rs#L407)
- [`struct DbCatalogSnapshot`](../pg_crud_common/src/db_schema_conformance.rs#L413)
- [`struct SqlxDbSchemaInspectionError`](../pg_crud_common/src/db_schema_conformance.rs#L435)
- [`enum DbSchemaConformanceError`](../pg_crud_common/src/db_schema_conformance.rs#L438)

## `pg_crud_common/src/errors.rs`

- [`struct SqlxBoxDynError`](../pg_crud_common/src/errors.rs#L5)
- [`struct SqlxPostgresQueryBindError`](../pg_crud_common/src/errors.rs#L8)
- [`enum PgCrudStringWrapperTryFromStringError`](../pg_crud_common/src/errors.rs#L24)
- [`enum QueryPartError`](../pg_crud_common/src/errors.rs#L53)

## `pg_crud_common/src/filter_bind_plan.rs`

- [`struct PgFilterBool`](../pg_crud_common/src/filter_bind_plan.rs#L10)
- [`struct PgFilterI64`](../pg_crud_common/src/filter_bind_plan.rs#L21)
- [`struct PgFilterText`](../pg_crud_common/src/filter_bind_plan.rs#L24)
- [`struct PgFilterTextError`](../pg_crud_common/src/filter_bind_plan.rs#L40)
- [`enum PgFilterBindValue`](../pg_crud_common/src/filter_bind_plan.rs#L43)
- [`struct FilterBindPlan`](../pg_crud_common/src/filter_bind_plan.rs#L58)

## `pg_crud_common/src/finite_f64.rs`

- [`enum FiniteF64Error`](../pg_crud_common/src/finite_f64.rs#L4)
- [`struct FiniteF64`](../pg_crud_common/src/finite_f64.rs#L18)
- [`enum PositiveFiniteF64Error`](../pg_crud_common/src/finite_f64.rs#L34)
- [`struct PositiveFiniteF64`](../pg_crud_common/src/finite_f64.rs#L50)
- [`enum UnitIntervalF64Error`](../pg_crud_common/src/finite_f64.rs#L69)
- [`struct UnitIntervalF64`](../pg_crud_common/src/finite_f64.rs#L85)

## `pg_crud_common/src/invariants.rs`

- [`enum BulkMutationOutcome`](../pg_crud_common/src/invariants.rs#L2)
- [`struct PaginationTotal`](../pg_crud_common/src/invariants.rs#L16)
- [`enum DataInvariantViolation`](../pg_crud_common/src/invariants.rs#L21)

## `pg_crud_common/src/lib.rs`

- [`struct AllEnumVariants`](../pg_crud_common/src/lib.rs#L148)
- [`trait AllEnumVariantsArrayDefaultSomeOneElement`](../pg_crud_common/src/lib.rs#L149)
- [`trait AllEnumVariantsArrayDefaultSomeOneElementMaxPageSize`](../pg_crud_common/src/lib.rs#L152)
- [`trait DefaultSomeOneElement`](../pg_crud_common/src/lib.rs#L155)
- [`trait DefaultSomeOneElementMaxPageSize`](../pg_crud_common/src/lib.rs#L158)
- [`enum Operator`](../pg_crud_common/src/lib.rs#L175)
- [`enum PgTypeGreaterThanVariant`](../pg_crud_common/src/lib.rs#L278)
- [`trait PgType`](../pg_crud_common/src/lib.rs#L318)
- [`trait PgTypePrimaryKey`](../pg_crud_common/src/lib.rs#L372)
- [`trait PgTypeNotPrimaryKey`](../pg_crud_common/src/lib.rs#L387)
- [`struct PgTypeGreaterThanTest`](../pg_crud_common/src/lib.rs#L469)
- [`struct PgTypeLenGreaterThanTest`](../pg_crud_common/src/lib.rs#L476)
- [`struct SqlxPostgresQuery`](../pg_crud_common/src/lib.rs#L482)
- [`struct AddOperator`](../pg_crud_common/src/lib.rs#L517)
- [`struct IsPrimaryKey`](../pg_crud_common/src/lib.rs#L528)
- [`trait PgTypeWhereFilter`](../pg_crud_common/src/lib.rs#L529)
- [`struct NullableJsonObjPgTypeWhereFilter`](../pg_crud_common/src/lib.rs#L555)
- [`struct PgTypeWhere`](../pg_crud_common/src/lib.rs#L656)
- [`enum __Field`](../pg_crud_common/src/lib.rs#L720)
- [`struct __FieldVisitor`](../pg_crud_common/src/lib.rs#L727)
- [`struct __Visitor`](../pg_crud_common/src/lib.rs#L781)

## `pg_crud_common/src/list_total.rs`

- [`struct ListOffset`](../pg_crud_common/src/list_total.rs#L2)
- [`enum ListRowsPresence`](../pg_crud_common/src/list_total.rs#L10)
- [`enum WindowTotalPresence`](../pg_crud_common/src/list_total.rs#L16)
- [`enum ListTotalSource`](../pg_crud_common/src/list_total.rs#L22)
- [`struct ListTotal`](../pg_crud_common/src/list_total.rs#L37)
- [`struct ListTotalError`](../pg_crud_common/src/list_total.rs#L43)
- [`struct ListItems`](../pg_crud_common/src/list_total.rs#L64)
- [`struct ListPage`](../pg_crud_common/src/list_total.rs#L67)
- [`struct ListRows`](../pg_crud_common/src/list_total.rs#L84)

## `pg_crud_common/src/operation_budget.rs`

- [`struct OperationBudget`](../pg_crud_common/src/operation_budget.rs#L10)
- [`struct OperationCount`](../pg_crud_common/src/operation_budget.rs#L22)
- [`struct OperationBudgetExceeded`](../pg_crud_common/src/operation_budget.rs#L28)

## `pg_crud_common/src/operational_invariants.rs`

- [`enum PgScopedForeignKeyOnDelete`](../pg_crud_common/src/operational_invariants.rs#L5)
- [`enum PgScopedForeignKeyError`](../pg_crud_common/src/operational_invariants.rs#L13)
- [`struct PgSqlIdentifiers`](../pg_crud_common/src/operational_invariants.rs#L24)
- [`struct PgScopedForeignKey`](../pg_crud_common/src/operational_invariants.rs#L28)
- [`enum PgDuplicateIdentifierPresence`](../pg_crud_common/src/operational_invariants.rs#L36)
- [`struct PgScopedForeignKeyClauseText`](../pg_crud_common/src/operational_invariants.rs#L42)
- [`struct PgCounterValue`](../pg_crud_common/src/operational_invariants.rs#L101)
- [`enum PgCounterReconciliation`](../pg_crud_common/src/operational_invariants.rs#L104)
- [`struct PgOperationalLimit`](../pg_crud_common/src/operational_invariants.rs#L113)
- [`enum PgOperationalLimitUpdateAuthority`](../pg_crud_common/src/operational_invariants.rs#L128)
- [`enum PgOperationalLimitError`](../pg_crud_common/src/operational_invariants.rs#L136)

## `pg_crud_common/src/order_preserving_deduplication.rs`

- [`enum SliceOrdering`](../pg_crud_common/src/order_preserving_deduplication.rs#L2)
- [`struct OrderPreservingValues`](../pg_crud_common/src/order_preserving_deduplication.rs#L16)

## `pg_crud_common/src/pagination.rs`

- [`struct PaginationLimit`](../pg_crud_common/src/pagination.rs#L20)
- [`struct PaginationPolicy`](../pg_crud_common/src/pagination.rs#L27)
- [`struct PaginationOffset`](../pg_crud_common/src/pagination.rs#L72)
- [`struct PaginationStart`](../pg_crud_common/src/pagination.rs#L90)
- [`struct PaginationEnd`](../pg_crud_common/src/pagination.rs#L103)

## `pg_crud_common/src/patch_field.rs`

- [`enum PatchField`](../pg_crud_common/src/patch_field.rs#L11)

## `pg_crud_common/src/pg_error.rs`

- [`enum PgErrorKind`](../pg_crud_common/src/pg_error.rs#L2)
- [`struct SqlxPgErrorRef`](../pg_crud_common/src/pg_error.rs#L18)

## `pg_crud_common/src/pg_values.rs`

- [`enum EqOperator`](../pg_crud_common/src/pg_values.rs#L2)
- [`struct EqOperatorQueryStr`](../pg_crud_common/src/pg_values.rs#L26)
- [`trait PgTypeEqOperator`](../pg_crud_common/src/pg_values.rs#L27)
- [`struct UnsignedPartOfI32`](../pg_crud_common/src/pg_values.rs#L44)
- [`enum UnsignedPartOfI32TryFromI32Error`](../pg_crud_common/src/pg_values.rs#L62)
- [`struct UnsignedPartOfI32Raw`](../pg_crud_common/src/pg_values.rs#L85)
- [`struct NotZeroUnsignedPartOfI32`](../pg_crud_common/src/pg_values.rs#L152)
- [`enum NotZeroUnsignedPartOfI32TryFromI32Error`](../pg_crud_common/src/pg_values.rs#L180)
- [`enum SingleOrMultiple`](../pg_crud_common/src/pg_values.rs#L250)
- [`struct UuidUuidTestCases`](../pg_crud_common/src/pg_values.rs#L305)

## `pg_crud_common/src/query_collections.rs`

- [`struct V`](../pg_crud_common/src/query_collections.rs#L12)
- [`struct IsStringEmptyRes`](../pg_crud_common/src/query_collections.rs#L45)
- [`trait IsStringEmpty`](../pg_crud_common/src/query_collections.rs#L46)
- [`enum NotEmptyUniqueVecTryNewError`](../pg_crud_common/src/query_collections.rs#L58)
- [`struct NotEmptyUniqueVec`](../pg_crud_common/src/query_collections.rs#L79)
- [`struct __Visitor`](../pg_crud_common/src/query_collections.rs#L151)
- [`struct NonPrimaryKeyPgTypeReadIds`](../pg_crud_common/src/query_collections.rs#L397)

## `pg_crud_common/src/query_fragment.rs`

- [`struct QueryPartFragment`](../pg_crud_common/src/query_fragment.rs#L16)
- [`struct ReadQueryBindIndexNonZeroU32`](../pg_crud_common/src/query_fragment.rs#L26)
- [`struct SqlColumnRef`](../pg_crud_common/src/query_fragment.rs#L94)

## `pg_crud_common/src/query_pagination.rs`

- [`enum Order`](../pg_crud_common/src/query_pagination.rs#L15)
- [`struct OrderSnakeCaseStr`](../pg_crud_common/src/query_pagination.rs#L38)
- [`struct OrderUpperCamelCaseStr`](../pg_crud_common/src/query_pagination.rs#L59)
- [`struct OrderBy`](../pg_crud_common/src/query_pagination.rs#L96)
- [`struct PaginationBase`](../pg_crud_common/src/query_pagination.rs#L135)
- [`struct PaginationStartsWithZeroRaw`](../pg_crud_common/src/query_pagination.rs#L215)
- [`struct PaginationStartsWithZero`](../pg_crud_common/src/query_pagination.rs#L234)
- [`enum PaginationStartsWithZeroTryNewError`](../pg_crud_common/src/query_pagination.rs#L245)

## `pg_crud_common/src/read_query_plan.rs`

- [`enum QuerySortOrder`](../pg_crud_common/src/read_query_plan.rs#L2)
- [`struct SqlSortOrderText`](../pg_crud_common/src/read_query_plan.rs#L25)
- [`struct ReadQueryPlan`](../pg_crud_common/src/read_query_plan.rs#L36)
- [`struct ReadQueryPlanError`](../pg_crud_common/src/read_query_plan.rs#L42)

## `pg_crud_common/src/rollback.rs`

- [`enum TransactionFailure`](../pg_crud_common/src/rollback.rs#L2)

## `pg_crud_common/src/sql_identifier.rs`

- [`struct SqlIdentifier`](../pg_crud_common/src/sql_identifier.rs#L13)
- [`enum SqlIdentifierError`](../pg_crud_common/src/sql_identifier.rs#L33)
- [`struct SqlQualifiedIdentifier`](../pg_crud_common/src/sql_identifier.rs#L40)
- [`struct SqlIdentifierListText`](../pg_crud_common/src/sql_identifier.rs#L45)
- [`struct SqlIdentifiers`](../pg_crud_common/src/sql_identifier.rs#L60)
- [`struct SqlQueryText`](../pg_crud_common/src/sql_identifier.rs#L88)
- [`struct SqlSelectBuilder`](../pg_crud_common/src/sql_identifier.rs#L126)

## `pg_crud_common/src/sql_like_pattern.rs`

- [`enum SqlLikeMatchMode`](../pg_crud_common/src/sql_like_pattern.rs#L2)
- [`struct SqlLikeInputRef`](../pg_crud_common/src/sql_like_pattern.rs#L9)
- [`struct SqlLikePattern`](../pg_crud_common/src/sql_like_pattern.rs#L24)
- [`struct SqlLikePatternError`](../pg_crud_common/src/sql_like_pattern.rs#L39)

## `pg_crud_macros_common/src/filters.rs`

- [`enum PgTypeFilter`](../pg_crud_macros_common/src/filters.rs#L9)
- [`trait PgFilter`](../pg_crud_macros_common/src/filters.rs#L162)

## `pg_crud_macros_common/src/lib.rs`

- [`struct NamesCtx`](../pg_crud_macros_common/src/lib.rs#L7)
- [`enum DeriveOrImpl`](../pg_crud_macros_common/src/lib.rs#L181)
- [`struct ProcMacro2GeneratedRustTokenStreamVec`](../pg_crud_macros_common/src/lib.rs#L186)
- [`struct NonNullOrNullableStr`](../pg_crud_macros_common/src/lib.rs#L216)
- [`struct IsNullablePrefixStr`](../pg_crud_macros_common/src/lib.rs#L225)
- [`struct ImportSnakeCaseStr`](../pg_crud_macros_common/src/lib.rs#L235)
- [`struct ImportPathStr`](../pg_crud_macros_common/src/lib.rs#L244)
- [`struct DimensionNumber`](../pg_crud_macros_common/src/lib.rs#L253)
- [`struct StructElsLen`](../pg_crud_macros_common/src/lib.rs#L262)
- [`struct DeLen`](../pg_crud_macros_common/src/lib.rs#L271)
- [`struct WrapIntoBraces`](../pg_crud_macros_common/src/lib.rs#L280)
- [`struct ParseTokenStreamStrings`](../pg_crud_macros_common/src/lib.rs#L282)
- [`struct ParseErrorIdRef`](../pg_crud_macros_common/src/lib.rs#L311)
- [`struct PanicUuidRef`](../pg_crud_macros_common/src/lib.rs#L320)
- [`struct SynIdentifierTypeRefs`](../pg_crud_macros_common/src/lib.rs#L322)
- [`struct SynFieldRefs`](../pg_crud_macros_common/src/lib.rs#L324)
- [`enum IsStandardNonNull`](../pg_crud_macros_common/src/lib.rs#L326)
- [`enum IsNullable`](../pg_crud_macros_common/src/lib.rs#L344)
- [`enum Import`](../pg_crud_macros_common/src/lib.rs#L397)
- [`macro-generated type AddOperatorUndrscr`](../pg_crud_macros_common/src/lib.rs#L464)
- [`macro-generated type ColumnParameterUndrscr`](../pg_crud_macros_common/src/lib.rs#L465)
- [`macro-generated type IncrementParameterUndrscr`](../pg_crud_macros_common/src/lib.rs#L466)
- [`macro-generated type IsCreateQueryBindMut`](../pg_crud_macros_common/src/lib.rs#L467)
- [`macro-generated type IsQueryBindMut`](../pg_crud_macros_common/src/lib.rs#L468)
- [`macro-generated type IsSelectOnlyCreatedIdsQueryBindMut`](../pg_crud_macros_common/src/lib.rs#L469)
- [`macro-generated type IsSelectOnlyUpdatedIdsQueryBindMut`](../pg_crud_macros_common/src/lib.rs#L470)
- [`macro-generated type IsSelectQueryPartColumnFieldForErrorMessageUsed`](../pg_crud_macros_common/src/lib.rs#L471)
- [`macro-generated type IsSelectQueryPartIsPgTypeUsed`](../pg_crud_macros_common/src/lib.rs#L472)
- [`macro-generated type IsSelectQueryPartSelfSelectUsed`](../pg_crud_macros_common/src/lib.rs#L473)
- [`macro-generated type IsUpdateQueryBindMut`](../pg_crud_macros_common/src/lib.rs#L474)
- [`macro-generated type IsUpdateQueryPartSelfUpdateUsed`](../pg_crud_macros_common/src/lib.rs#L475)
- [`macro-generated type ShouldDSchemarsJsonSchema`](../pg_crud_macros_common/src/lib.rs#L476)
- [`macro-generated type ShouldDeriveUtoipaToSchema`](../pg_crud_macros_common/src/lib.rs#L477)
- [`enum ReadOrUpdate`](../pg_crud_macros_common/src/lib.rs#L479)
- [`macro-generated type IsPrimaryKeyUndrscr`](../pg_crud_macros_common/src/lib.rs#L492)
- [`enum DefaultSomeOneOrDefaultSomeOneWithMaxPageSize`](../pg_crud_macros_common/src/lib.rs#L494)
- [`enum EqOrEqUsingFields`](../pg_crud_macros_common/src/lib.rs#L499)
- [`enum EqOperatorHandle`](../pg_crud_macros_common/src/lib.rs#L504)
- [`enum Dimension`](../pg_crud_macros_common/src/lib.rs#L527)
- [`enum DimensionIndexNumber`](../pg_crud_macros_common/src/lib.rs#L535)
- [`macro-generated type CreateQueryBindValueUndrscr`](../pg_crud_macros_common/src/lib.rs#L551)
- [`macro-generated type CreateQueryPartIncrementUndrscr`](../pg_crud_macros_common/src/lib.rs#L552)
- [`macro-generated type CreateQueryPartValueUndrscr`](../pg_crud_macros_common/src/lib.rs#L553)
- [`macro-generated type SelectQueryPartValueUndrscr`](../pg_crud_macros_common/src/lib.rs#L554)
- [`macro-generated type UpdateQueryPartAccumulatorUndrscr`](../pg_crud_macros_common/src/lib.rs#L555)
- [`macro-generated type UpdateQueryPartPathUndrscr`](../pg_crud_macros_common/src/lib.rs#L556)
- [`macro-generated type UpdateQueryPartTargetUndrscr`](../pg_crud_macros_common/src/lib.rs#L557)
- [`macro-generated type UpdateQueryPartValueUndrscr`](../pg_crud_macros_common/src/lib.rs#L558)

## `pg_crud_pg_table/src/lib.rs`

- [`trait CombinationOfAppStateLogicTraits`](../pg_crud_pg_table/src/lib.rs#L3)
- [`struct PgTableIdempotencyActor`](../pg_crud_pg_table/src/lib.rs#L18)
- [`struct PgTableIdempotencyKey`](../pg_crud_pg_table/src/lib.rs#L22)
- [`struct PgTableIdempotencyMethod`](../pg_crud_pg_table/src/lib.rs#L24)
- [`struct PgTableIdempotencyRoute`](../pg_crud_pg_table/src/lib.rs#L26)
- [`struct PgTableIdempotencyRequestHash`](../pg_crud_pg_table/src/lib.rs#L36)
- [`struct PgTableIdempotencyBody`](../pg_crud_pg_table/src/lib.rs#L40)
- [`struct PgTableIdempotencyBodyError`](../pg_crud_pg_table/src/lib.rs#L51)
- [`struct PgTableIdempotencyBodyRef`](../pg_crud_pg_table/src/lib.rs#L71)
- [`struct PgTableIdempotencyResponseStatus`](../pg_crud_pg_table/src/lib.rs#L86)
- [`enum PgTableIdempotencyKnownResponseStatus`](../pg_crud_pg_table/src/lib.rs#L88)
- [`struct PgTableIdempotencyResponseStatusTryFromU16Error`](../pg_crud_pg_table/src/lib.rs#L116)
- [`struct PgTableIdempotencyTextBytes`](../pg_crud_pg_table/src/lib.rs#L127)
- [`struct PgTableIdempotencyCleanupRetentionSeconds`](../pg_crud_pg_table/src/lib.rs#L135)
- [`struct PgTableIdempotencyCleanupBatchSize`](../pg_crud_pg_table/src/lib.rs#L153)
- [`enum PgTableIdempotencyCleanupValueTryFromI64Error`](../pg_crud_pg_table/src/lib.rs#L167)
- [`struct PgTableIdempotencyCleanupRows`](../pg_crud_pg_table/src/lib.rs#L183)
- [`struct SqlxPgTablePgConnectionRef`](../pg_crud_pg_table/src/lib.rs#L185)
- [`struct PgTableRevision`](../pg_crud_pg_table/src/lib.rs#L197)
- [`struct PgTableRevisionParseIntError`](../pg_crud_pg_table/src/lib.rs#L202)
- [`enum PgTableRevisionTryFromStringError`](../pg_crud_pg_table/src/lib.rs#L204)
- [`struct PgTableIdempotencyScope`](../pg_crud_pg_table/src/lib.rs#L224)
- [`struct PgTableIdempotencyRequest`](../pg_crud_pg_table/src/lib.rs#L231)
- [`struct PgTableIdempotencyReplay`](../pg_crud_pg_table/src/lib.rs#L236)
- [`enum PgTableIdempotencyBegin`](../pg_crud_pg_table/src/lib.rs#L241)
- [`enum PgTableIdempotencyTextError`](../pg_crud_pg_table/src/lib.rs#L250)
- [`struct SqlxPgTableIdempotencyError`](../pg_crud_pg_table/src/lib.rs#L267)
- [`enum InsertValuesFmt`](../pg_crud_pg_table/src/lib.rs#L637)
- [`enum SelectWhereFmt`](../pg_crud_pg_table/src/lib.rs#L642)
- [`enum UpdateSelectorFmt`](../pg_crud_pg_table/src/lib.rs#L647)
- [`struct PgTableNameRef`](../pg_crud_pg_table/src/lib.rs#L659)
- [`struct PgTableSqlFragmentRef`](../pg_crud_pg_table/src/lib.rs#L676)
- [`struct PgTableQueryString`](../pg_crud_pg_table/src/lib.rs#L688)
- [`enum PgTableStringWrapperTryFromStringError`](../pg_crud_pg_table/src/lib.rs#L690)
- [`struct PgTableQueryPartFragment`](../pg_crud_pg_table/src/lib.rs#L725)

## `pg_crud_pg_table_generate_src/src/model.rs`

- [`struct GeneratePgTableFieldCount`](../pg_crud_pg_table_generate_src/src/model.rs#L12)
- [`struct GeneratePgTableModel`](../pg_crud_pg_table_generate_src/src/model.rs#L15)
- [`struct SynGeneratePgTableModelInput`](../pg_crud_pg_table_generate_src/src/model.rs#L22)
- [`struct SynGeneratePgTableModelError`](../pg_crud_pg_table_generate_src/src/model.rs#L26)
- [`struct OperationDsc`](../pg_crud_pg_table_generate_src/src/model.rs#L60)

## `pg_crud_pg_table_generate_src/src/pipeline.rs`

- [`struct SynParsedGeneratePgTableInput`](../pg_crud_pg_table_generate_src/src/pipeline.rs#L2)
- [`struct SynBuiltGeneratePgTableInput`](../pg_crud_pg_table_generate_src/src/pipeline.rs#L5)
- [`struct SynValidatedGeneratePgTableInput`](../pg_crud_pg_table_generate_src/src/pipeline.rs#L15)
- [`struct SynGeneratePgTablePipelineError`](../pg_crud_pg_table_generate_src/src/pipeline.rs#L31)
- [`enum GeneratePgTablePipelineError`](../pg_crud_pg_table_generate_src/src/pipeline.rs#L34)

## `pg_crud_pg_table_generate_src/src/source.rs`

- [`struct CompileErrorMessage`](../pg_crud_pg_table_generate_src/src/source.rs#L2)
- [`struct TableTestNames`](../pg_crud_pg_table_generate_src/src/source.rs#L10)
- [`struct SynVariant`](../pg_crud_pg_table_generate_src/src/source.rs#L75)
- [`enum AddBorrow`](../pg_crud_pg_table_generate_src/src/source.rs#L90)
- [`enum AddReturn`](../pg_crud_pg_table_generate_src/src/source.rs#L103)
- [`enum Operation`](../pg_crud_pg_table_generate_src/src/source.rs#L116)
- [`enum OperationHttpMethod`](../pg_crud_pg_table_generate_src/src/source.rs#L304)
- [`enum OperationKind`](../pg_crud_pg_table_generate_src/src/source.rs#L310)
- [`enum RmOrDm`](../pg_crud_pg_table_generate_src/src/source.rs#L374)
- [`enum RmOrRo`](../pg_crud_pg_table_generate_src/src/source.rs#L379)
- [`enum GeneratePgTableAttr`](../pg_crud_pg_table_generate_src/src/source.rs#L395)
- [`enum ShouldWrapIntoV`](../pg_crud_pg_table_generate_src/src/source.rs#L441)
- [`enum CreateOrUpdateOrDm`](../pg_crud_pg_table_generate_src/src/source.rs#L447)
- [`enum CreateOrUpdateOrDlo`](../pg_crud_pg_table_generate_src/src/source.rs#L454)
- [`struct GeneratePgTableConfig`](../pg_crud_pg_table_generate_src/src/source.rs#L461)
- [`struct GeneratePgTableDbForeignKey`](../pg_crud_pg_table_generate_src/src/source.rs#L488)
- [`struct GeneratePgTableDbColumn`](../pg_crud_pg_table_generate_src/src/source.rs#L503)
- [`struct GeneratePgTableExcludeField`](../pg_crud_pg_table_generate_src/src/source.rs#L513)
- [`enum GeneratePgTableApiMode`](../pg_crud_pg_table_generate_src/src/source.rs#L535)
- [`struct StdBulkItemsMax`](../pg_crud_pg_table_generate_src/src/source.rs#L548)
- [`struct UsizeGeneratePgTableDbColumns`](../pg_crud_pg_table_generate_src/src/source.rs#L552)
- [`struct UsizeCreateExcludeFields`](../pg_crud_pg_table_generate_src/src/source.rs#L561)
- [`struct UsizeReadExcludeFields`](../pg_crud_pg_table_generate_src/src/source.rs#L570)
- [`struct GeneratePgTableEmissionModel`](../pg_crud_pg_table_generate_src/src/source.rs#L602)
- [`struct ProcMacro2GeneratePgTableTestsTokenStream`](../pg_crud_pg_table_generate_src/src/source.rs#L610)
- [`struct ProcMacro2GeneratePgTableCommonTokenStream`](../pg_crud_pg_table_generate_src/src/source.rs#L621)
- [`struct ProcMacro2GeneratePgTableWholeTokenStream`](../pg_crud_pg_table_generate_src/src/source.rs#L629)
- [`struct SynGeneratePgTableDeriveInput`](../pg_crud_pg_table_generate_src/src/source.rs#L640)
- [`struct GeneratePgTableFieldEmissionModel`](../pg_crud_pg_table_generate_src/src/source.rs#L648)
- [`struct GeneratePgTableFrontendFieldEmission`](../pg_crud_pg_table_generate_src/src/source.rs#L656)
- [`enum GeneratePgTableFrontendFlag`](../pg_crud_pg_table_generate_src/src/source.rs#L667)
- [`struct GeneratePgTableVariantFieldEmission`](../pg_crud_pg_table_generate_src/src/source.rs#L674)
- [`struct GeneratePgTableVariantEmission`](../pg_crud_pg_table_generate_src/src/source.rs#L680)
- [`enum GeneratePgTableVariantEmissionRef`](../pg_crud_pg_table_generate_src/src/source.rs#L685)
- [`struct GeneratePgTableFieldIdx`](../pg_crud_pg_table_generate_src/src/source.rs#L698)
- [`struct GeneratePgTableFieldsEmissionModel`](../pg_crud_pg_table_generate_src/src/source.rs#L706)
- [`struct SynGeneratePgTableFieldRef`](../pg_crud_pg_table_generate_src/src/source.rs#L714)
- [`struct SynGeneratePgTableIdentifierRef`](../pg_crud_pg_table_generate_src/src/source.rs#L722)
- [`struct SynGeneratePgTableTypeRef`](../pg_crud_pg_table_generate_src/src/source.rs#L730)
- [`struct GeneratePgTableVariantLocationAttr`](../pg_crud_pg_table_generate_src/src/source.rs#L738)
- [`struct GeneratePgTablePrimaryKeyAttrName`](../pg_crud_pg_table_generate_src/src/source.rs#L748)
- [`enum WrapIntoOptional`](../pg_crud_pg_table_generate_src/src/source.rs#L3476)
- [`enum AddDotClone`](../pg_crud_pg_table_generate_src/src/source.rs#L8072)

## `pg_crud_pg_types_chrono_net/src/lib.rs`

- [`macro-generated struct OptionalSqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNullableTimestampTzRange`](../pg_crud_pg_types_chrono_net/src/lib.rs#L1)
- [`macro-generated struct OptionalSqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsNullableDateRange`](../pg_crud_pg_types_chrono_net/src/lib.rs#L1)
- [`macro-generated struct OptionalSqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsNullableTimestampRange`](../pg_crud_pg_types_chrono_net/src/lib.rs#L1)
- [`macro-generated struct OptionalSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNullableTimestampTz`](../pg_crud_pg_types_chrono_net/src/lib.rs#L1)
- [`macro-generated struct OptionalSqlxTypesChronoNaiveDateAsNullableDate`](../pg_crud_pg_types_chrono_net/src/lib.rs#L1)
- [`macro-generated struct OptionalSqlxTypesChronoNaiveDateTimeAsNullableTimestamp`](../pg_crud_pg_types_chrono_net/src/lib.rs#L1)
- [`macro-generated struct OptionalSqlxTypesChronoNaiveTimeAsNullableTime`](../pg_crud_pg_types_chrono_net/src/lib.rs#L1)
- [`macro-generated struct OptionalSqlxTypesIpnetworkIpNetworkAsNullableInet`](../pg_crud_pg_types_chrono_net/src/lib.rs#L1)
- [`macro-generated struct OptionalSqlxTypesMacAddressMacAddressAsNullableMacAddr`](../pg_crud_pg_types_chrono_net/src/lib.rs#L1)
- [`macro-generated struct SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNonNullTimestampTzRange`](../pg_crud_pg_types_chrono_net/src/lib.rs#L1)
- [`macro-generated struct SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsNonNullDateRange`](../pg_crud_pg_types_chrono_net/src/lib.rs#L1)
- [`macro-generated struct SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsNonNullTimestampRange`](../pg_crud_pg_types_chrono_net/src/lib.rs#L1)
- [`macro-generated struct SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNonNullTimestampTz`](../pg_crud_pg_types_chrono_net/src/lib.rs#L1)
- [`macro-generated struct SqlxTypesChronoNaiveDateAsNonNullDate`](../pg_crud_pg_types_chrono_net/src/lib.rs#L1)
- [`macro-generated struct SqlxTypesChronoNaiveDateTimeAsNonNullTimestamp`](../pg_crud_pg_types_chrono_net/src/lib.rs#L1)
- [`macro-generated struct SqlxTypesChronoNaiveTimeAsNonNullTime`](../pg_crud_pg_types_chrono_net/src/lib.rs#L1)
- [`macro-generated struct SqlxTypesIpnetworkIpNetworkAsNonNullInet`](../pg_crud_pg_types_chrono_net/src/lib.rs#L1)
- [`macro-generated struct SqlxTypesMacAddressMacAddressAsNonNullMacAddr`](../pg_crud_pg_types_chrono_net/src/lib.rs#L1)

## `pg_crud_pg_types_common/src/domain_types.rs`

- [`struct PaginationStartsWithOneRaw`](../pg_crud_pg_types_common/src/domain_types.rs#L4)
- [`struct PaginationStartsWithOneValue`](../pg_crud_pg_types_common/src/domain_types.rs#L27)
- [`struct IsPrimaryKey`](../pg_crud_pg_types_common/src/domain_types.rs#L38)
- [`struct PaginationStartsWithOne`](../pg_crud_pg_types_common/src/domain_types.rs#L59)
- [`enum PaginationStartsWithOneTryNewError`](../pg_crud_pg_types_common/src/domain_types.rs#L70)

## `pg_crud_pg_types_generate_src/src/model.rs`

- [`struct PgTypeSpec`](../pg_crud_pg_types_generate_src/src/model.rs#L3)

## `pg_crud_pg_types_generate_src/src/source.rs`

- [`enum RustTypeName`](../pg_crud_pg_types_generate_src/src/source.rs#L6)
- [`enum PgTypeName`](../pg_crud_pg_types_generate_src/src/source.rs#L62)
- [`enum PgType`](../pg_crud_pg_types_generate_src/src/source.rs#L136)
- [`enum WireKind`](../pg_crud_pg_types_generate_src/src/source.rs#L167)
- [`enum FilterKind`](../pg_crud_pg_types_generate_src/src/source.rs#L192)
- [`enum CanBePrimaryKey`](../pg_crud_pg_types_generate_src/src/source.rs#L208)
- [`struct PgSqlName`](../pg_crud_pg_types_generate_src/src/source.rs#L220)
- [`enum CanBeNullable`](../pg_crud_pg_types_generate_src/src/source.rs#L431)
- [`enum Range`](../pg_crud_pg_types_generate_src/src/source.rs#L457)
- [`enum PgTypePattern`](../pg_crud_pg_types_generate_src/src/source.rs#L528)
- [`struct PgTypeRecord`](../pg_crud_pg_types_generate_src/src/source.rs#L544)
- [`struct PgTypeRecordRaw`](../pg_crud_pg_types_generate_src/src/source.rs#L551)
- [`struct GeneratePgTypeRecords`](../pg_crud_pg_types_generate_src/src/source.rs#L587)
- [`struct GeneratePgTypes`](../pg_crud_pg_types_generate_src/src/source.rs#L592)
- [`struct GeneratePgTypesLengthError`](../pg_crud_pg_types_generate_src/src/source.rs#L597)
- [`enum GeneratePgTypesConfigVariant`](../pg_crud_pg_types_generate_src/src/source.rs#L621)
- [`struct GenerateSecretText`](../pg_crud_pg_types_generate_src/src/source.rs#L631)
- [`struct GeneratePgTypesConfig`](../pg_crud_pg_types_generate_src/src/source.rs#L635)
- [`enum PgTypeInitializationTryNew`](../pg_crud_pg_types_generate_src/src/source.rs#L644)
- [`enum PgTypeImplTryNewForDe`](../pg_crud_pg_types_generate_src/src/source.rs#L712)
- [`enum PgTypeImplNewForDeserializeOrTryNewForDe`](../pg_crud_pg_types_generate_src/src/source.rs#L723)
- [`enum PgTypeDeserialize`](../pg_crud_pg_types_generate_src/src/source.rs#L728)
- [`struct ParsedGeneratePgTypesConfig`](../pg_crud_pg_types_generate_src/src/source.rs#L766)
- [`struct BuiltGeneratePgTypesModel`](../pg_crud_pg_types_generate_src/src/source.rs#L769)
- [`struct ValidatedGeneratePgTypesConfig`](../pg_crud_pg_types_generate_src/src/source.rs#L774)
- [`struct PgTypesModelEntryCount`](../pg_crud_pg_types_generate_src/src/source.rs#L788)
- [`struct SerdeJsonGeneratePgTypesError`](../pg_crud_pg_types_generate_src/src/source.rs#L799)
- [`enum GeneratePgTypesPipelineError`](../pg_crud_pg_types_generate_src/src/source.rs#L802)
- [`enum PgTypeOrPgTypeTestCases`](../pg_crud_pg_types_generate_src/src/source.rs#L1039)
- [`enum IsNonNullStandardCanBePrimaryKey`](../pg_crud_pg_types_generate_src/src/source.rs#L1044)
- [`enum StartOrEnd`](../pg_crud_pg_types_generate_src/src/source.rs#L1049)
- [`enum ShouldImplFrom`](../pg_crud_pg_types_generate_src/src/source.rs#L1054)
- [`enum IntRangeType`](../pg_crud_pg_types_generate_src/src/source.rs#L1059)
- [`enum ParameterNumber`](../pg_crud_pg_types_generate_src/src/source.rs#L1521)
- [`enum DateOrTime`](../pg_crud_pg_types_generate_src/src/source.rs#L1697)
- [`enum DateNaiveOrTime`](../pg_crud_pg_types_generate_src/src/source.rs#L1732)
- [`enum IsConst`](../pg_crud_pg_types_generate_src/src/source.rs#L1944)
- [`enum IsNeedToUseInto`](../pg_crud_pg_types_generate_src/src/source.rs#L4519)
- [`enum Bnd`](../pg_crud_pg_types_generate_src/src/source.rs#L4538)
- [`enum IsNeedToImplPgTypeGreaterThanTest`](../pg_crud_pg_types_generate_src/src/source.rs#L5291)
- [`enum CreateReadIds`](../pg_crud_pg_types_generate_src/src/source.rs#L5297)

## `pg_crud_pg_types_numeric/src/lib.rs`

- [`macro-generated struct BoolAsNonNullBool`](../pg_crud_pg_types_numeric/src/lib.rs#L1)
- [`macro-generated struct F32AsNonNullFloat4`](../pg_crud_pg_types_numeric/src/lib.rs#L1)
- [`macro-generated struct F64AsNonNullFloat8`](../pg_crud_pg_types_numeric/src/lib.rs#L1)
- [`macro-generated struct I16AsNonNullInt2`](../pg_crud_pg_types_numeric/src/lib.rs#L1)
- [`macro-generated struct I16AsNonNullSmallSerialInitializationByPg`](../pg_crud_pg_types_numeric/src/lib.rs#L1)
- [`macro-generated struct I32AsNonNullInt4`](../pg_crud_pg_types_numeric/src/lib.rs#L1)
- [`macro-generated struct I32AsNonNullSerialInitializationByPg`](../pg_crud_pg_types_numeric/src/lib.rs#L1)
- [`macro-generated struct I64AsNonNullBigSerialInitializationByPg`](../pg_crud_pg_types_numeric/src/lib.rs#L1)
- [`macro-generated struct I64AsNonNullInt8`](../pg_crud_pg_types_numeric/src/lib.rs#L1)
- [`macro-generated struct OptionalBoolAsNullableBool`](../pg_crud_pg_types_numeric/src/lib.rs#L1)
- [`macro-generated struct OptionalF32AsNullableFloat4`](../pg_crud_pg_types_numeric/src/lib.rs#L1)
- [`macro-generated struct OptionalF64AsNullableFloat8`](../pg_crud_pg_types_numeric/src/lib.rs#L1)
- [`macro-generated struct OptionalI16AsNullableInt2`](../pg_crud_pg_types_numeric/src/lib.rs#L1)
- [`macro-generated struct OptionalI16AsNullableSmallSerialInitializationByPg`](../pg_crud_pg_types_numeric/src/lib.rs#L1)
- [`macro-generated struct OptionalI32AsNullableInt4`](../pg_crud_pg_types_numeric/src/lib.rs#L1)
- [`macro-generated struct OptionalI32AsNullableSerialInitializationByPg`](../pg_crud_pg_types_numeric/src/lib.rs#L1)
- [`macro-generated struct OptionalI64AsNullableBigSerialInitializationByPg`](../pg_crud_pg_types_numeric/src/lib.rs#L1)
- [`macro-generated struct OptionalI64AsNullableInt8`](../pg_crud_pg_types_numeric/src/lib.rs#L1)
- [`macro-generated struct OptionalSqlxPgTypesPgMoneyAsNullableMoney`](../pg_crud_pg_types_numeric/src/lib.rs#L1)
- [`macro-generated struct OptionalSqlxPgTypesPgRangeI32AsNullableInt4Range`](../pg_crud_pg_types_numeric/src/lib.rs#L1)
- [`macro-generated struct OptionalSqlxPgTypesPgRangeI64AsNullableInt8Range`](../pg_crud_pg_types_numeric/src/lib.rs#L1)
- [`macro-generated struct SqlxPgTypesPgMoneyAsNonNullMoney`](../pg_crud_pg_types_numeric/src/lib.rs#L1)
- [`macro-generated struct SqlxPgTypesPgRangeI32AsNonNullInt4Range`](../pg_crud_pg_types_numeric/src/lib.rs#L1)
- [`macro-generated struct SqlxPgTypesPgRangeI64AsNonNullInt8Range`](../pg_crud_pg_types_numeric/src/lib.rs#L1)

## `pg_crud_pg_types_text_misc/src/lib.rs`

- [`macro-generated struct OptionalSqlxPgTypesPgIntervalAsNullableInterval`](../pg_crud_pg_types_text_misc/src/lib.rs#L1)
- [`macro-generated struct OptionalSqlxTypesTimeTimeAsNullableTime`](../pg_crud_pg_types_text_misc/src/lib.rs#L1)
- [`macro-generated struct OptionalSqlxTypesUuidUuidAsNullableUuidInitializationByClient`](../pg_crud_pg_types_text_misc/src/lib.rs#L1)
- [`macro-generated struct OptionalSqlxTypesUuidUuidAsNullableUuidV4InitializationByPg`](../pg_crud_pg_types_text_misc/src/lib.rs#L1)
- [`macro-generated struct OptionalStdVecVecU8AsNullableBytea`](../pg_crud_pg_types_text_misc/src/lib.rs#L1)
- [`macro-generated struct OptionalStringAsNullableText`](../pg_crud_pg_types_text_misc/src/lib.rs#L1)
- [`macro-generated struct SqlxPgTypesPgIntervalAsNonNullInterval`](../pg_crud_pg_types_text_misc/src/lib.rs#L1)
- [`macro-generated struct SqlxTypesTimeTimeAsNonNullTime`](../pg_crud_pg_types_text_misc/src/lib.rs#L1)
- [`macro-generated struct SqlxTypesUuidUuidAsNonNullUuidInitializationByClient`](../pg_crud_pg_types_text_misc/src/lib.rs#L1)
- [`macro-generated struct SqlxTypesUuidUuidAsNonNullUuidV4InitializationByPg`](../pg_crud_pg_types_text_misc/src/lib.rs#L1)
- [`macro-generated struct StdVecVecU8AsNonNullBytea`](../pg_crud_pg_types_text_misc/src/lib.rs#L1)
- [`macro-generated struct StringAsNonNullText`](../pg_crud_pg_types_text_misc/src/lib.rs#L1)

## `pg_crud_where_filters/src/lib.rs`

- [`enum EncodeFormat`](../pg_crud_where_filters/src/lib.rs#L18)
- [`struct RegexRegex`](../pg_crud_where_filters/src/lib.rs#L51)
- [`struct DefaultRegexPattern`](../pg_crud_where_filters/src/lib.rs#L53)
- [`struct RegexError`](../pg_crud_where_filters/src/lib.rs#L63)
- [`enum RegexRegexTryFromStringError`](../pg_crud_where_filters/src/lib.rs#L65)
- [`enum RegexCase`](../pg_crud_where_filters/src/lib.rs#L135)
- [`struct RegexCasePostgreqlSyntax`](../pg_crud_where_filters/src/lib.rs#L150)
- [`struct Between`](../pg_crud_where_filters/src/lib.rs#L175)
- [`enum BetweenTryNewError`](../pg_crud_where_filters/src/lib.rs#L226)
- [`enum __Field`](../pg_crud_where_filters/src/lib.rs#L270)
- [`struct __FieldVisitor`](../pg_crud_where_filters/src/lib.rs#L277)
- [`struct __Visitor`](../pg_crud_where_filters/src/lib.rs#L331)
- [`struct PgTypeNotEmptyUniqueVec`](../pg_crud_where_filters/src/lib.rs#L522)
- [`struct __Visitor`](../pg_crud_where_filters/src/lib.rs#L577)
- [`struct BoundedVec`](../pg_crud_where_filters/src/lib.rs#L658)
- [`enum BoundedVecTryNewError`](../pg_crud_where_filters/src/lib.rs#L677)
- [`struct BoundedVecLen`](../pg_crud_where_filters/src/lib.rs#L700)
- [`enum Variant`](../pg_crud_where_filters/src/lib.rs#L708)

## `pg_crud_where_filters_generate_src/src/bind.rs`

- [`struct FilterPlaceholderCount`](../pg_crud_where_filters_generate_src/src/bind.rs#L11)

## `pg_crud_where_filters_generate_src/src/model.rs`

- [`struct BindCount`](../pg_crud_where_filters_generate_src/src/model.rs#L2)
- [`struct FilterSqlOperator`](../pg_crud_where_filters_generate_src/src/model.rs#L12)
- [`struct FilterSqlSuffix`](../pg_crud_where_filters_generate_src/src/model.rs#L21)
- [`struct FilterSpecValid`](../pg_crud_where_filters_generate_src/src/model.rs#L23)
- [`enum FilterValueShape`](../pg_crud_where_filters_generate_src/src/model.rs#L30)
- [`struct FilterSpec`](../pg_crud_where_filters_generate_src/src/model.rs#L35)

## `pg_crud_where_filters_generate_src/src/source.rs`

- [`struct ProcMacro2GenerateWhereFiltersInput`](../pg_crud_where_filters_generate_src/src/source.rs#L9)
- [`struct ProcMacro2GenerateWhereFiltersTokenStream`](../pg_crud_where_filters_generate_src/src/source.rs#L18)
- [`struct ParsedGenerateWhereFiltersConfig`](../pg_crud_where_filters_generate_src/src/source.rs#L20)
- [`struct BuiltGenerateWhereFiltersModel`](../pg_crud_where_filters_generate_src/src/source.rs#L25)
- [`struct ValidatedGenerateWhereFiltersConfig`](../pg_crud_where_filters_generate_src/src/source.rs#L30)
- [`struct SerdeJsonGenerateWhereFiltersError`](../pg_crud_where_filters_generate_src/src/source.rs#L35)
- [`enum GenerateWhereFiltersPipelineError`](../pg_crud_where_filters_generate_src/src/source.rs#L37)
- [`enum Generic`](../pg_crud_where_filters_generate_src/src/source.rs#L102)
- [`enum PgTypePtrn`](../pg_crud_where_filters_generate_src/src/source.rs#L110)
- [`enum PgTypeKind`](../pg_crud_where_filters_generate_src/src/source.rs#L115)

## `prepare_postgresql_databases/src/domain_types.rs`

- [`struct DatabaseUrl`](../prepare_postgresql_databases/src/domain_types.rs#L11)
- [`enum DatabaseUrlError`](../prepare_postgresql_databases/src/domain_types.rs#L16)
- [`struct MigrationsSource`](../prepare_postgresql_databases/src/domain_types.rs#L33)
- [`enum MigrationsSourceError`](../prepare_postgresql_databases/src/domain_types.rs#L38)
- [`struct DatabasePreparationSpec`](../prepare_postgresql_databases/src/domain_types.rs#L44)
- [`struct ProcessCommand`](../prepare_postgresql_databases/src/domain_types.rs#L60)
- [`enum ProcessArgument`](../prepare_postgresql_databases/src/domain_types.rs#L66)
- [`struct ProcessStaticArgument`](../prepare_postgresql_databases/src/domain_types.rs#L81)
- [`struct ProcessArguments`](../prepare_postgresql_databases/src/domain_types.rs#L120)
- [`struct ProcessCommands`](../prepare_postgresql_databases/src/domain_types.rs#L132)
- [`struct ProcessProgram`](../prepare_postgresql_databases/src/domain_types.rs#L146)

## `route_validators/src/check_body_size.rs`

- [`struct AxumBody`](../route_validators/src/check_body_size.rs#L2)
- [`struct BodySizeLimitBytes`](../route_validators/src/check_body_size.rs#L17)
- [`struct AxumBodySizeError`](../route_validators/src/check_body_size.rs#L21)
- [`struct HttpBodySizeHint`](../route_validators/src/check_body_size.rs#L23)
- [`struct BytesBodyBytes`](../route_validators/src/check_body_size.rs#L40)
- [`enum BodySizeError`](../route_validators/src/check_body_size.rs#L46)

## `route_validators/src/check_commit.rs`

- [`struct CommitNotEqMessage`](../route_validators/src/check_commit.rs#L13)
- [`struct CommitToUse`](../route_validators/src/check_commit.rs#L24)
- [`struct NoCommitHeaderMessage`](../route_validators/src/check_commit.rs#L35)
- [`struct AxumCommitToStrConversionError`](../route_validators/src/check_commit.rs#L39)
- [`struct EnableApiGitCommitCheck`](../route_validators/src/check_commit.rs#L49)
- [`enum CommitError`](../route_validators/src/check_commit.rs#L53)

## `route_validators/src/hdr_val.rs`

- [`struct AxumHeadersRef`](../route_validators/src/hdr_val.rs#L2)
- [`struct AxumHeaderValueRef`](../route_validators/src/hdr_val.rs#L20)
- [`struct HeaderStrRef`](../route_validators/src/hdr_val.rs#L32)

## `route_validators/src/lib.rs`

- [`struct AxumHttpStatusCode`](../route_validators/src/lib.rs#L17)
- [`trait GetAxumHttpStatusCode`](../route_validators/src/lib.rs#L32)

## `route_validators/src/test_hlp.rs`

- [`struct TestExpId`](../route_validators/src/test_hlp.rs#L4)
- [`struct TestPanicText`](../route_validators/src/test_hlp.rs#L6)
- [`struct AxumTestHeaders`](../route_validators/src/test_hlp.rs#L14)
- [`struct AxumTestHeadersMutRef`](../route_validators/src/test_hlp.rs#L17)
- [`struct AxumTestHeaderValue`](../route_validators/src/test_hlp.rs#L25)
- [`struct TestPollCount`](../route_validators/src/test_hlp.rs#L27)
- [`struct TestPollLimitReached`](../route_validators/src/test_hlp.rs#L29)

## `runtime_tests/src/domain_types.rs`

- [`struct ServiceBaseUrl`](../runtime_tests/src/domain_types.rs#L4)
- [`enum ServiceBaseUrlError`](../runtime_tests/src/domain_types.rs#L9)
- [`struct RuntimeTestConfig`](../runtime_tests/src/domain_types.rs#L51)
- [`enum RuntimeTestKind`](../runtime_tests/src/domain_types.rs#L80)
- [`struct RuntimeTestReport`](../runtime_tests/src/domain_types.rs#L103)
- [`struct HttpRuntimeTestStatus`](../runtime_tests/src/domain_types.rs#L128)
- [`struct ReqwestRuntimeTestClient`](../runtime_tests/src/domain_types.rs#L131)
- [`struct ReqwestRuntimeTestResponse`](../runtime_tests/src/domain_types.rs#L168)
- [`struct RuntimeTestUrl`](../runtime_tests/src/domain_types.rs#L207)
- [`enum RuntimeTestError`](../runtime_tests/src/domain_types.rs#L222)

## `server/src/main.rs`

- [`struct ServerIoError`](../server/src/main.rs#L9)
- [`struct ServerRuntimeServeError`](../server/src/main.rs#L14)
- [`struct MetricsExporterPrometheusBuildError`](../server/src/main.rs#L19)
- [`struct MetricsExporterPrometheusHandle`](../server/src/main.rs#L21)
- [`struct ServerRuntimeRequestTimeoutError`](../server/src/main.rs#L26)
- [`struct ServerRuntimeRunIntervalError`](../server/src/main.rs#L31)
- [`struct ServerRuntimeBackgroundTaskShutdownError`](../server/src/main.rs#L36)
- [`struct ServerObservabilityInitError`](../server/src/main.rs#L41)
- [`struct ServerObservabilityShutdownError`](../server/src/main.rs#L46)
- [`struct ServerAdminCleanupCfgError`](../server/src/main.rs#L53)
- [`enum AdminMetricsError`](../server/src/main.rs#L56)
- [`struct ServerConfigError`](../server/src/main.rs#L74)
- [`struct ServerConfigProductionError`](../server/src/main.rs#L79)
- [`struct SqlxServerPgConnectError`](../server/src/main.rs#L84)
- [`struct ServerAdminMigrateError`](../server/src/main.rs#L89)
- [`struct ServerAdminAuthSvcStateBuildError`](../server/src/main.rs#L94)
- [`struct ServerRuntimeContentSecurityPolicyError`](../server/src/main.rs#L99)
- [`struct ServerRuntimeTrustedProxyRangesParseError`](../server/src/main.rs#L104)
- [`struct AxumApiRoutes`](../server/src/main.rs#L106)
- [`struct HttpBodyMaximumBytes`](../server/src/main.rs#L108)
- [`struct SharedServerAppStateArc`](../server/src/main.rs#L112)
- [`struct TokioServerRuntime`](../server/src/main.rs#L123)
- [`struct ServerExitCode`](../server/src/main.rs#L125)
- [`enum RunServerError`](../server/src/main.rs#L132)

## `server_admin/src/auth.rs`

- [`struct JsonwebtokenAdminEncodingKey`](../server_admin/src/auth.rs#L15)
- [`struct JsonwebtokenAdminDecodingKeys`](../server_admin/src/auth.rs#L19)
- [`struct StdAdminAccessTtlSeconds`](../server_admin/src/auth.rs#L34)
- [`struct StdAdminRefreshTtlSeconds`](../server_admin/src/auth.rs#L59)
- [`struct StdAdminSessionLimit`](../server_admin/src/auth.rs#L84)
- [`struct StdAdminFailureThreshold`](../server_admin/src/auth.rs#L109)
- [`struct AdminAuthPositiveValueError`](../server_admin/src/auth.rs#L124)
- [`struct StdAdminFailureDelayMillis`](../server_admin/src/auth.rs#L134)
- [`struct StdAdminRateLimitCount`](../server_admin/src/auth.rs#L143)
- [`struct StdAdminRateLimitWindowSeconds`](../server_admin/src/auth.rs#L152)
- [`struct AdminAuthPolicy`](../server_admin/src/auth.rs#L154)
- [`struct AdminAuthSvcState`](../server_admin/src/auth.rs#L192)
- [`struct SharedAdminAuthSvcStateArc`](../server_admin/src/auth.rs#L213)
- [`enum AdminAuthSvcStateBuildError`](../server_admin/src/auth.rs#L215)
- [`struct AuthenticatedAdmin`](../server_admin/src/auth.rs#L236)
- [`struct AdminAuditQuery`](../server_admin/src/auth.rs#L295)
- [`struct AdminAuditQueryParts`](../server_admin/src/auth.rs#L318)
- [`struct HttpAdminHeaderMap`](../server_admin/src/auth.rs#L360)
- [`struct AdminAuthReq`](../server_admin/src/auth.rs#L362)
- [`struct AdminPeerAddr`](../server_admin/src/auth.rs#L368)
- [`struct AdminSignInJson`](../server_admin/src/auth.rs#L393)
- [`struct AxumAdminJson`](../server_admin/src/auth.rs#L395)
- [`struct AxumAdminForm`](../server_admin/src/auth.rs#L397)
- [`struct AxumAdminPath`](../server_admin/src/auth.rs#L399)
- [`struct AxumAdminQuery`](../server_admin/src/auth.rs#L401)
- [`struct AdminSessionPath`](../server_admin/src/auth.rs#L403)
- [`struct HttpAdminHeaderValueError`](../server_admin/src/auth.rs#L714)
- [`enum AdminObservedErrorCode`](../server_admin/src/auth.rs#L716)
- [`enum AdminError`](../server_admin/src/auth.rs#L741)
- [`struct AxumAdminResponse`](../server_admin/src/auth.rs#L874)
- [`macro-generated type AdminAuditLogError`](../server_admin/src/auth.rs#L933)
- [`macro-generated type AdminAuditExportError`](../server_admin/src/auth.rs#L934)
- [`macro-generated type AdminBrandingError`](../server_admin/src/auth.rs#L935)
- [`macro-generated type AdminChangeOwnPasswordError`](../server_admin/src/auth.rs#L936)
- [`macro-generated type AdminCreateRoleError`](../server_admin/src/auth.rs#L937)
- [`macro-generated type AdminCreateUserError`](../server_admin/src/auth.rs#L938)
- [`macro-generated type AdminDataTableError`](../server_admin/src/auth.rs#L939)
- [`macro-generated type AdminDataTablesError`](../server_admin/src/auth.rs#L940)
- [`macro-generated type AdminDeleteRoleError`](../server_admin/src/auth.rs#L941)
- [`macro-generated type AdminDeleteUserError`](../server_admin/src/auth.rs#L942)
- [`macro-generated type AdminListPermissionsError`](../server_admin/src/auth.rs#L943)
- [`macro-generated type AdminListRolesError`](../server_admin/src/auth.rs#L944)
- [`macro-generated type AdminListUsersError`](../server_admin/src/auth.rs#L945)
- [`macro-generated type AdminMeError`](../server_admin/src/auth.rs#L946)
- [`macro-generated type AdminRefreshError`](../server_admin/src/auth.rs#L947)
- [`macro-generated type AdminRevokeAllSessionsError`](../server_admin/src/auth.rs#L948)
- [`macro-generated type AdminRevokeSessionError`](../server_admin/src/auth.rs#L949)
- [`macro-generated type AdminSessionsError`](../server_admin/src/auth.rs#L950)
- [`macro-generated type AdminSetRolePermissionsError`](../server_admin/src/auth.rs#L951)
- [`macro-generated type AdminSetUserBanError`](../server_admin/src/auth.rs#L952)
- [`macro-generated type AdminSetUserPasswordError`](../server_admin/src/auth.rs#L953)
- [`macro-generated type AdminSetUserRolesError`](../server_admin/src/auth.rs#L954)
- [`macro-generated type AdminSettingsError`](../server_admin/src/auth.rs#L955)
- [`macro-generated type AdminSignInError`](../server_admin/src/auth.rs#L956)
- [`macro-generated type AdminSignOutError`](../server_admin/src/auth.rs#L957)
- [`macro-generated type AdminUpdateRoleError`](../server_admin/src/auth.rs#L958)
- [`macro-generated type AdminUpdateSettingsError`](../server_admin/src/auth.rs#L959)
- [`macro-generated type AdminUpdateUserError`](../server_admin/src/auth.rs#L960)
- [`struct AdminAuditSuccessRef`](../server_admin/src/auth.rs#L983)
- [`enum AdminAuditResourceId`](../server_admin/src/auth.rs#L991)
- [`struct SqlxAdminPgConnectionRef`](../server_admin/src/auth.rs#L1014)
- [`struct AxumAdminAuthRouter`](../server_admin/src/auth.rs#L1128)
- [`struct UtoipaAdminAuthOpenApi`](../server_admin/src/auth.rs#L1132)
- [`struct AdminHtmlSwaggerEnabled`](../server_admin/src/auth.rs#L1151)
- [`struct AdminSessionBundle`](../server_admin/src/auth.rs#L1232)
- [`enum AdminSessionError`](../server_admin/src/auth.rs#L1257)

## `server_admin/src/auth/html.rs`

- [`struct SignInForm`](../server_admin/src/auth/html.rs#L9)
- [`struct ChangePasswordForm`](../server_admin/src/auth/html.rs#L16)
- [`struct RevokeSessionForm`](../server_admin/src/auth/html.rs#L23)
- [`struct CreateUserForm`](../server_admin/src/auth/html.rs#L30)
- [`struct UpdateUserForm`](../server_admin/src/auth/html.rs#L37)
- [`struct UserPasswordForm`](../server_admin/src/auth/html.rs#L44)
- [`struct UserBanForm`](../server_admin/src/auth/html.rs#L50)
- [`struct UserIdForm`](../server_admin/src/auth/html.rs#L56)
- [`struct UserRolesForm`](../server_admin/src/auth/html.rs#L61)
- [`struct CreateRoleForm`](../server_admin/src/auth/html.rs#L69)
- [`struct UpdateRoleForm`](../server_admin/src/auth/html.rs#L74)
- [`struct RoleIdForm`](../server_admin/src/auth/html.rs#L80)
- [`struct RolePermissionsForm`](../server_admin/src/auth/html.rs#L85)
- [`struct AdminHtmlFormTextError`](../server_admin/src/auth/html.rs#L96)
- [`struct AdminHtmlFormKeyError`](../server_admin/src/auth/html.rs#L104)
- [`struct StdAdminHtmlSelectedError`](../server_admin/src/auth/html.rs#L112)
- [`struct AdminHtmlFormText`](../server_admin/src/auth/html.rs#L121)
- [`struct AdminHtmlFormKey`](../server_admin/src/auth/html.rs#L142)
- [`struct StdAdminHtmlSelected`](../server_admin/src/auth/html.rs#L159)
- [`struct SettingsForm`](../server_admin/src/auth/html.rs#L181)
- [`enum AdminCrudPage`](../server_admin/src/auth/html.rs#L488)
- [`struct AdminHtmlRouteRegistry`](../server_admin/src/auth/html.rs#L1244)
- [`struct AdminHtmlSwaggerRouteRegistry`](../server_admin/src/auth/html.rs#L1254)

## `server_admin/src/auth/rate_limit.rs`

- [`enum AdminRateLimitScope`](../server_admin/src/auth/rate_limit.rs#L3)

## `server_admin/src/auth/routes.rs`

- [`struct AdminAuthRouteRegistry`](../server_admin/src/auth/routes.rs#L98)

## `server_admin/src/generated_auth.rs`

- [`struct AdminGeneratedAuthLayer`](../server_admin/src/generated_auth.rs#L2)
- [`struct AdminGeneratedAuthService`](../server_admin/src/generated_auth.rs#L11)

## `server_admin/src/generated_tables.rs`

- [`struct AdminUsers`](../server_admin/src/generated_tables.rs#L22)
- [`struct AdminUserRoles`](../server_admin/src/generated_tables.rs#L73)
- [`struct AdminRolePermissions`](../server_admin/src/generated_tables.rs#L104)
- [`struct AdminRoles`](../server_admin/src/generated_tables.rs#L131)
- [`struct AdminPermissions`](../server_admin/src/generated_tables.rs#L162)
- [`struct AdminSystemSettings`](../server_admin/src/generated_tables.rs#L187)
- [`enum AdminGeneratedTable`](../server_admin/src/generated_tables.rs#L219)
- [`struct AdminGeneratedRouteContract`](../server_admin/src/generated_tables.rs#L425)
- [`struct UtoipaAdminOpenApi`](../server_admin/src/generated_tables.rs#L458)
- [`struct SharedAdminGeneratedTableStateArc`](../server_admin/src/generated_tables.rs#L462)
- [`struct AdminGeneratedTablesValidationError`](../server_admin/src/generated_tables.rs#L469)

## `server_admin/src/generated_tables/tests.rs`

- [`struct ClientTransport`](../server_admin/src/generated_tables/tests.rs#L2)

## `server_admin/src/lib.rs`

- [`struct AdminPasswordChangeRequired`](../server_admin/src/lib.rs#L35)
- [`enum AdminSecretTextError`](../server_admin/src/lib.rs#L39)
- [`struct AdminPermissions`](../server_admin/src/lib.rs#L83)
- [`struct AdminRoleNames`](../server_admin/src/lib.rs#L109)
- [`struct AdminAuthCollectionError`](../server_admin/src/lib.rs#L130)
- [`struct AdminSharedSemaphoreArc`](../server_admin/src/lib.rs#L153)
- [`struct TokioAdminJoinError`](../server_admin/src/lib.rs#L157)
- [`struct TokioAdminAcquireError`](../server_admin/src/lib.rs#L161)
- [`struct Argon2AdminPasswordHashError`](../server_admin/src/lib.rs#L169)
- [`struct SqlxAdminError`](../server_admin/src/lib.rs#L177)
- [`struct AdminPassword`](../server_admin/src/lib.rs#L195)
- [`enum AdminPasswordTryFromStringError`](../server_admin/src/lib.rs#L199)
- [`struct AdminPasswordHash`](../server_admin/src/lib.rs#L254)
- [`struct AdminJwtSecret`](../server_admin/src/lib.rs#L264)
- [`struct AdminOpaqueToken`](../server_admin/src/lib.rs#L274)
- [`struct AdminRefreshToken`](../server_admin/src/lib.rs#L284)
- [`struct AdminTokenHash`](../server_admin/src/lib.rs#L298)
- [`struct AdminGeneratedToken`](../server_admin/src/lib.rs#L314)
- [`struct AdminCookieSecure`](../server_admin/src/lib.rs#L343)
- [`struct AdminCookieMaxAgeSeconds`](../server_admin/src/lib.rs#L353)
- [`struct StdAdminCookie`](../server_admin/src/lib.rs#L365)
- [`struct HttpAdminHeaderMapRef`](../server_admin/src/lib.rs#L374)
- [`enum AdminCookieKind`](../server_admin/src/lib.rs#L376)
- [`struct AdminPasswordHashConcurrency`](../server_admin/src/lib.rs#L452)
- [`struct AdminUnixTokenStream`](../server_admin/src/lib.rs#L465)
- [`struct AdminSessionId`](../server_admin/src/lib.rs#L479)
- [`struct AdminAccessClaims`](../server_admin/src/lib.rs#L489)
- [`enum AdminPasswordHashError`](../server_admin/src/lib.rs#L526)
- [`struct AdminPasswordHasher`](../server_admin/src/lib.rs#L535)
- [`struct JsonwebtokenAdminError`](../server_admin/src/lib.rs#L541)
- [`struct AdminAccessTokenError`](../server_admin/src/lib.rs#L545)
- [`struct StdAdminAccessToken`](../server_admin/src/lib.rs#L556)
- [`enum AdminAuditAction`](../server_admin/src/lib.rs#L589)
- [`enum AdminAuditResource`](../server_admin/src/lib.rs#L610)
- [`struct SqlxAdminMigrateError`](../server_admin/src/lib.rs#L621)
- [`enum AdminMigrateErrorInner`](../server_admin/src/lib.rs#L623)
- [`struct AdminMigrateError`](../server_admin/src/lib.rs#L632)
- [`struct AdminCleanupBatchSize`](../server_admin/src/lib.rs#L639)
- [`struct AdminCleanupRetentionSeconds`](../server_admin/src/lib.rs#L641)
- [`struct AdminCleanupCfg`](../server_admin/src/lib.rs#L643)
- [`struct AdminCleanupReport`](../server_admin/src/lib.rs#L652)
- [`struct AdminCleanupRows`](../server_admin/src/lib.rs#L670)
- [`enum AdminCleanupCfgError`](../server_admin/src/lib.rs#L685)
- [`enum AdminCleanupError`](../server_admin/src/lib.rs#L692)
- [`enum AdminBootstrapError`](../server_admin/src/lib.rs#L760)
- [`enum AdminPasswordResetError`](../server_admin/src/lib.rs#L777)

## `server_admin/src/repository.rs`

- [`enum AdminRepositoryError`](../server_admin/src/repository.rs#L14)
- [`enum ReplaceRolePermissionsOutcome`](../server_admin/src/repository.rs#L21)
- [`enum ReplaceUserRolesOutcome`](../server_admin/src/repository.rs#L29)
- [`enum AdminRateLimitOutcome`](../server_admin/src/repository.rs#L37)
- [`enum AdminRateLimitRepositoryError`](../server_admin/src/repository.rs#L42)
- [`enum AdminRepositoryDbRef`](../server_admin/src/repository.rs#L47)
- [`struct AdminAuthenticatedRecord`](../server_admin/src/repository.rs#L52)
- [`struct AdminCleanupRepositoryReport`](../server_admin/src/repository.rs#L60)
- [`struct SqlxAdminRepositoryConnectionMutRef`](../server_admin/src/repository.rs#L115)
- [`struct SqlxAdminRepositoryPoolRef`](../server_admin/src/repository.rs#L120)
- [`struct AdminRecentLoginFailureCount`](../server_admin/src/repository.rs#L122)
- [`struct AdminPageTotalCount`](../server_admin/src/repository.rs#L124)
- [`struct AdminSignInUser`](../server_admin/src/repository.rs#L146)

## `server_admin/src/repository/data_tables.rs`

- [`struct DataPermissionsFlt`](../server_admin/src/repository/data_tables.rs#L84)
- [`struct DataRolePermissionsFlt`](../server_admin/src/repository/data_tables.rs#L88)
- [`struct DataRolesFlt`](../server_admin/src/repository/data_tables.rs#L92)
- [`struct DataSystemSettingsFlt`](../server_admin/src/repository/data_tables.rs#L94)
- [`struct DataUserRolesFlt`](../server_admin/src/repository/data_tables.rs#L98)
- [`struct DataUsersFlt`](../server_admin/src/repository/data_tables.rs#L102)
- [`struct DataFltJson`](../server_admin/src/repository/data_tables.rs#L105)
- [`enum DataFlt`](../server_admin/src/repository/data_tables.rs#L108)

## `server_admin/src/repository/roles.rs`

- [`struct AdminActiveAdministratorCount`](../server_admin/src/repository/roles.rs#L4)
- [`struct LastAdminState`](../server_admin/src/repository/roles.rs#L7)

## `server_admin/tests/admin_api.rs`

- [`struct StdAdminApiTestStrRef`](../server_admin/tests/admin_api.rs#L20)
- [`struct AxumAdminApiTestRouter`](../server_admin/tests/admin_api.rs#L22)
- [`struct SqlxAdminApiTestPool`](../server_admin/tests/admin_api.rs#L24)
- [`struct SqlxAdminHtmlTestTransaction`](../server_admin/tests/admin_api.rs#L26)
- [`struct HttpAdminApiTestMethod`](../server_admin/tests/admin_api.rs#L28)
- [`struct HttpAdminApiTestRequest`](../server_admin/tests/admin_api.rs#L30)
- [`struct HttpAdminHtmlTestResponse`](../server_admin/tests/admin_api.rs#L32)
- [`struct HttpAdminApiTestResponseRef`](../server_admin/tests/admin_api.rs#L34)
- [`struct StdAdminApiTestCookie`](../server_admin/tests/admin_api.rs#L38)
- [`struct AdminHtmlTestBody`](../server_admin/tests/admin_api.rs#L41)
- [`struct AdminHtmlTestFormBody`](../server_admin/tests/admin_api.rs#L44)
- [`struct AdminHtmlTestFixture`](../server_admin/tests/admin_api.rs#L46)
- [`struct AdminHtmlSettingsTestValues`](../server_admin/tests/admin_api.rs#L54)

## `server_admin_contract/src/collections.rs`

- [`enum AdminCollectionError`](../server_admin_contract/src/collections.rs#L5)
- [`struct AdminBoundedVec`](../server_admin_contract/src/collections.rs#L25)
- [`struct AdminOpenApiVecPhantomData`](../server_admin_contract/src/collections.rs#L47)
- [`struct AdminOpenApiVec`](../server_admin_contract/src/collections.rs#L50)
- [`struct AdminPermissionValues`](../server_admin_contract/src/collections.rs#L92)
- [`struct AdminRoleNames`](../server_admin_contract/src/collections.rs#L111)
- [`struct AdminRoleIds`](../server_admin_contract/src/collections.rs#L135)
- [`struct AdminPermissionIds`](../server_admin_contract/src/collections.rs#L159)
- [`struct AdminUserSummaries`](../server_admin_contract/src/collections.rs#L183)
- [`struct AdminRoleSummaries`](../server_admin_contract/src/collections.rs#L207)
- [`struct AdminPermissionSummaries`](../server_admin_contract/src/collections.rs#L231)
- [`struct AdminAuditViews`](../server_admin_contract/src/collections.rs#L255)
- [`struct AdminTexts`](../server_admin_contract/src/collections.rs#L279)
- [`struct AdminDataRows`](../server_admin_contract/src/collections.rs#L303)
- [`struct AdminDataTables`](../server_admin_contract/src/collections.rs#L327)
- [`struct AdminOptionalSettings`](../server_admin_contract/src/collections.rs#L351)
- [`struct AdminSessionViews`](../server_admin_contract/src/collections.rs#L370)
- [`struct AdminEmptyCollection`](../server_admin_contract/src/collections.rs#L401)

## `server_admin_contract/src/dto.rs`

- [`struct AuthenticatedAdmin`](../server_admin_contract/src/dto.rs#L9)
- [`struct AdminSignInRes`](../server_admin_contract/src/dto.rs#L84)
- [`struct AdminCreateUserReq`](../server_admin_contract/src/dto.rs#L99)
- [`struct AdminCreateUserRes`](../server_admin_contract/src/dto.rs#L115)
- [`struct AdminUpdateUserReq`](../server_admin_contract/src/dto.rs#L129)
- [`struct AdminSetUserPasswordReq`](../server_admin_contract/src/dto.rs#L144)
- [`struct AdminChangeOwnPasswordReq`](../server_admin_contract/src/dto.rs#L159)
- [`struct AdminSetUserBanReq`](../server_admin_contract/src/dto.rs#L175)
- [`struct AdminCreateRoleReq`](../server_admin_contract/src/dto.rs#L190)
- [`struct AdminCreateRoleRes`](../server_admin_contract/src/dto.rs#L205)
- [`struct AdminUpdateRoleReq`](../server_admin_contract/src/dto.rs#L219)
- [`struct AdminSetUserRolesReq`](../server_admin_contract/src/dto.rs#L234)
- [`struct AdminSetRolePermissionsReq`](../server_admin_contract/src/dto.rs#L249)
- [`struct AdminUserSummary`](../server_admin_contract/src/dto.rs#L264)
- [`struct AdminRoleSummary`](../server_admin_contract/src/dto.rs#L288)
- [`struct AdminPermissionSummary`](../server_admin_contract/src/dto.rs#L313)
- [`struct AdminUsersPage`](../server_admin_contract/src/dto.rs#L342)
- [`struct AdminRolesPage`](../server_admin_contract/src/dto.rs#L361)
- [`struct AdminPermissionsPage`](../server_admin_contract/src/dto.rs#L380)
- [`struct AdminAuditView`](../server_admin_contract/src/dto.rs#L398)
- [`struct AdminAuditCursor`](../server_admin_contract/src/dto.rs#L428)
- [`struct AdminAuditPage`](../server_admin_contract/src/dto.rs#L444)
- [`struct AdminDataColumn`](../server_admin_contract/src/dto.rs#L462)
- [`struct AdminDataFilter`](../server_admin_contract/src/dto.rs#L512)
- [`struct AdminDataFilters`](../server_admin_contract/src/dto.rs#L559)
- [`enum AdminDataInputKind`](../server_admin_contract/src/dto.rs#L584)
- [`struct AdminDataColumns`](../server_admin_contract/src/dto.rs#L617)
- [`struct AdminDataRow`](../server_admin_contract/src/dto.rs#L640)
- [`struct AdminDataTableView`](../server_admin_contract/src/dto.rs#L655)
- [`struct AdminDataTableCatalog`](../server_admin_contract/src/dto.rs#L676)
- [`struct AdminAuditExportCsv`](../server_admin_contract/src/dto.rs#L695)
- [`struct AdminAuditExport`](../server_admin_contract/src/dto.rs#L706)
- [`struct AdminSignInReq`](../server_admin_contract/src/dto.rs#L720)

## `server_admin_contract/src/lib.rs`

- [`struct AdminApiBodyMaxBytes`](../server_admin_contract/src/lib.rs#L21)
- [`struct PositiveNonZeroI64`](../server_admin_contract/src/lib.rs#L57)
- [`struct AdminText`](../server_admin_contract/src/lib.rs#L103)
- [`struct AdminLogin`](../server_admin_contract/src/lib.rs#L124)
- [`struct AdminDisplayName`](../server_admin_contract/src/lib.rs#L145)
- [`struct AdminRoleName`](../server_admin_contract/src/lib.rs#L166)
- [`struct AdminPassword`](../server_admin_contract/src/lib.rs#L186)
- [`struct AdminNewPassword`](../server_admin_contract/src/lib.rs#L207)
- [`struct AdminPermissionValue`](../server_admin_contract/src/lib.rs#L226)
- [`struct AdminPermissionStrRef`](../server_admin_contract/src/lib.rs#L238)
- [`enum AdminPermission`](../server_admin_contract/src/lib.rs#L253)
- [`enum AdminDataTable`](../server_admin_contract/src/lib.rs#L330)
- [`struct AdminDataTableStrRef`](../server_admin_contract/src/lib.rs#L367)
- [`struct AdminDataColumnsCsvRef`](../server_admin_contract/src/lib.rs#L379)
- [`struct AdminDataOrderRef`](../server_admin_contract/src/lib.rs#L391)
- [`struct AdminDataTableSpec`](../server_admin_contract/src/lib.rs#L393)
- [`struct AdminAuditTimestamp`](../server_admin_contract/src/lib.rs#L586)
- [`struct AdminAuditDetailsBytes`](../server_admin_contract/src/lib.rs#L598)
- [`struct AdminAuditDetailsTooLarge`](../server_admin_contract/src/lib.rs#L614)
- [`struct SerdeJsonAdminAuditDetails`](../server_admin_contract/src/lib.rs#L637)
- [`struct AdminDefaultRoute`](../server_admin_contract/src/lib.rs#L658)
- [`struct AdminSiteName`](../server_admin_contract/src/lib.rs#L669)
- [`struct AdminMainLogo`](../server_admin_contract/src/lib.rs#L686)
- [`struct AdminOrganizationContacts`](../server_admin_contract/src/lib.rs#L701)
- [`struct AdminOrganizationName`](../server_admin_contract/src/lib.rs#L716)
- [`struct AdminPrimaryColor`](../server_admin_contract/src/lib.rs#L733)
- [`struct AdminSupportUrl`](../server_admin_contract/src/lib.rs#L750)
- [`struct AdminTabTitle`](../server_admin_contract/src/lib.rs#L767)
- [`enum AdminTableSortField`](../server_admin_contract/src/lib.rs#L769)
- [`struct AdminTableSortFieldTryFromKeyError`](../server_admin_contract/src/lib.rs#L789)
- [`struct AdminTableSortValues`](../server_admin_contract/src/lib.rs#L791)
- [`struct AdminTableSortKeyRef`](../server_admin_contract/src/lib.rs#L804)
- [`struct AdminUserId`](../server_admin_contract/src/lib.rs#L888)
- [`struct AdminRoleId`](../server_admin_contract/src/lib.rs#L922)
- [`struct AdminPermissionId`](../server_admin_contract/src/lib.rs#L956)
- [`struct AdminAuditLogId`](../server_admin_contract/src/lib.rs#L989)
- [`struct AdminIdTryFromI64Error`](../server_admin_contract/src/lib.rs#L1011)

## `server_admin_contract/src/query.rs`

- [`struct AdminBool`](../server_admin_contract/src/query.rs#L17)
- [`struct AdminPageOffset`](../server_admin_contract/src/query.rs#L33)
- [`struct AdminPageOffsetVisitor`](../server_admin_contract/src/query.rs#L35)
- [`struct AdminPageLimit`](../server_admin_contract/src/query.rs#L81)
- [`struct AdminPageLimitVisitor`](../server_admin_contract/src/query.rs#L83)
- [`struct AdminDefaultPageLimit`](../server_admin_contract/src/query.rs#L119)
- [`struct AdminPageLimitError`](../server_admin_contract/src/query.rs#L153)
- [`struct AdminPageTotal`](../server_admin_contract/src/query.rs#L171)
- [`struct AdminTableSearch`](../server_admin_contract/src/query.rs#L188)
- [`struct AdminTableSortKey`](../server_admin_contract/src/query.rs#L205)
- [`enum AdminSortDirection`](../server_admin_contract/src/query.rs#L220)
- [`struct AdminTableQuery`](../server_admin_contract/src/query.rs#L245)
- [`struct AdminFilterField`](../server_admin_contract/src/query.rs#L310)
- [`struct AdminFilterValue`](../server_admin_contract/src/query.rs#L329)
- [`struct AdminFilterOperationKey`](../server_admin_contract/src/query.rs#L342)
- [`struct AdminDataTableFilterQuery`](../server_admin_contract/src/query.rs#L370)
- [`struct AdminDataTableQuery`](../server_admin_contract/src/query.rs#L425)

## `server_admin_contract/src/routes.rs`

- [`struct AdminSignInRoute`](../server_admin_contract/src/routes.rs#L28)
- [`struct AdminRefreshRoute`](../server_admin_contract/src/routes.rs#L46)
- [`struct AdminMeRoute`](../server_admin_contract/src/routes.rs#L64)
- [`struct AdminChangeOwnPasswordRoute`](../server_admin_contract/src/routes.rs#L70)
- [`struct AdminSignOutRoute`](../server_admin_contract/src/routes.rs#L76)
- [`struct AdminSessionsRoute`](../server_admin_contract/src/routes.rs#L82)
- [`struct AdminRevokeSessionRoute`](../server_admin_contract/src/routes.rs#L88)
- [`struct AdminRevokeAllSessionsRoute`](../server_admin_contract/src/routes.rs#L94)
- [`struct AdminListUsersRoute`](../server_admin_contract/src/routes.rs#L100)
- [`struct AdminCreateUserRoute`](../server_admin_contract/src/routes.rs#L106)
- [`struct AdminUpdateUserRoute`](../server_admin_contract/src/routes.rs#L112)
- [`struct AdminDeleteUserRoute`](../server_admin_contract/src/routes.rs#L118)
- [`struct AdminSetUserPasswordRoute`](../server_admin_contract/src/routes.rs#L124)
- [`struct AdminSetUserBanRoute`](../server_admin_contract/src/routes.rs#L130)
- [`struct AdminSetUserRolesRoute`](../server_admin_contract/src/routes.rs#L136)
- [`struct AdminListRolesRoute`](../server_admin_contract/src/routes.rs#L142)
- [`struct AdminCreateRoleRoute`](../server_admin_contract/src/routes.rs#L148)
- [`struct AdminUpdateRoleRoute`](../server_admin_contract/src/routes.rs#L154)
- [`struct AdminDeleteRoleRoute`](../server_admin_contract/src/routes.rs#L160)
- [`struct AdminSetRolePermissionsRoute`](../server_admin_contract/src/routes.rs#L166)
- [`struct AdminListPermissionsRoute`](../server_admin_contract/src/routes.rs#L172)
- [`struct AdminAuditLogRoute`](../server_admin_contract/src/routes.rs#L178)
- [`struct AdminAuditExportRoute`](../server_admin_contract/src/routes.rs#L184)
- [`struct AdminBrandingRoute`](../server_admin_contract/src/routes.rs#L190)
- [`struct AdminDataTablesRoute`](../server_admin_contract/src/routes.rs#L196)
- [`struct AdminDataTableRoute`](../server_admin_contract/src/routes.rs#L202)
- [`struct AdminSettingsRoute`](../server_admin_contract/src/routes.rs#L208)
- [`struct AdminUpdateSettingsRoute`](../server_admin_contract/src/routes.rs#L214)
- [`enum AdminRoute`](../server_admin_contract/src/routes.rs#L229)
- [`struct AdminDataTableFrontendPath`](../server_admin_contract/src/routes.rs#L332)
- [`struct AdminRoutePath`](../server_admin_contract/src/routes.rs#L343)
- [`enum AdminRoutePathError`](../server_admin_contract/src/routes.rs#L345)
- [`struct AdminPagePathRef`](../server_admin_contract/src/routes.rs#L374)
- [`enum AdminFrontendPath`](../server_admin_contract/src/routes.rs#L389)
- [`enum AdminHtmlAction`](../server_admin_contract/src/routes.rs#L437)
- [`enum AdminPage`](../server_admin_contract/src/routes.rs#L525)
- [`enum AdminPageCapability`](../server_admin_contract/src/routes.rs#L626)
- [`enum AdminPageClientMode`](../server_admin_contract/src/routes.rs#L631)
- [`enum AdminPageNavigation`](../server_admin_contract/src/routes.rs#L639)
- [`struct AdminPageMetadata`](../server_admin_contract/src/routes.rs#L648)
- [`enum AdminPageTitle`](../server_admin_contract/src/routes.rs#L672)
- [`struct AdminPageSpec`](../server_admin_contract/src/routes.rs#L685)

## `server_admin_contract/src/sessions.rs`

- [`struct AdminNoBody`](../server_admin_contract/src/sessions.rs#L10)
- [`struct AdminSessionIdentifier`](../server_admin_contract/src/sessions.rs#L26)
- [`struct AdminSessionTimestamp`](../server_admin_contract/src/sessions.rs#L42)
- [`struct AdminSessionView`](../server_admin_contract/src/sessions.rs#L52)
- [`struct AdminSessionsPage`](../server_admin_contract/src/sessions.rs#L70)

## `server_admin_contract/src/settings.rs`

- [`struct AdminSettingsView`](../server_admin_contract/src/settings.rs#L11)
- [`struct AdminBrandingView`](../server_admin_contract/src/settings.rs#L39)
- [`struct AdminUpdateSettingsReq`](../server_admin_contract/src/settings.rs#L93)
- [`enum AdminSettingInputKind`](../server_admin_contract/src/settings.rs#L106)
- [`struct AdminSettingLabel`](../server_admin_contract/src/settings.rs#L121)
- [`struct AdminSettingName`](../server_admin_contract/src/settings.rs#L132)
- [`struct AdminSettingSpec`](../server_admin_contract/src/settings.rs#L134)
- [`enum AdminSettingOptionality`](../server_admin_contract/src/settings.rs#L166)
- [`enum AdminSetting`](../server_admin_contract/src/settings.rs#L180)
- [`enum AdminOptionalSetting`](../server_admin_contract/src/settings.rs#L204)

## `server_admin_contract/src/tests.rs`

- [`struct ClientTransport`](../server_admin_contract/src/tests.rs#L2)

## `server_admin_core/src/domain_types.rs`

- [`struct SecrecyAdminString`](../server_admin_core/src/domain_types.rs#L3)
- [`struct StdAdminString`](../server_admin_core/src/domain_types.rs#L37)
- [`enum AdminResourceText`](../server_admin_core/src/domain_types.rs#L44)
- [`struct StdAdminStrRef`](../server_admin_core/src/domain_types.rs#L86)
- [`struct StdAdminBool`](../server_admin_core/src/domain_types.rs#L100)
- [`struct AdminNonZeroUsize`](../server_admin_core/src/domain_types.rs#L112)
- [`struct UuidAdminValue`](../server_admin_core/src/domain_types.rs#L126)
- [`struct AdminSocketAddr`](../server_admin_core/src/domain_types.rs#L149)
- [`struct AdminUserId`](../server_admin_core/src/domain_types.rs#L166)
- [`struct AdminRoleId`](../server_admin_core/src/domain_types.rs#L201)
- [`struct AdminPermissionId`](../server_admin_core/src/domain_types.rs#L236)
- [`struct AdminAuditLogId`](../server_admin_core/src/domain_types.rs#L266)
- [`struct AdminIdTryFromI64Error`](../server_admin_core/src/domain_types.rs#L289)
- [`struct AdminPermissionName`](../server_admin_core/src/domain_types.rs#L299)

## `server_admin_frontend/src/app/http/mutation.rs`

- [`struct AdminCsrfToken`](../server_admin_frontend/src/app/http/mutation.rs#L5)

## `server_admin_frontend/src/app/http/url.rs`

- [`struct AdminHttpStatus`](../server_admin_frontend/src/app/http/url.rs#L9)
- [`struct AdminCsrApiUrl`](../server_admin_frontend/src/app/http/url.rs#L20)
- [`struct AdminCsrApiUrlSuffixRef`](../server_admin_frontend/src/app/http/url.rs#L30)

## `server_admin_frontend/src/app/mutation.rs`

- [`enum AdminMutationMethod`](../server_admin_frontend/src/app/mutation.rs#L2)

## `server_admin_frontend/src/app/query.rs`

- [`struct AdminCsrQuery`](../server_admin_frontend/src/app/query.rs#L6)

## `server_admin_frontend/src/app/state.rs`

- [`enum AdminLoadState`](../server_admin_frontend/src/app/state.rs#L2)
- [`enum AdminTableLoadError`](../server_admin_frontend/src/app/state.rs#L50)

## `server_admin_frontend/src/lib.rs`

- [`struct AxumAdminFrontendRouter`](../server_admin_frontend/src/lib.rs#L11)
- [`enum AdminAssetsError`](../server_admin_frontend/src/lib.rs#L16)

## `server_admin_frontend/src/shared/data_grid/column/filter.rs`

- [`struct LeptosAdminFilterOperationSignal`](../server_admin_frontend/src/shared/data_grid/column/filter.rs#L17)

## `server_admin_frontend/src/shared/data_grid/column/filter/input_kind.rs`

- [`enum AdminDataGridInputType`](../server_admin_frontend/src/shared/data_grid/column/filter/input_kind.rs#L2)

## `server_admin_frontend/src/shared/pagination.rs`

- [`struct AdminPageNavDisabled`](../server_admin_frontend/src/shared/pagination.rs#L16)
- [`struct AdminPageRange`](../server_admin_frontend/src/shared/pagination.rs#L19)

## `server_admin_frontend/src/shared/settings/input.rs`

- [`struct AdminSettingDisabled`](../server_admin_frontend/src/shared/settings/input.rs#L19)
- [`struct AdminSettingRequired`](../server_admin_frontend/src/shared/settings/input.rs#L28)

## `server_admin_frontend/src/shared/settings/signals.rs`

- [`struct AdminSettingsFormSignals`](../server_admin_frontend/src/shared/settings/signals.rs#L8)

## `server_admin_frontend/src/shared/settings/values.rs`

- [`struct AdminSettingInputValue`](../server_admin_frontend/src/shared/settings/values.rs#L9)
- [`struct AdminSettingsFormValues`](../server_admin_frontend/src/shared/settings/values.rs#L12)

## `server_admin_frontend/src/shared/table_filters/query.rs`

- [`enum AdminTableQueryDirection`](../server_admin_frontend/src/shared/table_filters/query.rs#L7)

## `server_admin_frontend/src/shared/text.rs`

- [`enum AdminJoinedTextTryFromStringError`](../server_admin_frontend/src/shared/text.rs#L4)
- [`struct AdminJoinedText`](../server_admin_frontend/src/shared/text.rs#L12)

## `server_admin_frontend/src/ssr.rs`

- [`struct AdminSsrHtmlTryFromStringError`](../server_admin_frontend/src/ssr.rs#L39)
- [`struct AdminSsrTextTryFromStringError`](../server_admin_frontend/src/ssr.rs#L49)
- [`struct AdminSsrErrorMessage`](../server_admin_frontend/src/ssr.rs#L59)
- [`struct AdminSsrText`](../server_admin_frontend/src/ssr.rs#L77)
- [`struct AdminSsrHtml`](../server_admin_frontend/src/ssr.rs#L96)

## `server_admin_frontend/src/ui/alert.rs`

- [`enum AdminAlertVariant`](../server_admin_frontend/src/ui/alert.rs#L21)

## `server_admin_frontend/src/ui/badge.rs`

- [`enum AdminBadgeVariant`](../server_admin_frontend/src/ui/badge.rs#L21)

## `server_admin_frontend/src/ui/button.rs`

- [`enum AdminButtonVariant`](../server_admin_frontend/src/ui/button.rs#L19)
- [`enum AdminButtonKind`](../server_admin_frontend/src/ui/button.rs#L58)

## `server_admin_frontend/src/ui/card.rs`

- [`enum AdminCardVariant`](../server_admin_frontend/src/ui/card.rs#L21)

## `server_admin_frontend/src/ui/field.rs`

- [`struct AdminFieldLabel`](../server_admin_frontend/src/ui/field.rs#L19)

## `server_admin_frontend/src/ui/input.rs`

- [`struct AdminInputName`](../server_admin_frontend/src/ui/input.rs#L21)
- [`struct LeptosAdminInputSignal`](../server_admin_frontend/src/ui/input.rs#L42)
- [`enum AdminInputKind`](../server_admin_frontend/src/ui/input.rs#L51)

## `server_app_state/src/domain_types.rs`

- [`struct ServerAppState`](../server_app_state/src/domain_types.rs#L2)

## `server_config/src/domain_types.rs`

- [`struct Config`](../server_config/src/domain_types.rs#L4)
- [`enum ProductionConfigError`](../server_config/src/domain_types.rs#L75)

## `server_observability/src/lib.rs`

- [`enum ServiceTracingFormat`](../server_observability/src/lib.rs#L15)

## `server_observability/src/observability.rs`

- [`struct ServiceName`](../server_observability/src/observability.rs#L9)
- [`struct OpentelemetryOtlpExporterBuildError`](../server_observability/src/observability.rs#L15)
- [`struct OpentelemetrySdkTracerProvider`](../server_observability/src/observability.rs#L18)
- [`struct TracingSubscriberInitError`](../server_observability/src/observability.rs#L24)
- [`enum ObservabilityInitError`](../server_observability/src/observability.rs#L27)
- [`struct OpentelemetrySdkObservabilityShutdownError`](../server_observability/src/observability.rs#L38)
- [`struct ObservabilityGuard`](../server_observability/src/observability.rs#L41)

## `server_observability/src/observed_error.rs`

- [`struct ObservedErrorCode`](../server_observability/src/observed_error.rs#L12)
- [`struct ObservedErrorBacktrace`](../server_observability/src/observed_error.rs#L17)
- [`struct StdPanicLocation`](../server_observability/src/observed_error.rs#L27)
- [`struct TracingObservedErrorSpanTrace`](../server_observability/src/observed_error.rs#L32)
- [`struct ObservedError`](../server_observability/src/observed_error.rs#L36)

## `server_runtime_core/src/background_job.rs`

- [`struct BackgroundJob`](../server_runtime_core/src/background_job.rs#L2)

## `server_runtime_core/src/deduplicating_queue.rs`

- [`struct QueueMaximumNonZeroUsize`](../server_runtime_core/src/deduplicating_queue.rs#L10)
- [`enum QueuePush`](../server_runtime_core/src/deduplicating_queue.rs#L13)
- [`struct DeduplicatingQueue`](../server_runtime_core/src/deduplicating_queue.rs#L20)
- [`struct CollectionsHashSet`](../server_runtime_core/src/deduplicating_queue.rs#L65)
- [`struct CollectionsVecDeque`](../server_runtime_core/src/deduplicating_queue.rs#L68)

## `server_runtime_core/src/exclusive_run.rs`

- [`struct ExclusiveRun`](../server_runtime_core/src/exclusive_run.rs#L2)
- [`struct ExclusiveRunAlreadyActive`](../server_runtime_core/src/exclusive_run.rs#L38)
- [`struct ExclusiveRunGuard`](../server_runtime_core/src/exclusive_run.rs#L42)
- [`struct ExclusiveRunAtomicBool`](../server_runtime_core/src/exclusive_run.rs#L54)

## `server_runtime_core/src/execution_plan.rs`

- [`enum ExecutionMode`](../server_runtime_core/src/execution_plan.rs#L2)
- [`enum ExecutionReport`](../server_runtime_core/src/execution_plan.rs#L8)

## `server_runtime_core/src/generation_gate.rs`

- [`struct Generation`](../server_runtime_core/src/generation_gate.rs#L10)
- [`enum GenerationCommit`](../server_runtime_core/src/generation_gate.rs#L13)
- [`struct GenerationGate`](../server_runtime_core/src/generation_gate.rs#L19)
- [`struct GenerationAtomicU64`](../server_runtime_core/src/generation_gate.rs#L44)

## `server_runtime_core/src/history.rs`

- [`struct RunReportsVecDeque`](../server_runtime_core/src/history.rs#L2)
- [`struct SharedRunReportsArc`](../server_runtime_core/src/history.rs#L7)
- [`struct AsyncRunHistory`](../server_runtime_core/src/history.rs#L11)
- [`struct AsyncRunHistoryMaximumLenNonZeroUsize`](../server_runtime_core/src/history.rs#L16)
- [`struct StdAsyncRunHistoryMaximumLenTryFromUsizeError`](../server_runtime_core/src/history.rs#L32)
- [`struct StdAsyncRunHistoryReportCount`](../server_runtime_core/src/history.rs#L43)
- [`struct AsyncRunHistorySnapshot`](../server_runtime_core/src/history.rs#L46)

## `server_runtime_core/src/identity_bootstrap.rs`

- [`struct IdentitySpec`](../server_runtime_core/src/identity_bootstrap.rs#L2)
- [`enum IdentityPresence`](../server_runtime_core/src/identity_bootstrap.rs#L47)
- [`enum IdentityRolePresence`](../server_runtime_core/src/identity_bootstrap.rs#L53)
- [`enum IdentityBootstrapDecision`](../server_runtime_core/src/identity_bootstrap.rs#L59)

## `server_runtime_core/src/lease_registry.rs`

- [`struct LeaseId`](../server_runtime_core/src/lease_registry.rs#L6)
- [`struct LeaseKey`](../server_runtime_core/src/lease_registry.rs#L20)
- [`enum LeaseTextError`](../server_runtime_core/src/lease_registry.rs#L34)
- [`enum LeaseState`](../server_runtime_core/src/lease_registry.rs#L44)
- [`struct LeaseRegistryMaximumNonZeroUsize`](../server_runtime_core/src/lease_registry.rs#L59)
- [`struct LeaseStaleTimeoutDuration`](../server_runtime_core/src/lease_registry.rs#L62)
- [`struct StdLeaseStaleTimeoutError`](../server_runtime_core/src/lease_registry.rs#L78)
- [`enum LeaseReservation`](../server_runtime_core/src/lease_registry.rs#L81)
- [`enum LeaseHeartbeat`](../server_runtime_core/src/lease_registry.rs#L88)
- [`struct LeaseIds`](../server_runtime_core/src/lease_registry.rs#L103)
- [`struct LeaseEntry`](../server_runtime_core/src/lease_registry.rs#L106)
- [`struct LeaseRegistryInner`](../server_runtime_core/src/lease_registry.rs#L113)
- [`struct LeaseRegistry`](../server_runtime_core/src/lease_registry.rs#L119)
- [`struct LeaseTextRef`](../server_runtime_core/src/lease_registry.rs#L215)
- [`struct TokioLeaseRegistryRwLockArc`](../server_runtime_core/src/lease_registry.rs#L218)
- [`struct TokioLeaseInstant`](../server_runtime_core/src/lease_registry.rs#L221)

## `server_runtime_core/src/resource_budget.rs`

- [`struct ResourceBudgetMaximum`](../server_runtime_core/src/resource_budget.rs#L2)
- [`struct ResourceBudgetAmount`](../server_runtime_core/src/resource_budget.rs#L12)
- [`struct SharedAtomicUsizeArc`](../server_runtime_core/src/resource_budget.rs#L15)
- [`struct ResourceBudgetConfigError`](../server_runtime_core/src/resource_budget.rs#L36)
- [`struct ResourceBudget`](../server_runtime_core/src/resource_budget.rs#L38)
- [`trait GetBulkItemResourceBudget`](../server_runtime_core/src/resource_budget.rs#L42)
- [`trait GetIdempotencyResponseResourceBudget`](../server_runtime_core/src/resource_budget.rs#L45)
- [`enum ResourceBudgetReserveError`](../server_runtime_core/src/resource_budget.rs#L51)
- [`struct ResourceBudgetReservation`](../server_runtime_core/src/resource_budget.rs#L59)

## `server_runtime_core/src/resource_utilization.rs`

- [`struct ResourceAmount`](../server_runtime_core/src/resource_utilization.rs#L16)
- [`struct ResourceUtilizationPercent`](../server_runtime_core/src/resource_utilization.rs#L34)
- [`enum ResourceUtilizationKnownPercent`](../server_runtime_core/src/resource_utilization.rs#L36)
- [`struct ResourceUtilizationPercentTryFromU8Error`](../server_runtime_core/src/resource_utilization.rs#L50)
- [`enum ResourceUtilizationStatus`](../server_runtime_core/src/resource_utilization.rs#L64)
- [`enum ResourceUtilizationError`](../server_runtime_core/src/resource_utilization.rs#L74)
- [`struct ResourceUtilization`](../server_runtime_core/src/resource_utilization.rs#L84)

## `server_runtime_core/src/retry.rs`

- [`struct RetryAttemptsNonZeroUsize`](../server_runtime_core/src/retry.rs#L10)
- [`struct StdRetryAttemptsError`](../server_runtime_core/src/retry.rs#L33)
- [`struct RetryDelayDuration`](../server_runtime_core/src/retry.rs#L44)
- [`struct RetryPolicy`](../server_runtime_core/src/retry.rs#L47)
- [`struct RetryOutcome`](../server_runtime_core/src/retry.rs#L73)

## `server_runtime_core/src/secret_text.rs`

- [`enum BoundedSecretTextError`](../server_runtime_core/src/secret_text.rs#L6)
- [`struct BoundedSecretText`](../server_runtime_core/src/secret_text.rs#L19)
- [`struct SecretTextRef`](../server_runtime_core/src/secret_text.rs#L46)
- [`enum SecretTextMatch`](../server_runtime_core/src/secret_text.rs#L73)

## `server_runtime_core/src/single_flight.rs`

- [`struct SingleFlightKey`](../server_runtime_core/src/single_flight.rs#L4)
- [`enum SingleFlightKeyError`](../server_runtime_core/src/single_flight.rs#L24)
- [`struct SingleFlightMaximumNonZeroUsize`](../server_runtime_core/src/single_flight.rs#L42)
- [`struct SingleFlight`](../server_runtime_core/src/single_flight.rs#L45)
- [`enum SingleFlightAcquire`](../server_runtime_core/src/single_flight.rs#L86)
- [`struct SingleFlightOwner`](../server_runtime_core/src/single_flight.rs#L94)
- [`struct SingleFlightWaiter`](../server_runtime_core/src/single_flight.rs#L111)
- [`enum SingleFlightWaitOutcome`](../server_runtime_core/src/single_flight.rs#L122)
- [`struct SingleFlightInner`](../server_runtime_core/src/single_flight.rs#L127)
- [`struct ArcSingleFlightRwLock`](../server_runtime_core/src/single_flight.rs#L136)
- [`struct SingleFlightRwLockWriteGuard`](../server_runtime_core/src/single_flight.rs#L145)
- [`enum SingleFlightSignal`](../server_runtime_core/src/single_flight.rs#L150)
- [`struct TokioSingleFlightReceiver`](../server_runtime_core/src/single_flight.rs#L156)
- [`struct TokioSingleFlightSender`](../server_runtime_core/src/single_flight.rs#L159)

## `server_runtime_core/src/source_selection.rs`

- [`enum SourceSelection`](../server_runtime_core/src/source_selection.rs#L2)
- [`struct SourceSelectionError`](../server_runtime_core/src/source_selection.rs#L15)

## `server_runtime_http/src/batched_cleanup.rs`

- [`struct CleanupBatchSize`](../server_runtime_http/src/batched_cleanup.rs#L2)
- [`struct CleanupBatchSizeError`](../server_runtime_http/src/batched_cleanup.rs#L8)
- [`struct CleanupRows`](../server_runtime_http/src/batched_cleanup.rs#L33)
- [`struct CleanupBatchCount`](../server_runtime_http/src/batched_cleanup.rs#L46)
- [`enum CleanupContinuation`](../server_runtime_http/src/batched_cleanup.rs#L49)
- [`enum CleanupCompletion`](../server_runtime_http/src/batched_cleanup.rs#L55)
- [`struct CleanupReport`](../server_runtime_http/src/batched_cleanup.rs#L62)

## `server_runtime_http/src/bounded_read.rs`

- [`struct PathRef`](../server_runtime_http/src/bounded_read.rs#L2)
- [`struct BoundedReadMaximumBytes`](../server_runtime_http/src/bounded_read.rs#L14)
- [`struct BoundedBytes`](../server_runtime_http/src/bounded_read.rs#L25)
- [`struct BoundedText`](../server_runtime_http/src/bounded_read.rs#L35)
- [`struct BoundedReadConcurrencyArcSemaphore`](../server_runtime_http/src/bounded_read.rs#L57)
- [`struct BoundedReadConcurrencyMaximumNonZeroUsize`](../server_runtime_http/src/bounded_read.rs#L60)
- [`struct BoundedReadIoError`](../server_runtime_http/src/bounded_read.rs#L74)
- [`enum IoErrorPresenceDisposition`](../server_runtime_http/src/bounded_read.rs#L76)
- [`struct ReqwestError`](../server_runtime_http/src/bounded_read.rs#L84)
- [`struct BoundedReadFromUtf8Error`](../server_runtime_http/src/bounded_read.rs#L89)
- [`struct ReqwestResponse`](../server_runtime_http/src/bounded_read.rs#L91)
- [`struct SerdeJsonError`](../server_runtime_http/src/bounded_read.rs#L97)
- [`struct BoundedJsonText`](../server_runtime_http/src/bounded_read.rs#L101)
- [`enum BoundedJsonReadError`](../server_runtime_http/src/bounded_read.rs#L135)
- [`enum BoundedReadError`](../server_runtime_http/src/bounded_read.rs#L142)
- [`struct BoundedReadObservedBytes`](../server_runtime_http/src/bounded_read.rs#L166)

## `server_runtime_http/src/child_process.rs`

- [`struct ChildDiagnosticMaximumNonZeroUsize`](../server_runtime_http/src/child_process.rs#L10)
- [`struct ChildProcessId`](../server_runtime_http/src/child_process.rs#L23)
- [`struct ChildProcessSetMaximumNonZeroUsize`](../server_runtime_http/src/child_process.rs#L34)
- [`struct StdCollectionsChildProcessMap`](../server_runtime_http/src/child_process.rs#L37)
- [`struct ChildProcessSet`](../server_runtime_http/src/child_process.rs#L46)
- [`struct ChildProcessReports`](../server_runtime_http/src/child_process.rs#L110)
- [`enum ChildProcessSetError`](../server_runtime_http/src/child_process.rs#L115)
- [`struct ChildDiagnostic`](../server_runtime_http/src/child_process.rs#L139)
- [`enum ChildProcessCompletion`](../server_runtime_http/src/child_process.rs#L142)
- [`struct ChildExitStatus`](../server_runtime_http/src/child_process.rs#L148)
- [`enum ChildProcessSucceeded`](../server_runtime_http/src/child_process.rs#L162)
- [`struct ChildProcessReport`](../server_runtime_http/src/child_process.rs#L169)
- [`struct TokioManagedChild`](../server_runtime_http/src/child_process.rs#L192)
- [`struct TokioChildProcess`](../server_runtime_http/src/child_process.rs#L195)
- [`struct TokioChildDiagnosticTask`](../server_runtime_http/src/child_process.rs#L198)
- [`struct ChildProcessSupervisor`](../server_runtime_http/src/child_process.rs#L204)
- [`enum ChildProcessError`](../server_runtime_http/src/child_process.rs#L267)
- [`struct ChildProcessIoError`](../server_runtime_http/src/child_process.rs#L285)
- [`struct TokioChildProcessJoinError`](../server_runtime_http/src/child_process.rs#L290)

## `server_runtime_http/src/client_ip.rs`

- [`struct HttpHeaderMapRef`](../server_runtime_http/src/client_ip.rs#L5)
- [`struct TrustedProxyRangesTextRef`](../server_runtime_http/src/client_ip.rs#L8)
- [`struct ClientSocketAddr`](../server_runtime_http/src/client_ip.rs#L20)
- [`struct ResolvedClientIpAddr`](../server_runtime_http/src/client_ip.rs#L33)
- [`struct TrustedProxyRange`](../server_runtime_http/src/client_ip.rs#L35)
- [`struct IpnetNetwork`](../server_runtime_http/src/client_ip.rs#L47)
- [`struct ParsedIpAddr`](../server_runtime_http/src/client_ip.rs#L57)
- [`struct StdRangeContains`](../server_runtime_http/src/client_ip.rs#L68)
- [`struct ClientAddrParseError`](../server_runtime_http/src/client_ip.rs#L79)
- [`struct ParseIntError`](../server_runtime_http/src/client_ip.rs#L85)
- [`enum TrustedProxyRangeParseError`](../server_runtime_http/src/client_ip.rs#L88)
- [`enum TrustedProxyRangesParseError`](../server_runtime_http/src/client_ip.rs#L105)
- [`struct TrustedProxyRanges`](../server_runtime_http/src/client_ip.rs#L136)
- [`struct TrustedProxyRangesError`](../server_runtime_http/src/client_ip.rs#L147)

## `server_runtime_http/src/cors.rs`

- [`struct HttpCorsAllowOriginTextRef`](../server_runtime_http/src/cors.rs#L5)
- [`struct HttpCorsAllowOriginHeaderValues`](../server_runtime_http/src/cors.rs#L10)
- [`enum HttpCorsAllowOriginHeaderValuesError`](../server_runtime_http/src/cors.rs#L15)

## `server_runtime_http/src/csp.rs`

- [`struct HttpCspDirectiveName`](../server_runtime_http/src/csp.rs#L6)
- [`struct HttpCspDirectiveValue`](../server_runtime_http/src/csp.rs#L9)
- [`enum HttpCspTokenError`](../server_runtime_http/src/csp.rs#L14)
- [`struct HttpCspBuilder`](../server_runtime_http/src/csp.rs#L64)
- [`struct HttpCspMaximumBytesError`](../server_runtime_http/src/csp.rs#L81)

## `server_runtime_http/src/fallback.rs`

- [`enum FallbackResponseMode`](../server_runtime_http/src/fallback.rs#L8)
- [`struct HttpFallbackRequestPathRef`](../server_runtime_http/src/fallback.rs#L14)
- [`struct HttpFallbackApiPrefixRef`](../server_runtime_http/src/fallback.rs#L17)
- [`struct HttpFallbackMetricsPathRef`](../server_runtime_http/src/fallback.rs#L20)
- [`struct HttpOptionalAcceptHeaderRef`](../server_runtime_http/src/fallback.rs#L23)
- [`struct HttpAcceptHeaderMaximumBytes`](../server_runtime_http/src/fallback.rs#L26)
- [`struct HttpMediaRangeRef`](../server_runtime_http/src/fallback.rs#L29)
- [`struct AcceptsApplicationJson`](../server_runtime_http/src/fallback.rs#L32)

## `server_runtime_http/src/geojson.rs`

- [`struct GeoJsonDocumentText`](../server_runtime_http/src/geojson.rs#L4)
- [`struct SerdeJsonGeoJsonError`](../server_runtime_http/src/geojson.rs#L27)
- [`enum GeoJsonValidationError`](../server_runtime_http/src/geojson.rs#L30)
- [`trait GeoJsonValidation`](../server_runtime_http/src/geojson.rs#L43)
- [`trait SupportedGeoJsonTypeValidation`](../server_runtime_http/src/geojson.rs#L47)

## `server_runtime_http/src/header_text.rs`

- [`struct HttpHeaderTextMaximumBytes`](../server_runtime_http/src/header_text.rs#L10)
- [`struct HttpHeaderTextBytes`](../server_runtime_http/src/header_text.rs#L21)
- [`struct HttpHeaderName`](../server_runtime_http/src/header_text.rs#L30)
- [`struct HttpHeaderTextRef`](../server_runtime_http/src/header_text.rs#L42)
- [`struct HttpHeaderTextMaximumBytesError`](../server_runtime_http/src/header_text.rs#L48)
- [`enum HttpHeaderTextResolution`](../server_runtime_http/src/header_text.rs#L62)

## `server_runtime_http/src/health.rs`

- [`struct HealthProbeTimeoutDuration`](../server_runtime_http/src/health.rs#L10)
- [`struct HealthProbeSucceeded`](../server_runtime_http/src/health.rs#L22)
- [`enum HealthComponentStatus`](../server_runtime_http/src/health.rs#L28)
- [`struct HealthSnapshot`](../server_runtime_http/src/health.rs#L36)
- [`enum HealthReadyError`](../server_runtime_http/src/health.rs#L41)
- [`struct ServiceLivenessSnapshot`](../server_runtime_http/src/health.rs#L48)
- [`struct SharedHealthReadinessArc`](../server_runtime_http/src/health.rs#L72)
- [`struct HealthReadiness`](../server_runtime_http/src/health.rs#L75)

## `server_runtime_http/src/http_client.rs`

- [`struct ReqwestClient`](../server_runtime_http/src/http_client.rs#L8)
- [`struct ReqwestConnectTimeoutDuration`](../server_runtime_http/src/http_client.rs#L11)
- [`struct ReqwestRequestTimeoutDuration`](../server_runtime_http/src/http_client.rs#L14)
- [`struct StdReqwestTimeoutError`](../server_runtime_http/src/http_client.rs#L20)
- [`struct ReqwestClientPolicy`](../server_runtime_http/src/http_client.rs#L47)
- [`struct ReqwestClientBuildError`](../server_runtime_http/src/http_client.rs#L69)
- [`struct TracingHttpClientSpan`](../server_runtime_http/src/http_client.rs#L72)

## `server_runtime_http/src/http_error_diagnostic.rs`

- [`struct HttpErrorCode`](../server_runtime_http/src/http_error_diagnostic.rs#L9)
- [`struct HttpErrorType`](../server_runtime_http/src/http_error_diagnostic.rs#L19)
- [`struct HttpErrorTelemetry`](../server_runtime_http/src/http_error_diagnostic.rs#L22)
- [`struct StdHttpErrorBacktrace`](../server_runtime_http/src/http_error_diagnostic.rs#L30)
- [`struct StdHttpErrorChain`](../server_runtime_http/src/http_error_diagnostic.rs#L35)
- [`struct TracingHttpSpanTrace`](../server_runtime_http/src/http_error_diagnostic.rs#L40)
- [`struct HttpErrorDiagnostic`](../server_runtime_http/src/http_error_diagnostic.rs#L43)
- [`struct HttpErrorWithoutDiagnosticContext`](../server_runtime_http/src/http_error_diagnostic.rs#L134)

## `server_runtime_http/src/http_header_policy.rs`

- [`struct HttpAttachmentFileNameRef`](../server_runtime_http/src/http_header_policy.rs#L14)
- [`struct HttpContentDisposition`](../server_runtime_http/src/http_header_policy.rs#L23)
- [`enum HttpContentDispositionError`](../server_runtime_http/src/http_header_policy.rs#L28)
- [`enum HttpContentLengthError`](../server_runtime_http/src/http_header_policy.rs#L88)
- [`struct HttpContentLength`](../server_runtime_http/src/http_header_policy.rs#L101)

## `server_runtime_http/src/http_policy.rs`

- [`struct HttpAuthorizationHeaderTextRef`](../server_runtime_http/src/http_policy.rs#L6)
- [`struct HttpBearerTokenRef`](../server_runtime_http/src/http_policy.rs#L17)
- [`enum BearerAuthorizationResolution`](../server_runtime_http/src/http_policy.rs#L24)
- [`struct HttpCookieHeadersRef`](../server_runtime_http/src/http_policy.rs#L53)
- [`struct HttpCookieNameRef`](../server_runtime_http/src/http_policy.rs#L56)
- [`struct HttpCookieValueRef`](../server_runtime_http/src/http_policy.rs#L68)
- [`enum CookieResolution`](../server_runtime_http/src/http_policy.rs#L75)
- [`struct HttpContentTypeTextRef`](../server_runtime_http/src/http_policy.rs#L135)
- [`enum OptionalJsonContentType`](../server_runtime_http/src/http_policy.rs#L138)
- [`enum OptionalJsonBodyPresence`](../server_runtime_http/src/http_policy.rs#L163)
- [`enum OptionalJsonContentTypeDecision`](../server_runtime_http/src/http_policy.rs#L168)

## `server_runtime_http/src/http_status_error.rs`

- [`struct HttpErrorStatus`](../server_runtime_http/src/http_status_error.rs#L10)
- [`enum HttpErrorClass`](../server_runtime_http/src/http_status_error.rs#L13)

## `server_runtime_http/src/lib.rs`

- [`struct AxumRouter`](../server_runtime_http/src/lib.rs#L190)
- [`struct HttpRequestSpanConfig`](../server_runtime_http/src/lib.rs#L194)
- [`struct RequestIdLayer`](../server_runtime_http/src/lib.rs#L214)
- [`struct RequestIdTowerLayer`](../server_runtime_http/src/lib.rs#L233)
- [`struct RequestIdService`](../server_runtime_http/src/lib.rs#L237)

## `server_runtime_http/src/lifecycle.rs`

- [`enum BackgroundTaskOutcome`](../server_runtime_http/src/lifecycle.rs#L2)
- [`struct TokioTaskJoinError`](../server_runtime_http/src/lifecycle.rs#L10)
- [`struct TokioAbortTask`](../server_runtime_http/src/lifecycle.rs#L12)
- [`enum BackgroundTaskShutdownError`](../server_runtime_http/src/lifecycle.rs#L15)
- [`struct BackgroundTask`](../server_runtime_http/src/lifecycle.rs#L23)
- [`struct TokioBackgroundTaskJoinHandle`](../server_runtime_http/src/lifecycle.rs#L28)
- [`struct TokioBackgroundTaskShutdownSender`](../server_runtime_http/src/lifecycle.rs#L31)
- [`struct RunIntervalDuration`](../server_runtime_http/src/lifecycle.rs#L79)
- [`struct StdRunIntervalTryFromDurationError`](../server_runtime_http/src/lifecycle.rs#L94)
- [`struct RequestTimeoutDuration`](../server_runtime_http/src/lifecycle.rs#L96)
- [`struct StdRequestTimeoutTryFromDurationError`](../server_runtime_http/src/lifecycle.rs#L116)

## `server_runtime_http/src/limits.rs`

- [`struct PermitWaitTimeoutDuration`](../server_runtime_http/src/limits.rs#L10)
- [`struct RetryAfterSecs`](../server_runtime_http/src/limits.rs#L13)
- [`struct RetryAfterSecsTryFromU64Error`](../server_runtime_http/src/limits.rs#L28)
- [`struct ArcTokioSemaphore`](../server_runtime_http/src/limits.rs#L36)
- [`struct SemaphorePermitCountNonZeroUsize`](../server_runtime_http/src/limits.rs#L46)
- [`struct TokioAcquireError`](../server_runtime_http/src/limits.rs#L68)
- [`enum AcquirePermitError`](../server_runtime_http/src/limits.rs#L70)
- [`struct TokioOwnedSemaphorePermit`](../server_runtime_http/src/limits.rs#L77)

## `server_runtime_http/src/metrics_layer.rs`

- [`struct MetricsResponseBody`](../server_runtime_http/src/metrics_layer.rs#L7)
- [`struct MetricsResponseBodyError`](../server_runtime_http/src/metrics_layer.rs#L30)
- [`struct HttpMetricsPathCacheMaximum`](../server_runtime_http/src/metrics_layer.rs#L33)
- [`struct HttpMetricsPathCacheMaximumTryFromUsizeError`](../server_runtime_http/src/metrics_layer.rs#L55)
- [`struct HttpMetricsPathCache`](../server_runtime_http/src/metrics_layer.rs#L58)
- [`struct HttpMetricsPathEntriesRwLock`](../server_runtime_http/src/metrics_layer.rs#L65)
- [`struct MetricsSharedString`](../server_runtime_http/src/metrics_layer.rs#L70)
- [`struct HttpMetricsPathText`](../server_runtime_http/src/metrics_layer.rs#L81)
- [`struct HttpMetricsPathTextError`](../server_runtime_http/src/metrics_layer.rs#L96)
- [`struct HttpMetricsPathTextRef`](../server_runtime_http/src/metrics_layer.rs#L99)
- [`struct SharedHttpMetricsPathCacheArc`](../server_runtime_http/src/metrics_layer.rs#L102)
- [`struct HttpMetricsLayer`](../server_runtime_http/src/metrics_layer.rs#L162)
- [`struct HttpMetricsTowerLayer`](../server_runtime_http/src/metrics_layer.rs#L191)
- [`struct HttpMetricsService`](../server_runtime_http/src/metrics_layer.rs#L207)

## `server_runtime_http/src/multipart.rs`

- [`struct MultipartPayloadMaximum`](../server_runtime_http/src/multipart.rs#L14)
- [`struct MultipartValueLength`](../server_runtime_http/src/multipart.rs#L27)
- [`enum MultipartValueError`](../server_runtime_http/src/multipart.rs#L32)
- [`struct MultipartFieldName`](../server_runtime_http/src/multipart.rs#L50)
- [`struct MultipartFileName`](../server_runtime_http/src/multipart.rs#L71)
- [`struct MultipartTextValue`](../server_runtime_http/src/multipart.rs#L100)
- [`struct MultipartBytes`](../server_runtime_http/src/multipart.rs#L118)
- [`struct MultipartTextPart`](../server_runtime_http/src/multipart.rs#L130)
- [`struct MultipartBytesPart`](../server_runtime_http/src/multipart.rs#L150)
- [`struct MultipartBytesParts`](../server_runtime_http/src/multipart.rs#L165)
- [`struct MultipartTextParts`](../server_runtime_http/src/multipart.rs#L177)
- [`enum MultipartRequestError`](../server_runtime_http/src/multipart.rs#L209)
- [`struct MultipartUploadRequest`](../server_runtime_http/src/multipart.rs#L217)
- [`enum FileStagingAction`](../server_runtime_http/src/multipart.rs#L287)
- [`struct FileStagingDirectoryName`](../server_runtime_http/src/multipart.rs#L295)
- [`struct StoragePathSegment`](../server_runtime_http/src/multipart.rs#L320)
- [`struct StoragePathSegmentError`](../server_runtime_http/src/multipart.rs#L325)
- [`struct StorageRelativePathBuf`](../server_runtime_http/src/multipart.rs#L349)

## `server_runtime_http/src/notification.rs`

- [`struct NotificationApiToken`](../server_runtime_http/src/notification.rs#L2)
- [`struct NotificationApiTokenRef`](../server_runtime_http/src/notification.rs#L5)
- [`struct NotificationApiTokenAuthorized`](../server_runtime_http/src/notification.rs#L22)
- [`enum NotificationApiTokenError`](../server_runtime_http/src/notification.rs#L62)
- [`struct NotificationMessage`](../server_runtime_http/src/notification.rs#L94)
- [`enum NotificationMessageError`](../server_runtime_http/src/notification.rs#L99)
- [`trait NotificationSender`](../server_runtime_http/src/notification.rs#L120)
- [`struct NotificationRequest`](../server_runtime_http/src/notification.rs#L133)
- [`struct NotificationServiceState`](../server_runtime_http/src/notification.rs#L144)
- [`struct AxumNotificationRouter`](../server_runtime_http/src/notification.rs#L167)
- [`struct HttpNotificationHeaderMap`](../server_runtime_http/src/notification.rs#L169)
- [`struct AxumNotificationState`](../server_runtime_http/src/notification.rs#L172)
- [`struct AxumNotificationJson`](../server_runtime_http/src/notification.rs#L193)

## `server_runtime_http/src/origin.rs`

- [`struct AllowedOrigin`](../server_runtime_http/src/origin.rs#L2)
- [`struct HttpOriginAuthorityText`](../server_runtime_http/src/origin.rs#L8)
- [`struct HttpOriginSchemeText`](../server_runtime_http/src/origin.rs#L46)
- [`struct AllowedOriginError`](../server_runtime_http/src/origin.rs#L94)
- [`struct AllowedOrigins`](../server_runtime_http/src/origin.rs#L97)
- [`struct AllowedOriginsError`](../server_runtime_http/src/origin.rs#L118)
- [`struct HttpOriginHeadersRef`](../server_runtime_http/src/origin.rs#L126)
- [`struct HttpOriginTextRef`](../server_runtime_http/src/origin.rs#L129)
- [`struct AllowOriginSuffix`](../server_runtime_http/src/origin.rs#L132)
- [`struct ParsedHttpOriginRef`](../server_runtime_http/src/origin.rs#L135)
- [`struct RequestOriginAllowed`](../server_runtime_http/src/origin.rs#L150)

## `server_runtime_http/src/outbound_url.rs`

- [`enum OutboundHostPolicy`](../server_runtime_http/src/outbound_url.rs#L2)
- [`enum OutboundUrlScheme`](../server_runtime_http/src/outbound_url.rs#L8)
- [`struct OutboundUrlTextRef`](../server_runtime_http/src/outbound_url.rs#L15)
- [`struct ReqwestOutboundUrl`](../server_runtime_http/src/outbound_url.rs#L18)
- [`struct OutboundAllowedHost`](../server_runtime_http/src/outbound_url.rs#L35)
- [`struct OutboundHostAllowlist`](../server_runtime_http/src/outbound_url.rs#L52)
- [`enum OutboundHostAllowlistError`](../server_runtime_http/src/outbound_url.rs#L94)
- [`struct OutboundIpAddr`](../server_runtime_http/src/outbound_url.rs#L113)
- [`struct OutboundUrlPolicy`](../server_runtime_http/src/outbound_url.rs#L117)
- [`enum OutboundUrlError`](../server_runtime_http/src/outbound_url.rs#L196)
- [`enum OutboundAddressDisposition`](../server_runtime_http/src/outbound_url.rs#L214)

## `server_runtime_http/src/path_policy.rs`

- [`struct HttpProxyPathRef`](../server_runtime_http/src/path_policy.rs#L6)
- [`struct HttpProxyPath`](../server_runtime_http/src/path_policy.rs#L11)
- [`enum HttpProxyPathError`](../server_runtime_http/src/path_policy.rs#L15)
- [`struct HttpAllowedPathPrefixRef`](../server_runtime_http/src/path_policy.rs#L80)
- [`struct HttpProxyPathPrefixMatch`](../server_runtime_http/src/path_policy.rs#L92)
- [`struct HttpRequestPathRef`](../server_runtime_http/src/path_policy.rs#L108)
- [`struct HttpNormalizedPath`](../server_runtime_http/src/path_policy.rs#L113)
- [`struct HttpNormalizedPathError`](../server_runtime_http/src/path_policy.rs#L118)

## `server_runtime_http/src/pg_rate_limit.rs`

- [`struct PgRateLimitQueryRef`](../server_runtime_http/src/pg_rate_limit.rs#L4)
- [`struct SqlxPgRateLimitPoolRef`](../server_runtime_http/src/pg_rate_limit.rs#L7)
- [`struct PgRateLimitScopeRef`](../server_runtime_http/src/pg_rate_limit.rs#L10)
- [`struct PgRateLimitSubjectRef`](../server_runtime_http/src/pg_rate_limit.rs#L26)
- [`struct PgRateLimitMaximum`](../server_runtime_http/src/pg_rate_limit.rs#L42)
- [`struct PgRateLimitWindowSeconds`](../server_runtime_http/src/pg_rate_limit.rs#L56)
- [`enum PgRateLimitDecision`](../server_runtime_http/src/pg_rate_limit.rs#L70)
- [`enum PgRateLimitValidationError`](../server_runtime_http/src/pg_rate_limit.rs#L78)
- [`struct SqlxPgRateLimitError`](../server_runtime_http/src/pg_rate_limit.rs#L95)
- [`enum PgRateLimitError`](../server_runtime_http/src/pg_rate_limit.rs#L98)

## `server_runtime_http/src/redacted_url.rs`

- [`struct RedactedUrl`](../server_runtime_http/src/redacted_url.rs#L2)
- [`struct RedactedUrlTextRef`](../server_runtime_http/src/redacted_url.rs#L25)

## `server_runtime_http/src/request_id.rs`

- [`struct RequestId`](../server_runtime_http/src/request_id.rs#L4)
- [`struct RequestIdTryFromStringError`](../server_runtime_http/src/request_id.rs#L23)
- [`struct HttpHeaderToStrError`](../server_runtime_http/src/request_id.rs#L28)
- [`enum RequestIdTryFromHttpHeaderValueError`](../server_runtime_http/src/request_id.rs#L30)

## `server_runtime_http/src/request_timeout.rs`

- [`struct RequestTimeoutLayer`](../server_runtime_http/src/request_timeout.rs#L2)
- [`enum RequestTimeoutError`](../server_runtime_http/src/request_timeout.rs#L5)
- [`struct StdRequestTimeoutMessage`](../server_runtime_http/src/request_timeout.rs#L13)
- [`struct RequestTimeoutBody`](../server_runtime_http/src/request_timeout.rs#L16)
- [`struct RequestTimeoutTowerLayer`](../server_runtime_http/src/request_timeout.rs#L41)
- [`struct RequestTimeoutService`](../server_runtime_http/src/request_timeout.rs#L44)

## `server_runtime_http/src/secure_cookie.rs`

- [`struct HttpCookieName`](../server_runtime_http/src/secure_cookie.rs#L2)
- [`struct HttpCookieValue`](../server_runtime_http/src/secure_cookie.rs#L37)
- [`struct StdCookieMaxAgeSeconds`](../server_runtime_http/src/secure_cookie.rs#L67)
- [`enum HttpCookieAccess`](../server_runtime_http/src/secure_cookie.rs#L70)
- [`enum HttpCookieSecure`](../server_runtime_http/src/secure_cookie.rs#L76)
- [`struct HttpSetCookieHeaderValue`](../server_runtime_http/src/secure_cookie.rs#L90)
- [`enum HttpSecureCookieError`](../server_runtime_http/src/secure_cookie.rs#L95)

## `server_runtime_http/src/security_headers.rs`

- [`enum ForwardedProtoTrust`](../server_runtime_http/src/security_headers.rs#L2)
- [`struct HttpContentSecurityPolicy`](../server_runtime_http/src/security_headers.rs#L8)
- [`struct HttpContentSecurityPolicyError`](../server_runtime_http/src/security_headers.rs#L12)
- [`struct SecurityHeadersLayer`](../server_runtime_http/src/security_headers.rs#L28)
- [`struct SecurityHeadersTowerLayer`](../server_runtime_http/src/security_headers.rs#L59)
- [`struct SecurityHeadersService`](../server_runtime_http/src/security_headers.rs#L65)

## `server_runtime_http/src/service.rs`

- [`struct ServiceRuntime`](../server_runtime_http/src/service.rs#L2)
- [`struct TokioTcpListener`](../server_runtime_http/src/service.rs#L26)
- [`struct ServeIoError`](../server_runtime_http/src/service.rs#L32)
- [`enum ServeWithGracefulShutdownError`](../server_runtime_http/src/service.rs#L35)

## `server_runtime_http/src/service_bootstrap.rs`

- [`struct TokioServiceRuntime`](../server_runtime_http/src/service_bootstrap.rs#L4)
- [`struct ServiceRuntimeIoError`](../server_runtime_http/src/service_bootstrap.rs#L10)

## `server_runtime_http/src/tests.rs`

- [`struct HttpErrorEventCapture`](../server_runtime_http/src/tests.rs#L8)
- [`struct HttpErrorEventFieldVisitor`](../server_runtime_http/src/tests.rs#L35)
- [`struct BoundaryTestError`](../server_runtime_http/src/tests.rs#L314)

## `server_runtime_http/src/trace_context.rs`

- [`struct HttpHeaderExtractor`](../server_runtime_http/src/trace_context.rs#L5)
- [`struct HttpHeaderInjector`](../server_runtime_http/src/trace_context.rs#L19)
- [`struct HttpTraceParent`](../server_runtime_http/src/trace_context.rs#L36)
- [`enum HttpTraceParentError`](../server_runtime_http/src/trace_context.rs#L41)
- [`struct HttpTraceState`](../server_runtime_http/src/trace_context.rs#L86)
- [`struct HttpTraceStateError`](../server_runtime_http/src/trace_context.rs#L92)
- [`struct OutboundTraceContext`](../server_runtime_http/src/trace_context.rs#L109)
- [`struct ReqwestRequestBuilder`](../server_runtime_http/src/trace_context.rs#L118)
- [`struct ReqwestRequest`](../server_runtime_http/src/trace_context.rs#L121)
- [`struct HttpOpentelemetryHeaderMapMut`](../server_runtime_http/src/trace_context.rs#L130)
- [`struct HttpOpentelemetryHeaderMapRef`](../server_runtime_http/src/trace_context.rs#L140)
- [`struct HttpHostRef`](../server_runtime_http/src/trace_context.rs#L151)
- [`struct HttpMethodRef`](../server_runtime_http/src/trace_context.rs#L162)
- [`struct OpentelemetryContext`](../server_runtime_http/src/trace_context.rs#L171)

## `server_runtime_http/src/wire_token.rs`

- [`enum VersionedUrlSafeWireTokenTextError`](../server_runtime_http/src/wire_token.rs#L4)
- [`struct VersionedUrlSafeWireTokenText`](../server_runtime_http/src/wire_token.rs#L14)

## `synchronization_service_runtime/src/domain_types.rs`

- [`struct SynchronizationRuntimeConfiguration`](../synchronization_service_runtime/src/domain_types.rs#L5)
- [`struct SynchronizationPayloadTooLarge`](../synchronization_service_runtime/src/domain_types.rs#L14)
- [`struct SynchronizationPayload`](../synchronization_service_runtime/src/domain_types.rs#L24)
- [`trait SynchronizationSource`](../synchronization_service_runtime/src/domain_types.rs#L38)

## `tests/src/code_style/advanced_policy.rs`

- [`struct AwaitVisitor`](../tests/src/code_style/advanced_policy.rs#L10)
- [`struct LockAcrossAwaitVisitor`](../tests/src/code_style/advanced_policy.rs#L21)
- [`struct LeakApiVisitor`](../tests/src/code_style/advanced_policy.rs#L113)
- [`struct SpawnConsumptionVisitor`](../tests/src/code_style/advanced_policy.rs#L160)
- [`struct SpawnLifecycleVisitor`](../tests/src/code_style/advanced_policy.rs#L228)
- [`struct RouteLiteralVisitor`](../tests/src/code_style/advanced_policy.rs#L261)
- [`struct SelectMacroVisitor`](../tests/src/code_style/advanced_policy.rs#L331)
- [`struct ExpressionPathVisitor`](../tests/src/code_style/advanced_policy.rs#L378)
- [`struct IgnoredMapErrBindingVisitor`](../tests/src/code_style/advanced_policy.rs#L393)
- [`struct RawVecTupleWrapperVisitor`](../tests/src/code_style/advanced_policy.rs#L431)
- [`struct FromVecImplVisitor`](../tests/src/code_style/advanced_policy.rs#L436)
- [`struct UsizeMaxExprVisitor`](../tests/src/code_style/advanced_policy.rs#L480)
- [`struct SharedDispatchVisitor`](../tests/src/code_style/advanced_policy.rs#L512)
- [`struct PublicApiVisitor`](../tests/src/code_style/advanced_policy.rs#L535)
- [`struct StructErrorVisitor`](../tests/src/code_style/advanced_policy.rs#L759)
- [`struct LoopAllocationVisitor`](../tests/src/code_style/advanced_policy.rs#L778)

## `tests/src/code_style/cargo_policy.rs`

- [`struct TestOwnershipException`](../tests/src/code_style/cargo_policy.rs#L315)

## `tests/src/code_style/domain_analysis.rs`

- [`struct StringWrapperNameVisitor`](../tests/src/code_style/domain_analysis.rs#L2)
- [`struct StringWrapperFromVisitor`](../tests/src/code_style/domain_analysis.rs#L16)
- [`struct LenCheckedFunctionNameVisitor`](../tests/src/code_style/domain_analysis.rs#L183)
- [`struct LenMethodCallVisitor`](../tests/src/code_style/domain_analysis.rs#L234)
- [`struct PublicTupleWrapperFieldVisitor`](../tests/src/code_style/domain_analysis.rs#L246)
- [`struct DirectDeserializeTupleWrapperVisitor`](../tests/src/code_style/domain_analysis.rs#L250)
- [`struct DeserializeConversionCallVisitor`](../tests/src/code_style/domain_analysis.rs#L254)
- [`struct ManualDeserializeTupleWrapperVisitor`](../tests/src/code_style/domain_analysis.rs#L258)
- [`struct TupleWrapperConversionCollector`](../tests/src/code_style/domain_analysis.rs#L263)
- [`struct DirectTupleWrapperConstructorVisitor`](../tests/src/code_style/domain_analysis.rs#L273)
- [`struct DeclaredDomainTypeVisitor`](../tests/src/code_style/domain_analysis.rs#L450)
- [`struct DomainTypePolicyVisitor`](../tests/src/code_style/domain_analysis.rs#L544)
- [`struct AnalyzerStateRawContainerFieldVisitor`](../tests/src/code_style/domain_analysis.rs#L552)
- [`struct HelperRawTextReturnVisitor`](../tests/src/code_style/domain_analysis.rs#L556)
- [`struct ExternalLeafWrapperNameVisitor`](../tests/src/code_style/domain_analysis.rs#L560)

## `tests/src/code_style/lint_sync.rs`

- [`enum LintProbeDisposition`](../tests/src/code_style/lint_sync.rs#L2)

## `tests/src/code_style/mod.rs`

- [`struct ExternalLeafWrapperNameException`](../tests/src/code_style/mod.rs#L20)
- [`enum RustOrClippy`](../tests/src/code_style/mod.rs#L25)

## `tests/src/code_style/reuse_policy.rs`

- [`struct FunctionBodyComplexity`](../tests/src/code_style/reuse_policy.rs#L260)
- [`struct FunctionBodyVisitor`](../tests/src/code_style/reuse_policy.rs#L265)
- [`struct ReviewedDuplicateGroup`](../tests/src/code_style/reuse_policy.rs#L272)

## `tests/src/code_style/runtime_analysis.rs`

- [`struct RuntimePanicExpectUnwrapVisitor`](../tests/src/code_style/runtime_analysis.rs#L2)
- [`struct RuntimeMutexVisitor`](../tests/src/code_style/runtime_analysis.rs#L33)
- [`struct RuntimeArcVisitor`](../tests/src/code_style/runtime_analysis.rs#L57)
- [`struct AsyncBlockingCallVisitor`](../tests/src/code_style/runtime_analysis.rs#L104)
- [`struct UnitTestExternalServiceVisitor`](../tests/src/code_style/runtime_analysis.rs#L186)

## `tests/src/code_style/secret_policy.rs`

- [`struct SecretBoxStringVisitor`](../tests/src/code_style/secret_policy.rs#L2)

## `tests/src/code_style/snapshot.rs`

- [`struct RsSourceFile`](../tests/src/code_style/snapshot.rs#L2)
- [`struct ProjectSourceFile`](../tests/src/code_style/snapshot.rs#L8)
- [`struct CargoTomlSourceFile`](../tests/src/code_style/snapshot.rs#L13)
- [`struct CodebaseSnapshot`](../tests/src/code_style/snapshot.rs#L19)
- [`struct CodebaseSourceSnapshot`](../tests/src/code_style/snapshot.rs#L24)

## `tests/src/code_style/source_analysis.rs`

- [`struct DbgVisitor`](../tests/src/code_style/source_analysis.rs#L2)
- [`struct OptimalMemoryLayoutVisitor`](../tests/src/code_style/source_analysis.rs#L7)
- [`struct TodoUnimplVisitor`](../tests/src/code_style/source_analysis.rs#L58)
- [`struct UnwrapVisitor`](../tests/src/code_style/source_analysis.rs#L78)
- [`struct ForLoopVisitor`](../tests/src/code_style/source_analysis.rs#L90)
- [`struct SourceDroppingMapErrVisitor`](../tests/src/code_style/source_analysis.rs#L95)
- [`struct NumericAsCastVisitor`](../tests/src/code_style/source_analysis.rs#L112)
- [`struct SerdeJsonValueFieldVisitor`](../tests/src/code_style/source_analysis.rs#L117)
- [`struct SerdeJsonValueTypeVisitor`](../tests/src/code_style/source_analysis.rs#L144)
- [`struct PublicStructFieldVisitor`](../tests/src/code_style/source_analysis.rs#L149)
- [`struct IncludeAssetMacroVisitor`](../tests/src/code_style/source_analysis.rs#L232)
- [`struct DirectPathCallVisitor`](../tests/src/code_style/source_analysis.rs#L247)
- [`struct UnboundedReadVisitor`](../tests/src/code_style/source_analysis.rs#L251)
- [`struct LostSpawnVisitor`](../tests/src/code_style/source_analysis.rs#L293)
- [`struct TestNondeterminismVisitor`](../tests/src/code_style/source_analysis.rs#L336)
- [`struct SensitiveTextDebugDeriveVisitor`](../tests/src/code_style/source_analysis.rs#L341)
- [`struct SensitiveErrorFormatVisitor`](../tests/src/code_style/source_analysis.rs#L345)
- [`struct GeneratedRandomnessVisitor`](../tests/src/code_style/source_analysis.rs#L349)
- [`struct StaticStateVisitor`](../tests/src/code_style/source_analysis.rs#L353)
- [`struct PrintMacroVisitor`](../tests/src/code_style/source_analysis.rs#L357)
- [`struct ProductionLinePrintMacroVisitor`](../tests/src/code_style/source_analysis.rs#L361)
- [`struct PublicLogicVisitor`](../tests/src/code_style/source_analysis.rs#L365)
- [`struct OwnedTestVisitor`](../tests/src/code_style/source_analysis.rs#L369)
- [`struct AllowReasonVisitor`](../tests/src/code_style/source_analysis.rs#L373)
- [`struct DiagnosticIdVisitor`](../tests/src/code_style/source_analysis.rs#L378)
- [`struct UseImportVisitor`](../tests/src/code_style/source_analysis.rs#L790)
- [`struct TypeAliasVisitor`](../tests/src/code_style/source_analysis.rs#L856)
- [`struct EmptyEnumVisitor`](../tests/src/code_style/source_analysis.rs#L869)
- [`struct InfallibleResultVisitor`](../tests/src/code_style/source_analysis.rs#L898)
- [`struct ConstantAliasVisitor`](../tests/src/code_style/source_analysis.rs#L962)
- [`struct ForwardingDerefVisitor`](../tests/src/code_style/source_analysis.rs#L993)
- [`struct ForwardingBorrowVisitor`](../tests/src/code_style/source_analysis.rs#L998)
- [`struct ForwardingDisplayVisitor`](../tests/src/code_style/source_analysis.rs#L1138)
- [`struct ManualErrorImplVisitor`](../tests/src/code_style/source_analysis.rs#L1142)
- [`struct ManualNotImplVisitor`](../tests/src/code_style/source_analysis.rs#L1146)
- [`struct ConstDisplayImplVisitor`](../tests/src/code_style/source_analysis.rs#L1150)
- [`struct JsonCallVisitor`](../tests/src/code_style/source_analysis.rs#L1222)
- [`struct JsonIntoResponseErrorVisitor`](../tests/src/code_style/source_analysis.rs#L1240)
- [`struct TupleResponseVisitor`](../tests/src/code_style/source_analysis.rs#L1245)
- [`struct ThiserrorEnumVisitor`](../tests/src/code_style/source_analysis.rs#L1305)
- [`struct ApiErrorLocationVisitor`](../tests/src/code_style/source_analysis.rs#L1372)
- [`struct IntoResponseTypeVisitor`](../tests/src/code_style/source_analysis.rs#L1377)
- [`struct ApiErrorSourceVisitor`](../tests/src/code_style/source_analysis.rs#L1397)
- [`struct RouteOperationErrorVisitor`](../tests/src/code_style/source_analysis.rs#L1402)
- [`struct ForwardingIntoIteratorVisitor`](../tests/src/code_style/source_analysis.rs#L1633)
- [`struct PassthroughIntoInnerFromVisitor`](../tests/src/code_style/source_analysis.rs#L1674)
- [`struct PassthroughFromVisitor`](../tests/src/code_style/source_analysis.rs#L1747)
- [`struct TestStringLiteralVisitor`](../tests/src/code_style/source_analysis.rs#L1839)
- [`struct ProductionStringLiteralVisitor`](../tests/src/code_style/source_analysis.rs#L1866)
- [`struct StringConstantDeclarationVisitor`](../tests/src/code_style/source_analysis.rs#L1908)
- [`struct ConstantInitializerStringLiteralVisitor`](../tests/src/code_style/source_analysis.rs#L1913)
- [`struct StringConstantVisitor`](../tests/src/code_style/source_analysis.rs#L2110)

## `tests/src/code_style/source_policy.rs`

- [`struct ReviewedPublicFields`](../tests/src/code_style/source_policy.rs#L2)
- [`struct StaticStateException`](../tests/src/code_style/source_policy.rs#L888)
- [`struct LegacySuppression`](../tests/src/code_style/source_policy.rs#L1199)

## `tests/src/code_style/types.rs`

- [`struct AnalyzerCount`](../tests/src/code_style/types.rs#L5)
- [`struct AnalyzerBool`](../tests/src/code_style/types.rs#L20)
- [`struct CargoTomlFileIdx`](../tests/src/code_style/types.rs#L30)
- [`struct AnalyzerChar`](../tests/src/code_style/types.rs#L37)
- [`struct CargoMetadata`](../tests/src/code_style/types.rs#L50)
- [`struct CargoMetadataRef`](../tests/src/code_style/types.rs#L59)
- [`struct CargoPackageIdRefHashSet`](../tests/src/code_style/types.rs#L72)
- [`struct ProcessOutputRef`](../tests/src/code_style/types.rs#L83)
- [`struct StaticStr`](../tests/src/code_style/types.rs#L85)
- [`struct StaticStrSliceRef`](../tests/src/code_style/types.rs#L92)
- [`struct SourceTextRef`](../tests/src/code_style/types.rs#L106)
- [`struct SourceTextRefHashSet`](../tests/src/code_style/types.rs#L121)
- [`struct SourceTextHashSet`](../tests/src/code_style/types.rs#L131)
- [`struct SynBlockRef`](../tests/src/code_style/types.rs#L140)
- [`struct DiagnosticMsgs`](../tests/src/code_style/types.rs#L151)
- [`struct DiagnosticMsgsMutRef`](../tests/src/code_style/types.rs#L159)
- [`struct SourceText`](../tests/src/code_style/types.rs#L166)
- [`struct SourceTextTryFromStringError`](../tests/src/code_style/types.rs#L173)
- [`struct SourceTextList`](../tests/src/code_style/types.rs#L202)
- [`struct SourceTextListRef`](../tests/src/code_style/types.rs#L211)
- [`struct SourceTextBTreeSet`](../tests/src/code_style/types.rs#L228)
- [`struct FunctionBodyHash`](../tests/src/code_style/types.rs#L240)
- [`struct RegexRegexRef`](../tests/src/code_style/types.rs#L250)
- [`struct FunctionBodyLocationsBTreeMap`](../tests/src/code_style/types.rs#L259)
- [`struct FunctionBodyLocationsBTreeMapMutRef`](../tests/src/code_style/types.rs#L269)
- [`struct SourceTextBTreeSetRef`](../tests/src/code_style/types.rs#L287)
- [`struct OwnedPathBuf`](../tests/src/code_style/types.rs#L301)
- [`struct PathRef`](../tests/src/code_style/types.rs#L310)
- [`struct SynFile`](../tests/src/code_style/types.rs#L318)
- [`struct SynFileRef`](../tests/src/code_style/types.rs#L327)
- [`struct SynAttributeRef`](../tests/src/code_style/types.rs#L336)
- [`struct SynAttributeListRef`](../tests/src/code_style/types.rs#L345)
- [`struct SynExprCallRef`](../tests/src/code_style/types.rs#L354)
- [`struct SynFieldsRef`](../tests/src/code_style/types.rs#L368)
- [`struct SynGenericsRef`](../tests/src/code_style/types.rs#L377)
- [`struct SynItemImplRef`](../tests/src/code_style/types.rs#L386)
- [`struct SynItemFnRef`](../tests/src/code_style/types.rs#L395)
- [`struct SynItemRef`](../tests/src/code_style/types.rs#L404)
- [`struct SynItemStructRef`](../tests/src/code_style/types.rs#L413)
- [`struct SynPathArgumentsRef`](../tests/src/code_style/types.rs#L422)
- [`struct SynPathSegmentRef`](../tests/src/code_style/types.rs#L431)
- [`struct SynPathRef`](../tests/src/code_style/types.rs#L450)
- [`struct SynSignatureRef`](../tests/src/code_style/types.rs#L459)
- [`struct SynTypePathRef`](../tests/src/code_style/types.rs#L468)
- [`struct SynTypeRef`](../tests/src/code_style/types.rs#L482)
- [`struct SynUseTreeRef`](../tests/src/code_style/types.rs#L491)
- [`struct SynIdentifierRef`](../tests/src/code_style/types.rs#L500)
- [`struct TomlTable`](../tests/src/code_style/types.rs#L514)
- [`struct TomlTableRef`](../tests/src/code_style/types.rs#L523)
- [`struct TomlValueRef`](../tests/src/code_style/types.rs#L537)
- [`struct TomlValue`](../tests/src/code_style/types.rs#L546)
- [`struct WalkdirWalkDir`](../tests/src/code_style/types.rs#L548)

## `tests/src/domain_type_policy_fixture.rs`

- [`struct DomainId`](../tests/src/domain_type_policy_fixture.rs#L3)
- [`struct DomainName`](../tests/src/domain_type_policy_fixture.rs#L5)
- [`enum DomainNameTryFromStringError`](../tests/src/domain_type_policy_fixture.rs#L9)
- [`struct DomainEntity`](../tests/src/domain_type_policy_fixture.rs#L39)
- [`enum DomainEvent`](../tests/src/domain_type_policy_fixture.rs#L44)
- [`struct DomainEvents`](../tests/src/domain_type_policy_fixture.rs#L49)

## `tests/trybuild/route_contract_catalog_missing_route.rs`

- [`enum BrokenCatalog`](../tests/trybuild/route_contract_catalog_missing_route.rs#L4)

## `tests/trybuild/route_contract_delegate_non_empty.rs`

- [`struct DelegateInput`](../tests/trybuild/route_contract_delegate_non_empty.rs#L2)
- [`struct DelegateError`](../tests/trybuild/route_contract_delegate_non_empty.rs#L11)

## `tests/trybuild/route_contract_page_catalog_non_unit.rs`

- [`enum BrokenPages`](../tests/trybuild/route_contract_page_catalog_non_unit.rs#L8)

## `tests/trybuild/route_contract_struct_api_non_named.rs`

- [`struct InvalidContract`](../tests/trybuild/route_contract_struct_api_non_named.rs#L4)

## `tests/trybuild/route_contract_wire_enum_duplicate.rs`

- [`enum DuplicateWireValue`](../tests/trybuild/route_contract_wire_enum_duplicate.rs#L4)

## `tests/trybuild/route_contract_wire_enum_non_unit.rs`

- [`enum NonUnitWireValue`](../tests/trybuild/route_contract_wire_enum_non_unit.rs#L4)

## `tests/trybuild/route_contract_wrong_family_empty.rs`

- [`struct EmptyRouteFamily`](../tests/trybuild/route_contract_wrong_family_empty.rs#L4)

## `tests/trybuild/route_contract_wrong_family_missing_attribute.rs`

- [`struct MissingAttributeRouteFamily`](../tests/trybuild/route_contract_wrong_family_missing_attribute.rs#L3)

## `tests/trybuild/route_contract_wrong_path_parameter.rs`

- [`struct ParameterizedTestRoute`](../tests/trybuild/route_contract_wrong_path_parameter.rs#L2)

## `tests/trybuild/route_contract_wrong_request.rs`

- [`struct Request`](../tests/trybuild/route_contract_wrong_request.rs#L3)
- [`struct Response`](../tests/trybuild/route_contract_wrong_request.rs#L6)
- [`struct Route`](../tests/trybuild/route_contract_wrong_request.rs#L8)

## `tests/trybuild/route_contract_wrong_response.rs`

- [`struct Request`](../tests/trybuild/route_contract_wrong_response.rs#L3)
- [`struct Response`](../tests/trybuild/route_contract_wrong_response.rs#L6)
- [`struct Route`](../tests/trybuild/route_contract_wrong_response.rs#L8)

## `tests/trybuild/route_contract_wrong_route.rs`

- [`struct FirstRequest`](../tests/trybuild/route_contract_wrong_route.rs#L3)
- [`struct SecondRequest`](../tests/trybuild/route_contract_wrong_route.rs#L6)
- [`struct Response`](../tests/trybuild/route_contract_wrong_route.rs#L9)
- [`struct FirstRoute`](../tests/trybuild/route_contract_wrong_route.rs#L11)
- [`struct SecondRoute`](../tests/trybuild/route_contract_wrong_route.rs#L13)

## `tests/trybuild/route_contract_wrong_transport.rs`

- [`struct Request`](../tests/trybuild/route_contract_wrong_transport.rs#L8)
- [`struct Response`](../tests/trybuild/route_contract_wrong_transport.rs#L11)
- [`struct AuthenticatedRoute`](../tests/trybuild/route_contract_wrong_transport.rs#L13)

## `text_policy/src/domain_types.rs`

- [`enum BoundedTextPolicyError`](../text_policy/src/domain_types.rs#L11)
- [`struct RequiredNulFreeBoundedText`](../text_policy/src/domain_types.rs#L23)
- [`struct NonEmptyTrimmedText`](../text_policy/src/domain_types.rs#L43)
- [`enum FixedLengthAsciiHexTextError`](../text_policy/src/domain_types.rs#L64)
- [`struct FixedLengthAsciiHexText`](../text_policy/src/domain_types.rs#L79)
- [`struct UrlSafeTokenPartMaximumBytes`](../text_policy/src/domain_types.rs#L105)
- [`struct UrlSafeTokenPartRef`](../text_policy/src/domain_types.rs#L116)
- [`enum UrlSafeTokenPartTextError`](../text_policy/src/domain_types.rs#L121)
- [`struct UrlSafeTokenPartText`](../text_policy/src/domain_types.rs#L133)
- [`struct PasswordTextRef`](../text_policy/src/domain_types.rs#L169)
- [`struct PasswordLength`](../text_policy/src/domain_types.rs#L185)
- [`struct PasswordLengthRange`](../text_policy/src/domain_types.rs#L188)
- [`struct PasswordLengthRangeError`](../text_policy/src/domain_types.rs#L202)
- [`enum PasswordPolicyViolation`](../text_policy/src/domain_types.rs#L219)

## `to_err_string/src/domain_types.rs`

- [`trait ToErrString`](../to_err_string/src/domain_types.rs#L33)
- [`struct ErrorText`](../to_err_string/src/domain_types.rs#L54)
- [`struct StaticStrToOwnedInput`](../to_err_string/src/domain_types.rs#L86)

## `token_patterns/src/domain_types.rs`

- [`struct ProcMacro2TokensMut`](../token_patterns/src/domain_types.rs#L2)

## `token_patterns_token_patterns_macros/src/domain_types.rs`

- [`struct ProcMacro2GenerateTpInput`](../token_patterns_token_patterns_macros/src/domain_types.rs#L4)
- [`struct ProcMacro2GenerateTpOutput`](../token_patterns_token_patterns_macros/src/domain_types.rs#L9)

## `workspace_macro_helpers/src/lib.rs`

- [`struct SynDeriveInputRef`](../workspace_macro_helpers/src/lib.rs#L4)
- [`enum SynStructShapeRef`](../workspace_macro_helpers/src/lib.rs#L17)
- [`struct SynFieldsNamedRef`](../workspace_macro_helpers/src/lib.rs#L23)
- [`struct SynFieldsUnnamedRef`](../workspace_macro_helpers/src/lib.rs#L42)
- [`struct ProcMacro2MacroTokens`](../workspace_macro_helpers/src/lib.rs#L79)
- [`struct ProcMacro2TopLevelCommaParts`](../workspace_macro_helpers/src/lib.rs#L145)
- [`struct TopLevelCommaPart`](../workspace_macro_helpers/src/lib.rs#L190)
- [`struct FirstIdentifier`](../workspace_macro_helpers/src/lib.rs#L234)
- [`struct FirstIdentifierifierTryFromStringError`](../workspace_macro_helpers/src/lib.rs#L236)
- [`struct UniqueOptionBTreeSet`](../workspace_macro_helpers/src/lib.rs#L271)
- [`struct StdUniqueOptionSetContains`](../workspace_macro_helpers/src/lib.rs#L280)
- [`struct StdUniqueOptionSetIsEmpty`](../workspace_macro_helpers/src/lib.rs#L293)
- [`struct FirstCommaStripped`](../workspace_macro_helpers/src/lib.rs#L339)
- [`struct PartIndex`](../workspace_macro_helpers/src/lib.rs#L354)
- [`struct ClosureIdentifierAndBody`](../workspace_macro_helpers/src/lib.rs#L456)

## `workspace_scaffold/src/main.rs`

- [`struct ProjectNameRef`](../workspace_scaffold/src/main.rs#L6)
- [`struct RepositoryUrlRef`](../workspace_scaffold/src/main.rs#L9)
- [`struct ServicePort`](../workspace_scaffold/src/main.rs#L12)
- [`struct ServiceCrate`](../workspace_scaffold/src/main.rs#L21)
- [`struct ServiceComposeName`](../workspace_scaffold/src/main.rs#L30)
- [`struct ServiceComposeFile`](../workspace_scaffold/src/main.rs#L39)
- [`struct ServiceDockerfile`](../workspace_scaffold/src/main.rs#L48)
- [`struct ServiceImage`](../workspace_scaffold/src/main.rs#L57)
- [`struct ServiceKubernetesManifest`](../workspace_scaffold/src/main.rs#L66)
- [`struct ServiceSocketEnv`](../workspace_scaffold/src/main.rs#L75)
- [`struct ServiceCatalogEntries`](../workspace_scaffold/src/main.rs#L77)
- [`struct ServiceCatalogEntriesRef`](../workspace_scaffold/src/main.rs#L81)
- [`struct ServiceCatalogEntry`](../workspace_scaffold/src/main.rs#L84)
- [`struct ShouldRelease`](../workspace_scaffold/src/main.rs#L103)
- [`struct IsCatalogPathSafe`](../workspace_scaffold/src/main.rs#L112)
- [`struct ServiceCatalogDraft`](../workspace_scaffold/src/main.rs#L115)
- [`struct ShouldWrite`](../workspace_scaffold/src/main.rs#L149)
- [`struct ScaffoldText`](../workspace_scaffold/src/main.rs#L159)
- [`struct ScaffoldTextRef`](../workspace_scaffold/src/main.rs#L161)
- [`struct ScaffoldPathRef`](../workspace_scaffold/src/main.rs#L163)
- [`struct ReplacementsRef`](../workspace_scaffold/src/main.rs#L165)
- [`struct CargoArgsRef`](../workspace_scaffold/src/main.rs#L167)
- [`struct UpdateEnvName`](../workspace_scaffold/src/main.rs#L169)
- [`enum GeneratedProjection`](../workspace_scaffold/src/main.rs#L171)
- [`struct ShouldSkip`](../workspace_scaffold/src/main.rs#L183)
- [`struct ScaffoldIoError`](../workspace_scaffold/src/main.rs#L188)
- [`struct ServerRuntimeBoundedReadError`](../workspace_scaffold/src/main.rs#L193)
- [`enum ScaffoldError`](../workspace_scaffold/src/main.rs#L196)

## `workspace_test_runner/src/admin_fixture.rs`

- [`struct AdminFixtureString`](../workspace_test_runner/src/admin_fixture.rs#L3)

## `workspace_test_runner/src/execution.rs`

- [`struct CommandIdx`](../workspace_test_runner/src/execution.rs#L4)
- [`struct CommandStartedAtInstant`](../workspace_test_runner/src/execution.rs#L11)
- [`struct CommandDuration`](../workspace_test_runner/src/execution.rs#L18)
- [`struct CommandDurationMillis`](../workspace_test_runner/src/execution.rs#L32)
- [`struct CommandSucceeded`](../workspace_test_runner/src/execution.rs#L34)
- [`struct CommandsRef`](../workspace_test_runner/src/execution.rs#L41)
- [`struct CommandProgramRef`](../workspace_test_runner/src/execution.rs#L56)
- [`struct CommandArgsRef`](../workspace_test_runner/src/execution.rs#L58)
- [`struct CommandText`](../workspace_test_runner/src/execution.rs#L63)
- [`struct CommandTexts`](../workspace_test_runner/src/execution.rs#L65)
- [`struct ExecutionIoError`](../workspace_test_runner/src/execution.rs#L72)
- [`struct TextRef`](../workspace_test_runner/src/execution.rs#L74)
- [`struct RunDirPathBuf`](../workspace_test_runner/src/execution.rs#L81)
- [`struct SummaryText`](../workspace_test_runner/src/execution.rs#L86)
- [`struct CommandRun`](../workspace_test_runner/src/execution.rs#L102)

## `workspace_test_runner/src/main.rs`

- [`struct MeasurementName`](../workspace_test_runner/src/main.rs#L11)
- [`struct CargoArgs`](../workspace_test_runner/src/main.rs#L18)
- [`struct StderrTextRef`](../workspace_test_runner/src/main.rs#L30)
- [`struct AnsiTextRef`](../workspace_test_runner/src/main.rs#L37)
- [`struct CleanAnsiText`](../workspace_test_runner/src/main.rs#L45)
- [`struct MemusageKey`](../workspace_test_runner/src/main.rs#L47)
- [`struct MemusageRowName`](../workspace_test_runner/src/main.rs#L54)
- [`struct MemusageColumnIdx`](../workspace_test_runner/src/main.rs#L61)
- [`struct MemusageValueRef`](../workspace_test_runner/src/main.rs#L68)
- [`struct ProgramPathRef`](../workspace_test_runner/src/main.rs#L75)
- [`struct ProgramArgsRef`](../workspace_test_runner/src/main.rs#L82)
- [`struct MemusageProgNameRef`](../workspace_test_runner/src/main.rs#L94)
- [`struct QuoteTokenStreamGeneratePgTableMeasureInputTokenStream`](../workspace_test_runner/src/main.rs#L103)
- [`struct ToolName`](../workspace_test_runner/src/main.rs#L105)
- [`struct ToolPath`](../workspace_test_runner/src/main.rs#L112)
- [`struct ToolAvailable`](../workspace_test_runner/src/main.rs#L119)
- [`struct RunnerIoErrorRef`](../workspace_test_runner/src/main.rs#L126)
- [`struct RunnerPathRef`](../workspace_test_runner/src/main.rs#L133)
- [`struct RunnerMode`](../workspace_test_runner/src/main.rs#L143)
- [`struct AllocationTool`](../workspace_test_runner/src/main.rs#L145)
