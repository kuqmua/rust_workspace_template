# Структуры-обёртки над типами

Этот документ содержит инвентаризацию одно-полевых tuple-структур (`struct Name(Type);`), объявленных в Rust-исходниках workspace. Именно такая форма считается здесь структурой-обёрткой над типом.

Включены production-, test-, bench- и example-модули всех workspace crates. Исключены `target/` и структуры, которые присутствуют только как токены внутри генераторов (`quote!`), поскольку они не являются объявленными items исходного crate.

Всего структур-обёрток: **930**.

## Crate `app_state`

### Модуль `app_state`

- `SqlxPgPool`
- `SqlxPgPoolRef`
## Crate `common_routes`

### Модуль `common_routes`

- `AxumCommonRoutes`
- `AxumHealthCheckStatus`
- `AxumHttpUriRef`
- `AxumJsonPayload`
- `HealthCheckSucceeded`
- `HealthComponents`
- `HealthDatabaseAvailable`
- `NoRouteMessageCapacity`
- `NotFoundMessage`
- `OpenApiSpecificationPath`
- `StdArcCommonRoutesAppState`
- `UriSuffixRef`
- `UtoipaCommonRoutesOpenApiDocument`
## Crate `config_lib`

### Модуль `config_lib`

- `AdminAccessTokenTtlSeconds`
- `AdminBoolParsingError`
- `AdminCookieSecure`
- `AdminJwtSecret`
- `AdminPasswordHashConcurrency`
- `AdminPositiveU64ParsingError`
- `AdminPositiveUsizeParsingError`
- `AdminRefreshTokenTtlSeconds`
- `AdminSessionLimit`
- `AdminSignInRateLimit`
- `AdminSwaggerEnabled`
- `AdminTokenAudience`
- `AdminTokenIssuer`
- `ChronoEastFixedOffset`
- `ChronoFixedOffsetError`
- `ChronoTimezone`
- `ConfigRustTypeName`
- `ContentSecurityPolicy`
- `EnvVarName`
- `EnvVarNameRef`
- `HttpGzipEnabled`
- `MaximumSizeOfHttpBodyInBytes`
- `PgPoolAcquireTimeoutSeconds`
- `PgPoolIdleTimeoutSeconds`
- `PgPoolMaxConnections`
- `PgPoolMaxLifetimeSeconds`
- `PgPoolMinConnections`
- `RequestTimeoutSeconds`
- `SecrecySecretBoxString`
- `StdEnvVarOk`
- `StdEnvVarOkRef`
- `StdI32ParsingError`
- `StdNonZeroU64`
- `StdNonZeroUsize`
- `StdParseBoolError`
- `StdParseIntError`
- `StdU32ParsingError`
- `StdUsizeParsingError`
- `TimezoneSeconds`
### Модуль `config_lib::types`

- `EnvParseError`
- `EnvVarNameRef`
- `EnvVarValueRef`
- `ParseCtxRef`
- `StdEnvVarResult`
- `TracingLevelName`
## Crate `config_lib_macros`

### Модуль `config_lib_macros`

- `ProcMacro2TryFromParseFixedErrorTy`
- `ProcMacro2TryFromParseInput`
- `ProcMacroTryFromParseTokenStream`
## Crate `development_data_bootstrap`

### Модуль `development_data_bootstrap`

- `DevelopmentIdentityCount`
- `DevelopmentIdentitySpecs`
## Crate `external_service_emulators`

### Модуль `external_service_emulators`

- `RemoteSyncRequestCount`
- `TokioMockNotificationReceiver`
- `TokioMockNotificationSender`
## Crate `file_storage`

### Модуль `file_storage`

- `DiskCacheEvictionPlan`
- `StdDiskCacheModifiedAt`
- `StdDiskCacheSize`
- `StdFileBytes`
- `StdFileStorageIoError`
- `StdFileStorageRoot`
- `StdStaleBefore`
- `StdStaleStagingEntryCount`
- `StdStaleStagingEntryLimit`
- `StdStorageOperationId`
- `StdStoragePathRef`
- `StdStorageRelativePath`
- `StorageDirectoryNameRef`
## Crate `frontend_contract`

### Модуль `frontend_contract`

- `ActionContracts`
- `ContractI64`
- `ContractStr`
- `FieldContracts`
- `FieldOrder`
- `FormValue`
- `FormValueError`
- `FormValueRef`
- `RouteContracts`
- `TransportBody`
- `TransportError`
- `TransportIdempotencyKey`
- `TransportIfMatch`
- `TransportPath`
- `TransportRetryAfter`
- `TransportStatus`
### Модуль `frontend_contract::auth_session_keep_alive`

- `StdAuthSessionInstant`
- `StdAuthSessionRefreshInterval`
### Модуль `frontend_contract::json_snapshot`

- `JsonContractSnapshot`
- `JsonSnapshotDynamicFieldRef`
### Модуль `frontend_contract::openapi_validation`

- `OpenApiContractText`
- `OpenApiContractTextError`
- `OpenApiResponseStatus`
- `RuntimeRoutesRef`
- `SerdeJsonOpenApiSerializationError`
### Модуль `frontend_contract::problem`

- `ApiProblemDetail`
- `ApiProblemField`
- `ApiProblemRequestId`
- `ApiProblemStatus`
- `ApiProblemViolations`
### Модуль `frontend_contract::route`

