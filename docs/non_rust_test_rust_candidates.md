# Non-Rust tests: candidates for a Rust rewrite

Analyzed on 2026-10-02 against the current worktree, including existing browser-test edits. The inventory below records the original analysis. The migration described next has now been implemented without adding dependencies or crates.

## Implemented migration

Twenty-nine browser test declarations were removed after their backend checks were implemented in Rust. Browser navigation, reloads, selected-row updates, cookies, history, dialogs, authorization controls, rendering, visual snapshots, and JavaScript behavior remain covered by Playwright.

| Migrated checks | Native owner |
| --- | --- |
| Role batch update and atomic rollback on a conflicting rule | `server_admin/tests/admin_api.rs`: `test_postgresql_browser_role_batch_update_rolls_back_rule_conflict` |
| Invalid identifiers and unauthenticated redirects across 32 legacy/canonical detail and update routes, including error bodies without record forms | `server_admin/tests/admin_api.rs`: `test_postgresql_detail_and_update_routes_enforce_identifiers_and_authentication` |
| User and role form identifier tampering, successful selected-record updates, and unchanged other records | `server_admin/tests/admin_api.rs`: `test_postgresql_record_update_forms_bind_identifiers_and_preserve_other_rows` |
| Captured access and refresh credentials rejected after HTML sign-out | `server_admin/tests/admin_api.rs`: `test_postgresql_html_sign_out_rejects_captured_access_and_refresh_credentials` |
| Role-rule columns and nonempty rows; rule column exclusions and row widths | Existing `test_postgresql_role_rules_read_returns_table_view` and `test_postgresql_rules_read_matches_rules_table_columns` in the same integration target |
| Pagination bounds, defaults, duplicate keys, default-route validation, and table-query classification for 30 detail routes | `server_admin_contract/src/test_migrated_query_contract.rs`; production CSR uses the shared `admin_path_uses_table_query` predicate |
| Production manifest validation and the CI example/candidate substitutions | `macro_helpers/src/test_production_manifest.rs`; shared validators invoked by `workspace_scaffold manifest` and `manifest_example` |

The fifteen browser detail-query scenarios each retain one combined query case to check adapter wiring. Native query tests cover boundaries independently. Backend-only access-token probes were removed from password-reset and ban scenarios; the browser still verifies actual failed/successful sign-in behavior. Existing native lifecycle, session, filtering, public-column, and settings tests are reused rather than duplicated. Browser rate-limit alerts and audit rendering remain because those checks exercise the client.

`deploy/validate-production-manifest.sh` remains a compatibility launcher with its original argument check and exit statuses. Validation now requires the repository Rust toolchain and reads UTF-8 input bounded to 16 MiB through the existing file owner. Textual validation rules and their error priority are covered by deterministic native fixtures; Kubernetes rendering remains external. CI and release both invoke the shared example/candidate check. No speedup has been measured.

### Verification

- `cargo fmt` and `cargo run -p workspace_test_runner -- static`: passed, including Clippy with warnings denied and all 307 code-style tests. The full code-style suite ran once through the runner; the intentional public API snapshot addition was updated separately with its targeted test.
- `cargo test --workspace --exclude tests_code_style_rust`: passed, 1,353 tests.
- `cargo run -p workspace_test_runner -- database`: passed, 43 provisioned ignored tests against a separate disposable database.
- `npm run test:full`: passed, 401 Chromium browser tests and 4 skips, including visual regression, against a separate disposable browser database.
- Wasm `cargo check`, `actionlint`, JavaScript syntax checks, manifest CLI fixtures, and shell compatibility exit statuses 0/1/2: passed.
- Standalone Node tests: health-probe test passed; the local development-server test remained skipped with its existing provisioning requirement.

The supplemental Wasm Clippy check reports 17 existing frontend lint failures involving single-call helpers, wildcard matches, a collapsible condition, and a borrowed parameter. Those unrelated functions were not refactored. The required workspace Clippy check passes. No comparative performance benchmark was run.

## Scope and performance assessment

The tracked source inventory contains **47 Playwright spec files**, **2 standalone Node test files**, and **1 shell manifest validator** with inline CI test scenarios. No tracked Python, TypeScript, Ruby, Go, or other-language test implementations were found. The inventory below covers every spec file, including optional and scheduled suites. Parameterized declarations expand into multiple runnable cases; file counts are not test-case counts.

Also reviewed: browser configuration, three support modules, server startup, frontend package scripts, fuzz launcher, CI and release workflows, and relevant existing Rust tests. Generated reports, logs, dependencies, PNG baselines, SQL migrations, and Rust tests are not non-Rust test implementations.

