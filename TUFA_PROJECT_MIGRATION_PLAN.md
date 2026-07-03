# Partial Migration Plan For `tufa_project`

Source workspace: `/home/sergey/projects/tufa_project`.

Target workspace: `/home/sergey/projects/rust_workspace_template`.

Goal: move crates by dependency layers so each step can be verified with a limited set of workspace members.

## General Rules

- Do not copy the whole workspace in one step.
- `optml`, `server`, and `tests` already exist in the target workspace, but they differ from the `tufa_project` versions. Merge them manually instead of replacing whole directories.
- Declare crates.io dependencies only in the root `[workspace.dependencies]`.
- Use `dependency.workspace = true` inside workspace crates.
- Keep default features disabled unless a specific feature is required.
- Run the listed verification command after each step.
- After the migration is complete, run:
  - `cargo fmt`
  - `cargo clippy --all-targets --all-features -- -D warnings`
  - `cargo test --quiet`

## Migration Order

Status legend: `[x]` source migration audited and verified, `[~]` exists in target but still needs source merge audit, `[ ]` not migrated yet.

Current audit note: the target workspace contains the listed crates and passed the previous workspace checks, but comparison with `/home/sergey/projects/tufa_project` shows that many crates still contain reduced or rewritten implementations. Treat `[~]` as "crate scaffold exists and compiles, but source parity is not proven".

1. Base independent crates: [~] `gen_quotes`, [~] `naming/naming_cmn`, [~] `panic_loc`, [~] `to_err_string`
   -> verify: each crate is added to `members`, dependencies are moved to `[workspace.dependencies]`, then run `cargo check -p <crate>`.

2. Existing [~] `optml`
   -> verify: merge with the current `optml` crate, do not replace the directory wholesale; then run `cargo test -p optml`.

3. Macro naming layer: [~] `token_patterns`, [~] `naming/naming_macros`, [~] `naming`
   -> verify: `cargo check -p token_patterns -p naming_macros -p naming`.

4. Macro helper layer: [~] `macros_helpers/gen_derive_ts_builder`, [~] `config_lib/gen_getter_traits_for_struct_fields`, [~] `macros_helpers`
   -> verify: `cargo check -p gen_derive_ts_builder -p gen_getter_traits_for_struct_fields -p macros_helpers`.

5. Config and git/meta infrastructure: [~] `config_lib/try_from_env`, [~] `config_lib`, [~] `git_info`
   -> verify: `cargo check -p try_from_env -p config_lib -p git_info`.

6. Runtime foundation: [~] `app_state`, [~] `cmn_routes`, [~] `server_config`
   -> verify: `cargo check -p app_state -p cmn_routes -p server_config`.

7. Location/domain layer: [~] `loc_lib/location`, [~] `loc_lib`
   -> verify: `cargo check -p location -p loc_lib`.

8. Shared PostgreSQL proc-macro layer: [~] `pg_crud/pg_crud_macros_cmn`
   -> verify: `cargo check -p pg_crud_macros_cmn`.

9. PostgreSQL source generator crates: [~] `pg_crud/pg_json/gen_pg_json_src`, [~] `pg_crud/pg_json_obj/gen_pg_json_obj_src`, [~] `pg_crud/pg_tbl/gen_pg_tbl_src`, [~] `pg_crud/wh_flts/gen_wh_flts_src`, [~] `pg_crud/pg_types/gen_pg_types_src`
   -> verify: `cargo check -p gen_pg_json_src -p gen_pg_json_obj_src -p gen_pg_tbl_src -p gen_wh_flts_src -p gen_pg_types_src`.

10. PostgreSQL proc-macro wrappers: [~] `pg_crud/pg_json/gen_pg_json`, [~] `pg_crud/pg_json_obj/gen_pg_json_obj`, [~] `pg_crud/pg_tbl/gen_pg_tbl`, [~] `pg_crud/wh_flts/gen_wh_flts`, [~] `pg_crud/pg_types/gen_pg_types`
    -> verify: `cargo check -p gen_pg_json -p gen_pg_json_obj -p gen_pg_tbl -p gen_wh_flts -p gen_pg_types`.

11. Base PostgreSQL runtime crates: [~] `pg_crud/pg_crud_cmn`, [~] `pg_crud/wh_flts`, [~] `pg_crud/pg_json_obj/pg_json_obj_cmn`, [~] `pg_crud/pg_types/pg_types_cmn`
    -> verify: `cargo check -p pg_crud_cmn -p wh_flts -p pg_json_obj_cmn -p pg_types_cmn`.