- `OpenApiSecuritySchemeRef`
- `ParameterizedRoutePath`
- `RouteBodyLimit`
- `RouteCoverageDescriptors`
- `RouteMetadataList`
- `RouteSchemaContracts`
- `UtoipaOpenApiPathParameter`
- `UtoipaOpenApiRouteSchema`
### Модуль `frontend_contract::route::tests`

- `Request`
- `Response`
### Модуль `frontend_contract::route_contract_validation`

- `HttpContractBody`
- `HttpContractStatus`
- `RouteContractMismatches`
### Модуль `frontend_contract::route_coverage`

- `RouteTestCategories`
### Модуль `frontend_contract::url_builder`

- `ApiUrl`
- `ApiUrlPathSegmentRef`
- `ApiUrlQueryComponentRef`
## Crate `frontend_contract_macros`

### Модуль `frontend_contract_macros`

- `StdBool`
- `SynExpr`
- `SynIdent`
- `SynRouteRegistryBindings`
- `SynRouteRegistryHandler`
- `SynRouteRegistryRoute`
- `SynRouteRegistryState`
- `SynType`
## Crate `generate_derive_token_stream_builder`

### Модуль `generate_derive_token_stream_builder`

- `SnakeCaseString`
- `ToSnakeCaseInput`
## Crate `generate_pg_table_src`

### Модуль `generate_pg_table_src::model`

- `GeneratePgTableFieldCount`
- `SynGeneratePgTableModelError`
- `SynGeneratePgTableModelInput`
### Модуль `generate_pg_table_src::pipeline`

- `SynBuiltGeneratePgTableInput`
- `SynGeneratePgTablePipelineError`
- `SynParsedGeneratePgTableInput`
- `SynValidatedGeneratePgTableInput`
### Модуль `generate_pg_table_src::source`

- `CompileErrorMessage`
- `TableTestNames`
## Crate `generate_pg_types_src`

### Модуль `generate_pg_types_src::source`

- `GeneratePgTypeRecords`
- `GeneratePgTypes`
- `GenerateSecretText`
- `ParsedGeneratePgTypesConfig`
- `PgSqlName`
- `PgTypesModelEntryCount`
- `SerdeJsonGeneratePgTypesError`
## Crate `generate_quotes`

### Модуль `generate_quotes`

- `ProcMacro2QuotedLiteralTokenStream`
- `QuoteChar`
- `QuotePanicId`
- `QuotePrefix`
- `QuotedLiteral`
## Crate `generate_where_filters_src`

### Модуль `generate_where_filters_src::bind`

- `FilterPlaceholderCount`
### Модуль `generate_where_filters_src::model`

- `BindCount`
- `FilterSpecValid`
- `FilterSqlOperator`
- `FilterSqlSuffix`
### Модуль `generate_where_filters_src::source`

- `ProcMacro2GenerateWhereFiltersInput`
- `ProcMacro2GenerateWhereFiltersTokenStream`
- `SerdeJsonGenerateWhereFiltersError`
- `ValidatedGenerateWhereFiltersConfig`
## Crate `git_info`

### Модуль `git_info`

- `GitCommitId`
- `GitCommitIdFallback`
- `GitCommitIdRef`
- `GitCommitLink`
- `GitCommitLinkCapacity`
- `GitCommitLinkOutputRefMut`
- `IsProjectCommit`
- `ProjectGitCommitLinkRef`
- `StdGitCommitIdCow`
- `StdGitCommitLinkCow`
- `ValidateProjectCommitError`
## Crate `location`

### Модуль `location`

- `SynItemEnumMutRef`
## Crate `location_lib`

### Модуль `location_lib::location`

- `ChronoLocationDateTime`
- `ChronoLocationDisplayTimezone`
- `LocationColumn`
- `LocationCommit`
- `LocationFile`
- `LocationFileRef`
- `LocationLine`
- `StdFmtRefMut`
- `StdLocationDuration`
- `StdTimeDurationNanos`
- `StdTimeDurationSecs`
## Crate `location_test`

### Модуль `location_test`

- `LocationTestCount`
- `LocationTestFlag`
- `LocationTestText`
## Crate `macro_clippy_check_common`

### Модуль `macro_clippy_check_common::tests`

- `StdTmpDir`
## Crate `macros_helpers`

### Модуль `macros_helpers::attr_identifier_str`

- `AttrIdentifierName`
### Модуль `macros_helpers::generate_field_location_new_token_stream`

- `FieldLocationColumn`
- `FieldLocationFile`
- `FieldLocationLine`
### Модуль `macros_helpers::generate_if_write_is_err_token_stream`

- `ProcMacro2IfWriteIsErrTokenStream`
### Модуль `macros_helpers::generate_simple_syn_punct`

- `SynPathSegment`
- `SynPathSegments`
### Модуль `macros_helpers::generated_rust_token_stream`

- `GeneratedRustTokenStream`
### Модуль `macros_helpers::get_macro_attr`

- `AttrPathMatches`
- `ProcMacro2MacroAttrMetaListTokenStreamRef`
- `SynMacroAttrRef`
### Модуль `macros_helpers::json_contract`

- `JsonFixtureRef`
- `SerdeJsonError`
### Модуль `macros_helpers::location`

- `CompileErrorMessage`
- `SynVariantRef`
### Модуль `macros_helpers::location_syn_field`