The expected benefit comes primarily from avoiding repeated browser navigation, rendering, sign-in, and polling for backend checks. Merely replacing the JavaScript harness with Rust while keeping the same browser operations would not remove those costs. No benchmarks were run, so priorities indicate potential rather than measured speedups. Database work, password hashing, application compilation, and server startup would still cost time.

`playwright.config.js` configures one worker and disables full parallelism. Its server startup timeout is 600 seconds; that is a timeout, not a measured startup duration. Shared mutable authentication/settings fixtures constrain parallel execution. Native Rust tests would still need isolated fixtures before parallelization.

## Recommended candidates, in order

1. **Role batch update and rollback:** `test_update_api.spec.js`, particularly `test_update_roles_api_updates_a_batch_and_rolls_back_a_conflict`. Exercise create/update/read/delete through Rust integration tests and confirm both records remain unchanged after the 409 conflict. This is the clearest whole-scenario candidate: its business assertions need no browser. Retain separate UI update coverage.
2. **Identifier, authentication, and detail-query matrices:** extract invalid-identifier 422 cases, unauthenticated redirects, missing-record behavior, and detail queries ignoring list filters from the detail/update suites below. Test real route adapters and response bodies in Rust. Keep representative browser link, reload, and selected-record checks. This removes many repeated page loads while preserving UI wiring coverage.
3. **Authentication and authorization lifecycle:** move refresh replay rejection, password reset session invalidation, banning, mutation denial, session revocation, and account rate-limit backend matrices from the full suites into provisioned Rust integration tests. Keep browser cookie handling, history, dialogs, navigation permissions, and automatic client recovery scenarios.
4. **Read API contracts:** move payload/schema/column and data consistency matrices into existing Rust contract/repository tests. Keep browser checks that the frontend actually sends the correct request and renders its response. A server test cannot prove those client behaviors.
5. **Pure Rust frontend validation and query handling:** expand native tests for pagination boundaries, duplicate query keys, filter mapping, and settings validation where the implementation is Rust. Keep browser cases for initialization from `location`, preventing requests, form interaction, and error display.
6. **Manifest guard:** Rust can combine the shell validator's repeated file scans into one validation pass and give each failure a deterministic fixture. Expected benefit is small because the input is small and Kubernetes rendering remains external. Preserve current textual semantics unless an explicit contract change is reviewed.

## Complete Playwright inventory

All paths in this table are relative to `browser_acceptance/tests/`. "Split" means only the specified assertions are candidates; it does not authorize deleting the complete browser scenario. "Keep" means a Rust rewrite would still require a browser to preserve the tested behavior and has no demonstrated performance advantage.