12. PostgreSQL leaf crates: [~] `pg_crud/pg_json/pg_json_nbr`, [~] `pg_crud/pg_json/pg_json_other`, [~] `pg_crud/pg_json_obj`, [~] `pg_crud/pg_types/pg_types_chrono_net`, [~] `pg_crud/pg_types/pg_types_numeric`, [~] `pg_crud/pg_types/pg_types_text_misc`, [~] `pg_crud/pg_tbl`
    -> verify: `cargo check -p pg_json_nbr -p pg_json_other -p pg_json_obj -p pg_types_chrono_net -p pg_types_numeric -p pg_types_text_misc -p pg_tbl`.

13. Aggregator crates: [~] `pg_crud/pg_json`, [~] `pg_crud/pg_types`, then [~] `pg_crud`
    -> verify: `cargo check -p pg_json -p pg_types -p pg_crud`.

14. API/domain boundary: [~] `route_validators`, [~] `server_types`
    -> verify: `cargo check -p route_validators -p server_types`.

15. Server state and example table layer: [~] `server_app_state`, [~] `server_tbl_example`
    -> verify: `cargo check -p server_app_state -p server_tbl_example`.

16. Binary crates: first [~] `server`, then [~] `telegram_bot`
    -> verify: `cargo check -p server`; for the bot, run `cargo check -p telegram_bot`.

17. Test/support crates: [~] `macro_clippy_check_cmn`, [~] `loc_lib/loc_test`, every [~] `*_test` and [~] `*_test_cnt`, then the existing [~] `tests`
    -> verify: `cargo test --quiet`; if feature flags such as `test-utils` remain, also run `cargo test --all-features`.

## Main Risk

The source `tufa_project` enables default features for many crates.io dependencies. The target template requires default features to stay disabled unless they are specifically needed. Each migration step must normalize `Cargo.toml`; otherwise failures will come from workspace policy as well as migrated code.

## Source Parity Audit Backlog

1. First layer audit:
   - `gen_quotes`: file list matches source, but implementation is policy-adapted. Source returns `String`, uses local imports and panics on token parsing failure; target uses `QuotedLiteralText`, explicit paths, and `compile_error!` token fallback. Source parity is not proven.
   - `naming/naming_cmn`: file list matches source, but implementation is policy-adapted. Source used local `macro_rules! case_trait_pair`; target uses the dedicated `naming_macros` proc-macro crate and domain return wrappers. Source parity is not proven.
   - `panic_loc`: file list matches source, but implementation is policy-adapted. The source `panic_with_location_msg` behavior is restored through public `panic_with_location_message` and domain wrapper parameters because private single-use helpers are forbidden by workspace lint policy. Source parity is still not exact because target moved message constants into `naming_constants` and writes through explicit `std::io::Write`.
   - `to_err_string`: file list matches source, but source API coverage is incomplete in target. Source implements `ToErrString` for external types from `axum`, `http`, `http-body`, `reqwest`, `serde_json`, `sqlx`, `time`, and `tracing`; target currently keeps only a reduced dependency-free surface plus `to_err_string_macros`.