- `SynLocationField`
### Модуль `macros_helpers::rs_file_path`

- `StdRsFilePath`
### Модуль `macros_helpers::status_code`

- `SynStatusCodeVariantRef`
### Модуль `macros_helpers::syn_field`

- `SynFieldIdentifier`
- `SynFieldType`
- `SynFieldVis`
### Модуль `macros_helpers::test_database`

- `SanitizedDatabaseTarget`
- `UrlRef`
### Модуль `macros_helpers::test_hlp`

- `ExpectedFileContent`
- `ExpectedFileContentRef`
- `StdAssertFilePath`
- `StdAssertFilePathRef`
- `TestPathStem`
- `TestPathStemRef`
### Модуль `macros_helpers::tool_command`

- `StdOsString`
- `StdPathRef`
- `StdProcessCommand`
- `StdProcessExitStatus`
- `StdProcessOutput`
- `ToolArgRef`
- `ToolArgsRef`
- `ToolEnvKeyRef`
- `ToolEnvValueRef`
- `ToolProgramRef`
### Модуль `macros_helpers::wrap_derive`

- `ProcMacro2DeriveTokensRef`
### Модуль `macros_helpers::write_string_into_file`

- `ShouldWriteString`
- `StdWrittenFilePath`
- `StdWrittenFilePathRef`
- `StringFileContentRef`
### Модуль `macros_helpers::write_token_stream_into_file`

- `ProcMacro2TokenStreamRef`
- `ShouldWriteTokenStreamFlag`
- `StdRustfmtPath`
## Crate `naming`

### Модуль `naming`

- `SwaggerUrlPathPrefix`
- `SwaggerUrlPathSelfQuotesStrValue`
- `SwaggerUrlPathSelfQuotesTokenStreamValue`
## Crate `naming_common`

### Модуль `naming_common`

- `CaseString`
- `ConvertCaseKind`
- `ProcMacro2CaseTokenStream`
## Crate `naming_macros`

### Модуль `naming_macros`

- `ProcMacro2GeneratedNamingTokenStream`
- `ProcMacro2VariantMatchingTokensRef`
- `SynEnumIdentifierRef`
## Crate `newtype`

### Модуль `newtype`

- `NewtypeBool`
- `ProcMacro2GeneratedTokenStream`
- `ProcMacroInputTokenStream`
- `SnakeIdentifier`
- `SnakeIdentifierifierLen`
- `SynAttrsRef`
- `SynDeriveInputRef`
- `SynExpr`
- `SynIdentifier`
- `SynIdentifierRef`
- `SynType`
- `SynTypeRef`
### Модуль `newtype::tests::newtype::tests`

- `CheckedText`
- `DebugValue`
- `DescribedValue`
- `ExplicitErrorCheckedText`
- `InnerValue`
- `InnerVecValue`
- `MutableValueRef`
- `OwnedSliceValue`
- `OwnedValue`
- `ProcMacro2TokenValue`
- `RedactedDebugValue`
- `ReferentValueRef`
- `RichValue`
- `SliceValueRef`
- `StdTransparentErrorValue`
- `StringValue`
- `TargetVecValue`
- `TransparentDebugValue`
- `UsizeValue`
- `ValidatedValue`
- `VecValue`
### Модуль `newtype::tests::newtype::tests::to_err_string`

- `ToErrStringValue`
## Crate `notification_service`

### Модуль `notification_service`

- `AxumNotificationJson`
- `AxumNotificationResponse`
- `AxumNotificationRouter`
- `AxumNotificationState`
- `HttpNotificationApiProblem`
- `HttpNotificationStatusCode`
- `MetricsExporterPrometheusHandle`
- `MetricsExporterPrometheusNotificationBuildError`
- `NotificationBodyMaximumBytes`
- `NotificationConfigError`
- `NotificationServeError`
- `SqlxNotificationDatabaseError`
- `SqlxNotificationMigrationError`
- `StdNotificationExitCode`
- `StdNotificationIoError`
## Crate `notification_service_contract`

### Модуль `notification_service_contract`

- `NotificationMessage`
- `UuidNotificationId`
## Crate `optml`

### Модуль `optml`

- `SynFieldTyWithStaticLts`
## Crate `panic_location`

### Модуль `panic_location`

- `PanicColumn`
- `PanicFile`
- `PanicLine`
- `PanicWithLocationMessage`
## Crate `pg_crud_common`

### Модуль `pg_crud_common`

- `AddOperator`
- `AllEnumVariants`
- `EqOperatorQueryStr`
- `IsPrimaryKey`
- `IsStringEmptyRes`
- `NonPrimaryKeyPgTypeReadIds`
- `NotEmptyUniqueVec`
- `NotZeroUnsignedPartOfI32`
- `NullableJsonObjPgTypeWhereFilter`
- `OrderSnakeCaseStr`
- `OrderUpperCamelCaseStr`
- `PaginationStartsWithZero`
- `SqlxPostgresQuery`
- `UnsignedPartOfI32`
- `UnsignedPartOfI32Raw`
- `UuidUuidTestCases`
### Модуль `pg_crud_common::advisory_lock`

- `PgRelationCapacityMaximum`
- `PgRelationLockNamespace`
- `PgRelationResourceId`
- `PgRelationResourceIds`
- `PgRelationRowCount`
- `SqlxPgRelationLockConnectionRef`
- `SqlxPgRelationLockError`
### Модуль `pg_crud_common::batch_validation`