| Spec file | Assessment | Potential Rust scope and browser coverage to retain |
| --- | --- | --- |
| `admin.spec.js` | Split | Security headers, cookie response attributes, initial-password access policy, and CRUD backend results can use Rust route tests. Keep forced-password UI, component semantics, filter dialogs, keyboard navigation, and fixed shell geometry. |
| `cross-browser.spec.js` | Keep | Chromium/Firefox/WebKit rendering and runtime-error checks require the actual engines. |
| `page-coverage.spec.js` | Split | Default-route resolution, unknown-table rejection, disabled OpenAPI rejection, and catalog contracts can use Rust. Keep rendered navigation, grids, reloads, and client errors. |
| `password-generator.spec.js` | Mostly keep | Native generator/policy fixtures are possible if testing the owning Rust implementation. Keep generated value reaching the input and show/hide toggles; deterministic unit fixtures must not depend on random output. |
| `production-readiness.spec.js` | Split, high priority | Rust integration candidates include refresh replay, CRUD persistence, reset invalidation, ban/unban, search results, and forbidden mutations. Keep browser history, permission-dependent controls, failed mutation input preservation, mobile accessibility, error monitoring, and CSRF recovery without replay. Its public deployment case already invokes Rust. |
| `screenshots.spec.js` | Keep | Documentation screenshot capture requires rendering; this is an artifact-producing scenario. |
| `test_access_session_detail.spec.js` | Split | Rust can cover invalid IDs, auth, selected/missing record, safe fields, and datetime filter serialization. Keep observed frontend request, detail navigation, and displayed values. |
| `test_actions_column.spec.js` | Mostly keep | Catalog/action availability can be checked in Rust, but consistent rendered action cells and per-row links need browser coverage. |
| `test_audit_log_page.spec.js` | Split | Generated read contract, auth, selected/missing record, and detail-query independence can use Rust. Keep CSR rows and clicked detail links. |
| `test_button_labels.spec.js` | Mostly keep | Rust can check owned label constants/rendered templates. Keep computed text transformation, password toggles, confirmation dialogs, and unchanged database display names. |
| `test_cleanup_status_detail.spec.js` | Split | Rust can cover selected/missing record, auth, and generated identifier filter. Keep client request mapping and detail link/reload behavior. |
| `test_csr_pagination.spec.js` | Split | Rust query-parser matrices can cover bounds, duplicates, defaults, unknown fields, and filter/query composition. Keep `addInitScript` initialization, zero read requests on invalid input, actual request payloads, filter clearing, and session query forwarding. |
| `test_font_family.spec.js` | Keep | Computed font family, weight, size, and dialog/popover typography require CSS rendering. |
| `test_generated_route_paths.spec.js` | Mostly keep | Rust route catalog checks are useful, but these tests verify that table pages actually issue POST read requests. Keep that observation. |
| `test_health_branding.spec.js` | Split | Health endpoint contracts and branding serialization can use Rust. Keep JavaScript probe execution, isolated failed/network-error display, optional field rendering, and reloads. |
| `test_interface_snake_case.spec.js` | Mostly keep | Native checks can audit Rust-owned labels and SSR markup. Keep live DOM auditing, accessible labels, loading state, CSS transforms, mobile wrapping, database text, and the JavaScript audit's own fixtures. |
| `test_login_attempt_detail.spec.js` | Split | Auth, selected record, and ignored list queries can use Rust. Keep clicked links and reload/display checks. |
| `test_navigation_sections.spec.js` | Mostly keep | Rust can check page-to-section catalog mapping. Keep active navigation on the rendered detail pages. |
| `test_permission_detail_pages.spec.js` | Split, high priority | Rust can cover all three resource families' invalid IDs, auth, legacy/read route equivalence, selected/missing record, and filter payload construction. Keep frontend filter requests and clicked links. |
| `test_rate_limit_detail.spec.js` | Split | Auth, selected/missing record, and identifier filtering can use Rust. Keep actual read request and link/reload behavior. |
| `test_record_action_panel_style.spec.js` | Keep | Desktop/mobile computed action panel style comparison requires rendering. |
| `test_record_query_context.spec.js` | Split, high priority | Native page/query logic can cover 15 detail resources ignoring invalid, repeated, oversized, or unrelated list queries, plus four non-table pages. Keep browser examples proving CSR initialization uses that logic and displays the same record. |
| `test_record_update_form_style.spec.js` | Keep | User/role form computed style and desktop/mobile equivalence require rendering. |
| `test_refresh_token_detail.spec.js` | Split | Rust can cover invalid IDs, auth, selected/missing record, ignored queries, and absence of token hashes in public payloads. Keep displayed detail values and navigation. |
| `test_role_read.spec.js` | Split | Rust can cover role fixtures, auth, invalid IDs, selected/missing record, and ignored queries. Keep rendered values, links, and reloads. |
| `test_role_rule_detail.spec.js` | Split | Rust can cover auth, invalid IDs, selected/missing association, route equivalence, and query independence. Keep table-to-detail navigation. |
| `test_role_rules_read.spec.js` | Split | Rust can verify table name, columns, populated API results, and row values. Keep DOM/API row equality, row count, and links. |
| `test_rule_read.spec.js` | Split | Rust can cover invalid IDs, auth, selected/missing record, and ignored queries. Keep clicked detail route and reload behavior. |
| `test_rules_table_schema.spec.js` | Split | Rust can assert API column names, excluded fields, and each row's value count. Keep header/API alignment and rendered actions count. |
| `test_sessions_actions.spec.js` | Mostly keep | Rust can verify session/detail contracts and revoke authorization. Keep icon/style equivalence, read navigation, and revoke confirmation dialog. |
| `test_settings_validation.spec.js` | Split | Rust can test invalid default-route validation. Keep client-side refusal to send PATCH, error text, and preservation of the entered value. |
| `test_shared_roles_sessions_grid.spec.js` | Mostly keep | Rust can check column/page-size metadata. Keep rendered shared grid and filter radio interactions. |
| `test_swagger_enabled.spec.js` | Split | Rust can test enabled/disabled routing, authentication, and OpenAPI version/paths/schemas. Keep active navigation and document rendering after reload under the Swagger-enabled server. |
| `test_system_role_update_action.spec.js` | Split | Rust can verify system/custom role action availability and target identifiers. Keep actual update links and form navigation. |
| `test_system_setting_detail.spec.js` | Split | Rust can cover auth, selected/missing record, safe field projection, and ignored queries. Keep table-to-detail display and reloads. |
| `test_table_api_paths.spec.js` | Mostly keep | Rust can assert unprefixed registered routes and response contracts. Keep proof that all 12 frontend tables use those routes and render successfully. |
| `test_table_cell_preview.spec.js` | Keep | Clipping, focus restoration, full-value dialogs, Unicode, markup-as-text, and empty-value reuse test actual browser/JavaScript behavior. Rust escaping tests would supplement them. |
| `test_table_height.spec.js` | Keep | Row height, compact mobile dialog bounds, and reachable pagination require layout. |
| `test_table_layout.spec.js` | Keep | Fixed headers/footers, row scrolling, and viewport geometry require layout. |
| `test_table_width.spec.js` | Keep | Available width and horizontal overflow/scrolling require layout. |
| `test_update_actions.spec.js` | Mostly keep | Native metadata checks can cover target paths. Keep per-row update navigation, icons, and computed style equality. |
| `test_update_api.spec.js` | Rewrite candidate, high priority | Move batch success and transactional rollback assertions to Rust integration tests using authoritative read results. Existing UI update suites retain display coverage. |
| `test_update_pages.spec.js` | Split, high priority | Rust can cover auth, invalid IDs, route/form ID mismatch rejection, unchanged other rows, and selected-ID contracts. Keep actual submits, reloads, hidden fields, HTML constraint validation, and successful redirect/UI behavior. |
| `test_user_read_page.spec.js` | Split | Rust can cover auth, invalid/missing IDs, selected-record isolation, and safe fields. Keep distinct links, new tabs, keyboard navigation, browser back, and rendered read-only detail. |
| `test_user_role_read.spec.js` | Split | Rust can cover auth, invalid/missing IDs, selected association, and ignored list queries. Keep link appearance, row-specific destinations, keyboard/back, and displayed values. |
| `visual-regression.spec.js` | Keep | Screenshots and geometry assertions need the browser. Status checks may be supplemented in Rust but removing screenshot scenarios would remove visual coverage. |
| `z-admin-full.spec.js` | Split, high priority | Rust can cover unauthorized direct API access, refresh, single/all-session revocation, account rate limits, and settings persistence. Keep browser refresh restoration, settings forms, dialogs, branding changes, and enforced navigation after revocation. |

