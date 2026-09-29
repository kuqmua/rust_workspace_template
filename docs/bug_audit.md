# Workspace bug audit

Scope: every workspace package and Rust source module. Inventory date: 2026-09-29.

Inventory: 191 packages; 3227 Rust source files.

## Status definitions

- Pending: semantic review has not been completed.
- Focused review: a particular behavior was reviewed; the rest of the module remains pending.
- Reviewed: the complete module was inspected, with evidence recorded.

Passing workspace tests and policy checks does not establish complete semantic coverage.

## Issue queue
| ID | Status | Location | Finding and evidence |
| --- | --- | --- | --- |
| A01 | Fixed and verified | pg_crud_common/src/sql_select_builder.rs | Reproduced with 8,065 identifiers of 128 bytes and two 128-byte table identifiers: valid columns produce an oversized query, formerly returned as diagnostic text. build now returns Result with the existing TooLong error. Intentional public API correction under the requested bug-fix scope; repository callers audited and success tests updated. The contract API snapshot does not cover this builder. |
| A02 | Fixed and verified | server_runtime_http/src/read_bounded_http_response.rs | Code inspection confirmed a full permitted Content-Length reservation before reading data. Initial reservation is now capped at 4 KiB; growth still follows the configured read limit. Deterministic in-memory response tests cover 8 KiB success and 4 KiB rejection; no network client is constructed. |
| A03 | Fixed and verified | server_runtime_http/src/child_process_supervisor.rs | A paused-clock pending-task regression reproduced the missing diagnostic deadline. Joining now uses the shutdown timeout. New DiagnosticTimeout variant retains the Tokio Elapsed source through a dedicated leaf wrapper; the borrowed task owner survives cancellation and the supervisor aborts it on drop. This is an intentional public error API addition; no dependency, crate or reviewed struct-error exception was added. |
| A04 | Fixed and verified | server_runtime_http/src/spawn_interval_task.rs | A paused-clock maximum-period regression reproduced a Tokio Skip arithmetic panic after advancing past its artificial far-future tick. The scheduler now checks deadline arithmetic, preserves the five-millisecond missed-tick tolerance and original phase, and computes duration remainders with bounded binary subtraction rather than narrowing nanoseconds to u64. An unschedulable next deadline stops before invoking the callback; the retained task join exposes typed IntervalOverflow instead of a panic. This intentionally adds public outcome/error variants while preserving the spawn signature and nonzero-duration conversion. Paused-clock regressions cover normal Skip cadence and a Linux-supported 600-year period whose remainder exceeds u64 nanoseconds. The existing one-site cancellation inventory was reviewed: dropping Sleep and the oneshot receive remains cancellation-safe. Evidence: target/audit_tmp/interval_overflow_before.log and target/audit_tmp/interval_overflow_tests_verified.log. |
| A05 | Fixed and verified | bounded_string_core/src/try_from_error_text.rs | A compiled one-byte-target probe never terminated and was killed and reaped externally. The generic helper and forwarding helper now return Result after one conversion, preserving its typed failure; regressions cover short maxima, large minima and successful diagnostic conversion. Twelve handwritten From<Error> adapters now use the existing bounded diagnostic constructor, matching the already-established proc-macro implementation; that generator never used the retry helper. The fixed default regex is constructed directly and checked against validated construction. Idempotency-key generation now propagates validation failures through Result and both generated client paths; native client errors gain an IdempotencyKey variant only for idempotent operations. These are intentional public API corrections under the requested bug-fix scope. The removed loop's exact allocation inventory entry and obsolete constants were removed, without adding an exception. The constructor policy is intentionally corrected to recognize From of typed sources, consistent with existing generated diagnostic conversions; From<String> and constructors outside conversion implementations remain rejected, covered by the existing policy regression with a new typed-error fixture. Verified cargo fmt and fmt check, final Clippy for all targets and features with warnings denied, 306 code-style tests through the workspace runner, and the complete workspace test command excluding the style crate. After the style pass, generated-client compatibility repairs (the error binding and a ToErrString adapter) were checked by the final full Clippy and workspace runs. Temporary files were redirected to target/tmp to avoid the exhausted /tmp user quota. Logs: target/audit_tmp/error_text_static_complete.log, error_text_clippy_final.log and error_text_workspace_final.log. The reviewed public contract snapshot covers common_routes, frontend_contract and server_admin_contract, so these API corrections require no snapshot edits. Evidence: target/audit_tmp/error_text_retry_loop_before.log. |
| A06 | Dismissed for pinned dependency | generate_quotes/src/quote_token_stream.rs | proc-macro2 1.0.107 fallback LexError Display is a fixed ASCII message; QuotePanicId values are fixed UUID fragments. User input is absent from the diagnostic interpolation. No failing reproduction established. |
| A07 | Fixed and verified | frontend_contract_validation/src/validate_openapi_json_payload.rs | Recursive traversal, initial serialization and enum/const equality could overflow the stack. The validator now borrows ready JSON through explicit Borrow<serde_json::Value> bounds and uses iterative traversal and structural comparison. This intentional API correction removes the former Serialize bounds and unreachable serialization error variants; all repository callers are tests and have been updated. Reference/payload identity guards and composition short-circuit semantics are preserved. Deterministic regressions cover 8192-level inline schemas, document trees and enum/const values, plus structural equality and 4096-node reference/composition chains. The same 512-level inline schema now succeeds on a 128 KiB worker stack; caller-owned JSON is returned before iterative cleanup. Evidence: target/audit_tmp/inline_schema_serialization.log, inline_schema_borrowed.log and borrowed_json_tests_deep.log. Other serialization-based validation APIs remain outside this fix. |
| A08 | Fixed and verified | workspace_macro_helpers/src/generate_private_field_getters.rs | Regression reproduced a panic for get_r#type. Accessor name composition now unraws identifiers; raw identifiers remain intact when referring to the field or bare getter. Default, bare, copy and mutable forms are covered. |
| A09 | Fixed and verified | workspace_macro_helpers/src/generate_private_field_getters.rs | Regression reproduced duplicate helper methods for value and get_value. Only the automatically added prefix is removed, once; bare mode preserves the full field name. This intentionally corrects helper names for get_-prefixed fields; no existing repository consumers were found outside the new fixtures. |
| A10 | Fixed and verified | workspace_macro_helpers/src/proc_macro2_top_level_comma_parts.rs | A compiled probe grew the validated collection to 10,001 parts through resize, bypassing the 10,000-part limit. DerefMut is intentionally removed from the public helper API; immutable Vec access and ownership transfer remain. The sole mutable consumer now takes the first owned iterator element and preserves remaining order without shifting or cloning. Exact-limit construction tests pass, and the original mutating probe now fails to compile with E0596. Evidence: target/audit_tmp/comma_parts_before.log, target/audit_tmp/comma_parts_after.log and target/audit_tmp/comma_parts_tests.log. |
| A11 | Fixed and verified | workspace_macro_helpers/src/parse_first_identifier.rs | An isolated compiled probe supplied a valid 1,048,577-byte identifier. Parsing returned Some containing diagnostic text instead of rejecting it, and reconstructing an Ident panicked because the diagnostic contains spaces. Repository consumers reconstruct identifiers from this helper. The case_trait_pair proc-macro also reproduces the panic with that identifier. Evidence: target/audit_tmp/identifier_overflow_before.log and target/audit_tmp/identifier_overflow_macro_before.log. Both identifier and closure-parameter parsing now return None on a failed bounded conversion instead of substituting diagnostic text. A deterministic regression checks acceptance at exactly 1 MiB and rejection one byte above in both paths. The real case_trait_pair probe now emits its ordinary missing-identifier diagnostic instead of panicking: target/audit_tmp/identifier_overflow_macro_after.log. |
| A12 | Fixed and verified | proc_macro_trait_alias/src/lib.rs | An isolated rustc probe with Name Extra reproduced a proc-macro panic from format_ident. Shared generation now parses an identifier token and requires the following equals sign, returning the existing syntax diagnostic for malformed names. Raw identifiers, associated-type bounds and delimiter-free forwarded identifiers are covered. Bounds retain their previous textual tokenization to preserve expansion span behavior and avoid newly exposing caller qualifications to lint checks. The proc-macro entrypoint delegates to the existing shared helper crate; unused direct constants_str, proc-macro2 and quote dependencies were removed. Evidence: target/audit_tmp/trait_alias_before.log. |
| A13 | Fixed and verified | proc_macro_bool_enum_to_tokens/src/lib.rs | An isolated rustc probe reproduced rejection of a valid false-branch quote containing the nested tokens value, true => value. Text splitting mistook the nested delimiter for the outer branch separator. The existing shared helper crate now parses complete Rust expressions and branch keywords. A compiled consumer regression verifies both emitted variants; helper tests cover a delimiter inside a string literal and reversed keywords. Public enum and ToTokens APIs remain unchanged. Evidence: target/audit_tmp/bool_enum_before.log. |
| A14 | Fixed and verified | pg_crud_pg_table/src/generate_*_query_string.rs | Four query builders, the optimistic-revision builder and two update fragment builders returned a short diagnostic as successful SQL when their unbounded input exceeded the 1 MiB bound. All seven now return typed length errors; generated CRUD handlers return the operation-specific QueryString error variant with HTTP 400. The error-to-success conversions on query and fragment wrappers were removed. A regression covers overflow in all seven paths. The generated macro Clippy probe, crate tests, full Clippy, all 306 code-style tests and workspace tests pass. Evidence: target/audit_tmp/a14_overflow_tests.log, a14_macro_clippy2.log, a14_static4.log and a14_workspace_final.log. |
| A15 | Fixed and verified | proc_macro_newtype_shared/src/lib.rs | Const-generic EnumFromStr fixture reproduced E0107. Both EnumFromStr and WireEnum omitted input generics. Generated inherent and trait impls now preserve implementation generics, type arguments and where clauses. Compiled regressions cover both generators and two concrete const arguments; no dependency or public signature change. |
| A16 | Fixed and verified | proc_macro_newtype_shared/src/lib.rs | Non-Copy WireEnum fixture reproduced E0507 inside the generated Serialize implementation. Serialization now matches the borrowed enum and preserves the same ref_type conversion and AsRef behavior as as_str. The by-value as_str API is preserved; the compiled regression passes without Copy or Clone. |
| A17 | Fixed and verified | proc_macro_newtype_shared/src/lib.rs | AsRefTarget and BorrowPath append a second where clause after the input where clause. Standalone rustc reproduction fails for both derives with duplicate where clauses and unparsable tokens. Evidence: /tmp/rust_audit_adapter_before.log. Both adapters now merge the new predicate into cloned input generics. A compiled fixture verifies both adapters on a generic Path wrapper with existing bounds. |
| A18 | Fixed and verified | proc_macro_newtype_shared/src/lib.rs | Accessor emits a provider trait using inner generic types without declaring those generics; ToErrString modes omit generics in the generated impl. Standalone rustc reproduction reports E0261 for a borrowed provider and E0107 for a const-generic error wrapper. Evidence: /tmp/rust_audit_provider_before.log. Generated provider traits now declare input generics and where clauses, and both provider implementations supply matching arguments. All three error-text modes preserve input generics. Borrowed provider and display/debug/reference error-text fixtures pass. Existing non-generic APIs remain unchanged; this deliberately repairs previously uncompilable generated generic APIs. |
| A19 | Fixed and verified | server_runtime_http/src/read_child_diagnostic.rs | An in-memory duplex regression reproduced a failed writer after the reader reached its capture limit and closed the stream. The reader now drains to EOF while retaining only the configured prefix. Initial allocation and retained bytes remain bounded; supervisor diagnostic timeout A03 bounds completion. This intentionally corrects premature stderr closure; no process or network client is constructed by the regression. |
| A20 | Fixed and verified | frontend_contract/src/api_url.rs | Three deterministic regressions failed before the fix: path insertion after an existing query/fragment, query insertion after a fragment containing a question mark, and appending to an existing query before its fragment. Path insertion now separates at the first question mark or hash; query insertion separates at the first hash and determines its separator only from the path/query prefix. Encoding, suffix bytes, fallible signatures and transactional replacement are preserved. All 46 frontend_contract unit tests and seven integration tests pass. Evidence: target/audit_tmp/api_url_suffix_before.log and target/audit_tmp/api_url_suffix_tests_final.log. |
| A21 | Fixed and verified | frontend_contract_validation/src/validate_openapi_operations.rs | Operation security validation ignored root-level security requirements. Effective security now inherits the document declaration when the operation omits it, while an explicit empty operation array overrides it. Deterministic regressions cover inherited Required/Public expectations and both expectations after the empty override. |
| A22 | Fixed and verified | proc_macro_naming_common/src/lib.rs | A compiled case_trait_pair probe with a two-parameter closure panics in format_ident on left, right instead of reporting unsupported closure syntax. The macro now uses the existing shared closure parser before formatting the bounded identifier. It retains prior prefix handling and textual body tokenization. Helper regressions reject multiple, missing, typed and mutable parameters; a compiled consumer covers move and raw parameters. Evidence: target/audit_tmp/case_trait_closure_before.log. |
| A23 | Partially fixed; string consumers pending | naming_naming_common/src/str_case.rs | A public snake-case conversion of a 1 MiB alternating-case input produces 1,572,864 bytes internally, then returns a 50-byte diagnostic string as successful data. The shared conversion now returns its typed bounded error and nine generated trait pairs expose try_case; token methods use try_case and emit compile_error on overflow. The accessor-trait generator, TryFromEnv derive, all six enum naming generators, both naming-template generators and derive-token-stream builder propagate conversion errors. Fixed Order variants now use exact spellings rather than an unnecessary case conversion; its dependency on naming_common was removed. The PostgreSQL table generator now emits a compile error for table-identifier conversion failure and reuses the checked conversion. Swagger URL path quoting now rejects case expansion that exceeds its shared 1 MiB bound, and token output emits compile_error. The existing String-returning case methods retain the old diagnostic-text fallback until remaining consumers, including generated self-formatters, are migrated, so this issue remains open. Raising the bound or truncating output would not solve the fault. Full Clippy, all 306 code-style tests and workspace tests pass after the Swagger helper migration. Evidence: target/audit_tmp/case_expansion_before.log, case_token_overflow.log, accessor_case_overflow.log, try_from_env_case_probe.log, naming_case_final_static.log, derive_builder_case_probe.log, derive_builder_case_workspace.log, order_case_fixed_tests.log, order_case_static.log, order_case_workspace.log, pg_table_case_tests.log, pg_table_case_static.log, pg_table_case_workspace.log, swagger_case_overflow.log, swagger_case_naming_tests.log, swagger_case_static.log and swagger_case_workspace.log. |
| A24 | Fixed and verified | proc_macro_generate_accessor_traits_for_struct_fields_shared/src/lib.rs | A compiled generic-newtype derive reproduced E0425 because the provider declaration omitted input parameters. Provider traits and reference forwarding now preserve lifetimes, type/const parameters and where clauses, with a fresh forwarding type name chosen outside every input identifier. Struct-field implementations also preserve owner generics while retaining registered provider paths. Compiled roundtrips cover forwarding-name collision and a const-generic owner; syntax regressions cover both generators. This intentionally extends previously invalid generated generic APIs. Evidence: target/audit_tmp/accessor_generic_before.log, target/audit_tmp/accessor_generics_after.log and target/audit_tmp/accessor_fields_after.log. |
| A25 | Fixed and verified | proc_macro_generate_accessor_traits_for_struct_fields_shared/src/lib.rs | A raw r#type field caused a proc-macro panic when token case conversion formed R#typeProvider. Field and owner identifiers are now unrawed only for case conversion; raw identifiers remain in actual member syntax. A helper regression and compiled input verify the raw-field path. Evidence: target/audit_tmp/accessor_raw_before.log and target/audit_tmp/accessor_raw_field_verified.log. |
| A26 | Fixed and verified | frontend_contract/src/auth_session_keep_alive.rs | The maximum accepted interval silently lost its deadline on checked_add overflow and allowed an immediate refresh retry. finish now intentionally returns a typed Result with IntervalOverflow. An explicit internal overflow state makes begin return SkipIntervalOverflow until mark_missing resets the schedule. Existing rejected/successful behavior is retained. The anchored regression covers normal delay, Refreshed/Failed overflow and explicit reset; no new clock read or dependency was added. No production consumers were found. This is an intentional public result/error/decision API correction. The generated contract snapshot was reviewed and updated for exactly three changed API entries, without exception inventory changes. Evidence: target/audit_tmp/auth_refresh_overflow_before.log, target/audit_tmp/auth_refresh_overflow_after.log and target/audit_tmp/auth_refresh_tests_final.log. |
| A27 | Fixed and verified | git_info/src/git_commit_id.rs | Oversized commit IDs and URLs were returned as diagnostic text rather than failures. Borrowed-to-owned conversion, owned/Cow providers, fallback/callback adapters, and free/provider link builders now propagate the existing typed length error through Result. Length validation precedes owned allocation; valid borrowing, static project links and successful fallback caching are preserved. Failed fallback construction leaves the cache empty and does not invoke the callback. HTTP git_info and unknown-route adapters now return an internal API problem with separate operation errors instead of invalid commit data. GitInfoRoute intentionally declares HTTP 500, with exactly one public-contract snapshot entry reviewed and updated. Boundary, source-preservation, ownership and HTTP regression tests cover the corrected paths. Evidence: target/audit_tmp/git_commit_ref_before.log, git_commit_fallible_regressions_final.log and git_commit_http_regressions.log. |
| A28 | Fixed and verified | proc_macro_naming_shared/src/lib.rs | Both enum naming generators omitted source generics. With a default const parameter, generated inherent/trait impls applied only to the default specialization; a compiled LIMIT=2 consumer failed with missing-method and unsatisfied-trait errors. The shared helpers now preserve split impl/type generics and where clauses for all six derives. A borrowed SynNamingGenericsRef keeps the added helper parameter in a repository wrapper. A deterministic AST regression covers all six generators, and the same compiled consumer now builds and runs successfully. No public item was renamed and nongeneric implementations retain their behavior. Evidence: target/audit_tmp/naming_enum_generics_before.log, naming_enum_generics_after.log and naming_enum_generics_tests.log. |
| A29 | Fixed and verified | proc_macro_naming_shared/src/lib.rs | The self-template generator's exactly-one-placeholder invariant used any(), accepting two or more self words. A compiled probe confirmed that duplicate self input produced SelfSelfUpperCamelCase instead of rejection. The guard now counts matching placeholders, stopping at the second match, and requires exactly one. The original 5680dd63 diagnostic is preserved through its shared catalog constant. Deterministic tests reject zero/two/three placeholders, inspect the original panic payload, and accept single placeholders in either position. All 53 registered templates contain exactly one placeholder. Evidence: target/audit_tmp/naming_self_duplicate_before.log, naming_self_duplicate_caught.log and naming_self_cardinality_tests_retry.log. |
| A30 | Fixed and verified | generate_quotes/src/quote_literal.rs | A compiled 1 MiB input probe showed quoted output exceeding its bound returned a 53-byte length diagnostic as successful literal data; the token API then emitted ordinary tokens instead of a compiler error. Quoted string helpers now propagate their typed bounded conversion error, while token helpers emit compile_error for conversion or parsing failure. The Swagger naming consumer propagates the string error and emits compile_error for token errors. Deterministic boundary and oversized-input regressions pass in both packages; full Clippy, all 306 code-style tests and workspace tests pass. Evidence: target/audit_tmp/quote_overflow_before.log, quote_overflow_regressions.log, quote_overflow_static.log and quote_overflow_workspace.log. |
| A31 | Fixed and verified | server_config/src/server_config.rs | Production CORS validation rejected uppercase HTTPS schemes even though the shared origin parser accepts schemes without regard to ASCII case. A package regression failed before the fix with CorsOriginInsecure, then passed after comparing the complete HTTPS prefix without regard to ASCII case. An uppercase HTTP scheme remains rejected. Full Clippy, all 306 code-style tests and workspace tests pass. Evidence: target/audit_tmp/cors_scheme_case_before.log, cors_scheme_case_after.log, cors_scheme_static.log and cors_scheme_workspace.log. |
| A32 | Fixed; provisioned database verification pending | pg_crud_pg_table/src/complete_pg_table_idempotency.rs | Pool-based completion ignored SQL rows_affected, reporting success when its pending reservation had already been completed or removed. The connection-based implementation already rejected zero affected rows. Pool completion now requires exactly one row and returns the same reservation-unavailable protocol error otherwise. An existing PostgreSQL integration test now checks that a second completion is rejected; it compiles but requires a provisioned database to run. Full Clippy, all 306 code-style tests and ordinary workspace tests pass. Evidence: target/audit_tmp/idempotency_complete_compile2.log, idempotency_complete_static.log and idempotency_complete_workspace.log. |
| A33 | Fixed and verified | pg_crud_pg_table/src/pg_table_idempotency_body.rs | Persisted-response validation discarded BoundedValueError through an ignored map_err binding, hiding the actual and maximum length from error sources. PgTableIdempotencyBodyError now carries that source while retaining its domain-level message. The inclusive-boundary unit regression verifies both lengths in the source error. The obsolete ignored-map_err inventory entry and its unused path constant were removed. Full Clippy, all 306 code-style tests and workspace tests pass. Evidence: target/audit_tmp/idempotency_body_source_test.log, idempotency_body_static2.log and idempotency_body_workspace.log. |
| A34 | Fixed and verified | pg_crud_pg_types_common/src/pagination_starts_with_one.rs | Derived Default delegated to PaginationBase::default(), which starts at offset zero despite this wrapper's one-based validation and name. An explicit Default now delegates to the valid first-page constructor. A unit regression checks that both the default start and end match the one-based standard page. Full Clippy, all 306 code-style tests and workspace tests pass. Evidence: target/audit_tmp/pagination_default_tests.log, pagination_default_static.log and pagination_default_workspace.log. |
| A35 | Fixed and verified | pg_crud_pg_types_generate_src/src/pg_type_initialization_try_new.rs | Generated float4 constructors accepted NaN and infinities; serializing NaN to JSON silently produced null. A failing regression reproduced this. Float4 now follows the existing float8 finite-value validation for construction, deserialization, and database decoding. The generated constructor API intentionally changes from new to try_new. The full Clippy command, all 306 code-style tests, workspace tests, and the final 22 generator tests pass. Evidence: target/audit_tmp/float4_nonfinite_before.log, float4_after.log, float4_clippy2.log, float4_static.log, float4_workspace.log and float4_generate_test_final.log. |
| A36 | Fixed and verified | pg_crud_where_filters/src/pg_filter_vec.rs | Derived Default created an empty vector for nonzero LENGTH, violating the exact-length invariant enforced by TryFrom and Deserialize. A two-element regression failed with length zero and passes after Default constructs exactly LENGTH default values. The existing T: Default API bound is retained. Full Clippy, 306 code-style tests and workspace tests pass with A37 included. Evidence: target/audit_tmp/filter_default_before.log, filter_default_after.log, filter_defaults_clippy.log, filter_defaults_static.log and filter_defaults_workspace.log. |
| A37 | Fixed and verified | pg_crud_where_filters/src/pg_type_not_empty_unique_vec.rs | Derived Default produced an empty list even though TryFrom rejects empty input. A regression reproduced length zero; Default now uses one default element and retains the existing T: Default bound. All 10 where_filters tests, full Clippy, 306 code-style tests and workspace tests pass. Evidence: target/audit_tmp/unique_default_before.log, filter_defaults_after.log, filter_defaults_clippy.log, filter_defaults_static.log and filter_defaults_workspace.log. |
| A38 | Fixed and verified | pg_crud_where_filters/src/between.rs | SQL BETWEEN is inclusive, but Between::try_new rejected equal endpoints and DefaultSomeOneElement constructed equal bounds. A failing regression confirmed the mismatch. The constructor now accepts equality and continues rejecting descending or unordered bounds; the error variant is intentionally renamed to StartNotLessThanOrEqualToEnd to describe both failures. This is a reviewed public error API correction under the requested bug-fix scope. Targeted regression, full Clippy, all 306 code-style tests and workspace tests pass. Evidence: target/audit_tmp/between_equal_before.log, between_equal_final.log, between_clippy3.log, between_static2.log and between_workspace.log. |
| A39 | Fixed; provisioned database verification pending | pg_crud_where_filters/src/regex_regex.rs | PostgreSQL accepts the ARE word-start escape `\m`, but Rust regex::Regex rejected it before SQL binding. A failing regression reproduced this valid-input rejection. RegexRegex now delegates byte-length validation to the shared bounded string and retains its typed source; PostgreSQL validates syntax when the query executes. The unused Regex error variant, public RegexError wrapper, crate dependency and reviewed snapshot entry were removed as an intentional public API correction. PostgreSQL syntax reference: https://www.postgresql.org/docs/19/functions-matching.html. The valid PostgreSQL escape and oversized-pattern source regressions, full Clippy, all 306 code-style tests and workspace tests pass. SQL execution requires a provisioned PostgreSQL environment. Evidence: target/audit_tmp/regex_postgres_before.log, regex_postgres_after.log, regex_postgres_package.log, regex_postgres_clippy2.log, regex_postgres_static2.log and regex_postgres_workspace.log. |
| A40 | Fixed and verified | pg_crud_where_filters_generate_src/src/emit_generate_where_filters.rs | Generated PgTypeWhereTextSearch::try_new rejected empty and oversized text, but derived Deserialize populated raw String fields directly. A JSON contract regression reproduced acceptance of an empty value. The value field now uses the existing validated BoundedString<1, 1024, false> wrapper, preserving the serialized string shape and the existing constructor signature. An additional typed bounded-error source preserves unexpected conversion failures. Empty and oversized JSON rejection, valid JSON round-trip, generator macro compile checks, full Clippy, all 306 code-style tests and workspace tests pass. Evidence: target/audit_tmp/text_search_deser_before.log, text_search_deser_after.log, text_search_deser_tests2.log, text_search_deser_bounds.log, text_search_deser_clippy2.log, text_search_deser_static2.log and text_search_deser_workspace2.log. |
| A41 | Fixed and verified | pg_crud_where_filters_generate_src/src/filter_spec.rs | StrictlyToLeftOfRange and StrictlyToRightOfRange generated `&<` and `&>` SQL operators, which mean does-not-extend in PostgreSQL and can match overlapping ranges. The strict operators are `<<` and `>>`. A generated-fragment regression failed before the catalog correction and passes afterward. The obsolete operator constants were removed. PostgreSQL operator reference: https://www.postgresql.org/docs/current/functions-range.html. Full Clippy, all 306 code-style tests and workspace tests pass. Evidence: target/audit_tmp/range_strict_before.log, range_strict_after.log, range_strict_clippy2.log, range_strict_static.log and range_strict_workspace.log. |
| A42 | Fixed and verified | pg_crud_where_filters_generate_src/src/emit_generate_where_filters.rs | IncludedLowerBound and ExcludedUpperBound compared only `lower(range)` or `upper(range)` with the input, allowing a range whose matching endpoint has the opposite inclusion status. Generated predicates now also require `lower_inc(range)` or `NOT upper_inc(range)`. An emitted-query regression covers both branches and bind counts. PostgreSQL function reference: https://www.postgresql.org/docs/current/functions-range.html. Full Clippy, all 306 code-style tests, workspace tests and formatting checks pass. Evidence: target/audit_tmp/range_inclusivity_test.log, range_inclusivity_clippy.log, range_inclusivity_static.log and range_inclusivity_workspace.log. |
| A43 | Fixed and verified | pg_crud_where_filters_generate_src/src/emit_generate_where_filters.rs | GreaterThanIncludedLowerBound and GreaterThanExcludedUpperBound compared only endpoint values and ignored inclusion status. A generated-fragment regression failed before the change and passes after both variants use the shared inclusion predicate with `>`. Full Clippy, all 306 code-style tests, workspace tests and formatting checks pass. Evidence: target/audit_tmp/range_greater_inclusivity_before.log, range_greater_inclusivity_after.log, range_greater_inclusivity_clippy.log, range_greater_inclusivity_static.log and range_greater_inclusivity_workspace.log. |
| A44 | Fixed; provisioned database verification pending | pg_crud_where_filters_generate_src/src/emit_generate_where_filters.rs | RangeLen emitted `upper(range) - lower(range) = $N` but bound every parameter as `NotZeroUnsignedPartOfI32`. The pg-type catalog exposes it for int4range, int8range, daterange, tsrange and tstzrange; timestamp subtraction yields PostgreSQL `interval`. RangeLen is now generic, with catalog-selected positive numeric, i32 or interval wrappers. The generated table OpenAPI component includes all three value types. Deterministic tests verify JSON validation, binary encoding across a day boundary and generated SQL fragments. Full Clippy, all 306 code-style tests, workspace tests and formatting checks passed before the separate A45 arithmetic correction. PostgreSQL reference: https://www.postgresql.org/docs/current/functions-datetime.html. Evidence: target/audit_tmp/range_len_clippy_final2.log, range_len_static_final.log and range_len_workspace_final.log. Database execution has not yet been provisioned. |
| A45 | Fixed; provisioned database verification pending | pg_crud_where_filters_generate_src/src/emit_generate_where_filters.rs | RangeLen subtracted int4range and int8range endpoints in their signed SQL types. A valid range can span more than `i32::MAX` or `i64::MAX`, so subtraction can overflow and a positive signed length wrapper cannot express the result. The integer-range variants now cast both bounds to exact PostgreSQL `numeric`; their validated `u64` wrapper binds as SQLx BigDecimal and admits `u64::MAX`. Date and timestamp variants retain their prior arithmetic. Fragment and binary-encoding tests, full Clippy, all 306 code-style tests, workspace tests and formatting checks pass. PostgreSQL reference: https://www.postgresql.org/docs/current/datatype-numeric.html. Evidence: target/audit_tmp/range_numeric_fragment_test1.log, range_numeric_wrapper_tests.log, range_numeric_clippy1.log, range_numeric_static1.log and range_numeric_workspace1.log. Database execution has not yet been provisioned. |
| A46 | Fixed; provisioned database verification pending | pg_crud_where_filters_generate_src/src/emit_generate_where_filters.rs | CurrentTime and GreaterThanCurrentTime were generated for `time without time zone` columns but used PostgreSQL `CURRENT_TIME` (`time with time zone`); PostgreSQL permits comparison of time values only within the same time type. Both predicates now use `LOCALTIME` and preserve their equality/greater-than operators and zero-bind shape. A generated-fragment regression failed before and passes after. Full Clippy, all 306 code-style tests, workspace tests and formatting checks pass. PostgreSQL reference: https://www.postgresql.org/docs/current/functions-datetime.html. Evidence: target/audit_tmp/current_time_before.log, current_time_after.log, current_time_clippy.log, current_time_static.log and current_time_workspace.log. Database execution has not yet been provisioned. |
| A47 | Fixed and verified | pg_crud_where_filters_generate_src/src/emit_generate_where_filters.rs | Generated PgTypeWhereIn::query_bind used a `for` loop over values, contrary to repository generated-code policy. It now uses iterator `try_for_each`, retaining value order and short-circuiting with the same typed bind error. Full Clippy, all 306 code-style tests, workspace tests and formatting checks pass. Evidence: target/audit_tmp/in_bind_iterator_clippy.log, in_bind_iterator_static.log and in_bind_iterator_workspace.log. This was a policy defect, not a demonstrated runtime bug. |
| A48 | Fixed and verified | frontend_contract_validation/src/validate_openapi_contract.rs | The validator normalized HTTP methods before inserting them into a map, so a document with `GET` and `get` for the same path silently discarded one operation and could pass route validation. A regression reproduced acceptance before the fix. A duplicate now returns the typed DuplicateOperation error with its method and path. The regression, full Clippy, all 306 code-style tests and workspace tests pass. Evidence: target/audit_tmp/openapi_duplicate_before.log, openapi_duplicate_after.log, openapi_duplicate_clippy.log, openapi_duplicate_static.log and openapi_duplicate_workspace.log. |
| A49 | Fixed and verified | frontend_contract_validation/src/openapi_schema_references.rs | A valid nested JSON Pointer such as `#/components/schemas/Item/items` was treated as a component named `Item/items` and rejected. The collector now resolves the full local pointer and decodes `~0` and `~1` in the component name, while retaining its bounded reference set. Regressions cover nested targets and an escaped `/` in a schema name; the nested case failed before and passes after. Full Clippy, all 306 code-style tests and workspace tests pass. OpenAPI and JSON Pointer references: https://spec.openapis.org/oas/v3.1.0 and https://www.rfc-editor.org/rfc/rfc6901. Evidence: target/audit_tmp/openapi_nested_ref_before.log, openapi_nested_ref_after.log, openapi_nested_ref_clippy.log, openapi_nested_ref_static.log and openapi_nested_ref_workspace.log. External URI references remain outside this local-reference validator's scope. |
| A50 | Fixed and verified | file_storage/src/plan_disk_cache_eviction.rs | Adding the current cache size and incoming size could overflow `u64` before eviction was planned, even when evicting an entry would meet the budget. The planner now compares the current size with the budget remaining after the incoming size. A regression uses a `u64::MAX` existing entry and a one-byte incoming entry. Full Clippy, all 306 code-style tests and workspace tests pass. |
| A51 | Fixed and verified | frontend_contract_validation/src/validate_openapi_contract.rs | A nonobject OpenAPI path item was silently skipped because `as_object()` returned `None` and the iterator was empty. The existing `test_non_object_path_item_is_rejected` failed before the fix and passes now; the validator returns its existing typed `InvalidPathItem` error. Full Clippy, all 306 code-style tests and workspace tests pass. |
| A52 | Fixed and verified | text_policy/src/validate_password_policy.rs | The policy said passwords must not contain whitespace, but checked only ASCII whitespace bytes and accepted a nonbreaking space. A deterministic regression failed with `Ok(())` before the fix and passes after the validator checks Unicode whitespace characters. All five `text_policy` unit tests, full Clippy, all 306 code-style tests and workspace tests pass. |
| A53 | Fixed and verified | text_policy/src/validate_password_policy.rs | Administrator password limits are character counts, but the shared validator used UTF-8 byte lengths. A regression reproduced acceptance of an 11-character password with 12 bytes and covers acceptance of a valid 1,024-character password with more than 1,024 bytes. Length checks now count characters, matching the administrator contract wrapper. All six `text_policy` unit tests, full Clippy, all 306 code-style tests and workspace tests pass. |
| A54 | Fixed and verified | bounded_types/src/bounded_string.rs | Schemars emitted an unconstrained string schema for bounded strings. Regressions failed before the fix for character minimum/maximum and byte minimum/maximum metadata. Character-counted strings now publish standard length bounds; byte-counted strings publish the same byte-bound extensions as the existing OpenAPI schema and omit the maximum extension for unbounded values. All 38 bounded-types tests, full Clippy, all 306 code-style tests and workspace tests pass. |
| A55 | Fixed and verified | common_routes/src/health_report_response.rs | The health and readiness route contracts declare a HealthReport for HTTP 503, but unavailable database responses emitted an ApiProblem document and discarded the degraded component report. The error now carries the HealthReport and serializes it with status 503. A deterministic response regression checks status and decoded report. Full Clippy, all 306 code-style tests and workspace tests pass. |
| A56 | Fixed and verified | common_routes/src/health_check_error.rs | The health-check route declares an empty HTTP 503 response, but the error emitted a nonempty ApiProblem document. A deterministic regression failed on the body before the fix and passes after the error returns status only. Full Clippy, all 306 code-style tests and workspace tests pass. Evidence: target/audit_tmp/a56_style.log and a56_workspace.log. |
| A57 | Fixed and verified | file_storage/src/safe_file_storage.rs, file_storage/src/file_storage_error.rs | When an operation and staging cleanup both failed, three branches returned only the cleanup I/O error. They now use the existing combined error variant, preserving both failures and exposing the operation as the error source. Deterministic unit tests cover I/O and non-I/O operation errors. Focused file-storage tests, formatting, full Clippy, code-style tests, workspace tests, and `git diff --check` passed (`target/audit_tmp/a57_focused_final.log`, `target/audit_tmp/a57_file_storage.log`, `target/audit_tmp/a57_clippy.log`, `target/audit_tmp/a57_style.log`, `target/audit_tmp/a57_workspace.log`). |
| A58 | Fixed and verified | config_lib/src/parse_required_env_var.rs | Oversized environment values previously became validation error text that was passed to the parser. The helper now returns a typed length error with the field name, and the generated configuration error carries it. A deterministic regression proves that an oversized value returns the original length error without calling the parser. Focused config tests, full Clippy, all 306 code-style tests, and workspace tests pass. The public helper signature and generated config error enum intentionally add a length-error mapping path. |
| A59 | Fixed and verified | config_lib/src/domain_types.rs | Any tracing format other than `json` silently became `text`, allowing typos such as `jsno` to pass configuration parsing. The parser now accepts only `json` and `text`, retains case-insensitive matching, and returns a typed error for unknown values. A deterministic test covers both valid values, uppercase JSON, and an invalid value. The generated notification-service descriptor test, full Clippy, all 306 code-style tests, and workspace tests pass; workspace evidence is in `target/audit_tmp/a59_workspace.log`. |
| A60 | Fixed and verified | location_lib/src/location.rs | An overlong file path supplied to `Location::new` failed bounded conversion, then stored the validation error text as the location file. The constructor now retains a bounded prefix of the original path through `From<LocationFileRef>`. A deterministic regression covers an oversized path and checks its original prefix and bounded length. Full Clippy, all 306 code-style tests, and workspace tests pass; evidence is in `target/audit_tmp/a60_clippy.log`, `a60_style.log`, and `a60_workspace.log`. |
| A61 | Fixed and verified | init_env_files/src/initialize.rs | The initializer used `filter_map(toml::Value::as_str)` on the workspace members array, silently skipping non-string entries. It now validates each member through a typed TOML wrapper and returns `InvalidMemberType` for a non-string entry. A deterministic conversion regression covers a numeric member. Full Clippy, all 306 code-style tests, and workspace tests pass; evidence is in `target/audit_tmp/a61_clippy.log`, `a61_style.log`, and `a61_workspace.log`. |
| A62 | Fixed and verified | proc_macro_location_bang/src/lib.rs, proc_macro_location_bang/tests | The `location!` proc macro silently accepted unexpected arguments. It now emits a compile error for nonempty input. A dependency-free rustc fixture compiled before the fix and is rejected with the expected diagnostic afterward (`target/audit_tmp/a121_location_before.log`, `a121_location_after.log`); the existing `location_test` package builds valid invocations. Formatting, full Clippy, code-style tests, workspace tests, and `git diff --check` passed (`target/audit_tmp/a121_clippy.log`, `a121_style.log`, `a121_workspace.log`). Run the negative fixture with `bash proc_macro_location_bang/tests/test_invalid_input.sh`. |
| A63 | Fixed and verified | proc_macro_config_lib_shared/src/lib.rs | The shared nonempty config-text generator emitted raw `String` fields and its direct `TryFrom<String>` accepted arbitrary length, bypassing the workspace configuration text bound. Generated wrappers now store `BoundedString` and return `TooLong` for values above the shared limit. The related `ServerConfig` provider returns the bounded field type. A deterministic regression checks all three generated config text types. Full Clippy, all 306 code-style tests, and workspace tests pass; evidence is in `target/audit_tmp/a63_clippy.log`, `a63_style.log`, and `a63_workspace.log`. |
| A64 | Fixed and verified | workspace_scaffold/src/template_fs_replace_file.rs | Template replacement treated every bounded read failure as a binary file and returned success, hiding missing files and oversized templates. It now skips only invalid UTF-8 and propagates other read failures. A deterministic missing-file regression covers the I/O error path. Full Clippy, code-style tests, and workspace tests pass; evidence is in `target/audit_tmp/a64_clippy.log`, `a64_style.log`, and `a64_workspace.log`. |
| A65 | Fixed and verified | workspace_scaffold/src/synchronize_deployment_projections.rs | Deployment synchronization validated catalog path components only after it had begun updating generated files. A regression with a parent-directory component failed before the fix because projection handling ran first. Path validation now runs immediately after catalog parsing and before all projection reads and writes. Full Clippy, code-style tests, and workspace tests pass; evidence is in `target/audit_tmp/a65_clippy.log`, `a65_style.log`, and `a65_workspace.log`. |
| A66 | Fixed and verified | workspace_scaffold/src/template_fs_copy_template_tree.rs | Template copying followed symlinked files and directories from the template source. A symlink could copy content outside the template tree or recurse into an ancestor. The copier now rejects symlinked source directories and entries; a deterministic Unix regression covers file and directory links. Full Clippy, code-style tests, and workspace tests pass; evidence is in `target/audit_tmp/a66_clippy.log`, `a66_style.log`, and `a66_workspace.log`. |
| A67 | Fixed and verified | workspace_scaffold/src/synchronize_generated_file.rs | Generated-file synchronization accepted duplicate begin and end markers when both blocks already contained the expected text, reporting success while leaving duplicate generated sections. A regression failed before the fix. The synchronizer now requires exactly one nonempty begin marker and one nonempty end marker. Full Clippy, code-style tests, and workspace tests pass; evidence is in `target/audit_tmp/a67_clippy.log`, `a67_style.log`, and `a67_workspace.log`. |
| A68 | Fixed and verified | workspace_scaffold/src/service_catalog_parse.rs | Invalid `port` and `release` values after valid assignments were silently ignored, so malformed service catalogs passed parsing. A regression failed before the fix. Recognized numeric and boolean assignments now return `Catalog` on parse failure. Full Clippy, code-style tests, and workspace tests pass; evidence is in `target/audit_tmp/a68_clippy.log`, `a68_style.log`, and `a68_workspace.log`. |
| A69 | Fixed and verified | workspace_scaffold/src/service_catalog_string_value.rs | The shared string-field parser returned `None` for unquoted assignments, letting a malformed field after an earlier valid value pass catalog parsing. An end-to-end catalog regression failed before the fix. Recognized assignments now return `Catalog` unless their values are quoted. Full Clippy, code-style tests, and workspace tests pass; evidence is in `target/audit_tmp/a69_clippy.log`, `a69_style.log`, and `a69_workspace.log`. |
| A70 | Fixed and verified | workspace_scaffold/src/service_catalog_parse.rs | The service catalog parser silently overwrote duplicate fields, accepting catalogs with repeated keys and using the last value. A regression failed before the fix; its final form covers all nine recognized fields. Every recognized assignment now rejects an existing value with `Catalog`. Full Clippy, code-style tests, and workspace tests pass; evidence is in `target/audit_tmp/a70_clippy.log`, `a70_style.log`, and `a70_workspace.log`. |
| A71 | Fixed and verified | workspace_scaffold/src/service_catalog_parse.rs | The parser ignored unknown nonempty lines before and inside service entries, allowing misspelled or unsupported catalog keys to pass. A regression failed before the fix for both positions. The parser now permits blank lines and comments and rejects other unrecognized lines with `Catalog`. Full Clippy, code-style tests, and workspace tests pass; evidence is in `target/audit_tmp/a71_clippy.log`, `a71_style.log`, and `a71_workspace.log`. |
| A72 | Fixed and verified | workspace_scaffold/src/template_fs_replace_file.rs | Sequential template replacements also rewrote text inserted by earlier replacements. A regression failed before the fix: a repository URL containing the project template token was changed by the later project-name replacement. Replacements now scan only the original template text and copy inserted values verbatim. An empty-pattern regression covers the required progress guard. Full Clippy, code-style tests, and workspace tests pass; evidence is in `target/audit_tmp/a72_clippy.log`, `a72_style.log`, and `a72_workspace.log`. |
| A73 | Fixed and verified | workspace_scaffold/src/service_catalog_string_value.rs | Quoted service-catalog values retained TOML escapes literally, so a value such as `ser\u0076er` reached projections with the wrong name. Focused regressions failed before the fix. Single-line basic-string escapes are now decoded; invalid escapes, control characters, and unescaped quotes are rejected. The parser integration regression and all 26 scaffold tests pass. Formatting, full Clippy, code-style tests, and workspace tests pass; evidence is in `target/audit_tmp/a73_clippy.log`, `a73_style.log`, and `a73_workspace.log`. |
| A74 | Fixed and verified | workspace_scaffold/src/naming_validate_project_name.rs | Project and service names beginning with a digit passed validation, but the pinned Cargo toolchain rejects such package names. A local `cargo new --lib --name 1foo` probe and a failing validator regression confirm the mismatch. Validation now requires a lowercase ASCII letter first, and the CLI error text states the rule. Full Clippy, code-style tests, and workspace tests pass; evidence is in `target/audit_tmp/a74_clippy.log`, `a74_style.log`, and `a74_workspace.log`. |
| A75 | Fixed and verified | workspace_scaffold/src/main.rs | The CLI duplicated a test-only URL validator that accepted a hostless URL with a path, such as `https:///notification_service`. A focused regression failed before the fix; the bare `https://` form was already rejected by the trailing-slash rule. The validator now checks for a host and the CLI calls it directly. Full Clippy, code-style tests, and workspace tests pass; evidence is in `target/audit_tmp/a75_clippy.log`, `a75_style.log`, and `a75_workspace.log`. |
| A76 | Fixed and verified | workspace_scaffold/src/naming_validate_project_name.rs | Validated project names had no length limit, so overlong names could reach bounded conversion helpers, which previously returned their validation error text as the generated name. A validator regression failed before the fix. Validation now enforces the scaffold text bound, and all name conversion helpers return `ProjectName` on overflow; a regression covers the validator and all three conversion forms. Full Clippy, code-style tests, and workspace tests pass; evidence is in `target/audit_tmp/a76_clippy.log`, `a76_style.log`, and `a76_workspace.log`. |
| A77 | Fixed and verified | workspace_scaffold/src/template_fs_insert_once.rs | Marker insertion returned success whenever a replacement already appeared anywhere in the file, leaving another marker unchanged. A regression failed before the fix. Insertion now searches outside existing replacement spans and keeps repeated calls idempotent; tests cover both cases and an empty marker. Full Clippy, code-style tests, and workspace tests pass; evidence is in `target/audit_tmp/a77_clippy.log`, `a77_style.log`, and `a77_workspace.log`. |
| A78 | Fixed and verified | workspace_scaffold/src/service_catalog_parse.rs | The service catalog accepted port zero even though service scaffolding rejects it. A catalog regression failed before the fix. Port parsing now returns `Catalog` for zero. Full Clippy, code-style tests, and workspace tests pass; evidence is in `target/audit_tmp/a78_clippy.log`, `a78_style.log`, and `a78_workspace.log`. |
| A79 | Fixed and verified | workspace_test_runner/src/check_tool_available.rs | Tool availability used `Path::exists`, so an existing directory was reported as a tool; the memusage branches could also try to load a directory at their configured path. A directory regression failed before the initial fix. The availability helper requires a regular file, and both memusage call sites now use that helper. The runner's 17 unit tests, formatting, Clippy, code-style suite, and workspace tests pass after this follow-up. |
| A80 | Fixed and verified | workspace_test_runner/src/run_commands.rs | `Command::output` buffered unbounded stdout and stderr; an output above the 16 MiB `CommandText` limit became conversion-error text in the report. The shared command owner now drains both pipes concurrently and retains the latest 2 MiB per stream, bounding capture memory and keeping failure output near the end. Tail-buffer, process, and worst-case invalid UTF-8 regressions pass. The runner now prints and reports retained output for oversized commands. Full Clippy, code-style tests, and workspace tests pass; evidence is in `target/audit_tmp/a80_clippy.log`, `a80_style.log`, and `a80_workspace.log`. |
| A81 | Fixed and verified | workspace_test_runner/src/failed_test_names.rs | Failed-test extraction parsed raw subprocess text, so ANSI color codes before a failure line hid that test name in the summary. A colored-log regression failed before the fix. The parser now removes ANSI sequences with the existing runner helper before extracting names. Full Clippy, code-style tests, and workspace tests pass; evidence is in `target/audit_tmp/a81_clippy.log`, `a81_style.log`, and `a81_workspace.log`. |
| A82 | Fixed and verified | workspace_test_runner/src/main.rs | An oversized CLI mode was converted into a bounded-string diagnostic, then reported as an unknown mode instead of a length error. The binary regression failed before the fix and now checks the direct length error with a nonzero exit and no unknown-mode fallback. All runner tests, full Clippy, code-style tests, and workspace tests pass; evidence is in `target/audit_tmp/a82_clippy.log`, `a82_style.log`, and `a82_workspace.log`. |
| A83 | Fixed and verified | workspace_test_runner/src/memusage_summary_text.rs | Memusage parsers scanned all captured stderr, so program text before the tool footer could be reported as heap or allocation measurements. A spoofed-prefix regression failed before the fix. Both parsers now read only after the final `Memory usage summary:` marker, including values on the marker line; missing-footer input returns `unavailable`. The real installed tool's footer format was inspected locally. Full Clippy, code-style tests, and workspace tests pass; evidence is in `target/audit_tmp/a83_clippy.log`, `a83_style.log`, and `a83_workspace.log`. |
| A84 | Fixed and verified | workspace_test_runner/src/print_without_memusage_footer.rs | The stderr printer stopped at the first `Memory usage summary:` marker while A83's parser uses the final marker, so a program's marker-like line could hide subsequent program stderr. Printing now stops at the final marker and preserves all text when no footer exists. Focused helper tests, formatting, full Clippy, code-style tests, and workspace tests pass; evidence is in `target/audit_tmp/a84_clippy.log`, `a84_style.log`, and `a84_workspace_confirm.log`. |
| A85 | Fixed and verified | workspace_test_runner/src/measure_cargo_command.rs | The Cargo timing parser took the first resource marker from all stderr, and the companion printer filtered every matching line. Program stderr could spoof measurements and lose matching lines. A shared footer parser now requires the final three numeric `/usr/bin/time` lines, reads their values, and preserves all preceding program stderr, including matching markers and text without a trailing newline. Three deterministic footer regressions pass. Formatting, full Clippy, code-style tests, and workspace tests pass; evidence is in `target/audit_tmp/a85_clippy.log`, `a85_style.log`, and `a85_workspace.log`. |
| A86 | Fixed and verified | workspace_test_runner/src/domain_types.rs | Three runner subprocess paths still used unbounded `Command::output`: the Cargo subcommand probe, Cargo measurements, and memusage measurements. All now use the shared bounded capture with a single 2 MiB per-stream limit also used by `run_commands`. A deterministic worst-case invalid UTF-8 regression proves the retained memusage text fits its 16 MiB wrapper; the existing command-log boundary and process tests cover the same capture helper. No direct unbounded output call remains in the runner. Formatting, full Clippy, code-style tests, and workspace tests pass; evidence is in `target/audit_tmp/a86_clippy.log`, `a86_style.log`, and `a86_workspace.log`. |
| A87 | Fixed and verified | workspace_test_runner/src/run_measurements_cli.rs | Three generation-stage failures called `panic_any` in the production measurement CLI. Each now prints the stage error to stderr and exits with failure, matching the runner's other measurement errors. A deterministic regression covers parse, build, and validation error propagation through the shared stage helper. Focused runner test, formatting, full Clippy, code-style tests, and workspace tests pass; evidence is in `target/audit_tmp/a87_focused.log`, `a87_clippy.log`, `a87_style.log`, and `a87_workspace.log`. |
| A88 | Fixed and verified | workspace_test_runner/src/run_commands.rs | The report sorted successful command results by index but assigned `usize::MAX` to panicked workers, moving an earlier panic summary after later commands. The sort was removed; scoped joins already collect results in input order. A deterministic synthetic panic-result test documents that ordering without scheduling threads. The obsolete sentinel constant and reviewed `usize::MAX` inventory entry were removed. Focused test, formatting, full Clippy, code-style tests, workspace tests, and `git diff --check` pass (`target/audit_tmp/a88_focused.log`, `a88_clippy_final.log`, `a88_style_final.log`, `a88_workspace.log`). The test covers the join-result ordering invariant, while the production report order follows directly from the same unmodified vector. |
| A89 | Fixed and verified | workspace_test_runner/src/admin_fixture_conversion_error.rs | A regression showed that `Error::source` was absent for wrapped fixture validation errors. The bounded-string generator now has an opt-in `error` mode that derives `thiserror::Error` with the prior messages; the ten affected wrappers in the runner and admin contract enable it, and every fixture conversion variant marks its source. Only crates already depending on `thiserror` use the option. The reviewed public contract snapshot records the nine public attribute changes. The source-chain regression failed before the fix and now passes; formatting, full Clippy, code-style tests, and workspace tests pass. Evidence is in `target/audit_tmp/a88_before.log`, `a89_focused.log`, `a89_clippy.log`, `a89_style_final.log`, and `a89_workspace.log`. |
| A90 | Fixed and verified | frontend_admin/src/admin_csr_query.rs | Browser query parsing rejected duplicate `limit` and `offset` parameters but accepted the first value of repeated filter, search, sort, and direction parameters. It now rejects duplicates for all nine typed keys read by the parser. Seven Playwright cases join the existing pagination regressions and assert an invalid-query diagnostic with no read request. The current CSR bundle was rebuilt, and all 21 browser cases passed against a disposable PostgreSQL container that was stopped afterward. The WASM build, formatting, full Clippy, code-style tests, and workspace tests also pass. The first browser-server compilation exhausted disk space; a package-scoped Cargo clean freed regenerable output before the successful retry. Evidence is in `target/audit_tmp/a90_playwright_discovery.log`, `a90_wasm_check.log`, `a90_clippy.log`, `a90_style.log`, `a90_workspace.log`, `a90_trunk_build.log`, and `a90_browser_runtime_retry.log`. |
| A91 | Fixed and verified | frontend_admin/src/admin_page_range.rs | At or beyond the total item count, pagination reported the final item number as both range endpoints even though the page contains no rows. It now displays `0 to 0` for empty ranges while preserving partial pages and the existing navigation offsets. The updated regression failed before the fix and passes afterward for both exactly-at-end and out-of-range offsets. Formatting, full Clippy, code-style tests, and workspace tests pass; evidence is in `target/audit_tmp/a91_before.log`, `a91_focused.log`, `a91_clippy.log`, `a91_style.log`, and `a91_workspace.log`. |
| A92 | Fixed and verified | frontend_admin/src/admin_column_filter.rs | CSR table filter forms omitted the active search, sort key, and direction, so applying a column filter reset those settings on query-enabled pages. The shared table-query hidden inputs now join the filter form through the grid. A browser regression on the roles page verifies all three submitted values; the initial combined browser run passed that case, and the focused rerun passed after separating the sessions finding below. WASM check, formatting, full Clippy, all 306 code-style tests, and workspace tests pass. Evidence: `target/audit_tmp/a92_browser.log`, `a92_browser_final.log`, `a92_wasm.log`, `a92_clippy.log`, `a92_style.log`, and `a92_workspace.log`. |
| A93 | Fixed and verified | server_admin_contract/src/admin_page.rs | The sessions page rendered pagination and filter controls, but its `Csr` page mode removed their query parameters before the browser request. Its existing GET endpoint already parses typed table queries and applies filter, limit, and offset. Sessions now uses `CsrTableQuery`; the catalog test and reviewed public API snapshot were updated. A browser regression observed an empty query before the fix and verified all five requested parameters afterward. The complete pagination browser file passes all 23 cases. Formatting, full Clippy, all 306 code-style tests, and workspace tests pass. Evidence: `target/audit_tmp/a93_browser_before.log`, `a93_browser_after.log`, `a93_browser_full.log`, `a93_clippy.log`, `a93_style_final.log`, and `a93_workspace_final.log`. |
| A94 | Fixed and verified | frontend_admin/src/admin_settings_view.rs | Invalid settings values made the Save handler skip its request silently. The form now shows a validation message while preserving the entered values; the reset path also reports local construction failures. A browser regression with an invalid default route failed before the fix and passes afterward, checking that no PATCH request is sent. The new message is assembled from existing string fragments. WASM check, frontend build, formatting, full Clippy, all 306 code-style tests, and workspace tests pass. Evidence: `target/audit_tmp/a94_browser_before.log`, `a94_browser_after.log`, `a94_wasm.log`, `a94_trunk.log`, `a94_clippy.log`, `a94_style.log`, and `a94_workspace.log`. |
| A95 | Fixed and verified | frontend_admin/src/admin_column_filter.rs | Clear on a column filter linked to the bare table path, dropping the active search, sort, direction, and page size. It now submits a GET form with the shared table query inputs and limit, omitting only filter fields. The SSR grid supplies its typed table query to the same renderer. A browser regression failed before the fix, then passed with a real Clear submission and the expected URL; all 24 pagination browser cases pass. The native grid regression verifies that both forms carry descending order. WASM check, formatting, full Clippy, all 306 code-style tests, and workspace tests pass. Evidence: `target/audit_tmp/a95_browser_before.log`, `a95_browser_submit.log`, `a95_browser_full.log`, `a95_native_final.log`, `a95_wasm.log`, `a95_clippy_final.log`, `a95_style_final.log`, and `a95_workspace_final.log`. |
| A96 | Fixed and verified | proc_macro_frontend_contract_shared/src/typed_route_args.rs | Five keyed frontend contract attribute parsers silently replaced earlier values when a field appeared twice. They now reject duplicate identifiers before overwriting any value, including a repeated bare `exclude_from_family` flag. Six deterministic parser cases failed before the fix and pass afterward. The guard moves identifiers into a per-parse set without cloning inside the loop. Formatting, full Clippy, all 306 code-style tests, and workspace tests pass. Evidence: `target/audit_tmp/a96_before.log`, `a96_focused_final.log`, `a96_clippy_final.log`, `a96_style_final.log`, and `a96_workspace.log`. |
| A97 | Fixed and verified | proc_macro_frontend_contract_shared/src/lib.rs | A repeated `slice` field option silently replaced its earlier type in `ContractStructApi`. The regression failed before the duplicate guard and passes afterward. Formatting, full Clippy, all 306 code-style tests, and workspace tests pass. Evidence: `target/audit_tmp/a97_before.log`, `a97_focused.log`, `a97_clippy.log`, `a97_style.log`, and `a97_workspace.log`. |
| A98 | Fixed and verified | proc_macro_frontend_contract_shared/src/lib.rs | Repeated `delegate` metadata in `route_openapi` silently selected the later endpoint path. A focused regression failed before the duplicate guard and passes afterward. Formatting, full Clippy, all 306 code-style tests, and workspace tests pass. Evidence: `target/audit_tmp/a98_before.log`, `a98_focused.log`, `a98_clippy.log`, `a98_style.log`, and `a98_workspace.log`. |
| A99 | Fixed and verified | proc_macro_frontend_contract_shared/src/lib.rs | `RouteFamily` silently ignored a second `route_family` or `route_family_body_limit` attribute. Two deterministic regression cases failed before duplicate detection and pass afterward. Formatting, full Clippy, all 306 code-style tests, and workspace tests pass. Evidence: `target/audit_tmp/a99_before.log`, `a99_focused.log`, `a99_clippy.log`, `a99_style.log`, and `a99_workspace.log`. |
| A100 | Fixed and verified | proc_macro_frontend_contract_shared/src/lib.rs | The `TypedRoute`, `RouteCatalog`, and `PageCatalog` derives silently ignored repeated top-level attributes, while the two catalogs also ignored repeated variant attributes. Five deterministic regression cases failed before the duplicate guards and pass afterward. Formatting, full Clippy, all 306 code-style tests, and workspace tests pass. Evidence: `target/audit_tmp/a100_before.log`, `a100_focused.log`, `a100_variant_before.log`, `a100_variant_focused.log`, `a100_clippy_final.log`, `a100_style.log`, and `a100_workspace.log`. |
| A101 | Fixed and verified | proc_macro_frontend_contract_shared/src/lib.rs | The route registry accepted multiple `openapi` attributes but generated its document from only the first. A deterministic regression failed before duplicate detection and passes afterward. Formatting, full Clippy, all 306 code-style tests, and workspace tests pass. Evidence: `target/audit_tmp/a101_before.log`, `a101_focused.log`, `a101_clippy.log`, `a101_style.log`, and `a101_workspace.log`. |
| A102 | Fixed and verified | proc_macro_frontend_contract_shared/src/lib.rs | The route registry silently discarded any outer attribute besides `openapi`. A deterministic regression failed before rejection and passes afterward. Formatting, full Clippy, all 306 code-style tests, and workspace tests pass. Evidence: `target/audit_tmp/a102_before.log`, `a102_focused.log`, `a102_clippy.log`, `a102_style.log`, and `a102_workspace.log`. |
| A103 | Fixed and verified | frontend_admin/src/admin_table_actions.rs | Both session table renderers used a shared revoke dialog trigger whose accessible name and tooltip said `Delete`. The component now names that action `Revoke session`. Its existing markup regression failed before the change and passes afterward. Formatting, full Clippy, all 306 code-style tests, and workspace tests pass. Evidence: `target/audit_tmp/a103_before.log`, `a103_focused.log`, `a103_clippy.log`, `a103_style.log`, and `a103_workspace.log`. |
| A104 | Fixed and verified | frontend_admin/src/fetch_permission_table_read.rs, server_admin_contract/src/admin_where_many.rs | Permission table lists discarded column filters before constructing their read requests. A browser regression failed before the fix because the request omitted `where_many`. The fetch path now validates each table's filter field and forwards typed filters for all three permission tables. The shared converter now accepts numeric relation columns, verified by a direct unit test. The browser regression verifies text and numeric filters and successful read responses across all three tables. Formatting, full Clippy, all 306 code-style tests, and workspace tests pass. Evidence: `target/audit_tmp/a104_browser_before.log`, `a104_unit.log`, `a104_browser_response.log`, `a104_clippy_final.log`, `a104_style.log`, and `a104_workspace.log`. |
| A105 | Fixed and verified | frontend_contract/src/parse_timestamp_filter_wire_json.rs, server_admin_contract/src/admin_where_many.rs, pg_crud_pg_types_generate_src/src/emit_generate_pg_types.rs | CSR fetch paths selected `InputKind::DateTime` for visible date-time filters, but the shared converter rejected that kind. The browser regression failed before the fix because no read request was sent. Timestamp form parsing now lives in `frontend_contract` and supplies the generated PostgreSQL type converter and CSR filter builder. Scalar, range, and list unit tests and the browser read-response regression pass. The reviewed public API snapshot records the shared parser and two typed timestamp diagnostics. Formatting, full Clippy, all 306 code-style tests, and workspace tests pass; the first workspace run exhausted the incremental-build disk cache, and a cache-only cleanup plus incremental-disabled retry passed. Evidence: `target/audit_tmp/a105_datetime_before.log`, `a105_browser_before_fix.log`, `a105_shared_unit.log`, `a105_timestamp_unit.log`, `a105_generated_unit.log`, `a105_browser_after.log`, `a105_clippy_final3.log`, `a105_style_final.log`, and `a105_workspace_retry.log`. |
| A106 | Fixed and verified | frontend_admin/src/fetch_rules_read.rs | An unknown rules filter field was treated as numeric and sent to `/rules/read`, which returned HTTP 400. The CSR fetcher now rejects unknown fields as an invalid table query before making a read request. Browser regression failed before and passed after the fix (`target/audit_tmp/a106_browser_before.log`, `target/audit_tmp/a106_browser_after.log`); formatting, Clippy, code style, and workspace tests passed (`target/audit_tmp/a106_clippy.log`, `target/audit_tmp/a106_style.log`, `target/audit_tmp/a106_workspace.log`). |
| A107 | Fixed and verified | frontend_admin/src/fetch_system_settings_read.rs | An unknown system settings filter field was treated as text and sent to `/system_settings/read`. The CSR reader now checks the field against the system settings column catalog before sending a read request. The browser regression failed before and passed after the fix (`target/audit_tmp/a107_browser_before.log`, `target/audit_tmp/a107_browser_after.log`); formatting, Clippy, code style, and workspace tests passed (`target/audit_tmp/a107_clippy.log`, `target/audit_tmp/a107_style.log`, `target/audit_tmp/a107_workspace.log`). |
| A108 | Fixed and verified | frontend_admin/src/render_document.rs | The configured tab title was interpolated directly into the HTML document, allowing markup such as a closing title tag and script tag. The title now renders as escaped Leptos text. A deterministic rendering regression failed before the fix and passes afterward (`target/audit_tmp/a108_before.log`, `target/audit_tmp/a108_focused_catalog.log`); formatting, full Clippy, all 306 code-style tests, and workspace tests passed (`target/audit_tmp/a108_clippy_final.log`, `target/audit_tmp/a108_style_final.log`, `target/audit_tmp/a108_workspace.log`). |
| A110 | Fixed and verified | frontend_admin/src/admin_data_table_grid.rs, frontend_admin/src/render_roles.rs | Both role table renderers offered an Update link for system roles even though the role update operation rejects them. The CSR table now shows Update only when the `is_system` cell is `false`, and the SSR table checks the typed role flag. The browser regression failed before and passed after the fix while retaining a custom-role Update link (`target/audit_tmp/a110_browser_before.log`, `target/audit_tmp/a110_browser_after.log`). The new SSR assertion failed with two Update links before the guard and passed after it (`target/audit_tmp/a110_ssr_before.log`, `target/audit_tmp/a110_ssr_focused.log`). The CSR build, formatting, full Clippy, all 306 code-style tests, and workspace tests passed (`target/audit_tmp/a110_trunk.log`, `target/audit_tmp/a110_clippy_final.log`, `target/audit_tmp/a110_style_final.log`, `target/audit_tmp/a110_workspace_final.log`). |
| A111 | Fixed and verified | server_admin_contract/src/admin_bounded_vec.rs, server_admin_contract/src/admin_collection_error.rs, server_admin/src, proc_macro_frontend_contract_shared/src/lib.rs | Collection conversion discarded its `BoundedValueError`, and five mutation adapters discarded the collection error. `TooLong` now carries its source; the adapters, server error, and generated typed-operation errors preserve the chain while retaining validation status. Focused tests cover the bounded lengths and both server error layers (`target/audit_tmp/a111_collection.log`, `target/audit_tmp/a111_server.log`). Formatting, full Clippy, code-style tests, and workspace tests passed after updating the reviewed API snapshot and removing the obsolete ignored-error inventory entry (`target/audit_tmp/a111_clippy.log`, `target/audit_tmp/a111_style_final.log`, `target/audit_tmp/a111_workspace.log`). |
| A112 | Fixed and verified | server_admin_contract/src/admin_user_roles_table_route.rs, server_admin_contract/src/admin_rate_limits_table_route.rs, server_admin_contract/src/admin_login_attempts_table_route.rs, server_admin_contract/src/admin_cleanup_status_table_route.rs, server_admin_contract/src/admin_refresh_tokens_table_route.rs | Five table route contracts advertised `Authenticated` although the shared table handler checks each table's read rule. They now declare their catalog rule. A catalog-wide contract test failed before the fix and passed after (`target/audit_tmp/a112_before.log`, `target/audit_tmp/a112_after.log`). The reviewed public API snapshot was updated, and formatting, full Clippy, the focused snapshot test, and workspace tests passed (`target/audit_tmp/a112_clippy.log`, `target/audit_tmp/a112_snapshot.log`, `target/audit_tmp/a112_workspace.log`). The full code-style suite passed after the snapshot update (`target/audit_tmp/a112_style_final.log`). |
| A113 | Fixed and verified | server_admin_contract/src/admin_data_filter.rs | Deserialization accepted an operation and incompatible `value_shape`, so frontend input requirements could disagree with the selected operation. The decoder now checks the derived shape and rejects mismatches; the regression failed before and passed after (`target/audit_tmp/a113_before.log`, `target/audit_tmp/a113_after.log`). Formatting, full Clippy, code-style tests, and workspace tests passed after updating the reviewed public API snapshot (`target/audit_tmp/a113_clippy.log`, `target/audit_tmp/a113_style_final.log`, `target/audit_tmp/a113_workspace.log`). |
| A114 | Fixed and verified | server_admin_contract/src/admin_data_table_query.rs, server_admin/src/data_tables_get.rs, server_admin/src/data_table_query_sql.rs | Generic table queries accepted `search`, `sort`, and `direction`, but the shared handler ignored them. The handler now binds search text, matches sort keys against static catalog columns, applies the selected direction with a stable identifier tie-breaker, and rejects descending direction without a sort key. A pure SQL builder keeps filter, search, limit, and offset placeholders ordered; focused tests cover combined queries, invalid sort, and default and search behavior across all 15 tables (`target/audit_tmp/a114_focused.log`). Formatting, full Clippy, code-style tests, and workspace tests passed after updating the typed-spec source check and obsolete ignored-error inventory (`target/audit_tmp/a114_clippy_final.log`, `target/audit_tmp/a114_style_final.log`, `target/audit_tmp/a114_workspace.log`). Database execution remains covered only by provisioned ignored tests. |
| A115 | Fixed and verified | server_admin_contract/src/admin_main_logo.rs, server_admin_contract/src/admin_support_url.rs, text_policy/src/validate_https_url_text.rs | Both administrator URL settings accepted malformed authorities such as a nonnumeric port. The shared text validator now checks the HTTPS authority, host labels, and optional numeric port while preserving valid HTTPS paths. The contract regression failed before and passed after (`target/audit_tmp/a115_before.log`, `target/audit_tmp/a115_after.log`); focused shared-validator tests passed (`target/audit_tmp/a115_text_policy.log`). Formatting, full Clippy, code-style tests, and workspace tests passed after the reviewed public API snapshot update (`target/audit_tmp/a115_clippy_final.log`, `target/audit_tmp/a115_style_final.log`, `target/audit_tmp/a115_workspace.log`). |
| A116 | Fixed and verified | server_admin_contract/src/admin_page_limit_visitor.rs, server_admin_contract/src/admin_page_offset_visitor.rs, server_admin_contract/src/visit_checked_unsigned_integer.rs | Page limit and offset deserialization rejected valid nonnegative signed and 128-bit integer inputs from generic serde deserializers. The visitors now accept representable integers through shared checked conversion while retaining range validation. The regression failed before and passed after (`target/audit_tmp/a116_before.log`, `target/audit_tmp/a116_wide_before.log`, `target/audit_tmp/a116_focused_final.log`). Formatting, full Clippy, code-style tests, and workspace tests passed (`target/audit_tmp/a116_clippy_final.log`, `target/audit_tmp/a116_style_final2.log`, `target/audit_tmp/a116_workspace.log`). |
| A117 | Fixed; browser execution pending | server_admin/src/data_tables_get.rs, server_admin_core/src/escape_admin_like_pattern.rs, browser_acceptance/tests/test_table_api_paths.spec.js | Generic table search bound user text directly into a PostgreSQL `ILIKE` pattern, so `%`, `_`, and backslash had pattern meaning. Search now escapes those characters in a shared bounded helper before binding the same value to count and data queries. Unit tests cover all three characters and output bounds; a provisioned browser regression checks `%` and `_` against existing user-role rows. A contract test confirms the browser test's empty and search-only JSON payloads deserialize (`target/audit_tmp/a117_payload.log`). Formatting, full Clippy, code-style tests, and workspace tests passed after that test addition (`target/audit_tmp/a117_clippy_payload.log`, `target/audit_tmp/a117_style_payload.log`, `target/audit_tmp/a117_workspace_payload.log`); browser execution still needs a provisioned environment. [PostgreSQL documents backslash as the default `LIKE` escape character](https://www.postgresql.org/docs/current/functions-matching.html). |
| A118 | Fixed and verified | server_admin_contract/src/admin_data_filters.rs | The advertised maximum of 100 filters per data column was not enforced: the wrapper used the general 10,000-item collection. It now stores a collection bounded to 100; construction and deserialization reject 101 items while 100 remains valid. The regression failed before and passed after (`target/audit_tmp/a118_before.log`, `target/audit_tmp/a118_after_final.log`). Formatting, full Clippy, code-style tests, and workspace tests passed after the reviewed public API snapshot update (`target/audit_tmp/a118_clippy_final.log`, `target/audit_tmp/a118_style_final.log`, `target/audit_tmp/a118_workspace.log`). |
| A119 | Fixed and verified | server_admin_contract/src/admin_optional_settings.rs | The settings update schema and handler limit the clear list to six fields, but the wrapper accepted up to 10,000. The wrapper and its schema now enforce six, so construction and deserialization reject seven while all six catalog settings remain valid. The regression failed before and passed after (`target/audit_tmp/a119_before.log`, `target/audit_tmp/a119_after.log`). Formatting, full Clippy, code-style tests, and workspace tests passed after the reviewed public API snapshot update (`target/audit_tmp/a119_clippy.log`, `target/audit_tmp/a119_style_final.log`, `target/audit_tmp/a119_workspace.log`). |
| A120 | Fixed and verified | frontend_contract/src/validate_route_coverage.rs | Duplicate detection compared all metadata, allowing two route descriptors with the same HTTP method and path when their operation IDs differed. It now compares method and path, preserving distinct methods at one path. The regression failed before and passed after (`target/audit_tmp/a122_before.log`, `target/audit_tmp/a122_focused.log`); frontend contract tests, formatting, full Clippy, code-style tests, workspace tests, and `git diff --check` passed (`target/audit_tmp/a122_frontend_contract.log`, `a122_clippy.log`, `a122_style.log`, `a122_workspace.log`). |
| A121 | Fixed and verified | frontend_contract_validation/src/validate_openapi_contract.rs | The validator rejected duplicate documented methods but accepted repeated runtime routes for one method and path. It now rejects the second runtime entry with the existing typed `DuplicateOperation` error before route matching. The regression failed before and passed after (`target/audit_tmp/a122_runtime_before.log`, `a122_runtime_after.log`); validation crate tests, formatting, full Clippy, code-style tests, workspace tests, and `git diff --check` passed (`target/audit_tmp/a122_validation.log`, `a122_clippy.log`, `a122_style.log`, `a122_workspace.log`). |
| A122 | Fixed and verified | frontend_contract_validation/src/validate_openapi_operations.rs | Required security passed if any OpenAPI security alternative named the expected scheme, even when another alternative was anonymous. The validator now requires a nonempty list where every alternative names that scheme. The regression failed before and passed after (`target/audit_tmp/a123_before.log`, `a123_after.log`); validation crate tests, formatting, full Clippy, code-style tests, workspace tests, and `git diff --check` passed (`target/audit_tmp/a123_validation.log`, `a123_clippy.log`, `a123_style.log`, `a123_workspace.log`). The [OpenAPI specification](https://spec.openapis.org/oas/v3.1.0.html#security-requirement-object) defines alternatives as OR and an empty requirement as anonymous access. |
| A123 | Fixed and verified | macro_helpers/src/generate_serde_version_of_named_syn_variant.rs | Malformed HashMap and Vec field types reached assertions and panicked during macro expansion. The generator now emits compile-error tokens for an unexpected collection name, generic count, or path shape. A regression covers four malformed shapes; focused test, formatting, full Clippy, code-style suite, and workspace tests passed (`target/audit_tmp/a123_focused.log`, `a123_clippy.log`, `a123_style.log`, `a123_workspace.log`). |
| A124 | Fixed and verified | macro_helpers/src/location_field_attr.rs | The location derive's shared attribute parser accepted arguments on marker attributes such as `#[eo_location(unexpected)]` and `#[eo_location = "unexpected"]`. Both now return a specific error; supported bare markers, unrelated attributes, missing markers, and duplicate markers retain their prior outcomes. The malformed-input regression failed before and passed after (`target/audit_tmp/a124_before.log`, `a124_after.log`). Formatting, macro_helpers tests, full Clippy, code-style suite, and workspace tests passed (`target/audit_tmp/a124_macro_helpers.log`, `a124_clippy_final.log`, `a124_style.log`, `a124_workspace.log`). |
| A125 | Fixed and verified | macro_helpers/src/only_one.rs | The status-code selector treated attributes such as `#[not_found_404(unexpected)]` and `#[not_found_404 = "unexpected"]` as valid bare markers. It now returns a distinct `MalformedAttribute` error while preserving missing and duplicate marker errors. The regression failed before and passed after (`target/audit_tmp/a125_before.log`, `a125_after.log`). Formatting, full Clippy, code-style suite, workspace tests, and `git diff --check` passed (`target/audit_tmp/a125_clippy.log`, `a125_style.log`, `a125_workspace.log`). The new public error variant is an intentional API correction for malformed input. |
| A126 | Fixed and verified | macro_helpers/src/try_write_string_into_file_with_outcome.rs | A generated `.rs` file with invalid UTF-8 and the same byte length as replacement text could not be regenerated: the changed-content branch validated the old file and returned an error. That validation is removed, so a changed file is replaced with the supplied text regardless of old encoding; unchanged files still avoid a rewrite. The regression failed before and passed after (`target/audit_tmp/a126_before.log`, `a126_after.log`). Former validation helpers are now test-only. Formatting, macro_helpers tests, full Clippy, code-style suite, workspace tests, and `git diff --check` passed (`target/audit_tmp/a126_macro_helpers.log`, `a126_clippy_final.log`, `a126_style.log`, `a126_workspace.log`). |
| A127 | Fixed and verified | macro_helpers/src/try_maybe_write_token_stream_into_file.rs | If rustfmt failed after writing token text, retrying the same token stream skipped formatting because the file bytes were unchanged and returned success. Formatting now runs whenever requested, so a retry observes the formatter failure again or can complete after the formatter is repaired. The two-call regression failed before and passed after (`target/audit_tmp/a127_before.log`, `a127_after.log`). Formatting, full Clippy, code-style suite, workspace tests, and `git diff --check` passed (`target/audit_tmp/a127_clippy.log`, `a127_style.log`, `a127_workspace.log`). |
| A128 | Fixed and verified | macro_helpers/src/validate_test_database_url.rs | The test database guard checked the URL authority and path but accepted `host`, `hostaddr`, and `dbname` query parameters that SQLx 0.9.0 applies afterward, allowing a checked loopback test URL to target a different server or database. It now rejects those query keys and conservatively rejects encoded keys; ordinary query parameters remain accepted. The regression failed before and passed after (`target/audit_tmp/a129_before.log`, `a129_after.log`). The pinned SQLx parser was inspected locally, and [SQLx connection option documentation](https://docs.rs/sqlx/latest/sqlx/postgres/struct.PgConnectOptions.html) lists these URL parameters. Formatting, macro_helpers tests with `test-utils`, full Clippy, code-style suite, workspace tests, and `git diff --check` passed (`target/audit_tmp/a129_macro_helpers.log`, `a129_clippy_final.log`, `a129_style.log`, `a129_workspace.log`). |
| A129 | Fixed and verified | macro_helpers/src/tool_ansi_chars.rs | ANSI stripping consumed characters until `m` for every escape, dropping report text after CSI controls with another final byte or OSC controls. The parser now recognizes CSI final bytes and OSC BEL or ST terminators. A regression covering CSI erase and both OSC forms failed before and passed after (`target/audit_tmp/a130_before.log`, `a130_after.log`). [ECMA-48](https://ecma-international.org/wp-content/uploads/ECMA-48_5th_edition_june_1991.pdf) defines the CSI final-byte range; [xterm control sequences](https://www.x.org/docs/xterm/ctlseqs.pdf) describe OSC terminators. Formatting, macro_helpers tests, full Clippy, code-style suite, workspace tests, and `git diff --check` passed (`target/audit_tmp/a130_macro_helpers.log`, `a130_clippy_final2.log`, `a130_style.log`, `a130_workspace.log`). |
| A130 | Fixed and verified | macro_helpers/src/generate_serde_version_of_named_syn_variant.rs | `#[eo_vec_location]` on `Option<Location>` was accepted because the generator checked only that the type had one path segment and one generic argument, then emitted a `Vec` serde field. It now requires the `Vec` identifier and emits a specific compile error for another type. The regression failed before and passed after (`target/audit_tmp/a132_before.log`, `a132_after.log`). Formatting, full Clippy, code-style suite, workspace tests, and `git diff --check` passed (`target/audit_tmp/a132_clippy.log`, `a132_style.log`, `a132_workspace.log`). |
| A131 | Fixed and verified | macro_helpers/src/should_write_string_into_file_tests.rs | A regression test reproduced the test-only predicate rejecting an invalid UTF-8 file with changed content of the same length. Removed its obsolete UTF-8 validation and the two unused test-only modules; the predicate now agrees with the production writer fixed in A126. Focused test and workspace gates passed. |
| A132 | Fixed and verified | pg_crud_common/src/build_date_sql_filter.rs | The builder incremented its bind index after emitting the final active bound, rejecting a valid single-bound fragment starting at `u32::MAX`. The regression failed before and passed after. It now advances only when another bound remains; a second bound at that start still returns `BindIndexOverflow`. Focused and workspace checks passed. |
| A133 | Fixed and verified | pg_crud_common/src/cursor_payload.rs, signed_cursor.rs | Oversized cursor values returned the `Empty` error and the false message "must not be empty." Regressions failed before and passed after. Both wrappers now distinguish empty and oversized input with `Empty` and `TooLong` variants, including bounded-storage error mapping. The new public variants are intentional error API corrections. Focused and workspace checks passed. |
| A134 | Fixed and verified | pg_crud_common/src/pg_bounded_vec.rs | Every `PgBoundedVec<T, MIN, MAX>` Utoipa schema used the same `BoundedVec` component name, allowing references with different bounds or item types to collide. A regression failed before and passed after. Component names now encode bounds and the full item type name; this intentional OpenAPI component API correction prevents collisions for nested generic item types. Focused and workspace checks passed. |
| A135 | Fixed and verified | pg_crud_common/src/pg_bounded_vec.rs, bounded_types/src/bounded_vec.rs | The manual Utoipa `ToSchema` implementations used the default empty `schemas` method, so custom item components were omitted from recursive schema registration. A regression failed before and passed after. A shared helper in `bounded_types` now registers item schemas and forwards nested schemas for both vectors and the existing `AdminOpenApiVec` adapter. Focused and workspace checks passed. |
| A136 | Fixed and verified | pg_crud_common/src/single_or_multiple.rs, explicit_value.rs, not_empty_unique_vec.rs, order_by.rs, pg_type_where.rs | Five generic Utoipa wrappers omitted one or more components referenced by their schemas. Each missing item registration was reproduced in focused tests. The wrappers now use the shared schema registration helper and recursively include nested wrapper, `Order`, and `Operator` components where referenced. Focused and workspace checks passed. |
| A137 | Fixed and verified | pg_crud_common/src/pg_type_where.rs, nullable_json_obj_pg_type_where_filter.rs | Two SQL fragment builders converted a `TooLong` validation error into a successful fragment containing diagnostic text. Regressions failed before and passed after. Both builders now return the existing `QueryPartError::StringWrapperTryFromString` error. Formatting, full Clippy, code-style suite, and workspace tests passed. |
| A138 | Fixed and verified | pg_crud_common/src/explicit_value.rs, not_empty_unique_vec.rs, order_by.rs, pg_type_where.rs, single_or_multiple.rs | Five generic Utoipa wrappers used one component name for every item type. Five regressions failed before and passed after. A shared hexadecimal type-name encoder now gives each concrete wrapper a distinct component key; `PgBoundedVec` reuses the encoder without changing its existing name format. Formatting, full Clippy, code-style suite, and workspace tests passed. |
| A139 | Fixed and verified | pg_crud_common/src/not_empty_unique_vec.rs | Derived `Default` created an empty collection that the validated constructor rejects. The regression failed before and passed after. `Default` now creates one default element for `T: Default`; the unused derive dependency was removed from this crate. Formatting, full Clippy, code-style suite, and workspace tests passed. |
| A140 | Fixed, verification pending | pg_crud_common/src/explicit_value.rs, non_primary_key_pg_type_read_ids.rs | `ExplicitValue<T>` serializes `value`, but both manual OpenAPI schemas declared `values`. The non-primary-key schema also allowed arbitrary non-null values despite using `Option<()>`. Regressions failed before and passed after. Both schemas now use the wire field name; the latter specifies null. |

Candidate rows are not confirmed bugs. Confirmed queued findings have a reproduction and await a fix; resolve each candidate with a reproduction or a documented dismissal.

A21 verification: all 12 frontend_contract_validation unit tests pass, including inherited root security and explicit operation overrides. Final workspace runner static verification passes all 306 code-style tests and full Clippy. Full workspace tests excluding code-style pass. Initial attempts hit temporary-file quota errors; retries use target/audit_tmp as TMPDIR without source-policy changes. Old incremental build caches were removed to reclaim disk space. Evidence: target/audit_tmp/security_static.log and target/audit_tmp/security_workspace.log. Security inheritance follows https://spec.openapis.org/oas/v3.1.1.html#operation-object.

A12/A13 verification: shared helper tests (12) and compiled pg_crud_macro_common tests (4) pass, including raw/forwarded alias names, malformed names, associated-type bounds, nested branch delimiters, string literals and both emitted variants. The invalid alias probe now reports the ordinary syntax diagnostic instead of a proc-macro panic. Final workspace runner static verification passes full Clippy and all 306 code-style tests. The initial reuse check rejected verbose duplicate compiler adapters; both now use the existing from_into conversion, with no inventory exception. A concurrent generated-project check captured the old dependency manifest while adapters were being shortened; the final complete workspace retry passes. Formatting and whitespace checks pass. Evidence: target/audit_tmp/trait_alias_before.log, target/audit_tmp/trait_alias_after.log, target/audit_tmp/bool_enum_before.log, target/audit_tmp/macro_parsers_tests.log, target/audit_tmp/macro_parsers_static_retry.log and target/audit_tmp/macro_parsers_workspace_retry.log.

A11 verification: the exact-boundary regression failed before the fix and all 13 shared helper tests pass after it. The real case_trait_pair macro now emits its ordinary identifier diagnostic instead of panicking. Final workspace runner static verification passes full Clippy and all 306 code-style tests; full workspace tests excluding code-style pass. Formatting and whitespace checks pass. Evidence: target/audit_tmp/identifier_limit_before.log, target/audit_tmp/identifier_limit_final.log, target/audit_tmp/identifier_overflow_macro_after.log, target/audit_tmp/identifier_static.log and target/audit_tmp/identifier_workspace.log.

A22 verification: all 14 shared helper tests and all 5 naming_common tests pass. The compiled move/raw-parameter fixture preserves both generated methods, and the former panic probe now emits the ordinary expected-closure diagnostic. Full Clippy and all 306 code-style tests pass through the workspace runner; full workspace tests excluding code-style pass. No dependency or inventory changes were required. Evidence: target/audit_tmp/case_trait_closure_before.log, target/audit_tmp/case_trait_closure_after.log, target/audit_tmp/case_trait_final_tests.log, target/audit_tmp/case_trait_static.log and target/audit_tmp/case_trait_workspace.log.

A10 verification: the mutation probe accepted 10,001 parts before the fix and now fails compilation with E0596. Final shared helper tests (15) and token_patterns consumer tests (5) pass, including ordered tp_parts output. All 306 code-style tests pass through the workspace runner. Test result-state and iterator-shadowing Clippy findings were corrected; final full Clippy and full workspace tests excluding code-style pass. Formatting and whitespace checks pass. Evidence: target/audit_tmp/comma_parts_before.log, target/audit_tmp/comma_parts_after.log, target/audit_tmp/comma_parts_tests_final.log, target/audit_tmp/comma_parts_static.log, target/audit_tmp/comma_parts_clippy_retry.log and target/audit_tmp/comma_parts_workspace.log.

## Previously fixed findings

These changes precede this complete-scope inventory. They do not imply that their containing crates are fully reviewed.

| Area | Fix | Verification |
| --- | --- | --- |
| file_storage | Reserved staging paths and NUL rejection; durable directory sync; collision-safe delete staging; reject staged non-files and symlinks. | Regression tests; workspace checks passed. |
| server_runtime_core | Rebinding leases at capacity; bounded initial queue and history allocations. | Regression tests; workspace checks passed. |
| server_runtime_http | Proxy range parse limit; special IPv6 outbound classification; Accept quality validation; bearer grammar; mapped IPv4 proxy matching. | Regression tests; workspace checks passed. |
| server_runtime_http | Retain task and process owners across cancelled joins; bounded diagnostic initial allocation. | Deterministic cancellation and allocation regressions; workspace checks passed. |
| pg_crud_common | Bound batch-validation initial allocation. | Regression tests; workspace checks passed. |
| bounded_types | Consistent invalid-bound validation for character strings. | Regression tests; workspace checks passed. |

Latest completed verification before this inventory: cargo fmt, workspace runner static (full Clippy and code-style suite), and workspace tests excluding the code-style crate. Evidence: /tmp/rust_bug12_static.log and /tmp/rust_bug12_workspace.log; these local logs are not durable artifacts.

## Current fix verification

A04 verification: a paused-clock maximum-duration missed-tick test failed before the fix with the pinned Tokio arithmetic panic. All 141 server_runtime_http tests pass after the fix, including an uninvoked pending callback for an unschedulable period, normal phase alignment, a Linux large-remainder fixture and retained task cancellation/shutdown. The cadence fixture bounds receives so automatic virtual-time advance cannot mask a whole-period delay. Source inspection verifies the pinned Sleep/TimerEntry is stored inline and creates no Box/Arc allocation per timer construction. All 306 code-style tests passed once through the workspace runner. Initial full Clippy found unchecked Duration operators and a shadowed deadline; checked/saturating operations with explicit invariants replaced them without new suppressions. Subsequent fixture-only assertions, shadowing and duration-unit warnings were corrected. Final full Clippy, final package tests and final complete workspace tests excluding code-style pass. Formatting and whitespace checks pass. No dependency, crate or exception inventory was added. Evidence: target/audit_tmp/interval_overflow_before.log, target/audit_tmp/interval_overflow_tests_complete.log, target/audit_tmp/interval_overflow_static.log, target/audit_tmp/interval_overflow_clippy_verified.log and target/audit_tmp/interval_overflow_workspace_final.log.

A26 verification: the original compiled maximum-interval probe reproduced an immediate refresh retry, while the revised probe returns IntervalOverflow and SkipIntervalOverflow. All 46 frontend_contract unit tests and seven integration tests pass. The expanded regression reuses the existing monotonic anchor, without another clock read, sleep, scheduling or randomness; it checks normal delay, Refreshed/Failed overflow and explicit reset. The initial static run passed full Clippy and 305 policy tests, with only the expected public API snapshot mismatch. Exactly three generated API entries were reviewed and updated: the finish result, the error variant and the skip decision. The final runner passes full Clippy and all 306 code-style tests. Full workspace tests excluding code-style pass. Formatting and whitespace checks pass. No dependency, crate or policy exception was added. Evidence: target/audit_tmp/auth_refresh_overflow_before.log, target/audit_tmp/auth_refresh_overflow_after.log, target/audit_tmp/auth_refresh_tests_final.log, target/audit_tmp/auth_refresh_static_final.log and target/audit_tmp/auth_refresh_workspace.log.

A20 verification: all three new suffix regressions fail before the production fix and pass afterward. Final frontend_contract tests pass: 46 unit tests and seven integration tests. All 306 code-style tests pass once through the workspace runner. Its initial Clippy command rejected a nested return-and_then in the new fixture; the equivalent question-mark form then required an explicit repository error return type. Final full Clippy passes with -D warnings, without suppressions, and the final package rerun verifies the corrected fixture. Full workspace tests excluding code-style finish successfully. Formatting and whitespace checks pass. No dependencies, crates or policy exceptions were added. Evidence: target/audit_tmp/api_url_suffix_before.log, target/audit_tmp/api_url_suffix_tests_verified.log, target/audit_tmp/api_url_suffix_static.log, target/audit_tmp/api_url_suffix_clippy_final.log and target/audit_tmp/api_url_suffix_workspace.log.

A24/A25 verification: all four shared-generator unit regressions pass. Real compiled probes cover lifetime/type/const forwarding with a colliding generic name, const-generic state fields, a raw field and a field-type name collision. Final static runner passes full Clippy and all 306 code-style tests; full workspace tests excluding code-style pass on the final source. Formatting and whitespace checks pass. The initial static run caught shadowed match bindings; they were removed without allowances, and the final runner verifies the complete revised implementation. No dependency or exception inventory changes were made. Evidence: target/audit_tmp/accessor_all_tests.log, target/audit_tmp/accessor_generics_roundtrip_verified.log, target/audit_tmp/accessor_fields_roundtrip_verified.log, target/audit_tmp/accessor_raw_field_verified.log, target/audit_tmp/accessor_field_collision.log, target/audit_tmp/accessor_final_static.log and target/audit_tmp/accessor_final_workspace.log.

A01 regression failed before the fix and passed after it. All 93 pg_crud_common tests pass on the final source. Full Clippy passes with -D warnings. All 304 code-style tests pass through the workspace runner retry; its overall command failed because an earlier test assertion attempted a move in a pattern guard, corrected to a borrowed comparison and then verified by the final Clippy and package test runs. The first runner attempt caught repeated diagnostic UUIDs in test setup, removed rather than suppressed. The full workspace test run completed successfully. cargo fmt --check also passes.

Local evidence: /tmp/rust_audit_sql_before.log, /tmp/rust_audit_sql_final.log, /tmp/rust_audit_static_retry.log, /tmp/rust_audit_clippy_final.log, /tmp/rust_audit_workspace.log.

A08 and A09 regressions both failed before the fixes. After sequential fixes, all workspace_macro_helpers and proc_macro_getters tests pass, including a compiled derive fixture that reads and mutates raw and prefixed fields independently. Full workspace tests and all 304 code-style tests passed. Full Clippy passed after correcting test-only enum matching, shadowing and item ordering. The runner itself reported the earlier Clippy failure; final Clippy evidence is /tmp/rust_audit_getters_clippy_retry.log. Evidence: /tmp/rust_audit_getters_before.log, /tmp/rust_audit_getters_raw_after.log, /tmp/rust_audit_getters_final.log, /tmp/rust_audit_getters_static.log, /tmp/rust_audit_getters_workspace.log.

A02 growth and read-limit tests pass. A03 paused-clock regression failed before the fix; all 137 server_runtime_http tests now pass, including retained task ownership and the Tokio Elapsed error source. Full workspace tests and all 304 code-style tests passed. The static runner reported a test-only redundant closure lint; the corrected method reference then passed full Clippy with -D warnings. cargo fmt --check and git diff --check pass. Local evidence: /tmp/rust_audit_http_capacity_final.log, /tmp/rust_audit_diagnostic_before.log, /tmp/rust_audit_http_final.log, /tmp/rust_audit_runtime_static.log, /tmp/rust_audit_runtime_clippy_final.log, /tmp/rust_audit_runtime_workspace.log.

A15 and A16 compiled regressions failed before their fixes and pass afterward. All 306 code-style tests passed through the runner. Its Clippy command rejected assertions using Result::is_err in the new fixtures; assertions were corrected without lint allowances. Final full Clippy passes after validating the error diagnostic in the assertion and preserving ref_type conversion semantics. Both final WireEnum focused tests and the final EnumFromStr diagnostic regression pass. The full workspace run completed successfully; formatter and diff whitespace checks pass. The 306 code-style tests passed before the test assertion and equivalent ref conversion refinements, and were not redundantly repeated; the runner itself reported the initial Clippy failure. Final source compile coverage is established by full Clippy and the final compiled regressions. Local evidence: /tmp/rust_audit_wire_before.log, /tmp/rust_audit_wire_after.log, /tmp/rust_audit_enum_generics_before.log, /tmp/rust_audit_enum_generics_after.log, /tmp/rust_audit_enum_static.log, /tmp/rust_audit_enum_clippy_retry.log, /tmp/rust_audit_wire_preserved_conversion.log, /tmp/rust_audit_enum_clippy_preserved.log, /tmp/rust_audit_enum_workspace.log.

A17-A19 regressions reproduced duplicate where clauses, missing generic names/arguments and premature stream closure before the fixes. All macro helper/newtype package tests pass after A17/A18. All 138 server_runtime_http tests pass. The first static run found an ASCII-only documentation violation and implicit raw-pointer coercions in the provider fixture; both were corrected without lint allowances. The static retry completed successfully: full Clippy, formatting and all 306 code-style tests passed. Full workspace tests completed successfully. The final macro package test rerun also passes with the corrected reference-identity assertion. git diff --check passes. Evidence: /tmp/rust_audit_where_before.log, /tmp/rust_audit_where_after.log, /tmp/rust_audit_generic_adapters_before.log, /tmp/rust_audit_adapters_packages.log, /tmp/rust_audit_stderr_before.log, /tmp/rust_audit_stderr_after.log, /tmp/rust_audit_adapters_static.log, /tmp/rust_audit_adapters_static_retry.log, /tmp/rust_audit_adapters_workspace.log, /tmp/rust_audit_adapters_packages_final.log.

A07 partial guard verification: all 12 frontend_contract_validation tests pass, including direct and allOf cycles, valid recursive children, independent branches and a 2,048-reference chain. All 306 code-style tests pass through the workspace runner. The runner reported test-only redundant clones and needless_for_each; corrected assertions and borrowed JSON inputs then passed full Clippy. Full workspace tests and final formatting and whitespace checks pass. The remaining deep-composition recursion is explicitly unresolved. Evidence: /tmp/rust_audit_cycle_tests_extended_final.log, /tmp/rust_audit_cycle_tests_retry.log, /tmp/rust_audit_cycle_static.log, /tmp/rust_audit_cycle_clippy_final.log, /tmp/rust_audit_cycle_workspace.log.

## Module coverage

### administrator_account_initialization_and_password_reset

| Source | Semantic review |
| --- | --- |
| `administrator_account_initialization_and_password_reset/src/admin_command.rs` | Reviewed: command variants carry the matching validated arguments. |
| `administrator_account_initialization_and_password_reset/src/administrator_account_command_error.rs` | Reviewed: command failures retain distinct operation errors. |
| `administrator_account_initialization_and_password_reset/src/administrator_account_command_exit_code.rs` | Reviewed: termination forwards the selected process exit code. |
| `administrator_account_initialization_and_password_reset/src/administrator_account_command_status.rs` | Reviewed: status wrapper preserves the exit code. |
| `administrator_account_initialization_and_password_reset/src/administrator_command_args_error.rs` | Reviewed: usage and invalid argument errors are distinct. |
| `administrator_account_initialization_and_password_reset/src/administrator_password_file_path_buf.rs` | Reviewed: wrapper exposes a typed borrowed path. |
| `administrator_account_initialization_and_password_reset/src/error_status.rs` | Reviewed: tests cover usage and already initialized status mapping. |
| `administrator_account_initialization_and_password_reset/src/initial_administrator_creation_args.rs` | Reviewed: ownership transfer preserves argument order. |
| `administrator_account_initialization_and_password_reset/src/main.rs` | Reviewed: command parsing, startup, operation dispatch, and status mapping are consistent. |
| `administrator_account_initialization_and_password_reset/src/password_from_bytes.rs` | Reviewed: test helper applies the same bounded UTF-8 and line ending policy. |
| `administrator_account_initialization_and_password_reset/src/password_from_file.rs` | Reviewed: bounded read and one optional line ending precede password validation. |
| `administrator_account_initialization_and_password_reset/src/password_reset_args.rs` | Reviewed: ownership transfer preserves argument order. |
| `administrator_account_initialization_and_password_reset/src/sqlx_administrator_database_connection_error.rs` | Reviewed: database connection error retains its source. |

### app_state

| Source | Semantic review |
| --- | --- |
| `app_state/src/lib.rs` | Reviewed: crate root declares the three pool boundary modules without behavior or state. |
| `app_state/src/sqlx_pg_pool.rs` | Reviewed: private pool wrapper forwards borrowing and owned construction through generated derives; Clone follows SQLx pool handle semantics. |
| `app_state/src/sqlx_pg_pool_provider.rs` | Reviewed: provider trait returns the borrowed typed pool reference without allocation or conversion. |
| `app_state/src/sqlx_pg_pool_ref.rs` | Reviewed: copyable borrowed pool wrapper retains the source lifetime and exposes only generated borrowing/construction. |

### bounded_string_core

| Source | Semantic review |
| --- | --- |
| `bounded_string_core/src/bounded_string_storage.rs` | Reviewed: byte and character bounds, fallible mutation without partial changes, UTF-8 truncation, unbounded-only raw mutation, conversion and deterministic tests. |
| `bounded_string_core/src/bounded_string_storage_error.rs` | Reviewed: the two length failures retain the actual and configured bounds; diagnostic formatting does not include input text. |
| `bounded_string_core/src/lib.rs` | Reviewed: flat module declarations only; implementation modules are tracked separately. |
| `bounded_string_core/src/try_from_error_text.rs` | Reviewed: A05 now performs one fallible conversion; deterministic tests cover success, a one-byte maximum and an unmet minimum. |

### bounded_types

| Source | Semantic review |
| --- | --- |
| `bounded_types/src/bounded_b_tree_map.rs` | Reviewed: capped construction and insertion, replacement at capacity, immutable keys, removal, serialization and visitor delegation; no raw mutable map access escapes. |
| `bounded_types/src/bounded_b_tree_map_visitor_phantom_data.rs` | Reviewed: bounded entry count including duplicate keys, stop before decoding overflow values, bounded HashMap preallocation and preserved deserialization errors. |
| `bounded_types/src/bounded_chars_string.rs` | Reviewed: validated construction, deserialization and OpenAPI bounds count Unicode characters; invalid generic bounds fail before conversion. |
| `bounded_types/src/bounded_hash_map.rs` | Reviewed: capped construction and insertion, replacement at capacity, immutable keys, removal, serialization and visitor delegation; no raw mutable map access escapes. |
| `bounded_types/src/bounded_hash_map_visitor_phantom_data.rs` | Reviewed: bounded entry count including duplicate keys, stop before decoding overflow values, bounded HashMap preallocation and preserved deserialization errors. |
| `bounded_types/src/bounded_len.rs` | Reviewed: typed copyable length and generated conversion, getter and display forwarding. |
| `bounded_types/src/bounded_string.rs` | Reviewed: validated storage, bounded mutation, truncation and serde behavior delegate to the shared core; OpenAPI and Schemars schemas now distinguish character and byte bounds under A54. |
| `bounded_types/src/bounded_string_error.rs` | Reviewed: typed errors retain actual and configured lengths without displaying string contents. |
| `bounded_types/src/bounded_value_error.rs` | Reviewed: typed length errors retain configured bounds and actual lengths without displaying input values. |
| `bounded_types/src/bounded_vec.rs` | Reviewed: min/max validation, fallible bounded growth, immutable slice dereference, capped size-hint allocation, schema bounds, and item schema registration; A135. Schema composition behavior for externally overridden generic arguments was not established. |
| `bounded_types/src/bounded_vec_visitor_phantom_data.rs` | Reviewed: invalid bounds fail before allocation, capped size-hint reservation, incremental maximum enforcement and final minimum validation. |
| `bounded_types/src/collection_max_len.rs` | Reviewed: fixed 10,000-item collection limit; callers choose this policy explicitly. |
| `bounded_types/src/deserialize_bounded_map.rs` | Reviewed: bounded entry count including duplicate keys, stop before decoding overflow values, bounded HashMap preallocation and preserved deserialization errors. |
| `bounded_types/src/lib.rs` | Reviewed: all source modules are declared at the root, with the test module gated by cfg(test). |
| `bounded_types/src/serde_prealloc_max_items.rs` | Reviewed: fixed 1,024-item reservation cap; it limits initial capacity rather than final collection length. |
| `bounded_types/src/test_bounded_types.rs` | Reviewed: deterministic tests cover bounded text, vectors, maps, deserialization limits and both schema systems, including A54 regressions. |
| `bounded_types/src/try_from_bounded_error_text.rs` | Reviewed: A05 forwarding contract propagates Result unchanged; deterministic success and short-target tests. |
| `bounded_types/src/utoipa_schema_entries_mut.rs` | Reviewed: shared item schema registration and recursive forwarding; A135. |
| `bounded_types/src/utoipa_schema_type_name.rs` | Reviewed: shared component-safe type-name encoding; A138. |
| `bounded_types/src/validate_len.rs` | Reviewed: invalid bounds take precedence, then minimum and maximum checks, with typed lengths preserved in each failure. |

### common_routes

| Source | Semantic review |
| --- | --- |
| `common_routes/src/arc_common_routes_app_state.rs` | Reviewed: shared state ownership, immutable provider access, structural Debug that omits the provider state, immediate state extraction and sized/trait-object Arc construction; Send/Sync comes from explicit provider bounds. |
| `common_routes/src/axum_common_routes.rs` | Reviewed: thin Axum router ownership wrapper; generated transfers preserve router state and clone semantics. |
| `common_routes/src/axum_health_check_status.rs` | Reviewed: status wrapper forwards HTTP status and OK detection. |
| `common_routes/src/axum_http_uri.rs` | Reviewed: extractor copies the request URI into a typed wrapper. |
| `common_routes/src/axum_http_uri_ref.rs` | Reviewed: test-only borrowed URI wrapper. |
| `common_routes/src/axum_json_payload.rs` | Reviewed: JSON response wrapper forwards Axum serialization. |
| `common_routes/src/common_no_body.rs` | Reviewed: empty route-body marker. |
| `common_routes/src/common_not_found_error.rs` | Reviewed: typed not-found payload remains HTTP 404; invalid commit-link configuration now produces the shared HTTP 500 API problem under A27. |
| `common_routes/src/common_route.rs` | Reviewed: route catalog includes all five operational and metadata routes. |
| `common_routes/src/common_route_registry.rs` | Reviewed: registry maps each typed route to its matching handler. |
| `common_routes/src/common_routes.rs` | Reviewed: typed-registry router and fallback URI/message construction; A27 forwards link failures into the distinct fallback error before constructing a not-found payload. |
| `common_routes/src/common_routes_open_api.rs` | Reviewed: OpenAPI wrapper forwards the generated registry document. |
| `common_routes/src/common_routes_parameters.rs` | Reviewed: explicit commit-link/database providers and Send/Sync bounds establish the shared state contract without hidden state. |
| `common_routes/src/database_is_ready.rs` | Reviewed: bounded database probe maps SQL failure or timeout to unavailable. |
| `common_routes/src/git_info.rs` | Reviewed: serialized commit contract stores a validated bounded GitCommitLinkCow; private construction transfers ownership and the test read compares borrowed text. |
| `common_routes/src/git_info_response.rs` | Reviewed: A27 preserves successful JSON payloads and maps typed link-building failures to the operation-owned HTTP error. |
| `common_routes/src/git_info_response_error.rs` | Reviewed: operation-owned thiserror enum retains the typed length failure as diagnostic context and returns the shared internal API problem without publishing that context. |
| `common_routes/src/git_info_route.rs` | Reviewed: public read-only typed route retains its request/response contract and intentionally declares Internal errors under A27; its exact snapshot entry was reviewed. |
| `common_routes/src/health.rs` | Reviewed: returns the readiness report or its typed degraded response. |
| `common_routes/src/health_check.rs` | Reviewed: probe success maps to OK and failure to typed unavailable error. |
| `common_routes/src/health_check_error.rs` | A56: unavailable error now matches the empty-body route contract. |
| `common_routes/src/health_check_route.rs` | Reviewed: typed route declares an empty-body 503 response. |
| `common_routes/src/health_check_succeeded.rs` | Reviewed: boolean wrapper preserves probe outcome. |
| `common_routes/src/health_component.rs` | Reviewed: component stores typed kind and status. |
| `common_routes/src/health_component_kind.rs` | Reviewed: serialized component names are stable snake case. |
| `common_routes/src/health_components.rs` | Reviewed: construction and deserialization enforce the shared two-component maximum. |
| `common_routes/src/health_components_error.rs` | Reviewed: oversize list has a typed error. |
| `common_routes/src/health_components_max_len.rs` | Reviewed: two-component limit matches the report construction. |
| `common_routes/src/health_database_available.rs` | Reviewed: availability wrapper exposes its boolean state. |
| `common_routes/src/health_error.rs` | A55: unavailable response now emits a HealthReport with HTTP 503. |
| `common_routes/src/health_live.rs` | Reviewed: infallible liveness handler emits an OK report. |
| `common_routes/src/health_live_route.rs` | Reviewed: liveness contract declares only an OK report. |
| `common_routes/src/health_probe_timeout.rs` | Reviewed: finite two-second probe deadline. |
| `common_routes/src/health_ready.rs` | Reviewed: readiness handler forwards the report or typed unavailable response. |
| `common_routes/src/health_ready_route.rs` | Reviewed: readiness contract declares OK and degraded report responses. |
| `common_routes/src/health_report.rs` | Reviewed: service and database components determine liveness and readiness status. |
| `common_routes/src/health_report_response.rs` | A55: preserves degraded report in typed error; unit regression covers serialization. |
| `common_routes/src/health_route.rs` | Reviewed: health contract declares OK and degraded report responses. |
| `common_routes/src/health_status.rs` | Reviewed: status enum uses stable snake-case serialization. |
| `common_routes/src/json_response.rs` | Reviewed: typed JSON payload ownership and borrowed access; response conversion delegates once with its required explicit trait bound. |
| `common_routes/src/lib.rs` | Reviewed: root declarations own production and test-only modules. |
| `common_routes/src/make_git_info_payload_tests.rs` | Reviewed: test helper constructs Git metadata payload. |
| `common_routes/src/make_json_response.rs` | Reviewed: response helper wraps a typed payload in Axum JSON. |
| `common_routes/src/make_no_route_message_for_suffix_tests.rs` | Reviewed: test helper prefixes the URI suffix once and validates bounded text. |
| `common_routes/src/make_no_route_message_tests.rs` | Reviewed: test helper delegates URI suffix formatting. |
| `common_routes/src/make_not_found_payload_tests.rs` | Reviewed: test helper carries URI and commit link into the not-found payload. |
| `common_routes/src/make_not_found_payload_with_message_tests.rs` | Reviewed: test helper constructs typed not-found fields. |
| `common_routes/src/map_health_check_status_tests.rs` | Reviewed: test-only mapper mirrors the probe status mapping. |
| `common_routes/src/no_route_message_capacity.rs` | Reviewed: test-only capacity wrapper. |
| `common_routes/src/not_found_payload.rs` | Reviewed: private fields serialize the validated not-found payload. |
| `common_routes/src/open_api_specification_path.rs` | Reviewed: static OpenAPI path wrapper. |
| `common_routes/src/readiness_report.rs` | Reviewed: bounded database probe determines the readiness report. |
| `common_routes/src/test_common_routes_tests.rs` | Reviewed: success and unavailable report mapping and HTTP response shape are covered. |
| `common_routes/src/test_tests_domain_types.rs` | Reviewed: tests cover Git metadata, not-found payload, URI suffix, route responses, and provider failures. |
| `common_routes/src/test_tests_domain_types_health.rs` | Reviewed: tests cover report state, component limit, schema, and serialization. |
| `common_routes/src/test_tests_domain_types_route_contract.rs` | Reviewed: tests cover route mapping, path spelling, and family coverage. |
| `common_routes/src/uri_suffix_ref.rs` | Reviewed: test-only borrowed URI suffix wrapper. |
| `common_routes/src/uri_suffix_tests.rs` | Reviewed: test helper retains path and query when present. |
| `common_routes/src/utoipa_common_routes_open_api_document.rs` | Reviewed: wrapper serializes the generated OpenAPI document. |

### config_lib

| Source | Semantic review |
| --- | --- |
| `config_lib/src/admin_access_token_ttl_seconds.rs` | Reviewed: conversion, bounds, and error propagation inspected; no additional defect confirmed. |
| `config_lib/src/admin_cookie_secure.rs` | Reviewed: configuration bounds, conversions, error propagation, or test coverage inspected; no additional defect confirmed. |
| `config_lib/src/admin_jwt_secret.rs` | Reviewed: conversion, bounds, and error propagation inspected; no additional defect confirmed. |
| `config_lib/src/admin_jwt_secret_max_count.rs` | Reviewed: configuration bounds, conversions, error propagation, or test coverage inspected; no additional defect confirmed. |
| `config_lib/src/admin_jwt_secret_min_len.rs` | Reviewed: configuration bounds, conversions, error propagation, or test coverage inspected; no additional defect confirmed. |
| `config_lib/src/admin_jwt_tests.rs` | Reviewed: conversion, bounds, and error propagation inspected; no additional defect confirmed. |
| `config_lib/src/admin_login_failure_limit.rs` | Reviewed: conversion, bounds, and error propagation inspected; no additional defect confirmed. |
| `config_lib/src/admin_password_hash_concurrency.rs` | Reviewed: configuration bounds, conversions, error propagation, or test coverage inspected; no additional defect confirmed. |
| `config_lib/src/admin_refresh_token_ttl_seconds.rs` | Reviewed: conversion, bounds, and error propagation inspected; no additional defect confirmed. |
| `config_lib/src/admin_session_limit.rs` | Reviewed: conversion, bounds, and error propagation inspected; no additional defect confirmed. |
| `config_lib/src/admin_sign_in_rate_limit.rs` | Reviewed: configuration bounds, conversions, error propagation, or test coverage inspected; no additional defect confirmed. |
| `config_lib/src/admin_swagger_enabled.rs` | Reviewed: configuration bounds, conversions, error propagation, or test coverage inspected; no additional defect confirmed. |
| `config_lib/src/admin_tests.rs` | Reviewed: configuration bounds, conversions, error propagation, or test coverage inspected; no additional defect confirmed. |
| `config_lib/src/admin_token_audience.rs` | Reviewed: conversion, bounds, and error propagation inspected; no additional defect confirmed. |
| `config_lib/src/admin_token_issuer.rs` | Reviewed: conversion, bounds, and error propagation inspected; no additional defect confirmed. |
| `config_lib/src/chrono_fixed_offset_error.rs` | Reviewed: configuration bounds, conversions, error propagation, or test coverage inspected; no additional defect confirmed. |
| `config_lib/src/chrono_timezone.rs` | Reviewed: conversion, bounds, and error propagation inspected; no additional defect confirmed. |
| `config_lib/src/config_example_validity.rs` | Reviewed: configuration bounds, conversions, error propagation, or test coverage inspected; no additional defect confirmed. |
| `config_lib/src/config_field_descriptor.rs` | Reviewed: configuration bounds, conversions, error propagation, or test coverage inspected; no additional defect confirmed. |
| `config_lib/src/config_field_example_ref.rs` | Reviewed: configuration bounds, conversions, error propagation, or test coverage inspected; no additional defect confirmed. |
| `config_lib/src/config_field_requirement.rs` | Reviewed: configuration bounds, conversions, error propagation, or test coverage inspected; no additional defect confirmed. |
| `config_lib/src/config_field_sensitivity.rs` | Reviewed: configuration bounds, conversions, error propagation, or test coverage inspected; no additional defect confirmed. |
| `config_lib/src/config_lib_string_wrapper_max_len.rs` | Reviewed: configuration bounds, conversions, error propagation, or test coverage inspected; no additional defect confirmed. |
| `config_lib/src/config_lib_string_wrapper_try_from_string_error.rs` | Reviewed: configuration bounds, conversions, error propagation, or test coverage inspected; no additional defect confirmed. |
| `config_lib/src/config_parse_int_error.rs` | Reviewed: configuration bounds, conversions, error propagation, or test coverage inspected; no additional defect confirmed. |
| `config_lib/src/config_rust_type_name.rs` | Reviewed: configuration bounds, conversions, error propagation, or test coverage inspected; no additional defect confirmed. |
| `config_lib/src/content_security_policy.rs` | Reviewed: conversion, bounds, and error propagation inspected; no additional defect confirmed. |
| `config_lib/src/content_security_policy_error.rs` | Reviewed: configuration bounds, conversions, error propagation, or test coverage inspected; no additional defect confirmed. |
| `config_lib/src/domain_types.rs` | Fixed: A59 rejects unknown tracing formats while preserving case-insensitive valid values. |
| `config_lib/src/env_parse_error.rs` | Reviewed: configuration bounds, conversions, error propagation, or test coverage inspected; no additional defect confirmed. |
| `config_lib/src/env_var_error.rs` | Reviewed: configuration bounds, conversions, error propagation, or test coverage inspected; no additional defect confirmed. |
| `config_lib/src/env_var_name.rs` | Reviewed: bounded storage, identical byte limits and error classification; A05 removes diagnostic retries. |
| `config_lib/src/env_var_name_ref.rs` | Reviewed: configuration bounds, conversions, error propagation, or test coverage inspected; no additional defect confirmed. |
| `config_lib/src/env_var_result_var_error.rs` | Reviewed: configuration bounds, conversions, error propagation, or test coverage inspected; no additional defect confirmed. |
| `config_lib/src/env_var_value_ref.rs` | Reviewed: configuration bounds, conversions, error propagation, or test coverage inspected; no additional defect confirmed. |
| `config_lib/src/http_gzip_enabled.rs` | Reviewed: configuration bounds, conversions, error propagation, or test coverage inspected; no additional defect confirmed. |
| `config_lib/src/http_tests.rs` | Reviewed: configuration bounds, conversions, error propagation, or test coverage inspected; no additional defect confirmed. |
| `config_lib/src/i32_parse_int_error.rs` | Reviewed: configuration bounds, conversions, error propagation, or test coverage inspected; no additional defect confirmed. |
| `config_lib/src/lib.rs` | Reviewed: configuration bounds, conversions, error propagation, or test coverage inspected; no additional defect confirmed. |
| `config_lib/src/maximum_size_of_http_body_in_bytes.rs` | Reviewed: conversion, bounds, and error propagation inspected; no additional defect confirmed. |
| `config_lib/src/maximum_size_of_http_body_in_bytes_try_from_usize_error.rs` | Reviewed: configuration bounds, conversions, error propagation, or test coverage inspected; no additional defect confirmed. |
| `config_lib/src/parse_admin_positive_u64.rs` | Reviewed: conversion, bounds, and error propagation inspected; no additional defect confirmed. |
| `config_lib/src/parse_admin_token_text.rs` | Reviewed: conversion, bounds, and error propagation inspected; no additional defect confirmed. |
| `config_lib/src/parse_bool_error.rs` | Reviewed: configuration bounds, conversions, error propagation, or test coverage inspected; no additional defect confirmed. |
| `config_lib/src/parse_context_ref.rs` | Reviewed: configuration bounds, conversions, error propagation, or test coverage inspected; no additional defect confirmed. |
| `config_lib/src/parse_east_fixed_offset.rs` | Reviewed: conversion, bounds, and error propagation inspected; no additional defect confirmed. |
| `config_lib/src/parse_env_var_name_ref.rs` | Reviewed: configuration bounds, conversions, error propagation, or test coverage inspected; no additional defect confirmed. |
| `config_lib/src/parse_from_env_var_from_str_tests.rs` | Reviewed: configuration bounds, conversions, error propagation, or test coverage inspected; no additional defect confirmed. |
| `config_lib/src/parse_from_env_var_with_tests.rs` | Reviewed: configuration bounds, conversions, error propagation, or test coverage inspected; no additional defect confirmed. |
| `config_lib/src/parse_from_str_with_context_tests.rs` | Reviewed: configuration bounds, conversions, error propagation, or test coverage inspected; no additional defect confirmed. |
| `config_lib/src/parse_from_str_with_error.rs` | Reviewed: configuration bounds, conversions, error propagation, or test coverage inspected; no additional defect confirmed. |
| `config_lib/src/parse_pg_pool_non_zero_seconds.rs` | Reviewed: conversion, bounds, and error propagation inspected; no additional defect confirmed. |
| `config_lib/src/parse_required_env_var.rs` | Fixed and verified: A58 rejects oversized values before parsing. |
| `config_lib/src/parse_required_env_var_value.rs` | Fixed and verified: A58 maps bounded-value conversion errors before parsing; directly covered by deterministic regression. |
| `config_lib/src/pg_pool_acquire_timeout_seconds.rs` | Reviewed: configuration bounds, conversions, error propagation, or test coverage inspected; no additional defect confirmed. |
| `config_lib/src/pg_pool_config_parse_error.rs` | Reviewed: conversion, bounds, and error propagation inspected; no additional defect confirmed. |
| `config_lib/src/pg_pool_idle_timeout_seconds.rs` | Reviewed: configuration bounds, conversions, error propagation, or test coverage inspected; no additional defect confirmed. |
| `config_lib/src/pg_pool_max_connections.rs` | Reviewed: configuration bounds, conversions, error propagation, or test coverage inspected; no additional defect confirmed. |
| `config_lib/src/pg_pool_max_connections_try_from_u32_error.rs` | Reviewed: configuration bounds, conversions, error propagation, or test coverage inspected; no additional defect confirmed. |
| `config_lib/src/pg_pool_max_lifetime_seconds.rs` | Reviewed: configuration bounds, conversions, error propagation, or test coverage inspected; no additional defect confirmed. |
| `config_lib/src/pg_pool_min_connections.rs` | Reviewed: configuration bounds, conversions, error propagation, or test coverage inspected; no additional defect confirmed. |
| `config_lib/src/pg_pool_tests.rs` | Reviewed: configuration bounds, conversions, error propagation, or test coverage inspected; no additional defect confirmed. |
| `config_lib/src/production_mode.rs` | Reviewed: configuration bounds, conversions, error propagation, or test coverage inspected; no additional defect confirmed. |
| `config_lib/src/request_timeout_seconds.rs` | Reviewed: conversion, bounds, and error propagation inspected; no additional defect confirmed. |
| `config_lib/src/secrecy_secret_box_string.rs` | Reviewed: configuration bounds, conversions, error propagation, or test coverage inspected; no additional defect confirmed. |
| `config_lib/src/source_place_type.rs` | Reviewed: configuration bounds, conversions, error propagation, or test coverage inspected; no additional defect confirmed. |
| `config_lib/src/std_config_secret_string.rs` | Reviewed: configuration bounds, conversions, error propagation, or test coverage inspected; no additional defect confirmed. |
| `config_lib/src/std_env_var_ok.rs` | Reviewed: bounded storage, owned transfer, byte-limit validation and source classification; A05. |
| `config_lib/src/std_env_var_ok_ref.rs` | Reviewed: configuration bounds, conversions, error propagation, or test coverage inspected; no additional defect confirmed. |
| `config_lib/src/svc_mode.rs` | Reviewed: configuration bounds, conversions, error propagation, or test coverage inspected; no additional defect confirmed. |
| `config_lib/src/test_config_lib.rs` | Reviewed: domain parser, bounds, timezone, and A58 regression cases inspected. |
| `config_lib/src/timezone_seconds.rs` | Reviewed: conversion, bounds, and error propagation inspected; no additional defect confirmed. |
| `config_lib/src/tracing_format.rs` | Fixed: A59 rejects unknown tracing formats while preserving case-insensitive valid values. |
| `config_lib/src/tracing_level.rs` | Reviewed: configuration bounds, conversions, error propagation, or test coverage inspected; no additional defect confirmed. |
| `config_lib/src/tracing_level_name.rs` | Reviewed: configuration bounds, conversions, error propagation, or test coverage inspected; no additional defect confirmed. |
| `config_lib/src/try_from_std_env_var_ok.rs` | Reviewed: configuration bounds, conversions, error propagation, or test coverage inspected; no additional defect confirmed. |
| `config_lib/src/try_from_std_env_var_ok_admin_jwt_secret_error.rs` | Reviewed: configuration bounds, conversions, error propagation, or test coverage inspected; no additional defect confirmed. |
| `config_lib/src/try_from_std_env_var_ok_admin_password_hash_concurrency_error.rs` | Reviewed: configuration bounds, conversions, error propagation, or test coverage inspected; no additional defect confirmed. |
| `config_lib/src/try_from_std_env_var_ok_admin_positive_u64_error.rs` | Reviewed: configuration bounds, conversions, error propagation, or test coverage inspected; no additional defect confirmed. |
| `config_lib/src/try_from_std_env_var_ok_admin_token_text_error.rs` | Reviewed: configuration bounds, conversions, error propagation, or test coverage inspected; no additional defect confirmed. |
| `config_lib/src/try_from_std_env_var_ok_maximum_size_of_http_body_in_bytes_error.rs` | Reviewed: configuration bounds, conversions, error propagation, or test coverage inspected; no additional defect confirmed. |
| `config_lib/src/try_from_std_env_var_ok_pg_pool_max_connections_error.rs` | Reviewed: configuration bounds, conversions, error propagation, or test coverage inspected; no additional defect confirmed. |
| `config_lib/src/try_from_std_env_var_ok_svc_mode_error.rs` | Reviewed: configuration bounds, conversions, error propagation, or test coverage inspected; no additional defect confirmed. |
| `config_lib/src/try_from_std_env_var_ok_timezone_error.rs` | Reviewed: configuration bounds, conversions, error propagation, or test coverage inspected; no additional defect confirmed. |
| `config_lib/src/try_from_std_env_var_ok_tracing_format_error.rs` | Fixed: A59 gives unknown tracing formats a typed error. |
| `config_lib/src/types_tests.rs` | Focused review: environment wrappers, enum conversions, and A59 regression inspected. |
| `config_lib/src/u32_parse_int_error.rs` | Reviewed: configuration bounds, conversions, error propagation, or test coverage inspected; no additional defect confirmed. |
| `config_lib/src/usize_parse_int_error.rs` | Reviewed: configuration bounds, conversions, error propagation, or test coverage inspected; no additional defect confirmed. |

### constants_i32

| Source | Semantic review |
| --- | --- |
| `constants_i32/src/lib.rs` | Reviewed: literal values match names and deterministic assertions; no arithmetic or state. |

### constants_i64

| Source | Semantic review |
| --- | --- |
| `constants_i64/src/lib.rs` | Reviewed: literal values match names and deterministic assertions; no arithmetic or state. |

### constants_str

| Source | Semantic review |
| --- | --- |
| `constants_str/src/lib.rs` | Focused review; A94, A96, A102 |

### constants_u128

| Source | Semantic review |
| --- | --- |
| `constants_u128/src/lib.rs` | Reviewed: literal values match names and deterministic assertions; no arithmetic or state. |

### constants_u16

| Source | Semantic review |
| --- | --- |
| `constants_u16/src/lib.rs` | Reviewed: literal values match names and deterministic assertions; no arithmetic or state. |

### constants_u32

| Source | Semantic review |
| --- | --- |
| `constants_u32/src/lib.rs` | Reviewed: literal values match names and deterministic assertions; no arithmetic or state. |

### constants_u64

| Source | Semantic review |
| --- | --- |
| `constants_u64/src/lib.rs` | Reviewed: literal values match names and deterministic assertions; no arithmetic or state. |

### constants_u8

| Source | Semantic review |
| --- | --- |
| `constants_u8/src/lib.rs` | Reviewed: literal values match names and deterministic assertions; no arithmetic or state. |

### constants_usize

| Source | Semantic review |
| --- | --- |
| `constants_usize/src/lib.rs` | Reviewed: literal values match names and deterministic assertions; no arithmetic or state. |

### dev_identity_creation_planner

| Source | Semantic review |
| --- | --- |
| `dev_identity_creation_planner/src/development_identity_count.rs` | Reviewed: collection limit, typed construction, exhaustive decisions and saturating counters; deterministic tests inspected. |
| `dev_identity_creation_planner/src/development_identity_creation_plan.rs` | Reviewed: collection limit, typed construction, exhaustive decisions and saturating counters; deterministic tests inspected. |
| `dev_identity_creation_planner/src/development_identity_creation_summary.rs` | Reviewed: collection limit, typed construction, exhaustive decisions and saturating counters; deterministic tests inspected. |
| `dev_identity_creation_planner/src/development_identity_specs.rs` | Reviewed: collection limit, typed construction, exhaustive decisions and saturating counters; deterministic tests inspected. |
| `dev_identity_creation_planner/src/development_identity_specs_error.rs` | Reviewed: collection limit, typed construction, exhaustive decisions and saturating counters; deterministic tests inspected. |
| `dev_identity_creation_planner/src/development_identity_specs_max_len.rs` | Reviewed: collection limit, typed construction, exhaustive decisions and saturating counters; deterministic tests inspected. |
| `dev_identity_creation_planner/src/lib.rs` | Reviewed: collection limit, typed construction, exhaustive decisions and saturating counters; deterministic tests inspected. |

### external_service_emulators

| Source | Semantic review |
| --- | --- |
| `external_service_emulators/src/create_mock_notification_provider.rs` | Reviewed: channel halves are returned as a paired provider and inbox without shared global state. |
| `external_service_emulators/src/lib.rs` | Reviewed: root owns all emulator modules and gates the test module. |
| `external_service_emulators/src/mock_notification_inbox.rs` | Reviewed: receive delegates to the owned channel receiver and retains closure as None. |
| `external_service_emulators/src/mock_notification_provider.rs` | Reviewed: the notification sender delegates to the channel and maps a closed receiver to its typed error. |
| `external_service_emulators/src/mock_notification_provider_closed.rs` | Reviewed: closed-channel outcome is a typed, nonsensitive error. |
| `external_service_emulators/src/remote_sync_request_count.rs` | Reviewed: request count is a copyable typed wrapper with saturating increment. |
| `external_service_emulators/src/remote_sync_source.rs` | Reviewed: each read invocation increments its request count and returns a clone of the same validated payload. |
| `external_service_emulators/src/test_external_service_emulators.rs` | Reviewed: deterministic tests cover synchronization payload/count and in-memory notification delivery. |
| `external_service_emulators/src/tokio_mock_notification_receiver.rs` | Reviewed: private receiver delegates async receive to Tokio's channel. |
| `external_service_emulators/src/tokio_mock_notification_sender.rs` | Reviewed: private sender delegates send and retains the channel error for its parent adapter. |

### file_storage

| Source | Semantic review |
| --- | --- |
| `file_storage/src/atomic_replace_durability.rs` | Reviewed: durability enum selects flush or full sync. |
| `file_storage/src/disk_cache_budget_error.rs` | Reviewed: typed errors distinguish oversized incoming entries and arithmetic overflow. |
| `file_storage/src/disk_cache_entry.rs` | Reviewed: cache entry stores typed path, size, and modification time. |
| `file_storage/src/disk_cache_eviction_plan.rs` | Reviewed: eviction plan retains ordered typed paths. |
| `file_storage/src/disk_cache_modified_at_system_time.rs` | Reviewed: typed modification-time wrapper. |
| `file_storage/src/domain_types.rs` | Reviewed: file, operation-ID, and path limits are shared by their wrappers. |
| `file_storage/src/file_storage_error.rs` | Fixed and reviewed; A57 combines operation and cleanup errors and keeps the operation in the source chain. |
| `file_storage/src/file_storage_io_error.rs` | Reviewed: I/O error wrapper retains its source. |
| `file_storage/src/file_storage_path_error.rs` | Reviewed: path, ID, and file-size failures have distinct variants. |
| `file_storage/src/file_storage_root_path_buf.rs` | Reviewed: root path conversion requires an absolute bounded path. |
| `file_storage/src/file_storage_staging_area.rs` | Reviewed: upload and delete areas map to their dedicated directory names. |
| `file_storage/src/lib.rs` | Reviewed: root declares storage domain, operations, and tests. |
| `file_storage/src/plan_disk_cache_eviction.rs` | Reviewed: A50 prevents projected-size overflow; oldest entries are selected until incoming data fits. |
| `file_storage/src/safe_file_storage.rs` | Fixed and reviewed; A57 preserves both operation and cleanup failures in three staging paths. |
| `file_storage/src/stale_before_system_time.rs` | Reviewed: typed stale threshold wrapper. |
| `file_storage/src/stale_staging_cleanup_configuration.rs` | Reviewed: cleanup configuration carries threshold and bounded scan/removal limits. |
| `file_storage/src/stale_staging_cleanup_configuration_error.rs` | Reviewed: invalid cleanup limit has a typed error. |
| `file_storage/src/stale_staging_cleanup_report.rs` | Reviewed: report increments scanned and removed counts without overflow. |
| `file_storage/src/std_disk_cache_size.rs` | Reviewed: typed cache-size wrapper. |
| `file_storage/src/std_file_bytes.rs` | Reviewed: file payload conversion enforces the 100 MiB limit. |
| `file_storage/src/std_stale_staging_entry_count.rs` | Reviewed: count wrapper increments with saturation. |
| `file_storage/src/std_stale_staging_entry_limit.rs` | Reviewed: cleanup limit conversion requires 1 through 10,000. |
| `file_storage/src/std_storage_operation_id.rs` | Reviewed: operation IDs are bounded URL-safe tokens. |
| `file_storage/src/storage_directory_name_ref.rs` | Reviewed: borrowed staging directory-name wrapper. |
| `file_storage/src/storage_path_ref.rs` | Reviewed: borrowed storage path wrapper. |
| `file_storage/src/storage_relative_path_buf.rs` | Reviewed: relative paths reject traversal, absolute paths, NUL, and owned staging roots. |
| `file_storage/src/test_adapters.rs` | Reviewed: test covers creation of both owned staging directories. |
| `file_storage/src/test_file_storage.rs` | Focused review: lifecycle, path safety, cleanup, and A50 planner tests inspected; A57 has deterministic error-combination tests in the error owner. |

### frontend_admin

| Source | Semantic review |
| --- | --- |
| `frontend_admin/src/admin_alert.rs` | Reviewed |
| `frontend_admin/src/admin_alert_dialog.rs` | Reviewed |
| `frontend_admin/src/admin_alert_variant.rs` | Reviewed |
| `frontend_admin/src/admin_api_url.rs` | Reviewed |
| `frontend_admin/src/admin_api_url_with_suffix.rs` | Reviewed |
| `frontend_admin/src/admin_app.rs` | Focused review: CSR page selection, detail dispatch, identifier filters, fetch branches, and error/loading rendering inspected; no confirmed route mismatch. |
| `frontend_admin/src/admin_assets_error.rs` | Reviewed |
| `frontend_admin/src/admin_badge.rs` | Reviewed: variant class and children forward to Badge. |
| `frontend_admin/src/admin_badge_variant.rs` | Reviewed |
| `frontend_admin/src/admin_branding_details.rs` | Reviewed |
| `frontend_admin/src/admin_button.rs` | Reviewed: button kind, state, attributes, children, and optional click callback forward to Button. |
| `frontend_admin/src/admin_button_kind.rs` | Reviewed |
| `frontend_admin/src/admin_button_link.rs` | Reviewed |
| `frontend_admin/src/admin_button_variant.rs` | Reviewed |
| `frontend_admin/src/admin_card.rs` | Reviewed: variant class and children forward through CardContent. |
| `frontend_admin/src/admin_card_description.rs` | Reviewed: SSR-only card description forwards children. |
| `frontend_admin/src/admin_card_header.rs` | Reviewed: card header forwards children. |
| `frontend_admin/src/admin_card_title.rs` | Reviewed: optional class and children forward to CardTitle. |
| `frontend_admin/src/admin_card_variant.rs` | Reviewed |
| `frontend_admin/src/admin_checkbox.rs` | Reviewed |
| `frontend_admin/src/admin_column_filter.rs` | Focused review; A92, A95 |
| `frontend_admin/src/admin_csr_api_url.rs` | Reviewed |
| `frontend_admin/src/admin_csr_api_url_suffix_ref.rs` | Reviewed |
| `frontend_admin/src/admin_csr_query.rs` | Focused review; A90 |
| `frontend_admin/src/admin_csrf_token.rs` | Reviewed |
| `frontend_admin/src/admin_data_grid.rs` | Focused review; A92, A93 |
| `frontend_admin/src/admin_data_grid_input_type.rs` | Reviewed |
| `frontend_admin/src/admin_data_table_grid.rs` | Focused review; A92, A95, A110 system-role update action. |
| `frontend_admin/src/admin_empty.rs` | Reviewed: empty-state title forwards children. |
| `frontend_admin/src/admin_field.rs` | Reviewed: label and children render under one FieldLabel. |
| `frontend_admin/src/admin_field_label.rs` | Reviewed: owned labels support the Leptos prop adapter; the code-style analyzer explicitly exempts this framework boundary, and the markup regression covers owned String labels. |
| `frontend_admin/src/admin_filter_hidden_inputs.rs` | Reviewed |
| `frontend_admin/src/admin_frontend_routes.rs` | Reviewed |
| `frontend_admin/src/admin_health_probe.rs` | Reviewed |
| `frontend_admin/src/admin_health_view.rs` | Reviewed |
| `frontend_admin/src/admin_health_wasm_bindgen_error.rs` | Reviewed |
| `frontend_admin/src/admin_http_status.rs` | Reviewed |
| `frontend_admin/src/admin_identifier_filter_query.rs` | Reviewed |
| `frontend_admin/src/admin_input.rs` | Reviewed: bound and unbound branches forward name, type, constraints, initial value, and state. |
| `frontend_admin/src/admin_input_group.rs` | Reviewed: input group forwards children under an owner. |
| `frontend_admin/src/admin_input_kind.rs` | Reviewed |
| `frontend_admin/src/admin_input_name.rs` | Reviewed |
| `frontend_admin/src/admin_joined_text.rs` | Reviewed: inclusive 16 MiB byte limit, checked subtraction and bounded diagnostic conversion; A05. |
| `frontend_admin/src/admin_joined_text_try_from_string_error.rs` | Reviewed |
| `frontend_admin/src/admin_load_state.rs` | Reviewed |
| `frontend_admin/src/admin_mutation_method.rs` | Reviewed |
| `frontend_admin/src/admin_navigation_link.rs` | Reviewed |
| `frontend_admin/src/admin_page_nav_disabled.rs` | Reviewed |
| `frontend_admin/src/admin_page_range.rs` | Reviewed; A91 |
| `frontend_admin/src/admin_password_generation_error.rs` | Reviewed: browser, randomness, and policy failures retain distinct typed variants. |
| `frontend_admin/src/admin_profile_view.rs` | Reviewed: password validation, browser entropy generation, submit behavior, and account rendering inspected; existing browser coverage checks generation and input visibility. |
| `frontend_admin/src/admin_read_action.rs` | Reviewed: read trigger targets its matching dialog and forwards row details. |
| `frontend_admin/src/admin_record_read_page.rs` | Reviewed |
| `frontend_admin/src/admin_record_view.rs` | Reviewed: missing-record state, first-row field rendering, and caller page selection inspected. |
| `frontend_admin/src/admin_role_update_action.rs` | Reviewed: action links to the typed role update page. |
| `frontend_admin/src/admin_role_view.rs` | Reviewed: typed role fields render in fixed label order with an empty fallback. |
| `frontend_admin/src/admin_roles_table_view.rs` | Reviewed: typed role page maps five columns and row values in matching order. |
| `frontend_admin/src/admin_route_path_url.rs` | Reviewed |
| `frontend_admin/src/admin_rule_view.rs` | Reviewed: delegates to the generic record view with the rule page marker. |
| `frontend_admin/src/admin_rules_view.rs` | Reviewed: delegates rule table state to the shared data grid. |
| `frontend_admin/src/admin_sessions_table_view.rs` | Reviewed: typed session page maps four columns and row values in matching order. |
| `frontend_admin/src/admin_setting_disabled.rs` | Reviewed: boolean wrapper forwards the edit permission state. |
| `frontend_admin/src/admin_setting_input_value.rs` | Reviewed: owned form text bridges settings values and reactive signals. |
| `frontend_admin/src/admin_setting_inputs.rs` | Reviewed: all catalog settings map to matching text, URL, or text-area controls with required and disabled state. |
| `frontend_admin/src/admin_setting_required.rs` | Reviewed: boolean wrapper forwards catalog required state. |
| `frontend_admin/src/admin_settings_form_signals.rs` | Reviewed: one signal per catalog setting with total indexed access. |
| `frontend_admin/src/admin_settings_form_values.rs` | Reviewed: all catalog settings map from the view; absent optional values become empty form text. |
| `frontend_admin/src/admin_settings_view.rs` | Focused review; A94 |
| `frontend_admin/src/admin_sidebar.rs` | Reviewed: the navigation component renders its supplied items in a labelled menu with a checkbox toggle; `target/audit_tmp/a109_navigation_review.log` passes. |
| `frontend_admin/src/admin_sidebar_item.rs` | Reviewed: wraps one supplied navigation child in a list item. |
| `frontend_admin/src/admin_spinner.rs` | Reviewed: the loading indicator exposes status and loading text to assistive technology. |
| `frontend_admin/src/admin_ssr_error_message.rs` | Reviewed: validated error text wrapper preserves bounded conversion. |
| `frontend_admin/src/admin_ssr_html.rs` | Reviewed: inclusive 16 MiB byte limit, owned transfer and bounded diagnostic conversion; A05. |
| `frontend_admin/src/admin_ssr_html_try_from_string_error.rs` | Reviewed: typed HTML size-limit error. |
| `frontend_admin/src/admin_ssr_text.rs` | Reviewed: inclusive 16 MiB byte limit and bounded storage; A05 diagnostic conversion. |
| `frontend_admin/src/admin_ssr_text_try_from_string_error.rs` | Reviewed: typed SSR text size-limit error. |
| `frontend_admin/src/admin_ssr_view_ext_tests.rs` | Reviewed: test-only render adapter inspected. |
| `frontend_admin/src/admin_table_action_trigger.rs` | Reviewed: read-dialog trigger forwards label, target, and children. |
| `frontend_admin/src/admin_table_actions.rs` | Fixed and reviewed; A103 session revoke action label. |
| `frontend_admin/src/admin_table_load_error.rs` | Reviewed: table failures retain typed sources; only missing CSRF and HTTP 401 request session refresh. |
| `frontend_admin/src/admin_table_query.rs` | Reviewed |
| `frontend_admin/src/admin_table_query_direction.rs` | Reviewed: target-specific direction variants match CSR and SSR callers. |
| `frontend_admin/src/admin_table_query_hidden_inputs.rs` | Reviewed: both target variants preserve search, sort, and direction in GET forms. |
| `frontend_admin/src/admin_textarea.rs` | Reviewed: bound and unbound branches forward name, required, disabled, and signal value; SSR signal rendering has a regression. |
| `frontend_admin/src/admin_user_roles.rs` | Reviewed: assigned role names render in catalog order; existing focused test covers missing and matching roles. |
| `frontend_admin/src/admin_user_update_action.rs` | Reviewed: action links to the typed user update page. |
| `frontend_admin/src/axum_admin_frontend_router.rs` | Reviewed: typed router ownership wrapper only. |
| `frontend_admin/src/crud_render_shell.rs` | Reviewed: renders CRUD content with authenticated shell and Users/Roles active-table context. |
| `frontend_admin/src/csr_admin_nav.rs` | Reviewed: permission-gated table and page navigation plus sign-out action. |
| `frontend_admin/src/data_table_grid.rs` | Focused review; A92, A95 |
| `frontend_admin/src/domain_types_ssr_tests.rs` | Reviewed: reusable authenticated-admin and branding fixtures inspected. |
| `frontend_admin/src/fetch_access_sessions_read.rs` | Focused review; A105 shared timestamp filter fix. Detail-path query parameters are ignored by `AdminCsrQuery`; the access-session detail browser regression passed without a source change. |
| `frontend_admin/src/fetch_account_read.rs` | Reviewed: generic read adapter constructs its typed request from the table query and forwards it. |
| `frontend_admin/src/fetch_account_read_request.rs` | Reviewed: serializes the typed request and route contract into a transport request. |
| `frontend_admin/src/fetch_audit_log_read.rs` | Focused review; A105 shared timestamp filter fix. |
| `frontend_admin/src/fetch_json.rs` | Reviewed: delegates GET JSON reads to the shared request fetcher. |
| `frontend_admin/src/fetch_json_request.rs` | Reviewed: constructs browser requests, checks HTTP status, decodes JSON, and runs within the session refresh wrapper. |
| `frontend_admin/src/fetch_permission_table_read.rs` | Fixed and reviewed; A104 permission list filters. |
| `frontend_admin/src/fetch_role_rules_read.rs` | Focused review; A105 shared timestamp filter fix. |
| `frontend_admin/src/fetch_roles_read.rs` | Reviewed: detail requests use an identifier filter and default pagination; list requests preserve table query and supported column kinds. |
| `frontend_admin/src/fetch_rules_read.rs` | Fixed and reviewed; A105 timestamp filter and A106 unknown field rejection. |
| `frontend_admin/src/fetch_system_settings_read.rs` | Fixed and reviewed; A105 shared timestamp filter and A107 unknown field rejection. |
| `frontend_admin/src/fetch_user_roles_read.rs` | Reviewed: detail requests use an identifier filter and default pagination; list requests preserve the typed table and filter queries. |
| `frontend_admin/src/fetch_users_read.rs` | Focused review; A105 shared timestamp filter fix. |
| `frontend_admin/src/join_text.rs` | Reviewed: joins borrowed role and rule labels in order, handles empty input, and has a deterministic unit test for both cases. The bounded result conversion follows the existing diagnostic fallback policy. |
| `frontend_admin/src/leptos_admin_filter_operation_signal.rs` | Reviewed: signal wrapper conversion and ownership inspected. |
| `frontend_admin/src/leptos_admin_input_signal.rs` | Reviewed: copied signal handle exposes reactive value and converts current text for CSR settings validation. |
| `frontend_admin/src/lib.rs` | Reviewed: module ownership, target gates, and bounded-string signature anchor inspected. |
| `frontend_admin/src/page_render_with_access.rs` | Reviewed: forwards page rendering to the shared table-aware shell. |
| `frontend_admin/src/page_render_with_table_access.rs` | Reviewed: composes the document title, branding, permission-gated navigation, and page content. |
| `frontend_admin/src/reload_after.rs` | Reviewed: sends a mutation under the session wrapper, reloads after success, and displays a failure when the request or reload fails. The browser-only path still needs provisioned browser coverage. |
| `frontend_admin/src/render_admin_csr.rs` | Reviewed: renders the loading root and CSR boot script with branding and password-change state. |
| `frontend_admin/src/render_admin_page.rs` | Reviewed: forwards the page and rendered content to the shared shell without access context. |
| `frontend_admin/src/render_admin_page_with_access.rs` | Reviewed: forwards to the shared page shell with access context. |
| `frontend_admin/src/render_admin_page_with_table_access.rs` | Reviewed: forwards to the shared page shell with table context. |
| `frontend_admin/src/render_admin_profile_page.rs` | Reviewed: renders validated administrator profile fields and roles in the shared shell. |
| `frontend_admin/src/render_admin_rules_page.rs` | Reviewed: displays typed rule rows and table pagination through the shared SSR shell; static page test passes (`target/audit_tmp/a109_static_review.log`). |
| `frontend_admin/src/render_admin_sessions_page.rs` | Reviewed: session rows use typed detail links and permission-gated revoke controls with per-row dialog and form identifiers; static page test passes (`target/audit_tmp/a109_static_review.log`). |
| `frontend_admin/src/render_admin_settings_page.rs` | Reviewed: SSR renders edit form only with update permission and uses the shared setting catalog controls. |
| `frontend_admin/src/render_data_tables.rs` | Reviewed: renders an optional typed table view with its query and pagination in the shared table-aware shell. |
| `frontend_admin/src/render_data_tables_csr.rs` | Reviewed: forwards the table selection and authenticated context to the CSR loading shell. |
| `frontend_admin/src/render_document.rs` | Fixed and reviewed; A108 escapes the configured tab title before embedding it in HTML. |
| `frontend_admin/src/render_role_create.rs` | Reviewed: the role creation form uses the typed HTML action and required role name; CRUD rendering test passes (`target/audit_tmp/a110_crud_review.log`). |
| `frontend_admin/src/render_role_manage.rs` | Reviewed: populated role forms require confirmation for deletion and disable update and deletion for system roles; CRUD rendering test passes (`target/audit_tmp/a110_crud_review.log`). |
| `frontend_admin/src/render_role_update.rs` | Reviewed: the SSR role-update form posts to the typed action and requires the role identifier and name. |
| `frontend_admin/src/render_roles.rs` | Fixed and reviewed; A110 hides the Update action for system roles, with an SSR regression in `test_static_pages_tests.rs`. |
| `frontend_admin/src/render_sign_in.rs` | Reviewed: the sign-in form uses the typed action, Leptos text rendering, and a validated hex primary color; document and navigation regressions pass (`target/audit_tmp/a109_ssr_review.log`, `target/audit_tmp/a109_navigation_review.log`). |
| `frontend_admin/src/render_text_page.rs` | Reviewed: renders escaped text through Leptos and the shared page shell; the page catalog supplies the displayed title. |
| `frontend_admin/src/render_text_page_with_access.rs` | Reviewed: renders the version value or escaped code text through the shared access-aware page shell. |
| `frontend_admin/src/render_user_create.rs` | Reviewed: the user creation form uses the typed HTML action and required account fields; CRUD rendering test passes (`target/audit_tmp/a110_crud_review.log`). |
| `frontend_admin/src/render_user_manage.rs` | Reviewed: populated user forms expose update and delete only with the corresponding rules and require deletion confirmation; CRUD rendering test passes (`target/audit_tmp/a110_crud_review.log`). |
| `frontend_admin/src/render_user_update.rs` | Reviewed: the SSR user-update form posts to the typed action and requires the user identifier, display name, and login. |
| `frontend_admin/src/render_users.rs` | Reviewed: the SSR user list gates create and update controls by rules, escapes row values through Leptos, and passes its typed query and total to pagination. |
| `frontend_admin/src/render_view.rs` | Reviewed: renders an owned Leptos view into bounded HTML; A108 regression also exercises escaped text rendering. |
| `frontend_admin/src/send_admin_request.rs` | Reviewed: serializes a mutation body, sets JSON and CSRF headers, checks the fetch response, and classifies failure through the admin load error. The browser-only path still needs provisioned browser coverage. |
| `frontend_admin/src/show_mutation_error.rs` | Reviewed: creates a text-only alert and falls back to root text on append failure, avoiding HTML interpretation of errors. The browser-only path still needs provisioned browser coverage. |
| `frontend_admin/src/start.rs` | Reviewed: browser root lookup, mount, and early returns inspected. |
| `frontend_admin/src/std_rc_serde_json_error.rs` | Reviewed: error source conversion inspected. |
| `frontend_admin/src/std_str_utf8_error.rs` | Reviewed: UTF-8 error forwarding wrapper inspected. |
| `frontend_admin/src/table.rs` | Reviewed: table element and attributes inspected. |
| `frontend_admin/src/table_body.rs` | Reviewed: table body element and attributes inspected. |
| `frontend_admin/src/table_caption.rs` | Reviewed: table caption element and attributes inspected. |
| `frontend_admin/src/table_cell.rs` | Reviewed: value preview, action cell bypass, and browser viewer behavior inspected; existing render and browser tests cover both modes. |
| `frontend_admin/src/table_footer.rs` | Reviewed: table footer element and attributes inspected. |
| `frontend_admin/src/table_head.rs` | Reviewed: header data attributes and children inspected. |
| `frontend_admin/src/table_header.rs` | Reviewed: table header element and attributes inspected. |
| `frontend_admin/src/table_pagination.rs` | Reviewed; A91 consumer |
| `frontend_admin/src/table_row.rs` | Reviewed: table row element and attributes inspected. |
| `frontend_admin/src/table_wrapper.rs` | Reviewed: scroll wrapper element and attributes inspected. |
| `frontend_admin/src/test_admin_ssr_html.rs` | Reviewed: exact byte bound, overflow, Unicode bytes, and fallback tests inspected. |
| `frontend_admin/src/test_render_document.rs` | A108 title escaping regression passed. |
| `frontend_admin/src/test_crud_tests.rs` | Reviewed: user and role CRUD render assertions and fixture setup inspected. |
| `frontend_admin/src/test_data_grid_tests.rs` | Focused review; A95 |
| `frontend_admin/src/test_domain_types_ssr_tests_document.rs` | Reviewed: CSR mount, SSR form/script separation, header routes, and restricted shell assertions inspected. |
| `frontend_admin/src/test_domain_types_ssr_tests_navigation.rs` | Reviewed: pagination, permission navigation, and sign-in branding assertions inspected. |
| `frontend_admin/src/test_domain_types_ssr_tests_settings.rs` | Reviewed: settings layout and contract input-kind render assertions inspected. |
| `frontend_admin/src/test_domain_types_with_owner_tests.rs` | Focused review; A103 session revoke action label regression. |
| `frontend_admin/src/test_static_pages_tests.rs` | Focused review; A110 system-role Update action regression. |
| `frontend_admin/src/test_table_cell_tests.rs` | Reviewed: preview content and action bypass assertions inspected. |
| `frontend_admin/src/wasm_bindgen_admin_read_error.rs` | Reviewed: browser error wrapper and redacted message inspected. |
| `frontend_admin/src/wasm_bindgen_password_generation_exception.rs` | Reviewed: browser randomness exception wrapper inspected. |
| `frontend_admin/src/with_admin_session.rs` | Focused review: authentication retry, refresh, and error precedence inspected; no confirmed behavior defect. |
| `frontend_admin/src/with_owner.rs` | Reviewed: Leptos owner creation and view attachment inspected. |

### frontend_contract

| Source | Semantic review |
| --- | --- |
| `frontend_contract/src/action_contract.rs` | Pending |
| `frontend_contract/src/action_contracts.rs` | Pending |
| `frontend_contract/src/api_problem.rs` | Pending |
| `frontend_contract/src/api_problem_detail.rs` | Reviewed: bounded 1024-byte detail, empty default and validated String deserialization; generated schema/serialization adapters. |
| `frontend_contract/src/api_problem_error.rs` | Pending |
| `frontend_contract/src/api_problem_field.rs` | Reviewed: bounded 128-byte field label, empty default and validated String deserialization. |
| `frontend_contract/src/api_problem_kind.rs` | Reviewed: explicit snake-case serializable API problem classification variants. |
| `frontend_contract/src/api_problem_request_id.rs` | Reviewed: bounded 128-byte request identifier and validated String deserialization. |
| `frontend_contract/src/api_problem_status.rs` | Reviewed: validated 100..999 HTTP code range, delegated known-status conversion and validated numeric deserialization. |
| `frontend_contract/src/api_problem_violation.rs` | Reviewed: private typed field/detail pair with serialization and schema derives. |
| `frontend_contract/src/api_problem_violations.rs` | Reviewed: typed bounded 128-element violation collection and delegated bounded deserialization. |
| `frontend_contract/src/api_url.rs` | Reviewed: bounded output, transactional mutation, component encoding and correct path/query/fragment insertion; A20. |
| `frontend_contract/src/api_url_build_error.rs` | Reviewed: typed invalid-segment failure contains no supplied URL text. |
| `frontend_contract/src/api_url_component_encode_set.rs` | Reviewed: percent encoding covers controls, spaces, percent, reserved separators and unsafe punctuation; non-ASCII handling delegates to pinned percent-encoding. |
| `frontend_contract/src/api_url_path_segment_ref.rs` | Reviewed: borrowed segment rejects empty, slash and exact dot traversal names before percent encoding. |
| `frontend_contract/src/api_url_query_component_ref.rs` | Reviewed: borrowed query text forwarded without allocation; encoding belongs to the URL builder. |
| `frontend_contract/src/apply_openapi_error_contract.rs` | Reviewed: generated route registration replaces documented 4xx and 5xx responses with typed route statuses, schemas, and the rate-limit retry header; the typed-route integration test checks declared JSON error content. |
| `frontend_contract/src/apply_openapi_path_parameter_contract.rs` | Reviewed: generated route registration adds the typed route path parameter when declared. The helper is invoked once for each generated operation. |
| `frontend_contract/src/apply_openapi_request_contract.rs` | Reviewed: generated route registration sets a required JSON request body only when the typed route declares one and provides a schema; the typed-route integration test checks that content. |
| `frontend_contract/src/apply_openapi_security_contract.rs` | Reviewed: public routes clear security; authenticated routes require the authentication scheme and mutating routes add the CSRF scheme in the same requirement. |
| `frontend_contract/src/apply_openapi_success_contract.rs` | Reviewed: generated route registration replaces 2xx responses with the typed success status and includes JSON content only for non-204 responses with a schema. |
| `frontend_contract/src/auth_session_instant.rs` | Reviewed: external Instant wrapper, generated copy access and explicit current-time Default adapter. |
| `frontend_contract/src/auth_session_keep_alive.rs` | Reviewed: single-flight begin, finish scheduling and missing-state reset; typed finish errors and explicit overflow suppression/reset under A26; no production consumers found. |
| `frontend_contract/src/auth_session_keep_alive_decision.rs` | Reviewed: typed refresh/skip decisions, explicit overflow suppression and domain-wrapped deadline representation under A26. |
| `frontend_contract/src/auth_session_keep_alive_error.rs` | Reviewed: typed zero-interval and interval-overflow classifications under A26. |
| `frontend_contract/src/auth_session_presence.rs` | Reviewed: missing/present state discriminator. |
| `frontend_contract/src/auth_session_refresh_interval_duration.rs` | Reviewed: nonzero duration validation; runtime scheduling overflow is explicitly handled by the keep-alive owner under A26. |
| `frontend_contract/src/auth_session_refresh_outcome.rs` | Reviewed: explicit successful, failed and rejected refresh outcomes. |
| `frontend_contract/src/auth_session_refresh_state.rs` | Reviewed: idle/running/interval-overflow state discriminator and explicit reset semantics under A26. |
| `frontend_contract/src/authenticated_transport.rs` | Pending |
| `frontend_contract/src/authentication_requirement.rs` | Reviewed: public/authenticated/typed static-rule access descriptors. |
| `frontend_contract/src/axum_method_filter.rs` | Pending |
| `frontend_contract/src/axum_route_method_router.rs` | Pending |
| `frontend_contract/src/capability_support.rs` | Reviewed: supported/unsupported capability discriminator. |
| `frontend_contract/src/client_error.rs` | Pending |
| `frontend_contract/src/client_request.rs` | Pending |
| `frontend_contract/src/client_route_metadata.rs` | Pending |
| `frontend_contract/src/client_tests.rs` | Pending |
| `frontend_contract/src/confirmation_requirement.rs` | Reviewed: explicit confirmation-required/not-required discriminator. |
| `frontend_contract/src/contract_i64.rs` | Pending |
| `frontend_contract/src/contract_str.rs` | Pending |
| `frontend_contract/src/covered_route.rs` | Pending |
| `frontend_contract/src/create_form_value_error.rs` | Pending |
| `frontend_contract/src/decode_api_problem.rs` | Pending |
| `frontend_contract/src/empty_filter_contracts.rs` | Pending |
| `frontend_contract/src/field_capability.rs` | Pending |
| `frontend_contract/src/field_contract.rs` | Pending |
| `frontend_contract/src/field_contracts.rs` | Pending |
| `frontend_contract/src/field_label.rs` | Pending |
| `frontend_contract/src/field_name.rs` | Pending |
| `frontend_contract/src/field_order.rs` | Pending |
| `frontend_contract/src/field_placeholder.rs` | Pending |
| `frontend_contract/src/field_visibility.rs` | Pending |
| `frontend_contract/src/filter_contracts.rs` | Pending |
| `frontend_contract/src/filter_form_value_contract.rs` | Pending |
| `frontend_contract/src/filter_operation.rs` | Pending |
| `frontend_contract/src/filter_value_shape.rs` | Pending |
| `frontend_contract/src/filter_wire_json.rs` | Pending |
| `frontend_contract/src/form_field_error.rs` | Pending |
| `frontend_contract/src/form_field_name_ref.rs` | Pending |
| `frontend_contract/src/form_value.rs` | Pending |
| `frontend_contract/src/form_value_contract.rs` | Pending |
| `frontend_contract/src/form_value_error.rs` | Pending |
| `frontend_contract/src/form_value_ref.rs` | Pending |
| `frontend_contract/src/frontend_contract_axum_router.rs` | Pending |
| `frontend_contract/src/frontend_contract_body_error.rs` | Pending |
| `frontend_contract/src/has_filter_contracts.rs` | Pending |
| `frontend_contract/src/has_type_contract.rs` | Pending |
| `frontend_contract/src/http_status_try_from_u16_error.rs` | Pending |
| `frontend_contract/src/input_kind.rs` | Pending |
| `frontend_contract/src/input_step.rs` | Pending |
| `frontend_contract/src/known_http_status.rs` | Pending |
| `frontend_contract/src/lib.rs` | Pending |
| `frontend_contract/src/missing_required_test_categories.rs` | Pending |
| `frontend_contract/src/mutation_kind.rs` | Pending |
| `frontend_contract/src/nullability.rs` | Pending |
| `frontend_contract/src/numeric_bound.rs` | Pending |
| `frontend_contract/src/open_api_security_scheme_ref.rs` | Pending |
| `frontend_contract/src/openapi_route_metadata.rs` | Pending |
| `frontend_contract/src/operation_kind.rs` | Pending |
| `frontend_contract/src/page_contract.rs` | Pending |
| `frontend_contract/src/page_transport_tests.rs` | Pending |
| `frontend_contract/src/parameterized_route.rs` | Pending |
| `frontend_contract/src/parameterized_route_path.rs` | Pending |
| `frontend_contract/src/parameterized_route_path_try_from_string_error.rs` | Pending |
| `frontend_contract/src/parse_timestamp_filter_wire_json.rs` | Fixed and reviewed; A105 shared timestamp form-value conversion. |
| `frontend_contract/src/primary_key_kind.rs` | Pending |
| `frontend_contract/src/problem_tests.rs` | Pending |
| `frontend_contract/src/public_transport.rs` | Pending |
| `frontend_contract/src/register_openapi_route_schemas.rs` | Pending |
| `frontend_contract/src/register_openapi_schema.rs` | Pending |
| `frontend_contract/src/register_route.rs` | Pending |
| `frontend_contract/src/required_test_categories.rs` | Pending |
| `frontend_contract/src/route_access.rs` | Pending |
| `frontend_contract/src/route_body_limit.rs` | Reviewed: copyable body-size domain wrapper; explicit zero values remain permitted. |
| `frontend_contract/src/route_contract.rs` | Pending |
| `frontend_contract/src/route_contracts.rs` | Pending |
| `frontend_contract/src/route_coverage_descriptor.rs` | Reviewed: private typed metadata/access/mutation/evidence fields with generated constructor ordering and getters. |
| `frontend_contract/src/route_coverage_descriptors.rs` | Reviewed: ordered coverage collection, validated Vec conversion and maximum iterator adapter with the reviewed usize::MAX bound. |
| `frontend_contract/src/route_coverage_error.rs` | Pending |
| `frontend_contract/src/route_coverage_evidence.rs` | Pending |
| `frontend_contract/src/route_coverage_obligation.rs` | Pending |
| `frontend_contract/src/route_coverage_tests.rs` | Focused review; A120 regression covers duplicate method/path with distinct operation IDs and permits distinct methods. |
| `frontend_contract/src/route_database_usage.rs` | Reviewed: explicit database/no-database descriptor variants. |
| `frontend_contract/src/route_error_policy.rs` | Pending |
| `frontend_contract/src/route_error_status.rs` | Pending |
| `frontend_contract/src/route_family.rs` | Reviewed: optional body-limit/default schema contracts and ordered metadata projection from coverage descriptors; matching usize::MAX collection bounds prevent projection truncation. |
| `frontend_contract/src/route_in_family.rs` | Pending |
| `frontend_contract/src/route_json_body_usage.rs` | Reviewed: explicit JSON-body/no-body capability variants. |
| `frontend_contract/src/route_metadata.rs` | Pending |
| `frontend_contract/src/route_metadata_list.rs` | Reviewed: ordered metadata collection, validated Vec conversion and matching reviewed usize::MAX bound. |
| `frontend_contract/src/route_method.rs` | Pending |
| `frontend_contract/src/route_method_router.rs` | Pending |
| `frontend_contract/src/route_mutation.rs` | Reviewed: mutating/read-only contract discriminator. |
| `frontend_contract/src/route_registration_contract.rs` | Pending |
| `frontend_contract/src/route_request.rs` | Pending |
| `frontend_contract/src/route_request_body.rs` | Reviewed: absent/JSON request-body discriminator. |
| `frontend_contract/src/route_response.rs` | Pending |
| `frontend_contract/src/route_response_kind.rs` | Reviewed: buffered/streaming response discriminator. |
| `frontend_contract/src/route_schema_contract.rs` | Pending |
| `frontend_contract/src/route_schema_contracts.rs` | Reviewed: ordered schema collection and validated Vec/iterator adapters with the reviewed usize::MAX bound. |
| `frontend_contract/src/route_test_capabilities.rs` | Pending |
| `frontend_contract/src/route_test_categories.rs` | Pending |
| `frontend_contract/src/route_test_category.rs` | Pending |
| `frontend_contract/src/route_tests.rs` | Pending |
| `frontend_contract/src/route_transport.rs` | Reviewed: compile-time transport marker trait without runtime state. |
| `frontend_contract/src/server_response.rs` | Pending |
| `frontend_contract/src/server_route_metadata.rs` | Pending |
| `frontend_contract/src/success_status.rs` | Pending |
| `frontend_contract/src/test_frontend_contract.rs` | Pending |
| `frontend_contract/src/to_axum_method_filter.rs` | Pending |
| `frontend_contract/src/transport.rs` | Pending |
| `frontend_contract/src/transport_body.rs` | Pending |
| `frontend_contract/src/transport_error.rs` | Pending |
| `frontend_contract/src/transport_idempotency_key.rs` | Pending |
| `frontend_contract/src/transport_if_match.rs` | Pending |
| `frontend_contract/src/transport_path.rs` | Pending |
| `frontend_contract/src/transport_request.rs` | Pending |
| `frontend_contract/src/transport_response.rs` | Pending |
| `frontend_contract/src/transport_retry_after.rs` | Pending |
| `frontend_contract/src/transport_status.rs` | Pending |
| `frontend_contract/src/type_contract.rs` | Pending |
| `frontend_contract/src/typed_client.rs` | Pending |
| `frontend_contract/src/typed_parameterized_route_path.rs` | Pending |
| `frontend_contract/src/typed_route.rs` | Pending |
| `frontend_contract/src/typed_route_path.rs` | Pending |
| `frontend_contract/src/url_builder_tests.rs` | Reviewed: deterministic encoding/traversal fixtures and three query/fragment insertion regressions using a shared typed URL fixture; A20. |
| `frontend_contract/src/utoipa_open_api_components_ref_mut.rs` | Pending |
| `frontend_contract/src/utoipa_open_api_path_parameter.rs` | Pending |
| `frontend_contract/src/utoipa_open_api_ref_mut.rs` | Pending |
| `frontend_contract/src/utoipa_open_api_route_schema.rs` | Pending |
| `frontend_contract/src/validate_route_coverage.rs` | Reviewed: rejects duplicate method/path pairs and checks baseline, authentication, and mutation coverage obligations; A120 fixed metadata-sensitive duplicate detection. |
| `frontend_contract/src/value_example.rs` | Pending |
| `frontend_contract/src/value_format.rs` | Pending |
| `frontend_contract/tests/typed_route.rs` | Pending |

### frontend_contract_validation

| Source | Semantic review |
| --- | --- |
| `frontend_contract_validation/src/canonical_json_contract_snapshot.rs` | Reviewed: iterative recursive field masking, deterministic pretty JSON and bounded output. Its two serialization error classifications are covered by the existing reviewed ignored-map_err inventory. |
| `frontend_contract_validation/src/http_contract_body.rs` | Reviewed: one callback invocation, metadata and status checks before body checks, bounded 16 MiB body, three-digit status range and private typed fields. Json means syntactically valid JSON; content-type headers are outside this fixture contract. |
| `frontend_contract_validation/src/http_contract_body_kind.rs` | Reviewed: one callback invocation, metadata and status checks before body checks, bounded 16 MiB body, three-digit status range and private typed fields. Json means syntactically valid JSON; content-type headers are outside this fixture contract. |
| `frontend_contract_validation/src/http_contract_expectation.rs` | Reviewed: one callback invocation, metadata and status checks before body checks, bounded 16 MiB body, three-digit status range and private typed fields. Json means syntactically valid JSON; content-type headers are outside this fixture contract. |
| `frontend_contract_validation/src/http_contract_mismatch.rs` | Reviewed: one callback invocation, metadata and status checks before body checks, bounded 16 MiB body, three-digit status range and private typed fields. Json means syntactically valid JSON; content-type headers are outside this fixture contract. |
| `frontend_contract_validation/src/http_contract_observation.rs` | Reviewed: one callback invocation, metadata and status checks before body checks, bounded 16 MiB body, three-digit status range and private typed fields. Json means syntactically valid JSON; content-type headers are outside this fixture contract. |
| `frontend_contract_validation/src/http_contract_status.rs` | Reviewed: one callback invocation, metadata and status checks before body checks, bounded 16 MiB body, three-digit status range and private typed fields. Json means syntactically valid JSON; content-type headers are outside this fixture contract. |
| `frontend_contract_validation/src/json_contract_snapshot.rs` | Reviewed: validated 1 MiB bounded storage; oversized output retains its bounded-string source. |
| `frontend_contract_validation/src/json_contract_snapshot_error.rs` | Reviewed: reviewed unit serialization classification and typed TooLong source; no original error details are interpolated into public diagnostics. |
| `frontend_contract_validation/src/json_snapshot_dynamic_field_ref.rs` | Reviewed: borrowed field-name forwarding; matching is by object key at every depth, not a path expression. |
| `frontend_contract_validation/src/lib.rs` | Reviewed: flat public contract modules and private validator state/helper ownership. |
| `frontend_contract_validation/src/open_api_contract_text.rs` | Reviewed: bounded validated contract text and generated forwarding. |
| `frontend_contract_validation/src/open_api_operation_expectation.rs` | Reviewed: private Copy metadata, status, content and security fields with generated constructor/getters. |
| `frontend_contract_validation/src/open_api_operation_validation_error.rs` | Reviewed: distinct operation validation failures and retained serialization error. |
| `frontend_contract_validation/src/open_api_payload_validation_error.rs` | Reviewed: distinct payload, schema and document serialization failures and typed mismatches. |
| `frontend_contract_validation/src/open_api_payload_validation_stage.rs` | Reviewed: private fieldless workflow states track composition and payload descent without recursive calls. |
| `frontend_contract_validation/src/open_api_response_status.rs` | Reviewed: validated inclusive HTTP status range 100 through 999. |
| `frontend_contract_validation/src/open_api_schema_mismatch.rs` | Reviewed: exhaustive mismatch categories used by the current payload validator. |
| `frontend_contract_validation/src/open_api_schema_reference_ref.rs` | Reviewed: ordered borrowed reference label wrapper, with generated conversion and private storage. |
| `frontend_contract_validation/src/open_api_schema_references_b_tree_set.rs` | Focused review: missing schemas and reference lookup; A49 supplies decoded component names and verifies full local pointers before insertion. |
| `frontend_contract_validation/src/open_api_security_expectation.rs` | Reviewed: explicit public or named required security expectation. |
| `frontend_contract_validation/src/open_api_validation_error.rs` | Reviewed: typed route/reference mismatch data and text conversion errors. |
| `frontend_contract_validation/src/openapi_schema_references.rs` | Focused review: iterative traversal, bounded reference names and A49 nested or escaped local JSON Pointers; external URI reference semantics remain pending. |
| `frontend_contract_validation/src/openapi_validation_tests.rs` | Reviewed: deterministic fixtures for references, operations, required fields and additional properties; cycle guards, recursive children and long reference chains are covered; operation security inheritance, A122 anonymous alternative rejection, explicit overrides, duplicate normalized methods, A121 duplicate runtime routes, and nested or escaped local references are also covered. |
| `frontend_contract_validation/src/route_contract_mismatch.rs` | Reviewed: exact method, operation identifier and path comparisons; all metadata differences are retained in fixed order and typed-route metadata delegates to the same owner. |
| `frontend_contract_validation/src/route_contract_mismatches.rs` | Reviewed: exact method, operation identifier and path comparisons; all metadata differences are retained in fixed order and typed-route metadata delegates to the same owner. |
| `frontend_contract_validation/src/route_contract_validation_tests.rs` | Reviewed: deterministic metadata mismatch order, typed route source and in-memory HTTP fixture assertions. |
| `frontend_contract_validation/src/run_http_contract_fixture.rs` | Reviewed: one callback invocation, metadata and status checks before body checks, bounded 16 MiB body, three-digit status range and private typed fields. Json means syntactically valid JSON; content-type headers are outside this fixture contract. |
| `frontend_contract_validation/src/runtime_routes_ref.rs` | Reviewed: borrowed typed route slice with generated forwarding. |
| `frontend_contract_validation/src/serde_json_open_api_serialization_error.rs` | Reviewed: transparent serialization error wrapper preserving the source. |
| `frontend_contract_validation/src/validate_openapi_contract.rs` | Focused review: schema use, route matching, operation identifiers, duplicate normalized method and runtime route rejection (A121), A49 local reference integration and A51 nonobject path-item rejection; external URI reference semantics remain pending. |
| `frontend_contract_validation/src/validate_openapi_json_payload.rs` | Reviewed: A07 borrows JSON without serialization or owned copies; iterative evaluation and equality preserve scoped reference guards and composition results. Deep input and comparison regressions verified. |
| `frontend_contract_validation/src/validate_openapi_operations.rs` | Reviewed: operation lookup, effective security inheritance, exact response status and content/schema checks; A21 fixes ignored document security and A122 requires every alternative to carry required security. |
| `frontend_contract_validation/src/validate_openapi_schema_references.rs` | Reviewed: serialization errors propagate and reference discovery and validation delegate to their tracked owners. |
| `frontend_contract_validation/src/validate_route_contract_metadata.rs` | Reviewed: exact method, operation identifier and path comparisons; all metadata differences are retained in fixed order and typed-route metadata delegates to the same owner. |
| `frontend_contract_validation/src/validate_typed_route_contract.rs` | Reviewed: exact method, operation identifier and path comparisons; all metadata differences are retained in fixed order and typed-route metadata delegates to the same owner. |

### fuzz

| Source | Semantic review |
| --- | --- |
| `fuzz/fuzz_targets/domain_boundaries.rs` | Reviewed: all seven parser branches and the signed-cursor branch inspected for panic paths, bounded input, and conversion errors; no defect confirmed. |

### generate_quotes

| Source | Semantic review |
| --- | --- |
| `generate_quotes/src/binary_double_quote_style.rs` | Reviewed: styles, wrappers, forwarding and existing deterministic literal tests inspected. |
| `generate_quotes/src/binary_double_quoted_str.rs` | Reviewed: styles, wrappers, forwarding and existing deterministic literal tests inspected. |
| `generate_quotes/src/binary_double_quoted_token_stream.rs` | Reviewed: styles, wrappers, forwarding and existing deterministic literal tests inspected. |
| `generate_quotes/src/binary_single_quote_style.rs` | Reviewed: styles, wrappers, forwarding and existing deterministic literal tests inspected. |
| `generate_quotes/src/binary_single_quotes_str.rs` | Reviewed: styles, wrappers, forwarding and existing deterministic literal tests inspected. |
| `generate_quotes/src/binary_single_quotes_token_stream.rs` | Reviewed: styles, wrappers, forwarding and existing deterministic literal tests inspected. |
| `generate_quotes/src/build_quote_style.rs` | Reviewed: styles, wrappers, forwarding and existing deterministic literal tests inspected. |
| `generate_quotes/src/double_quote_style.rs` | Reviewed: styles, wrappers, forwarding and existing deterministic literal tests inspected. |
| `generate_quotes/src/double_quoted_string.rs` | Reviewed: styles, wrappers, forwarding and existing deterministic literal tests inspected. |
| `generate_quotes/src/dq_token_stream.rs` | Reviewed: styles, wrappers, forwarding and existing deterministic literal tests inspected. |
| `generate_quotes/src/lib.rs` | Reviewed: styles, wrappers, forwarding and existing deterministic literal tests inspected. |
| `generate_quotes/src/proc_macro2_quoted_literal_token_stream.rs` | Reviewed: styles, wrappers, forwarding and existing deterministic literal tests inspected. |
| `generate_quotes/src/quote_char.rs` | Reviewed: styles, wrappers, forwarding and existing deterministic literal tests inspected. |
| `generate_quotes/src/quote_literal.rs` | Reviewed: literal construction and bounded conversion now propagate A30 errors; A06 remains dismissed. |
| `generate_quotes/src/quote_panic_id.rs` | Reviewed: styles, wrappers, forwarding and existing deterministic literal tests inspected. |
| `generate_quotes/src/quote_prefix.rs` | Reviewed: styles, wrappers, forwarding and existing deterministic literal tests inspected. |
| `generate_quotes/src/quote_str.rs` | Reviewed: styles, wrappers, forwarding and existing deterministic literal tests inspected. |
| `generate_quotes/src/quote_style.rs` | Reviewed: styles, wrappers, forwarding and existing deterministic literal tests inspected. |
| `generate_quotes/src/quote_token_stream.rs` | Reviewed: A30 conversion and parse failures emit compile_error tokens; A06 remains dismissed. |
| `generate_quotes/src/quoted_literal.rs` | Focused review: literal construction, parse fallback and bounded conversion; A06 dismissed. |
| `generate_quotes/src/quoted_literal_max_len.rs` | Reviewed: styles, wrappers, forwarding and existing deterministic literal tests inspected. |
| `generate_quotes/src/single_quote_style.rs` | Reviewed: styles, wrappers, forwarding and existing deterministic literal tests inspected. |
| `generate_quotes/src/single_quotes_str.rs` | Reviewed: styles, wrappers, forwarding and existing deterministic literal tests inspected. |
| `generate_quotes/src/single_quotes_token_stream.rs` | Reviewed: styles, wrappers, forwarding and existing deterministic literal tests inspected. |
| `generate_quotes/src/tests_domain_types.rs` | Reviewed: styles, wrappers, forwarding and existing deterministic literal tests inspected. |

### git_info

| Source | Semantic review |
| --- | --- |
| `git_info/src/base_git_commit_link_len.rs` | Reviewed: exact compile-time byte sum of the URL prefix and tree segment, used consistently by the size guard and capacity. |
| `git_info/src/build_git_commit_link.rs` | Reviewed: A27 propagates typed URL-length errors and transfers successful Cow ownership into the bounded owned link. |
| `git_info/src/build_git_commit_link_cow.rs` | Reviewed: A27 rejects total-byte overflow before allocating, preserves the static project-link shortcut and returns validated URLs through Result. |
| `git_info/src/check_is_project_commit.rs` | Reviewed: compares the supplied commit with the repository commit. |
| `git_info/src/git_commit_id.rs` | Reviewed: A27 borrowed-to-owned TryFrom rejects excessive length before allocation; string conversion uses shared byte validation and explicit diagnostic conversion remains bounded. |
| `git_info/src/git_commit_id_cow.rs` | Reviewed: borrowed or owned byte-length validation and diagnostic conversion; provider substitution remains part of A27. |
| `git_info/src/git_commit_id_fallback.rs` | Reviewed: optional owned fallback with generated borrowed and mutable access; callers retain lifetime ownership. |
| `git_info/src/git_commit_id_provider.rs` | Reviewed: A27 Result propagation for owned/Cow IDs, validated borrowed callbacks and fallible fallback initialization; valid cache entries are retained and failed initialization leaves the cache empty. |
| `git_info/src/git_commit_id_ref.rs` | Reviewed: borrowed unbounded reference, lifetime-preserving conversions and equality; owned conversion now rejects excessive length through TryFrom under A27. |
| `git_info/src/git_commit_link.rs` | Reviewed: shared bounded validation, validated Cow ownership transfer and static-reference equality; A05 diagnostic conversion. |
| `git_info/src/git_commit_link_capacity.rs` | Reviewed: copyable capacity domain wrapper with generated owned conversion and borrowed access. |
| `git_info/src/git_commit_link_capacity_value.rs` | Reviewed: saturating prefix-plus-ID capacity calculation; the allocation caller validates its combined bound first. |
| `git_info/src/git_commit_link_cow.rs` | Reviewed: borrowed or owned storage, byte limit, validated deserialization and diagnostic formatting; owned link conversion uses the same limit. |
| `git_info/src/git_commit_link_provider.rs` | Reviewed: A27 forwards provider and URL-length errors through Result, preserving borrowed commit dispatch and ownership transfer. |
| `git_info/src/git_info_string_max_len.rs` | Reviewed: shared 1 MiB byte bound used by commit and link validation. |
| `git_info/src/git_info_string_try_from_string_error.rs` | Reviewed: copyable TooLong diagnostic preserves actual and maximum lengths without interpolating input text. |
| `git_info/src/is_project_commit.rs` | Reviewed: boolean wrapper preserves project-commit classification. |
| `git_info/src/lib.rs` | Reviewed: root declares commit, link, provider, and validation owners. |
| `git_info/src/project_git_commit_link.rs` | Reviewed: owned project link derives from the validated static link. |
| `git_info/src/project_git_commit_link_ref.rs` | Reviewed: static borrowed link wrapper with generated lifetime-preserving conversions and equality. |
| `git_info/src/project_git_commit_link_ref_value.rs` | Reviewed: static link constant is wrapped without allocation. |
| `git_info/src/project_git_info.rs` | Reviewed: project metadata stores a borrowed commit and exposes it through a generated getter. |
| `git_info/src/project_git_info_value.rs` | Reviewed: wraps the repository commit constant as project metadata. |
| `git_info/src/test_git_info.rs` | Reviewed: tests cover project link consistency, commit validation, fallback dispatch, and size boundaries. |
| `git_info/src/try_bounded_git_info_string.rs` | Reviewed: inclusive byte limit and bounded storage validation with matching TooLong classification. |
| `git_info/src/validate_project_commit.rs` | Reviewed: rejects a different commit with the static project link. |
| `git_info/src/validate_project_commit_error.rs` | Reviewed: typed validation error retains the project link. |
| `git_info/src/with_git_commit_id_ref_or.rs` | Reviewed: lifetime-preserving optional reference dispatch invokes exactly one callback and avoids owned allocation when a reference exists. |

### init_env_files

| Source | Semantic review |
| --- | --- |
| `init_env_files/src/env_content.rs` | Reviewed: environment initialization parsing, bounded I/O, typed metadata, or fixture behavior inspected; no additional defect confirmed. |
| `init_env_files/src/env_content_ref.rs` | Reviewed: environment initialization parsing, bounded I/O, typed metadata, or fixture behavior inspected; no additional defect confirmed. |
| `init_env_files/src/env_key.rs` | Reviewed: environment initialization parsing, bounded I/O, typed metadata, or fixture behavior inspected; no additional defect confirmed. |
| `init_env_files/src/env_keys.rs` | Reviewed: environment initialization parsing, bounded I/O, typed metadata, or fixture behavior inspected; no additional defect confirmed. |
| `init_env_files/src/environment_keys.rs` | Reviewed: environment initialization parsing, bounded I/O, typed metadata, or fixture behavior inspected; no additional defect confirmed. |
| `init_env_files/src/init_entries.rs` | Reviewed: environment initialization parsing, bounded I/O, typed metadata, or fixture behavior inspected; no additional defect confirmed. |
| `init_env_files/src/init_io_error.rs` | Reviewed: environment initialization parsing, bounded I/O, typed metadata, or fixture behavior inspected; no additional defect confirmed. |
| `init_env_files/src/init_max_bytes.rs` | Reviewed: environment initialization parsing, bounded I/O, typed metadata, or fixture behavior inspected; no additional defect confirmed. |
| `init_env_files/src/init_path_exists.rs` | Reviewed: environment initialization parsing, bounded I/O, typed metadata, or fixture behavior inspected; no additional defect confirmed. |
| `init_env_files/src/init_path_ref.rs` | Reviewed: environment initialization parsing, bounded I/O, typed metadata, or fixture behavior inspected; no additional defect confirmed. |
| `init_env_files/src/init_string_error.rs` | Reviewed: environment initialization parsing, bounded I/O, typed metadata, or fixture behavior inspected; no additional defect confirmed. |
| `init_env_files/src/initialization_entry.rs` | Reviewed: environment initialization parsing, bounded I/O, typed metadata, or fixture behavior inspected; no additional defect confirmed. |
| `init_env_files/src/initialization_status.rs` | Reviewed: environment initialization parsing, bounded I/O, typed metadata, or fixture behavior inspected; no additional defect confirmed. |
| `init_env_files/src/initialize.rs` | Fixed: A61 rejects non-string workspace members with a deterministic regression. |
| `init_env_files/src/initialize_error.rs` | Fixed: A61 rejects non-string workspace members with a deterministic regression. |
| `init_env_files/src/main.rs` | Reviewed: environment initialization parsing, bounded I/O, typed metadata, or fixture behavior inspected; no additional defect confirmed. |
| `init_env_files/src/path_exists.rs` | Reviewed: environment initialization parsing, bounded I/O, typed metadata, or fixture behavior inspected; no additional defect confirmed. |
| `init_env_files/src/read_bounded_content.rs` | Reviewed: environment initialization parsing, bounded I/O, typed metadata, or fixture behavior inspected; no additional defect confirmed. |
| `init_env_files/src/run_mode.rs` | Reviewed: environment initialization parsing, bounded I/O, typed metadata, or fixture behavior inspected; no additional defect confirmed. |
| `init_env_files/src/test_initialize_tests.rs` | Fixed: A61 rejects non-string workspace members with a deterministic regression. |
| `init_env_files/src/toml_init_error.rs` | Reviewed: environment initialization parsing, bounded I/O, typed metadata, or fixture behavior inspected; no additional defect confirmed. |
| `init_env_files/src/toml_member_value.rs` | Fixed: typed TOML member boundary for A61. |
| `init_env_files/src/workspace_member.rs` | Fixed: A61 rejects non-string workspace members with a deterministic regression. |
| `init_env_files/src/workspace_root_path_ref.rs` | Reviewed: environment initialization parsing, bounded I/O, typed metadata, or fixture behavior inspected; no additional defect confirmed. |
| `init_env_files/src/write_content.rs` | Reviewed: environment initialization parsing, bounded I/O, typed metadata, or fixture behavior inspected; no additional defect confirmed. |

### initialize_environment_files

| Source | Semantic review |
| --- | --- |
| `initialize_environment_files/src/configuration_field.rs` | Reviewed: static configuration fragments become owned bounded text before assembly. |
| `initialize_environment_files/src/docker_compose_database_service.rs` | Reviewed: database fields are appended under one service key. |
| `initialize_environment_files/src/docker_compose_file.rs` | Reviewed: service, network, and volume sections are assembled in fixed order. |
| `initialize_environment_files/src/docker_compose_notification_database_service.rs` | Reviewed: notification database fields are appended under one service key. |
| `initialize_environment_files/src/docker_compose_notification_service.rs` | Reviewed: notification service fields include build, dependencies, environment, health, and runtime settings. |
| `initialize_environment_files/src/docker_compose_notification_service_environment.rs` | Reviewed: notification environment fields are appended once in fixed order. |
| `initialize_environment_files/src/docker_compose_notification_service_migrate_service.rs` | Reviewed: migration service receives its own environment and dependency settings. |
| `initialize_environment_files/src/docker_compose_server_environment.rs` | Reviewed: server environment order preserves generated socket markers in serve mode. |
| `initialize_environment_files/src/docker_compose_server_environment_order.rs` | Reviewed: mode enum selects migration or serve field order. |
| `initialize_environment_files/src/docker_compose_server_migrate_service.rs` | Reviewed: server migration fields and mode are assembled under the migration service. |
| `initialize_environment_files/src/docker_compose_server_service.rs` | Reviewed: server fields include build, dependencies, environment, health, and runtime settings. |
| `initialize_environment_files/src/generate_environment_files.rs` | Reviewed: writes the owned Compose and environment files and propagates I/O errors. |
| `initialize_environment_files/src/main.rs` | Reviewed: command resolves the workspace root and invokes its filesystem owner. |
| `initialize_environment_files/src/std_byte_vector.rs` | Reviewed: generated fragments append to the owned unbounded text buffer. |
| `initialize_environment_files/src/test_initialize_environment_files.rs` | Reviewed: test covers replacement and output of every owned file. |

### location_lib

| Source | Semantic review |
| --- | --- |
| `location_lib/src/chrono_location_date_time.rs` | Reviewed: location conversion, formatting, schema, or bounds behavior inspected; no additional defect confirmed. |
| `location_lib/src/chrono_location_display_timezone.rs` | Reviewed: location conversion, formatting, schema, or bounds behavior inspected; no additional defect confirmed. |
| `location_lib/src/domain_types.rs` | Reviewed: location conversion, formatting, schema, or bounds behavior inspected; no additional defect confirmed. |
| `location_lib/src/formatter_ref_mut.rs` | Reviewed: location conversion, formatting, schema, or bounds behavior inspected; no additional defect confirmed. |
| `location_lib/src/lib.rs` | Reviewed: location conversion, formatting, schema, or bounds behavior inspected; no additional defect confirmed. |
| `location_lib/src/location.rs` | Fixed: A60 retains the original bounded path prefix and adds a deterministic regression. |
| `location_lib/src/location_column.rs` | Reviewed: location conversion, formatting, schema, or bounds behavior inspected; no additional defect confirmed. |
| `location_lib/src/location_commit.rs` | Reviewed: location conversion, formatting, schema, or bounds behavior inspected; no additional defect confirmed. |
| `location_lib/src/location_coordinate_try_from_u32_error.rs` | Reviewed: location conversion, formatting, schema, or bounds behavior inspected; no additional defect confirmed. |
| `location_lib/src/location_duration.rs` | Reviewed: location conversion, formatting, schema, or bounds behavior inspected; no additional defect confirmed. |
| `location_lib/src/location_file.rs` | Fixed: A60 retains the original bounded path prefix and adds a deterministic regression. |
| `location_lib/src/location_file_ref.rs` | Reviewed: location conversion, formatting, schema, or bounds behavior inspected; no additional defect confirmed. |
| `location_lib/src/location_line.rs` | Reviewed: location conversion, formatting, schema, or bounds behavior inspected; no additional defect confirmed. |
| `location_lib/src/occurrence.rs` | Reviewed: location conversion, formatting, schema, or bounds behavior inspected; no additional defect confirmed. |
| `location_lib/src/std_time_duration.rs` | Reviewed: location conversion, formatting, schema, or bounds behavior inspected; no additional defect confirmed. |
| `location_lib/src/std_time_duration_nanos.rs` | Reviewed: location conversion, formatting, schema, or bounds behavior inspected; no additional defect confirmed. |
| `location_lib/src/std_time_duration_nanos_try_from_u32_error.rs` | Reviewed: location conversion, formatting, schema, or bounds behavior inspected; no additional defect confirmed. |
| `location_lib/src/std_time_duration_secs.rs` | Reviewed: location conversion, formatting, schema, or bounds behavior inspected; no additional defect confirmed. |
| `location_lib/src/test_location_lib.rs` | Fixed: A60 retains the original bounded path prefix and adds a deterministic regression. |

### location_lib_location_test

| Source | Semantic review |
| --- | --- |
| `location_lib_location_test/src/create_location_test_text.rs` | Reviewed: generated-location fixture construction, typed fields, and error rendering inspected. |
| `location_lib_location_test/src/display_struct.rs` | Reviewed: generated-location fixture construction, typed fields, and error rendering inspected. |
| `location_lib_location_test/src/error_one.rs` | Reviewed: generated-location fixture construction, typed fields, and error rendering inspected. |
| `location_lib_location_test/src/error_two.rs` | Reviewed: generated-location fixture construction, typed fields, and error rendering inspected. |
| `location_lib_location_test/src/error_unnamed_one.rs` | Reviewed: generated-location fixture construction, typed fields, and error rendering inspected. |
| `location_lib_location_test/src/loc_test_text_max_len.rs` | Reviewed: generated-location fixture construction, typed fields, and error rendering inspected. |
| `location_lib_location_test/src/location_test_count.rs` | Reviewed: generated-location fixture construction, typed fields, and error rendering inspected. |
| `location_lib_location_test/src/location_test_flag.rs` | Reviewed: generated-location fixture construction, typed fields, and error rendering inspected. |
| `location_lib_location_test/src/location_test_text.rs` | Reviewed: generated-location fixture construction, typed fields, and error rendering inspected. |
| `location_lib_location_test/src/main.rs` | Reviewed: generated-location fixture construction, typed fields, and error rendering inspected. |
| `location_lib_location_test/src/serde_struct.rs` | Reviewed: generated-location fixture construction, typed fields, and error rendering inspected. |

### macro_clippy_check_test_common

| Source | Semantic review |
| --- | --- |
| `macro_clippy_check_test_common/src/generated_crate_phase.rs` | Reviewed: generated-crate command sequence, manifest transformation, cleanup, and test coverage inspected. |
| `macro_clippy_check_test_common/src/generated_crate_step.rs` | Reviewed: generated-crate command sequence, manifest transformation, cleanup, and test coverage inspected. |
| `macro_clippy_check_test_common/src/generated_crate_steps_tests.rs` | Reviewed: generated-crate command sequence, manifest transformation, cleanup, and test coverage inspected. |
| `macro_clippy_check_test_common/src/lib.rs` | Reviewed: generated-crate command sequence, manifest transformation, cleanup, and test coverage inspected. |
| `macro_clippy_check_test_common/src/remove_dir_on_drop.rs` | Reviewed: generated-crate command sequence, manifest transformation, cleanup, and test coverage inspected. |

### macro_helpers

| Source | Semantic review |
| --- | --- |
| `macro_helpers/src/assert_file_content.rs` | Reviewed: the test assertion reads no more than the expected byte count, validates UTF-8, and compares exact content. |
| `macro_helpers/src/assert_file_path_ref.rs` | Reviewed: borrowed path wrapper preserves the input lifetime and forwards PathBuf borrowing. |
| `macro_helpers/src/attr_identifier_name.rs` | Reviewed: borrowed attribute-name wrapper preserves the input lifetime. |
| `macro_helpers/src/attr_identifier_str.rs` | Reviewed: provider trait returns a borrowed attribute-name wrapper. |
| `macro_helpers/src/cleanup_test_file.rs` | Reviewed: test cleanup ignores absent files and reports other removal failures. |
| `macro_helpers/src/compile_error_message.rs` | Reviewed: borrowed diagnostic text wrapper preserves the source lifetime. |
| `macro_helpers/src/contract_error.rs` | Reviewed: round-trip contract failures remain distinct typed variants. |
| `macro_helpers/src/derive_token_stream_builder.rs` | Reviewed: derive builder delegates its fixed supported derive catalog to the generator. |
| `macro_helpers/src/ensure_json_contract_round_trip.rs` | Reviewed: fixture decoding, serialization, second decoding, and value equality have distinct typed errors; existing tests cover failures at each stage. |
| `macro_helpers/src/expected_file_content.rs` | Reviewed: borrowed text wrapper preserves the lifetime through its input conversion. |
| `macro_helpers/src/expected_file_content_ref.rs` | Reviewed: conversions retain borrowed text without copying. |
| `macro_helpers/src/field_location_column.rs` | Reviewed: zero is rejected and positive coordinates retain their value. |
| `macro_helpers/src/field_location_coordinate_try_from_u32_error.rs` | Reviewed: typed zero-coordinate error. |
| `macro_helpers/src/field_location_file.rs` | Reviewed: borrowed static file path wrapper. |
| `macro_helpers/src/field_location_line.rs` | Reviewed: zero is rejected and positive coordinates retain their value. |
| `macro_helpers/src/find_macro_attribute.rs` | Reviewed: compares complete attribute path segments and intentionally normalizes spaces and empty lookup segments, as its focused tests specify. |
| `macro_helpers/src/format_with_cargofmt.rs` | Reviewed: explicit formatting-mode discriminator. |
| `macro_helpers/src/generate_const_new_token_stream_impl.rs` | Reviewed: passes the const modifier to the shared constructor emitter. |
| `macro_helpers/src/generate_const_try_new_token_stream_impl.rs` | Reviewed: passes const and error type to the shared fallible constructor emitter. |
| `macro_helpers/src/generate_field_location_new_token_stream.rs` | Reviewed: emits current and stored field locations; coordinate wrappers reject zero and generated-token tests cover both positions. |
| `macro_helpers/src/generate_if_write_is_error_token_stream.rs` | Reviewed: generated branch executes the supplied tokens only when formatting fails. |
| `macro_helpers/src/generate_impl_const_new_for_identifier_token_stream_impl.rs` | Reviewed: wraps the const constructor in the supplied impl; token test covers output. |
| `macro_helpers/src/generate_impl_default_token_stream.rs` | Reviewed: preserves the supplied type and default body; token test covers emitted syntax. |
| `macro_helpers/src/generate_impl_display_token_stream.rs` | Reviewed: forwards generics and method body into the Display implementation. |
| `macro_helpers/src/generate_impl_from_token_stream.rs` | Reviewed: binds the From input to the expected generated identifier and emits the supplied body. |
| `macro_helpers/src/generate_impl_modified_new_for_identifier_token_stream_impl.rs` | Reviewed: composes modifier, constructor, and impl wrapper without dropping inputs. |
| `macro_helpers/src/generate_impl_modified_try_new_for_identifier_token_stream_impl.rs` | Reviewed: composes modifier, fallible constructor, error type, and impl wrapper. |
| `macro_helpers/src/generate_impl_new_for_identifier_token_stream_impl.rs` | Reviewed: emits unmodified constructor in the supplied impl; token test covers output. |
| `macro_helpers/src/generate_impl_pub_const_new_for_identifier_token_stream_impl.rs` | Reviewed: emits public const constructor in the supplied impl; token test covers output. |
| `macro_helpers/src/generate_impl_pub_const_try_new_for_identifier_token_stream_impl.rs` | Reviewed: emits public const fallible constructor in the supplied impl. |
| `macro_helpers/src/generate_impl_pub_new_for_identifier_token_stream_impl.rs` | Reviewed: emits public constructor in the supplied impl. |
| `macro_helpers/src/generate_impl_pub_try_new_for_identifier_token_stream_impl.rs` | Reviewed: emits public fallible constructor in the supplied impl. |
| `macro_helpers/src/generate_impl_to_err_string_token_stream.rs` | Reviewed: generated conversion validates bounded error text with its existing fallback. |
| `macro_helpers/src/generate_impl_try_from_token_stream.rs` | Reviewed: generated TryFrom implementation preserves its error type and supplied body. |
| `macro_helpers/src/generate_impl_try_new_for_identifier_token_stream_impl.rs` | Reviewed: emits unmodified fallible constructor in the supplied impl. |
| `macro_helpers/src/generate_modified_new_token_stream_impl.rs` | Reviewed: attaches attributes before visibility or const modifiers and preserves the constructor body. |
| `macro_helpers/src/generate_modified_try_new_token_stream_impl.rs` | Reviewed: attaches attributes before modifiers and preserves the fallible constructor body and error type. |
| `macro_helpers/src/generate_new_token_stream_impl.rs` | Reviewed: emits an inherent new method with supplied parameters and body. |
| `macro_helpers/src/generate_pub_const_new_token_stream_impl.rs` | Reviewed: passes public const modifiers to the shared constructor emitter. |
| `macro_helpers/src/generate_pub_const_try_new_token_stream_impl.rs` | Reviewed: passes public const modifiers and error type to the shared fallible constructor emitter. |
| `macro_helpers/src/generate_pub_new_token_stream_impl.rs` | Reviewed: passes public visibility to the shared constructor emitter. |
| `macro_helpers/src/generate_pub_try_new_token_stream_impl.rs` | Reviewed: passes public visibility and error type to the shared fallible constructor emitter. |
| `macro_helpers/src/generate_pub_type_alias_token_stream.rs` | Reviewed: emits public alias with supplied generic target; token test covers it. |
| `macro_helpers/src/generate_serde_version_of_named_syn_variant.rs` | Focused review: malformed collection types, panic paths, and Vec shape (A123, A130); remaining generation branches pending. |
| `macro_helpers/src/generate_simple_syn_punct.rs` | Reviewed: punctuates supplied path segments; tests cover empty, one, and three segments. |
| `macro_helpers/src/generate_try_new_token_stream_impl.rs` | Reviewed: emits a fallible inherent constructor with supplied error type, parameters, and body. |
| `macro_helpers/src/generate_validated_tokens.rs` | Reviewed: stops at the first failed stage and emits only validated output; tests cover both paths. |
| `macro_helpers/src/get_macro_attribute_tests.rs` | Reviewed: exact path matching, normalized lookup separators, missing attributes, and non-list errors. |
| `macro_helpers/src/impl_identifier_token_stream_impl.rs` | Reviewed: wraps supplied tokens in the named inherent impl. |
| `macro_helpers/src/json_contract_tests.rs` | Reviewed: typed failures at fixture decode, serialization, and round-trip decode phases. |
| `macro_helpers/src/json_fixture_ref.rs` | Reviewed: borrowed fixture wrapper retains its source lifetime. |
| `macro_helpers/src/lib.rs` | Reviewed: module ownership and test-only or feature-gated declarations match the source inventory. |
| `macro_helpers/src/location_field_attr.rs` | Reviewed: all supported markers, duplicate/missing selection, and marker syntax; A124. |
| `macro_helpers/src/location_syn_field.rs` | Reviewed: constructs the named location field with the expected three-segment type path. |
| `macro_helpers/src/macro_attr_error.rs` | Reviewed: distinct missing-attribute and wrong-meta-shape outcomes. |
| `macro_helpers/src/macro_compile_error_tokens.rs` | Reviewed: emits a compile-error invocation with borrowed diagnostic text. |
| `macro_helpers/src/macro_path_ref.rs` | Reviewed: borrowed path wrapper retains the input lifetime. |
| `macro_helpers/src/macro_serde_json_error.rs` | Reviewed: transparent typed wrapper retains the serde JSON source. |
| `macro_helpers/src/only_one.rs` | Reviewed: selects exactly one bare supported status marker, rejects malformed, duplicate, and missing cases; A125. |
| `macro_helpers/src/only_one_status_code_error.rs` | Reviewed: distinct typed outcomes for malformed, duplicate, and missing status markers; A125. |
| `macro_helpers/src/os_string_value.rs` | Reviewed: converts a borrowed program path to owned OsString for command ownership. |
| `macro_helpers/src/pagination_start_end_initialization_token_stream.rs` | Reviewed: generated start and end bindings preserve the receiver; token test covers output. |
| `macro_helpers/src/proc_macro2_derive_tokens_ref.rs` | Reviewed: borrowed derive-token slice preserves token lifetimes and order. |
| `macro_helpers/src/proc_macro2_generated_rust_token_stream.rs` | Reviewed: owned token wrapper forwards display, tokens, and conversions. |
| `macro_helpers/src/proc_macro2_if_write_is_err_token_stream.rs` | Reviewed: owned formatting-error token wrapper forwards token emission. |
| `macro_helpers/src/proc_macro2_macro_attr_meta_list_token_stream_ref.rs` | Reviewed: borrowed list-token wrapper forwards token emission. |
| `macro_helpers/src/proc_macro2_token_stream_ref.rs` | Reviewed: borrowed token wrapper preserves source lifetime. |
| `macro_helpers/src/process_exit_status.rs` | Reviewed: exit-status wrapper forwards display and status inspection. |
| `macro_helpers/src/process_output.rs` | Reviewed: owned process output wrapper preserves status and both byte streams. |
| `macro_helpers/src/rs_file_path_buf.rs` | Reviewed: owned generated `.rs` path wrapper preserves the path. |
| `macro_helpers/src/rs_file_path_tests.rs` | Reviewed: extension replacement and parent-directory preservation tests. |
| `macro_helpers/src/sanitized_database_target.rs` | Reviewed: bounded target display excludes credentials and query values. |
| `macro_helpers/src/should_write_string.rs` | Reviewed: typed change flag for generated-file outcomes. |
| `macro_helpers/src/should_write_string_into_file_tests.rs` | Reviewed: test-only chunked comparison aligns with production on changed invalid UTF-8 content; A131. |
| `macro_helpers/src/should_write_token_stream_into_file.rs` | Reviewed: typed write-selection flag. |
| `macro_helpers/src/status_code.rs` | Reviewed: all 60 numeric mappings match their variants and all 60 HTTP token names match pinned http 1.5.0; token and parsing tests pass. |
| `macro_helpers/src/std_assert_file_path.rs` | Reviewed: borrowed assertion-path wrapper preserves the source lifetime. |
| `macro_helpers/src/std_fmt_arguments.rs` | Reviewed: borrowed formatting arguments preserve the formatting lifetime. |
| `macro_helpers/src/std_io_write_ref.rs` | Reviewed: mutable writer wrapper retains exclusive borrow and Write bound. |
| `macro_helpers/src/std_str_chars.rs` | Reviewed: character iterator wrapper preserves the source lifetime. |
| `macro_helpers/src/std_tool_io_error.rs` | Reviewed: transparent I/O source wrapper preserves error chaining. |
| `macro_helpers/src/string_file_content_ref.rs` | Reviewed: borrowed write-content wrapper preserves text lifetime. |
| `macro_helpers/src/string_syn_punct.rs` | Reviewed: builds the fixed three-segment string path through the shared helper. |
| `macro_helpers/src/syn_field.rs` | Reviewed: generated constructor and getters preserve named identifier, type, and visibility fields. |
| `macro_helpers/src/syn_field_identifier.rs` | Reviewed: identifier wrapper forwards display and token emission. |
| `macro_helpers/src/syn_field_type.rs` | Reviewed: type wrapper forwards token emission. |
| `macro_helpers/src/syn_field_vis.rs` | Reviewed: visibility wrapper forwards token emission. |
| `macro_helpers/src/syn_location_field.rs` | Reviewed: owned field wrapper supports construction and ownership transfer. |
| `macro_helpers/src/syn_macro_attr_ref.rs` | Reviewed: borrowed attribute wrapper preserves the source lifetime and token emission. |
| `macro_helpers/src/syn_path_segment.rs` | Reviewed: owned path-segment wrapper preserves the segment. |
| `macro_helpers/src/syn_path_segments.rs` | Reviewed: punctuated path wrapper preserves segments and token emission. |
| `macro_helpers/src/syn_variant_ref.rs` | Reviewed: borrowed variant wrapper preserves its source lifetime. |
| `macro_helpers/src/test_database.rs` | Reviewed: loopback test targets, ambiguous and remote targets, credential redaction, and query overrides; A128. |
| `macro_helpers/src/test_generate_new_or_try_new_tests.rs` | Reviewed: token checks cover plain, const, and public const inherent constructors. |
| `macro_helpers/src/test_generate_validated_tokens.rs` | Reviewed: success and first-error short-circuit tests for generation stages. |
| `macro_helpers/src/test_path.rs` | Reviewed: test-only path generation uses a process-local sequence for unique fixtures. |
| `macro_helpers/src/test_path_stem.rs` | Reviewed: borrowed test-path stem wrapper preserves its lifetime. |
| `macro_helpers/src/tool_ansi_chars.rs` | Reviewed: plain Unicode, CSI and OSC control filtering; A129. |
| `macro_helpers/src/tool_ansi_escape_state.rs` | Reviewed: parser states cover text, ESC, CSI, and OSC termination; A129. |
| `macro_helpers/src/tool_ansi_text_ref.rs` | Reviewed: borrowed ANSI text wrapper retains its lifetime. |
| `macro_helpers/src/tool_arg_ref.rs` | Reviewed: borrowed command argument wrapper. |
| `macro_helpers/src/tool_args_ref.rs` | Reviewed: borrowed command argument slice wrapper. |
| `macro_helpers/src/tool_command.rs` | Reviewed: command construction, bounded dual-pipe draining, status propagation, and argument-redacted Debug; A80 covers retained output. |
| `macro_helpers/src/tool_console_stream.rs` | Reviewed: stream selection, write error propagation, and failure exit; deterministic writer tests cover success and broken pipe. |
| `macro_helpers/src/tool_console_write_error.rs` | Reviewed: distinct stdout and stderr errors retain I/O sources. |
| `macro_helpers/src/tool_env_key_ref.rs` | Reviewed: borrowed environment-key wrapper. |
| `macro_helpers/src/tool_env_value_ref.rs` | Reviewed: borrowed environment-value wrapper. |
| `macro_helpers/src/tool_output_byte.rs` | Reviewed; A80 |
| `macro_helpers/src/tool_output_byte_vec_deque.rs` | Reviewed; A80 |
| `macro_helpers/src/tool_output_limit.rs` | Reviewed; A80 |
| `macro_helpers/src/tool_output_tail.rs` | Reviewed; A80 |
| `macro_helpers/src/tool_process_command.rs` | Reviewed: owned process-command wrapper forwards mutable command construction. |
| `macro_helpers/src/tool_program_ref.rs` | Reviewed: borrowed program path wrapper. |
| `macro_helpers/src/try_get_macro_attr_meta_list_token_stream.rs` | Reviewed: returns list tokens for a matching attribute and a typed wrong-shape error otherwise. |
| `macro_helpers/src/try_get_macro_attribute.rs` | Reviewed: forwards exact lookup and returns a typed missing-attribute error. |
| `macro_helpers/src/try_maybe_write_token_stream_into_file.rs` | Reviewed: write selection, token conversion, formatter invocation, and retry after failure; A127. |
| `macro_helpers/src/try_write_string_into_file.rs` | Reviewed: forwards the generated-file write outcome to its owned path. |
| `macro_helpers/src/try_write_string_into_file_with_outcome.rs` | Reviewed: chunked equality, changed-file replacement, atomic commit, and path outcome; A126. |
| `macro_helpers/src/try_write_string_into_path_tests.rs` | Reviewed: test-only exact-path adapter returns the owned path from the write outcome. |
| `macro_helpers/src/try_write_string_into_path_with_outcome_tests.rs` | Reviewed: test-only path adapter preserves changed or unchanged outcome. |
| `macro_helpers/src/url_error.rs` | Reviewed: typed malformed, non-loopback, and ambiguous-database outcomes. |
| `macro_helpers/src/url_ref.rs` | Reviewed: borrowed URL wrapper preserves the source lifetime. |
| `macro_helpers/src/validate_test_database_url.rs` | Reviewed: scheme, authority, loopback host, test database name, redaction, and query overrides; A128. |
| `macro_helpers/src/with_attr_token_stream_impl.rs` | Reviewed: places supplied attributes before the generated item. |
| `macro_helpers/src/wrap_derive.rs` | Reviewed: preserves derive order; empty input produces `derive()` as its explicit test specifies and has no production caller. |
| `macro_helpers/src/write_path_outcome.rs` | Reviewed: changed and unchanged outcomes preserve owned path and typed change flag. |
| `macro_helpers/src/write_string_if_needed_tests.rs` | Focused review: test-only atomic writer uses the predicate aligned in A131. |
| `macro_helpers/src/write_string_into_file_tests.rs` | Reviewed: exact-path, `.rs` extension, chunked comparison, changed/unchanged outcome, and invalid UTF-8 production regression coverage; A126, A131. |
| `macro_helpers/src/write_token_stream_into_file_tests.rs` | Reviewed: skip, write, path, successful format, and failed-format retry tests; A127. |
| `macro_helpers/src/written_file_path_buf.rs` | Reviewed: owned written-file path wrapper preserves the path. |
| `macro_helpers/src/written_file_path_ref.rs` | Reviewed: borrowed written-file path wrapper preserves its lifetime. |

### naming

| Source | Semantic review |
| --- | --- |
| `naming/src/display_plus_to_tokens.rs` | Reviewed: marker trait combines Display and ToTokens with blanket forwarding and no state. |
| `naming/src/domain_types.rs` | Focused review: generated fixed naming catalog and representative tests inspected; A23 regression now rejects oversized Swagger path case expansion. Full expanded output remains part of A23 consumer review. |
| `naming/src/hash_map.rs` | Reviewed: fieldless naming marker with generated Copy/Clone/layout behavior; no conversion or allocation logic. |
| `naming/src/hash_map_snake_case.rs` | Reviewed: intentional hashmap spelling is checked by the existing domain_types test; Display and ToTokens emit the same spelling without allocation in the owner. |
| `naming/src/hash_map_upper_camel_case.rs` | Reviewed: Display and ToTokens consistently emit HashMap; the existing domain_types test confirms the spelling. |
| `naming/src/lib.rs` | Reviewed: flat module ownership declarations. |
| `naming/src/parameter.rs` | Reviewed: fixed self-template catalog contains exactly one placeholder per entry under A29. |
| `naming/src/swagger_url_path_prefix.rs` | Reviewed: borrowed prefix wrapper forwards immutable text without allocation or validation. |
| `naming/src/swagger_url_path_self_quotes_str.rs` | Reviewed: A30 propagates quote-length errors and A23 now propagates case-conversion failures through the equivalent quote error variants. |
| `naming/src/swagger_url_path_self_quotes_token_stream.rs` | Reviewed: A30 maps quote conversion and parsing failures to compile_error tokens. |

### naming_naming_common

| Source | Semantic review |
| --- | --- |
| `naming_naming_common/src/case_from_string.rs` | Reviewed: delegates case conversion with the selected typed case kind. |
| `naming_naming_common/src/case_string.rs` | Reviewed: bounded case-result wrapper now exposes its generated typed conversion error for A23 fallible callers. |
| `naming_naming_common/src/case_string_max_len.rs` | Reviewed: explicit 1 MiB output bound, which case expansion can exceed under A23. |
| `naming_naming_common/src/convert_case_kind.rs` | Reviewed: private Copy adapter for static convert_case variants. |
| `naming_naming_common/src/display_case_str.rs` | Reviewed: formats Display once and delegates to the shared case conversion; A23 propagates through this path. |
| `naming_naming_common/src/domain_types.rs` | Reviewed: nine generated case-trait pairs and deterministic string/token conversion fixtures; A23 fallible methods and token overflow regression pass while old string methods remain pending migration. |
| `naming_naming_common/src/lib.rs` | Reviewed: flat case conversion module ownership and the compiled macro regression module. |
| `naming_naming_common/src/proc_macro2_case_token_stream.rs` | Reviewed: owned token-stream adapter with generated conversion and ownership transfer. |
| `naming_naming_common/src/str_case.rs` | Reviewed: shared conversion now returns typed overflow rather than substituting diagnostic text under A23. |
| `naming_naming_common/src/test_case_trait_pair.rs` | Reviewed: compiled move/raw-parameter fixture verifies both generated methods. |
| `naming_naming_common/src/to_token_stream_or_panic.rs` | Reviewed: token parsing returns ordinary compile_error tokens for invalid text rather than panicking. |
| `naming_naming_common/src/tokenized_case_str.rs` | Reviewed: quotes the input once and delegates case conversion; A23 propagates through this path. |

### notification_service

| Source | Semantic review |
| --- | --- |
| `notification_service/src/axum_notification_response.rs` | Reviewed: response wrapper forwards Axum response ownership. |
| `notification_service/src/build_notification_router.rs` | Reviewed: API and operational routes use typed registries, a bounded body layer, and shared common routes. |
| `notification_service/src/create_notification.rs` | Reviewed: inserts validated message and returns its generated UUID only after persistence succeeds. |
| `notification_service/src/create_notification_error.rs` | Reviewed: persistence and validation failures map to distinct statuses and telemetry. |
| `notification_service/src/http_notification_status_code.rs` | Reviewed: HTTP status wrapper forwards response conversion. |
| `notification_service/src/main.rs` | Reviewed: migrate and serve modes propagate startup, runtime, and observability shutdown failures. |
| `notification_service/src/metrics.rs` | Reviewed: bounded metrics rendering retains observed failure context. |
| `notification_service/src/metrics_error.rs` | Reviewed: render failure emits internal API problem with diagnostic telemetry. |
| `notification_service/src/metrics_exporter_prometheus_notification_build_error.rs` | Reviewed: recorder initialization error wrapper preserves display text. |
| `notification_service/src/notification_api_route_registry.rs` | Reviewed: create route and schemas come from the typed API catalog. |
| `notification_service/src/notification_axum_json.rs` | Reviewed: JSON extractor maps invalid requests to typed validation failure. |
| `notification_service/src/notification_axum_router.rs` | Reviewed: typed Axum router ownership wrapper. |
| `notification_service/src/notification_axum_state.rs` | Reviewed: state extractor returns a clone of the shared pool and metrics handles. |
| `notification_service/src/notification_body_maximum_bytes.rs` | Reviewed: typed body limit wrapper. |
| `notification_service/src/notification_error_code.rs` | Reviewed: fixed codes distinguish metrics, persistence, and validation. |
| `notification_service/src/notification_exit_code.rs` | Reviewed: termination forwards process exit status. |
| `notification_service/src/notification_io_error.rs` | Reviewed: socket error wrapper preserves display text. |
| `notification_service/src/notification_metrics_exporter_prometheus_renderer.rs` | Reviewed: renderer validates response size before return. |
| `notification_service/src/notification_open_api.rs` | Reviewed: API and common-route documents merge for the operational endpoint. |
| `notification_service/src/notification_route_registry.rs` | Reviewed: operational registry maps metrics and OpenAPI handlers. |
| `notification_service/src/notification_service_error.rs` | Reviewed: startup and runtime failures use distinct typed variants. |
| `notification_service/src/notification_state.rs` | Reviewed: pool, metrics, and Git metadata providers forward their stored fields. |
| `notification_service/src/open_api_document.rs` | Reviewed: test helper exposes the generated API document. |
| `notification_service/src/shared_notification_state_arc.rs` | Reviewed: one Arc allocation shares immutable service state with common routes. |
| `notification_service/src/sqlx_notification_database_error.rs` | Reviewed: database error retains its source. |
| `notification_service/src/sqlx_notification_migration_error.rs` | Reviewed: migration error wrapper preserves display text. |
| `notification_service/src/test_notification_service.rs` | Reviewed: tests cover routes, OpenAPI, error telemetry, adapters, and provisioned persistence. |

### notification_service_config

| Source | Semantic review |
| --- | --- |
| `notification_service_config/src/lib.rs` | Reviewed: flat module declaration only. |
| `notification_service_config/src/notification_service_config.rs` | Reviewed: private typed config fields, generated accessors, secret marking, examples and accessor test; no local parsing logic. |
| `notification_service_config/tests/config_descriptor.rs` | Reviewed: generated examples, descriptor requirements, Compose environment, deployment ports and image references; all four integration tests pass. |

### notification_service_contract

| Source | Semantic review |
| --- | --- |
| `notification_service_contract/src/create_notification_request.rs` | Reviewed: request carries one validated message and rejects unknown JSON fields. |
| `notification_service_contract/src/create_notification_response.rs` | Reviewed: response carries a typed notification identifier. |
| `notification_service_contract/src/create_notification_route.rs` | Reviewed: typed public POST contract matches the request and 201 response. |
| `notification_service_contract/src/lib.rs` | Reviewed: root declares the message, identifier, route, and test modules. |
| `notification_service_contract/src/notification_api_body_max_bytes.rs` | Reviewed: finite body limit is used by both route catalogs. |
| `notification_service_contract/src/notification_message.rs` | Reviewed: string conversion and deserialization enforce nonempty and 4,096-byte bounds. |
| `notification_service_contract/src/notification_message_max_len.rs` | Reviewed: message maximum matches the bounded storage type. |
| `notification_service_contract/src/notification_message_try_from_string_error.rs` | Reviewed: typed errors distinguish empty and excessive messages. |
| `notification_service_contract/src/notification_operational_route.rs` | Reviewed: operational routes are registered and excluded from the API family. |
| `notification_service_contract/src/notification_route.rs` | Reviewed: API family contains the create route and shares the body limit. |
| `notification_service_contract/src/tests_domain_types.rs` | Reviewed: tests cover route generation, message bounds, and validated deserialization. |
| `notification_service_contract/src/uuid_notification_id.rs` | Reviewed: UUID wrapper serializes and deserializes through the typed conversion. |

### panic_location

| Source | Semantic review |
| --- | --- |
| `panic_location/src/lib.rs` | Reviewed: once-installed panic hook captures payload and source location through tracing; three package tests pass. |
| `panic_location/src/panic_column.rs` | Reviewed: typed column forwarding and formatting. |
| `panic_location/src/panic_file.rs` | Reviewed: borrowed file path forwarding and formatting. |
| `panic_location/src/panic_line.rs` | Reviewed: typed line forwarding and formatting. |
| `panic_location/src/panic_with_location_message.rs` | Reviewed: typed message formatting, payload conversion and deterministic tests. |

### pg_crud_common

| Source | Semantic review |
| --- | --- |
| `pg_crud_common/benches/query_builders.rs` | Pending |
| `pg_crud_common/src/add_operator.rs` | Pending |
| `pg_crud_common/src/all_enum_variants.rs` | Pending |
| `pg_crud_common/src/all_enum_variants_array_default_some_one_element.rs` | Pending |
| `pg_crud_common/src/all_enum_variants_array_default_some_one_element_max_page_size.rs` | Pending |
| `pg_crud_common/src/batch_duplicate_policy.rs` | Reviewed: three explicit duplicate policies inspected. |
| `pg_crud_common/src/batch_invalid_item_count.rs` | Reviewed: count wrapper and conversions inspected. |
| `pg_crud_common/src/batch_invalid_items.rs` | Reviewed: invalid-item collection wrapper inspected. |
| `pg_crud_common/src/batch_processed_item_count.rs` | Reviewed: processed-count wrapper inspected. |
| `pg_crud_common/src/batch_records_b_tree_map.rs` | Reviewed: keyed-record collection wrapper inspected. |
| `pg_crud_common/src/batch_stopped_early.rs` | Reviewed: early-stop flag wrapper inspected. |
| `pg_crud_common/src/batch_validation_report.rs` | Reviewed: report fields, invalid count, and ownership transfer inspected. |
| `pg_crud_common/src/batch_validation_tests.rs` | Reviewed: duplicate, keep-first, keep-last, invalid-limit, and zero-limit assertions inspected. |
| `pg_crud_common/src/bool_test_cases_vec.rs` | Pending |
| `pg_crud_common/src/bounded_b_tree_map_error.rs` | Pending |
| `pg_crud_common/src/bounded_unique_vec.rs` | Reviewed: length bounds, duplicate detection, and serialization boundary inspected; existing tests cover errors. |
| `pg_crud_common/src/bounded_unique_vec_visitor_phantom_data.rs` | Reviewed: bounded preallocation, duplicate detection, and excess-item deserialization behavior inspected. |
| `pg_crud_common/src/bounded_vec_error.rs` | Pending |
| `pg_crud_common/src/build_date_sql_filter.rs` | Reviewed: bound order, emitted values, final representable bind index, and overflow behavior; A132. |
| `pg_crud_common/src/build_pg_scoped_foreign_key_clause.rs` | Pending |
| `pg_crud_common/src/build_sql_like_pattern.rs` | Pending |
| `pg_crud_common/src/build_stable_read_query_plan.rs` | Pending |
| `pg_crud_common/src/bulk_mutation_outcome.rs` | Pending |
| `pg_crud_common/src/chrono_utc_date_time_ref.rs` | Pending |
| `pg_crud_common/src/chrono_utc_date_times.rs` | Pending |
| `pg_crud_common/src/classify_pg_code.rs` | Reviewed: mapped and unknown SQLSTATE cases inspected. |
| `pg_crud_common/src/classify_pg_error.rs` | Reviewed: database, pool, connection, and unknown SQLx error paths inspected. |
| `pg_crud_common/src/classify_slice_ordering.rs` | Reviewed: increasing, duplicate, and descending transitions inspected. |
| `pg_crud_common/src/contains_duplicate_identifier.rs` | Reviewed: pairwise identifier duplicate detection inspected. |
| `pg_crud_common/src/cursor_codec.rs` | Reviewed: version, length, signature verification, UTF-8 decode, and round-trip tests inspected. |
| `pg_crud_common/src/cursor_codec_build_error.rs` | Pending |
| `pg_crud_common/src/cursor_decode_error.rs` | Pending |
| `pg_crud_common/src/cursor_encode_error.rs` | Pending |
| `pg_crud_common/src/cursor_maximum_length.rs` | Pending |
| `pg_crud_common/src/cursor_pagination_usage.rs` | Pending |
| `pg_crud_common/src/cursor_payload.rs` | Reviewed: empty, oversized, and bounded-storage conversion paths; A133. |
| `pg_crud_common/src/cursor_payload_error.rs` | Reviewed: distinct empty and oversized errors; A133. |
| `pg_crud_common/src/cursor_signing_key.rs` | Pending |
| `pg_crud_common/src/cursor_signing_key_error.rs` | Pending |
| `pg_crud_common/src/cursor_signing_key_maximum_length.rs` | Pending |
| `pg_crud_common/src/data_invariant_violation.rs` | Pending |
| `pg_crud_common/src/date_filter_bounds.rs` | Pending |
| `pg_crud_common/src/date_sql_bind_start_non_zero_u32.rs` | Pending |
| `pg_crud_common/src/date_sql_filter.rs` | Pending |
| `pg_crud_common/src/date_sql_filter_error.rs` | Pending |
| `pg_crud_common/src/db_catalog_snapshot.rs` | Pending |
| `pg_crud_common/src/db_column_contract_snapshot.rs` | Pending |
| `pg_crud_common/src/db_column_contract_snapshots.rs` | Pending |
| `pg_crud_common/src/db_column_has_server_default.rs` | Pending |
| `pg_crud_common/src/db_column_nullable.rs` | Pending |
| `pg_crud_common/src/db_column_snapshot.rs` | Pending |
| `pg_crud_common/src/db_column_snapshots.rs` | Pending |
| `pg_crud_common/src/db_column_spec.rs` | Pending |
| `pg_crud_common/src/db_column_specs.rs` | Pending |
| `pg_crud_common/src/db_default_spec.rs` | Pending |
| `pg_crud_common/src/db_default_specs.rs` | Pending |
| `pg_crud_common/src/db_extended_table_schema.rs` | Pending |
| `pg_crud_common/src/db_key_contract_snapshot.rs` | Pending |
| `pg_crud_common/src/db_key_contract_snapshots.rs` | Pending |
| `pg_crud_common/src/db_key_spec.rs` | Pending |
| `pg_crud_common/src/db_key_specs.rs` | Pending |
| `pg_crud_common/src/db_object_kind.rs` | Pending |
| `pg_crud_common/src/db_object_snapshot.rs` | Pending |
| `pg_crud_common/src/db_object_snapshots.rs` | Pending |
| `pg_crud_common/src/db_object_spec.rs` | Pending |
| `pg_crud_common/src/db_object_specs.rs` | Pending |
| `pg_crud_common/src/db_schema_conformance_error.rs` | Pending |
| `pg_crud_common/src/db_schema_name_ref.rs` | Pending |
| `pg_crud_common/src/db_schema_text.rs` | Pending |
| `pg_crud_common/src/db_schema_texts.rs` | Pending |
| `pg_crud_common/src/db_static_schema_text.rs` | Pending |
| `pg_crud_common/src/db_static_schema_texts.rs` | Pending |
| `pg_crud_common/src/db_table_name_ref.rs` | Pending |
| `pg_crud_common/src/db_table_schema.rs` | Pending |
| `pg_crud_common/src/db_table_snapshot.rs` | Pending |
| `pg_crud_common/src/deduplicate_preserving_order_by_key.rs` | Pending |
| `pg_crud_common/src/default_some_one_element.rs` | Pending |
| `pg_crud_common/src/default_some_one_element_max_page_size.rs` | Pending |
| `pg_crud_common/src/domain_types.rs` | Pending |
| `pg_crud_common/src/duplicate_candidates.rs` | Pending |
| `pg_crud_common/src/duplicate_index.rs` | Pending |
| `pg_crud_common/src/eq_operator.rs` | Pending |
| `pg_crud_common/src/eq_operator_query_str.rs` | Pending |
| `pg_crud_common/src/explicit_value.rs` | Reviewed: generic value schema, item registration, distinct component names, and serialized field alignment; A136, A138, A140. |
| `pg_crud_common/src/f32_test_cases_vec.rs` | Pending |
| `pg_crud_common/src/f64_test_cases_vec.rs` | Pending |
| `pg_crud_common/src/filter_bind_plan.rs` | Pending |
| `pg_crud_common/src/finite_f64.rs` | Pending |
| `pg_crud_common/src/finite_f64_error.rs` | Pending |
| `pg_crud_common/src/first_duplicate_index.rs` | Pending |
| `pg_crud_common/src/first_duplicate_index_by_hash.rs` | Pending |
| `pg_crud_common/src/i16_test_cases_vec.rs` | Pending |
| `pg_crud_common/src/i32_test_cases_vec.rs` | Pending |
| `pg_crud_common/src/i64_test_cases_vec.rs` | Pending |
| `pg_crud_common/src/i8_test_cases_vec.rs` | Pending |
| `pg_crud_common/src/increment_checked_add_one_returning_increment.rs` | Pending |
| `pg_crud_common/src/inspect_postgres_catalog.rs` | Pending |
| `pg_crud_common/src/inspect_postgres_table.rs` | Pending |
| `pg_crud_common/src/is_string_empty.rs` | Pending |
| `pg_crud_common/src/is_string_empty_result.rs` | Pending |
| `pg_crud_common/src/lib.rs` | Pending |
| `pg_crud_common/src/list_items.rs` | Pending |
| `pg_crud_common/src/list_offset.rs` | Pending |
| `pg_crud_common/src/list_page.rs` | Pending |
| `pg_crud_common/src/list_rows.rs` | Pending |
| `pg_crud_common/src/list_rows_presence.rs` | Pending |
| `pg_crud_common/src/list_total.rs` | Pending |
| `pg_crud_common/src/list_total_error.rs` | Pending |
| `pg_crud_common/src/list_total_source.rs` | Pending |
| `pg_crud_common/src/lock_pg_relation_resources.rs` | Pending |
| `pg_crud_common/src/make_query_bind_error.rs` | Pending |
| `pg_crud_common/src/maximum_resource_count.rs` | Pending |
| `pg_crud_common/src/maximum_scoped_foreign_key_columns.rs` | Pending |
| `pg_crud_common/src/minimum_scoped_foreign_key_columns.rs` | Pending |
| `pg_crud_common/src/non_primary_key_pg_type_read_ids.rs` | Focused review: OpenAPI field and null-value schema match serialization; A140. |
| `pg_crud_common/src/not_empty_unique_vec.rs` | Focused review: generic array schema, item registration, distinct component names, and nonempty Default; A136, A138-A139. |
| `pg_crud_common/src/not_empty_unique_vec_max_len.rs` | Pending |
| `pg_crud_common/src/not_empty_unique_vec_try_new_error.rs` | Pending |
| `pg_crud_common/src/not_zero_unsigned_part_of_i32.rs` | Pending |
| `pg_crud_common/src/not_zero_unsigned_part_of_i32_try_from_i32_error.rs` | Pending |
| `pg_crud_common/src/nullable_json_obj_pg_type_where_filter.rs` | Focused review: SQL fragment length error propagation; A137. |
| `pg_crud_common/src/offset_pagination_presence.rs` | Pending |
| `pg_crud_common/src/operation_budget.rs` | Pending |
| `pg_crud_common/src/operation_budget_exceeded.rs` | Pending |
| `pg_crud_common/src/pg_numeric_range_length.rs` | Reviewed: A45 validates positive full-width `u64` length and binds it as exact PostgreSQL numeric. |
| `pg_crud_common/src/pg_numeric_range_length_error.rs` | Reviewed: typed zero-length rejection. |
| `pg_crud_common/src/pg_range_length_sql.rs` | Reviewed: A45 selects widened numeric arithmetic for integer ranges. |
| `pg_crud_common/src/operation_count.rs` | Pending |
| `pg_crud_common/src/operator.rs` | Pending |
| `pg_crud_common/src/order.rs` | Reviewed: fixed Ascending/Descending display, serde names and case strings; A23 removes dynamic case conversion for these bounded enum values. |
| `pg_crud_common/src/order_by.rs` | Reviewed: column/order schema, dependency registration, and distinct component names; A136, A138. |
| `pg_crud_common/src/order_preserving_values.rs` | Pending |
| `pg_crud_common/src/order_snake_case_str.rs` | Pending |
| `pg_crud_common/src/order_text_string.rs` | Reviewed: bounded storage, byte-limit validation and diagnostic conversion; A05. |
| `pg_crud_common/src/order_upper_camel_case_str.rs` | Pending |
| `pg_crud_common/src/pagination_base.rs` | Pending |
| `pg_crud_common/src/pagination_end.rs` | Pending |
| `pg_crud_common/src/pagination_limit.rs` | Pending |
| `pg_crud_common/src/pagination_offset.rs` | Pending |
| `pg_crud_common/src/pagination_policy.rs` | Pending |
| `pg_crud_common/src/pagination_start.rs` | Pending |
| `pg_crud_common/src/pagination_starts_with_zero.rs` | Pending |
| `pg_crud_common/src/pagination_starts_with_zero_raw.rs` | Pending |
| `pg_crud_common/src/pagination_starts_with_zero_try_new_error.rs` | Pending |
| `pg_crud_common/src/pagination_total.rs` | Pending |
| `pg_crud_common/src/patch_field.rs` | Pending |
| `pg_crud_common/src/pg_bounded_b_tree_map.rs` | Pending |
| `pg_crud_common/src/pg_bounded_vec.rs` | Reviewed: inclusive bounds, serde validation, JSON/OpenAPI schema bounds, distinct component names, and item schema registration; A134-A135, A138. |
| `pg_crud_common/src/pg_bounded_vec_len.rs` | Pending |
| `pg_crud_common/src/pg_column_schema.rs` | Pending |
| `pg_crud_common/src/pg_counter_reconciliation.rs` | Pending |
| `pg_crud_common/src/pg_counter_value.rs` | Pending |
| `pg_crud_common/src/pg_crud_string_wrapper_max_len.rs` | Pending |
| `pg_crud_common/src/pg_crud_string_wrapper_try_from_string_error.rs` | Pending |
| `pg_crud_common/src/pg_duplicate_identifier_presence.rs` | Pending |
| `pg_crud_common/src/pg_error_kind.rs` | Pending |
| `pg_crud_common/src/pg_filter_bind_value.rs` | Pending |
| `pg_crud_common/src/pg_filter_bool.rs` | Pending |
| `pg_crud_common/src/pg_filter_i64.rs` | Pending |
| `pg_crud_common/src/pg_filter_text.rs` | Pending |
| `pg_crud_common/src/pg_filter_text_error.rs` | Pending |
| `pg_crud_common/src/pg_is_primary_key.rs` | Pending |
| `pg_crud_common/src/pg_operational_limit.rs` | Pending |
| `pg_crud_common/src/pg_operational_limit_error.rs` | Pending |
| `pg_crud_common/src/pg_operational_limit_update_authority.rs` | Pending |
| `pg_crud_common/src/pg_relation_capacity_error.rs` | Pending |
| `pg_crud_common/src/pg_relation_capacity_maximum.rs` | Pending |
| `pg_crud_common/src/pg_relation_lock_error.rs` | Pending |
| `pg_crud_common/src/pg_relation_lock_namespace.rs` | Pending |
| `pg_crud_common/src/pg_relation_resource_id.rs` | Pending |
| `pg_crud_common/src/pg_relation_resource_ids.rs` | Pending |
| `pg_crud_common/src/pg_relation_row_count.rs` | Pending |
| `pg_crud_common/src/pg_scoped_foreign_key.rs` | Pending |
| `pg_crud_common/src/pg_scoped_foreign_key_clause_text.rs` | Pending |
| `pg_crud_common/src/pg_scoped_foreign_key_error.rs` | Pending |
| `pg_crud_common/src/pg_scoped_foreign_key_on_delete.rs` | Pending |
| `pg_crud_common/src/pg_sql_identifiers.rs` | Pending |
| `pg_crud_common/src/pg_type.rs` | Pending |
| `pg_crud_common/src/pg_type_eq_operator.rs` | Pending |
| `pg_crud_common/src/pg_type_greater_than_test.rs` | Pending |
| `pg_crud_common/src/pg_type_greater_than_variant.rs` | Pending |
| `pg_crud_common/src/pg_type_len_greater_than_test.rs` | Pending |
| `pg_crud_common/src/pg_type_not_primary_key.rs` | Pending |
| `pg_crud_common/src/pg_type_primary_key.rs` | Pending |
| `pg_crud_common/src/pg_type_test_cases.rs` | Pending |
| `pg_crud_common/src/pg_type_where.rs` | Focused review: value/operator schema registration, SQL fragment length error propagation, and distinct component names; A136-A138. |
| `pg_crud_common/src/pg_type_where_filter.rs` | Pending |
| `pg_crud_common/src/positive_finite_f64.rs` | Pending |
| `pg_crud_common/src/positive_finite_f64_error.rs` | Pending |
| `pg_crud_common/src/push_identifier_list.rs` | Pending |
| `pg_crud_common/src/query_part_error.rs` | Pending |
| `pg_crud_common/src/query_part_fragment.rs` | Reviewed: bounded writes reject overflow before mutation; allocation-free bind-index digit emission checks arithmetic and bounds; A05 diagnostic conversion. |
| `pg_crud_common/src/query_part_increment.rs` | Pending |
| `pg_crud_common/src/query_part_increment_mut.rs` | Pending |
| `pg_crud_common/src/query_sort_order.rs` | Pending |
| `pg_crud_common/src/read_query_bind_index_non_zero_u32.rs` | Pending |
| `pg_crud_common/src/read_query_plan.rs` | Pending |
| `pg_crud_common/src/read_query_plan_error.rs` | Pending |
| `pg_crud_common/src/read_search.rs` | Pending |
| `pg_crud_common/src/reconcile_pg_counter.rs` | Pending |
| `pg_crud_common/src/resolve_list_total_source.rs` | Pending |
| `pg_crud_common/src/resolve_pg_operational_limit_update.rs` | Pending |
| `pg_crud_common/src/run_list_with_total.rs` | Pending |
| `pg_crud_common/src/schema_text.rs` | Pending |
| `pg_crud_common/src/schema_texts.rs` | Pending |
| `pg_crud_common/src/serde_prealloc_max_items.rs` | Pending |
| `pg_crud_common/src/signed_cursor.rs` | Reviewed: empty, oversized, and bounded-storage conversion paths; A133. |
| `pg_crud_common/src/signed_cursor_error.rs` | Reviewed: distinct empty and oversized errors; A133. |
| `pg_crud_common/src/signed_cursor_presence.rs` | Pending |
| `pg_crud_common/src/single_or_multiple.rs` | Reviewed: two-branch schema, nested component registration, and distinct component names; A136, A138. |
| `pg_crud_common/src/test_generic_utoipa_schema_registration.rs` | Reviewed: focused regressions for generic Utoipa dependency registration; A136. |
| `pg_crud_common/src/test_generic_utoipa_schema_names.rs` | Reviewed: five generic component-name collision regressions; A138. |
| `pg_crud_common/src/test_not_empty_unique_vec_default.rs` | Reviewed: nonempty Default regression; A139. |
| `pg_crud_common/src/test_query_part_fragment_overflow.rs` | Reviewed: focused regressions for SQL fragment overflow; A137. |
| `pg_crud_common/src/slice_ordering.rs` | Pending |
| `pg_crud_common/src/snapshot_mismatch.rs` | Pending |
| `pg_crud_common/src/sql_column_ref.rs` | Pending |
| `pg_crud_common/src/sql_identifier.rs` | Focused review: length validation, conversions, and boundary behavior; see issue queue. |
| `pg_crud_common/src/sql_identifier_error.rs` | Pending |
| `pg_crud_common/src/sql_identifier_list_text.rs` | Focused review: length validation, conversions, and boundary behavior; see issue queue. |
| `pg_crud_common/src/sql_identifiers.rs` | Focused review: length validation, conversions, and boundary behavior; see issue queue. |
| `pg_crud_common/src/sql_like_input_ref.rs` | Pending |
| `pg_crud_common/src/sql_like_match_mode.rs` | Pending |
| `pg_crud_common/src/sql_like_pattern.rs` | Pending |
| `pg_crud_common/src/sql_like_pattern_error.rs` | Pending |
| `pg_crud_common/src/sql_qualified_identifier.rs` | Focused review: length validation, conversions, and boundary behavior; see issue queue. |
| `pg_crud_common/src/sql_select_builder.rs` | Focused review: length validation, conversions, and boundary behavior; see issue queue. |
| `pg_crud_common/src/sql_sort_order_text.rs` | Pending |
| `pg_crud_common/src/sqlx_box_dyn_error.rs` | Pending |
| `pg_crud_common/src/sqlx_db_schema_inspection_error.rs` | Pending |
| `pg_crud_common/src/sqlx_pg_catalog_pool_ref.rs` | Pending |
| `pg_crud_common/src/sqlx_pg_error_ref.rs` | Pending |
| `pg_crud_common/src/sqlx_pg_relation_lock_connection_ref.rs` | Pending |
| `pg_crud_common/src/sqlx_pg_relation_lock_error.rs` | Pending |
| `pg_crud_common/src/sqlx_postgres_query.rs` | Pending |
| `pg_crud_common/src/sqlx_postgres_query_bind_error.rs` | Pending |
| `pg_crud_common/src/static_schema_text.rs` | Pending |
| `pg_crud_common/src/static_schema_texts.rs` | Pending |
| `pg_crud_common/src/std_bounded_b_tree_map_len.rs` | Pending |
| `pg_crud_common/src/std_duration_range_length.rs` | Reviewed: A44 validates microsecond precision and PostgreSQL interval day capacity; binary encoding is tested without a client. |
| `pg_crud_common/src/std_duration_range_length_error.rs` | Reviewed: typed duration validation error. |
| `pg_crud_common/src/string_test_cases_vec.rs` | Pending |
| `pg_crud_common/src/take_first_duplicate.rs` | Pending |
| `pg_crud_common/src/take_first_duplicate_by.rs` | Pending |
| `pg_crud_common/src/take_first_duplicate_by_hash.rs` | Pending |
| `pg_crud_common/src/test_domain_types_db_schema_conformance_tests.rs` | Pending |
| `pg_crud_common/src/test_domain_types_query_pagination_tests.rs` | Pending |
| `pg_crud_common/src/test_explicit_value_serializes_with_full_field_name.rs` | Pending |
| `pg_crud_common/src/test_explicit_value_openapi_contract.rs` | Reviewed: OpenAPI field and null-schema regressions; A140. |
| `pg_crud_common/src/test_order_serializes_with_full_variant_names.rs` | Reviewed: serialization and both fixed case spellings for every Order variant. |
| `pg_crud_common/src/test_pg_type_where_serializes_and_deserializes_with_full_field_name.rs` | Pending |
| `pg_crud_common/src/test_tests_domain_types_operator_to_query_part.rs` | Pending |
| `pg_crud_common/src/transaction_failure.rs` | Pending |
| `pg_crud_common/src/try_new_unique_vec.rs` | Pending |
| `pg_crud_common/src/u16_test_cases_vec.rs` | Pending |
| `pg_crud_common/src/u32_test_cases_vec.rs` | Pending |
| `pg_crud_common/src/u64_test_cases_vec.rs` | Pending |
| `pg_crud_common/src/u8_test_cases_vec.rs` | Pending |
| `pg_crud_common/src/unique_vec_error.rs` | Pending |
| `pg_crud_common/src/unique_vec_len.rs` | Pending |
| `pg_crud_common/src/unit_interval_f64.rs` | Pending |
| `pg_crud_common/src/unit_interval_f64_error.rs` | Pending |
| `pg_crud_common/src/unsigned_part_of_i32.rs` | Pending |
| `pg_crud_common/src/unsigned_part_of_i32_raw.rs` | Pending |
| `pg_crud_common/src/unsigned_part_of_i32_try_from_i32_error.rs` | Pending |
| `pg_crud_common/src/uuid_uuid_test_cases.rs` | Pending |
| `pg_crud_common/src/uuid_uuid_test_cases_vec.rs` | Pending |
| `pg_crud_common/src/validate_batch_by_key.rs` | Reviewed: duplicate policies, invalid limits, processed count, and early-stop behavior inspected with existing deterministic tests. |
| `pg_crud_common/src/validate_bulk_atomicity.rs` | Pending |
| `pg_crud_common/src/validate_generated_postgres_table.rs` | Pending |
| `pg_crud_common/src/validate_migration_idempotency.rs` | Pending |
| `pg_crud_common/src/validate_operation_budget.rs` | Pending |
| `pg_crud_common/src/validate_pagination_invariants.rs` | Pending |
| `pg_crud_common/src/validate_pg_relation_capacity.rs` | Pending |
| `pg_crud_common/src/validate_postgres_catalog.rs` | Pending |
| `pg_crud_common/src/validate_postgres_table_extensions.rs` | Pending |
| `pg_crud_common/src/validate_postgres_table_schema.rs` | Pending |
| `pg_crud_common/src/validate_snapshot.rs` | Pending |
| `pg_crud_common/src/window_total_presence.rs` | Pending |

### pg_crud_macro_common

| Source | Semantic review |
| --- | --- |
| `pg_crud_macro_common/src/common_d_token_stream_builder.rs` | Pending |
| `pg_crud_macro_common/src/de_len.rs` | Pending |
| `pg_crud_macro_common/src/default_some_one_or_default_some_one_with_max_page_size.rs` | Pending |
| `pg_crud_macro_common/src/derive_or_impl.rs` | Pending |
| `pg_crud_macro_common/src/dimension.rs` | Pending |
| `pg_crud_macro_common/src/dimension_index_number.rs` | Pending |
| `pg_crud_macro_common/src/dimension_number.rs` | Pending |
| `pg_crud_macro_common/src/emission_types.rs` | Reviewed: boolean token enums choose existing typed naming values or quote fragments; compiled A13 regression covers nested delimiters and both branch emissions. |
| `pg_crud_macro_common/src/eq_operator_variant.rs` | Pending |
| `pg_crud_macro_common/src/eq_or_eq_using_fields.rs` | Pending |
| `pg_crud_macro_common/src/error_enum_d_token_stream_builder.rs` | Pending |
| `pg_crud_macro_common/src/generate_de_double_quoted_token_stream.rs` | Pending |
| `pg_crud_macro_common/src/generate_dimension_number_pagination_token_stream.rs` | Pending |
| `pg_crud_macro_common/src/generate_explicit_value_declaration_token_stream.rs` | Pending |
| `pg_crud_macro_common/src/generate_explicit_value_initialization_token_stream.rs` | Pending |
| `pg_crud_macro_common/src/generate_if_let_some_match_ok_assign_query_or_return_err_token_stream.rs` | Pending |
| `pg_crud_macro_common/src/generate_impl_all_variants_default_some_one_element_max_page_size_token_stream.rs` | Pending |
| `pg_crud_macro_common/src/generate_impl_all_variants_default_some_one_element_token_stream.rs` | Pending |
| `pg_crud_macro_common/src/generate_impl_crate_is_string_empty_for_identifier_token_stream.rs` | Pending |
| `pg_crud_macro_common/src/generate_impl_de_for_struct_by_fields_token_stream.rs` | Pending |
| `pg_crud_macro_common/src/generate_impl_de_for_struct_token_stream.rs` | Pending |
| `pg_crud_macro_common/src/generate_impl_default_some_one_element_max_page_size_token_stream.rs` | Pending |
| `pg_crud_macro_common/src/generate_impl_default_some_one_element_token_stream.rs` | Pending |
| `pg_crud_macro_common/src/generate_impl_display_and_to_err_string_debug_token_stream.rs` | Pending |
| `pg_crud_macro_common/src/generate_impl_pg_crud_all_variants_default_some_one_element_max_page_size_token_stream.rs` | Pending |
| `pg_crud_macro_common/src/generate_impl_pg_crud_all_variants_default_some_one_element_token_stream.rs` | Pending |
| `pg_crud_macro_common/src/generate_impl_pg_crud_common_all_variants_default_some_one_element_token_stream.rs` | Pending |
| `pg_crud_macro_common/src/generate_impl_pg_crud_common_default_some_one_element_max_page_size_token_stream.rs` | Pending |
| `pg_crud_macro_common/src/generate_impl_pg_crud_common_default_some_one_element_token_stream.rs` | Pending |
| `pg_crud_macro_common/src/generate_impl_pg_crud_default_some_one_element_max_page_size_token_stream.rs` | Pending |
| `pg_crud_macro_common/src/generate_impl_pg_crud_default_some_one_element_token_stream.rs` | Pending |
| `pg_crud_macro_common/src/generate_impl_pg_type_not_primary_key_for_identifier_token_stream.rs` | Pending |
| `pg_crud_macro_common/src/generate_impl_pg_type_test_cases_for_identifier_token_stream.rs` | Pending |
| `pg_crud_macro_common/src/generate_impl_pg_type_token_stream.rs` | Pending |
| `pg_crud_macro_common/src/generate_impl_sqlx_decode_sqlx_pg_for_identifier_token_stream.rs` | Pending |
| `pg_crud_macro_common/src/generate_impl_sqlx_encode_sqlx_pg_for_identifier_token_stream.rs` | Pending |
| `pg_crud_macro_common/src/generate_impl_sqlx_type_and_encode_for_identifier_token_stream.rs` | Pending |
| `pg_crud_macro_common/src/generate_impl_sqlx_type_for_identifier_token_stream.rs` | Pending |
| `pg_crud_macro_common/src/generate_impl_to_err_string_no_generics_token_stream.rs` | Pending |
| `pg_crud_macro_common/src/generate_match_not_empty_unique_vec_try_new_some_or_none_token_stream.rs` | Pending |
| `pg_crud_macro_common/src/generate_match_ok_assign_or_return_err_token_stream.rs` | Pending |
| `pg_crud_macro_common/src/generate_match_ok_or_return_err_token_stream.rs` | Pending |
| `pg_crud_macro_common/src/generate_match_try_new_in_de_token_stream.rs` | Pending |
| `pg_crud_macro_common/src/generate_mod_with_pub_use_token_stream.rs` | Pending |
| `pg_crud_macro_common/src/generate_optional_type_declaration_token_stream.rs` | Pending |
| `pg_crud_macro_common/src/generate_pg_type_where_token_stream.rs` | Pending |
| `pg_crud_macro_common/src/generate_query_part_error_write_into_buffer_token_stream.rs` | Pending |
| `pg_crud_macro_common/src/generate_read_ids_and_create_into_vec_where_eq_using_fields_token_stream.rs` | Pending |
| `pg_crud_macro_common/src/generate_read_ids_and_create_into_where_eq_token_stream.rs` | Pending |
| `pg_crud_macro_common/src/generate_read_inner_into_read_or_update_with_new_or_try_new_unwraped_token_stream.rs` | Pending |
| `pg_crud_macro_common/src/generate_return_err_query_part_error_write_into_buffer_token_stream.rs` | Pending |
| `pg_crud_macro_common/src/generate_sqlx_types_json_type_declaration_token_stream.rs` | Pending |
| `pg_crud_macro_common/src/generate_struct_identifier_double_quoted_token_stream.rs` | Pending |
| `pg_crud_macro_common/src/generate_struct_identifier_with_number_elements_double_quoted_token_stream.rs` | Pending |
| `pg_crud_macro_common/src/generate_vec_tokens_declaration_token_stream.rs` | Pending |
| `pg_crud_macro_common/src/impl_pg_type_eq_operator_for_identifier_token_stream.rs` | Pending |
| `pg_crud_macro_common/src/impl_pg_type_where_filter_for_identifier_token_stream.rs` | Pending |
| `pg_crud_macro_common/src/import.rs` | Pending |
| `pg_crud_macro_common/src/import_path_str.rs` | Pending |
| `pg_crud_macro_common/src/import_snake_case_str.rs` | Pending |
| `pg_crud_macro_common/src/is_nl_prefix_str_max_len.rs` | Pending |
| `pg_crud_macro_common/src/is_nullable.rs` | Pending |
| `pg_crud_macro_common/src/is_nullable_prefix_str.rs` | Pending |
| `pg_crud_macro_common/src/is_standard_non_null.rs` | Pending |
| `pg_crud_macro_common/src/lib.rs` | Pending |
| `pg_crud_macro_common/src/maybe_wrap_into_braces_token_stream.rs` | Pending |
| `pg_crud_macro_common/src/names_context.rs` | Pending |
| `pg_crud_macro_common/src/non_null_or_nullable_str.rs` | Pending |
| `pg_crud_macro_common/src/panic_uuid_ref.rs` | Pending |
| `pg_crud_macro_common/src/parse_error_id_ref.rs` | Pending |
| `pg_crud_macro_common/src/parse_strs_to_ts2_vec.rs` | Pending |
| `pg_crud_macro_common/src/parse_token_stream_strings.rs` | Pending |
| `pg_crud_macro_common/src/pg_crud_common_query_part_error_checked_add_initialization_token_stream.rs` | Pending |
| `pg_crud_macro_common/src/pg_crud_common_query_part_error_token_stream.rs` | Pending |
| `pg_crud_macro_common/src/pg_filter.rs` | Pending |
| `pg_crud_macro_common/src/pg_type_filter.rs` | Focused review: A44 carries the range-specific length type through the filter descriptor. |
| `pg_crud_macro_common/src/proc_macro2_generated_rust_token_stream_vec.rs` | Pending |
| `pg_crud_macro_common/src/read_or_update.rs` | Pending |
| `pg_crud_macro_common/src/serde_error_enum_d_token_stream_builder.rs` | Pending |
| `pg_crud_macro_common/src/struct_elements_length.rs` | Pending |
| `pg_crud_macro_common/src/syn_field_refs.rs` | Pending |
| `pg_crud_macro_common/src/syn_identifier_type_refs.rs` | Pending |
| `pg_crud_macro_common/src/test_domain_types_token_emission_tests.rs` | Pending |
| `pg_crud_macro_common/src/test_tests_domain_types.rs` | Pending |
| `pg_crud_macro_common/src/wrap_into_braces.rs` | Pending |
| `pg_crud_macro_common/src/wrap_into_scopes_token_stream.rs` | Pending |

### pg_crud_pg_table

| Source | Semantic review |
| --- | --- |
| `pg_crud_pg_table/src/add_update_optimistic_revision_predicate.rs` | Reviewed: A14 now returns a typed overflow error and preserves the existing query when no RETURNING clause is present. |
| `pg_crud_pg_table/src/begin_pg_table_idempotency.rs` | Focused review: reservation, hash conflict and replay branches inspected; concurrent cleanup interactions remain for integration review. |
| `pg_crud_pg_table/src/calculate_pg_table_idempotency_request_hash.rs` | Reviewed: SHA-256 body digest is copied into a fixed-size wrapper. |
| `pg_crud_pg_table/src/cleanup_pg_table_idempotency.rs` | Focused review: bounded cleanup query and affected-row conversion inspected; database behavior remains under integration review. |
| `pg_crud_pg_table/src/combination_of_app_state_logic_traits.rs` | Reviewed: explicit provider bounds combine API configuration, database and resource-budget capabilities. |
| `pg_crud_pg_table/src/complete_pg_table_idempotency.rs` | Reviewed: A32 now rejects completion when SQL updates no pending reservation. |
| `pg_crud_pg_table/src/complete_pg_table_idempotency_in_connection.rs` | Reviewed: transactional completion already requires exactly one updated reservation. |
| `pg_crud_pg_table/src/ensure_pg_table_idempotency_schema.rs` | Focused review: advisory lock and schema statements execute within one transaction; database behavior remains under integration review. |
| `pg_crud_pg_table/src/generate_cm_query_string.rs` | Reviewed: SQL assembly returns typed length errors; A14. |
| `pg_crud_pg_table/src/generate_column_eqs_case_accumulator_else_column_end_comma_um_query_part.rs` | Reviewed: update fragment returns typed length errors; A14. |
| `pg_crud_pg_table/src/generate_dm_query_string.rs` | Reviewed: SQL assembly returns typed length errors; A14. |
| `pg_crud_pg_table/src/generate_rm_query_string.rs` | Reviewed: SQL assembly returns typed length errors; A14. |
| `pg_crud_pg_table/src/generate_um_query_string.rs` | Reviewed: SQL assembly returns typed length errors; A14. |
| `pg_crud_pg_table/src/generate_when_column_id_then_v_um_query_part.rs` | Reviewed: update fragment returns typed length errors; A14. |
| `pg_crud_pg_table/src/lib.rs` | Reviewed: flat module declarations match present source owners; no root logic. |
| `pg_crud_pg_table/src/new_pg_table_idempotency_key.rs` | Reviewed: UUID generation delegates validated conversion and propagates its typed Result; both generated clients updated for A05. |
| `pg_crud_pg_table/src/pg_table_idempotency_actor.rs` | Reviewed: validates nonempty bounded actor text through the shared validator. |
| `pg_crud_pg_table/src/pg_table_idempotency_begin.rs` | Reviewed: acquisition, conflict, progress and replay are distinct outcomes. |
| `pg_crud_pg_table/src/pg_table_idempotency_body.rs` | Reviewed: inclusive 1 MiB bound; A33 preserves the bounded-vector error source. |
| `pg_crud_pg_table/src/pg_table_idempotency_body_error.rs` | Reviewed: domain-level size error retains its underlying bounded-value source; A33. |
| `pg_crud_pg_table/src/pg_table_idempotency_body_ref.rs` | Reviewed: borrowed byte-slice wrapper for request and response bodies. |
| `pg_crud_pg_table/src/pg_table_idempotency_cleanup_batch_size.rs` | Reviewed: rejects zero and negative batch sizes. |
| `pg_crud_pg_table/src/pg_table_idempotency_cleanup_retention_seconds.rs` | Reviewed: rejects negative retention, permits immediate zero retention. |
| `pg_crud_pg_table/src/pg_table_idempotency_cleanup_rows.rs` | Reviewed: affected-row count wrapper. |
| `pg_crud_pg_table/src/pg_table_idempotency_cleanup_value_try_from_i64_error.rs` | Reviewed: distinct negative and nonpositive validation errors. |
| `pg_crud_pg_table/src/pg_table_idempotency_key.rs` | Reviewed: nonempty bounded byte storage and validated common text conversion; A05 callers now propagate its typed failure. |
| `pg_crud_pg_table/src/pg_table_idempotency_known_response_status.rs` | Reviewed: fixed internal-server-error status source. |
| `pg_crud_pg_table/src/pg_table_idempotency_method.rs` | Reviewed: accepts only POST, PATCH and DELETE with bounded storage. |
| `pg_crud_pg_table/src/pg_table_idempotency_replay.rs` | Reviewed: ownership transfer preserves status and bounded response body. |
| `pg_crud_pg_table/src/pg_table_idempotency_request.rs` | Reviewed: scope and body hash are constructed together. |
| `pg_crud_pg_table/src/pg_table_idempotency_request_hash.rs` | Reviewed: fixed 32-byte digest wrapper. |
| `pg_crud_pg_table/src/pg_table_idempotency_response_status.rs` | Reviewed: accepts three-digit HTTP status values; i16 conversion remains in completion owner. |
| `pg_crud_pg_table/src/pg_table_idempotency_response_status_try_from_u16_error.rs` | Reviewed: out-of-range status error. |
| `pg_crud_pg_table/src/pg_table_idempotency_route.rs` | Reviewed: requires a leading slash and at most 1024 bytes. |
| `pg_crud_pg_table/src/pg_table_idempotency_scope.rs` | Reviewed: typed actor, method, route and key fields. |
| `pg_crud_pg_table/src/pg_table_idempotency_text_bytes.rs` | Reviewed: byte-length diagnostic wrapper. |
| `pg_crud_pg_table/src/pg_table_idempotency_text_error.rs` | Reviewed: domain enum retains empty, shape and length meaning; diagnostic trait conversion added for generated A05 client errors and covered by an existing unit test. |
| `pg_crud_pg_table/src/pg_table_name_ref.rs` | Focused review: SQL assembly and length-error handling; see A14 and generated consumers. |
| `pg_crud_pg_table/src/pg_table_query_part_fragment.rs` | Reviewed: bounded storage and inclusive byte limit; removed error-to-success conversion under A14. |
| `pg_crud_pg_table/src/pg_table_query_string.rs` | Reviewed: bounded storage and inclusive byte limit; removed error-to-success conversion under A14. |
| `pg_crud_pg_table/src/pg_table_revision.rs` | Reviewed: parses signed decimal text and rejects negative revisions. |
| `pg_crud_pg_table/src/pg_table_revision_parse_int_error.rs` | Reviewed: retains integer parse source. |
| `pg_crud_pg_table/src/pg_table_revision_try_from_string_error.rs` | Reviewed: distinguishes invalid text from negative revision. |
| `pg_crud_pg_table/src/pg_table_sql_fragment_ref.rs` | Focused review: SQL assembly and length-error handling; see A14 and generated consumers. |
| `pg_crud_pg_table/src/pg_table_string_wrapper_try_from_string_error.rs` | Reviewed: typed length error now derives thiserror and implements generated route diagnostic conversion; A14. |
| `pg_crud_pg_table/src/pg_tbl_idempotency_route_max_bytes.rs` | Reviewed: matches route wrapper's 1024-byte bound. |
| `pg_crud_pg_table/src/pg_tbl_idempotency_text_max_bytes.rs` | Reviewed: matches actor and key text's 255-byte bound. |
| `pg_crud_pg_table/src/pg_tbl_string_wrapper_max_len.rs` | Reviewed: matches the query and fragment 1 MiB storage limit. |
| `pg_crud_pg_table/src/release_pg_table_idempotency.rs` | Focused review: releases matching pending reservation by request hash; zero affected rows remain valid for idempotent release. |
| `pg_crud_pg_table/src/sqlx_pg_table_idempotency_error.rs` | Reviewed: retains SQLx source and exposes a bounded diagnostic conversion. |
| `pg_crud_pg_table/src/sqlx_pg_table_pg_connection_ref.rs` | Reviewed: mutable borrowed PostgreSQL connection adapter. |
| `pg_crud_pg_table/src/test_pg_crud_pg_table.rs` | Focused review: A14 tests successful SQL and typed overflow from seven query and fragment builders. |
| `pg_crud_pg_table/src/test_tests_domain_types_idempotency.rs` | Focused review: A33 boundary regression checks preserved bounded-value length source; other idempotency domain tests remain under review. |
| `pg_crud_pg_table/src/validate_pg_table_idempotency_text.rs` | Reviewed: rejects empty and oversized text before bounded conversion; matching minimum/maximum errors preserve the relevant lengths. |

### pg_crud_pg_table_generate_src

| Source | Semantic review |
| --- | --- |
| `pg_crud_pg_table_generate_src/src/build_generate_pg_table.rs` | Reviewed: requires struct shape and records field count before validation. |
| `pg_crud_pg_table_generate_src/src/emit_generate_pg_table.rs` | Focused review: A23 propagates user-provided table identifier case-conversion errors; A14 routes query and fragment length errors through each operation's QueryString HTTP 400 variant; A44 and A45 register all three typed range-length schemas. Fixed local enum conversions and the rest of this large generator remain under review. |
| `pg_crud_pg_table_generate_src/src/generate_pg_table.rs` | Reviewed: dispatches staged validation and turns typed stage errors into compiler diagnostics. |
| `pg_crud_pg_table_generate_src/src/generate_pg_table_field_count.rs` | Reviewed: typed field-count wrapper. |
| `pg_crud_pg_table_generate_src/src/generate_pg_table_model.rs` | Reviewed: rejects empty struct models before emission. |
| `pg_crud_pg_table_generate_src/src/generate_pg_table_pipeline_error.rs` | Reviewed: parse, build and validation failures remain distinct. |
| `pg_crud_pg_table_generate_src/src/idempotency_capability.rs` | Reviewed: boolean capability conversion preserves both states. |
| `pg_crud_pg_table_generate_src/src/idempotency_capable.rs` | Reviewed: projects the descriptor's idempotency flag. |
| `pg_crud_pg_table_generate_src/src/lib.rs` | Reviewed: flat module declarations match source owners. |
| `pg_crud_pg_table_generate_src/src/operation_descriptor.rs` | Reviewed: transport, rule and capability fields have generated getters. |
| `pg_crud_pg_table_generate_src/src/optimistic_concurrency_capability.rs` | Reviewed: boolean capability conversion preserves both states. |
| `pg_crud_pg_table_generate_src/src/optimistic_concurrency_capable.rs` | Reviewed: projects the independent optimistic-concurrency flag. |
| `pg_crud_pg_table_generate_src/src/parse_generate_pg_table.rs` | Reviewed: syn parse failure retains its source in the parse-stage error. |
| `pg_crud_pg_table_generate_src/src/pg_table_compile_error_message.rs` | Reviewed: borrowed compiler-message wrapper. |
| `pg_crud_pg_table_generate_src/src/pg_table_compile_error_tokens.rs` | Reviewed: emits a compile_error token from a borrowed message. |
| `pg_crud_pg_table_generate_src/src/route_http_method.rs` | Reviewed: copies typed HTTP method from descriptor. |
| `pg_crud_pg_table_generate_src/src/route_success_status.rs` | Reviewed: copies typed success status from descriptor. |
| `pg_crud_pg_table_generate_src/src/struct_shape.rs` | Reviewed: delegates struct-shape parsing without discarding syn errors. |
| `pg_crud_pg_table_generate_src/src/success_status.rs` | Reviewed: delegates typed status projection. |
| `pg_crud_pg_table_generate_src/src/syn_built_generate_pg_table_input.rs` | Reviewed: typed built-model stage wrapper. |
| `pg_crud_pg_table_generate_src/src/syn_generate_pg_table_model_error.rs` | Reviewed: typed internal syn model-error wrapper. |
| `pg_crud_pg_table_generate_src/src/syn_generate_pg_table_model_input.rs` | Reviewed: typed syn input wrapper. |
| `pg_crud_pg_table_generate_src/src/syn_generate_pg_table_pipeline_error.rs` | Reviewed: transparent syn stage-error source. |
| `pg_crud_pg_table_generate_src/src/syn_parsed_generate_pg_table_input.rs` | Reviewed: typed parsed-input stage wrapper. |
| `pg_crud_pg_table_generate_src/src/syn_validated_generate_pg_table_input.rs` | Reviewed: typed validated-input stage wrapper. |
| `pg_crud_pg_table_generate_src/src/table_test_names.rs` | Reviewed: exactly-four-name cardinality validated by TryFrom. |
| `pg_crud_pg_table_generate_src/src/test_pg_crud_pg_table_generate_src.rs` | Focused review: pipeline stage tests cover non-struct and empty input; full emitter behavior remains under review. |
| `pg_crud_pg_table_generate_src/src/validate_generate_pg_table.rs` | Reviewed: applies typed nonempty-model validation before emission. |

### pg_crud_pg_table_generate_test

| Source | Semantic review |
| --- | --- |
| `pg_crud_pg_table_generate_test/src/lib.rs` | Reviewed: generated table fixture, option-rejection checks, metrics labels, deterministic output, and compile gate inspected; no defect confirmed. |

### pg_crud_pg_types_chrono_net

| Source | Semantic review |
| --- | --- |
| `pg_crud_pg_types_chrono_net/src/lib.rs` | Reviewed: macro configuration and generated type subset; generated implementation is tracked at the emitter. |

### pg_crud_pg_types_common

| Source | Semantic review |
| --- | --- |
| `pg_crud_pg_types_common/src/is_primary_key.rs` | Reviewed: typed primary-key flag preserves shared database flag. |
| `pg_crud_pg_types_common/src/lib.rs` | Reviewed: flat declarations match seven source owners. |
| `pg_crud_pg_types_common/src/maybe_primary_key.rs` | Reviewed: emits the primary-key suffix only for a true typed flag. |
| `pg_crud_pg_types_common/src/pagination_starts_with_one.rs` | Reviewed: A34 fixes default offset; typed construction checks positive limit, one-based offset and checked end. |
| `pg_crud_pg_types_common/src/pagination_starts_with_one_raw.rs` | Reviewed: deserialization passes limit and offset through validated conversion. |
| `pg_crud_pg_types_common/src/pagination_starts_with_one_try_new_error.rs` | Reviewed: distinct limit, offset and overflow errors retain location. |
| `pg_crud_pg_types_common/src/pagination_starts_with_one_value.rs` | Reviewed: typed signed pagination value wrapper. |

### pg_crud_pg_types_generate_src

| Source | Semantic review |
| --- | --- |
| `pg_crud_pg_types_generate_src/src/build_generate_pg_types.rs` | Reviewed: parse, build, validation and diagnostic pipeline; emitter behavior remains separate. |
| `pg_crud_pg_types_generate_src/src/built_generate_pg_types_model.rs` | Reviewed: generated-type configuration or typed metadata helper. |
| `pg_crud_pg_types_generate_src/src/can_be_nullable.rs` | Reviewed: generated-type configuration or typed metadata helper. |
| `pg_crud_pg_types_generate_src/src/can_be_primary_key.rs` | Reviewed: generated-type configuration or typed metadata helper. |
| `pg_crud_pg_types_generate_src/src/contract_tests.rs` | Reviewed: generated-type configuration or typed metadata helper. |
| `pg_crud_pg_types_generate_src/src/emit_generate_pg_types.rs` | Focused review: A35, A44, A45 range-specific length-type selection, and A105 shared timestamp conversion. |
| `pg_crud_pg_types_generate_src/src/filter_kind.rs` | Reviewed: generated-type configuration or typed metadata helper. |
| `pg_crud_pg_types_generate_src/src/generate_pg_type_records.rs` | Reviewed: bounded input list and typed length rejection. |
| `pg_crud_pg_types_generate_src/src/generate_pg_types.rs` | Reviewed: bounded input list and typed length rejection. |
| `pg_crud_pg_types_generate_src/src/generate_pg_types_config.rs` | Reviewed: generated-type configuration or typed metadata helper. |
| `pg_crud_pg_types_generate_src/src/generate_pg_types_config_variant.rs` | Reviewed: generated-type configuration or typed metadata helper. |
| `pg_crud_pg_types_generate_src/src/generate_pg_types_length_error.rs` | Reviewed: bounded input list and typed length rejection. |
| `pg_crud_pg_types_generate_src/src/generate_pg_types_max_len.rs` | Reviewed: bounded input list and typed length rejection. |
| `pg_crud_pg_types_generate_src/src/generate_pg_types_pipeline_error.rs` | Reviewed: parse, build, validation and diagnostic pipeline; emitter behavior remains separate. |
| `pg_crud_pg_types_generate_src/src/generate_pg_types_tokens.rs` | Reviewed: parse, build, validation and diagnostic pipeline; emitter behavior remains separate. |
| `pg_crud_pg_types_generate_src/src/generate_secret_text.rs` | Reviewed: generated-type configuration or typed metadata helper. |
| `pg_crud_pg_types_generate_src/src/lib.rs` | Reviewed: generated-type configuration or typed metadata helper. |
| `pg_crud_pg_types_generate_src/src/parse_generate_pg_types.rs` | Reviewed: parse, build, validation and diagnostic pipeline; emitter behavior remains separate. |
| `pg_crud_pg_types_generate_src/src/parsed_generate_pg_types_config.rs` | Reviewed: generated-type configuration or typed metadata helper. |
| `pg_crud_pg_types_generate_src/src/pg_name.rs` | Reviewed: generated-type configuration or typed metadata helper. |
| `pg_crud_pg_types_generate_src/src/pg_sql_name.rs` | Reviewed: complete catalog capabilities and SQL names mapped per variant. |
| `pg_crud_pg_types_generate_src/src/pg_type_can_be_nullable.rs` | Reviewed: generated-type configuration or typed metadata helper. |
| `pg_crud_pg_types_generate_src/src/pg_type_catalog_kind.rs` | Reviewed: catalog to SQL, Rust and wire classification dispatch. |
| `pg_crud_pg_types_generate_src/src/pg_type_deserialize.rs` | Reviewed: catalog to SQL, Rust and wire classification dispatch. |
| `pg_crud_pg_types_generate_src/src/pg_type_impl_new_for_deserialize_or_try_new_for_de.rs` | Reviewed: generated-type configuration or typed metadata helper. |
| `pg_crud_pg_types_generate_src/src/pg_type_impl_try_new_for_de.rs` | Reviewed: generated-type configuration or typed metadata helper. |
| `pg_crud_pg_types_generate_src/src/pg_type_initialization_try_new.rs` | Reviewed: finite float4 and float8 initialization dispatch; A35. |
| `pg_crud_pg_types_generate_src/src/pg_type_name.rs` | Reviewed: catalog to SQL, Rust and wire classification dispatch. |
| `pg_crud_pg_types_generate_src/src/pg_type_pattern.rs` | Reviewed: generated-type configuration or typed metadata helper. |
| `pg_crud_pg_types_generate_src/src/pg_type_record.rs` | Reviewed: nullable-capability validation of concrete records. |
| `pg_crud_pg_types_generate_src/src/pg_type_record_raw.rs` | Reviewed: nullable-capability validation of concrete records. |
| `pg_crud_pg_types_generate_src/src/pg_type_spec.rs` | Reviewed: generated-type configuration or typed metadata helper. |
| `pg_crud_pg_types_generate_src/src/pg_types_model_entry_count.rs` | Reviewed: generated-type configuration or typed metadata helper. |
| `pg_crud_pg_types_generate_src/src/range.rs` | Reviewed: catalog to SQL, Rust and wire classification dispatch. |
| `pg_crud_pg_types_generate_src/src/rust_type_name.rs` | Reviewed: catalog to SQL, Rust and wire classification dispatch. |
| `pg_crud_pg_types_generate_src/src/rust_type_wire_kind.rs` | Reviewed: generated-type configuration or typed metadata helper. |
| `pg_crud_pg_types_generate_src/src/schema_wire_kind.rs` | Reviewed: generated-type configuration or typed metadata helper. |
| `pg_crud_pg_types_generate_src/src/serde_json_generate_pg_types_error.rs` | Reviewed: generated-type configuration or typed metadata helper. |
| `pg_crud_pg_types_generate_src/src/test_pg_crud_pg_types_generate_src.rs` | Reviewed: generated-type configuration or typed metadata helper. |
| `pg_crud_pg_types_generate_src/src/validate_generate_pg_types.rs` | Reviewed: parse, build, validation and diagnostic pipeline; emitter behavior remains separate. |
| `pg_crud_pg_types_generate_src/src/validated_generate_pg_types_config.rs` | Reviewed: generated-type configuration or typed metadata helper. |
| `pg_crud_pg_types_generate_src/src/wire_kind.rs` | Reviewed: catalog to SQL, Rust and wire classification dispatch. |

### pg_crud_pg_types_generate_test

| Source | Semantic review |
| --- | --- |
| `pg_crud_pg_types_generate_test/src/lib.rs` | Focused review: A35 |

### pg_crud_pg_types_numeric

| Source | Semantic review |
| --- | --- |
| `pg_crud_pg_types_numeric/src/lib.rs` | Reviewed: macro configuration and generated type subset; generated implementation is tracked at the emitter. |

### pg_crud_pg_types_text_misc

| Source | Semantic review |
| --- | --- |
| `pg_crud_pg_types_text_misc/src/lib.rs` | Reviewed: macro configuration and generated type subset; generated implementation is tracked at the emitter. |

### pg_crud_where_filters

| Source | Semantic review |
| --- | --- |
| `pg_crud_where_filters/src/between.rs` | Focused review: A38 fixes inclusive range validation and default invariant; serialization, deserialization and query binding inspected. |
| `pg_crud_where_filters/src/between_try_new_error.rs` | Reviewed: A38 error variant covers descending and unordered bounds. |
| `pg_crud_where_filters/src/bounded_vec_try_new_error.rs` | Reviewed: exact-length error fields. |
| `pg_crud_where_filters/src/default_regex_pattern.rs` | Reviewed: validated default regex marker. |
| `pg_crud_where_filters/src/domain_types.rs` | Reviewed: generated filter invocation and configuration. |
| `pg_crud_where_filters/src/encode_format.rs` | Reviewed: bytea encoding variants and display values. |
| `pg_crud_where_filters/src/lib.rs` | Reviewed: module declarations and generated filter entrypoint. |
| `pg_crud_where_filters/src/pg_filter_vec.rs` | Focused review: A36 fixes Default length invariant; query fragment and binding paths also inspected. |
| `pg_crud_where_filters/src/pg_filter_vec_len.rs` | Reviewed: length diagnostic wrapper. |
| `pg_crud_where_filters/src/pg_type_not_empty_unique_vec.rs` | Focused review: A37 fixes Default nonempty invariant; uniqueness conversion and deserialization inspected. |
| `pg_crud_where_filters/src/regex_case.rs` | Reviewed: case-sensitive and insensitive SQL operator mapping. |
| `pg_crud_where_filters/src/regex_case_postgreql_syntax.rs` | Reviewed: static SQL operator wrapper. |
| `pg_crud_where_filters/src/regex_regex.rs` | Reviewed: A39 removes cross-dialect syntax validation; bounded storage and validated serde conversion retained. |
| `pg_crud_where_filters/src/regex_regex_try_from_string_error.rs` | Reviewed: A39 leaves only the overlong-pattern error. |
| `pg_crud_where_filters/src/test_pg_crud_where_filters.rs` | Focused review: A36-A39 regressions. |
| `pg_crud_where_filters/src/variant.rs` | Reviewed: normal and minus-one query fragment selector. |

### pg_crud_where_filters_generate_src

| Source | Semantic review |
| --- | --- |
| `pg_crud_where_filters_generate_src/src/bind_count.rs` | Reviewed: typed bind count stored in the filter catalog. |
| `pg_crud_where_filters_generate_src/src/bind_count_matches.rs` | Reviewed: descriptor count comparison. |
| `pg_crud_where_filters_generate_src/src/build_generate_where_filters.rs` | Reviewed: all catalog entries checked for bind and value-shape consistency. |
| `pg_crud_where_filters_generate_src/src/built_generate_where_filters_model.rs` | Reviewed: validated pipeline model fields. |
| `pg_crud_where_filters_generate_src/src/client_uses_text_value.rs` | Reviewed: text-value capability adapter. |
| `pg_crud_where_filters_generate_src/src/emit_generate_where_filters.rs` | Focused review: A40 validates deserialized text search values; A42 and A43 correct bound-inclusivity filters; A44 makes RangeLen type generic; A45 widens integer-range arithmetic; A46 matches local time types; other emitter branches pending. |
| `pg_crud_where_filters_generate_src/src/filter_placeholder_count.rs` | Reviewed: typed one-placeholder count. |
| `pg_crud_where_filters_generate_src/src/filter_spec.rs` | Reviewed: catalog SQL operators, suffix, bind count and value shape; A41 corrects strict range operators. |
| `pg_crud_where_filters_generate_src/src/filter_spec_valid.rs` | Reviewed: typed catalog validation result. |
| `pg_crud_where_filters_generate_src/src/filter_sql_operator.rs` | Reviewed: static operator wrapper. |
| `pg_crud_where_filters_generate_src/src/filter_sql_operator_value.rs` | Reviewed: operator catalog accessor. |
| `pg_crud_where_filters_generate_src/src/filter_sql_suffix.rs` | Reviewed: static SQL suffix wrapper. |
| `pg_crud_where_filters_generate_src/src/filter_sql_suffix_value.rs` | Reviewed: suffix catalog accessor. |
| `pg_crud_where_filters_generate_src/src/generate_where_filters_pipeline_error.rs` | Reviewed: parse and invalid-contract errors. |
| `pg_crud_where_filters_generate_src/src/generate_where_filters_source.rs` | Reviewed: parse/build/validate/emit pipeline and compiler diagnostic conversion. |
| `pg_crud_where_filters_generate_src/src/lib.rs` | Reviewed: module ownership and exports. |
| `pg_crud_where_filters_generate_src/src/parse_generate_where_filters.rs` | Reviewed: JSON config parsing and source preservation. |
| `pg_crud_where_filters_generate_src/src/parsed_generate_where_filters_config.rs` | Reviewed: typed write-to-file options. |
| `pg_crud_where_filters_generate_src/src/pg_filter_value_shape.rs` | Reviewed: scalar and text shape variants. |
| `pg_crud_where_filters_generate_src/src/proc_macro2_generate_where_filters_input.rs` | Reviewed: borrowed proc-macro input wrapper. |
| `pg_crud_where_filters_generate_src/src/proc_macro2_generate_where_filters_token_stream.rs` | Reviewed: generated token wrapper. |
| `pg_crud_where_filters_generate_src/src/schema_uses_text_value.rs` | Reviewed: schema text-value capability adapter. |
| `pg_crud_where_filters_generate_src/src/serde_json_generate_where_filters_error.rs` | Reviewed: typed JSON parse source. |
| `pg_crud_where_filters_generate_src/src/source_tests.rs` | Reviewed: config pipeline regression. |
| `pg_crud_where_filters_generate_src/src/spec_tests.rs` | Reviewed: all catalog descriptor invariants. |
| `pg_crud_where_filters_generate_src/src/validate_generate_where_filters.rs` | Reviewed: contract validation and typed error return. |
| `pg_crud_where_filters_generate_src/src/validated_generate_where_filters_config.rs` | Reviewed: validated configuration wrapper. |

### pg_crud_where_filters_generate_test

| Source | Semantic review |
| --- | --- |
| `pg_crud_where_filters_generate_test/src/lib.rs` | Focused review: A40 JSON contracts, A41 strict range SQL fragments, A42 equality bound-inclusivity fragments, A43 greater-than bound-inclusivity fragments, A44 generic range-length fragments, A45 numeric casts and A46 local time predicates. |

### prepare_pg_databases

| Source | Semantic review |
| --- | --- |
| `prepare_pg_databases/src/database_preparation_spec.rs` | Reviewed: typed specification transfers URL and migration source in constructor order. |
| `prepare_pg_databases/src/database_url.rs` | Reviewed: conversion rejects empty or excessive text within its declared byte bound. |
| `prepare_pg_databases/src/database_url_error.rs` | Reviewed: URL length failures use typed variants. |
| `prepare_pg_databases/src/lib.rs` | Reviewed: root declares the migration command and domain modules. |
| `prepare_pg_databases/src/migration_commands.rs` | Reviewed: one input specification produces one SQLx migration command with ordered flags. |
| `prepare_pg_databases/src/migrations_source.rs` | Reviewed: migration source conversion enforces its declared maximum byte length. |
| `prepare_pg_databases/src/migrations_source_error.rs` | Reviewed: source length failure is typed. |
| `prepare_pg_databases/src/process_argument.rs` | Reviewed: typed argument variants expose the corresponding text. |
| `prepare_pg_databases/src/process_arguments.rs` | Reviewed: command arguments retain insertion order. |
| `prepare_pg_databases/src/process_command.rs` | Reviewed: process command stores program and ordered arguments. |
| `prepare_pg_databases/src/process_commands.rs` | Reviewed: generated command collection retains input order. |
| `prepare_pg_databases/src/process_program.rs` | Reviewed: static executable-name wrapper. |
| `prepare_pg_databases/src/process_static_argument.rs` | Reviewed: static flag wrapper exposes its value. |
| `prepare_pg_databases/src/tests_domain_types.rs` | Reviewed: tests cover command shape and empty URL rejection. |

### proc_macro_bool_enum_to_tokens

| Source | Semantic review |
| --- | --- |
| `proc_macro_bool_enum_to_tokens/src/lib.rs` | Reviewed: compiler token adapter delegates expression parsing and generation to the existing shared owner; A13. |

### proc_macro_config_lib_assert_empty_parse_err_matches

| Source | Semantic review |
| --- | --- |
| `proc_macro_config_lib_assert_empty_parse_err_matches/src/lib.rs` | Reviewed: compiler entrypoint attributes and token forwarding inspected; shared implementation review remains independent. |

### proc_macro_config_lib_assert_parse_err_matches

| Source | Semantic review |
| --- | --- |
| `proc_macro_config_lib_assert_parse_err_matches/src/lib.rs` | Reviewed: compiler entrypoint attributes and token forwarding inspected; shared implementation review remains independent. |

### proc_macro_config_lib_assert_parse_ok_matches

| Source | Semantic review |
| --- | --- |
| `proc_macro_config_lib_assert_parse_ok_matches/src/lib.rs` | Reviewed: compiler entrypoint attributes and token forwarding inspected; shared implementation review remains independent. |

### proc_macro_config_lib_impl_try_from_non_empty_string

| Source | Semantic review |
| --- | --- |
| `proc_macro_config_lib_impl_try_from_non_empty_string/src/lib.rs` | Reviewed: compiler entrypoint attributes and token forwarding inspected; shared implementation review remains independent. |

### proc_macro_config_lib_impl_try_from_parse

| Source | Semantic review |
| --- | --- |
| `proc_macro_config_lib_impl_try_from_parse/src/lib.rs` | Reviewed: compiler entrypoint attributes and token forwarding inspected; shared implementation review remains independent. |

### proc_macro_config_lib_impl_try_from_parse_string_error

| Source | Semantic review |
| --- | --- |
| `proc_macro_config_lib_impl_try_from_parse_string_error/src/lib.rs` | Reviewed: compiler entrypoint attributes and token forwarding inspected; shared implementation review remains independent. |

### proc_macro_config_lib_impl_try_from_secret_url

| Source | Semantic review |
| --- | --- |
| `proc_macro_config_lib_impl_try_from_secret_url/src/lib.rs` | Reviewed: compiler entrypoint attributes and token forwarding inspected; shared implementation review remains independent. |

### proc_macro_config_lib_shared

| Source | Semantic review |
| --- | --- |
| `proc_macro_config_lib_shared/src/lib.rs` | Fixed: A63 generates bounded nonempty text and typed overlength errors; remaining generator branches reviewed. |
| `proc_macro_config_lib_shared/src/proc_macro2_try_from_parse_fixed_error_ty.rs` | Reviewed: typed optional generated error token wrapper. |
| `proc_macro_config_lib_shared/src/proc_macro2_try_from_parse_input.rs` | Reviewed: typed parse input token wrapper. |
| `proc_macro_config_lib_shared/src/proc_macro_try_from_parse_token_stream.rs` | Reviewed: typed generated token result wrapper. |

### proc_macro_constants_str_shared

| Source | Semantic review |
| --- | --- |
| `proc_macro_constants_str_shared/src/lib.rs` | Reviewed: block parsing, collection limits, duplicate names and words, unknown references, fragment use counts and quoted output. Rust fragments reference word fragments only, so expansion has no reference cycle. |

### proc_macro_define_git_info_constants

| Source | Semantic review |
| --- | --- |
| `proc_macro_define_git_info_constants/src/lib.rs` | Reviewed: compiler entrypoint attributes and token forwarding inspected; shared implementation review remains independent. |

### proc_macro_define_str_constants

| Source | Semantic review |
| --- | --- |
| `proc_macro_define_str_constants/src/lib.rs` | Reviewed: compiler entrypoint attributes and token forwarding inspected; shared implementation review remains independent. |

### proc_macro_frontend_contract_api_operation_error

| Source | Semantic review |
| --- | --- |
| `proc_macro_frontend_contract_api_operation_error/src/lib.rs` | Reviewed: compiler entrypoint attributes and token forwarding inspected; shared implementation review remains independent. |

### proc_macro_frontend_contract_derive_contract_struct_api

| Source | Semantic review |
| --- | --- |
| `proc_macro_frontend_contract_derive_contract_struct_api/src/lib.rs` | Reviewed: compiler entrypoint attributes and token forwarding inspected; shared implementation review remains independent. |

### proc_macro_frontend_contract_derive_page_catalog

| Source | Semantic review |
| --- | --- |
| `proc_macro_frontend_contract_derive_page_catalog/src/lib.rs` | Reviewed: compiler entrypoint attributes and token forwarding inspected; shared implementation review remains independent. |

### proc_macro_frontend_contract_derive_route_catalog

| Source | Semantic review |
| --- | --- |
| `proc_macro_frontend_contract_derive_route_catalog/src/lib.rs` | Reviewed: compiler entrypoint attributes and token forwarding inspected; shared implementation review remains independent. |

### proc_macro_frontend_contract_derive_route_family

| Source | Semantic review |
| --- | --- |
| `proc_macro_frontend_contract_derive_route_family/src/lib.rs` | Reviewed: compiler entrypoint attributes and token forwarding inspected; shared implementation review remains independent. |

### proc_macro_frontend_contract_derive_typed_route

| Source | Semantic review |
| --- | --- |
| `proc_macro_frontend_contract_derive_typed_route/src/lib.rs` | Reviewed: compiler entrypoint attributes and token forwarding inspected; shared implementation review remains independent. |

### proc_macro_frontend_contract_derive_unit_enum_catalog

| Source | Semantic review |
| --- | --- |
| `proc_macro_frontend_contract_derive_unit_enum_catalog/src/lib.rs` | Reviewed: compiler entrypoint attributes and token forwarding inspected; shared implementation review remains independent. |

### proc_macro_frontend_contract_derive_unit_enum_index

| Source | Semantic review |
| --- | --- |
| `proc_macro_frontend_contract_derive_unit_enum_index/src/lib.rs` | Reviewed: compiler entrypoint attributes and token forwarding inspected; shared implementation review remains independent. |

### proc_macro_frontend_contract_route_error

| Source | Semantic review |
| --- | --- |
| `proc_macro_frontend_contract_route_error/src/lib.rs` | Reviewed: compiler entrypoint attributes and token forwarding inspected; shared implementation review remains independent. |

### proc_macro_frontend_contract_route_openapi

| Source | Semantic review |
| --- | --- |
| `proc_macro_frontend_contract_route_openapi/src/lib.rs` | Reviewed: compiler entrypoint attributes and token forwarding inspected; shared implementation review remains independent. |

### proc_macro_frontend_contract_route_operation

| Source | Semantic review |
| --- | --- |
| `proc_macro_frontend_contract_route_operation/src/lib.rs` | Reviewed: compiler entrypoint attributes and token forwarding inspected; shared implementation review remains independent. |

### proc_macro_frontend_contract_route_registry

| Source | Semantic review |
| --- | --- |
| `proc_macro_frontend_contract_route_registry/src/lib.rs` | Focused review: input forwarding and output parsing or registry dispatch; shared implementation remains pending. |

### proc_macro_frontend_contract_shared

| Source | Semantic review |
| --- | --- |
| `proc_macro_frontend_contract_shared/src/contract_struct_api_args.rs` | Focused review; A97 derive options. |
| `proc_macro_frontend_contract_shared/src/contract_struct_api_field_args.rs` | Focused review; A97 repeated slice type. |
| `proc_macro_frontend_contract_shared/src/contract_syn_expr.rs` | Reviewed; generated conversion and borrowed access match parser use. |
| `proc_macro_frontend_contract_shared/src/contract_syn_ident.rs` | Reviewed; generated conversion and ownership transfer match parser use. |
| `proc_macro_frontend_contract_shared/src/contract_syn_type.rs` | Reviewed; generated conversion and ownership transfer match parser use. |
| `proc_macro_frontend_contract_shared/src/endpoint_registry_args.rs` | Reviewed; required state and nonempty bindings parse through `syn`. |
| `proc_macro_frontend_contract_shared/src/endpoint_registry_binding.rs` | Reviewed; parenthesized parser rejects trailing input through `syn`. |
| `proc_macro_frontend_contract_shared/src/lib.rs` | Focused review; A97 through A102 attribute validation. |
| `proc_macro_frontend_contract_shared/src/page_catalog_args.rs` | Fixed and reviewed; A96 duplicate field rejection. |
| `proc_macro_frontend_contract_shared/src/page_catalog_page_args.rs` | Fixed and reviewed; A96 duplicate field rejection. |
| `proc_macro_frontend_contract_shared/src/route_catalog_args.rs` | Fixed and reviewed; A96 duplicate field rejection. |
| `proc_macro_frontend_contract_shared/src/route_catalog_route_args.rs` | Fixed and reviewed; A96 duplicate field and bare flag rejection. |
| `proc_macro_frontend_contract_shared/src/route_registry_args.rs` | Reviewed; required sections and parenthesized security expressions reject extra input through `syn`. |
| `proc_macro_frontend_contract_shared/src/route_registry_binding.rs` | Reviewed; parenthesized parser rejects trailing input through `syn`. |
| `proc_macro_frontend_contract_shared/src/std_bool.rs` | Reviewed; generated boolean conversion and copy access match flag use. |
| `proc_macro_frontend_contract_shared/src/syn_attributes_ref.rs` | Reviewed; test-only borrowed attribute slice wrapper. |
| `proc_macro_frontend_contract_shared/src/syn_endpoint_registry_bindings.rs` | Reviewed; wrapper forwards the parsed bindings. |
| `proc_macro_frontend_contract_shared/src/syn_endpoint_registry_contract.rs` | Reviewed; parsed expression wrapper forwards to registry generation. |
| `proc_macro_frontend_contract_shared/src/syn_endpoint_registry_endpoint.rs` | Reviewed; parsed path wrapper forwards to registry generation. |
| `proc_macro_frontend_contract_shared/src/syn_endpoint_registry_state.rs` | Reviewed; parsed state type wrapper forwards to registry generation. |
| `proc_macro_frontend_contract_shared/src/syn_route_registry_bindings.rs` | Reviewed; wrapper forwards the parsed bindings. |
| `proc_macro_frontend_contract_shared/src/syn_route_registry_endpoint.rs` | Reviewed; parsed path wrapper forwards to registry generation. |
| `proc_macro_frontend_contract_shared/src/syn_route_registry_family.rs` | Reviewed; parsed family type wrapper forwards to registry generation. |
| `proc_macro_frontend_contract_shared/src/syn_route_registry_route.rs` | Reviewed; parsed route type wrapper forwards to registry generation. |
| `proc_macro_frontend_contract_shared/src/syn_route_registry_schemas.rs` | Reviewed; parsed schema type collection forwards to registry generation. |
| `proc_macro_frontend_contract_shared/src/syn_route_registry_state.rs` | Reviewed; parsed state type wrapper forwards to registry generation. |
| `proc_macro_frontend_contract_shared/src/syn_typed_route_errors.rs` | Reviewed; policy/status variant selection is exhaustive at parser and generator use. |
| `proc_macro_frontend_contract_shared/src/test_proc_macro_frontend_contract_shared.rs` | Focused review; A96 through A102 regression cases. |
| `proc_macro_frontend_contract_shared/src/typed_route_args.rs` | Fixed and reviewed; A96 duplicate field rejection. |

### proc_macro_generate_accessor_traits_for_struct_fields_generate_accessor_trait

| Source | Semantic review |
| --- | --- |
| `proc_macro_generate_accessor_traits_for_struct_fields_generate_accessor_trait/src/lib.rs` | Reviewed: compiler entrypoint attributes and token forwarding inspected; shared implementation review remains independent. |

### proc_macro_generate_accessor_traits_for_struct_fields_generate_accessor_traits_for_struct_fields

| Source | Semantic review |
| --- | --- |
| `proc_macro_generate_accessor_traits_for_struct_fields_generate_accessor_traits_for_struct_fields/src/lib.rs` | Reviewed: compiler entrypoint attributes and token forwarding inspected; shared implementation review remains independent. |

### proc_macro_generate_accessor_traits_for_struct_fields_shared

| Source | Semantic review |
| --- | --- |
| `proc_macro_generate_accessor_traits_for_struct_fields_shared/src/lib.rs` | Reviewed: named-field and tuple-wrapper provider generation, shape diagnostics, generic declaration/forwarding and fresh parameter selection, raw identifier normalization and field-type collision avoidance; A24/A25. A23 conversion overflow now emits compiler errors in both accessor generators. |
| `proc_macro_generate_accessor_traits_for_struct_fields_shared/src/test_accessor_generics.rs` | Reviewed: generic parameters/where clauses, const-owner implementations, raw field identifiers, field-type collisions and A23 oversized-case conversion regressions. |

### proc_macro_generate_derive_token_stream_builder

| Source | Semantic review |
| --- | --- |
| `proc_macro_generate_derive_token_stream_builder/src/lib.rs` | Focused review: A23 normalized case conversion and local bounded-string validation now return compiler errors; normal macro_helpers consumer tests pass and an oversized rustc probe reports the actual length. Remaining token builder emission requires full review. |
| `proc_macro_generate_derive_token_stream_builder/src/snake_case_string.rs` | Reviewed: bounded storage for normalized snake-case names with a 1 MiB limit. |
| `proc_macro_generate_derive_token_stream_builder/src/to_snake_case_input.rs` | Reviewed: borrowed input wrapper with generated immutable access. |

### proc_macro_generate_pg_table_cm_error_variants

| Source | Semantic review |
| --- | --- |
| `proc_macro_generate_pg_table_cm_error_variants/src/lib.rs` | Reviewed: compiler entrypoint attributes and token forwarding inspected; shared implementation review remains independent. |

### proc_macro_generate_pg_table_cm_logic

| Source | Semantic review |
| --- | --- |
| `proc_macro_generate_pg_table_cm_logic/src/lib.rs` | Reviewed: compiler entrypoint attributes and token forwarding inspected; shared implementation review remains independent. |

### proc_macro_generate_pg_table_common_error_variants

| Source | Semantic review |
| --- | --- |
| `proc_macro_generate_pg_table_common_error_variants/src/lib.rs` | Reviewed: compiler entrypoint attributes and token forwarding inspected; shared implementation review remains independent. |

### proc_macro_generate_pg_table_common_logic

| Source | Semantic review |
| --- | --- |
| `proc_macro_generate_pg_table_common_logic/src/lib.rs` | Reviewed: compiler entrypoint attributes and token forwarding inspected; shared implementation review remains independent. |

### proc_macro_generate_pg_table_derive_generate_pg_table

| Source | Semantic review |
| --- | --- |
| `proc_macro_generate_pg_table_derive_generate_pg_table/src/lib.rs` | Reviewed: compiler entrypoint attributes and token forwarding inspected; shared implementation review remains independent. |

### proc_macro_generate_pg_table_dm_error_variants

| Source | Semantic review |
| --- | --- |
| `proc_macro_generate_pg_table_dm_error_variants/src/lib.rs` | Reviewed: compiler entrypoint attributes and token forwarding inspected; shared implementation review remains independent. |

### proc_macro_generate_pg_table_dm_logic

| Source | Semantic review |
| --- | --- |
| `proc_macro_generate_pg_table_dm_logic/src/lib.rs` | Reviewed: compiler entrypoint attributes and token forwarding inspected; shared implementation review remains independent. |

### proc_macro_generate_pg_table_generate_pg_table_config

| Source | Semantic review |
| --- | --- |
| `proc_macro_generate_pg_table_generate_pg_table_config/src/lib.rs` | Reviewed: compiler entrypoint attributes and token forwarding inspected; shared implementation review remains independent. |

### proc_macro_generate_pg_table_rm_error_variants

| Source | Semantic review |
| --- | --- |
| `proc_macro_generate_pg_table_rm_error_variants/src/lib.rs` | Reviewed: compiler entrypoint attributes and token forwarding inspected; shared implementation review remains independent. |

### proc_macro_generate_pg_table_rm_logic

| Source | Semantic review |
| --- | --- |
| `proc_macro_generate_pg_table_rm_logic/src/lib.rs` | Reviewed: compiler entrypoint attributes and token forwarding inspected; shared implementation review remains independent. |

### proc_macro_generate_pg_table_shared

| Source | Semantic review |
| --- | --- |
| `proc_macro_generate_pg_table_shared/src/lib.rs` | Reviewed: all attribute passthrough entries and the derive delegation inspected against their facade callers and generator tests; no defect confirmed. |

### proc_macro_generate_pg_table_um_error_variants

| Source | Semantic review |
| --- | --- |
| `proc_macro_generate_pg_table_um_error_variants/src/lib.rs` | Reviewed: compiler entrypoint attributes and token forwarding inspected; shared implementation review remains independent. |

### proc_macro_generate_pg_table_um_logic

| Source | Semantic review |
| --- | --- |
| `proc_macro_generate_pg_table_um_logic/src/lib.rs` | Reviewed: compiler entrypoint attributes and token forwarding inspected; shared implementation review remains independent. |

### proc_macro_generate_pg_types

| Source | Semantic review |
| --- | --- |
| `proc_macro_generate_pg_types/src/lib.rs` | Focused review: input forwarding and output parsing or registry dispatch; shared implementation remains pending. |

### proc_macro_generate_where_filters

| Source | Semantic review |
| --- | --- |
| `proc_macro_generate_where_filters/src/lib.rs` | Focused review: input forwarding and output parsing or registry dispatch; shared implementation remains pending. |

### proc_macro_getters

| Source | Semantic review |
| --- | --- |
| `proc_macro_getters/src/lib.rs` | Focused review: token forwarding and getter name construction; see A08 and A09. |
| `proc_macro_getters/tests/getters.rs` | Focused review: compiled raw and prefixed field fixture, existing optional and legacy getter tests; complete remainder review pending. |

### proc_macro_impl_cfg_accessor

| Source | Semantic review |
| --- | --- |
| `proc_macro_impl_cfg_accessor/src/lib.rs` | Focused review: token forwarding and getter name construction; see A08 and A09. |

### proc_macro_location_bang

| Source | Semantic review |
| --- | --- |
| `proc_macro_location_bang/src/lib.rs` | Reviewed: A62 rejects unexpected input at compile time; valid empty-input expansion retains location construction. |
| `proc_macro_location_bang/tests/ui/invalid_input.rs` | A62 compile-fail fixture: unexpected macro tokens must produce the declared diagnostic. |

### proc_macro_location_derive_location

| Source | Semantic review |
| --- | --- |
| `proc_macro_location_derive_location/src/lib.rs` | Reviewed: sole derive entrypoint forwards tokens to shared implementation. |

### proc_macro_location_errors_with_location

| Source | Semantic review |
| --- | --- |
| `proc_macro_location_errors_with_location/src/lib.rs` | Reviewed: compiler entrypoint attributes and token forwarding inspected; shared implementation review remains independent. |

### proc_macro_location_shared

| Source | Semantic review |
| --- | --- |
| `proc_macro_location_shared/src/lib.rs` | Reviewed: location field injection, named and unnamed enum generation, and error-display paths inspected. |
| `proc_macro_location_shared/src/syn_item_enum_mut_ref.rs` | Reviewed: mutable enum wrapper used by the location field injector. |

### proc_macro_naming_as_ref_str_enum_with_unit_fields_to_snake_case_str

| Source | Semantic review |
| --- | --- |
| `proc_macro_naming_as_ref_str_enum_with_unit_fields_to_snake_case_str/src/lib.rs` | Reviewed: compiler entrypoint attributes and token forwarding inspected; shared implementation review remains independent. |

### proc_macro_naming_as_ref_str_enum_with_unit_fields_to_upper_camel_case_str

| Source | Semantic review |
| --- | --- |
| `proc_macro_naming_as_ref_str_enum_with_unit_fields_to_upper_camel_case_str/src/lib.rs` | Reviewed: compiler entrypoint attributes and token forwarding inspected; shared implementation review remains independent. |

### proc_macro_naming_as_ref_str_enum_with_unit_fields_to_upper_snake_case_str

| Source | Semantic review |
| --- | --- |
| `proc_macro_naming_as_ref_str_enum_with_unit_fields_to_upper_snake_case_str/src/lib.rs` | Reviewed: compiler entrypoint attributes and token forwarding inspected; shared implementation review remains independent. |

### proc_macro_naming_common

| Source | Semantic review |
| --- | --- |
| `proc_macro_naming_common/src/lib.rs` | Reviewed: four-part validation, bounded trait names, shared closure parsing and generated trait implementations; A22 rejects unsupported parameters, and A23 adds fallible conversion plus compiler-error tokens. |

### proc_macro_naming_enum_with_unit_fields_to_snake_case_str

| Source | Semantic review |
| --- | --- |
| `proc_macro_naming_enum_with_unit_fields_to_snake_case_str/src/lib.rs` | Reviewed: compiler entrypoint attributes and token forwarding inspected; shared implementation review remains independent. |

### proc_macro_naming_enum_with_unit_fields_to_upper_camel_case_str

| Source | Semantic review |
| --- | --- |
| `proc_macro_naming_enum_with_unit_fields_to_upper_camel_case_str/src/lib.rs` | Reviewed: compiler entrypoint attributes and token forwarding inspected; shared implementation review remains independent. |

### proc_macro_naming_enum_with_unit_fields_to_upper_snake_case_str

| Source | Semantic review |
| --- | --- |
| `proc_macro_naming_enum_with_unit_fields_to_upper_snake_case_str/src/lib.rs` | Reviewed: compiler entrypoint attributes and token forwarding inspected; shared implementation review remains independent. |

### proc_macro_naming_generate_self_upper_camel_case_and_snake_case_str_and_token_stream

| Source | Semantic review |
| --- | --- |
| `proc_macro_naming_generate_self_upper_camel_case_and_snake_case_str_and_token_stream/src/lib.rs` | Reviewed: compiler entrypoint attributes and token forwarding inspected; shared implementation review remains independent. |

### proc_macro_naming_generate_upper_camel_case_and_snake_case_str_and_token_stream

| Source | Semantic review |
| --- | --- |
| `proc_macro_naming_generate_upper_camel_case_and_snake_case_str_and_token_stream/src/lib.rs` | Reviewed: compiler entrypoint attributes and token forwarding inspected; shared implementation review remains independent. |

### proc_macro_naming_shared

| Source | Semantic review |
| --- | --- |
| `proc_macro_naming_shared/src/lib.rs` | Reviewed: both JSON naming-template workflows, generated formatting/token/type-path adapters, and six enum naming generators were inspected. A28 preserves generics, A29 enforces one self placeholder, and A23 propagates overflow in generator-time case conversion; generated self-formatters remain open. Raw enum spelling retains r# in string results, recorded as a behavior requiring consumer-contract review rather than an independently confirmed defect. |
| `proc_macro_naming_shared/src/proc_macro2_generated_naming_token_stream.rs` | Reviewed: owned proc-macro2 output leaf wrapper; generated construction, transfer and token forwarding retain the complete expansion. |
| `proc_macro_naming_shared/src/proc_macro2_variant_matching_tokens_ref.rs` | Reviewed: borrowed proc-macro2 match-arm slice wrapper; generated access preserves its lifetime and performs no clone or allocation. |
| `proc_macro_naming_shared/src/syn_enum_identifier_ref.rs` | Reviewed: borrowed Syn identifier leaf wrapper with generated immutable access and conversion; preserves raw identifier tokens and lifetime. |
| `proc_macro_naming_shared/src/syn_naming_generics_ref.rs` | Reviewed: borrowed Syn generics leaf wrapper with generated conversions and immutable access; preserves source lifetime without allocation. |
| `proc_macro_naming_shared/src/test_naming_case_overflow.rs` | Reviewed: both naming-template generators emit compile_error for oversized case conversion. |
| `proc_macro_naming_shared/src/test_naming_enum_generics.rs` | Reviewed: deterministic syntax regression covers all six naming generators, requires exactly one impl, and checks const parameters, default removal, where-clause and self-type arguments; A23 tests oversized variants in all six. |
| `proc_macro_naming_shared/src/test_self_placeholder_cardinality.rs` | Reviewed: deterministic cardinality and panic-prefix regression uses only primitive unwind-safe captures; accepted templates must generate a nonempty syntactically valid expansion. |

### proc_macro_new

| Source | Semantic review |
| --- | --- |
| `proc_macro_new/src/lib.rs` | Reviewed: struct and field validation, contiguous constructor order, generics and visibility propagation, generated const construction. |
| `proc_macro_new/tests/new.rs` | Reviewed: generic, const and reordered constructor fixtures; package test passes. |

### proc_macro_newtype_accessor

| Source | Semantic review |
| --- | --- |
| `proc_macro_newtype_accessor/src/lib.rs` | Reviewed: compiler entrypoint attributes and token forwarding inspected; shared implementation review remains independent. |

### proc_macro_newtype_as_mut

| Source | Semantic review |
| --- | --- |
| `proc_macro_newtype_as_mut/src/lib.rs` | Reviewed: compiler entrypoint attributes and token forwarding inspected; shared implementation review remains independent. |

### proc_macro_newtype_as_ref

| Source | Semantic review |
| --- | --- |
| `proc_macro_newtype_as_ref/src/lib.rs` | Reviewed: compiler entrypoint attributes and token forwarding inspected; shared implementation review remains independent. |

### proc_macro_newtype_as_ref_inner

| Source | Semantic review |
| --- | --- |
| `proc_macro_newtype_as_ref_inner/src/lib.rs` | Reviewed: compiler entrypoint attributes and token forwarding inspected; shared implementation review remains independent. |

### proc_macro_newtype_as_ref_owned

| Source | Semantic review |
| --- | --- |
| `proc_macro_newtype_as_ref_owned/src/lib.rs` | Reviewed: compiler entrypoint attributes and token forwarding inspected; shared implementation review remains independent. |

### proc_macro_newtype_as_ref_str

| Source | Semantic review |
| --- | --- |
| `proc_macro_newtype_as_ref_str/src/lib.rs` | Reviewed: compiler entrypoint attributes and token forwarding inspected; shared implementation review remains independent. |

### proc_macro_newtype_as_ref_target

| Source | Semantic review |
| --- | --- |
| `proc_macro_newtype_as_ref_target/src/lib.rs` | Reviewed: compiler entrypoint attributes and token forwarding inspected; shared implementation review remains independent. |

### proc_macro_newtype_as_slice

| Source | Semantic review |
| --- | --- |
| `proc_macro_newtype_as_slice/src/lib.rs` | Reviewed: compiler entrypoint attributes and token forwarding inspected; shared implementation review remains independent. |

### proc_macro_newtype_borrow_inner

| Source | Semantic review |
| --- | --- |
| `proc_macro_newtype_borrow_inner/src/lib.rs` | Reviewed: compiler entrypoint attributes and token forwarding inspected; shared implementation review remains independent. |

### proc_macro_newtype_borrow_owned

| Source | Semantic review |
| --- | --- |
| `proc_macro_newtype_borrow_owned/src/lib.rs` | Reviewed: compiler entrypoint attributes and token forwarding inspected; shared implementation review remains independent. |

### proc_macro_newtype_borrow_path

| Source | Semantic review |
| --- | --- |
| `proc_macro_newtype_borrow_path/src/lib.rs` | Reviewed: compiler entrypoint attributes and token forwarding inspected; shared implementation review remains independent. |

### proc_macro_newtype_borrow_str

| Source | Semantic review |
| --- | --- |
| `proc_macro_newtype_borrow_str/src/lib.rs` | Reviewed: compiler entrypoint attributes and token forwarding inspected; shared implementation review remains independent. |

### proc_macro_newtype_bounded_string_wrapper

| Source | Semantic review |
| --- | --- |
| `proc_macro_newtype_bounded_string_wrapper/src/lib.rs` | Reviewed: compiler entrypoint attributes and token forwarding inspected; shared implementation review remains independent. |

### proc_macro_newtype_clone_fields

| Source | Semantic review |
| --- | --- |
| `proc_macro_newtype_clone_fields/src/lib.rs` | Reviewed: compiler entrypoint attributes and token forwarding inspected; shared implementation review remains independent. |

### proc_macro_newtype_clone_inner

| Source | Semantic review |
| --- | --- |
| `proc_macro_newtype_clone_inner/src/lib.rs` | Reviewed: compiler entrypoint attributes and token forwarding inspected; shared implementation review remains independent. |

### proc_macro_newtype_debug_display

| Source | Semantic review |
| --- | --- |
| `proc_macro_newtype_debug_display/src/lib.rs` | Reviewed: compiler entrypoint attributes and token forwarding inspected; shared implementation review remains independent. |

### proc_macro_newtype_debug_redacted

| Source | Semantic review |
| --- | --- |
| `proc_macro_newtype_debug_redacted/src/lib.rs` | Reviewed: compiler entrypoint attributes and token forwarding inspected; shared implementation review remains independent. |

### proc_macro_newtype_debug_transparent

| Source | Semantic review |
| --- | --- |
| `proc_macro_newtype_debug_transparent/src/lib.rs` | Reviewed: compiler entrypoint attributes and token forwarding inspected; shared implementation review remains independent. |

### proc_macro_newtype_default_inner

| Source | Semantic review |
| --- | --- |
| `proc_macro_newtype_default_inner/src/lib.rs` | Reviewed: compiler entrypoint attributes and token forwarding inspected; shared implementation review remains independent. |

### proc_macro_newtype_deref_inner

| Source | Semantic review |
| --- | --- |
| `proc_macro_newtype_deref_inner/src/lib.rs` | Reviewed: compiler entrypoint attributes and token forwarding inspected; shared implementation review remains independent. |

### proc_macro_newtype_deref_mut_inner

| Source | Semantic review |
| --- | --- |
| `proc_macro_newtype_deref_mut_inner/src/lib.rs` | Reviewed: compiler entrypoint attributes and token forwarding inspected; shared implementation review remains independent. |

### proc_macro_newtype_deref_mut_target

| Source | Semantic review |
| --- | --- |
| `proc_macro_newtype_deref_mut_target/src/lib.rs` | Reviewed: compiler entrypoint attributes and token forwarding inspected; shared implementation review remains independent. |

### proc_macro_newtype_deref_target

| Source | Semantic review |
| --- | --- |
| `proc_macro_newtype_deref_target/src/lib.rs` | Reviewed: compiler entrypoint attributes and token forwarding inspected; shared implementation review remains independent. |

### proc_macro_newtype_display

| Source | Semantic review |
| --- | --- |
| `proc_macro_newtype_display/src/lib.rs` | Reviewed: compiler entrypoint attributes and token forwarding inspected; shared implementation review remains independent. |

### proc_macro_newtype_display_const

| Source | Semantic review |
| --- | --- |
| `proc_macro_newtype_display_const/src/lib.rs` | Reviewed: compiler entrypoint attributes and token forwarding inspected; shared implementation review remains independent. |

### proc_macro_newtype_enum_from_str

| Source | Semantic review |
| --- | --- |
| `proc_macro_newtype_enum_from_str/src/lib.rs` | Reviewed: compiler entrypoint attributes and token forwarding inspected; shared implementation review remains independent. |

### proc_macro_newtype_foundation_foundation_as_ref_inner

| Source | Semantic review |
| --- | --- |
| `proc_macro_newtype_foundation_foundation_as_ref_inner/src/lib.rs` | Reviewed: compiler entrypoint attributes and token forwarding inspected; shared implementation review remains independent. |

### proc_macro_newtype_foundation_foundation_from_inner

| Source | Semantic review |
| --- | --- |
| `proc_macro_newtype_foundation_foundation_from_inner/src/lib.rs` | Reviewed: compiler entrypoint attributes and token forwarding inspected; shared implementation review remains independent. |

### proc_macro_newtype_foundation_foundation_get_inner

| Source | Semantic review |
| --- | --- |
| `proc_macro_newtype_foundation_foundation_get_inner/src/lib.rs` | Reviewed: compiler entrypoint attributes and token forwarding inspected; shared implementation review remains independent. |

### proc_macro_newtype_foundation_foundation_to_tokens

| Source | Semantic review |
| --- | --- |
| `proc_macro_newtype_foundation_foundation_to_tokens/src/lib.rs` | Reviewed: compiler entrypoint attributes and token forwarding inspected; shared implementation review remains independent. |

### proc_macro_newtype_foundation_shared

| Source | Semantic review |
| --- | --- |
| `proc_macro_newtype_foundation_shared/src/lib.rs` | Reviewed: single-field tuple validation, generic impl forwarding, reference adaptation, getter receiver and ToTokens delegation. Generated borrowed value getters require Copy inner types at compilation. |

### proc_macro_newtype_from_getter

| Source | Semantic review |
| --- | --- |
| `proc_macro_newtype_from_getter/src/lib.rs` | Reviewed: compiler entrypoint attributes and token forwarding inspected; shared implementation review remains independent. |

### proc_macro_newtype_from_inner

| Source | Semantic review |
| --- | --- |
| `proc_macro_newtype_from_inner/src/lib.rs` | Reviewed: compiler entrypoint attributes and token forwarding inspected; shared implementation review remains independent. |

### proc_macro_newtype_get_inner

| Source | Semantic review |
| --- | --- |
| `proc_macro_newtype_get_inner/src/lib.rs` | Reviewed: compiler entrypoint attributes and token forwarding inspected; shared implementation review remains independent. |

### proc_macro_newtype_into_inner

| Source | Semantic review |
| --- | --- |
| `proc_macro_newtype_into_inner/src/lib.rs` | Reviewed: compiler entrypoint attributes and token forwarding inspected; shared implementation review remains independent. |

### proc_macro_newtype_into_inner_from

| Source | Semantic review |
| --- | --- |
| `proc_macro_newtype_into_inner_from/src/lib.rs` | Reviewed: compiler entrypoint attributes and token forwarding inspected; shared implementation review remains independent. |

### proc_macro_newtype_into_iterator

| Source | Semantic review |
| --- | --- |
| `proc_macro_newtype_into_iterator/src/lib.rs` | Reviewed: compiler entrypoint attributes and token forwarding inspected; shared implementation review remains independent. |

### proc_macro_newtype_into_vec

| Source | Semantic review |
| --- | --- |
| `proc_macro_newtype_into_vec/src/lib.rs` | Reviewed: compiler entrypoint attributes and token forwarding inspected; shared implementation review remains independent. |

### proc_macro_newtype_not_inner

| Source | Semantic review |
| --- | --- |
| `proc_macro_newtype_not_inner/src/lib.rs` | Reviewed: compiler entrypoint attributes and token forwarding inspected; shared implementation review remains independent. |

### proc_macro_newtype_partial_eq_inner

| Source | Semantic review |
| --- | --- |
| `proc_macro_newtype_partial_eq_inner/src/lib.rs` | Reviewed: compiler entrypoint attributes and token forwarding inspected; shared implementation review remains independent. |

### proc_macro_newtype_shared

| Source | Semantic review |
| --- | --- |
| `proc_macro_newtype_shared/src/bounded_string_attrs.rs` | Reviewed: default optional metadata and unique option storage; behavior is implemented in the separately tracked generator. |
| `proc_macro_newtype_shared/src/bounded_string_option.rs` | Reviewed: unique ordered enum discriminants for bounded-string generator switches; A89 opt-in error mode. |
| `proc_macro_newtype_shared/src/lib.rs` | Reviewed: derive parsing, bounded-string validation and emitted forwarding/enum implementations inspected; fixes A15 and A16 verified. Confirmed generic adapter failures A17 and A18 are fixed and verified; A89 opt-in error generation verified. |
| `proc_macro_newtype_shared/src/newtype_attrs.rs` | Reviewed: default unique option storage and forwarding membership; duplicate rejection belongs to the shared option collection. |
| `proc_macro_newtype_shared/src/newtype_bool.rs` | Reviewed: typed boolean with derived conversion and borrowed copy accessor. |
| `proc_macro_newtype_shared/src/newtype_option.rs` | Reviewed: unique ordered enum discriminants for newtype generator switches. |
| `proc_macro_newtype_shared/src/newtype_syn_derive_input_ref.rs` | Reviewed: borrowed Syn input forwarding preserves the source lifetime; ownership-free conversions. |
| `proc_macro_newtype_shared/src/proc_macro2_generated_token_stream.rs` | Reviewed: owned token stream forwarding between compiler and proc-macro2 streams without text reparsing. |
| `proc_macro_newtype_shared/src/proc_macro_input_token_stream.rs` | Reviewed: owned compiler and proc-macro2 token stream conversion; input is consumed exactly once. |
| `proc_macro_newtype_shared/src/snake_ident_max_len.rs` | Reviewed: fixed 1 MiB limit used consistently by the SnakeIdentifier validator and diagnostic. |
| `proc_macro_newtype_shared/src/snake_identifier.rs` | Reviewed: validates the byte limit before bounded storage conversion and forwards token formatting; conversion failures retain their observed length. |
| `proc_macro_newtype_shared/src/snake_identifierifier_len.rs` | Reviewed: typed usize with derived conversion and borrowed getter. |
| `proc_macro_newtype_shared/src/snake_identifierifier_try_from_string_error.rs` | Reviewed: identifier length diagnostic contains lengths and limits, without input text. |
| `proc_macro_newtype_shared/src/syn_expr.rs` | Reviewed: owned Syn expression forwarding through derived references and token emission. |
| `proc_macro_newtype_shared/src/syn_identifier_ref.rs` | Reviewed: borrowed Syn identifier forwarding preserves its lifetime. |
| `proc_macro_newtype_shared/src/syn_type.rs` | Reviewed: owned Syn type forwarding through derived references. |
| `proc_macro_newtype_shared/src/syn_type_ref.rs` | Reviewed: borrowed Syn type forwarding preserves its lifetime. |
| `proc_macro_newtype_shared/src/test_proc_macro_newtype_shared.rs` | Reviewed: bounded-string derive diagnostics cover missing maximum, byte schema mismatch, and duplicate options. |
| `proc_macro_newtype_shared/src/to_err_string_mode.rs` | Reviewed: exhaustive ordered formatting modes; generic expansion behavior is tracked in A18. |
| `proc_macro_newtype_shared/src/wire_enum_attrs.rs` | Focused review: parses required error and reference types; duplicate scalar option behavior remains to be checked. |

### proc_macro_newtype_tests

| Source | Semantic review |
| --- | --- |
| `proc_macro_newtype_tests/tests/test_foundation_derives.rs` | Reviewed: foundation derive fixtures cover borrowed and owned getters, references, and token forwarding. |
| `proc_macro_newtype_tests/tests/test_newtype.rs` | Reviewed: newtype derive integration fixtures cover conversions, bounds, schemas, generics, and secret redaction. |

### proc_macro_newtype_to_err_string

| Source | Semantic review |
| --- | --- |
| `proc_macro_newtype_to_err_string/src/lib.rs` | Reviewed: compiler entrypoint attributes and token forwarding inspected; shared implementation review remains independent. |

### proc_macro_newtype_to_err_string_as_ref_str

| Source | Semantic review |
| --- | --- |
| `proc_macro_newtype_to_err_string_as_ref_str/src/lib.rs` | Reviewed: compiler entrypoint attributes and token forwarding inspected; shared implementation review remains independent. |

### proc_macro_newtype_to_err_string_debug

| Source | Semantic review |
| --- | --- |
| `proc_macro_newtype_to_err_string_debug/src/lib.rs` | Reviewed: compiler entrypoint attributes and token forwarding inspected; shared implementation review remains independent. |

### proc_macro_newtype_to_tokens

| Source | Semantic review |
| --- | --- |
| `proc_macro_newtype_to_tokens/src/lib.rs` | Reviewed: compiler entrypoint attributes and token forwarding inspected; shared implementation review remains independent. |

### proc_macro_newtype_utoipa_schema

| Source | Semantic review |
| --- | --- |
| `proc_macro_newtype_utoipa_schema/src/lib.rs` | Reviewed: compiler entrypoint attributes and token forwarding inspected; shared implementation review remains independent. |

### proc_macro_newtype_wire_enum

| Source | Semantic review |
| --- | --- |
| `proc_macro_newtype_wire_enum/src/lib.rs` | Reviewed: compiler entrypoint attributes and token forwarding inspected; shared implementation review remains independent. |

### proc_macro_optimal_memory_layout

| Source | Semantic review |
| --- | --- |
| `proc_macro_optimal_memory_layout/src/lib.rs` | Reviewed: attribute parsing and alignment assertions; generic types, tuple fields and wasm are deliberately skipped by existing behavior. |

### proc_macro_to_err_string_impl_to_err_string_as_ref_str

| Source | Semantic review |
| --- | --- |
| `proc_macro_to_err_string_impl_to_err_string_as_ref_str/src/lib.rs` | Reviewed: compiler entrypoint attributes and token forwarding inspected; shared implementation review remains independent. |

### proc_macro_to_err_string_impl_to_err_string_const

| Source | Semantic review |
| --- | --- |
| `proc_macro_to_err_string_impl_to_err_string_const/src/lib.rs` | Reviewed: compiler entrypoint attributes and token forwarding inspected; shared implementation review remains independent. |

### proc_macro_to_err_string_impl_to_err_string_with

| Source | Semantic review |
| --- | --- |
| `proc_macro_to_err_string_impl_to_err_string_with/src/lib.rs` | Reviewed: compiler entrypoint attributes and token forwarding inspected; shared implementation review remains independent. |

### proc_macro_to_err_string_shared

| Source | Semantic review |
| --- | --- |
| `proc_macro_to_err_string_shared/src/lib.rs` | Reviewed: closure, constant and AsRef string generators route through bounded diagnostic storage; malformed closure and pair syntax emit compile errors, and downstream to_err_string tests exercise generated implementations. |

### proc_macro_token_patterns_shared

| Source | Semantic review |
| --- | --- |
| `proc_macro_token_patterns_shared/src/lib.rs` | Reviewed: identifier/comma guards, ordered owned part emission, function emission and parenthesized batch filtering; A10 preserves part order without mutable wrapper access. |
| `proc_macro_token_patterns_shared/src/proc_macro2_generate_tp_input.rs` | Reviewed: private compiler token input wrapper with generated ownership conversion. |
| `proc_macro_token_patterns_shared/src/proc_macro2_generate_tp_output.rs` | Reviewed: private compiler token output wrapper with generated ownership conversion. |

### proc_macro_token_patterns_tp

| Source | Semantic review |
| --- | --- |
| `proc_macro_token_patterns_tp/src/lib.rs` | Reviewed: single compiler adapter delegates complete token generation to the shared owner. |

### proc_macro_token_patterns_tp_batch

| Source | Semantic review |
| --- | --- |
| `proc_macro_token_patterns_tp_batch/src/lib.rs` | Reviewed: single compiler adapter delegates parenthesized batch generation to the shared owner. |

### proc_macro_token_patterns_tp_parts

| Source | Semantic review |
| --- | --- |
| `proc_macro_token_patterns_tp_parts/src/lib.rs` | Reviewed: single compiler adapter delegates ordered part generation to the shared owner. |

### proc_macro_token_patterns_ts_path_fn

| Source | Semantic review |
| --- | --- |
| `proc_macro_token_patterns_ts_path_fn/src/lib.rs` | Reviewed: single compiler adapter delegates function generation to the shared owner. |

### proc_macro_trait_alias

| Source | Semantic review |
| --- | --- |
| `proc_macro_trait_alias/src/lib.rs` | Reviewed: compiler token adapter delegates alias generation and diagnostics to the shared owner; A12. |

### proc_macro_try_from_env

| Source | Semantic review |
| --- | --- |
| `proc_macro_try_from_env/src/lib.rs` | Focused review: A23 environment-name conversion now uses one fallible result across descriptors, examples and reads; oversized-field rustc probe emits a spanned length error. The remaining derive branches require full semantic review. |

### route_validators

| Source | Semantic review |
| --- | --- |
| `route_validators/src/assert_err_status_code.rs` | Reviewed: typed wrapper or test utility preserves its stated conversion, assertion, or panic behavior. |
| `route_validators/src/assert_err_status_code_only.rs` | Reviewed: typed wrapper or test utility preserves its stated conversion, assertion, or panic behavior. |
| `route_validators/src/assert_err_status_code_variant_ref.rs` | Reviewed: typed wrapper or test utility preserves its stated conversion, assertion, or panic behavior. |
| `route_validators/src/assert_ok_eq.rs` | Reviewed: typed wrapper or test utility preserves its stated conversion, assertion, or panic behavior. |
| `route_validators/src/assert_panics.rs` | Reviewed: typed wrapper or test utility preserves its stated conversion, assertion, or panic behavior. |
| `route_validators/src/axum_body.rs` | Reviewed: body wrapper forwards ownership and size hint. |
| `route_validators/src/axum_body_size_error.rs` | Reviewed: typed wrapper or test utility preserves its stated conversion, assertion, or panic behavior. |
| `route_validators/src/axum_commit_to_str_conversion_error.rs` | Reviewed: typed wrapper or test utility preserves its stated conversion, assertion, or panic behavior. |
| `route_validators/src/axum_header_value_ref.rs` | Reviewed: borrowed header value preserves lifetime. |
| `route_validators/src/axum_headers_ref.rs` | Reviewed: borrowed header map returns matching values. |
| `route_validators/src/axum_http_status_code.rs` | Reviewed: fixed status constructors match their HTTP codes. |
| `route_validators/src/axum_http_status_code_provider.rs` | Reviewed: trait and test preserve error status dispatch. |
| `route_validators/src/axum_test_header_value.rs` | Reviewed: typed wrapper or test utility preserves its stated conversion, assertion, or panic behavior. |
| `route_validators/src/axum_test_headers.rs` | Reviewed: typed wrapper or test utility preserves its stated conversion, assertion, or panic behavior. |
| `route_validators/src/axum_test_headers_mut_ref.rs` | Reviewed: typed wrapper or test utility preserves its stated conversion, assertion, or panic behavior. |
| `route_validators/src/body_size_error.rs` | Reviewed: exceeded body limit maps to HTTP 413 with size context. |
| `route_validators/src/body_size_limit_bytes.rs` | Reviewed: typed body-size limit preserves byte count. |
| `route_validators/src/bytes_body_bytes.rs` | Reviewed: typed wrapper or test utility preserves its stated conversion, assertion, or panic behavior. |
| `route_validators/src/check_body_size.rs` | Reviewed: bounded read accepts the exact limit and rejects excess bytes. |
| `route_validators/src/check_commit.rs` | Reviewed: enabled check validates the commit header and disabled check bypasses it. |
| `route_validators/src/commit_error.rs` | Reviewed: missing, non-UTF8, and mismatched commits map to HTTP 400. |
| `route_validators/src/commit_header_name.rs` | Reviewed: typed wrapper or test utility preserves its stated conversion, assertion, or panic behavior. |
| `route_validators/src/commit_not_eq_message.rs` | Reviewed: typed wrapper or test utility preserves its stated conversion, assertion, or panic behavior. |
| `route_validators/src/commit_to_use.rs` | Reviewed: mismatch response retains the static project link. |
| `route_validators/src/enable_api_git_commit_check.rs` | Reviewed: boolean wrapper controls commit validation. |
| `route_validators/src/expect_err_variant_ref_with_status.rs` | Reviewed: typed wrapper or test utility preserves its stated conversion, assertion, or panic behavior. |
| `route_validators/src/expect_error.rs` | Reviewed: typed wrapper or test utility preserves its stated conversion, assertion, or panic behavior. |
| `route_validators/src/expect_error_mapped.rs` | Reviewed: typed wrapper or test utility preserves its stated conversion, assertion, or panic behavior. |
| `route_validators/src/expect_error_variant_ref.rs` | Reviewed: typed wrapper or test utility preserves its stated conversion, assertion, or panic behavior. |
| `route_validators/src/expect_ok.rs` | Reviewed: typed wrapper or test utility preserves its stated conversion, assertion, or panic behavior. |
| `route_validators/src/expect_variant.rs` | Reviewed: typed wrapper or test utility preserves its stated conversion, assertion, or panic behavior. |
| `route_validators/src/expect_variant_ref.rs` | Reviewed: typed wrapper or test utility preserves its stated conversion, assertion, or panic behavior. |
| `route_validators/src/header_str_ref.rs` | Reviewed: borrowed header text wrapper preserves lifetime. |
| `route_validators/src/header_value_tests.rs` | Reviewed: deterministic header and test-helper cases cover success and error paths. |
| `route_validators/src/http_body_size_hint.rs` | Reviewed: size hint wrapper renders bounded diagnostic text. |
| `route_validators/src/increment_block_on_poll_count.rs` | Reviewed: typed wrapper or test utility preserves its stated conversion, assertion, or panic behavior. |
| `route_validators/src/insert_header_no_prev.rs` | Reviewed: typed wrapper or test utility preserves its stated conversion, assertion, or panic behavior. |
| `route_validators/src/is_block_on_poll_limit_reached.rs` | Reviewed: typed wrapper or test utility preserves its stated conversion, assertion, or panic behavior. |
| `route_validators/src/lib.rs` | Reviewed: module declarations and test-only ownership match the implementation. |
| `route_validators/src/make_headers_with_entry.rs` | Reviewed: typed wrapper or test utility preserves its stated conversion, assertion, or panic behavior. |
| `route_validators/src/map_err.rs` | Reviewed: test helper inspects errors before mapping. |
| `route_validators/src/map_err_after_status_check.rs` | Reviewed: test helper verifies status before mapping. |
| `route_validators/src/map_or_panic_unexpected_variant.rs` | Reviewed: typed wrapper or test utility preserves its stated conversion, assertion, or panic behavior. |
| `route_validators/src/max_block_on_polls.rs` | Reviewed: typed wrapper or test utility preserves its stated conversion, assertion, or panic behavior. |
| `route_validators/src/no_commit_header_message.rs` | Reviewed: typed wrapper or test utility preserves its stated conversion, assertion, or panic behavior. |
| `route_validators/src/non_utf8_header_value.rs` | Reviewed: typed wrapper or test utility preserves its stated conversion, assertion, or panic behavior. |
| `route_validators/src/panic_unexpected_result.rs` | Reviewed: typed wrapper or test utility preserves its stated conversion, assertion, or panic behavior. |
| `route_validators/src/panic_unexpected_variant.rs` | Reviewed: typed wrapper or test utility preserves its stated conversion, assertion, or panic behavior. |
| `route_validators/src/poll_test_future.rs` | Reviewed: test helper bounds manual future polls. |
| `route_validators/src/read_commit_header_str.rs` | Reviewed: typed header reader distinguishes absent and non-UTF8 values. |
| `route_validators/src/replace_header_name.rs` | Reviewed: typed wrapper or test utility preserves its stated conversion, assertion, or panic behavior. |
| `route_validators/src/required_header_str.rs` | Reviewed: reader delegates missing and decoding errors through callbacks. |
| `route_validators/src/required_header_str_parsed.rs` | Reviewed: parsed reader preserves borrowed header lifetime and parse failures. |
| `route_validators/src/required_header_value.rs` | Reviewed: header lookup maps absence to caller-owned error. |
| `route_validators/src/test_exp_id.rs` | Reviewed: typed wrapper or test utility preserves its stated conversion, assertion, or panic behavior. |
| `route_validators/src/test_helper.rs` | Reviewed: deterministic header and test-helper cases cover success and error paths. |
| `route_validators/src/test_panic_text.rs` | Reviewed: typed wrapper or test utility preserves its stated conversion, assertion, or panic behavior. |
| `route_validators/src/test_poll_count.rs` | Reviewed: typed wrapper or test utility preserves its stated conversion, assertion, or panic behavior. |
| `route_validators/src/test_poll_limit_reached.rs` | Reviewed: typed wrapper or test utility preserves its stated conversion, assertion, or panic behavior. |
| `route_validators/src/validate_commit_header.rs` | Reviewed: header reader passes typed text to commit validator. |
| `route_validators/src/validate_commit_header_value.rs` | Reviewed: mismatch retains the project commit link. |

### runtime_tests

| Source | Semantic review |
| --- | --- |
| `runtime_tests/src/http_runtime_test_status.rs` | Reviewed: typed status wrapper supports exact comparisons. |
| `runtime_tests/src/lib.rs` | Reviewed: probe runner checks liveness, readiness, creation status and decoded responses in order. |
| `runtime_tests/src/main.rs` | Reviewed: command invokes local configuration and returns probe failures. |
| `runtime_tests/src/reqwest_runtime_test_client.rs` | Reviewed: blocking client sends typed GET and JSON POST requests. |
| `runtime_tests/src/reqwest_runtime_test_response.rs` | Reviewed: response wrapper decodes the matching typed report or creation response. |
| `runtime_tests/src/runtime_test_config.rs` | Reviewed: config stores distinct application and notification service base URLs. |
| `runtime_tests/src/runtime_test_error.rs` | Reviewed: errors retain the failed test and request, response, status, or validation source. |
| `runtime_tests/src/runtime_test_kind.rs` | Reviewed: fixed test kinds have stable display text. |
| `runtime_tests/src/runtime_test_report.rs` | Reviewed: report bounds the five expected passed test kinds. |
| `runtime_tests/src/runtime_test_url.rs` | Reviewed: assembled route URL enforces the configured byte limit. |
| `runtime_tests/src/service_base_url.rs` | Reviewed: base URL normalization accepts HTTP(S) hosts and rejects query or fragment suffixes. |
| `runtime_tests/src/service_base_url_error.rs` | Reviewed: typed errors distinguish host, length, scheme, and suffix failures. |
| `runtime_tests/src/tests_domain_types.rs` | Reviewed: tests cover trailing slash normalization and invalid scheme or suffix. |

### server

| Source | Semantic review |
| --- | --- |
| `server/src/admin_metrics.rs` | Reviewed: bounded metrics rendering returns an OK response or typed render error. |
| `server/src/admin_metrics_error.rs` | Reviewed: render failure maps to HTTP 500. |
| `server/src/admin_metrics_page.rs` | Reviewed: metrics page validates rendered text before HTML response. |
| `server/src/admin_metrics_page_route_registry.rs` | Reviewed: page registry maps the metrics path to its handler. |
| `server/src/admin_metrics_route_registry.rs` | Reviewed: API registry maps the metrics route to its handler. |
| `server/src/admin_open_api.rs` | Reviewed: handler serializes the generated administrator OpenAPI document. |
| `server/src/admin_open_api_route_registry.rs` | Reviewed: OpenAPI registry maps the typed administrator route. |
| `server/src/axum_api_routes.rs` | Reviewed: typed API router ownership wrapper. |
| `server/src/axum_metrics_exporter_prometheus_renderer.rs` | Reviewed: extractor clones the shared metrics handle. |
| `server/src/frontend_fallback_routes.rs` | Reviewed: unknown frontend paths redirect to the typed sign-in path. |
| `server/src/http_body_maximum_bytes.rs` | Reviewed: typed body-limit wrapper. |
| `server/src/main.rs` | Reviewed: startup validates config, builds the pool and routes, supervises cleanup, and shuts down observability. |
| `server/src/make_postgresql_pool.rs` | Reviewed: pool options reject inverted connection bounds and preserve connection errors. |
| `server/src/metrics_exporter_prometheus_build_error.rs` | Reviewed: metrics recorder build error retains its source. |
| `server/src/metrics_exporter_prometheus_renderer.rs` | Reviewed: typed Prometheus handle wrapper. |
| `server/src/mount_service_routes.rs` | Reviewed: operational and API routes are root mounted with API body limits. |
| `server/src/run_server_error.rs` | Reviewed: startup and serving failure variants retain their operation context. |
| `server/src/server_exit_code.rs` | Reviewed: termination forwards the selected exit code. |
| `server/src/server_io_error.rs` | Reviewed: I/O error wrapper retains its source. |
| `server/src/shared_server_app_state_arc.rs` | Reviewed: one Arc allocation shares immutable application state across handlers. |
| `server/src/sqlx_server_pg_connect_error.rs` | Reviewed: connection error wrapper retains its source. |
| `server/src/tests_domain_types.rs` | Reviewed: tests cover router mounting, fallback redirect, asset serving, and bind errors. |
| `server/src/tokio_server_runtime.rs` | Reviewed: typed Tokio runtime ownership wrapper. |

### server_admin

| Source | Semantic review |
| --- | --- |
| `server_admin/src/access_session_read_page.rs` | Pending |
| `server_admin/src/account_change_own_password.rs` | Pending |
| `server_admin/src/account_me.rs` | Pending |
| `server_admin/src/account_me_context_view_ref.rs` | Pending |
| `server_admin/src/action_result_impl.rs` | Pending |
| `server_admin/src/admin_access_claims.rs` | Pending |
| `server_admin/src/admin_access_sessions.rs` | Pending |
| `server_admin/src/admin_access_sessions_read_page_error.rs` | Pending |
| `server_admin/src/admin_access_token_error.rs` | Pending |
| `server_admin/src/admin_access_token_validation.rs` | Pending |
| `server_admin/src/admin_active_administrator_count.rs` | Pending |
| `server_admin/src/admin_api_open_api.rs` | Pending |
| `server_admin/src/admin_audit_action.rs` | Pending |
| `server_admin/src/admin_audit_log.rs` | Pending |
| `server_admin/src/admin_audit_log_read_page_error.rs` | Pending |
| `server_admin/src/admin_audit_resource.rs` | Pending |
| `server_admin/src/admin_audit_resource_id.rs` | Pending |
| `server_admin/src/admin_audit_success_ref.rs` | Pending |
| `server_admin/src/admin_auth_collection_error.rs` | Pending |
| `server_admin/src/admin_auth_collection_max_len.rs` | Pending |
| `server_admin/src/admin_auth_html_routes.rs` | Pending |
| `server_admin/src/admin_auth_policy.rs` | Pending |
| `server_admin/src/admin_auth_positive_value_error.rs` | Pending |
| `server_admin/src/admin_auth_request.rs` | Pending |
| `server_admin/src/admin_auth_route_registry.rs` | Pending |
| `server_admin/src/admin_auth_routes.rs` | Pending |
| `server_admin/src/admin_auth_rules.rs` | Pending |
| `server_admin/src/admin_auth_svc_state.rs` | Pending |
| `server_admin/src/admin_auth_svc_state_build_error.rs` | Pending |
| `server_admin/src/admin_branding_page.rs` | Pending |
| `server_admin/src/admin_cleanup_batch_size.rs` | Pending |
| `server_admin/src/admin_cleanup_configuration.rs` | Pending |
| `server_admin/src/admin_cleanup_configuration_error.rs` | Pending |
| `server_admin/src/admin_cleanup_error.rs` | Pending |
| `server_admin/src/admin_cleanup_report.rs` | Pending |
| `server_admin/src/admin_cleanup_retention_seconds.rs` | Pending |
| `server_admin/src/admin_cleanup_rows.rs` | Pending |
| `server_admin/src/admin_cleanup_status.rs` | Pending |
| `server_admin/src/admin_cookie_kind.rs` | Pending |
| `server_admin/src/admin_cookie_max_age_seconds.rs` | Pending |
| `server_admin/src/admin_create_roles_payload_example_error.rs` | Pending |
| `server_admin/src/admin_create_user_payload_example_error.rs` | Pending |
| `server_admin/src/admin_crud_page.rs` | Pending |
| `server_admin/src/admin_data_columns.rs` | Pending |
| `server_admin/src/admin_db_ref.rs` | Pending |
| `server_admin/src/admin_delete_roles_payload_example_error.rs` | Pending |
| `server_admin/src/admin_delete_users_payload_example_error.rs` | Pending |
| `server_admin/src/admin_error.rs` | Pending |
| `server_admin/src/admin_error_response_parts.rs` | Pending |
| `server_admin/src/admin_generated_auth_layer.rs` | Pending |
| `server_admin/src/admin_generated_auth_service.rs` | Pending |
| `server_admin/src/admin_generated_route_contract.rs` | Pending |
| `server_admin/src/admin_generated_table.rs` | Pending |
| `server_admin/src/admin_generated_token.rs` | Pending |
| `server_admin/src/admin_health_page.rs` | Pending |
| `server_admin/src/admin_html_action_route_registry.rs` | Pending |
| `server_admin/src/admin_html_auth_action_route_registry.rs` | Pending |
| `server_admin/src/admin_html_form_key.rs` | Pending |
| `server_admin/src/admin_html_form_key_error.rs` | Pending |
| `server_admin/src/admin_html_form_selected_max_items.rs` | Pending |
| `server_admin/src/admin_html_form_text.rs` | Pending |
| `server_admin/src/admin_html_form_text_error.rs` | Pending |
| `server_admin/src/admin_html_open_api.rs` | Pending |
| `server_admin/src/admin_html_page_route_registry.rs` | Pending |
| `server_admin/src/admin_html_role_action_route_registry.rs` | Pending |
| `server_admin/src/admin_html_session_action_route_registry.rs` | Pending |
| `server_admin/src/admin_html_sessions_page.rs` | Pending |
| `server_admin/src/admin_html_settings_action_route_registry.rs` | Pending |
| `server_admin/src/admin_html_swagger_enabled.rs` | Pending |
| `server_admin/src/admin_html_swagger_route_registry.rs` | Pending |
| `server_admin/src/admin_html_user_action_route_registry.rs` | Pending |
| `server_admin/src/admin_login_attempts.rs` | Pending |
| `server_admin/src/admin_migrate_error.rs` | Pending |
| `server_admin/src/admin_migrator.rs` | Pending |
| `server_admin/src/admin_new_password_from_contract.rs` | Pending |
| `server_admin/src/admin_observed_error_code.rs` | Pending |
| `server_admin/src/admin_observed_internal_error_response.rs` | Pending |
| `server_admin/src/admin_opaque_token.rs` | Pending |
| `server_admin/src/admin_page_total_count.rs` | Pending |
| `server_admin/src/admin_password_change_required.rs` | Pending |
| `server_admin/src/admin_password_from_contract.rs` | Pending |
| `server_admin/src/admin_password_hash.rs` | Pending |
| `server_admin/src/admin_password_hash_error.rs` | Pending |
| `server_admin/src/admin_password_hasher.rs` | Pending |
| `server_admin/src/admin_password_reset_error.rs` | Pending |
| `server_admin/src/admin_password_try_from_string_error.rs` | Pending |
| `server_admin/src/admin_peer_addr.rs` | Pending |
| `server_admin/src/admin_permission_actions.rs` | Pending |
| `server_admin/src/admin_permission_actions_read_page_error.rs` | Pending |
| `server_admin/src/admin_permission_resource_actions.rs` | Pending |
| `server_admin/src/admin_permission_resource_actions_read_page_error.rs` | Pending |
| `server_admin/src/admin_permission_resources.rs` | Pending |
| `server_admin/src/admin_permission_resources_read_page_error.rs` | Pending |
| `server_admin/src/admin_rate_limit_scope.rs` | Pending |
| `server_admin/src/admin_rate_limits.rs` | Pending |
| `server_admin/src/admin_recent_login_failure_count.rs` | Pending |
| `server_admin/src/admin_refresh_token.rs` | Pending |
| `server_admin/src/admin_refresh_tokens.rs` | Pending |
| `server_admin/src/admin_repository_error.rs` | Pending |
| `server_admin/src/admin_role_rules.rs` | Pending |
| `server_admin/src/admin_role_rules_read_page_error.rs` | Pending |
| `server_admin/src/admin_role_update_slice.rs` | Pending |
| `server_admin/src/admin_roles.rs` | Pending |
| `server_admin/src/admin_roles_read_page.rs` | Pending |
| `server_admin/src/admin_roles_read_page_error.rs` | Pending |
| `server_admin/src/admin_rules.rs` | Pending |
| `server_admin/src/admin_rules_read_page_error.rs` | Pending |
| `server_admin/src/admin_secret_text_error.rs` | Pending |
| `server_admin/src/admin_session_bundle.rs` | Pending |
| `server_admin/src/admin_session_error.rs` | Pending |
| `server_admin/src/admin_session_id.rs` | Pending |
| `server_admin/src/admin_session_path.rs` | Pending |
| `server_admin/src/admin_shared_semaphore_arc.rs` | Pending |
| `server_admin/src/admin_sign_in_json.rs` | Pending |
| `server_admin/src/admin_sign_in_user.rs` | Pending |
| `server_admin/src/admin_system_settings.rs` | Pending |
| `server_admin/src/admin_system_settings_read_page_error.rs` | Pending |
| `server_admin/src/admin_token_hash.rs` | Pending |
| `server_admin/src/admin_unix_token_stream.rs` | Pending |
| `server_admin/src/admin_update_roles_payload_example_error.rs` | Pending |
| `server_admin/src/admin_update_users_payload_example_error.rs` | Pending |
| `server_admin/src/admin_user_roles.rs` | Pending |
| `server_admin/src/admin_user_update_slice.rs` | Pending |
| `server_admin/src/admin_users_database_read.rs` | Pending |
| `server_admin/src/admin_users_database_read_page_error.rs` | Pending |
| `server_admin/src/api_branding.rs` | Pending |
| `server_admin/src/api_change_own_password.rs` | Pending |
| `server_admin/src/api_cleanup_status_table.rs` | Reviewed: typed cleanup-status read delegates to the shared generic table handler. |
| `server_admin/src/api_create_roles.rs` | Pending |
| `server_admin/src/api_create_roles_payload_example.rs` | Pending |
| `server_admin/src/api_create_user.rs` | Pending |
| `server_admin/src/api_create_user_payload_example.rs` | Pending |
| `server_admin/src/api_data_tables.rs` | Pending |
| `server_admin/src/api_delete_access_sessions.rs` | Pending |
| `server_admin/src/api_delete_roles.rs` | Pending |
| `server_admin/src/api_delete_roles_payload_example.rs` | Pending |
| `server_admin/src/api_delete_users.rs` | Pending |
| `server_admin/src/api_delete_users_payload_example.rs` | Pending |
| `server_admin/src/api_login_attempts_table.rs` | Reviewed: typed login-attempt read delegates to the shared generic table handler. |
| `server_admin/src/api_me.rs` | Pending |
| `server_admin/src/api_rate_limits_table.rs` | Reviewed: typed rate-limit read delegates to the shared generic table handler. |
| `server_admin/src/api_refresh.rs` | Pending |
| `server_admin/src/api_refresh_tokens_table.rs` | Reviewed: typed refresh-token read delegates to the shared generic table handler. |
| `server_admin/src/api_revoke_all_sessions.rs` | Pending |
| `server_admin/src/api_revoke_session.rs` | Pending |
| `server_admin/src/api_sessions.rs` | Pending |
| `server_admin/src/api_settings.rs` | Pending |
| `server_admin/src/api_sign_in.rs` | Pending |
| `server_admin/src/api_sign_out.rs` | Pending |
| `server_admin/src/api_update_roles.rs` | Pending |
| `server_admin/src/api_update_roles_payload_example.rs` | Pending |
| `server_admin/src/api_update_settings.rs` | Pending |
| `server_admin/src/api_update_users.rs` | Pending |
| `server_admin/src/api_update_users_payload_example.rs` | Pending |
| `server_admin/src/api_user_roles_table.rs` | Reviewed: typed user-role POST read adapts its JSON query to the shared generic table handler. |
| `server_admin/src/append_cleared_session_cookies.rs` | Pending |
| `server_admin/src/append_session_cookies.rs` | Pending |
| `server_admin/src/application_auth.rs` | Pending |
| `server_admin/src/application_tests_helper.rs` | Pending |
| `server_admin/src/argon2_admin_password_hash_error.rs` | Pending |
| `server_admin/src/assignment_action.rs` | Pending |
| `server_admin/src/assignment_form_action.rs` | Pending |
| `server_admin/src/assignment_form_target.rs` | Pending |
| `server_admin/src/assignment_ids_impl.rs` | Pending |
| `server_admin/src/audit_log_read_page.rs` | Pending |
| `server_admin/src/authenticated_action_impl.rs` | Pending |
| `server_admin/src/authenticated_admin_contract.rs` | Pending |
| `server_admin/src/authn_apply_refresh_failure_delay.rs` | Pending |
| `server_admin/src/authn_refresh.rs` | Pending |
| `server_admin/src/authn_sign_in.rs` | Pending |
| `server_admin/src/authn_sign_out.rs` | Pending |
| `server_admin/src/authorization_authenticate.rs` | Pending |
| `server_admin/src/authorization_authorize_generated_request.rs` | Pending |
| `server_admin/src/authorization_hash_refresh_token_with_context.rs` | Pending |
| `server_admin/src/authorization_origin_is_present_and_allowed.rs` | Pending |
| `server_admin/src/authorization_session_context_hash.rs` | Pending |
| `server_admin/src/authorization_validate_csrf.rs` | Pending |
| `server_admin/src/authorize_custom.rs` | Pending |
| `server_admin/src/axum_admin_auth_router.rs` | Pending |
| `server_admin/src/axum_admin_form.rs` | Pending |
| `server_admin/src/axum_admin_json.rs` | Pending |
| `server_admin/src/axum_admin_path.rs` | Pending |
| `server_admin/src/axum_admin_query.rs` | Pending |
| `server_admin/src/axum_admin_response.rs` | Pending |
| `server_admin/src/axum_admin_state_router.rs` | Pending |
| `server_admin/src/base_sql.rs` | Pending |
| `server_admin/src/build_admin_cookie.rs` | Pending |
| `server_admin/src/change_password.rs` | Pending |
| `server_admin/src/change_password_form.rs` | Pending |
| `server_admin/src/cleanup_admin_tables.rs` | Pending |
| `server_admin/src/cleanup_status_read_page.rs` | Pending |
| `server_admin/src/clear_admin_cookie.rs` | Pending |
| `server_admin/src/confirmed_authenticated_action_impl.rs` | Pending |
| `server_admin/src/confirmed_delete_target.rs` | Pending |
| `server_admin/src/create_initial_administrator.rs` | Pending |
| `server_admin/src/create_role.rs` | Pending |
| `server_admin/src/create_role_form.rs` | Pending |
| `server_admin/src/create_session_in_connection.rs` | Pending |
| `server_admin/src/create_user.rs` | Pending |
| `server_admin/src/create_user_form.rs` | Pending |
| `server_admin/src/created_json_response.rs` | Pending |
| `server_admin/src/crud_page.rs` | Pending |
| `server_admin/src/crud_resource_page.rs` | Pending |
| `server_admin/src/csr_page.rs` | Pending |
| `server_admin/src/csr_table_page.rs` | Pending |
| `server_admin/src/data_access_sessions_flt.rs` | Pending |
| `server_admin/src/data_audit_log_flt.rs` | Pending |
| `server_admin/src/data_cleanup_status_flt.rs` | Pending |
| `server_admin/src/data_filter.rs` | Reviewed: incomplete, unknown, and mismatched filter shapes are rejected before typed predicate construction; focused tests cover table and operation variants. |
| `server_admin/src/data_flt.rs` | Pending |
| `server_admin/src/data_login_attempts_flt.rs` | Pending |
| `server_admin/src/data_permission_actions_flt.rs` | Pending |
| `server_admin/src/data_permission_resource_actions_flt.rs` | Pending |
| `server_admin/src/data_permission_resources_flt.rs` | Pending |
| `server_admin/src/data_rate_limits_flt.rs` | Pending |
| `server_admin/src/data_refresh_tokens_flt.rs` | Pending |
| `server_admin/src/data_role_rules_flt.rs` | Pending |
| `server_admin/src/data_roles_flt.rs` | Pending |
| `server_admin/src/data_rules_flt.rs` | Pending |
| `server_admin/src/data_system_settings_flt.rs` | Pending |
| `server_admin/src/data_table_query_sql.rs` | Fixed and reviewed; A114 pure SQL builder and catalog sort validation. |
| `server_admin/src/data_tables.rs` | Pending |
| `server_admin/src/data_tables_get.rs` | Fixed and reviewed; A114 honors search, sort, and direction, and A117 binds literal search patterns in generic table queries. |
| `server_admin/src/data_tables_list.rs` | Reviewed: authenticated catalog listing filters each table by the actor's read rule. |
| `server_admin/src/data_user_roles_flt.rs` | Pending |
| `server_admin/src/data_users_flt.rs` | Pending |
| `server_admin/src/decode_access_token.rs` | Pending |
| `server_admin/src/delete_confirmed_entity.rs` | Pending |
| `server_admin/src/delete_role.rs` | Pending |
| `server_admin/src/delete_user.rs` | Pending |
| `server_admin/src/dispatch_filtered_update.rs` | Pending |
| `server_admin/src/encode_access_token.rs` | Pending |
| `server_admin/src/enforce_rate_limit.rs` | Pending |
| `server_admin/src/enrich_access_sessions_read_page.rs` | Pending |
| `server_admin/src/enrich_audit_log_read_page.rs` | Pending |
| `server_admin/src/enrich_permission_actions_read_page.rs` | Pending |
| `server_admin/src/enrich_permission_resource_actions_read_page.rs` | Pending |
| `server_admin/src/enrich_permission_resources_read_page.rs` | Pending |
| `server_admin/src/enrich_role_rules_read_page.rs` | Pending |
| `server_admin/src/enrich_roles_read_page.rs` | Pending |
| `server_admin/src/enrich_rules_read_page.rs` | Pending |
| `server_admin/src/enrich_system_settings_read_page.rs` | Pending |
| `server_admin/src/enrich_users_database_read_page.rs` | Pending |
| `server_admin/src/finalize_audited_transaction.rs` | Pending |
| `server_admin/src/find_admin_cookie.rs` | Pending |
| `server_admin/src/form_auth_impl.rs` | Pending |
| `server_admin/src/generated_data_table_view.rs` | Pending |
| `server_admin/src/generated_open_api.rs` | Pending |
| `server_admin/src/generated_routes.rs` | Pending |
| `server_admin/src/hash_opaque_token.rs` | Pending |
| `server_admin/src/html_page_error_impl.rs` | Pending |
| `server_admin/src/html_response_impl.rs` | Pending |
| `server_admin/src/html_routes.rs` | Pending |
| `server_admin/src/html_routes_with_swagger.rs` | Pending |
| `server_admin/src/http_admin_header_map.rs` | Pending |
| `server_admin/src/http_admin_header_map_ref.rs` | Pending |
| `server_admin/src/http_admin_header_value_error.rs` | Pending |
| `server_admin/src/initial_administrator_creation_error.rs` | Pending |
| `server_admin/src/insert_audit_success.rs` | Pending |
| `server_admin/src/insert_user.rs` | Pending |
| `server_admin/src/json_response.rs` | Pending |
| `server_admin/src/jsonwebtoken_admin_decoding_keys.rs` | Pending |
| `server_admin/src/jsonwebtoken_admin_encoding_key.rs` | Pending |
| `server_admin/src/jsonwebtoken_admin_error.rs` | Pending |
| `server_admin/src/last_admin_state.rs` | Pending |
| `server_admin/src/lib.rs` | Pending |
| `server_admin/src/load_authenticated_admin_from_db.rs` | Pending |
| `server_admin/src/load_users_role_catalog.rs` | Pending |
| `server_admin/src/lock_last_admin.rs` | Pending |
| `server_admin/src/login_attempt_read_page.rs` | Pending |
| `server_admin/src/map_repository_error.rs` | Pending |
| `server_admin/src/map_unique_violation.rs` | Pending |
| `server_admin/src/migrator.rs` | Pending |
| `server_admin/src/optional_setting_impl.rs` | Pending |
| `server_admin/src/page_context_impl.rs` | Pending |
| `server_admin/src/page_total.rs` | Pending |
| `server_admin/src/permission_action_read_page.rs` | Pending |
| `server_admin/src/permission_resource_action_read_page.rs` | Pending |
| `server_admin/src/permission_resource_read_page.rs` | Pending |
| `server_admin/src/prepare_postgresql.rs` | Pending |
| `server_admin/src/profile.rs` | Pending |
| `server_admin/src/queries_users_page.rs` | Pending |
| `server_admin/src/rate_limit_read_page.rs` | Pending |
| `server_admin/src/rbac.rs` | Pending |
| `server_admin/src/read_last_admin_state.rs` | Pending |
| `server_admin/src/read_settings.rs` | Pending |
| `server_admin/src/record_audit_success_in_connection.rs` | Pending |
| `server_admin/src/record_login_attempt.rs` | Pending |
| `server_admin/src/refresh_token_read_page.rs` | Pending |
| `server_admin/src/replace_user_roles_in_connection.rs` | Pending |
| `server_admin/src/repository_page_total.rs` | Pending |
| `server_admin/src/reset_admin_password.rs` | Pending |
| `server_admin/src/revoke_access_session.rs` | Pending |
| `server_admin/src/revoke_refresh_token.rs` | Pending |
| `server_admin/src/revoke_session.rs` | Pending |
| `server_admin/src/revoke_session_form.rs` | Pending |
| `server_admin/src/revoke_user_sessions.rs` | Pending |
| `server_admin/src/role_id_form.rs` | Pending |
| `server_admin/src/role_ids_impl.rs` | Pending |
| `server_admin/src/role_mutations_create_many.rs` | Pending |
| `server_admin/src/role_mutations_delete_filtered.rs` | Pending |
| `server_admin/src/role_mutations_update_many.rs` | Pending |
| `server_admin/src/role_read_page.rs` | Pending |
| `server_admin/src/role_rule_read_page.rs` | Pending |
| `server_admin/src/role_rules.rs` | Pending |
| `server_admin/src/role_rules_form.rs` | Pending |
| `server_admin/src/roles.rs` | Pending |
| `server_admin/src/roles_create_page.rs` | Pending |
| `server_admin/src/roles_manage_page.rs` | Pending |
| `server_admin/src/roles_update_page.rs` | Pending |
| `server_admin/src/root.rs` | Pending |
| `server_admin/src/rule_ids_impl.rs` | Pending |
| `server_admin/src/rule_read_page.rs` | Pending |
| `server_admin/src/rules.rs` | Pending |
| `server_admin/src/runtime_admin_cookie_secure.rs` | Pending |
| `server_admin/src/runtime_admin_jwt_secret.rs` | Pending |
| `server_admin/src/runtime_admin_password.rs` | Pending |
| `server_admin/src/runtime_admin_password_hash_concurrency.rs` | Pending |
| `server_admin/src/runtime_admin_role_names.rs` | Pending |
| `server_admin/src/runtime_authenticated_admin.rs` | Pending |
| `server_admin/src/select_filtered_role_ids.rs` | Pending |
| `server_admin/src/select_filtered_user_ids.rs` | Pending |
| `server_admin/src/sessions.rs` | Pending |
| `server_admin/src/sessions_revoke_all_sessions.rs` | Pending |
| `server_admin/src/sessions_revoke_session.rs` | Pending |
| `server_admin/src/settings.rs` | Pending |
| `server_admin/src/settings_branding.rs` | Pending |
| `server_admin/src/settings_branding_view.rs` | Pending |
| `server_admin/src/settings_branding_view_ref.rs` | Pending |
| `server_admin/src/settings_form.rs` | Pending |
| `server_admin/src/settings_get.rs` | Pending |
| `server_admin/src/settings_update.rs` | Pending |
| `server_admin/src/shared_admin_auth_svc_state_arc.rs` | Pending |
| `server_admin/src/shared_admin_generated_table_state_arc.rs` | Pending |
| `server_admin/src/sign_in.rs` | Pending |
| `server_admin/src/sign_in_form.rs` | Pending |
| `server_admin/src/sign_in_page.rs` | Pending |
| `server_admin/src/sign_out.rs` | Pending |
| `server_admin/src/sqlx_admin_error.rs` | Pending |
| `server_admin/src/sqlx_admin_migrate_error.rs` | Pending |
| `server_admin/src/sqlx_admin_migrator_ref.rs` | Pending |
| `server_admin/src/sqlx_admin_repository_connection_mut_ref.rs` | Pending |
| `server_admin/src/sqlx_admin_repository_pool_ref.rs` | Pending |
| `server_admin/src/sqlx_admin_transaction.rs` | Pending |
| `server_admin/src/std_admin_access_token.rs` | Pending |
| `server_admin/src/std_admin_access_ttl_seconds.rs` | Pending |
| `server_admin/src/std_admin_auth_ttl_seconds.rs` | Pending |
| `server_admin/src/std_admin_cookie.rs` | Pending |
| `server_admin/src/std_admin_failure_delay_millis.rs` | Pending |
| `server_admin/src/std_admin_failure_threshold.rs` | Pending |
| `server_admin/src/std_admin_html_selected.rs` | Pending |
| `server_admin/src/std_admin_html_selected_error.rs` | Pending |
| `server_admin/src/std_admin_rate_limit_count.rs` | Pending |
| `server_admin/src/std_admin_rate_limit_window_seconds.rs` | Pending |
| `server_admin/src/std_admin_refresh_ttl_seconds.rs` | Pending |
| `server_admin/src/std_admin_session_limit.rs` | Pending |
| `server_admin/src/success_redirect_impl.rs` | Pending |
| `server_admin/src/system_setting_read_page.rs` | Pending |
| `server_admin/src/test_adapters_repository_data_tables_tests.rs` | Focused review; A114 generic table SQL regressions. |
| `server_admin/src/test_adapters_repository_roles_tests.rs` | Pending |
| `server_admin/src/test_admin_service_tests.rs` | Pending |
| `server_admin/src/test_application_html_tests.rs` | Pending |
| `server_admin/src/test_application_tests.rs` | Pending |
| `server_admin/src/test_domain_types_generated_tables_tests.rs` | Pending |
| `server_admin/src/test_generated_data_table_view.rs` | Pending |
| `server_admin/src/test_maintenance_tests.rs` | Pending |
| `server_admin/src/test_shared_tests.rs` | Pending |
| `server_admin/src/test_tests_domain_types.rs` | Pending |
| `server_admin/src/token.rs` | Pending |
| `server_admin/src/tokio_admin_acquire_error.rs` | Pending |
| `server_admin/src/tokio_admin_join_error.rs` | Pending |
| `server_admin/src/tokio_admin_owned_semaphore_permit.rs` | Pending |
| `server_admin/src/update_role.rs` | Pending |
| `server_admin/src/update_role_form.rs` | Pending |
| `server_admin/src/update_settings.rs` | Pending |
| `server_admin/src/update_user.rs` | Pending |
| `server_admin/src/update_user_form.rs` | Pending |
| `server_admin/src/update_user_password.rs` | Pending |
| `server_admin/src/user_ban.rs` | Pending |
| `server_admin/src/user_ban_form.rs` | Pending |
| `server_admin/src/user_id_form.rs` | Pending |
| `server_admin/src/user_mutation_form_action.rs` | Pending |
| `server_admin/src/user_mutation_form_target.rs` | Pending |
| `server_admin/src/user_mutations_create.rs` | Pending |
| `server_admin/src/user_mutations_delete_filtered.rs` | Pending |
| `server_admin/src/user_mutations_update.rs` | Pending |
| `server_admin/src/user_mutations_update_filtered.rs` | Pending |
| `server_admin/src/user_password.rs` | Pending |
| `server_admin/src/user_password_form.rs` | Pending |
| `server_admin/src/user_path_impl.rs` | Pending |
| `server_admin/src/user_read_page.rs` | Pending |
| `server_admin/src/user_role_read_page.rs` | Pending |
| `server_admin/src/user_roles.rs` | Pending |
| `server_admin/src/user_roles_form.rs` | Pending |
| `server_admin/src/users.rs` | Pending |
| `server_admin/src/users_create_page.rs` | Pending |
| `server_admin/src/users_manage_page.rs` | Pending |
| `server_admin/src/users_update_page.rs` | Pending |
| `server_admin/src/utoipa_admin_auth_open_api.rs` | Pending |
| `server_admin/src/utoipa_admin_open_api.rs` | Pending |
| `server_admin/src/validate_admin_access_claims.rs` | Pending |
| `server_admin/src/validate_catalog_schema.rs` | Pending |
| `server_admin/src/validate_table_sort.rs` | Pending |
| `server_admin/src/version.rs` | Pending |
| `server_admin/tests/admin_api.rs` | Focused review: A32 adds a compiled provisioned PostgreSQL regression for duplicate idempotency completion; broader API integration coverage remains under review. |

### server_admin_contract

| Source | Semantic review |
| --- | --- |
| `server_admin_contract/src/admin_access_session_filter.rs` | Reviewed: typed optional session and user identifiers, with unknown fields denied. |
| `server_admin_contract/src/admin_access_session_id.rs` | Reviewed: fixed-length hex identifier and exact detail-path parsing; contract tests pass (`target/audit_tmp/a111_contract_review.log`). |
| `server_admin_contract/src/admin_access_sessions_read_request.rs` | Reviewed: rejects unsupported sort keys and builds the default created-at order with typed pagination. |
| `server_admin_contract/src/admin_access_sessions_table_route.rs` | Pending |
| `server_admin_contract/src/admin_api_body_max_bytes.rs` | Reviewed: typed size wrapper and fixed body-size limit. |
| `server_admin_contract/src/admin_api_route_path.rs` | Pending |
| `server_admin_contract/src/admin_audit_cursor.rs` | Reviewed: typed timestamp and identifier cursor fields. |
| `server_admin_contract/src/admin_audit_details_bytes.rs` | Reviewed: typed byte count wrapper. |
| `server_admin_contract/src/admin_audit_details_max_bytes.rs` | Reviewed: fixed audit detail limit. |
| `server_admin_contract/src/admin_audit_details_too_large.rs` | Reviewed: reports actual and maximum bytes through typed accessors. |
| `server_admin_contract/src/admin_audit_log_id.rs` | Reviewed: positive identifier conversion and exact detail-path parsing; contract tests pass (`target/audit_tmp/a111_contract_review.log`). |
| `server_admin_contract/src/admin_audit_log_read_request.rs` | Reviewed: maps supported audit sort keys, default ordering, search, and typed pagination. |
| `server_admin_contract/src/admin_audit_page.rs` | Reviewed: bounded audit views, optional cursor, and typed total. |
| `server_admin_contract/src/admin_audit_timestamp.rs` | Focused review: bounded response text; timestamp format validation remains unreviewed at its API boundaries. |
| `server_admin_contract/src/admin_audit_view.rs` | Reviewed: typed audit row fields and optional details. |
| `server_admin_contract/src/admin_audit_views.rs` | Reviewed: delegates collection construction and deserialization to the bounded collection wrapper. |
| `server_admin_contract/src/admin_bool.rs` | Reviewed: typed bool conversion and validated serde representation. |
| `server_admin_contract/src/admin_bounded_vec.rs` | Fixed and reviewed; A111 preserves the bounded error source. |
| `server_admin_contract/src/admin_branding_route.rs` | Pending |
| `server_admin_contract/src/admin_branding_view.rs` | Pending |
| `server_admin_contract/src/admin_change_own_password_request.rs` | Pending |
| `server_admin_contract/src/admin_change_own_password_route.rs` | Pending |
| `server_admin_contract/src/admin_cleanup_status_id.rs` | Reviewed: positive identifier conversion and exact detail-path parsing; contract tests pass (`target/audit_tmp/a111_contract_review.log`). |
| `server_admin_contract/src/admin_cleanup_status_table_route.rs` | Fixed and reviewed; A112 table read rule metadata. |
| `server_admin_contract/src/admin_collection_error.rs` | Fixed and reviewed; A111 preserves the typed bounded error source. |
| `server_admin_contract/src/admin_collection_max_items.rs` | Reviewed: fixed shared collection item limit. |
| `server_admin_contract/src/admin_create_role_request.rs` | Pending |
| `server_admin_contract/src/admin_create_roles_payload_example_route.rs` | Pending |
| `server_admin_contract/src/admin_create_roles_request.rs` | Pending |
| `server_admin_contract/src/admin_create_roles_route.rs` | Pending |
| `server_admin_contract/src/admin_create_user_payload_example_route.rs` | Pending |
| `server_admin_contract/src/admin_create_user_request.rs` | Pending |
| `server_admin_contract/src/admin_create_user_response.rs` | Pending |
| `server_admin_contract/src/admin_create_user_route.rs` | Pending |
| `server_admin_contract/src/admin_create_users_request.rs` | Pending |
| `server_admin_contract/src/admin_data_column.rs` | Focused review: typed column metadata and bounded filter collection. |
| `server_admin_contract/src/admin_data_columns.rs` | Focused review: bounded column collection and schema limit. |
| `server_admin_contract/src/admin_data_columns_csv_ref.rs` | Focused review: borrowed catalog column specification. |
| `server_admin_contract/src/admin_data_filter.rs` | Fixed and reviewed; A113 validates operation and value shape. |
| `server_admin_contract/src/admin_data_filters.rs` | Fixed and reviewed; A118 enforces the declared 100-filter limit. |
| `server_admin_contract/src/admin_data_order_ref.rs` | Focused review: borrowed catalog ordering specification. |
| `server_admin_contract/src/admin_data_row.rs` | Focused review: bounded row value ownership. |
| `server_admin_contract/src/admin_data_rows.rs` | Focused review: bounded row collection and schema limit. |
| `server_admin_contract/src/admin_data_table.rs` | Focused review: catalog route and rule mappings, table specifications, and frontend path conversion. |
| `server_admin_contract/src/admin_data_table_catalog.rs` | Focused review: bounded table catalog representation. |
| `server_admin_contract/src/admin_data_table_filter_query.rs` | Focused review: typed optional filter query fields. |
| `server_admin_contract/src/admin_data_table_frontend_path.rs` | Focused review: table frontend path construction. |
| `server_admin_contract/src/admin_data_table_query.rs` | Fixed and reviewed; A114 search and sort fields now drive the generic table handler. |
| `server_admin_contract/src/admin_data_table_spec.rs` | Focused review: typed catalog specification fields. |
| `server_admin_contract/src/admin_data_table_str_ref.rs` | Focused review: borrowed table wire name. |
| `server_admin_contract/src/admin_data_table_view.rs` | Focused review: typed columns, rows, table and total. |
| `server_admin_contract/src/admin_data_tables.rs` | Reviewed: catalog table list enforces its declared 10,000-item collection bound. |
| `server_admin_contract/src/admin_data_tables_route.rs` | Reviewed: read-only table catalog route requires the TablesRead rule and matches its handler. |
| `server_admin_contract/src/admin_default_page_limit.rs` | Reviewed: marker initializes the validated default page limit. |
| `server_admin_contract/src/admin_default_route.rs` | Reviewed: default route validator accepts cataloged page and data-table paths. |
| `server_admin_contract/src/admin_delete_access_sessions_request.rs` | Pending |
| `server_admin_contract/src/admin_delete_access_sessions_route.rs` | Pending |
| `server_admin_contract/src/admin_delete_roles_payload_example_route.rs` | Pending |
| `server_admin_contract/src/admin_delete_roles_request.rs` | Pending |
| `server_admin_contract/src/admin_delete_roles_route.rs` | Pending |
| `server_admin_contract/src/admin_delete_users_payload_example_route.rs` | Pending |
| `server_admin_contract/src/admin_delete_users_request.rs` | Pending |
| `server_admin_contract/src/admin_delete_users_route.rs` | Pending |
| `server_admin_contract/src/admin_display_name.rs` | Pending |
| `server_admin_contract/src/admin_empty_collection.rs` | Pending |
| `server_admin_contract/src/admin_filter_field.rs` | Focused review: bounded filter field text. |
| `server_admin_contract/src/admin_filter_operation_key.rs` | Focused review: operation key conversion from filter enum names. |
| `server_admin_contract/src/admin_filter_value.rs` | Focused review: bounded filter value text. |
| `server_admin_contract/src/admin_frontend_path.rs` | Reviewed: page path enum and route registration share path values. |
| `server_admin_contract/src/admin_html_action.rs` | Pending |
| `server_admin_contract/src/admin_id_try_from_i64_error.rs` | Pending |
| `server_admin_contract/src/admin_identifier_filter_error.rs` | Fixed and reviewed; A105 timestamp filter diagnostics. |
| `server_admin_contract/src/admin_login.rs` | Pending |
| `server_admin_contract/src/admin_login_attempt_id.rs` | Pending |
| `server_admin_contract/src/admin_login_attempts_table_route.rs` | Fixed and reviewed; A112 table read rule metadata. |
| `server_admin_contract/src/admin_main_logo.rs` | Fixed and reviewed; A115 HTTPS URL authority validation. |
| `server_admin_contract/src/admin_me_route.rs` | Pending |
| `server_admin_contract/src/admin_new_password.rs` | Pending |
| `server_admin_contract/src/admin_no_body.rs` | Pending |
| `server_admin_contract/src/admin_open_api_vec.rs` | Reviewed: typed array schema and shared item schema registration; A135. |
| `server_admin_contract/src/admin_open_api_vec_phantom_data.rs` | Pending |
| `server_admin_contract/src/admin_optional_setting.rs` | Reviewed: clearable settings are explicit catalog variants. |
| `server_admin_contract/src/admin_optional_settings.rs` | Fixed and reviewed; A119 enforces the six-field clear list bound. |
| `server_admin_contract/src/admin_organization_contacts.rs` | Reviewed: optional contact text uses bounded validated storage. |
| `server_admin_contract/src/admin_organization_name.rs` | Reviewed: optional organization text uses bounded validated storage. |
| `server_admin_contract/src/admin_page.rs` | Focused review; A93 |
| `server_admin_contract/src/admin_page_capability.rs` | Reviewed: page availability is expressed by explicit variants. |
| `server_admin_contract/src/admin_page_client_mode.rs` | Reviewed: CSR and table-query predicates match the three page modes. |
| `server_admin_contract/src/admin_page_limit.rs` | Focused review; A116 pagination validation. |
| `server_admin_contract/src/admin_page_limit_error.rs` | Reviewed: out-of-range limit error reports the validated minimum and maximum. |
| `server_admin_contract/src/admin_page_limit_visitor.rs` | Fixed and reviewed; A116 signed and wide integer deserialization. |
| `server_admin_contract/src/admin_page_metadata.rs` | Reviewed: page metadata stores mode and optional navigation order. |
| `server_admin_contract/src/admin_page_navigation.rs` | Reviewed: navigation order uses explicit ordered variants. |
| `server_admin_contract/src/admin_page_offset.rs` | Focused review; A116 pagination validation. |
| `server_admin_contract/src/admin_page_offset_visitor.rs` | Fixed and reviewed; A116 signed and wide integer deserialization. |
| `server_admin_contract/src/admin_page_path_ref.rs` | Reviewed: record extraction requires a table path, separator, parseable positive identifier. |
| `server_admin_contract/src/admin_page_spec.rs` | Reviewed: page route, path, title, client mode, and navigation derive from one spec. |
| `server_admin_contract/src/admin_page_title.rs` | Reviewed: title variants are mapped by the page spec. |
| `server_admin_contract/src/admin_page_total.rs` | Reviewed: page totals are represented as nonnegative u64 values. |
| `server_admin_contract/src/admin_parameterized_route_path.rs` | Reviewed: parameterized paths delegate to the typed route path helper and API path wrapper. |
| `server_admin_contract/src/admin_password.rs` | Pending |
| `server_admin_contract/src/admin_password_entropy.rs` | Pending |
| `server_admin_contract/src/admin_path_route_name.rs` | Reviewed: route names derive from the final static path segment. |
| `server_admin_contract/src/admin_permission_action_id.rs` | Pending |
| `server_admin_contract/src/admin_permission_actions_read_request.rs` | Reviewed: permission-action sort keys map to typed ID or key columns and retain pagination. |
| `server_admin_contract/src/admin_permission_actions_table_route.rs` | Pending |
| `server_admin_contract/src/admin_permission_resource_action_id.rs` | Pending |
| `server_admin_contract/src/admin_permission_resource_actions_read_request.rs` | Reviewed: resource-action sort keys map to typed identifier columns and retain pagination. |
| `server_admin_contract/src/admin_permission_resource_actions_table_route.rs` | Pending |
| `server_admin_contract/src/admin_permission_resource_id.rs` | Pending |
| `server_admin_contract/src/admin_permission_resources_read_request.rs` | Reviewed: permission-resource sort keys map to typed ID or key columns and retain pagination. |
| `server_admin_contract/src/admin_permission_resources_table_route.rs` | Pending |
| `server_admin_contract/src/admin_primary_color.rs` | Reviewed: exact seven-byte ASCII hexadecimal color validation. |
| `server_admin_contract/src/admin_rate_limit_id.rs` | Pending |
| `server_admin_contract/src/admin_rate_limits_table_route.rs` | Fixed and reviewed; A112 table read rule metadata. |
| `server_admin_contract/src/admin_read_access_session_column.rs` | Pending |
| `server_admin_contract/src/admin_read_access_session_order.rs` | Pending |
| `server_admin_contract/src/admin_read_access_session_selection.rs` | Pending |
| `server_admin_contract/src/admin_read_audit_log_column.rs` | Pending |
| `server_admin_contract/src/admin_read_audit_log_order.rs` | Pending |
| `server_admin_contract/src/admin_read_audit_log_route.rs` | Pending |
| `server_admin_contract/src/admin_read_audit_log_selection.rs` | Pending |
| `server_admin_contract/src/admin_read_field.rs` | Pending |
| `server_admin_contract/src/admin_read_page.rs` | Reviewed: request pagination stores the validated offset and limit wrappers. |
| `server_admin_contract/src/admin_read_permission_action_column.rs` | Pending |
| `server_admin_contract/src/admin_read_permission_action_order.rs` | Reviewed: typed action column and sort direction are serialized together. |
| `server_admin_contract/src/admin_read_permission_action_selection.rs` | Reviewed: default selection includes both action ID and key. |
| `server_admin_contract/src/admin_read_permission_resource_action_column.rs` | Pending |
| `server_admin_contract/src/admin_read_permission_resource_action_order.rs` | Reviewed: typed resource-action column and sort direction are serialized together. |
| `server_admin_contract/src/admin_read_permission_resource_action_selection.rs` | Reviewed: default selection includes association ID and both referenced IDs. |
| `server_admin_contract/src/admin_read_permission_resource_column.rs` | Pending |
| `server_admin_contract/src/admin_read_permission_resource_order.rs` | Reviewed: typed resource column and sort direction are serialized together. |
| `server_admin_contract/src/admin_read_permission_resource_selection.rs` | Reviewed: default selection includes both resource ID and key. |
| `server_admin_contract/src/admin_read_role_column.rs` | Pending |
| `server_admin_contract/src/admin_read_role_order.rs` | Pending |
| `server_admin_contract/src/admin_read_role_rule_column.rs` | Pending |
| `server_admin_contract/src/admin_read_role_rule_order.rs` | Pending |
| `server_admin_contract/src/admin_read_role_rule_selection.rs` | Pending |
| `server_admin_contract/src/admin_read_role_selection.rs` | Pending |
| `server_admin_contract/src/admin_read_roles_route.rs` | Pending |
| `server_admin_contract/src/admin_read_rule_column.rs` | Pending |
| `server_admin_contract/src/admin_read_rule_order.rs` | Pending |
| `server_admin_contract/src/admin_read_rule_selection.rs` | Pending |
| `server_admin_contract/src/admin_read_rules_route.rs` | Pending |
| `server_admin_contract/src/admin_read_system_settings_column.rs` | Pending |
| `server_admin_contract/src/admin_read_system_settings_order.rs` | Pending |
| `server_admin_contract/src/admin_read_system_settings_route.rs` | Pending |
| `server_admin_contract/src/admin_read_system_settings_selection.rs` | Pending |
| `server_admin_contract/src/admin_read_user_column.rs` | Pending |
| `server_admin_contract/src/admin_read_user_order.rs` | Pending |
| `server_admin_contract/src/admin_read_user_selection.rs` | Pending |
| `server_admin_contract/src/admin_read_users_route.rs` | Pending |
| `server_admin_contract/src/admin_refresh_route.rs` | Pending |
| `server_admin_contract/src/admin_refresh_token_id.rs` | Pending |
| `server_admin_contract/src/admin_refresh_tokens_table_route.rs` | Fixed and reviewed; A112 table read rule metadata. |
| `server_admin_contract/src/admin_revoke_all_sessions_route.rs` | Pending |
| `server_admin_contract/src/admin_revoke_session_route.rs` | Pending |
| `server_admin_contract/src/admin_role_filter.rs` | Pending |
| `server_admin_contract/src/admin_role_id.rs` | Pending |
| `server_admin_contract/src/admin_role_ids.rs` | Pending |
| `server_admin_contract/src/admin_role_name.rs` | Pending |
| `server_admin_contract/src/admin_role_names.rs` | Pending |
| `server_admin_contract/src/admin_role_rule_id.rs` | Pending |
| `server_admin_contract/src/admin_role_rules_read_request.rs` | Reviewed: unsupported sort is rejected and the default created-at ordering uses typed pagination. |
| `server_admin_contract/src/admin_role_rules_table_route.rs` | Pending |
| `server_admin_contract/src/admin_role_summaries.rs` | Pending |
| `server_admin_contract/src/admin_role_summary.rs` | Pending |
| `server_admin_contract/src/admin_role_timestamp.rs` | Pending |
| `server_admin_contract/src/admin_role_update.rs` | Pending |
| `server_admin_contract/src/admin_role_updates.rs` | Pending |
| `server_admin_contract/src/admin_roles_page.rs` | Pending |
| `server_admin_contract/src/admin_roles_read_request.rs` | Reviewed: role sort keys, default ordering, search, and typed pagination map to the read request. |
| `server_admin_contract/src/admin_route.rs` | Pending |
| `server_admin_contract/src/admin_route_path.rs` | Pending |
| `server_admin_contract/src/admin_route_path_error.rs` | Pending |
| `server_admin_contract/src/admin_rule.rs` | Pending |
| `server_admin_contract/src/admin_rule_id.rs` | Pending |
| `server_admin_contract/src/admin_rule_ids.rs` | Pending |
| `server_admin_contract/src/admin_rule_requirement.rs` | Pending |
| `server_admin_contract/src/admin_rule_str_ref.rs` | Pending |
| `server_admin_contract/src/admin_rule_summaries.rs` | Pending |
| `server_admin_contract/src/admin_rule_summary.rs` | Pending |
| `server_admin_contract/src/admin_rule_timestamp.rs` | Pending |
| `server_admin_contract/src/admin_rule_value.rs` | Pending |
| `server_admin_contract/src/admin_rule_values.rs` | Pending |
| `server_admin_contract/src/admin_rules_page.rs` | Pending |
| `server_admin_contract/src/admin_rules_read_request.rs` | Reviewed: rule sort keys map to typed rule columns and retain pagination. |
| `server_admin_contract/src/admin_selected_value.rs` | Pending |
| `server_admin_contract/src/admin_session_identifier.rs` | Pending |
| `server_admin_contract/src/admin_session_timestamp.rs` | Pending |
| `server_admin_contract/src/admin_session_view.rs` | Pending |
| `server_admin_contract/src/admin_session_views.rs` | Pending |
| `server_admin_contract/src/admin_sessions_page.rs` | Pending |
| `server_admin_contract/src/admin_sessions_route.rs` | Pending |
| `server_admin_contract/src/admin_set_role_rules_request.rs` | Pending |
| `server_admin_contract/src/admin_setting.rs` | Pending |
| `server_admin_contract/src/admin_setting_input_kind.rs` | Pending |
| `server_admin_contract/src/admin_setting_label.rs` | Pending |
| `server_admin_contract/src/admin_setting_name.rs` | Pending |
| `server_admin_contract/src/admin_setting_optionality.rs` | Pending |
| `server_admin_contract/src/admin_setting_spec.rs` | Pending |
| `server_admin_contract/src/admin_settings_route.rs` | Pending |
| `server_admin_contract/src/admin_settings_view.rs` | Pending |
| `server_admin_contract/src/admin_sign_in_request.rs` | Pending |
| `server_admin_contract/src/admin_sign_in_response.rs` | Pending |
| `server_admin_contract/src/admin_sign_in_route.rs` | Pending |
| `server_admin_contract/src/admin_sign_out_route.rs` | Pending |
| `server_admin_contract/src/admin_site_name.rs` | Reviewed: required site name rejects whitespace-only text. |
| `server_admin_contract/src/admin_sort_direction.rs` | Pending |
| `server_admin_contract/src/admin_support_url.rs` | Fixed and reviewed; A115 HTTPS URL authority validation. |
| `server_admin_contract/src/admin_system_setting_id.rs` | Pending |
| `server_admin_contract/src/admin_system_settings_read_request.rs` | Reviewed: unsupported sort is rejected and settings search and pagination are retained. |
| `server_admin_contract/src/admin_tab_title.rs` | Reviewed: optional title value rejects whitespace-only text. |
| `server_admin_contract/src/admin_table_query.rs` | Reviewed: default pagination and bounded search and sort fields match query parameter metadata. |
| `server_admin_contract/src/admin_table_search.rs` | Reviewed: search text has the declared 128-character bound. |
| `server_admin_contract/src/admin_table_sort_field.rs` | Reviewed: per-table option arrays restrict duplicate wire keys to the intended typed sort fields. |
| `server_admin_contract/src/admin_table_sort_field_try_from_key_error.rs` | Pending |
| `server_admin_contract/src/admin_table_sort_key.rs` | Reviewed: sort text has the declared 32-character bound; callers validate catalog keys. |
| `server_admin_contract/src/admin_table_sort_key_ref.rs` | Pending |
| `server_admin_contract/src/admin_table_sort_values.rs` | Pending |
| `server_admin_contract/src/admin_text.rs` | Pending |
| `server_admin_contract/src/admin_texts.rs` | Pending |
| `server_admin_contract/src/admin_unsearchable_read_query.rs` | Reviewed: unsupported sort is rejected and pagination is retained for fixed-order reads. |
| `server_admin_contract/src/admin_update_role_request.rs` | Pending |
| `server_admin_contract/src/admin_update_roles_payload_example_route.rs` | Pending |
| `server_admin_contract/src/admin_update_roles_request.rs` | Pending |
| `server_admin_contract/src/admin_update_roles_route.rs` | Pending |
| `server_admin_contract/src/admin_update_settings_request.rs` | Reviewed: empty updates, duplicate clears, six-item clear limit, and clear/set conflicts are validated before mutation. |
| `server_admin_contract/src/admin_update_settings_route.rs` | Pending |
| `server_admin_contract/src/admin_update_user_request.rs` | Pending |
| `server_admin_contract/src/admin_update_users_payload_example_route.rs` | Pending |
| `server_admin_contract/src/admin_update_users_request.rs` | Pending |
| `server_admin_contract/src/admin_update_users_route.rs` | Pending |
| `server_admin_contract/src/admin_user_filter.rs` | Pending |
| `server_admin_contract/src/admin_user_id.rs` | Pending |
| `server_admin_contract/src/admin_user_ids.rs` | Pending |
| `server_admin_contract/src/admin_user_role_id.rs` | Pending |
| `server_admin_contract/src/admin_user_roles_table_route.rs` | Fixed and reviewed; A112 table read rule metadata. |
| `server_admin_contract/src/admin_user_summaries.rs` | Pending |
| `server_admin_contract/src/admin_user_summary.rs` | Pending |
| `server_admin_contract/src/admin_user_update.rs` | Pending |
| `server_admin_contract/src/admin_user_updates.rs` | Pending |
| `server_admin_contract/src/admin_users_page.rs` | Pending |
| `server_admin_contract/src/admin_users_read_request.rs` | Reviewed: user sort keys, default ordering, search, and typed pagination map to the read request. |
| `server_admin_contract/src/admin_where_many.rs` | Fixed and reviewed; A104 numeric relation and A105 timestamp filter conversion. |
| `server_admin_contract/src/admin_where_many_try_from_string_error.rs` | Pending |
| `server_admin_contract/src/authenticated_admin.rs` | Pending |
| `server_admin_contract/src/default_admin_api_body_max_bytes.rs` | Pending |
| `server_admin_contract/src/identity.rs` | Pending |
| `server_admin_contract/src/lib.rs` | Focused review; A116 checked integer visitor helper module. |
| `server_admin_contract/src/positive_non_zero_i64.rs` | Pending |
| `server_admin_contract/src/serde_json_admin_audit_details.rs` | Pending |
| `server_admin_contract/src/test_delete_access_sessions_contract.rs` | Pending |
| `server_admin_contract/src/test_delete_roles_contract.rs` | Pending |
| `server_admin_contract/src/test_delete_users_contract.rs` | Pending |
| `server_admin_contract/src/test_domain_types_dto_tests.rs` | Pending |
| `server_admin_contract/src/test_domain_types_query_tests.rs` | Pending |
| `server_admin_contract/src/test_domain_types_routes_tests.rs` | Pending |
| `server_admin_contract/src/test_domain_types_sessions_tests.rs` | Pending |
| `server_admin_contract/src/test_domain_types_settings_tests.rs` | Focused review; A119 six-field clear list regression. |
| `server_admin_contract/src/test_tests_domain_types.rs` | Focused review; A93, A111, A112, A113, A115, A116, A117, and A118 regressions. |
| `server_admin_contract/src/test_update_users_contract.rs` | Pending |
| `server_admin_contract/src/visit_checked_unsigned_integer.rs` | Fixed and reviewed; A116 shared checked integer conversion for pagination visitors. |

### server_admin_core

| Source | Semantic review |
| --- | --- |
| `server_admin_core/src/admin_entity_id_from_i64.rs` | Reviewed: positive nonzero conversion maps invalid IDs to a typed domain error. |
| `server_admin_core/src/admin_entity_id_try_from_i64_error.rs` | Reviewed: invalid entity IDs use one typed error. |
| `server_admin_core/src/admin_resource_text.rs` | Reviewed: enum distinguishes positive IDs, system settings, and UUID resource text. |
| `server_admin_core/src/admin_role_record_id.rs` | Reviewed: validated ID wrapper deserializes through positive nonzero conversion. |
| `server_admin_core/src/admin_socket_addr.rs` | Reviewed: typed socket-address wrapper forwards owned and borrowed access. |
| `server_admin_core/src/admin_user_record_id.rs` | Reviewed: validated user ID deserializes through positive nonzero conversion. |
| `server_admin_core/src/escape_admin_like_pattern.rs` | Fixed and reviewed; A117 escapes PostgreSQL LIKE pattern characters with bounded output. |
| `server_admin_core/src/lib.rs` | Reviewed: root declarations, bounded-string validator reference, and A117 pattern helper module are consistent. |
| `server_admin_core/src/secrecy_admin_string.rs` | Reviewed: secret stores bounded text, redacts Debug, and exposes through secrecy's trait. |
| `server_admin_core/src/std_admin_bool.rs` | Reviewed: boolean wrapper deserializes through its typed conversion. |
| `server_admin_core/src/std_admin_str_ref.rs` | Reviewed: borrowed string wrapper preserves its lifetime. |
| `server_admin_core/src/std_admin_string.rs` | Reviewed: bounded internal text supports zeroization and bounded resource formatting. |
| `server_admin_core/src/tests_domain_types.rs` | Reviewed: tests cover secret length, redaction, zeroization, and resource values. |
| `server_admin_core/src/uuid_admin_value.rs` | Reviewed: UUID wrapper serializes and deserializes through typed conversion. |

### server_app_state

| Source | Semantic review |
| --- | --- |
| `server_app_state/src/lib.rs` | Reviewed: module declarations and test utility gating are consistent. |
| `server_app_state/src/make_test_server_app_state.rs` | Reviewed: test state uses validated configuration values and a lazy database pool. |
| `server_app_state/src/server_app_state.rs` | Reviewed: provider implementations forward the corresponding state and configuration fields. |
| `server_app_state/src/test_env.rs` | Reviewed: test conversion propagates invalid fixtures through a diagnostic failure. |
| `server_app_state/src/test_server_app_state.rs` | Reviewed: tests cover configuration forwarding, pool identity, and Git metadata. |

### server_config

| Source | Semantic review |
| --- | --- |
| `server_config/src/lib.rs` | Reviewed: flat module and test declarations. |
| `server_config/src/production_config_error.rs` | Reviewed: typed production-validation failures disclose no secret data. |
| `server_config/src/server_config.rs` | Reviewed: typed configuration providers, generated accessors, production cookie/Swagger/CORS/JWT validation; A31 aligns HTTPS scheme case handling with the origin parser. |
| `server_config/src/tests_domain_types.rs` | Reviewed: generated accessors and every production validation branch, including the A31 HTTPS/HTTP case regression. |
| `server_config/tests/config_descriptor.rs` | Reviewed: generated environment example, dotenv parsing, descriptor count and public-field parser validation; package tests pass. |

### server_observability

| Source | Semantic review |
| --- | --- |
| `server_observability/src/init_service_observability.rs` | Reviewed: optional OTLP setup, subscriber initialization, and cleanup preserve failures. |
| `server_observability/src/initialization_tests.rs` | Reviewed: tests cover disabled export and explicit or drop-based tracer shutdown. |
| `server_observability/src/initialize_otlp_tracer_provider.rs` | Reviewed: disabled export avoids client creation; enabled export builds a named provider. |
| `server_observability/src/lib.rs` | Reviewed: root declares initialization, guard, and observed-error owners. |
| `server_observability/src/observability_guard.rs` | Reviewed: explicit shutdown consumes the provider and Drop supervises an unconsumed provider. |
| `server_observability/src/observability_init_error.rs` | Reviewed: exporter and subscriber failures remain distinct. |
| `server_observability/src/observed_error.rs` | Reviewed: captured error retains source, code, call-site location, backtrace, and span data. |
| `server_observability/src/observed_error_backtrace.rs` | Reviewed: backtrace wrapper forwards display text. |
| `server_observability/src/observed_error_code.rs` | Reviewed: static error-code wrapper preserves typed display. |
| `server_observability/src/opentelemetry_otlp_exporter_build_error.rs` | Reviewed: exporter error wrapper retains its source. |
| `server_observability/src/opentelemetry_sdk_observability_shutdown_error.rs` | Reviewed: shutdown error wrapper retains SDK failure. |
| `server_observability/src/opentelemetry_sdk_tracer_provider.rs` | Reviewed: typed provider forwards owned shutdown. |
| `server_observability/src/otlp_export_mode.rs` | Reviewed: boolean mode conversion selects enabled or disabled export. |
| `server_observability/src/service_name.rs` | Reviewed: static service name wrapper supports display. |
| `server_observability/src/service_tracing_format.rs` | Reviewed: format enum selects JSON or text subscriber. |
| `server_observability/src/std_panic_location.rs` | Reviewed: static source-location wrapper supports display. |
| `server_observability/src/tracing_observed_error_span_trace.rs` | Reviewed: owned span text wrapper supports display. |
| `server_observability/src/tracing_subscriber_init_error.rs` | Reviewed: subscriber error wrapper retains its source. |

### server_runtime_core

| Source | Semantic review |
| --- | --- |
| `server_runtime_core/src/arc_single_flight_rw_lock.rs` | Reviewed: typed wrapper, validation, error mapping, or module wiring inspected; no defect confirmed. |
| `server_runtime_core/src/async_run_history.rs` | Reviewed: nonzero capacity, bounded initial reservation, oldest-report eviction, shared clone ownership and snapshot counts; no lock guard crosses a subsequent await. |
| `server_runtime_core/src/async_run_history_maximum_len_non_zero_usize.rs` | Reviewed: nonzero capacity, bounded initial reservation, oldest-report eviction, shared clone ownership and snapshot counts; no lock guard crosses a subsequent await. |
| `server_runtime_core/src/async_run_history_snapshot.rs` | Reviewed: nonzero capacity, bounded initial reservation, oldest-report eviction, shared clone ownership and snapshot counts; no lock guard crosses a subsequent await. |
| `server_runtime_core/src/background_job.rs` | Reviewed: typed wrapper, validation, error mapping, or module wiring inspected; no defect confirmed. |
| `server_runtime_core/src/bounded_secret_text.rs` | Reviewed: byte bounds, whitespace and repeated-byte rejection, redacted formatting and equality over the bounded range. Timing guarantees were not established by this review. |
| `server_runtime_core/src/bounded_secret_text_error.rs` | Reviewed: byte bounds, whitespace and repeated-byte rejection, redacted formatting and equality over the bounded range. Timing guarantees were not established by this review. |
| `server_runtime_core/src/bulk_item_resource_budget_provider.rs` | Reviewed: typed wrapper, validation, error mapping, or module wiring inspected; no defect confirmed. |
| `server_runtime_core/src/calculate_resource_utilization.rs` | Reviewed: exhaustive identity decisions or bounded resource arithmetic and threshold tests; zero and u64 maximum cases inspected. |
| `server_runtime_core/src/collections_hash_set.rs` | Reviewed: bounded initial allocation, duplicate-before-full precedence, FIFO removal and key release; paired collections are privately owned. Deterministic maximum and reinsertion tests inspected. |
| `server_runtime_core/src/collections_vec_deque.rs` | Reviewed: bounded initial allocation, duplicate-before-full precedence, FIFO removal and key release; paired collections are privately owned. Deterministic maximum and reinsertion tests inspected. |
| `server_runtime_core/src/critical_percent.rs` | Reviewed: exhaustive identity decisions or bounded resource arithmetic and threshold tests; zero and u64 maximum cases inspected. |
| `server_runtime_core/src/deduplicating_queue.rs` | Reviewed: bounded initial allocation, duplicate-before-full precedence, FIFO removal and key release; paired collections are privately owned. Deterministic maximum and reinsertion tests inspected. |
| `server_runtime_core/src/exclusive_run.rs` | Reviewed: atomic transitions, release on guard drop, and generation overflow; deterministic tests inspected. Classification is an observation of the current generation. |
| `server_runtime_core/src/exclusive_run_already_active.rs` | Reviewed: atomic transitions, release on guard drop, and generation overflow; deterministic tests inspected. Classification is an observation of the current generation. |
| `server_runtime_core/src/exclusive_run_atomic_bool.rs` | Reviewed: atomic transitions, release on guard drop, and generation overflow; deterministic tests inspected. Classification is an observation of the current generation. |
| `server_runtime_core/src/exclusive_run_guard.rs` | Reviewed: atomic transitions, release on guard drop, and generation overflow; deterministic tests inspected. Classification is an observation of the current generation. |
| `server_runtime_core/src/execute_plan.rs` | Reviewed: policy storage, nonzero attempts, apply and dry-run dispatch; propagation and no-mutation tests inspected. |
| `server_runtime_core/src/execution_mode.rs` | Reviewed: policy storage, nonzero attempts, apply and dry-run dispatch; propagation and no-mutation tests inspected. |
| `server_runtime_core/src/execution_plan_tests.rs` | Reviewed: policy storage, nonzero attempts, apply and dry-run dispatch; propagation and no-mutation tests inspected. |
| `server_runtime_core/src/execution_report.rs` | Reviewed: policy storage, nonzero attempts, apply and dry-run dispatch; propagation and no-mutation tests inspected. |
| `server_runtime_core/src/generation.rs` | Reviewed: atomic transitions, release on guard drop, and generation overflow; deterministic tests inspected. Classification is an observation of the current generation. |
| `server_runtime_core/src/generation_atomic_u64.rs` | Reviewed: atomic transitions, release on guard drop, and generation overflow; deterministic tests inspected. Classification is an observation of the current generation. |
| `server_runtime_core/src/generation_begin_error.rs` | Reviewed: atomic transitions, release on guard drop, and generation overflow; deterministic tests inspected. Classification is an observation of the current generation. |
| `server_runtime_core/src/generation_commit.rs` | Reviewed: atomic transitions, release on guard drop, and generation overflow; deterministic tests inspected. Classification is an observation of the current generation. |
| `server_runtime_core/src/generation_gate.rs` | Reviewed: atomic transitions, release on guard drop, and generation overflow; deterministic tests inspected. Classification is an observation of the current generation. |
| `server_runtime_core/src/idempotency_response_resource_budget_provider.rs` | Reviewed: typed wrapper, validation, error mapping, or module wiring inspected; no defect confirmed. |
| `server_runtime_core/src/identity_creation_decision.rs` | Reviewed: exhaustive identity decisions or bounded resource arithmetic and threshold tests; zero and u64 maximum cases inspected. |
| `server_runtime_core/src/identity_creation_plan_tests.rs` | Reviewed: exhaustive identity decisions or bounded resource arithmetic and threshold tests; zero and u64 maximum cases inspected. |
| `server_runtime_core/src/identity_presence.rs` | Reviewed: exhaustive identity decisions or bounded resource arithmetic and threshold tests; zero and u64 maximum cases inspected. |
| `server_runtime_core/src/identity_role_presence.rs` | Reviewed: exhaustive identity decisions or bounded resource arithmetic and threshold tests; zero and u64 maximum cases inspected. |
| `server_runtime_core/src/identity_spec.rs` | Reviewed: exhaustive identity decisions or bounded resource arithmetic and threshold tests; zero and u64 maximum cases inspected. |
| `server_runtime_core/src/lease_entry.rs` | Reviewed: paired lease indexes, stale cleanup, rebinding at capacity, heartbeat and release transitions; lock scope and paused-clock tests inspected. Stale listings can repeat already-stale leases. |
| `server_runtime_core/src/lease_heartbeat.rs` | Reviewed: paired lease indexes, stale cleanup, rebinding at capacity, heartbeat and release transitions; lock scope and paused-clock tests inspected. Stale listings can repeat already-stale leases. |
| `server_runtime_core/src/lease_id.rs` | Reviewed: shared text validation precedes bounded storage; hash and equality use validated text. The shared validator is tracked separately. |
| `server_runtime_core/src/lease_ids.rs` | Reviewed: paired lease indexes, stale cleanup, rebinding at capacity, heartbeat and release transitions; lock scope and paused-clock tests inspected. Stale listings can repeat already-stale leases. |
| `server_runtime_core/src/lease_key.rs` | Reviewed: shared text validation precedes bounded storage; hash and equality use validated text. The shared validator is tracked separately. |
| `server_runtime_core/src/lease_registry.rs` | Reviewed: paired lease indexes, stale cleanup, rebinding at capacity, heartbeat and release transitions; lock scope and paused-clock tests inspected. Stale listings can repeat already-stale leases. |
| `server_runtime_core/src/lease_registry_inner.rs` | Reviewed: paired lease indexes, stale cleanup, rebinding at capacity, heartbeat and release transitions; lock scope and paused-clock tests inspected. Stale listings can repeat already-stale leases. |
| `server_runtime_core/src/lease_registry_maximum_non_zero_usize.rs` | Reviewed: paired lease indexes, stale cleanup, rebinding at capacity, heartbeat and release transitions; lock scope and paused-clock tests inspected. Stale listings can repeat already-stale leases. |
| `server_runtime_core/src/lease_reservation.rs` | Reviewed: paired lease indexes, stale cleanup, rebinding at capacity, heartbeat and release transitions; lock scope and paused-clock tests inspected. Stale listings can repeat already-stale leases. |
| `server_runtime_core/src/lease_stale_timeout_duration.rs` | Reviewed: paired lease indexes, stale cleanup, rebinding at capacity, heartbeat and release transitions; lock scope and paused-clock tests inspected. Stale listings can repeat already-stale leases. |
| `server_runtime_core/src/lease_state.rs` | Reviewed: paired lease indexes, stale cleanup, rebinding at capacity, heartbeat and release transitions; lock scope and paused-clock tests inspected. Stale listings can repeat already-stale leases. |
| `server_runtime_core/src/lease_text_error.rs` | Reviewed: typed wrapper, validation, error mapping, or module wiring inspected; no defect confirmed. |
| `server_runtime_core/src/lease_text_maximum_bytes.rs` | Reviewed: typed wrapper, validation, error mapping, or module wiring inspected; no defect confirmed. |
| `server_runtime_core/src/lease_text_ref.rs` | Reviewed: typed wrapper, validation, error mapping, or module wiring inspected; no defect confirmed. |
| `server_runtime_core/src/lib.rs` | Reviewed: typed wrapper, validation, error mapping, or module wiring inspected; no defect confirmed. |
| `server_runtime_core/src/plan_identity_creation.rs` | Reviewed: exhaustive identity decisions or bounded resource arithmetic and threshold tests; zero and u64 maximum cases inspected. |
| `server_runtime_core/src/queue_maximum_non_zero_usize.rs` | Reviewed: bounded initial allocation, duplicate-before-full precedence, FIFO removal and key release; paired collections are privately owned. Deterministic maximum and reinsertion tests inspected. |
| `server_runtime_core/src/queue_push.rs` | Reviewed: bounded initial allocation, duplicate-before-full precedence, FIFO removal and key release; paired collections are privately owned. Deterministic maximum and reinsertion tests inspected. |
| `server_runtime_core/src/reject_non_essential_writes_percent.rs` | Reviewed: exhaustive identity decisions or bounded resource arithmetic and threshold tests; zero and u64 maximum cases inspected. |
| `server_runtime_core/src/resource_amount.rs` | Reviewed: exhaustive identity decisions or bounded resource arithmetic and threshold tests; zero and u64 maximum cases inspected. |
| `server_runtime_core/src/resource_budget.rs` | Reviewed: atomic accounting, retry exits, owner release, waiter wakeup, and cancellation behavior inspected. |
| `server_runtime_core/src/resource_budget_amount.rs` | Reviewed: typed wrapper, validation, error mapping, or module wiring inspected; no defect confirmed. |
| `server_runtime_core/src/resource_budget_config_error.rs` | Reviewed: typed wrapper, validation, error mapping, or module wiring inspected; no defect confirmed. |
| `server_runtime_core/src/resource_budget_maximum.rs` | Reviewed: typed wrapper, validation, error mapping, or module wiring inspected; no defect confirmed. |
| `server_runtime_core/src/resource_budget_reservation.rs` | Reviewed: atomic accounting, retry exits, owner release, waiter wakeup, and cancellation behavior inspected. |
| `server_runtime_core/src/resource_budget_reserve_error.rs` | Reviewed: typed wrapper, validation, error mapping, or module wiring inspected; no defect confirmed. |
| `server_runtime_core/src/resource_utilization.rs` | Reviewed: exhaustive identity decisions or bounded resource arithmetic and threshold tests; zero and u64 maximum cases inspected. |
| `server_runtime_core/src/resource_utilization_error.rs` | Reviewed: exhaustive identity decisions or bounded resource arithmetic and threshold tests; zero and u64 maximum cases inspected. |
| `server_runtime_core/src/resource_utilization_known_percent.rs` | Reviewed: exhaustive identity decisions or bounded resource arithmetic and threshold tests; zero and u64 maximum cases inspected. |
| `server_runtime_core/src/resource_utilization_percent.rs` | Reviewed: exhaustive identity decisions or bounded resource arithmetic and threshold tests; zero and u64 maximum cases inspected. |
| `server_runtime_core/src/resource_utilization_percent_try_from_u8_error.rs` | Reviewed: exhaustive identity decisions or bounded resource arithmetic and threshold tests; zero and u64 maximum cases inspected. |
| `server_runtime_core/src/resource_utilization_status.rs` | Reviewed: exhaustive identity decisions or bounded resource arithmetic and threshold tests; zero and u64 maximum cases inspected. |
| `server_runtime_core/src/retry_attempts_non_zero_usize.rs` | Reviewed: policy storage, nonzero attempts, apply and dry-run dispatch; propagation and no-mutation tests inspected. |
| `server_runtime_core/src/retry_delay_duration.rs` | Reviewed: policy storage, nonzero attempts, apply and dry-run dispatch; propagation and no-mutation tests inspected. |
| `server_runtime_core/src/retry_outcome.rs` | Reviewed: typed wrapper, validation, error mapping, or module wiring inspected; no defect confirmed. |
| `server_runtime_core/src/retry_policy.rs` | Reviewed: policy storage, nonzero attempts, apply and dry-run dispatch; propagation and no-mutation tests inspected. |
| `server_runtime_core/src/retry_tests.rs` | Reviewed: typed wrapper, validation, error mapping, or module wiring inspected; no defect confirmed. |
| `server_runtime_core/src/run_reports_vec_deque.rs` | Reviewed: nonzero capacity, bounded initial reservation, oldest-report eviction, shared clone ownership and snapshot counts; no lock guard crosses a subsequent await. |
| `server_runtime_core/src/run_with_retries.rs` | Reviewed: atomic accounting, retry exits, owner release, waiter wakeup, and cancellation behavior inspected. |
| `server_runtime_core/src/secret_text_match.rs` | Reviewed: byte bounds, whitespace and repeated-byte rejection, redacted formatting and equality over the bounded range. Timing guarantees were not established by this review. |
| `server_runtime_core/src/secret_text_minimum_bytes.rs` | Reviewed: byte bounds, whitespace and repeated-byte rejection, redacted formatting and equality over the bounded range. Timing guarantees were not established by this review. |
| `server_runtime_core/src/secret_text_ref.rs` | Reviewed: byte bounds, whitespace and repeated-byte rejection, redacted formatting and equality over the bounded range. Timing guarantees were not established by this review. |
| `server_runtime_core/src/secret_text_tests.rs` | Reviewed: byte bounds, whitespace and repeated-byte rejection, redacted formatting and equality over the bounded range. Timing guarantees were not established by this review. |
| `server_runtime_core/src/secret_texts_match.rs` | Reviewed: byte bounds, whitespace and repeated-byte rejection, redacted formatting and equality over the bounded range. Timing guarantees were not established by this review. |
| `server_runtime_core/src/select_sources.rs` | Reviewed: typed wrapper, validation, error mapping, or module wiring inspected; no defect confirmed. |
| `server_runtime_core/src/shared_atomic_usize_arc.rs` | Reviewed: typed wrapper, validation, error mapping, or module wiring inspected; no defect confirmed. |
| `server_runtime_core/src/shared_run_reports_arc.rs` | Reviewed: nonzero capacity, bounded initial reservation, oldest-report eviction, shared clone ownership and snapshot counts; no lock guard crosses a subsequent await. |
| `server_runtime_core/src/single_flight.rs` | Reviewed: atomic accounting, retry exits, owner release, waiter wakeup, and cancellation behavior inspected. |
| `server_runtime_core/src/single_flight_acquire.rs` | Reviewed: typed wrapper, validation, error mapping, or module wiring inspected; no defect confirmed. |
| `server_runtime_core/src/single_flight_inner.rs` | Reviewed: typed wrapper, validation, error mapping, or module wiring inspected; no defect confirmed. |
| `server_runtime_core/src/single_flight_key.rs` | Reviewed: typed wrapper, validation, error mapping, or module wiring inspected; no defect confirmed. |
| `server_runtime_core/src/single_flight_key_error.rs` | Reviewed: typed wrapper, validation, error mapping, or module wiring inspected; no defect confirmed. |
| `server_runtime_core/src/single_flight_key_maximum_bytes.rs` | Reviewed: typed wrapper, validation, error mapping, or module wiring inspected; no defect confirmed. |
| `server_runtime_core/src/single_flight_maximum_non_zero_usize.rs` | Reviewed: typed wrapper, validation, error mapping, or module wiring inspected; no defect confirmed. |
| `server_runtime_core/src/single_flight_owner.rs` | Reviewed: atomic accounting, retry exits, owner release, waiter wakeup, and cancellation behavior inspected. |
| `server_runtime_core/src/single_flight_rw_lock_write_guard.rs` | Reviewed: typed wrapper, validation, error mapping, or module wiring inspected; no defect confirmed. |
| `server_runtime_core/src/single_flight_signal.rs` | Reviewed: typed wrapper, validation, error mapping, or module wiring inspected; no defect confirmed. |
| `server_runtime_core/src/single_flight_wait_outcome.rs` | Reviewed: typed wrapper, validation, error mapping, or module wiring inspected; no defect confirmed. |
| `server_runtime_core/src/single_flight_waiter.rs` | Reviewed: atomic accounting, retry exits, owner release, waiter wakeup, and cancellation behavior inspected. |
| `server_runtime_core/src/source_selection.rs` | Reviewed: typed wrapper, validation, error mapping, or module wiring inspected; no defect confirmed. |
| `server_runtime_core/src/source_selection_error.rs` | Reviewed: typed wrapper, validation, error mapping, or module wiring inspected; no defect confirmed. |
| `server_runtime_core/src/std_async_run_history_maximum_len_try_from_usize_error.rs` | Reviewed: nonzero capacity, bounded initial reservation, oldest-report eviction, shared clone ownership and snapshot counts; no lock guard crosses a subsequent await. |
| `server_runtime_core/src/std_async_run_history_report_count.rs` | Reviewed: nonzero capacity, bounded initial reservation, oldest-report eviction, shared clone ownership and snapshot counts; no lock guard crosses a subsequent await. |
| `server_runtime_core/src/std_lease_stale_timeout_error.rs` | Reviewed: typed wrapper, validation, error mapping, or module wiring inspected; no defect confirmed. |
| `server_runtime_core/src/std_retry_attempts_error.rs` | Reviewed: typed wrapper, validation, error mapping, or module wiring inspected; no defect confirmed. |
| `server_runtime_core/src/tokio_lease_instant.rs` | Reviewed: paired lease indexes, stale cleanup, rebinding at capacity, heartbeat and release transitions; lock scope and paused-clock tests inspected. Stale listings can repeat already-stale leases. |
| `server_runtime_core/src/tokio_lease_registry_rw_lock_arc.rs` | Reviewed: paired lease indexes, stale cleanup, rebinding at capacity, heartbeat and release transitions; lock scope and paused-clock tests inspected. Stale listings can repeat already-stale leases. |
| `server_runtime_core/src/tokio_single_flight_receiver.rs` | Reviewed: typed wrapper, validation, error mapping, or module wiring inspected; no defect confirmed. |
| `server_runtime_core/src/tokio_single_flight_sender.rs` | Reviewed: typed wrapper, validation, error mapping, or module wiring inspected; no defect confirmed. |
| `server_runtime_core/src/validate_lease_text.rs` | Reviewed: typed wrapper, validation, error mapping, or module wiring inspected; no defect confirmed. |
| `server_runtime_core/src/warning_percent.rs` | Reviewed: exhaustive identity decisions or bounded resource arithmetic and threshold tests; zero and u64 maximum cases inspected. |
| `server_runtime_core/src/write_inner.rs` | Reviewed: typed wrapper, validation, error mapping, or module wiring inspected; no defect confirmed. |

### server_runtime_http

| Source | Semantic review |
| --- | --- |
| `server_runtime_http/src/abort_and_wait_task.rs` | Pending |
| `server_runtime_http/src/acquire_permit.rs` | Pending |
| `server_runtime_http/src/acquire_permit_error.rs` | Pending |
| `server_runtime_http/src/allow_origin_suffix.rs` | Pending |
| `server_runtime_http/src/allowed_origin.rs` | Pending |
| `server_runtime_http/src/allowed_origin_error.rs` | Pending |
| `server_runtime_http/src/allowed_origins.rs` | Pending |
| `server_runtime_http/src/allowed_origins_error.rs` | Pending |
| `server_runtime_http/src/arc_tokio_semaphore.rs` | Pending |
| `server_runtime_http/src/attach_http_error_diagnostic.rs` | Pending |
| `server_runtime_http/src/attach_http_error_telemetry.rs` | Pending |
| `server_runtime_http/src/axum_notification_router.rs` | Pending |
| `server_runtime_http/src/axum_router.rs` | Pending |
| `server_runtime_http/src/background_task.rs` | Reviewed: retained ownership through cancelled joins, shutdown notification, timeout then abort and join, and drop cancellation. Abort remains cooperative under Tokio task scheduling. |
| `server_runtime_http/src/background_task_outcome.rs` | Reviewed: explicit completed/shutdown/interval-overflow task completion states; A04. |
| `server_runtime_http/src/background_task_shutdown_error.rs` | Reviewed: typed join-source, shutdown-timeout and interval-overflow errors; A04. |
| `server_runtime_http/src/bearer_authorization_resolution.rs` | Pending |
| `server_runtime_http/src/bounded_bytes.rs` | Pending |
| `server_runtime_http/src/bounded_json_read_error.rs` | Pending |
| `server_runtime_http/src/bounded_json_text.rs` | Pending |
| `server_runtime_http/src/bounded_read_concurrency_arc_semaphore.rs` | Pending |
| `server_runtime_http/src/bounded_read_concurrency_maximum_non_zero_usize.rs` | Pending |
| `server_runtime_http/src/bounded_read_error.rs` | Pending |
| `server_runtime_http/src/bounded_read_from_utf8_error.rs` | Pending |
| `server_runtime_http/src/bounded_read_io_error.rs` | Pending |
| `server_runtime_http/src/bounded_read_maximum_bytes.rs` | Pending |
| `server_runtime_http/src/bounded_read_observed_bytes.rs` | Pending |
| `server_runtime_http/src/bounded_text.rs` | Pending |
| `server_runtime_http/src/build_attachment_content_disposition.rs` | Pending |
| `server_runtime_http/src/build_secure_strict_cookie.rs` | Pending |
| `server_runtime_http/src/build_service_runtime.rs` | Pending |
| `server_runtime_http/src/child_diagnostic.rs` | Reviewed: typed diagnostic and process result wrappers, nonzero capture limit, exit-success projection, private report fields and error forwarding. |
| `server_runtime_http/src/child_diagnostic_maximum_non_zero_usize.rs` | Reviewed: typed diagnostic and process result wrappers, nonzero capture limit, exit-success projection, private report fields and error forwarding. |
| `server_runtime_http/src/child_exit_status.rs` | Reviewed: typed diagnostic and process result wrappers, nonzero capture limit, exit-success projection, private report fields and error forwarding. |
| `server_runtime_http/src/child_process_completion.rs` | Reviewed: typed diagnostic and process result wrappers, nonzero capture limit, exit-success projection, private report fields and error forwarding. |
| `server_runtime_http/src/child_process_error.rs` | Reviewed: retained child and diagnostic owners, bounded wait and kill phases, diagnostic deadline A03, cancellation cleanup and typed failures. The stderr capture owner is separately tracked in A19. |
| `server_runtime_http/src/child_process_id.rs` | Reviewed: configured capacity, checked monotonic identifiers, ordered owned supervisors and typed reports. Shutdown stops at the first error; remaining supervisors then use their Drop cleanup. |
| `server_runtime_http/src/child_process_io_error.rs` | Reviewed: typed diagnostic and process result wrappers, nonzero capture limit, exit-success projection, private report fields and error forwarding. |
| `server_runtime_http/src/child_process_report.rs` | Reviewed: typed diagnostic and process result wrappers, nonzero capture limit, exit-success projection, private report fields and error forwarding. |
| `server_runtime_http/src/child_process_reports.rs` | Reviewed: configured capacity, checked monotonic identifiers, ordered owned supervisors and typed reports. Shutdown stops at the first error; remaining supervisors then use their Drop cleanup. |
| `server_runtime_http/src/child_process_set.rs` | Reviewed: configured capacity, checked monotonic identifiers, ordered owned supervisors and typed reports. Shutdown stops at the first error; remaining supervisors then use their Drop cleanup. |
| `server_runtime_http/src/child_process_set_error.rs` | Reviewed: configured capacity, checked monotonic identifiers, ordered owned supervisors and typed reports. Shutdown stops at the first error; remaining supervisors then use their Drop cleanup. |
| `server_runtime_http/src/child_process_set_maximum_non_zero_usize.rs` | Reviewed: configured capacity, checked monotonic identifiers, ordered owned supervisors and typed reports. Shutdown stops at the first error; remaining supervisors then use their Drop cleanup. |
| `server_runtime_http/src/child_process_succeeded.rs` | Pending |
| `server_runtime_http/src/child_process_supervisor.rs` | Reviewed: retained child and diagnostic owners, bounded wait and kill phases, diagnostic deadline A03, cancellation cleanup and typed failures. The stderr capture owner is separately tracked in A19. |
| `server_runtime_http/src/classify_http_error_status.rs` | Pending |
| `server_runtime_http/src/classify_not_found_io_error.rs` | Pending |
| `server_runtime_http/src/classify_optional_json_content_type.rs` | Pending |
| `server_runtime_http/src/cleanup_batch_count.rs` | Pending |
| `server_runtime_http/src/cleanup_batch_size.rs` | Pending |
| `server_runtime_http/src/cleanup_batch_size_error.rs` | Pending |
| `server_runtime_http/src/cleanup_completion.rs` | Pending |
| `server_runtime_http/src/cleanup_continuation.rs` | Pending |
| `server_runtime_http/src/cleanup_report.rs` | Pending |
| `server_runtime_http/src/cleanup_rows.rs` | Pending |
| `server_runtime_http/src/client_addr_parse_error.rs` | Pending |
| `server_runtime_http/src/client_socket_addr.rs` | Pending |
| `server_runtime_http/src/content_disposition_percent_encode_set.rs` | Pending |
| `server_runtime_http/src/cookie_resolution.rs` | Pending |
| `server_runtime_http/src/cors_allow_origin_max_bytes.rs` | Pending |
| `server_runtime_http/src/cors_allow_origin_max_items.rs` | Pending |
| `server_runtime_http/src/cors_allow_origin_split_ch.rs` | Pending |
| `server_runtime_http/src/enforce_pg_rate_limit.rs` | Pending |
| `server_runtime_http/src/ensure_size_within_limit.rs` | Pending |
| `server_runtime_http/src/extract_remote_trace_context.rs` | Pending |
| `server_runtime_http/src/fallback_response_mode.rs` | Pending |
| `server_runtime_http/src/file_staging_action.rs` | Pending |
| `server_runtime_http/src/file_staging_directory_name.rs` | Pending |
| `server_runtime_http/src/forwarded_proto_trust.rs` | Pending |
| `server_runtime_http/src/frontend_build_environment.rs` | Pending |
| `server_runtime_http/src/frontend_build_step.rs` | Pending |
| `server_runtime_http/src/frontend_dependency_fingerprint.rs` | Pending |
| `server_runtime_http/src/frontend_dependency_inputs.rs` | Pending |
| `server_runtime_http/src/frontend_preparation_error.rs` | Pending |
| `server_runtime_http/src/geo_json_document_text.rs` | Pending |
| `server_runtime_http/src/geo_json_validation.rs` | Pending |
| `server_runtime_http/src/geo_json_validation_error.rs` | Pending |
| `server_runtime_http/src/health_component_status.rs` | Pending |
| `server_runtime_http/src/health_probe_succeeded.rs` | Pending |
| `server_runtime_http/src/health_probe_timeout_duration.rs` | Pending |
| `server_runtime_http/src/health_readiness.rs` | Pending |
| `server_runtime_http/src/health_snapshot.rs` | Pending |
| `server_runtime_http/src/http_accept_header_maximum_bytes.rs` | Pending |
| `server_runtime_http/src/http_allowed_path_prefix_ref.rs` | Pending |
| `server_runtime_http/src/http_attachment_file_name_ref.rs` | Pending |
| `server_runtime_http/src/http_authorization_header_text_ref.rs` | Pending |
| `server_runtime_http/src/http_bearer_token_ref.rs` | Pending |
| `server_runtime_http/src/http_content_disposition.rs` | Pending |
| `server_runtime_http/src/http_content_disposition_error.rs` | Pending |
| `server_runtime_http/src/http_content_length.rs` | Pending |
| `server_runtime_http/src/http_content_length_error.rs` | Pending |
| `server_runtime_http/src/http_content_security_policy.rs` | Pending |
| `server_runtime_http/src/http_content_security_policy_error.rs` | Pending |
| `server_runtime_http/src/http_content_type_text_ref.rs` | Pending |
| `server_runtime_http/src/http_cookie_access.rs` | Pending |
| `server_runtime_http/src/http_cookie_headers_ref.rs` | Pending |
| `server_runtime_http/src/http_cookie_name.rs` | Pending |
| `server_runtime_http/src/http_cookie_name_ref.rs` | Pending |
| `server_runtime_http/src/http_cookie_secure.rs` | Pending |
| `server_runtime_http/src/http_cookie_value.rs` | Pending |
| `server_runtime_http/src/http_cookie_value_ref.rs` | Pending |
| `server_runtime_http/src/http_cors_allow_origin_header_values.rs` | Pending |
| `server_runtime_http/src/http_cors_allow_origin_header_values_error.rs` | Pending |
| `server_runtime_http/src/http_cors_allow_origin_text_ref.rs` | Pending |
| `server_runtime_http/src/http_csp_builder.rs` | Pending |
| `server_runtime_http/src/http_csp_directive_name.rs` | Pending |
| `server_runtime_http/src/http_csp_directive_value.rs` | Pending |
| `server_runtime_http/src/http_csp_maximum_bytes_error.rs` | Pending |
| `server_runtime_http/src/http_csp_token_error.rs` | Pending |
| `server_runtime_http/src/http_csp_token_text.rs` | Pending |
| `server_runtime_http/src/http_error_class.rs` | Pending |
| `server_runtime_http/src/http_error_code.rs` | Pending |
| `server_runtime_http/src/http_error_diagnostic.rs` | Pending |
| `server_runtime_http/src/http_error_status.rs` | Pending |
| `server_runtime_http/src/http_error_telemetry.rs` | Pending |
| `server_runtime_http/src/http_error_type.rs` | Pending |
| `server_runtime_http/src/http_error_without_diagnostic_context.rs` | Pending |
| `server_runtime_http/src/http_fallback_api_prefix_ref.rs` | Pending |
| `server_runtime_http/src/http_fallback_metrics_path_ref.rs` | Pending |
| `server_runtime_http/src/http_fallback_request_path_ref.rs` | Pending |
| `server_runtime_http/src/http_header_extractor.rs` | Pending |
| `server_runtime_http/src/http_header_injector.rs` | Pending |
| `server_runtime_http/src/http_header_map_ref.rs` | Pending |
| `server_runtime_http/src/http_header_name.rs` | Pending |
| `server_runtime_http/src/http_header_text_bytes.rs` | Pending |
| `server_runtime_http/src/http_header_text_maximum_bytes.rs` | Pending |
| `server_runtime_http/src/http_header_text_maximum_bytes_error.rs` | Pending |
| `server_runtime_http/src/http_header_text_ref.rs` | Pending |
| `server_runtime_http/src/http_header_text_resolution.rs` | Pending |
| `server_runtime_http/src/http_header_to_str_error.rs` | Pending |
| `server_runtime_http/src/http_host_ref.rs` | Pending |
| `server_runtime_http/src/http_method_ref.rs` | Pending |
| `server_runtime_http/src/http_metrics_layer.rs` | Pending |
| `server_runtime_http/src/http_metrics_path_cache.rs` | Pending |
| `server_runtime_http/src/http_metrics_path_cache_maximum.rs` | Pending |
| `server_runtime_http/src/http_metrics_path_cache_maximum_try_from_usize_error.rs` | Pending |
| `server_runtime_http/src/http_metrics_path_entries_rw_lock.rs` | Pending |
| `server_runtime_http/src/http_metrics_path_text.rs` | Pending |
| `server_runtime_http/src/http_metrics_path_text_error.rs` | Pending |
| `server_runtime_http/src/http_metrics_path_text_ref.rs` | Pending |
| `server_runtime_http/src/http_metrics_service.rs` | Pending |
| `server_runtime_http/src/http_metrics_tower_layer.rs` | Pending |
| `server_runtime_http/src/http_normalized_path.rs` | Pending |
| `server_runtime_http/src/http_normalized_path_error.rs` | Pending |
| `server_runtime_http/src/http_opentelemetry_header_map_mut.rs` | Pending |
| `server_runtime_http/src/http_opentelemetry_header_map_ref.rs` | Pending |
| `server_runtime_http/src/http_optional_accept_header_ref.rs` | Pending |
| `server_runtime_http/src/http_origin_authority_text.rs` | Pending |
| `server_runtime_http/src/http_origin_headers_ref.rs` | Pending |
| `server_runtime_http/src/http_origin_scheme_text.rs` | Pending |
| `server_runtime_http/src/http_origin_text_ref.rs` | Pending |
| `server_runtime_http/src/http_proxy_path.rs` | Pending |
| `server_runtime_http/src/http_proxy_path_error.rs` | Pending |
| `server_runtime_http/src/http_proxy_path_prefix_match.rs` | Pending |
| `server_runtime_http/src/http_proxy_path_ref.rs` | Pending |
| `server_runtime_http/src/http_request_path_ref.rs` | Pending |
| `server_runtime_http/src/http_request_span_config.rs` | Pending |
| `server_runtime_http/src/http_secure_cookie_error.rs` | Pending |
| `server_runtime_http/src/http_set_cookie_header_value.rs` | Pending |
| `server_runtime_http/src/http_trace_parent.rs` | Pending |
| `server_runtime_http/src/http_trace_parent_error.rs` | Pending |
| `server_runtime_http/src/http_trace_state.rs` | Pending |
| `server_runtime_http/src/http_trace_state_error.rs` | Pending |
| `server_runtime_http/src/identifier_file_storage_relative_path.rs` | Pending |
| `server_runtime_http/src/inject_trace_context.rs` | Pending |
| `server_runtime_http/src/io_error_presence_disposition.rs` | Pending |
| `server_runtime_http/src/ipnet_network.rs` | Pending |
| `server_runtime_http/src/join_diagnostic.rs` | Reviewed: retained child and diagnostic owners, bounded wait and kill phases, diagnostic deadline A03, cancellation cleanup and typed failures. The stderr capture owner is separately tracked in A19. |
| `server_runtime_http/src/lib.rs` | Pending |
| `server_runtime_http/src/metrics_response_body.rs` | Pending |
| `server_runtime_http/src/metrics_response_body_error.rs` | Pending |
| `server_runtime_http/src/metrics_shared_string.rs` | Pending |
| `server_runtime_http/src/multipart_bounded_text.rs` | Pending |
| `server_runtime_http/src/multipart_bytes.rs` | Pending |
| `server_runtime_http/src/multipart_bytes_part.rs` | Pending |
| `server_runtime_http/src/multipart_bytes_parts.rs` | Pending |
| `server_runtime_http/src/multipart_field_name.rs` | Pending |
| `server_runtime_http/src/multipart_file_name.rs` | Pending |
| `server_runtime_http/src/multipart_payload_maximum.rs` | Pending |
| `server_runtime_http/src/multipart_request_error.rs` | Pending |
| `server_runtime_http/src/multipart_text_part.rs` | Pending |
| `server_runtime_http/src/multipart_text_parts.rs` | Pending |
| `server_runtime_http/src/multipart_text_value.rs` | Pending |
| `server_runtime_http/src/multipart_upload_request.rs` | Pending |
| `server_runtime_http/src/multipart_value_error.rs` | Pending |
| `server_runtime_http/src/multipart_value_length.rs` | Pending |
| `server_runtime_http/src/normalize_identifier_path.rs` | Pending |
| `server_runtime_http/src/notification_api_token.rs` | Pending |
| `server_runtime_http/src/notification_api_token_authorized.rs` | Pending |
| `server_runtime_http/src/notification_api_token_error.rs` | Pending |
| `server_runtime_http/src/notification_api_token_ref.rs` | Pending |
| `server_runtime_http/src/notification_message_error.rs` | Pending |
| `server_runtime_http/src/notification_request.rs` | Pending |
| `server_runtime_http/src/notification_sender.rs` | Pending |
| `server_runtime_http/src/notification_service_state.rs` | Pending |
| `server_runtime_http/src/opentelemetry_context.rs` | Pending |
| `server_runtime_http/src/optional_json_body_presence.rs` | Pending |
| `server_runtime_http/src/optional_json_content_type.rs` | Pending |
| `server_runtime_http/src/optional_json_content_type_decision.rs` | Pending |
| `server_runtime_http/src/outbound_address_disposition.rs` | Pending |
| `server_runtime_http/src/outbound_allowed_host.rs` | Pending |
| `server_runtime_http/src/outbound_dns_resolver.rs` | Pending |
| `server_runtime_http/src/outbound_host_allowlist.rs` | Pending |
| `server_runtime_http/src/outbound_host_allowlist_error.rs` | Pending |
| `server_runtime_http/src/outbound_host_policy.rs` | Pending |
| `server_runtime_http/src/outbound_ip_addr.rs` | Pending |
| `server_runtime_http/src/outbound_trace_context.rs` | Pending |
| `server_runtime_http/src/outbound_url_error.rs` | Pending |
| `server_runtime_http/src/outbound_url_policy.rs` | Pending |
| `server_runtime_http/src/outbound_url_scheme.rs` | Pending |
| `server_runtime_http/src/outbound_url_text_ref.rs` | Pending |
| `server_runtime_http/src/parse_bounded_json.rs` | Pending |
| `server_runtime_http/src/parse_bounded_json_owned.rs` | Pending |
| `server_runtime_http/src/parse_cors_allow_origin.rs` | Pending |
| `server_runtime_http/src/parse_int_error.rs` | Pending |
| `server_runtime_http/src/parse_trusted_proxy_ranges.rs` | Pending |
| `server_runtime_http/src/parsed_http_origin_ref.rs` | Pending |
| `server_runtime_http/src/parsed_ip_addr.rs` | Pending |
| `server_runtime_http/src/permit_wait_timeout_duration.rs` | Pending |
| `server_runtime_http/src/pg_rate_limit_decision.rs` | Pending |
| `server_runtime_http/src/pg_rate_limit_error.rs` | Pending |
| `server_runtime_http/src/pg_rate_limit_key_part_max_len.rs` | Pending |
| `server_runtime_http/src/pg_rate_limit_maximum.rs` | Pending |
| `server_runtime_http/src/pg_rate_limit_query_ref.rs` | Pending |
| `server_runtime_http/src/pg_rate_limit_scope_ref.rs` | Pending |
| `server_runtime_http/src/pg_rate_limit_subject_ref.rs` | Pending |
| `server_runtime_http/src/pg_rate_limit_validation_error.rs` | Pending |
| `server_runtime_http/src/pg_rate_limit_window_seconds.rs` | Pending |
| `server_runtime_http/src/proxy_path_matches_prefix.rs` | Pending |
| `server_runtime_http/src/read_bounded_file.rs` | Pending |
| `server_runtime_http/src/read_bounded_file_async.rs` | Pending |
| `server_runtime_http/src/read_bounded_http_response.rs` | Focused review: capacity, metadata limits, incremental size checks and semaphore ownership; A02. |
| `server_runtime_http/src/read_bounded_json_file_async.rs` | Pending |
| `server_runtime_http/src/read_bounded_json_http_response.rs` | Pending |
| `server_runtime_http/src/read_child_diagnostic.rs` | Reviewed: bounded initial allocation and retained prefix, full stream draining to EOF, preserved I/O failures; A19 deterministic writer regression passes. Supervisor A03 bounds diagnostic completion. |
| `server_runtime_http/src/redact_rtsp_url_userinfo.rs` | Pending |
| `server_runtime_http/src/redact_url_userinfo.rs` | Pending |
| `server_runtime_http/src/redacted_url.rs` | Pending |
| `server_runtime_http/src/redacted_url_text_ref.rs` | Pending |
| `server_runtime_http/src/request_id.rs` | Pending |
| `server_runtime_http/src/request_id_layer.rs` | Pending |
| `server_runtime_http/src/request_id_service.rs` | Pending |
| `server_runtime_http/src/request_id_tower_layer.rs` | Pending |
| `server_runtime_http/src/request_id_try_from_http_header_value_error.rs` | Pending |
| `server_runtime_http/src/request_id_try_from_string_error.rs` | Pending |
| `server_runtime_http/src/request_origin_allowed.rs` | Pending |
| `server_runtime_http/src/request_origin_value_is_allowed.rs` | Pending |
| `server_runtime_http/src/request_timeout_body.rs` | Pending |
| `server_runtime_http/src/request_timeout_duration.rs` | Pending |
| `server_runtime_http/src/request_timeout_error.rs` | Pending |
| `server_runtime_http/src/request_timeout_layer.rs` | Pending |
| `server_runtime_http/src/request_timeout_service.rs` | Pending |
| `server_runtime_http/src/request_timeout_tower_layer.rs` | Pending |
| `server_runtime_http/src/reqwest_client.rs` | Pending |
| `server_runtime_http/src/reqwest_client_build_error.rs` | Pending |
| `server_runtime_http/src/reqwest_client_policy.rs` | Pending |
| `server_runtime_http/src/reqwest_connect_timeout_duration.rs` | Pending |
| `server_runtime_http/src/reqwest_error.rs` | Pending |
| `server_runtime_http/src/reqwest_outbound_url.rs` | Pending |
| `server_runtime_http/src/reqwest_request.rs` | Pending |
| `server_runtime_http/src/reqwest_request_builder.rs` | Pending |
| `server_runtime_http/src/reqwest_request_timeout_duration.rs` | Pending |
| `server_runtime_http/src/reqwest_response.rs` | Pending |
| `server_runtime_http/src/resolve_bearer_authorization.rs` | Pending |
| `server_runtime_http/src/resolve_client_ip.rs` | Pending |
| `server_runtime_http/src/resolve_fallback_response_mode.rs` | Pending |
| `server_runtime_http/src/resolve_header_text.rs` | Pending |
| `server_runtime_http/src/resolve_optional_json_content_type_decision.rs` | Pending |
| `server_runtime_http/src/resolve_outbound_address_disposition.rs` | Pending |
| `server_runtime_http/src/resolve_request_origin_allowed.rs` | Pending |
| `server_runtime_http/src/resolve_unique_cookie.rs` | Pending |
| `server_runtime_http/src/resolved_client_ip_addr.rs` | Pending |
| `server_runtime_http/src/retry_after_secs.rs` | Pending |
| `server_runtime_http/src/retry_after_secs_try_from_u64_error.rs` | Pending |
| `server_runtime_http/src/run_batched_cleanup.rs` | Pending |
| `server_runtime_http/src/run_health_probe.rs` | Pending |
| `server_runtime_http/src/run_interval_duration.rs` | Reviewed: private Duration wrapper with validated nonzero conversion; scheduling representability is checked by its task owner under A04. |
| `server_runtime_http/src/runtime_notification_message.rs` | Pending |
| `server_runtime_http/src/runtime_path_ref.rs` | Pending |
| `server_runtime_http/src/runtime_storage_relative_path_buf.rs` | Pending |
| `server_runtime_http/src/security_headers_layer.rs` | Pending |
| `server_runtime_http/src/security_headers_service.rs` | Pending |
| `server_runtime_http/src/security_headers_tower_layer.rs` | Pending |
| `server_runtime_http/src/semaphore_permit_count_non_zero_usize.rs` | Pending |
| `server_runtime_http/src/serde_json_error.rs` | Pending |
| `server_runtime_http/src/serde_json_geo_json_error.rs` | Pending |
| `server_runtime_http/src/serve_io_error.rs` | Pending |
| `server_runtime_http/src/serve_with_graceful_shutdown.rs` | Pending |
| `server_runtime_http/src/serve_with_graceful_shutdown_error.rs` | Pending |
| `server_runtime_http/src/service_liveness_snapshot.rs` | Pending |
| `server_runtime_http/src/service_runtime.rs` | Pending |
| `server_runtime_http/src/service_runtime_io_error.rs` | Pending |
| `server_runtime_http/src/shared_health_readiness_arc.rs` | Pending |
| `server_runtime_http/src/shared_http_metrics_path_cache_arc.rs` | Pending |
| `server_runtime_http/src/spawn_interval_task.rs` | Reviewed: owned task/shutdown channel, checked cadence-preserving scheduling, binary duration remainder, cancellation-safe Sleep/oneshot select and explicit interval overflow under A04. |
| `server_runtime_http/src/sqlx_pg_rate_limit_error.rs` | Pending |
| `server_runtime_http/src/sqlx_pg_rate_limit_pool_ref.rs` | Pending |
| `server_runtime_http/src/staging_directory_name.rs` | Pending |
| `server_runtime_http/src/std_collections_child_process_map.rs` | Reviewed: configured capacity, checked monotonic identifiers, ordered owned supervisors and typed reports. Shutdown stops at the first error; remaining supervisors then use their Drop cleanup. |
| `server_runtime_http/src/std_cookie_max_age_seconds.rs` | Pending |
| `server_runtime_http/src/std_frontend_os_string.rs` | Pending |
| `server_runtime_http/src/std_frontend_path_buf.rs` | Pending |
| `server_runtime_http/src/std_http_error_backtrace.rs` | Pending |
| `server_runtime_http/src/std_http_error_chain.rs` | Pending |
| `server_runtime_http/src/std_range_contains.rs` | Pending |
| `server_runtime_http/src/std_request_timeout_message.rs` | Pending |
| `server_runtime_http/src/std_request_timeout_try_from_duration_error.rs` | Pending |
| `server_runtime_http/src/std_reqwest_timeout_duration_ref.rs` | Pending |
| `server_runtime_http/src/std_reqwest_timeout_error.rs` | Pending |
| `server_runtime_http/src/std_run_interval_try_from_duration_error.rs` | Reviewed: typed zero-period validation with centralized diagnostic text. |
| `server_runtime_http/src/storage_path_segment.rs` | Pending |
| `server_runtime_http/src/storage_path_segment_error.rs` | Pending |
| `server_runtime_http/src/supported_geo_json_type_validation.rs` | Pending |
| `server_runtime_http/src/test_batched_cleanup_tests.rs` | Pending |
| `server_runtime_http/src/test_bounded_read_tests.rs` | Focused review: deterministic HTTP response growth and rejection regressions; other read fixtures remain pending. |
| `server_runtime_http/src/test_child_process_tests.rs` | Focused review: shutdown ownership, diagnostic bounds and deadline; A03 and cancellation regressions. |
| `server_runtime_http/src/test_client_ip_tests.rs` | Pending |
| `server_runtime_http/src/test_cors_tests.rs` | Pending |
| `server_runtime_http/src/test_csp_tests.rs` | Pending |
| `server_runtime_http/src/test_domain_types_security_headers_tests.rs` | Pending |
| `server_runtime_http/src/test_domain_types_service_runtime_tests.rs` | Pending |
| `server_runtime_http/src/test_fallback_tests.rs` | Pending |
| `server_runtime_http/src/test_header_text_tests.rs` | Pending |
| `server_runtime_http/src/test_health_tests.rs` | Pending |
| `server_runtime_http/src/test_http_client_tests.rs` | Pending |
| `server_runtime_http/src/test_http_header_policy_tests.rs` | Pending |
| `server_runtime_http/src/test_http_policy_tests.rs` | Pending |
| `server_runtime_http/src/test_http_status_error_tests.rs` | Pending |
| `server_runtime_http/src/test_lifecycle_tests.rs` | Pending |
| `server_runtime_http/src/test_metrics_layer_tests.rs` | Pending |
| `server_runtime_http/src/test_multipart_tests.rs` | Pending |
| `server_runtime_http/src/test_notification_tests.rs` | Pending |
| `server_runtime_http/src/test_origin_tests.rs` | Pending |
| `server_runtime_http/src/test_outbound_url_tests.rs` | Pending |
| `server_runtime_http/src/test_path_policy_tests.rs` | Pending |
| `server_runtime_http/src/test_pg_rate_limit_tests.rs` | Pending |
| `server_runtime_http/src/test_request_timeout_tests.rs` | Pending |
| `server_runtime_http/src/test_secure_cookie_tests.rs` | Pending |
| `server_runtime_http/src/test_server_runtime_http.rs` | Pending |
| `server_runtime_http/src/test_service_tests.rs` | Pending |
| `server_runtime_http/src/test_tests_domain_types_request_id.rs` | Pending |
| `server_runtime_http/src/test_tests_domain_types_resource_budget.rs` | Pending |
| `server_runtime_http/src/test_tests_domain_types_security_headers.rs` | Pending |
| `server_runtime_http/src/test_tests_domain_types_service_runtime.rs` | Reviewed: history/runtime ownership, join/panic/cancellation/timeout cases, permit boundaries and three virtual-time interval regressions with a shared cadence fixture; A04. |
| `server_runtime_http/src/test_trace_context_tests.rs` | Pending |
| `server_runtime_http/src/test_wire_token_tests.rs` | Pending |
| `server_runtime_http/src/tokio_abort_task.rs` | Pending |
| `server_runtime_http/src/tokio_acquire_error.rs` | Pending |
| `server_runtime_http/src/tokio_background_task_join.rs` | Reviewed: retained JoinHandle borrowing, preserved Tokio join source, explicit interval failure classification and cooperative abort; A04. |
| `server_runtime_http/src/tokio_background_task_shutdown_sender.rs` | Reviewed: retained ownership through cancelled joins, shutdown notification, timeout then abort and join, and drop cancellation. Abort remains cooperative under Tokio task scheduling. |
| `server_runtime_http/src/tokio_child_diagnostic_elapsed.rs` | Reviewed: retained child and diagnostic owners, bounded wait and kill phases, diagnostic deadline A03, cancellation cleanup and typed failures. The stderr capture owner is separately tracked in A19. |
| `server_runtime_http/src/tokio_child_diagnostic_task.rs` | Reviewed: retained child and diagnostic owners, bounded wait and kill phases, diagnostic deadline A03, cancellation cleanup and typed failures. The stderr capture owner is separately tracked in A19. |
| `server_runtime_http/src/tokio_child_process.rs` | Pending |
| `server_runtime_http/src/tokio_child_process_join_error.rs` | Reviewed: typed diagnostic and process result wrappers, nonzero capture limit, exit-success projection, private report fields and error forwarding. |
| `server_runtime_http/src/tokio_frontend_build_command.rs` | Pending |
| `server_runtime_http/src/tokio_managed_child.rs` | Reviewed: retained child and diagnostic owners, bounded wait and kill phases, diagnostic deadline A03, cancellation cleanup and typed failures. The stderr capture owner is separately tracked in A19. |
| `server_runtime_http/src/tokio_owned_semaphore_permit.rs` | Pending |
| `server_runtime_http/src/tokio_service_runtime.rs` | Pending |
| `server_runtime_http/src/tokio_task_join_error.rs` | Pending |
| `server_runtime_http/src/tokio_tcp_listener.rs` | Pending |
| `server_runtime_http/src/tracing_http_client_span.rs` | Pending |
| `server_runtime_http/src/tracing_http_span_trace.rs` | Pending |
| `server_runtime_http/src/trusted_proxy_range.rs` | Pending |
| `server_runtime_http/src/trusted_proxy_range_parse_error.rs` | Pending |
| `server_runtime_http/src/trusted_proxy_ranges.rs` | Pending |
| `server_runtime_http/src/trusted_proxy_ranges_error.rs` | Pending |
| `server_runtime_http/src/trusted_proxy_ranges_parse_error.rs` | Pending |
| `server_runtime_http/src/trusted_proxy_ranges_text_ref.rs` | Pending |
| `server_runtime_http/src/validate_frontend_node_version.rs` | Pending |
| `server_runtime_http/src/validate_outbound_resolved_addresses.rs` | Pending |
| `server_runtime_http/src/versioned_url_safe_wire_token_text.rs` | Pending |
| `server_runtime_http/src/versioned_url_safe_wire_token_text_error.rs` | Pending |
| `server_runtime_http/src/wait_for_service_shutdown_signal.rs` | Pending |

### synchronization_service_runtime

| Source | Semantic review |
| --- | --- |
| `synchronization_service_runtime/src/lib.rs` | Reviewed: payload bounds, typed policies and source trait; maximum and maximum-plus-one tests inspected. |
| `synchronization_service_runtime/src/synchronization_payload.rs` | Reviewed: payload bounds, typed policies and source trait; maximum and maximum-plus-one tests inspected. |
| `synchronization_service_runtime/src/synchronization_payload_max_bytes.rs` | Reviewed: payload bounds, typed policies and source trait; maximum and maximum-plus-one tests inspected. |
| `synchronization_service_runtime/src/synchronization_payload_too_large.rs` | Reviewed: payload bounds, typed policies and source trait; maximum and maximum-plus-one tests inspected. |
| `synchronization_service_runtime/src/synchronization_runtime_configuration.rs` | Reviewed: payload bounds, typed policies and source trait; maximum and maximum-plus-one tests inspected. |
| `synchronization_service_runtime/src/synchronization_source.rs` | Reviewed: payload bounds, typed policies and source trait; maximum and maximum-plus-one tests inspected. |
| `synchronization_service_runtime/src/test_synchronization_service_runtime.rs` | Reviewed: payload bounds, typed policies and source trait; maximum and maximum-plus-one tests inspected. |

### tests_code_style_rust

| Source | Semantic review |
| --- | --- |
| `tests_code_style_rust/src/analyzer_bool.rs` | Pending |
| `tests_code_style_rust/src/analyzer_count.rs` | Pending |
| `tests_code_style_rust/src/cargo_metadata.rs` | Pending |
| `tests_code_style_rust/src/cargo_metadata_ref.rs` | Pending |
| `tests_code_style_rust/src/cargo_package_id_ref_hash_set.rs` | Pending |
| `tests_code_style_rust/src/cargo_toml_file_index.rs` | Pending |
| `tests_code_style_rust/src/code_style.rs` | Pending |
| `tests_code_style_rust/src/diagnostic_messages.rs` | Pending |
| `tests_code_style_rust/src/diagnostic_messages_mut_ref.rs` | Pending |
| `tests_code_style_rust/src/domain_analysis.rs` | Pending |
| `tests_code_style_rust/src/domain_type_policy_fixture.rs` | Pending |
| `tests_code_style_rust/src/function_body_hash.rs` | Pending |
| `tests_code_style_rust/src/function_body_locations_b_tree_map.rs` | Pending |
| `tests_code_style_rust/src/function_body_locations_b_tree_map_mut_ref.rs` | Pending |
| `tests_code_style_rust/src/lib.rs` | Pending |
| `tests_code_style_rust/src/owned_path_buf.rs` | Pending |
| `tests_code_style_rust/src/path_ref.rs` | Pending |
| `tests_code_style_rust/src/regex_regex_ref.rs` | Pending |
| `tests_code_style_rust/src/rs_source_files_ref.rs` | Pending |
| `tests_code_style_rust/src/runtime_analysis.rs` | Pending |
| `tests_code_style_rust/src/source_analysis.rs` | Pending |
| `tests_code_style_rust/src/source_text.rs` | Pending |
| `tests_code_style_rust/src/source_text_b_tree_set.rs` | Pending |
| `tests_code_style_rust/src/source_text_b_tree_set_ref.rs` | Pending |
| `tests_code_style_rust/src/source_text_hash_set.rs` | Pending |
| `tests_code_style_rust/src/source_text_list.rs` | Pending |
| `tests_code_style_rust/src/source_text_list_ref.rs` | Pending |
| `tests_code_style_rust/src/source_text_ref.rs` | Pending |
| `tests_code_style_rust/src/source_text_ref_hash_set.rs` | Pending |
| `tests_code_style_rust/src/source_text_try_from_string_error.rs` | Pending |
| `tests_code_style_rust/src/static_str.rs` | Pending |
| `tests_code_style_rust/src/static_str_slice_ref.rs` | Pending |
| `tests_code_style_rust/src/syn_attribute_list_ref.rs` | Pending |
| `tests_code_style_rust/src/syn_attribute_ref.rs` | Pending |
| `tests_code_style_rust/src/syn_block_ref.rs` | Pending |
| `tests_code_style_rust/src/syn_expr_call_ref.rs` | Pending |
| `tests_code_style_rust/src/syn_fields_ref.rs` | Pending |
| `tests_code_style_rust/src/syn_file.rs` | Pending |
| `tests_code_style_rust/src/syn_file_ref.rs` | Pending |
| `tests_code_style_rust/src/syn_generics_ref.rs` | Pending |
| `tests_code_style_rust/src/syn_identifier_ref.rs` | Pending |
| `tests_code_style_rust/src/syn_item_fn_ref.rs` | Pending |
| `tests_code_style_rust/src/syn_item_impl_ref.rs` | Pending |
| `tests_code_style_rust/src/syn_item_ref.rs` | Pending |
| `tests_code_style_rust/src/syn_item_struct_ref.rs` | Pending |
| `tests_code_style_rust/src/syn_path_arguments_ref.rs` | Pending |
| `tests_code_style_rust/src/syn_path_ref.rs` | Pending |
| `tests_code_style_rust/src/syn_path_segment_ref.rs` | Pending |
| `tests_code_style_rust/src/syn_signature_ref.rs` | Pending |
| `tests_code_style_rust/src/syn_type_path_ref.rs` | Pending |
| `tests_code_style_rust/src/syn_type_ref.rs` | Pending |
| `tests_code_style_rust/src/syn_use_tree_ref.rs` | Pending |
| `tests_code_style_rust/src/test_code_style_advanced_policy.rs` | Pending |
| `tests_code_style_rust/src/test_code_style_cargo_policy.rs` | Pending |
| `tests_code_style_rust/src/test_code_style_ci_policy.rs` | Pending |
| `tests_code_style_rust/src/test_code_style_contract_source_policy.rs` | Pending |
| `tests_code_style_rust/src/test_code_style_deployment_policy.rs` | Pending |
| `tests_code_style_rust/src/test_code_style_domain_type_policy.rs` | Pending |
| `tests_code_style_rust/src/test_code_style_lint_sync.rs` | Pending |
| `tests_code_style_rust/src/test_code_style_module_policy.rs` | Pending |
| `tests_code_style_rust/src/test_code_style_reuse_policy.rs` | Pending |
| `tests_code_style_rust/src/test_code_style_route_contract_policy.rs` | Pending |
| `tests_code_style_rust/src/test_code_style_runtime_policy.rs` | Pending |
| `tests_code_style_rust/src/test_code_style_secret_policy.rs` | Pending |
| `tests_code_style_rust/src/test_code_style_snapshot.rs` | Pending |
| `tests_code_style_rust/src/test_code_style_source_policy.rs` | Pending |
| `tests_code_style_rust/src/toml_table.rs` | Pending |
| `tests_code_style_rust/src/toml_table_ref.rs` | Pending |
| `tests_code_style_rust/src/toml_value_ref.rs` | Pending |
| `tests_code_style_rust/src/walkdir_walk_dir.rs` | Pending |
| `tests_code_style_rust/trybuild/route_contract_catalog_missing_route.rs` | Pending |
| `tests_code_style_rust/trybuild/route_contract_delegate_non_empty.rs` | Pending |
| `tests_code_style_rust/trybuild/route_contract_page_catalog_non_unit.rs` | Pending |
| `tests_code_style_rust/trybuild/route_contract_struct_api_non_named.rs` | Pending |
| `tests_code_style_rust/trybuild/route_contract_wire_enum_duplicate.rs` | Pending |
| `tests_code_style_rust/trybuild/route_contract_wire_enum_non_unit.rs` | Pending |
| `tests_code_style_rust/trybuild/route_contract_wrong_family_empty.rs` | Pending |
| `tests_code_style_rust/trybuild/route_contract_wrong_family_missing_attribute.rs` | Pending |
| `tests_code_style_rust/trybuild/route_contract_wrong_path_parameter.rs` | Pending |
| `tests_code_style_rust/trybuild/route_contract_wrong_request.rs` | Pending |
| `tests_code_style_rust/trybuild/route_contract_wrong_response.rs` | Pending |
| `tests_code_style_rust/trybuild/route_contract_wrong_route.rs` | Pending |
| `tests_code_style_rust/trybuild/route_contract_wrong_transport.rs` | Pending |

### text_policy

| Source | Semantic review |
| --- | --- |
| `text_policy/src/bounded_text_policy_error.rs` | Reviewed: empty, NUL and byte-limit failures have distinct variants; no source is discarded by this enum. |
| `text_policy/src/fixed_length_ascii_hex_text.rs` | Reviewed: construction requires exactly 40 bytes and lowercase ASCII hexadecimal symbols before bounded storage. |
| `text_policy/src/fixed_length_ascii_hex_text_error.rs` | Reviewed: separate length and symbol errors match the fixed-hex constructor. |
| `text_policy/src/https_url_text_error.rs` | Focused review; A115 shared HTTPS URL validation. |
| `text_policy/src/https_url_text_ref.rs` | Focused review; A115 shared HTTPS URL validation. |
| `text_policy/src/lib.rs` | Reviewed: all production modules are declared from the crate root; tests are compiled only for test builds. |
| `text_policy/src/non_empty_trimmed_text.rs` | Reviewed: input byte length is bounded before trimming; the trimmed value must be nonempty and NUL-free. No repository consumer currently uses this public wrapper. |
| `text_policy/src/password_length.rs` | Reviewed: typed length forwards its private usize value through generated conversion derives. |
| `text_policy/src/password_length_range.rs` | Reviewed: TryFrom rejects inverted bounds; the explicitly named prevalidated constructor is used with ordered administrator constants. |
| `text_policy/src/password_length_range_error.rs` | Reviewed: the single invalid-range variant describes inverted bounds. |
| `text_policy/src/password_policy_violation.rs` | Reviewed: typed password policy outcomes include a general whitespace violation; A52 aligns detection with that meaning. |
| `text_policy/src/password_text_ref.rs` | Reviewed: borrowed password text has redacted Debug and private storage. |
| `text_policy/src/required_nul_free_bounded_text.rs` | Reviewed: rejects oversized, empty and NUL-bearing input before validated bounded storage. |
| `text_policy/src/tests_domain_types.rs` | Reviewed: deterministic fixtures cover fixed hex, URL-safe tokens, required NUL-free text and password policy including A52, A53, and A115. |
| `text_policy/src/url_safe_token_part_maximum_bytes.rs` | Reviewed: private catalog maximum is 4096 bytes; the public wrapper carries caller-selected limits for the reusable validator. |
| `text_policy/src/url_safe_token_part_ref.rs` | Reviewed: borrowed token text uses private storage and generated conversion. |
| `text_policy/src/url_safe_token_part_text.rs` | Reviewed: construction applies the 4096-byte limit and URL-safe validator before bounded storage. |
| `text_policy/src/url_safe_token_part_text_error.rs` | Reviewed: empty, invalid symbol and excessive length are distinguished. |
| `text_policy/src/validate_https_url_text.rs` | Focused review; A115 shared HTTPS URL validation. |
| `text_policy/src/validate_password_policy.rs` | Reviewed: A53 counts characters for password length and A52 rejects Unicode whitespace before checking required digit, letter and punctuation classes. |
| `text_policy/src/validate_url_safe_token_part.rs` | Reviewed: nonempty input must fit the caller's byte limit and contain only ASCII alphanumeric, hyphen or underscore bytes. |

### to_err_string

| Source | Semantic review |
| --- | --- |
| `to_err_string/src/as_ref_str_to_owned.rs` | Reviewed: borrowed text is copied once into bounded diagnostic storage; oversize text becomes a length diagnostic. |
| `to_err_string/src/debug_to_string.rs` | Reviewed: Debug output is bounded before entering diagnostic storage. |
| `to_err_string/src/domain_types.rs` | Focused review: generated conversions and four deterministic tests pass; proc-macro implementation remains separately inventoried. |
| `to_err_string/src/error_text.rs` | Reviewed: bounded diagnostic wrapper, validated conversion, display, deref and serde derive configuration. |
| `to_err_string/src/error_text_max_len.rs` | Reviewed: shared 1 MiB diagnostic storage limit. |
| `to_err_string/src/lib.rs` | Reviewed: flat owner-module declarations. |
| `to_err_string/src/static_str_to_owned.rs` | Reviewed: static text is copied once and bounded, preserving the diagnostic fallback policy. |
| `to_err_string/src/static_str_to_owned_input.rs` | Reviewed: typed static-string input wrapper and generated getter. |
| `to_err_string/src/to_err_string.rs` | Reviewed: borrowed, Option and Result forwarding preserves diagnostic representation; source package tests pass. |

### token_patterns

| Source | Semantic review |
| --- | --- |
| `token_patterns/src/lib.rs` | Reviewed: fixed token catalogs and all five deterministic generated-token tests; no dynamic parser or state. |
| `token_patterns/src/proc_macro2_tokens_mut.rs` | Reviewed: borrowed mutable stream appends through ToTokens without hidden ownership or allocation policy. |

### workspace_macro_helpers

| Source | Semantic review |
| --- | --- |
| `workspace_macro_helpers/src/closure_identifier_and_body.rs` | Reviewed: identifier-only closure parameter parsing, complete body token capture and bounded conversion; oversized parameters now return None under A11. |
| `workspace_macro_helpers/src/collection_max_len.rs` | Reviewed: explicit 10,000-part construction limit. |
| `workspace_macro_helpers/src/compile_error_token_stream.rs` | Reviewed: quote inserts the owned message as a literal into compile_error tokens. |
| `workspace_macro_helpers/src/first_comma_stripped.rs` | Reviewed: private Copy boolean adapter and negation forwarding. |
| `workspace_macro_helpers/src/first_ident_max_len.rs` | Reviewed: explicit 1 MiB byte limit for identifier text. |
| `workspace_macro_helpers/src/first_identifier.rs` | Reviewed: bounded byte-length conversion and diagnostic-text conversion; the parse fallback remains candidate A11. |
| `workspace_macro_helpers/src/first_identifier_at.rs` | Reviewed: checked part lookup and delegation to identifier parsing; A11 remains in that owner. |
| `workspace_macro_helpers/src/first_identifierifier_try_from_string_error.rs` | Reviewed: actual length is retained and formatted against the identifier maximum; bounded diagnostic conversion fits this error text. |
| `workspace_macro_helpers/src/generate_bool_enum_to_tokens.rs` | Reviewed: syn expression parsing respects nested delimiters, validates branch booleans and emits unchanged enum/ToTokens interfaces. |
| `workspace_macro_helpers/src/generate_private_field_getters.rs` | Focused review: token forwarding and getter name construction; see A08 and A09. |
| `workspace_macro_helpers/src/generate_trait_alias.rs` | Reviewed: validated alias identifier and equals token, preserved bounds, raw/forwarded identifiers and compile diagnostics. |
| `workspace_macro_helpers/src/lib.rs` | Reviewed: flat shared helper ownership, typed token adapters and test module declarations. |
| `workspace_macro_helpers/src/parse_first_identifier.rs` | Reviewed: identifier tokens, delimiter-free forwarding and non-identifier rejection; A11 rejects oversized text. |
| `workspace_macro_helpers/src/part_at.rs` | Reviewed: checked indexed lookup and owned token-stream copy; an out-of-range index returns None. |
| `workspace_macro_helpers/src/part_index.rs` | Reviewed: private Copy index wrapper with generated construction and crate-scoped read access. |
| `workspace_macro_helpers/src/proc_macro2_macro_tokens.rs` | Reviewed: owned token-tree adapter, display and quote forwarding, and complete parse cursor advancement. |
| `workspace_macro_helpers/src/proc_macro2_top_level_comma_parts.rs` | Reviewed: construction and parsing enforce the collection maximum; A10 removes mutable Vec access to preserve the validated collection limit. |
| `workspace_macro_helpers/src/split_fat_arrow.rs` | Reviewed: top-level arrow tokenizer validates punctuation and preserves before and after streams. |
| `workspace_macro_helpers/src/split_top_level_commas.rs` | Focused review: token parsing, generic comma handling and bounded conversions; A10 and A11 require follow-up. |
| `workspace_macro_helpers/src/std_unique_option_set_contains.rs` | Reviewed: private Copy membership-result wrapper with generated conversion/read access. |
| `workspace_macro_helpers/src/std_unique_option_set_is_empty.rs` | Reviewed: private Copy emptiness-result wrapper with generated conversion/read access. |
| `workspace_macro_helpers/src/strip_first_comma.rs` | Reviewed: consumes one token and reports whether it is a comma. |
| `workspace_macro_helpers/src/syn_derive_input_ref.rs` | Reviewed: borrowed derive-input wrapper with generated forwarding. |
| `workspace_macro_helpers/src/syn_fields_named_ref.rs` | Reviewed: borrowed named-field syntax view without ownership or mutation. |
| `workspace_macro_helpers/src/syn_fields_unnamed_ref.rs` | Reviewed: borrowed tuple-field syntax view without ownership or mutation. |
| `workspace_macro_helpers/src/syn_struct_shape_ref.rs` | Reviewed: struct-only conversion distinguishes named, tuple and unit forms and rejects enum/union inputs. |
| `workspace_macro_helpers/src/tests_domain_types.rs` | Reviewed: deterministic struct-shape, generic comma/fat-arrow, token forwarding, first-error, getter and macro parser fixtures. |
| `workspace_macro_helpers/src/top_level_comma_part.rs` | Reviewed: speculative type/expression parsing commits complete comma-delimited parts, then falls back to grouped-token scanning. |
| `workspace_macro_helpers/src/unique_option_b_tree_set.rs` | Reviewed: ordered uniqueness, duplicate-only lazy diagnostics and preserved existing membership. |

### workspace_scaffold

| Source | Semantic review |
| --- | --- |
| `workspace_scaffold/src/cargo_args_ref.rs` | Reviewed |
| `workspace_scaffold/src/generated_projection.rs` | Reviewed |
| `workspace_scaffold/src/main.rs` | Focused review; A75 |
| `workspace_scaffold/src/naming_capitalized_parts.rs` | Reviewed; A76 |
| `workspace_scaffold/src/naming_kebab_case.rs` | Reviewed; A76 |
| `workspace_scaffold/src/naming_title_case.rs` | Reviewed; A76 |
| `workspace_scaffold/src/naming_upper_camel_case.rs` | Reviewed; A76 |
| `workspace_scaffold/src/naming_validate_project_name.rs` | Reviewed; A74, A76 |
| `workspace_scaffold/src/naming_validate_repository_url.rs` | Reviewed; A75 |
| `workspace_scaffold/src/project_name_ref.rs` | Reviewed |
| `workspace_scaffold/src/replacements_ref.rs` | Reviewed |
| `workspace_scaffold/src/repository_url_ref.rs` | Reviewed |
| `workspace_scaffold/src/scaffold_error.rs` | Reviewed |
| `workspace_scaffold/src/scaffold_io_error.rs` | Reviewed |
| `workspace_scaffold/src/scaffold_path_ref.rs` | Reviewed |
| `workspace_scaffold/src/scaffold_run_ok.rs` | Reviewed |
| `workspace_scaffold/src/scaffold_text.rs` | Reviewed |
| `workspace_scaffold/src/scaffold_text_ref.rs` | Reviewed |
| `workspace_scaffold/src/service_catalog_draft.rs` | Reviewed |
| `workspace_scaffold/src/service_catalog_entries.rs` | Reviewed |
| `workspace_scaffold/src/service_catalog_entries_ref.rs` | Reviewed |
| `workspace_scaffold/src/service_catalog_entry.rs` | Reviewed |
| `workspace_scaffold/src/service_catalog_parse.rs` | Focused review; A68, A70, A71, A73, A78 |
| `workspace_scaffold/src/service_catalog_render_release_entries.rs` | Reviewed |
| `workspace_scaffold/src/service_catalog_string_value.rs` | Reviewed; A69, A73 |
| `workspace_scaffold/src/service_compose_file.rs` | Reviewed |
| `workspace_scaffold/src/service_compose_name.rs` | Reviewed |
| `workspace_scaffold/src/service_crate.rs` | Reviewed |
| `workspace_scaffold/src/service_dockerfile.rs` | Reviewed |
| `workspace_scaffold/src/service_image.rs` | Reviewed |
| `workspace_scaffold/src/service_kubernetes_manifest.rs` | Reviewed |
| `workspace_scaffold/src/service_port.rs` | Reviewed |
| `workspace_scaffold/src/service_socket_env.rs` | Reviewed |
| `workspace_scaffold/src/should_release.rs` | Reviewed |
| `workspace_scaffold/src/should_skip.rs` | Reviewed |
| `workspace_scaffold/src/should_write.rs` | Reviewed |
| `workspace_scaffold/src/synchronize_cargo_owned_projection.rs` | Reviewed |
| `workspace_scaffold/src/synchronize_deployment_projections.rs` | Focused review; A65 |
| `workspace_scaffold/src/synchronize_generated_file.rs` | Reviewed; A67 |
| `workspace_scaffold/src/template_fs_copy_template_tree.rs` | Reviewed; A66 |
| `workspace_scaffold/src/template_fs_insert_once.rs` | Reviewed; A77 |
| `workspace_scaffold/src/template_fs_read_bounded_text.rs` | Reviewed |
| `workspace_scaffold/src/template_fs_replace_file.rs` | Reviewed; A64, A72 |
| `workspace_scaffold/src/template_fs_should_skip.rs` | Reviewed |
| `workspace_scaffold/src/template_fs_write_text.rs` | Reviewed |
| `workspace_scaffold/src/test_workspace_scaffold.rs` | Focused review; A76 call sites |
| `workspace_scaffold/src/update_env_name.rs` | Reviewed |

### workspace_test_runner

| Source | Semantic review |
| --- | --- |
| `workspace_test_runner/src/admin_fixture.rs` | Reviewed |
| `workspace_test_runner/src/admin_fixture_conversion_error.rs` | Reviewed; A89 |
| `workspace_test_runner/src/admin_fixture_string.rs` | Reviewed |
| `workspace_test_runner/src/allocation_tool.rs` | Reviewed |
| `workspace_test_runner/src/allocation_tools.rs` | Reviewed |
| `workspace_test_runner/src/cargo_args.rs` | Reviewed |
| `workspace_test_runner/src/cargo_measurement_error.rs` | Reviewed |
| `workspace_test_runner/src/cargo_measurement_footer.rs` | Reviewed; A85 |
| `workspace_test_runner/src/cargo_subcommand_available.rs` | Reviewed; A86 |
| `workspace_test_runner/src/check_tool_available.rs` | Reviewed; A79 |
| `workspace_test_runner/src/clean_ansi_text.rs` | Reviewed; A23 fallback consumer, A86 bound |
| `workspace_test_runner/src/command_duration.rs` | Reviewed |
| `workspace_test_runner/src/command_duration_millis.rs` | Reviewed |
| `workspace_test_runner/src/command_failure.rs` | Reviewed |
| `workspace_test_runner/src/command_failures_vec_deque.rs` | Reviewed |
| `workspace_test_runner/src/command_index.rs` | Reviewed |
| `workspace_test_runner/src/command_run.rs` | Reviewed |
| `workspace_test_runner/src/command_started_at_instant.rs` | Reviewed |
| `workspace_test_runner/src/command_text.rs` | Reviewed; A23 fallback consumer, A80 bound |
| `workspace_test_runner/src/command_texts.rs` | Reviewed |
| `workspace_test_runner/src/commands_ref.rs` | Reviewed |
| `workspace_test_runner/src/create_admin_fixture_string.rs` | Reviewed |
| `workspace_test_runner/src/direct_generation_measurement.rs` | Reviewed |
| `workspace_test_runner/src/direct_generation_output_measurement.rs` | Reviewed |
| `workspace_test_runner/src/domain_types.rs` | Reviewed; A86 |
| `workspace_test_runner/src/execution_tests.rs` | Reviewed; A81 |
| `workspace_test_runner/src/failed_test_names.rs` | Reviewed; A81 |
| `workspace_test_runner/src/generate_pg_table_measure_input_token_stream.rs` | Reviewed |
| `workspace_test_runner/src/generation_stage_measurement.rs` | Reviewed |
| `workspace_test_runner/src/macro_generation_measurements.rs` | Reviewed |
| `workspace_test_runner/src/main.rs` | Focused review; A82 |
| `workspace_test_runner/src/measure_cargo_command.rs` | Focused review; A85, A86 |
| `workspace_test_runner/src/measure_direct_generation.rs` | Reviewed |
| `workspace_test_runner/src/measure_generation_stages.rs` | Reviewed |
| `workspace_test_runner/src/measure_memusage_command.rs` | Focused review; A79, A86 |
| `workspace_test_runner/src/measurement_name.rs` | Reviewed |
| `workspace_test_runner/src/memory_usage_column_index.rs` | Reviewed |
| `workspace_test_runner/src/memusage_heap_value.rs` | Reviewed; A83 |
| `workspace_test_runner/src/memusage_key.rs` | Reviewed |
| `workspace_test_runner/src/memusage_measurement_error.rs` | Reviewed |
| `workspace_test_runner/src/memusage_prog_name_ref.rs` | Reviewed |
| `workspace_test_runner/src/memusage_program_text.rs` | Reviewed; A84 |
| `workspace_test_runner/src/memusage_row_name.rs` | Reviewed |
| `workspace_test_runner/src/memusage_summary_text.rs` | Reviewed; A83 |
| `workspace_test_runner/src/memusage_table_value.rs` | Reviewed; A83 |
| `workspace_test_runner/src/memusage_value_ref.rs` | Reviewed |
| `workspace_test_runner/src/parse_cargo_measurement_footer.rs` | Reviewed; A85 |
| `workspace_test_runner/src/print_without_measurement_footer.rs` | Reviewed; A85 |
| `workspace_test_runner/src/print_without_memusage_footer.rs` | Reviewed; A84 |
| `workspace_test_runner/src/program_args_ref.rs` | Reviewed |
| `workspace_test_runner/src/program_path_ref.rs` | Reviewed |
| `workspace_test_runner/src/quote_token_stream_generate_pg_table_measure_input_token_stream.rs` | Reviewed |
| `workspace_test_runner/src/run_admin_fixture_cli.rs` | Pending |
| `workspace_test_runner/src/run_commands.rs` | Focused review; A80, A86, and A88 fixed; A88 removes the sort that displaced panicked worker summaries. |
| `workspace_test_runner/src/run_commands_error.rs` | Reviewed |
| `workspace_test_runner/src/run_counter.rs` | Reviewed: atomic counter is used only to distinguish runner output directories within one process; no leak or defect confirmed. |
| `workspace_test_runner/src/run_measurements_cli.rs` | Focused review; A79, A87 |
| `workspace_test_runner/src/run_report_error.rs` | Reviewed |
| `workspace_test_runner/src/run_workspace_tests.rs` | Reviewed |
| `workspace_test_runner/src/runner_cli_outcome.rs` | Reviewed |
| `workspace_test_runner/src/runner_mode.rs` | Reviewed; A82 |
| `workspace_test_runner/src/stderr_text_ref.rs` | Reviewed |
| `workspace_test_runner/src/strip_ansi.rs` | Focused review; A81 |
| `workspace_test_runner/src/strip_ansi_codes.rs` | Reviewed; A23 fallback consumer |
| `workspace_test_runner/src/summary_text.rs` | Reviewed |
| `workspace_test_runner/src/summary_text_append_error.rs` | Reviewed |
| `workspace_test_runner/src/test_runner_errors.rs` | Reviewed; A89 |
| `workspace_test_runner/src/test_workspace_test_runner.rs` | Focused review; A79, A80, A83, A84, A86 |
| `workspace_test_runner/src/text_ref.rs` | Reviewed |
| `workspace_test_runner/src/tool_available.rs` | Reviewed |
| `workspace_test_runner/src/tool_name.rs` | Reviewed |
| `workspace_test_runner/src/tool_path.rs` | Reviewed |
| `workspace_test_runner/tests/test_runner_cli.rs` | Reviewed; A82 |

A07 iterative traversal verification: final full Clippy passes, all workspace tests excluding the style crate pass, including 13 frontend_contract_validation tests and generated-client checks. The code-style suite passed once (306 tests) through the workspace runner before the subsequent Clippy-driven private-helper removal, binding cleanup and formatting changes. The public validation owner now contains the same reviewed workflow directly. The scoped work/reference vectors grow amortized with active traversal depth; reference-tree insertion replaces the former repeated active-set cloning, and no child lists or whole documents are allocated per descent. No lint allowance, dependency, crate, process-static state or lifetime exception was added. Cargo formatting/check and whitespace checks pass. Logs: target/audit_tmp/deep_composition_static.log, deep_composition_clippy.log and deep_composition_workspace.log. Initial generic serde serialization remains a confirmed open part of A07, rather than being hidden by the green traversal tests.

A07 borrowed JSON verification: cargo fmt, full Clippy with all targets/features and denied warnings, and the full workspace test command excluding tests_code_style_rust pass. All 16 frontend validation tests pass. The 306 code-style tests passed exactly once through workspace_test_runner static before the subsequent test-only Clippy corrections (semicolon and explicit numeric suffixes) and added structural-equality matrix; the suite was not repeated. The 512-level inline-schema probe exits successfully on a 128 KiB worker stack, with caller-owned trees cleaned iteratively after joining. No dependency, crate, lint suppression or policy exception was added. The public API intentionally requires Borrow<serde_json::Value> instead of Serialize and removes the now-unreachable payload/schema/document serialization error variants. This fixes the payload validator only; other generic serialization entrypoints require their own audit. Logs: target/audit_tmp/borrowed_json_static.log, borrowed_json_clippy_final.log, borrowed_json_workspace.log and inline_schema_borrowed.log.

A27 direct conversion progress: the GitCommitIdRef-to-GitCommitId conversion now returns Result and rejects excessive length before cloning borrowed text. Error fields retain the actual input length and configured bound. The existing explicit diagnostic-error conversion is unchanged for previously reviewed diagnostic adapters; it is no longer used by this data conversion. All 32 git_info tests and 27 common_routes tests pass. Provider, Cow-provider and link-building paths still substitute diagnostics and remain unfinished under A27. Final static checks pass: full Clippy and all 306 code-style tests through the workspace runner. The full workspace test command excluding tests_code_style_rust also passes; cargo fmt --check and git diff --check pass. No dependency, crate, suppression or policy exception was added. Logs: target/audit_tmp/git_commit_conversion_tests.log, git_commit_conversion_static.log and git_commit_conversion_workspace.log.

A27 Cow-provider progress: git_commit_id_cow now returns Result and propagates validation failures without converting diagnostics into an ID. Repository callers are test helpers and now inspect success explicitly. All 33 git_info tests pass, including zero/exact-bound/oversized lengths, pointer identity for borrowed text, owned fallback behavior and fallback call counts. This is an intentional provider-method API correction. Owned-provider and link-builder paths remain open under A27. Full Clippy and all 306 code-style tests pass through the workspace runner; formatting and whitespace checks pass. The full workspace test command excluding tests_code_style_rust also passes. No dependency, crate, lint allowance or policy exception was added. Logs: target/audit_tmp/git_commit_cow_tests.log, git_commit_cow_static.log and git_commit_cow_workspace.log.

A27 final correction: the owned commit provider, fallback lookup, callback adapter and both link-building APIs now propagate typed length failures through Result. Borrowed IDs are validated before invoking callbacks; failed fallback construction leaves the cache empty, and successful fallback values remain cached exactly once. URL construction checks prefix plus commit length before allocation and preserves static project-link borrowing. The git_info HTTP handler owns GitInfoResponseError; the unknown-route fallback retains its separate CommonNotFoundError. Both preserve typed failure context and produce the shared internal API problem instead of a diagnostic commit value. The typed GitInfoRoute intentionally gains an Internal error status; exactly its one reviewed public-contract snapshot entry was updated. Successful response schemas and 404 behavior remain unchanged. All 35 git_info and 28 common_routes tests pass at the first regression stage; final HTTP-adapter regression and workspace/static checks are running. One new private error module was added, without a new crate or dependency, and the complete source inventory now has 3194 unique rows. Logs: target/audit_tmp/git_commit_fallible_regressions_final.log, git_commit_http_regressions.log, git_commit_fallible_static_retry.log and git_commit_fallible_workspace.log.

A27 final verification: all 35 git_info tests and 29 common_routes tests pass, including the actual failing-provider git_info handler and in-memory unknown-route request; no HTTP/database client is constructed. Full Clippy and all 306 code-style tests pass through workspace_test_runner static on the final sources. Snapshot retries corrected the one intentional GitInfoRoute error-status entry and its required sorted position; no public entry was otherwise changed. The first workspace run failed in server_admin rustdoc with E0463 while other builds/source updates were occurring; the subsequent sequential full workspace command excluding tests_code_style_rust passes, including doctests, with sources held unchanged. Final cargo fmt/check and git diff --check pass. No dependency, crate, suppression or exception inventory was added. One private route-error module and one unique test diagnostic constant were added. Inventory is verified against disk: 191 workspace packages and 3194 unique Rust files, with zero missing or obsolete rows; 429 modules reviewed, 29 focused reviews and 2736 pending. Final logs: target/audit_tmp/git_commit_http_regressions.log, git_commit_fallible_static_final.log and git_commit_fallible_workspace_final.log. A23 remains the next confirmed conversion bug; the overall workspace audit is unfinished.

A23 additional evidence: an independently compiled call to AsRefStrToSnakeCaseTokenStream::case_or_panic with a 1048576-byte alternating-case input exits normally and returns the tokens `case string length 1572864 exceeds maximum 1048576`; the output contains no compile_error. The helper parses the earlier diagnostic substitution successfully, so merely changing token parsing cannot repair the failure. The coordinated correction must preserve a typed conversion error through all nine generated case-trait pairs and emit a compiler diagnostic at token consumers. No production code was changed in this audit step; the prior final A27 checks still describe the unchanged source state. Three additional naming marker/formatting modules were fully reviewed and recorded; their intentional HashMap/hashmap spellings agree with their existing tests. Coverage now: 432 fully reviewed files, 29 focused reviews, 2733 pending, out of 3194. Evidence: target/audit_tmp/case_token_overflow.rs and case_token_overflow.log.

A28 progress: the independently compiled default-const enum succeeds after preserving its generic impl; the old expansion compiled only the default specialization and rejected LIMIT=2 consumers. Both shared generator paths were corrected in one shared crate, with a typed borrowed generics wrapper. The unit regression invokes every one of the six public shared generator entries; no source exception, dependency or crate was added. Static checks are running; full workspace checks will follow sequentially. A23 remains unresolved and is not substituted by this separate generic-generation fix.

A28 static verification: full Clippy and all 306 code-style tests pass once through workspace_test_runner static; formatting and whitespace checks pass. The independent compiled probe also succeeds with the nontrivial `[u8; LIMIT]: Default` where-clause (target/audit_tmp/naming_enum_generic_bound.log). Three pre-existing naming leaf-wrapper modules were fully reviewed; none contain additional conversion logic or an independently confirmed defect. Full workspace tests are running sequentially with production sources unchanged.

A28 final verification: cargo fmt/check, full Clippy for all targets/features with denied warnings, all 306 code-style tests once through the workspace runner, and the complete sequential workspace test command excluding tests_code_style_rust pass, including generated-code checks and doctests. Both compiled probes exit successfully: the original LIMIT=2 consumer and the explicit array-Default where-bound consumer. The unit regression covers all six public generation paths and rejects an empty expansion. Normal nongeneric case outputs and method/trait names are preserved. No new crate, dependency, lint suppression, public snapshot or exception inventory was added. Final evidence: target/audit_tmp/naming_enum_generics_tests.log, naming_enum_generics_after.log, naming_enum_generic_bound.log, naming_enum_generics_static.log and naming_enum_generics_workspace.log. The source inventory matches all 191 packages and 3196 files exactly: 437 reviewed, 30 focused reviews, 2729 pending. A23 remains open; the complete workspace audit is unfinished.

A29 static verification: all shared naming tests pass, and full Clippy plus all 306 code-style tests pass once through workspace_test_runner static. The separately compiled duplicate-input consumer now rejects the input; because panic_location installs a tracing-only hook, a second probe catches the panic and confirms its unchanged 5680dd63 payload directly. All 53 production template rows were inspected and contain exactly one self placeholder; the only production macro invocation is naming/src/parameter.rs. No crate, dependency, lint allowance or policy exception was added. Full workspace tests are running sequentially with source files unchanged. Logs: target/audit_tmp/naming_self_cardinality_static.log and naming_self_cardinality_workspace.log.

Additional naming source review: the remaining template-generator body and all enum implementation builders were inspected, completing review of proc_macro_naming_shared/src/lib.rs. A separate compiled raw-variant probe returns r#type from both snake-case APIs (target/audit_tmp/naming_raw_enum_before.log); that records current token-spelling behavior but does not yet prove a separate bug or authorize silently changing token consumers. The panic_location root hook was inspected to explain the uncaught probe silence; its broader formatting/storage behavior remains pending.

A29 final verification: cargo fmt/check, full Clippy for all targets/features with warnings denied, all 306 code-style tests once through the workspace runner, and the complete sequential workspace test command excluding tests_code_style_rust pass, including generated-code checks and doctests. The shared tests retain the A28 generic regression and add A29 cardinality/payload checks. The compiled duplicate-self probe confirms rejection with the preserved 5680dd63 payload; all 53 production templates remain valid. No dependency, crate, suppression, public contract snapshot or exception inventory was changed. The diagnostic prefix moved into the shared string catalog without changing its value. Inventory exactly matches all 191 packages and 3197 source files: 439 reviewed, 30 focused reviews and 2728 pending. Final logs: target/audit_tmp/naming_self_cardinality_tests_retry.log, naming_self_duplicate_caught.log, naming_self_cardinality_static.log and naming_self_cardinality_workspace.log. A23 and the complete workspace audit remain unfinished.