- `BatchInvalidItemCount`
- `BatchInvalidItems`
- `BatchProcessedItemCount`
- `BatchStoppedEarly`
- `StdBatchRecords`
### Модуль `pg_crud_common::bind_index`

- `QueryPartIncrement`
### Модуль `pg_crud_common::bounded_btree_map`

- `StdBoundedBTreeMap`
- `StdBoundedBTreeMapLen`
- `StdBoundedBTreeMapVisitor`
### Модуль `pg_crud_common::bounded_unique_vec`

- `BoundedUniqueVec`
- `StdBoundedUniqueVecVisitor`
- `UniqueVecLen`
### Модуль `pg_crud_common::bounded_vec`

- `BoundedVec`
- `BoundedVecLen`
- `StdPhantomDataBoundedVecVisitor`
### Модуль `pg_crud_common::cardinality`

- `DuplicateCandidates`
- `DuplicateIdx`
### Модуль `pg_crud_common::cursor`

- `CursorMaximumLength`
- `CursorPayload`
- `CursorSigningKey`
- `SignedCursor`
### Модуль `pg_crud_common::date_sql_filter`

- `ChronoUtcDateTimeRef`
- `ChronoUtcDateTimes`
- `StdDateSqlBindStart`
### Модуль `pg_crud_common::db_schema_conformance`

- `DbColumnContractSnapshots`
- `DbColumnHasServerDefault`
- `DbColumnNullable`
- `DbColumnSnapshots`
- `DbColumnSpecs`
- `DbDefaultSpecs`
- `DbKeyContractSnapshots`
- `DbKeySpecs`
- `DbObjectSnapshots`
- `DbObjectSpecs`
- `DbSchemaNameRef`
- `DbSchemaText`
- `DbSchemaTextError`
- `DbSchemaTexts`
- `DbStaticSchemaText`
- `DbStaticSchemaTexts`
- `DbTableNameRef`
- `SqlxDbSchemaInspectionError`
- `SqlxPgPoolRef`
### Модуль `pg_crud_common::errors`

- `SqlxPostgresQueryBindError`
### Модуль `pg_crud_common::filter_bind_plan`

- `PgFilterBindValues`
- `PgFilterBool`
- `PgFilterI64`
- `PgFilterText`
### Модуль `pg_crud_common::finite_f64`

- `FiniteF64`
- `PositiveFiniteF64`
- `UnitIntervalF64`
### Модуль `pg_crud_common::invariants`

- `PaginationTotal`
### Модуль `pg_crud_common::list_total`

- `ListItems`
- `ListOffset`
- `ListTotal`
### Модуль `pg_crud_common::operation_budget`

- `OperationBudget`
- `OperationCount`
### Модуль `pg_crud_common::operational_invariants`

- `PgCounterValue`
- `PgOperationalLimit`
- `PgScopedForeignKeyClauseText`
- `PgSqlIdentifiers`
### Модуль `pg_crud_common::order_preserving_deduplication`

- `OrderPreservingValues`
### Модуль `pg_crud_common::pagination`

- `PaginationEnd`
- `PaginationLimit`
- `PaginationOffset`
- `PaginationStart`
### Модуль `pg_crud_common::pg_error`

- `SqlxPgErrorRef`
### Модуль `pg_crud_common::query_fragment`

- `QueryPartFragment`
- `SqlColumnRef`
### Модуль `pg_crud_common::read_query_plan`

- `ReadQueryPlan`
- `SqlSortOrderText`
- `StdReadQueryBindIndex`
### Модуль `pg_crud_common::sql_identifier`

- `SqlIdentifier`
- `SqlIdentifiers`
- `SqlQueryText`
### Модуль `pg_crud_common::sql_like_pattern`

- `SqlLikeInputRef`
- `SqlLikePattern`
### Модуль `pg_crud_common::tests_not_empty_unique_vec`

- `NonClone`
## Crate `pg_crud_macros_common`

### Модуль `pg_crud_macros_common`

- `DeLen`
- `DimensionNumber`
- `GeneratedRustTokenStreamVec`
- `ImportPathStr`
- `ImportSnakeCaseStr`
- `IsNullablePrefixStr`
- `NonNullOrNullableStr`
- `PanicUuidRef`
- `ParseErrorIdRef`
- `ParseTokenStreamStrings`
- `StructElsLen`
- `SynFieldRefs`
- `SynIdentifierTypeRefs`
- `WrapIntoBraces`
## Crate `pg_table`

### Модуль `pg_table`

- `PgTableIdempotencyActor`
- `PgTableIdempotencyBody`
- `PgTableIdempotencyBodyRef`
- `PgTableIdempotencyCleanupBatchSize`
- `PgTableIdempotencyCleanupRetentionSeconds`
- `PgTableIdempotencyCleanupRows`
- `PgTableIdempotencyKey`
- `PgTableIdempotencyMethod`
- `PgTableIdempotencyRequestHash`
- `PgTableIdempotencyResponseStatus`
- `PgTableIdempotencyRoute`
- `PgTableIdempotencyTextBytes`
- `PgTableNameRef`
- `PgTableQueryPartFragment`
- `PgTableQueryString`
- `PgTableRevision`
- `PgTableSqlFragmentRef`
- `SqlxPgTableIdempotencyError`
- `SqlxPgTablePgConnectionRef`
- `StdPgTableRevisionParseIntError`
## Crate `pg_types_common`