## Standalone Node tests

| File and scenario | Assessment |
| --- | --- |
| `browser_acceptance/test_health_probe.mjs`: `test_health_probe_bounds_reads_and_preserves_failures` | Keep in Node. It imports the production `frontend_admin/static/health_probe.js` and substitutes JavaScript Fetch/Response/ReadableStream implementations to verify bounded reads, stream cancellation, original error identity, and 503 text. A Rust equivalent would test different code unless it still ran JavaScript; no meaningful performance benefit is established. |
| `browser_acceptance/test_development_admin.mjs`: `test_development_admin_documented_password_signs_in` | Split, low priority. Rust can check the documentation credential format and provisioned authentication/filter results. Keep actual sign-in, UI filters, role filters, and logout. It is opt-in via `RUN_DEVELOPMENT_ADMIN_TEST=1` and requires the development server/database on port 8080. |

Neither standalone `.mjs` file is selected by the Playwright `testDir` configuration or explicitly invoked by the reviewed CI workflows/package scripts. Their presence does not establish that CI runs them.

## Shell tests and other checks

| Source | Potential Rust scope | Expected benefit and limits |
| --- | --- | --- |
| `deploy/validate-production-manifest.sh` | Reject placeholders, loopback trusted proxies, insufficient digest-pinned image lines, missing production/secure-cookie values, and missing Deployment/NetworkPolicy/PodDisruptionBudget kinds. Add deterministic accepted/rejected fixtures in an existing tooling owner. | Low: replace several grep/awk processes and repeated reads. Preserve the current line-based rules and exit/error semantics. The existing test only requires at least two matching image lines; validating each service structurally would be a separate behavior change. |
| `.github/workflows/ci.yml`, `static-contracts` production manifest guard | Rust fixtures can cover rejection of the rendered example and acceptance after the exact sed substitutions. Keep real Kustomize rendering as integration coverage. | Low: rendering remains an external process. |
| `.github/workflows/release.yml`, production manifest guard | Share guard fixtures with CI; the release workflow checks rejection of the example. | Low; reuse avoids duplicated test orchestration. |
| `.github/workflows/release.yml`, changelog grep | Rust can validate that the release version appears in the changelog, preserving the current substring semantics. | Negligible for one small-file check; not a priority. |
| `.github/workflows/release.yml`, candidate/release digest equality | Rust can compare values, but registry/build/signing orchestration still owns the evidence. | Negligible; keep the shell comparison. |
| CI/release Kustomize render checks | Rust can orchestrate existing commands or supplement catalog checks. | Keep Kustomize as the authoritative renderer; rewriting it would be much larger than rewriting a test. |
| CI `actionlint`, Trivy, Gitleaks, checksum verification | Keep dedicated workflow/security tools. | Reimplementing their rules would risk coverage loss; Rust orchestration does not remove scanner work. |
| CI Taplo, typos, cargo tooling, Miri, coverage, feature/API/dependency checks | Keep established tools. Several are already Rust-based, and the others are tool invocations rather than repository-owned test implementations. | No evidence that a custom Rust replacement would improve total runtime. |

