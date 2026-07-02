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

Status legend: `[x]` migrated and verified, `[~]` exists in target but still needs source merge audit, `[ ]` not migrated yet.

1. Base independent crates: [x] `gen_quotes`, [x] `naming/naming_cmn`, [x] `panic_loc`, [x] `to_err_string`
   -> verify: each crate is added to `members`, dependencies are moved to `[workspace.dependencies]`, then run `cargo check -p <crate>`.

2. Existing [x] `optml`
   -> verify: merge with the current `optml` crate, do not replace the directory wholesale; then run `cargo test -p optml`.

3. Macro naming layer: [x] `token_patterns`, [x] `naming/naming_macros`, [x] `naming`
   -> verify: `cargo check -p token_patterns -p naming_macros -p naming`.

4. Macro helper layer: [x] `macros_helpers/gen_derive_ts_builder`, [x] `config_lib/gen_getter_traits_for_struct_fields`, [x] `macros_helpers`
   -> verify: `cargo check -p gen_derive_ts_builder -p gen_getter_traits_for_struct_fields -p macros_helpers`.

5. Config and git/meta infrastructure: [x] `config_lib/try_from_env`, [x] `config_lib`, [x] `git_info`
   -> verify: `cargo check -p try_from_env -p config_lib -p git_info`.

6. Runtime foundation: [x] `app_state`, [x] `cmn_routes`, [x] `server_config`
   -> verify: `cargo check -p app_state -p cmn_routes -p server_config`.

7. Location/domain layer: `loc_lib/location`, `loc_lib`
   -> verify: `cargo check -p location -p loc_lib`.

8. Shared PostgreSQL proc-macro layer: `pg_crud/pg_crud_macros_cmn`
   -> verify: `cargo check -p pg_crud_macros_cmn`.

9. PostgreSQL source generator crates: `pg_crud/pg_json/gen_pg_json_src`, `pg_crud/pg_json_obj/gen_pg_json_obj_src`, `pg_crud/pg_tbl/gen_pg_tbl_src`, `pg_crud/wh_flts/gen_wh_flts_src`, `pg_crud/pg_types/gen_pg_types_src`
   -> verify: `cargo check -p gen_pg_json_src -p gen_pg_json_obj_src -p gen_pg_tbl_src -p gen_wh_flts_src -p gen_pg_types_src`.

10. PostgreSQL proc-macro wrappers: `pg_crud/pg_json/gen_pg_json`, `pg_crud/pg_json_obj/gen_pg_json_obj`, `pg_crud/pg_tbl/gen_pg_tbl`, `pg_crud/wh_flts/gen_wh_flts`, `pg_crud/pg_types/gen_pg_types`
    -> verify: `cargo check -p gen_pg_json -p gen_pg_json_obj -p gen_pg_tbl -p gen_wh_flts -p gen_pg_types`.

11. Base PostgreSQL runtime crates: `pg_crud/pg_crud_cmn`, `pg_crud/wh_flts`, `pg_crud/pg_json_obj/pg_json_obj_cmn`, `pg_crud/pg_types/pg_types_cmn`
    -> verify: `cargo check -p pg_crud_cmn -p wh_flts -p pg_json_obj_cmn -p pg_types_cmn`.

12. PostgreSQL leaf crates: `pg_crud/pg_json/pg_json_nbr`, `pg_crud/pg_json/pg_json_other`, `pg_crud/pg_json_obj`, `pg_crud/pg_types/pg_types_chrono_net`, `pg_crud/pg_types/pg_types_numeric`, `pg_crud/pg_types/pg_types_text_misc`, `pg_crud/pg_tbl`
    -> verify: `cargo check -p pg_json_nbr -p pg_json_other -p pg_json_obj -p pg_types_chrono_net -p pg_types_numeric -p pg_types_text_misc -p pg_tbl`.

13. Aggregator crates: `pg_crud/pg_json`, `pg_crud/pg_types`, then `pg_crud`
    -> verify: `cargo check -p pg_json -p pg_types -p pg_crud`.

14. API/domain boundary: `route_validators`, `server_types`
    -> verify: `cargo check -p route_validators -p server_types`.

15. Server state and example table layer: `server_app_state`, `server_tbl_example`
    -> verify: `cargo check -p server_app_state -p server_tbl_example`.

16. Binary crates: first [~] `server`, then [ ] `telegram_bot` only if needed
    -> verify: `cargo check -p server`; for the bot, run `cargo check -p telegram_bot`.

17. Test/support crates: [ ] `macro_clippy_check_cmn`, [ ] `loc_lib/loc_test`, every [ ] `*_test` and [ ] `*_test_cnt`, then the existing [~] `tests`
    -> verify: `cargo test --quiet`; if feature flags such as `test-utils` remain, also run `cargo test --all-features`.

## Main Risk

The source `tufa_project` enables default features for many crates.io dependencies. The target template requires default features to stay disabled unless they are specifically needed. Each migration step must normalize `Cargo.toml`; otherwise failures will come from workspace policy as well as migrated code.