### Модуль `pg_types_common`

- `IsPrimaryKey`
- `PaginationStartsWithOne`
- `PaginationStartsWithOneValue`
## Crate `prepare_postgresql_databases`

### Модуль `prepare_postgresql_databases`

- `DatabaseUrl`
- `MigrationsSource`
- `ProcessArguments`
- `ProcessCommands`
- `ProcessProgram`
- `ProcessStaticArgument`
## Crate `route_validators`

### Модуль `route_validators`

- `AxumHttpStatusCode`
### Модуль `route_validators::check_body_size`

- `AxumBody`
- `AxumBodySizeError`
- `BodySizeLimitBytes`
- `BytesBodyBytes`
- `HttpBodySizeHint`
### Модуль `route_validators::check_commit`

- `AxumCommitToStrConversionError`
- `CommitNotEqMessage`
- `CommitToUse`
- `EnableApiGitCommitCheck`
- `NoCommitHeaderMessage`
### Модуль `route_validators::hdr_val`

- `AxumHeaderValueRef`
- `AxumHeadersRef`
- `HeaderStrRef`
### Модуль `route_validators::test_hlp`

- `AxumTestHeaderValue`
- `AxumTestHeaders`
- `AxumTestHeadersMutRef`
- `TestExpId`
- `TestPanicText`
- `TestPollCount`
- `TestPollLimitReached`
## Crate `server`

### Модуль `server`

- `AxumApiRoutes`
- `MetricsExporterPrometheusBuildError`
- `MetricsExporterPrometheusHandle`
- `ServerAdminAuthSvcStateBuildError`
- `ServerAdminCleanupCfgError`
- `ServerAdminMigrateError`
- `ServerConfigError`
- `ServerRuntimeBackgroundTaskShutdownError`
- `ServerRuntimeContentSecurityPolicyError`
- `ServerRuntimeRequestTimeoutError`
- `ServerRuntimeRunIntervalError`
- `ServerRuntimeServeError`
- `SqlxServerPgConnectError`
- `StdServerExitCode`
- `StdServerIoError`
- `StdSharedServerAppState`
- `TokioServerRuntime`
## Crate `server_admin`

### Модуль `server_admin`

- `AdminAccessTokenError`
- `AdminCleanupBatchSize`
- `AdminCleanupRetentionSeconds`
- `AdminCleanupRows`
- `AdminCookieMaxAgeSeconds`
- `AdminCookieSecure`
- `AdminJwtSecret`
- `AdminMigrateError`
- `AdminOpaqueToken`
- `AdminPassword`
- `AdminPasswordHash`
- `AdminPasswordHashConcurrency`
- `AdminPermissions`
- `AdminRefreshToken`
- `AdminRoleNames`
- `AdminSessionId`
- `AdminTokenHash`
- `AdminUnixTokenStream`
- `Argon2AdminPasswordHashError`
- `HttpAdminHeaderMapRef`
- `JsonwebtokenAdminError`
- `SqlxAdminError`
- `SqlxAdminMigrateError`
- `StdAdminAccessToken`
- `StdAdminCookie`
- `StdAdminSharedSemaphore`
- `TokioAdminAcquireError`
- `TokioAdminJoinError`
### Модуль `server_admin::auth`

- `AdminHtmlSwaggerEnabled`
- `AdminPeerAddr`
- `AdminSessionPath`
- `AdminSignInJson`
- `AxumAdminAuthRouter`
- `AxumAdminForm`
- `AxumAdminJson`
- `AxumAdminPath`
- `AxumAdminQuery`
- `AxumAdminResponse`
- `HttpAdminHeaderMap`
- `HttpAdminHeaderValueError`
- `JsonwebtokenAdminDecodingKey`
- `JsonwebtokenAdminDecodingKeys`
- `JsonwebtokenAdminEncodingKey`
- `SqlxAdminPgConnectionRef`
- `StdAdminAccessTtlSeconds`
- `StdAdminFailureDelayMillis`
- `StdAdminFailureThreshold`
- `StdAdminRateLimitCount`
- `StdAdminRateLimitWindowSeconds`
- `StdAdminRefreshTtlSeconds`
- `StdAdminSessionLimit`
- `StdSharedAdminAuthSvcState`
- `UtoipaAdminAuthOpenApi`
### Модуль `server_admin::auth::html`

- `AdminHtmlFormKey`
- `AdminHtmlFormText`
- `StdAdminHtmlSelected`
### Модуль `server_admin::domain`

- `AdminAuditLogId`
- `AdminPermissionId`
- `AdminPermissionName`
- `AdminRoleId`
- `AdminUserId`
- `SecrecyAdminString`
- `StdAdminBool`
- `StdAdminNonZeroUsize`
- `StdAdminSocketAddr`
- `StdAdminStrRef`
- `StdAdminString`
- `UuidAdminValue`
### Модуль `server_admin::generated_tables`

- `UtoipaAdminOpenApi`
### Модуль `server_admin::repository`

- `AdminPageTotalCount`
- `AdminRecentLoginFailureCount`
- `SqlxAdminRepositoryConnectionMutRef`
- `SqlxAdminRepositoryPoolRef`
### Модуль `server_admin::repository::roles`