`browser_acceptance/run-server.sh` provisions/builds/starts the application; it contains setup safety checks but is not a test suite. `fuzz/run_domain.sh` launches the existing Rust fuzz target. The three `tests/support/*.js` files provide fixtures, authentication, page catalogs, and request helpers; rewriting helpers alone would leave Playwright requiring a JavaScript bridge. Frontend package scripts build CSS and append a final newline; they do not define tests. Compose/Kubernetes health probes are operational checks, not test suites to migrate.

## Existing Rust destinations and safeguards

Use existing owners before adding anything:

- `server_admin/src/test_application_tests.rs` and `test_application_html_tests.rs`: route/auth/form behavior; inspect fixture capabilities before selecting in-process or provisioned tests.
- `server_admin/src/test_adapters_repository_roles_tests.rs`: role repository behavior and transaction checks.
- `server_admin/src/test_adapters_repository_data_tables_tests.rs`: filter and query construction.
- `server_admin/src/test_domain_types_generated_tables_tests.rs`: typed routes, schemas, generated POST read contracts, and frontend filter metadata. Existing tests already cover several contract portions of the browser candidates; compare assertions rather than duplicating them.
- `frontend_admin/src/test_crud_tests.rs`, `test_data_grid_tests.rs`, `test_static_pages_tests.rs`, and `test_admin_ssr_html.rs`: native frontend logic and SSR output. SSR shell checks cannot substitute for CSR DOM rendering.
- `runtime_tests/tests/test_admin_deployment.rs`: already owns `tests::test_public_admin_deployment_endpoints`, invoked from `production-readiness.spec.js`. Moving that invocation to provisioned Rust orchestration is a harness change, not a test rewrite.
- `workspace_scaffold` and shared tooling owners: potential manifest/catalog validation ownership, subject to dependency direction and existing module responsibilities.

No new crate or dependency is authorized by this report. Future migrations must retain domain wrappers, shared logic ownership, deterministic fixtures, `test_` naming, and the repository's test-runner policy. HTTP/database client tests belong in provisioned integration coverage, marked ignored with a specific reason; ordinary unit tests must remain independent of services.

Before removing any browser case, map every assertion to its replacement or retained scenario, including headers, cookies, field secrecy, transaction rollback, and client request counts. Native validator tests do not prove that JavaScript or browser adapters invoke the validator correctly.

## How to confirm performance before migration

Measure per-test and whole-job wall time on the same provisioned environment, separating cold compilation/server startup from warm execution. Pilot the role rollback scenario first, then the identifier/detail-query matrices. Compare equivalent assertions and include fixture setup/cleanup and Rust compilation in the totals. Keep a small browser set for each distinct adapter behavior, with full browser coverage for layout, accessibility, JavaScript, screenshots, history, and engine differences.

The CI configuration also runs full browser scenarios in both `browser-acceptance` and `browser-acceptance-full` on scheduled/manual events, and runs Swagger tests in both jobs. Review whether that duplication is intentional before changing CI; consolidating equivalent runs may save more than changing harness language.

## Verification of this report

The inventory was checked against every tracked `.spec.js` file: all 47 appear exactly once in the table. Both standalone Node tests and the manifest validator are included. Referenced Rust destination files exist, and the report passes ASCII, LF, final-newline, and trailing-whitespace checks.

`cargo fmt` passed. `cargo run -p workspace_test_runner -- static` ran formatting, Clippy for all targets/features with warnings denied, and the code-style suite once. Formatting and Clippy passed; 306 code-style tests passed, and the English-text test initially rejected curly quotes in this report. After replacing those quotes, the same compiled English-text test passed in a targeted rerun. `cargo test --workspace --exclude tests_code_style_rust` passed. No provisioned database or browser suites were launched for this documentation-only analysis.
