#[cfg(test)]
mod tests {
    #[test]
    fn clippy() {
        macro_clippy_check_cmn::clippy_check(
            "gen_pg_tbl_test_cnt",
            "../pg_crud/pg_tbl/",
            r#"[dependencies]
axum = { workspace = true }
futures = { workspace = true }
http = { workspace = true }
sqlx = { workspace = true }
reqwest = { workspace = true }
serde = { workspace = true }
serde_json = { workspace = true }
thiserror = { workspace = true }
utoipa = { workspace = true }
tracing = { workspace = true }
wh_flts = { workspace = true }
git_info = { workspace = true }
loc_lib = { workspace = true }
loc_macros = { workspace = true }
metrics = { workspace = true }
location = { workspace = true }
pg_crud = { workspace = true, features = ["test-utils"] }
pg_crud_cmn = { workspace = true }
pg_tbl = { workspace = true }
pg_types_numeric = { workspace = true }
pg_types_text_misc = { workspace = true }
gen_pg_tbl = { workspace = true }
optml = { workspace = true }
route_validators = { workspace = true }
to_err_string = { workspace = true }
"#,
            &{
                #[derive(optml::Optml)]
                enum AddGenPgTblPk {
                    False,
                    True,
                }
                let allow_clippy_arbitrary_src_item_ordering =
                    token_patterns::AllowClippyArbitrarySrcItemOrdering;
                let gen_tbl_example_ts = |add_gen_pg_tbl_pk: AddGenPgTblPk| {
                    let mb_gen_pg_tbl_pk_ts = match add_gen_pg_tbl_pk {
                        AddGenPgTblPk::False => proc_macro2::TokenStream::new(),
                        AddGenPgTblPk::True => {
                            quote::quote! {#[gen_pg_tbl_pk]}
                        }
                    };
                    quote::quote! {
                        #allow_clippy_arbitrary_src_item_ordering
                        #[derive(Debug, Clone, Copy, optml::Optml)]
                        #[gen_pg_tbl::gen_pg_tbl_config{{
                            "cm_write_into_file": "False",
                            "co_write_into_file": "False",
                            "rm_write_into_file": "False",
                            "ro_write_into_file": "False",
                            "um_write_into_file": "False",
                            "uo_write_into_file": "False",
                            "dm_write_into_file": "False",
                            "dlo_write_into_file": "False",
                            "tests_write_into_file": "False",
                            "cmn_write_into_file": "False",
                            "whole_write_into_file": "False"
                        }}]
                        #[gen_pg_tbl::cm_er_vrts{enum CmErVrts{}}]
                        #[gen_pg_tbl::co_er_vrts{enum CoErVrts{}}]
                        #[gen_pg_tbl::rm_er_vrts{enum RmErVrts{}}]
                        #[gen_pg_tbl::ro_er_vrts{enum RoErVrts{}}]
                        #[gen_pg_tbl::um_er_vrts{enum UmErVrts{}}]
                        #[gen_pg_tbl::uo_er_vrts{enum UoErVrts{}}]
                        #[gen_pg_tbl::dm_er_vrts{enum DmErVrts{}}]
                        #[gen_pg_tbl::dlo_er_vrts{enum DloErVrts{}}]
                        #[gen_pg_tbl::cmn_er_vrts{
                            enum CmnErVrts {
                                CheckCommit {
                                    #[eo_loc]
                                    check_commit: route_validators::check_commit::CommitEr,
                                    loc: loc_lib::loc::Loc,
                                },
                            }
                        }]
                        #[gen_pg_tbl::cm_logic{}]
                        #[gen_pg_tbl::co_logic{}]
                        #[gen_pg_tbl::rm_logic{}]
                        #[gen_pg_tbl::ro_logic{}]
                        #[gen_pg_tbl::um_logic{}]
                        #[gen_pg_tbl::uo_logic{}]
                        #[gen_pg_tbl::dm_logic{}]
                        #[gen_pg_tbl::dlo_logic{}]
                        #[gen_pg_tbl::cmn_logic{}]
                        pub struct TblExample {
                            #mb_gen_pg_tbl_pk_ts
                            pub pk_col:
                                pg_types_text_misc::SqlxTypesUuidUuidAsNnUuidV4InitByPg,
                            pub col_0: pg_types_numeric::I16AsNnInt2,
                            pub col_1: pg_types_numeric::OptI16AsNlInt2,
                            pub col_2: pg_types_numeric::I32AsNnInt4,
                        }
                    }
                };
                let gen_pg_tbl_input_ts = gen_tbl_example_ts(AddGenPgTblPk::True);
                let ts = gen_pg_tbl_src::gen_pg_tbl(
                    macros_helpers::ts_writer::ProcMacro2TsRef::from(&gen_pg_tbl_input_ts),
                );
                let tbl_struct_ts = gen_tbl_example_ts(AddGenPgTblPk::False);
                quote::quote! {
                    #ts
                    #tbl_struct_ts
                }
            }
            .to_string(),
        );
    }
}