- `AdminActiveAdministratorCount`
### Модуль `server_admin::tests::admin_api`

- `AdminHtmlTestBody`
- `AdminHtmlTestFormBody`
- `AxumAdminApiTestRouter`
- `HttpAdminApiTestMethod`
- `HttpAdminApiTestRequest`
- `HttpAdminApiTestResponseRef`
- `HttpAdminHtmlTestResponse`
- `SqlxAdminApiTestPool`
- `SqlxAdminHtmlTestTransaction`
- `StdAdminApiTestCookie`
- `StdAdminApiTestStrRef`
## Crate `server_admin_contract`

### Модуль `server_admin_contract`

- `AdminApiBodyMaxBytes`
- `AdminAuditDetailsBytes`
- `AdminAuditExportCsv`
- `AdminAuditLogId`
- `AdminAuditTimestamp`
- `AdminAuditViews`
- `AdminBool`
- `AdminDataRows`
- `AdminDataTableStrRef`
- `AdminDataTables`
- `AdminDefaultRoute`
- `AdminDisplayName`
- `AdminLogin`
- `AdminMainLogo`
- `AdminNewPassword`
- `AdminOptionalSettings`
- `AdminOrganizationContacts`
- `AdminOrganizationName`
- `AdminPageLimit`
- `AdminPageOffset`
- `AdminPagePathRef`
- `AdminPageTotal`
- `AdminPassword`
- `AdminPermissionId`
- `AdminPermissionIds`
- `AdminPermissionStrRef`
- `AdminPermissionSummaries`
- `AdminPermissionValue`
- `AdminPermissionValues`
- `AdminPrimaryColor`
- `AdminRoleId`
- `AdminRoleIds`
- `AdminRoleName`
- `AdminRoleNames`
- `AdminRoleSummaries`
- `AdminRoutePath`
- `AdminSessionIdentifier`
- `AdminSessionTimestamp`
- `AdminSessionViews`
- `AdminSiteName`
- `AdminSupportUrl`
- `AdminTabTitle`
- `AdminTableSearch`
- `AdminTableSortKey`
- `AdminTableSortKeyRef`
- `AdminText`
- `AdminTexts`
- `AdminUserId`
- `AdminUserSummaries`
- `SerdeJsonAdminAuditDetails`
## Crate `server_admin_frontend`

### Модуль `server_admin_frontend`

- `AxumAdminFrontendRouter`
### Модуль `server_admin_frontend::ssr`

- `AdminSsrErrorMessage`
- `AdminSsrHtml`
- `AdminSsrText`
## Crate `server_runtime`

### Модуль `server_runtime`

- `AxumRouter`
- `HttpContentSecurityPolicy`
- `RequestTimeoutLayer`
- `RequestTimeoutTowerLayer`
- `ReqwestClient`
- `ReqwestClientBuildError`
- `StdRequestTimeoutMessage`
- `StdReqwestConnectTimeout`
- `StdReqwestRequestTimeout`
- `StdServeIoError`
- `TokioTcpListener`
### Модуль `server_runtime::batched_cleanup`

- `CleanupBatchCount`
- `CleanupBatchSize`
- `CleanupRows`
### Модуль `server_runtime::bounded_read`

- `BoundedBytes`
- `BoundedJsonText`
- `BoundedReadMaximumBytes`
- `BoundedReadObservedBytes`
- `BoundedText`
- `ReqwestError`
- `ReqwestResponse`
- `SerdeJsonError`
- `StdBoundedReadConcurrency`
- `StdBoundedReadConcurrencyMaximum`
- `StdFromUtf8Error`
- `StdIoError`
- `StdPathRef`
### Модуль `server_runtime::child_process`

- `ChildDiagnostic`
- `ChildProcessId`
- `ChildProcessReports`
- `StdChildDiagnosticMaximum`
- `StdChildExitStatus`
- `StdChildProcessIoError`
- `StdChildProcessSetMaximum`
- `StdCollectionsChildProcessMap`
- `TokioChildDiagnosticTask`
- `TokioChildProcess`
- `TokioChildProcessJoinError`
- `TokioManagedChild`
### Модуль `server_runtime::client_ip`

- `HttpHeaderMapRef`
- `StdAddrParseError`
- `StdIpAddr`
- `StdParseIntError`
- `StdRangeContains`
- `StdResolvedClientIp`
- `StdSocketAddr`
- `StdTrustedProxyPrefixBits`
- `TrustedProxyRanges`
### Модуль `server_runtime::cors`

- `HttpCorsAllowOriginHeaderValues`
- `HttpCorsAllowOriginTextRef`
### Модуль `server_runtime::csp`

- `HttpCspBuilder`
- `HttpCspDirectiveName`
- `HttpCspDirectiveValue`
### Модуль `server_runtime::deduplicating_queue`

- `StdCollectionsHashSet`
- `StdCollectionsVecDeque`
- `StdQueueMaximum`
### Модуль `server_runtime::exclusive_run`

- `StdExclusiveRunAtomicBool`
### Модуль `server_runtime::fallback`

- `AcceptsApplicationJson`
- `HttpAcceptHeaderMaximumBytes`
- `HttpFallbackApiPrefixRef`
- `HttpFallbackMetricsPathRef`
- `HttpFallbackRequestPathRef`
- `HttpMediaRangeRef`
- `HttpOptionalAcceptHeaderRef`
### Модуль `server_runtime::generation_gate`