2. Missing source files already observed:
   - `config_lib`: `src/str_from_enum_macros.rs` and empty source README files in generator crates have been restored in policy-adapted form. `str_from_enum_macros` now preserves the source helper behavior for case-insensitive lookup, allowed-values ordering, empty variant error text, and allocation-conscious allowed-values formatting, but still uses domain result/error wrappers instead of source `Result<T, String>` boundary types.
   - `loc_lib`: empty source README files in `loc_lib` and `loc_lib/location` have been restored.
   - `macros_helpers`: `attr_ident_str`, `derive_ts_builder`, policy-adapted `gen_field_loc_new_ts`, `gen_if_write_is_err_ts`, `gen_impl_*`, `gen_new_or_try_new`, `gen_pub_type_al_ts`, `gen_simple_syn_punct`, `loc_syn_field`, `panic_if_err`, `pgn_start_end_init_ts`, `rs_file_path`, `syn_field`, `test_hlp`, `wrap_derive`, a minimal policy-adapted `status_code` module, and placeholder modules for `get_macro_attr`, `loc`, `write_string_into_file`, and `write_ts_into_file` have been restored. The `panic_if_err`, `rs_file_path`, `test_hlp`, `get_macro_attr`, `loc`, `write_string_into_file`, and `write_ts_into_file` modules are placeholders because the source behavior uses `use` imports, aliases, `panic!`, `expect`, `assert!`, `assert_eq!`, `PathBuf`/`str`/`syn` boundary types, and global atomic test state that conflict with current style rules. The `derive_ts_builder` file is source-shaped, but its proc-macro generator remains a target stub.
   - `pg_crud/pg_crud_macros_cmn`: `src/flts.rs` has been restored in policy-adapted form with `PgTypeFlt`, `PgJsonFlt`, `PgFlt`, and dimension constructors; source-only `strum`, `Optml`, root re-export, and naming-token behavior are still not exact.
   - `route_validators`: `check_body_size.rs`, `check_commit.rs`, `hdr_val.rs`, and `test_hlp.rs` have been restored in policy-adapted form. `check_body_size` now preserves the source validation branches for body below limit, body equal to limit, empty body with zero limit, body over limit, and non-empty body with zero limit, but it models those branches with local domain enums instead of source `axum::body::Body`/`bytes::Bytes`/`to_bytes` behavior. `check_commit` now preserves the source validation branches for disabled validation, accepted project commit, missing commit header, non-text commit header, and mismatched commit, but it still models those branches with local domain enums instead of source `axum::http::HeaderMap`/`HeaderValue` parsing. `hdr_val` now preserves the source helper branches for raw header present/missing, string conversion success/failure, parsed value success/failure, and no-parse behavior for missing/non-text headers, but it still models those branches with local domain enums instead of source `HeaderMap`/`HeaderValue`/`AsHeaderName` helpers because `axum`, `http-body`, `bytes`, `serde`, and `thiserror` are not currently declared in the target workspace dependencies.
   - `server_types`: empty source README has been restored. Target now includes policy-adapted top-level `Animal`, `Doggie`, and `Cat` domain markers with source-active field marker getters, plus the existing route-validation boundary wrapper. Source `src/lib.rs` still has 1058 lines of `pg_crud`/`optml` generated API surface while target only models the top-level object shape and active field markers; exact migration requires adding/normalizing `serde`, `serde_json`, `thiserror`, `sqlx`, `utoipa`, `schemars`, `uuid`, `loc_lib`, `pg_crud`, and `optml` dependencies/features under workspace policy and then adapting public fields/derives to current style rules.

3. Large implementation gaps by source line count:
   - `pg_crud/pg_json_obj/gen_pg_json_obj_src`
   - `pg_crud/pg_tbl/gen_pg_tbl_src`
   - `pg_crud/pg_types/gen_pg_types_src`
   - `pg_crud/pg_json/gen_pg_json_src`
   - `pg_crud/wh_flts/gen_wh_flts_src`
   - `pg_crud/pg_crud_cmn`
   - `pg_crud/pg_crud_macros_cmn`
   - `server_types`
   - `tests`

4. Current exact source-file presence gap from `find`/`comm` audit:
   - 0 files exist in `/home/sergey/projects/tufa_project` but not in target.
   - Repository/config docs: empty source `.gitmodules`, `.config/nextest.toml`, `config.toml`, `CLAUDE.md`, `NAMING.md`, `ide_configuration.md`, `setup.md`, `todo.md`, the source file named `use` + `full.md`, and `variable_names.txt` have been restored.
   - `loc_lib`: `.gitignore`, `loc_test/.env`.
   - `macros_helpers`: empty source `README.md` and source file names `src/get_macro_attr.rs`, `src/loc.rs`, `src/write_string_into_file.rs`, and `src/write_ts_into_file.rs` have been restored; the Rust implementations are placeholder/policy-adapted, not source-equivalent.
   - `server`: `.dockerignore`, `.envexample`, `docker-compose.yml`, `example.http`, and `dockerfile` have been restored.
   - `telegram_bot`: empty source `README.md`, empty source `.github/workflows/continuous_integration.yml`, and `.env` have been restored.
   - `.typos.toml` excludes `todo.md`, the source file named `use` + `full.md`, and `variable_names.txt` so those restored source artifacts can keep their exact source spelling while the workspace typo gate remains enabled for regular files.
   - `typos` was checked against `loc_lib/.gitignore` and `server/dockerfile`; both pass the current workspace typo gate as restored.
   - Target also intentionally has template-only files that source does not have, including `naming_constants`, `to_err_string_macros`, style tests, root Docker/template files, and `TUFA_PROJECT_MIGRATION_PLAN.md`; these are not source parity proof.