- `Generation`
- `StdGenerationAtomicU64`
### Модуль `server_runtime::geojson`

- `GeoJsonDocumentText`
- `SerdeJsonGeoJsonError`
### Модуль `server_runtime::header_text`

- `HttpHeaderName`
- `HttpHeaderTextBytes`
- `HttpHeaderTextMaximumBytes`
- `HttpHeaderTextRef`
### Модуль `server_runtime::health`

- `HealthProbeSucceeded`
- `StdHealthProbeTimeout`
- `StdHealthReadinessAtomicBool`
- `StdSharedHealthReadiness`
### Модуль `server_runtime::history`

- `StdArcSharedRunReports`
- `StdAsyncRunHistoryMaximumLen`
- `StdAsyncRunHistoryReportCount`
- `StdVecDequeRunReports`
- `TokioRwLockRunReports`
### Модуль `server_runtime::http_header_policy`

- `HttpAttachmentFileNameRef`
- `HttpContentDisposition`
- `HttpContentLength`
### Модуль `server_runtime::http_policy`

- `HttpAuthorizationHeaderTextRef`
- `HttpBearerTokenRef`
- `HttpContentTypeTextRef`
- `HttpCookieHeadersRef`
- `HttpCookieNameRef`
- `HttpCookieValueRef`
### Модуль `server_runtime::http_status_error`

- `HttpErrorStatus`
### Модуль `server_runtime::lease_registry`

- `LeaseId`
- `LeaseIds`
- `LeaseKey`
- `LeaseTextRef`
- `StdArcTokioLeaseRegistryRwLock`
- `StdLeaseRegistryMaximum`
- `StdLeaseStaleTimeout`
- `TokioLeaseInstant`
- `TokioLeaseRegistryRwLock`
### Модуль `server_runtime::lifecycle`

- `StdRequestTimeout`
- `StdRunInterval`
- `TokioAbortTask`
- `TokioBackgroundTaskJoinHandle`
- `TokioBackgroundTaskShutdownSender`
- `TokioTaskJoinError`
### Модуль `server_runtime::limits`

- `RetryAfterSecs`
- `StdArcTokioSemaphore`
- `StdPermitWaitTimeout`
- `StdSemaphorePermitCount`
- `TokioAcquireError`
- `TokioOwnedSemaphorePermit`
### Модуль `server_runtime::metrics_layer`

- `HttpMetricsPathCacheMaximum`
- `HttpMetricsPathText`
- `HttpMetricsPathTextRef`
- `MetricsResponseBody`
- `MetricsSharedString`
- `StdHttpMetricsPathEntries`
- `StdSharedHttpMetricsPathCache`
### Модуль `server_runtime::multipart`

- `FileStagingDirectoryName`
- `MultipartBytes`
- `MultipartBytesParts`
- `MultipartFieldName`
- `MultipartFileName`
- `MultipartPayloadMaximum`
- `MultipartTextParts`
- `MultipartTextValue`
- `MultipartValueLength`
- `StdStorageRelativePath`
- `StoragePathSegment`
### Модуль `server_runtime::notification`

- `AxumNotificationJson`
- `AxumNotificationRouter`
- `HttpNotificationHeaderMap`
- `NotificationApiToken`
- `NotificationApiTokenAuthorized`
- `NotificationApiTokenRef`
- `NotificationMessage`
### Модуль `server_runtime::origin`

- `AllowOriginSuffix`
- `AllowedOrigins`
- `HttpOriginAuthorityText`
- `HttpOriginHeadersRef`
- `HttpOriginSchemeText`
- `HttpOriginTextRef`
- `RequestOriginAllowed`
### Модуль `server_runtime::outbound_url`

- `OutboundAllowedHost`
- `OutboundHostAllowlist`
- `OutboundUrlTextRef`
- `ReqwestOutboundUrl`
- `StdOutboundIpAddr`
### Модуль `server_runtime::path_policy`

- `HttpAllowedPathPrefixRef`
- `HttpNormalizedPath`
- `HttpProxyPath`
- `HttpProxyPathPrefixMatch`
- `HttpProxyPathRef`
- `HttpRequestPathRef`
### Модуль `server_runtime::pg_rate_limit`

- `PgRateLimitMaximum`
- `PgRateLimitQueryRef`
- `PgRateLimitScopeRef`
- `PgRateLimitSubjectRef`
- `PgRateLimitWindowSeconds`
- `SqlxPgRateLimitError`
- `SqlxPgRateLimitPoolRef`
### Модуль `server_runtime::redacted_url`

- `RedactedUrl`
- `RedactedUrlTextRef`
### Модуль `server_runtime::request_id`

- `HttpHeaderToStrError`
- `RequestId`
### Модуль `server_runtime::resource_budget`

- `ResourceBudgetAmount`
- `ResourceBudgetMaximum`
- `StdAtomicUsize`
- `StdSharedAtomicUsize`
### Модуль `server_runtime::resource_utilization`

- `ResourceAmount`
- `ResourceUtilizationPercent`
### Модуль `server_runtime::retry`

- `StdRetryAttempts`
- `StdRetryDelay`
### Модуль `server_runtime::secret_text`

- `BoundedSecretText`
- `SecretTextRef`
### Модуль `server_runtime::secure_cookie`

- `HttpCookieName`
- `HttpCookieValue`
- `HttpSetCookieHeaderValue`
- `StdCookieMaxAgeSeconds`
### Модуль `server_runtime::service_bootstrap`

- `StdServiceRuntimeIoError`
- `TokioServiceRuntime`
- `TracingSubscriberInitError`
### Модуль `server_runtime::single_flight`

- `SingleFlightKey`
- `SingleFlightWaiter`
- `StdArcStdSingleFlightRwLock`
- `StdSingleFlightMaximum`
- `StdSingleFlightRwLock`
- `StdSingleFlightWriteGuard`
- `TokioSingleFlightReceiver`
- `TokioSingleFlightSender`
### Модуль `server_runtime::trace_context`

- `HttpTraceParent`
- `HttpTraceState`
- `ReqwestRequestBuilder`
## Crate `str_constants_macros`

### Модуль `str_constants_macros`

- `SynIdent`
- `SynLitStr`
- `SynVisibility`
## Crate `synchronization_service_runtime`

### Модуль `synchronization_service_runtime`

- `SynchronizationPayload`
## Crate `tests`

### Модуль `tests::code_style::types`

- `AnalyzerBool`
- `AnalyzerChar`
- `AnalyzerCount`
- `CargoMetadata`
- `CargoMetadataRef`
- `CargoTomlFileIdx`
- `DiagnosticMsgs`
- `DiagnosticMsgsMutRef`
- `SourceText`
- `SourceTextList`
- `SourceTextListRef`
- `SourceTextRef`
- `StaticStr`
- `StaticStrSliceRef`
- `StdCargoPackageIdRefSet`
- `StdPathBuf`
- `StdPathRef`
- `StdProcessOutputRef`
- `StdSourceTextHashSet`
- `StdSourceTextRefSet`
- `StdSourceTextSet`
- `StdStdSourceTextSetRef`
- `SynAttributeListRef`
- `SynAttributeRef`
- `SynBlockRef`
- `SynExprCallRef`
- `SynFieldsRef`
- `SynFile`
- `SynFileRef`
- `SynGenericsRef`
- `SynIdentifierRef`
- `SynItemFnRef`
- `SynItemImplRef`
- `SynItemRef`
- `SynItemStructRef`
- `SynPathArgumentsRef`
- `SynPathRef`
- `SynPathSegmentRef`
- `SynSignatureRef`
- `SynTypePathRef`
- `SynTypeRef`
- `SynUseTreeRef`
- `TomlTable`
- `TomlTableRef`
- `TomlValue`
- `TomlValueRef`
- `WalkdirWalkDir`
### Модуль `tests::domain_type_policy_fixture`

- `DomainEvents`
- `DomainId`
- `DomainName`
## Crate `text_policy`

### Модуль `text_policy`

- `FixedLengthAsciiHexText`
- `NonEmptyTrimmedText`
- `PasswordLength`
- `PasswordTextRef`
- `RequiredNulFreeBoundedText`
- `UrlSafeTokenPartMaximumBytes`
- `UrlSafeTokenPartRef`
- `UrlSafeTokenPartText`
## Crate `to_err_string`

### Модуль `to_err_string`

- `StaticStrToOwnedInput`
- `ToErrStringValue`
## Crate `token_patterns`

### Модуль `token_patterns`

- `ProcMacro2TokensMut`
## Crate `token_patterns_macros`

### Модуль `token_patterns_macros`

- `ProcMacro2GenerateTpInput`
- `ProcMacro2GenerateTpOutput`
## Crate `where_filters`

### Модуль `where_filters`

- `BoundedVec`
- `BoundedVecLen`
- `PgTypeNotEmptyUniqueVec`
- `RegexCasePostgreqlSyntax`
- `RegexRegex`
### Модуль `where_filters::tests`

- `NonClone`
## Crate `workspace_macro_helpers`

### Модуль `workspace_macro_helpers`

- `FirstCommaStripped`
- `FirstIdentifier`
- `FirstIdentifierifierTryFromStringError`
- `PartIndex`
- `ProcMacro2MacroTokens`
- `ProcMacro2TopLevelCommaParts`
- `StdUniqueOptionSet`
- `StdUniqueOptionSetContains`
- `StdUniqueOptionSetIsEmpty`
- `SynDeriveInputRef`
- `SynFieldsNamedRef`
- `SynFieldsUnnamedRef`
- `TopLevelCommaPart`
## Crate `workspace_scaffold`

### Модуль `workspace_scaffold`

- `ProjectNameRef`
- `RepositoryUrlRef`
- `ServicePort`
## Crate `workspace_test_runner`

### Модуль `workspace_test_runner`

- `AnsiTextRef`
- `CargoArgs`
- `CleanAnsiText`
- `MeasurementName`
- `MemusageColumnIdx`
- `MemusageKey`
- `MemusageProgNameRef`
- `MemusageRowName`
- `MemusageValueRef`
- `ProgramArgsRef`
- `ProgramPathRef`
- `QuoteTokenStreamGeneratePgTableMeasureInputTokenStream`
- `StderrTextRef`
- `ToolName`
- `ToolPath`
### Модуль `workspace_test_runner::execution`

- `CommandIdx`
- `CommandStartedAt`
- `RunDir`
- `SummaryText`
