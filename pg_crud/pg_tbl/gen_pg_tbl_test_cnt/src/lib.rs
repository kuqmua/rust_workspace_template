#[allow(unused_qualifications)]
#[allow(clippy::absolute_paths)]
mod tbl_example_gen_pg_tbl_mod {
    use super::TblExample;
    #[allow(clippy::arbitrary_source_item_ordering)]
    impl TblExample {
        #[must_use]
        pub const fn tbl_name() -> &'static str {
            "tbl_example"
        }
        const fn pk() -> &'static str {
            "pk_col"
        }
        pub async fn prep_extensions(
            pool: &sqlx::Pool<sqlx::Postgres>,
        ) -> Result<(), TblExamplePrepPgEr> {
            if let Err(er) = sqlx::query("create extension if not exists \"uuid-ossp\"")
                .execute(pool)
                .await
            {
                return Err(TblExamplePrepPgEr::CrExtensionIfNotExistsUuidOssp {
                    er,
                    loc: loc_lib::loc!(),
                });
            }
            Ok(())
        }
        pub async fn prep_pg_tbl(
            pool: &sqlx::Pool<sqlx::Postgres>,
            tbl: &str,
        ) -> Result<(), TblExamplePrepPgEr> {
            if let Err(er) = sqlx::query(&format!(
                "create table if not exists {tbl} ({},{},{},{})",
                <pg_crud::SqlxTypesUuidUuidAsNnUuidV4InitByPg as pg_crud::PgType>::cr_tbl_col_qp(
                    &"pk_col", true
                ),
                <pg_crud::I16AsNnInt2 as pg_crud::PgType>::cr_tbl_col_qp(&"col_0", false),
                <pg_crud::OptI16AsNlInt2 as pg_crud::PgType>::cr_tbl_col_qp(&"col_1", false),
                <pg_crud::I32AsNnInt4 as pg_crud::PgType>::cr_tbl_col_qp(&"col_2", false)
            ))
            .execute(pool)
            .await
            {
                return Err(TblExamplePrepPgEr::PrepPg {
                    er,
                    loc: loc_lib::loc!(),
                });
            }
            Ok(())
        }
        pub async fn prep_pg(pool: &sqlx::Pool<sqlx::Postgres>) -> Result<(), TblExamplePrepPgEr> {
            Self::prep_extensions(pool).await?;
            Self::prep_pg_tbl(pool, "tbl_example").await?;
            Ok(())
        }
        #[must_use]
        pub const fn allow_methods() -> [http::Method; 4] {
            [
                http::Method::GET,
                http::Method::POST,
                http::Method::PATCH,
                http::Method::DELETE,
            ]
        }
        fn gen_sel_qp(
            sel: &pg_crud::NotEmptyUnqVec<TblExampleSel>,
        ) -> Result<String, pg_crud::QpEr> {
            let mut acc = String::new();
            for el in sel.to_vec() {
                acc . push_str (& match el { TblExampleSel :: PkCol (col) => match < pg_crud :: SqlxTypesUuidUuidAsNnUuidV4InitByPg as pg_crud :: PgType > :: sel_qp (col , "pk_col") { Ok (v_820e1163) => v_820e1163 , Err (er_0) => { { return Err (er_0) ; } } } , TblExampleSel :: Col0 (col) => match < pg_crud :: I16AsNnInt2 as pg_crud :: PgType > :: sel_qp (col , "col_0") { Ok (v_820e1163) => v_820e1163 , Err (er_0) => { { return Err (er_0) ; } } } , TblExampleSel :: Col1 (col) => match < pg_crud :: OptI16AsNlInt2 as pg_crud :: PgType > :: sel_qp (col , "col_1") { Ok (v_820e1163) => v_820e1163 , Err (er_0) => { { return Err (er_0) ; } } } , TblExampleSel :: Col2 (col) => match < pg_crud :: I32AsNnInt4 as pg_crud :: PgType > :: sel_qp (col , "col_2") { Ok (v_820e1163) => v_820e1163 , Err (er_0) => { { return Err (er_0) ; } } } }) ;
                acc.push(',');
            }
            let _: Option<char> = acc.pop();
            Ok(acc)
        }
        #[allow(clippy::single_call_fn)]
        async fn cm_h(
            app_state: axum::extract::State<
                std::sync::Arc<dyn pg_crud::CombinationOfAppStateLogicTraits>,
            >,
            req: axum::extract::Request,
            tbl: &str,
        ) -> axum::response::Response {
            let (parts, body) = req.into_parts();
            let headers = parts.headers;
            if !matches ! (headers . get (http :: header :: CONTENT_TYPE) , Some (v_e3f6eecd) if v_e3f6eecd == http :: header :: HeaderValue :: from_static ("application/json"))
            {
                let er = TblExampleCmEr::HeaderContentTypeAppJsonNotFound {
                    loc: loc_lib::loc::Loc::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(loc_lib::loc::Occr {
                            file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                            line: 2555,
                            col: 25,
                        }),
                    ),
                };
                let mut res = axum::response::IntoResponse::into_response(axum::Json(
                    TblExampleCmResVrts::from_h(er),
                ));
                *res.status_mut() = http::StatusCode::BAD_REQUEST;
                return res;
            }
            let body_bytes = match pg_crud::check_body_size::check_body_size(
                body,
                *app_state.get_maximum_size_of_http_body_in_bytes(),
            )
            .await
            {
                Ok(v_cfac9140) => v_cfac9140,
                Err(er_0) => {
                    let er = TblExampleCmEr::CheckBodySize {
                        check_body_size: er_0,
                        loc: loc_lib::loc::Loc::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(loc_lib::loc::Occr {
                                file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                                line: 2564,
                                col: 33,
                            }),
                        ),
                    };
                    let mut res = axum::response::IntoResponse::into_response(axum::Json(
                        TblExampleCmResVrts::from_h(er),
                    ));
                    *res.status_mut() = http::StatusCode::BAD_REQUEST;
                    return res;
                }
            };
            let prms = TblExampleCmPrms {
                payload: match serde_json::from_slice::<TblExampleCmPayload>(&body_bytes) {
                    Ok(v_9e6fcd2d) => v_9e6fcd2d,
                    Err(er_0) => {
                        let er = TblExampleCmEr::SerdeJson {
                            serde_json: er_0,
                            loc: loc_lib::loc::Loc::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(loc_lib::loc::Occr {
                                    file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                                    line: 2609,
                                    col: 37,
                                }),
                            ),
                        };
                        let mut res = axum::response::IntoResponse::into_response(axum::Json(
                            TblExampleCmResVrts::from_h(er),
                        ));
                        *res.status_mut() = http::StatusCode::BAD_REQUEST;
                        return res;
                    }
                },
            };
            let query_string = pg_crud::gen_cm_query_string(
                tbl,
                "pk_col,col_0,col_1,col_2",
                &{
                    let mut incr: u64 = 0;
                    let mut acc_8a58994e = String::new();
                    for el_1651705d in &prms.payload.0 {
                        match el_1651705d.cr_qp(&mut incr) {
                            Ok(v_f4fdd10d) => {
                                if {
                                    use std::fmt::Write as _;
                                    write!(acc_8a58994e, "({v_f4fdd10d}),")
                                }
                                .is_err()
                                {
                                    let er_0 = pg_crud::QpEr::WriteIntoBuffer {
                                        loc: loc_lib::loc!(),
                                    };
                                    let er = TblExampleCmEr::Qp {
                                        er: er_0,
                                        loc: loc_lib::loc::Loc::new(
                                            file!().to_owned(),
                                            line!(),
                                            column!(),
                                            Some(loc_lib::loc::Occr {
                                                file: String::from(
                                                    "pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs",
                                                ),
                                                line: 2319,
                                                col: 80,
                                            }),
                                        ),
                                    };
                                    let mut res = axum::response::IntoResponse::into_response(
                                        axum::Json(TblExampleCmResVrts::from_h(er)),
                                    );
                                    *res.status_mut() = http::StatusCode::BAD_REQUEST;
                                    return res;
                                }
                            }
                            Err(er_0) => {
                                let er = TblExampleCmEr::Qp {
                                    er: er_0,
                                    loc: loc_lib::loc::Loc::new(
                                        file!().to_owned(),
                                        line!(),
                                        column!(),
                                        Some(loc_lib::loc::Occr {
                                            file: String::from(
                                                "pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs",
                                            ),
                                            line: 2319,
                                            col: 80,
                                        }),
                                    ),
                                };
                                let mut res = axum::response::IntoResponse::into_response(
                                    axum::Json(TblExampleCmResVrts::from_h(er)),
                                );
                                *res.status_mut() = http::StatusCode::BAD_REQUEST;
                                return res;
                            }
                        }
                    }
                    let _: Option<char> = acc_8a58994e.pop();
                    acc_8a58994e
                },
                &{
                    let mut acc_a35168d8 = String::new();
                    match < pg_crud :: SqlxTypesUuidUuidAsNnUuidV4InitByPg as pg_crud :: PgType > :: sel_only_ids_qp ("pk_col") { Ok (v_aa341baf) => { acc_a35168d8 . push_str (& v_aa341baf) ; } , Err (er_0) => { let er = TblExampleCmEr :: Qp { er : er_0 , loc : loc_lib :: loc :: Loc :: new (file ! () . to_owned () , line ! () , column ! () , Some (loc_lib :: loc :: Occr { file : String :: from ("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs") , line : 2319 , col : 80 , })) } ; let mut res = axum :: response :: IntoResponse :: into_response (axum :: Json (TblExampleCmResVrts :: from_h (er))) ; * res . status_mut () = http :: StatusCode :: BAD_REQUEST ; return res ; } }
                    match <pg_crud::I16AsNnInt2 as pg_crud::PgType>::sel_only_ids_qp("col_0") {
                        Ok(v_aa341baf) => {
                            acc_a35168d8.push_str(&v_aa341baf);
                        }
                        Err(er_0) => {
                            let er = TblExampleCmEr::Qp {
                                er: er_0,
                                loc: loc_lib::loc::Loc::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(loc_lib::loc::Occr {
                                        file: String::from(
                                            "pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs",
                                        ),
                                        line: 2319,
                                        col: 80,
                                    }),
                                ),
                            };
                            let mut res = axum::response::IntoResponse::into_response(axum::Json(
                                TblExampleCmResVrts::from_h(er),
                            ));
                            *res.status_mut() = http::StatusCode::BAD_REQUEST;
                            return res;
                        }
                    }
                    match <pg_crud::OptI16AsNlInt2 as pg_crud::PgType>::sel_only_ids_qp("col_1") {
                        Ok(v_aa341baf) => {
                            acc_a35168d8.push_str(&v_aa341baf);
                        }
                        Err(er_0) => {
                            let er = TblExampleCmEr::Qp {
                                er: er_0,
                                loc: loc_lib::loc::Loc::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(loc_lib::loc::Occr {
                                        file: String::from(
                                            "pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs",
                                        ),
                                        line: 2319,
                                        col: 80,
                                    }),
                                ),
                            };
                            let mut res = axum::response::IntoResponse::into_response(axum::Json(
                                TblExampleCmResVrts::from_h(er),
                            ));
                            *res.status_mut() = http::StatusCode::BAD_REQUEST;
                            return res;
                        }
                    }
                    match <pg_crud::I32AsNnInt4 as pg_crud::PgType>::sel_only_ids_qp("col_2") {
                        Ok(v_aa341baf) => {
                            acc_a35168d8.push_str(&v_aa341baf);
                        }
                        Err(er_0) => {
                            let er = TblExampleCmEr::Qp {
                                er: er_0,
                                loc: loc_lib::loc::Loc::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(loc_lib::loc::Occr {
                                        file: String::from(
                                            "pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs",
                                        ),
                                        line: 2319,
                                        col: 80,
                                    }),
                                ),
                            };
                            let mut res = axum::response::IntoResponse::into_response(axum::Json(
                                TblExampleCmResVrts::from_h(er),
                            ));
                            *res.status_mut() = http::StatusCode::BAD_REQUEST;
                            return res;
                        }
                    }
                    let _: Option<char> = acc_a35168d8.pop();
                    acc_a35168d8
                },
            );
            let binded_query = {
                let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
                for el_7f862135 in prms.payload.0 {
                    match el_7f862135.cr_qb(query) {
                        Ok(v_011a3eb4) => {
                            query = v_011a3eb4;
                        }
                        Err(er_0) => {
                            let er = TblExampleCmEr::TryBind {
                                try_bind: er_0,
                                loc: loc_lib::loc::Loc::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(loc_lib::loc::Occr {
                                        file: String::from(
                                            "pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs",
                                        ),
                                        line: 3002,
                                        col: 25,
                                    }),
                                ),
                            };
                            let mut res = axum::response::IntoResponse::into_response(axum::Json(
                                TblExampleCmResVrts::from_h(er),
                            ));
                            *res.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                            return res;
                        }
                    }
                }
                query
            };
            let mut pool_connection = match app_state.get_pg_pool().acquire().await {
                Ok(v_4535ee48) => v_4535ee48,
                Err(er_0) => {
                    let er = TblExampleCmEr::Pg {
                        pg: er_0,
                        loc: loc_lib::loc::Loc::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(loc_lib::loc::Occr {
                                file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                                line: 3165,
                                col: 29,
                            }),
                        ),
                    };
                    let mut res = axum::response::IntoResponse::into_response(axum::Json(
                        TblExampleCmResVrts::from_h(er),
                    ));
                    *res.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                    return res;
                }
            };
            let executor_acquire = match sqlx::Acquire::acquire(&mut pool_connection).await {
                Ok(v_61ae8f84) => v_61ae8f84,
                Err(er_0) => {
                    let er = TblExampleCmEr::Pg {
                        pg: er_0,
                        loc: loc_lib::loc::Loc::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(loc_lib::loc::Occr {
                                file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                                line: 3165,
                                col: 29,
                            }),
                        ),
                    };
                    let mut res = axum::response::IntoResponse::into_response(axum::Json(
                        TblExampleCmResVrts::from_h(er),
                    ));
                    *res.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                    return res;
                }
            };
            let v = {
                let mut executor = match sqlx::Acquire::begin(executor_acquire).await {
                    Ok(v_1aaca28f) => v_1aaca28f,
                    Err(er_0) => {
                        let er = TblExampleCmEr::Pg {
                            pg: er_0,
                            loc: loc_lib::loc::Loc::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(loc_lib::loc::Occr {
                                    file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                                    line: 2207,
                                    col: 65,
                                }),
                            ),
                        };
                        let mut res = axum::response::IntoResponse::into_response(axum::Json(
                            TblExampleCmResVrts::from_h(er),
                        ));
                        *res.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                        return res;
                    }
                };
                let v = {
                    let mut rows = binded_query.fetch(executor.as_mut());
                    let mut acc_d16ac269 = Vec::new();
                    while let Some(v_d9cc2c36) = match pg_crud::TryStreamExt::try_next(&mut rows)
                        .await
                    {
                        Ok(v_19f3d6e1) => match v_19f3d6e1 {
                            Some(v_b27d7d79) => Some(
                                match <TblExampleRdIds as sqlx::FromRow<
                                    '_,
                                    sqlx::postgres::PgRow,
                                >>::from_row(&v_b27d7d79)
                                {
                                    Ok(v_33759463) => v_33759463,
                                    Err(er_0) => {
                                        drop(rows);
                                        {
                                            if let Err(er_1) = executor.rollback().await {
                                                let er = TblExampleCmEr::RowAndRollback {
                                                    row: er_0,
                                                    rollback: er_1,
                                                    loc: loc_lib::loc::Loc::new(
                                                        file!().to_owned(),
                                                        line!(),
                                                        column!(),
                                                        Some(loc_lib::loc::Occr {
                                                            file: String::from(
                                                                "pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs",
                                                            ),
                                                            line: 3204,
                                                            col: 45,
                                                        }),
                                                    ),
                                                };
                                                let mut res =
                                                    axum::response::IntoResponse::into_response(
                                                        axum::Json(TblExampleCmResVrts::from_h(er)),
                                                    );
                                                *res.status_mut() =
                                                    http::StatusCode::INTERNAL_SERVER_ERROR;
                                                return res;
                                            }
                                            let er = TblExampleCmEr::Pg {
                                                pg: er_0,
                                                loc: loc_lib::loc::Loc::new(
                                                    file!().to_owned(),
                                                    line!(),
                                                    column!(),
                                                    Some(loc_lib::loc::Occr {
                                                        file: String::from(
                                                            "pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs",
                                                        ),
                                                        line: 3204,
                                                        col: 45,
                                                    }),
                                                ),
                                            };
                                            let mut res =
                                                axum::response::IntoResponse::into_response(
                                                    axum::Json(TblExampleCmResVrts::from_h(er)),
                                                );
                                            *res.status_mut() =
                                                http::StatusCode::INTERNAL_SERVER_ERROR;
                                            return res;
                                        }
                                    }
                                },
                            ),
                            None => None,
                        },
                        Err(er_0) => {
                            drop(rows);
                            {
                                if let Err(er_1) = executor.rollback().await {
                                    let er = TblExampleCmEr::RowAndRollback {
                                        row: er_0,
                                        rollback: er_1,
                                        loc: loc_lib::loc::Loc::new(
                                            file!().to_owned(),
                                            line!(),
                                            column!(),
                                            Some(loc_lib::loc::Occr {
                                                file: String::from(
                                                    "pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs",
                                                ),
                                                line: 3219,
                                                col: 37,
                                            }),
                                        ),
                                    };
                                    let mut res = axum::response::IntoResponse::into_response(
                                        axum::Json(TblExampleCmResVrts::from_h(er)),
                                    );
                                    *res.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                                    return res;
                                }
                                let er = TblExampleCmEr::Pg {
                                    pg: er_0,
                                    loc: loc_lib::loc::Loc::new(
                                        file!().to_owned(),
                                        line!(),
                                        column!(),
                                        Some(loc_lib::loc::Occr {
                                            file: String::from(
                                                "pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs",
                                            ),
                                            line: 3219,
                                            col: 37,
                                        }),
                                    ),
                                };
                                let mut res = axum::response::IntoResponse::into_response(
                                    axum::Json(TblExampleCmResVrts::from_h(er)),
                                );
                                *res.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                                return res;
                            }
                        }
                    } {
                        acc_d16ac269.push(v_d9cc2c36);
                    }
                    acc_d16ac269
                };
                if let Err(er_0) = executor.commit().await {
                    let er = TblExampleCmEr::Pg {
                        pg: er_0,
                        loc: loc_lib::loc::Loc::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(loc_lib::loc::Occr {
                                file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                                line: 2217,
                                col: 65,
                            }),
                        ),
                    };
                    let mut res = axum::response::IntoResponse::into_response(axum::Json(
                        TblExampleCmResVrts::from_h(er),
                    ));
                    *res.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                    return res;
                }
                v
            };
            let mut res = axum::response::IntoResponse::into_response(axum::Json(
                TblExampleCmResVrts::Desirable(v),
            ));
            *res.status_mut() = http::StatusCode::CREATED;
            res
        }
        #[allow(clippy::single_call_fn)]
        pub async fn cm(
            app_state: axum::extract::State<
                std::sync::Arc<dyn pg_crud::CombinationOfAppStateLogicTraits>,
            >,
            req: axum::extract::Request,
        ) -> axum::response::Response {
            Self::cm_h(app_state, req, Self::tbl_name()).await
        }
        #[allow(clippy::single_call_fn)]
        async fn try_cm_h(
            endpoint_loc: &str,
            prms: TblExampleCmPrms,
            tbl: &str,
        ) -> Result<Vec<TblExampleRdIds>, TblExampleTryCmEr> {
            let payload = {
                match serde_json::to_string(&prms.payload) {
                    Ok(v_1772a83e) => v_1772a83e,
                    Err(er_0) => {
                        return Err(TblExampleTryCmEr::SerdeJsonToString {
                            serde_json_to_string: er_0,
                            loc: loc_lib::loc::Loc::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(loc_lib::loc::Occr {
                                    file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                                    line: 2396,
                                    col: 81,
                                }),
                            ),
                        });
                    }
                }
            };
            let url = format!("{endpoint_loc}/{tbl}/cm");
            let future = reqwest::Client::new()
                .post(&url)
                .header(&"commit".to_owned(), git_info::PROJECT_GIT_INFO.commit)
                .header(reqwest::header::CONTENT_TYPE, "application/json")
                .body(payload)
                .send();
            let res = match future.await {
                Ok(v_180559e9) => v_180559e9,
                Err(er_0) => {
                    return Err(TblExampleTryCmEr::Reqwest {
                        reqwest: er_0,
                        loc: loc_lib::loc::Loc::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(loc_lib::loc::Occr {
                                file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                                line: 2440,
                                col: 68,
                            }),
                        ),
                    });
                }
            };
            let er_0 = res.status();
            let er_1 = res.headers().clone();
            let er_2 = match res.text().await {
                Ok(v_6a62b2b9) => v_6a62b2b9,
                Err(er_2) => {
                    return Err(TblExampleTryCmEr::FailedToGetResText {
                        status_code: er_0,
                        headers: er_1,
                        reqwest: er_2,
                        loc: loc_lib::loc::Loc::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(loc_lib::loc::Occr {
                                file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                                line: 2461,
                                col: 78,
                            }),
                        ),
                    });
                }
            };
            let expected_res = match serde_json::from_str::<TblExampleCmResVrts>(&er_2) {
                Ok(v_563d2a75) => v_563d2a75,
                Err(er_3) => {
                    return Err(TblExampleTryCmEr::DeRes {
                        status_code: er_0,
                        headers: er_1,
                        res_text: er_2,
                        serde: er_3,
                        loc: loc_lib::loc::Loc::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(loc_lib::loc::Occr {
                                file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                                line: 2472,
                                col: 63,
                            }),
                        ),
                    });
                }
            };
            let cm_er_with_serde = match expected_res {
                TblExampleCmResVrts::Desirable(v) => {
                    return Ok(v);
                }
                TblExampleCmResVrts::CheckBodySize {
                    check_body_size,
                    loc,
                } => TblExampleCmErWithSerde::CheckBodySize {
                    check_body_size,
                    loc,
                },
                TblExampleCmResVrts::Pg { pg, loc } => TblExampleCmErWithSerde::Pg { pg, loc },
                TblExampleCmResVrts::SerdeJson { serde_json, loc } => {
                    TblExampleCmErWithSerde::SerdeJson { serde_json, loc }
                }
                TblExampleCmResVrts::HeaderContentTypeAppJsonNotFound { loc } => {
                    TblExampleCmErWithSerde::HeaderContentTypeAppJsonNotFound { loc }
                }
                TblExampleCmResVrts::CheckCommit { check_commit, loc } => {
                    TblExampleCmErWithSerde::CheckCommit { check_commit, loc }
                }
                TblExampleCmResVrts::Qp { er, loc } => TblExampleCmErWithSerde::Qp { er, loc },
                TblExampleCmResVrts::RowAndRollback { row, rollback, loc } => {
                    TblExampleCmErWithSerde::RowAndRollback { row, rollback, loc }
                }
                TblExampleCmResVrts::TryBind { try_bind, loc } => {
                    TblExampleCmErWithSerde::TryBind { try_bind, loc }
                }
            };
            Err(TblExampleTryCmEr::TblExampleCmErWithSerde {
                cm_er_with_serde,
                loc: loc_lib::loc::Loc::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(loc_lib::loc::Occr {
                        file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                        line: 2512,
                        col: 83,
                    }),
                ),
            })
        }
        pub async fn try_cm(
            endpoint_loc: &str,
            prms: TblExampleCmPrms,
        ) -> Result<Vec<TblExampleRdIds>, TblExampleTryCmEr> {
            Self::try_cm_h(endpoint_loc, prms, Self::tbl_name()).await
        }
        #[must_use]
        pub fn cm_payload_example() -> axum::response::Response {
            let mut res = axum::response::IntoResponse::into_response(axum::Json(
                <TblExampleCmPayload as pg_crud::DfltSomeOneEl>::dflt_some_one_el(),
            ));
            *res.status_mut() = http::StatusCode::OK;
            res
        }
        #[allow(clippy::single_call_fn)]
        async fn co_h(
            app_state: axum::extract::State<
                std::sync::Arc<dyn pg_crud::CombinationOfAppStateLogicTraits>,
            >,
            req: axum::extract::Request,
            tbl: &str,
        ) -> axum::response::Response {
            let (parts, body) = req.into_parts();
            let headers = parts.headers;
            if !matches ! (headers . get (http :: header :: CONTENT_TYPE) , Some (v_e3f6eecd) if v_e3f6eecd == http :: header :: HeaderValue :: from_static ("application/json"))
            {
                let er = TblExampleCoEr::HeaderContentTypeAppJsonNotFound {
                    loc: loc_lib::loc::Loc::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(loc_lib::loc::Occr {
                            file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                            line: 2555,
                            col: 25,
                        }),
                    ),
                };
                let mut res = axum::response::IntoResponse::into_response(axum::Json(
                    TblExampleCoResVrts::from_h(er),
                ));
                *res.status_mut() = http::StatusCode::BAD_REQUEST;
                return res;
            }
            let body_bytes = match pg_crud::check_body_size::check_body_size(
                body,
                *app_state.get_maximum_size_of_http_body_in_bytes(),
            )
            .await
            {
                Ok(v_cfac9140) => v_cfac9140,
                Err(er_0) => {
                    let er = TblExampleCoEr::CheckBodySize {
                        check_body_size: er_0,
                        loc: loc_lib::loc::Loc::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(loc_lib::loc::Occr {
                                file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                                line: 2564,
                                col: 33,
                            }),
                        ),
                    };
                    let mut res = axum::response::IntoResponse::into_response(axum::Json(
                        TblExampleCoResVrts::from_h(er),
                    ));
                    *res.status_mut() = http::StatusCode::BAD_REQUEST;
                    return res;
                }
            };
            let prms = TblExampleCoPrms {
                payload: match serde_json::from_slice::<TblExampleCr>(&body_bytes) {
                    Ok(v_9e6fcd2d) => v_9e6fcd2d,
                    Err(er_0) => {
                        let er = TblExampleCoEr::SerdeJson {
                            serde_json: er_0,
                            loc: loc_lib::loc::Loc::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(loc_lib::loc::Occr {
                                    file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                                    line: 2609,
                                    col: 37,
                                }),
                            ),
                        };
                        let mut res = axum::response::IntoResponse::into_response(axum::Json(
                            TblExampleCoResVrts::from_h(er),
                        ));
                        *res.status_mut() = http::StatusCode::BAD_REQUEST;
                        return res;
                    }
                },
            };
            let query_string = pg_crud::gen_co_query_string(
                tbl,
                "pk_col,col_0,col_1,col_2",
                &match prms.payload.cr_qp(&mut 0) {
                    Ok(v_3267d57d) => v_3267d57d,
                    Err(er_0) => {
                        let er = TblExampleCoEr::Qp {
                            er: er_0,
                            loc: loc_lib::loc::Loc::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(loc_lib::loc::Occr {
                                    file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                                    line: 2319,
                                    col: 80,
                                }),
                            ),
                        };
                        let mut res = axum::response::IntoResponse::into_response(axum::Json(
                            TblExampleCoResVrts::from_h(er),
                        ));
                        *res.status_mut() = http::StatusCode::BAD_REQUEST;
                        return res;
                    }
                },
                &{
                    let mut acc_a35168d8 = String::new();
                    match < pg_crud :: SqlxTypesUuidUuidAsNnUuidV4InitByPg as pg_crud :: PgType > :: sel_only_ids_qp ("pk_col") { Ok (v_aa341baf) => { acc_a35168d8 . push_str (& v_aa341baf) ; } , Err (er_0) => { let er = TblExampleCoEr :: Qp { er : er_0 , loc : loc_lib :: loc :: Loc :: new (file ! () . to_owned () , line ! () , column ! () , Some (loc_lib :: loc :: Occr { file : String :: from ("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs") , line : 2319 , col : 80 , })) } ; let mut res = axum :: response :: IntoResponse :: into_response (axum :: Json (TblExampleCoResVrts :: from_h (er))) ; * res . status_mut () = http :: StatusCode :: BAD_REQUEST ; return res ; } }
                    match <pg_crud::I16AsNnInt2 as pg_crud::PgType>::sel_only_ids_qp("col_0") {
                        Ok(v_aa341baf) => {
                            acc_a35168d8.push_str(&v_aa341baf);
                        }
                        Err(er_0) => {
                            let er = TblExampleCoEr::Qp {
                                er: er_0,
                                loc: loc_lib::loc::Loc::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(loc_lib::loc::Occr {
                                        file: String::from(
                                            "pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs",
                                        ),
                                        line: 2319,
                                        col: 80,
                                    }),
                                ),
                            };
                            let mut res = axum::response::IntoResponse::into_response(axum::Json(
                                TblExampleCoResVrts::from_h(er),
                            ));
                            *res.status_mut() = http::StatusCode::BAD_REQUEST;
                            return res;
                        }
                    }
                    match <pg_crud::OptI16AsNlInt2 as pg_crud::PgType>::sel_only_ids_qp("col_1") {
                        Ok(v_aa341baf) => {
                            acc_a35168d8.push_str(&v_aa341baf);
                        }
                        Err(er_0) => {
                            let er = TblExampleCoEr::Qp {
                                er: er_0,
                                loc: loc_lib::loc::Loc::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(loc_lib::loc::Occr {
                                        file: String::from(
                                            "pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs",
                                        ),
                                        line: 2319,
                                        col: 80,
                                    }),
                                ),
                            };
                            let mut res = axum::response::IntoResponse::into_response(axum::Json(
                                TblExampleCoResVrts::from_h(er),
                            ));
                            *res.status_mut() = http::StatusCode::BAD_REQUEST;
                            return res;
                        }
                    }
                    match <pg_crud::I32AsNnInt4 as pg_crud::PgType>::sel_only_ids_qp("col_2") {
                        Ok(v_aa341baf) => {
                            acc_a35168d8.push_str(&v_aa341baf);
                        }
                        Err(er_0) => {
                            let er = TblExampleCoEr::Qp {
                                er: er_0,
                                loc: loc_lib::loc::Loc::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(loc_lib::loc::Occr {
                                        file: String::from(
                                            "pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs",
                                        ),
                                        line: 2319,
                                        col: 80,
                                    }),
                                ),
                            };
                            let mut res = axum::response::IntoResponse::into_response(axum::Json(
                                TblExampleCoResVrts::from_h(er),
                            ));
                            *res.status_mut() = http::StatusCode::BAD_REQUEST;
                            return res;
                        }
                    }
                    let _: Option<char> = acc_a35168d8.pop();
                    acc_a35168d8
                },
            );
            let binded_query = {
                let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
                match prms.payload.cr_qb(query) {
                    Ok(v_06f852cd) => {
                        query = v_06f852cd;
                    }
                    Err(er_0) => {
                        let er = TblExampleCoEr::TryBind {
                            try_bind: er_0,
                            loc: loc_lib::loc::Loc::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(loc_lib::loc::Occr {
                                    file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                                    line: 3002,
                                    col: 25,
                                }),
                            ),
                        };
                        let mut res = axum::response::IntoResponse::into_response(axum::Json(
                            TblExampleCoResVrts::from_h(er),
                        ));
                        *res.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                        return res;
                    }
                }
                query
            };
            let mut pool_connection = match app_state.get_pg_pool().acquire().await {
                Ok(v_4535ee48) => v_4535ee48,
                Err(er_0) => {
                    let er = TblExampleCoEr::Pg {
                        pg: er_0,
                        loc: loc_lib::loc::Loc::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(loc_lib::loc::Occr {
                                file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                                line: 3165,
                                col: 29,
                            }),
                        ),
                    };
                    let mut res = axum::response::IntoResponse::into_response(axum::Json(
                        TblExampleCoResVrts::from_h(er),
                    ));
                    *res.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                    return res;
                }
            };
            let executor_acquire = match sqlx::Acquire::acquire(&mut pool_connection).await {
                Ok(v_61ae8f84) => v_61ae8f84,
                Err(er_0) => {
                    let er = TblExampleCoEr::Pg {
                        pg: er_0,
                        loc: loc_lib::loc::Loc::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(loc_lib::loc::Occr {
                                file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                                line: 3165,
                                col: 29,
                            }),
                        ),
                    };
                    let mut res = axum::response::IntoResponse::into_response(axum::Json(
                        TblExampleCoResVrts::from_h(er),
                    ));
                    *res.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                    return res;
                }
            };
            let v = {
                let mut executor = match sqlx::Acquire::begin(executor_acquire).await {
                    Ok(v_1aaca28f) => v_1aaca28f,
                    Err(er_0) => {
                        let er = TblExampleCoEr::Pg {
                            pg: er_0,
                            loc: loc_lib::loc::Loc::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(loc_lib::loc::Occr {
                                    file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                                    line: 2207,
                                    col: 65,
                                }),
                            ),
                        };
                        let mut res = axum::response::IntoResponse::into_response(axum::Json(
                            TblExampleCoResVrts::from_h(er),
                        ));
                        *res.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                        return res;
                    }
                };
                let v = {
                    match binded_query.fetch_one(executor.as_mut()).await {
                        Ok(v_b27d7d79) => {
                            match < TblExampleRdIds as sqlx :: FromRow < '_ , sqlx :: postgres :: PgRow >> :: from_row (& v_b27d7d79) { Ok (v_33759463) => v_33759463 , Err (er_0) => { { if let Err (er_1) = executor . rollback () . await { let er = TblExampleCoEr :: RowAndRollback { row : er_0 , rollback : er_1 , loc : loc_lib :: loc :: Loc :: new (file ! () . to_owned () , line ! () , column ! () , Some (loc_lib :: loc :: Occr { file : String :: from ("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs") , line : 3230 , col : 37 , })) } ; let mut res = axum :: response :: IntoResponse :: into_response (axum :: Json (TblExampleCoResVrts :: from_h (er))) ; * res . status_mut () = http :: StatusCode :: INTERNAL_SERVER_ERROR ; return res ; } let er = TblExampleCoEr :: Pg { pg : er_0 , loc : loc_lib :: loc :: Loc :: new (file ! () . to_owned () , line ! () , column ! () , Some (loc_lib :: loc :: Occr { file : String :: from ("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs") , line : 3230 , col : 37 , })) } ; let mut res = axum :: response :: IntoResponse :: into_response (axum :: Json (TblExampleCoResVrts :: from_h (er))) ; * res . status_mut () = http :: StatusCode :: INTERNAL_SERVER_ERROR ; return res ; } } }
                        }
                        Err(er_0) => {
                            if let Err(er_1) = executor.rollback().await {
                                let er = TblExampleCoEr::RowAndRollback {
                                    row: er_0,
                                    rollback: er_1,
                                    loc: loc_lib::loc::Loc::new(
                                        file!().to_owned(),
                                        line!(),
                                        column!(),
                                        Some(loc_lib::loc::Occr {
                                            file: String::from(
                                                "pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs",
                                            ),
                                            line: 3230,
                                            col: 37,
                                        }),
                                    ),
                                };
                                let mut res = axum::response::IntoResponse::into_response(
                                    axum::Json(TblExampleCoResVrts::from_h(er)),
                                );
                                *res.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                                return res;
                            }
                            let er = TblExampleCoEr::Pg {
                                pg: er_0,
                                loc: loc_lib::loc::Loc::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(loc_lib::loc::Occr {
                                        file: String::from(
                                            "pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs",
                                        ),
                                        line: 3230,
                                        col: 37,
                                    }),
                                ),
                            };
                            let mut res = axum::response::IntoResponse::into_response(axum::Json(
                                TblExampleCoResVrts::from_h(er),
                            ));
                            *res.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                            return res;
                        }
                    }
                };
                if let Err(er_0) = executor.commit().await {
                    let er = TblExampleCoEr::Pg {
                        pg: er_0,
                        loc: loc_lib::loc::Loc::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(loc_lib::loc::Occr {
                                file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                                line: 2217,
                                col: 65,
                            }),
                        ),
                    };
                    let mut res = axum::response::IntoResponse::into_response(axum::Json(
                        TblExampleCoResVrts::from_h(er),
                    ));
                    *res.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                    return res;
                }
                v
            };
            let mut res = axum::response::IntoResponse::into_response(axum::Json(
                TblExampleCoResVrts::Desirable(v),
            ));
            *res.status_mut() = http::StatusCode::CREATED;
            res
        }
        #[allow(clippy::single_call_fn)]
        pub async fn co(
            app_state: axum::extract::State<
                std::sync::Arc<dyn pg_crud::CombinationOfAppStateLogicTraits>,
            >,
            req: axum::extract::Request,
        ) -> axum::response::Response {
            Self::co_h(app_state, req, Self::tbl_name()).await
        }
        #[allow(clippy::single_call_fn)]
        async fn try_co_h(
            endpoint_loc: &str,
            prms: TblExampleCoPrms,
            tbl: &str,
        ) -> Result<TblExampleRdIds, TblExampleTryCoEr> {
            let payload = {
                match serde_json::to_string(&prms.payload) {
                    Ok(v_1772a83e) => v_1772a83e,
                    Err(er_0) => {
                        return Err(TblExampleTryCoEr::SerdeJsonToString {
                            serde_json_to_string: er_0,
                            loc: loc_lib::loc::Loc::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(loc_lib::loc::Occr {
                                    file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                                    line: 2396,
                                    col: 81,
                                }),
                            ),
                        });
                    }
                }
            };
            let url = format!("{endpoint_loc}/{tbl}/co");
            let future = reqwest::Client::new()
                .post(&url)
                .header(&"commit".to_owned(), git_info::PROJECT_GIT_INFO.commit)
                .header(reqwest::header::CONTENT_TYPE, "application/json")
                .body(payload)
                .send();
            let res = match future.await {
                Ok(v_180559e9) => v_180559e9,
                Err(er_0) => {
                    return Err(TblExampleTryCoEr::Reqwest {
                        reqwest: er_0,
                        loc: loc_lib::loc::Loc::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(loc_lib::loc::Occr {
                                file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                                line: 2440,
                                col: 68,
                            }),
                        ),
                    });
                }
            };
            let er_0 = res.status();
            let er_1 = res.headers().clone();
            let er_2 = match res.text().await {
                Ok(v_6a62b2b9) => v_6a62b2b9,
                Err(er_2) => {
                    return Err(TblExampleTryCoEr::FailedToGetResText {
                        status_code: er_0,
                        headers: er_1,
                        reqwest: er_2,
                        loc: loc_lib::loc::Loc::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(loc_lib::loc::Occr {
                                file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                                line: 2461,
                                col: 78,
                            }),
                        ),
                    });
                }
            };
            let expected_res = match serde_json::from_str::<TblExampleCoResVrts>(&er_2) {
                Ok(v_563d2a75) => v_563d2a75,
                Err(er_3) => {
                    return Err(TblExampleTryCoEr::DeRes {
                        status_code: er_0,
                        headers: er_1,
                        res_text: er_2,
                        serde: er_3,
                        loc: loc_lib::loc::Loc::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(loc_lib::loc::Occr {
                                file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                                line: 2472,
                                col: 63,
                            }),
                        ),
                    });
                }
            };
            let co_er_with_serde = match expected_res {
                TblExampleCoResVrts::Desirable(v) => {
                    return Ok(v);
                }
                TblExampleCoResVrts::CheckBodySize {
                    check_body_size,
                    loc,
                } => TblExampleCoErWithSerde::CheckBodySize {
                    check_body_size,
                    loc,
                },
                TblExampleCoResVrts::Pg { pg, loc } => TblExampleCoErWithSerde::Pg { pg, loc },
                TblExampleCoResVrts::SerdeJson { serde_json, loc } => {
                    TblExampleCoErWithSerde::SerdeJson { serde_json, loc }
                }
                TblExampleCoResVrts::HeaderContentTypeAppJsonNotFound { loc } => {
                    TblExampleCoErWithSerde::HeaderContentTypeAppJsonNotFound { loc }
                }
                TblExampleCoResVrts::CheckCommit { check_commit, loc } => {
                    TblExampleCoErWithSerde::CheckCommit { check_commit, loc }
                }
                TblExampleCoResVrts::Qp { er, loc } => TblExampleCoErWithSerde::Qp { er, loc },
                TblExampleCoResVrts::RowAndRollback { row, rollback, loc } => {
                    TblExampleCoErWithSerde::RowAndRollback { row, rollback, loc }
                }
                TblExampleCoResVrts::TryBind { try_bind, loc } => {
                    TblExampleCoErWithSerde::TryBind { try_bind, loc }
                }
            };
            Err(TblExampleTryCoEr::TblExampleCoErWithSerde {
                co_er_with_serde,
                loc: loc_lib::loc::Loc::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(loc_lib::loc::Occr {
                        file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                        line: 2512,
                        col: 83,
                    }),
                ),
            })
        }
        pub async fn try_co(
            endpoint_loc: &str,
            prms: TblExampleCoPrms,
        ) -> Result<TblExampleRdIds, TblExampleTryCoEr> {
            Self::try_co_h(endpoint_loc, prms, Self::tbl_name()).await
        }
        #[must_use]
        pub fn co_payload_example() -> axum::response::Response {
            let mut res = axum::response::IntoResponse::into_response(axum::Json(
                <TblExampleCr as pg_crud::DfltSomeOneEl>::dflt_some_one_el(),
            ));
            *res.status_mut() = http::StatusCode::OK;
            res
        }
        #[allow(clippy::single_call_fn)]
        async fn rm_h(
            app_state: axum::extract::State<
                std::sync::Arc<dyn pg_crud::CombinationOfAppStateLogicTraits>,
            >,
            req: axum::extract::Request,
            tbl: &str,
        ) -> axum::response::Response {
            let (parts, body) = req.into_parts();
            let headers = parts.headers;
            if !matches ! (headers . get (http :: header :: CONTENT_TYPE) , Some (v_e3f6eecd) if v_e3f6eecd == http :: header :: HeaderValue :: from_static ("application/json"))
            {
                let er = TblExampleRmEr::HeaderContentTypeAppJsonNotFound {
                    loc: loc_lib::loc::Loc::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(loc_lib::loc::Occr {
                            file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                            line: 2555,
                            col: 25,
                        }),
                    ),
                };
                let mut res = axum::response::IntoResponse::into_response(axum::Json(
                    TblExampleRmResVrts::from_h(er),
                ));
                *res.status_mut() = http::StatusCode::BAD_REQUEST;
                return res;
            }
            let body_bytes = match pg_crud::check_body_size::check_body_size(
                body,
                *app_state.get_maximum_size_of_http_body_in_bytes(),
            )
            .await
            {
                Ok(v_cfac9140) => v_cfac9140,
                Err(er_0) => {
                    let er = TblExampleRmEr::CheckBodySize {
                        check_body_size: er_0,
                        loc: loc_lib::loc::Loc::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(loc_lib::loc::Occr {
                                file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                                line: 2564,
                                col: 33,
                            }),
                        ),
                    };
                    let mut res = axum::response::IntoResponse::into_response(axum::Json(
                        TblExampleRmResVrts::from_h(er),
                    ));
                    *res.status_mut() = http::StatusCode::BAD_REQUEST;
                    return res;
                }
            };
            let prms = TblExampleRmPrms {
                payload: match serde_json::from_slice::<TblExampleRmPayload>(&body_bytes) {
                    Ok(v_9e6fcd2d) => v_9e6fcd2d,
                    Err(er_0) => {
                        let er = TblExampleRmEr::SerdeJson {
                            serde_json: er_0,
                            loc: loc_lib::loc::Loc::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(loc_lib::loc::Occr {
                                    file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                                    line: 2609,
                                    col: 37,
                                }),
                            ),
                        };
                        let mut res = axum::response::IntoResponse::into_response(axum::Json(
                            TblExampleRmResVrts::from_h(er),
                        ));
                        *res.status_mut() = http::StatusCode::BAD_REQUEST;
                        return res;
                    }
                },
            };
            let query_string = pg_crud::gen_rm_query_string(
                tbl,
                &match Self::gen_sel_qp(&prms.payload.sel) {
                    Ok(v_357219fb) => v_357219fb,
                    Err(er_0) => {
                        let er = TblExampleRmEr::Qp {
                            er: er_0,
                            loc: loc_lib::loc::Loc::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(loc_lib::loc::Occr {
                                    file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                                    line: 911,
                                    col: 74,
                                }),
                            ),
                        };
                        let mut res = axum::response::IntoResponse::into_response(axum::Json(
                            TblExampleRmResVrts::from_h(er),
                        ));
                        *res.status_mut() = http::StatusCode::BAD_REQUEST;
                        return res;
                    }
                },
                &{
                    let mut incr: u64 = 0;
                    let mut extra_prms = match pg_crud::PgTypeWhFlt::qp(
                        &prms.payload.wh_many,
                        &mut incr,
                        &"",
                        false,
                    ) {
                        Ok(v_d1627695) => v_d1627695,
                        Err(er_0) => {
                            let er = TblExampleRmEr::Qp {
                                er: er_0,
                                loc: loc_lib::loc::Loc::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(loc_lib::loc::Occr {
                                        file: String::from(
                                            "pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs",
                                        ),
                                        line: 1300,
                                        col: 21,
                                    }),
                                ),
                            };
                            let mut res = axum::response::IntoResponse::into_response(axum::Json(
                                TblExampleRmResVrts::from_h(er),
                            ));
                            *res.status_mut() = http::StatusCode::BAD_REQUEST;
                            return res;
                        }
                    };
                    let prefix = if extra_prms.is_empty() { "" } else { " " };
                    if {
                        use std::fmt::Write as _;
                        write!(
                            extra_prms,
                            "{}order by {} {}",
                            prefix,
                            match &prms.payload.order_by.col {
                                TblExampleSel::PkCol(_) => "pk_col",
                                TblExampleSel::Col0(_) => "col_0",
                                TblExampleSel::Col1(_) => "col_1",
                                TblExampleSel::Col2(_) => "col_2",
                            },
                            prms.payload.order_by.order.as_ref().map_or_else(
                                || pg_crud::Order::default().to_sc_str(),
                                pg_crud::Order::to_sc_str
                            )
                        )
                    }
                    .is_err()
                    {
                        let er_0 = pg_crud::QpEr::WriteIntoBuffer {
                            loc: loc_lib::loc!(),
                        };
                        let er = TblExampleRmEr::Qp {
                            er: er_0,
                            loc: loc_lib::loc::Loc::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(loc_lib::loc::Occr {
                                    file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                                    line: 2319,
                                    col: 80,
                                }),
                            ),
                        };
                        let mut res = axum::response::IntoResponse::into_response(axum::Json(
                            TblExampleRmResVrts::from_h(er),
                        ));
                        *res.status_mut() = http::StatusCode::BAD_REQUEST;
                        return res;
                    }
                    if {
                        use std::fmt::Write as _;
                        write!(
                            extra_prms,
                            "{prefix}{}",
                            match pg_crud::PgTypeWhFlt::qp(
                                &prms.payload.pgn,
                                &mut incr,
                                &"",
                                bool::default()
                            ) {
                                Ok(v_742be6cf) => v_742be6cf,
                                Err(er_0) => {
                                    {
                                        let er = TblExampleRmEr::Qp {
                                            er: er_0,
                                            loc: loc_lib::loc::Loc::new(
                                                file!().to_owned(),
                                                line!(),
                                                column!(),
                                                Some(loc_lib::loc::Occr {
                                                    file: String::from(
                                                        "pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs",
                                                    ),
                                                    line: 2319,
                                                    col: 80,
                                                }),
                                            ),
                                        };
                                        let mut res = axum::response::IntoResponse::into_response(
                                            axum::Json(TblExampleRmResVrts::from_h(er)),
                                        );
                                        *res.status_mut() = http::StatusCode::BAD_REQUEST;
                                        return res;
                                    }
                                }
                            }
                        )
                    }
                    .is_err()
                    {
                        let er_0 = pg_crud::QpEr::WriteIntoBuffer {
                            loc: loc_lib::loc!(),
                        };
                        let er = TblExampleRmEr::Qp {
                            er: er_0,
                            loc: loc_lib::loc::Loc::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(loc_lib::loc::Occr {
                                    file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                                    line: 2319,
                                    col: 80,
                                }),
                            ),
                        };
                        let mut res = axum::response::IntoResponse::into_response(axum::Json(
                            TblExampleRmResVrts::from_h(er),
                        ));
                        *res.status_mut() = http::StatusCode::BAD_REQUEST;
                        return res;
                    }
                    extra_prms
                },
            );
            let binded_query = {
                let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
                match pg_crud::PgTypeWhFlt::qb(prms.payload.wh_many, query) {
                    Ok(v_03a58371) => {
                        query = v_03a58371;
                    }
                    Err(er_0) => {
                        let er = TblExampleRmEr::TryBind {
                            try_bind: er_0,
                            loc: loc_lib::loc::Loc::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(loc_lib::loc::Occr {
                                    file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                                    line: 1324,
                                    col: 68,
                                }),
                            ),
                        };
                        let mut res = axum::response::IntoResponse::into_response(axum::Json(
                            TblExampleRmResVrts::from_h(er),
                        ));
                        *res.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                        return res;
                    }
                }
                match pg_crud::PgTypeWhFlt::qb(prms.payload.pgn, query) {
                    Ok(v_9f7e487b) => {
                        query = v_9f7e487b;
                    }
                    Err(er_0) => {
                        let er = TblExampleRmEr::TryBind {
                            try_bind: er_0,
                            loc: loc_lib::loc::Loc::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(loc_lib::loc::Occr {
                                    file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                                    line: 3002,
                                    col: 25,
                                }),
                            ),
                        };
                        let mut res = axum::response::IntoResponse::into_response(axum::Json(
                            TblExampleRmResVrts::from_h(er),
                        ));
                        *res.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                        return res;
                    }
                }
                query
            };
            let mut pool_connection = match app_state.get_pg_pool().acquire().await {
                Ok(v_4535ee48) => v_4535ee48,
                Err(er_0) => {
                    let er = TblExampleRmEr::Pg {
                        pg: er_0,
                        loc: loc_lib::loc::Loc::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(loc_lib::loc::Occr {
                                file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                                line: 3165,
                                col: 29,
                            }),
                        ),
                    };
                    let mut res = axum::response::IntoResponse::into_response(axum::Json(
                        TblExampleRmResVrts::from_h(er),
                    ));
                    *res.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                    return res;
                }
            };
            let executor_acquire = match sqlx::Acquire::acquire(&mut pool_connection).await {
                Ok(v_61ae8f84) => v_61ae8f84,
                Err(er_0) => {
                    let er = TblExampleRmEr::Pg {
                        pg: er_0,
                        loc: loc_lib::loc::Loc::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(loc_lib::loc::Occr {
                                file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                                line: 3165,
                                col: 29,
                            }),
                        ),
                    };
                    let mut res = axum::response::IntoResponse::into_response(axum::Json(
                        TblExampleRmResVrts::from_h(er),
                    ));
                    *res.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                    return res;
                }
            };
            let v = {
                {
                    let mut rows = binded_query.fetch(executor_acquire.as_mut());
                    let mut acc_d16ac269 = Vec::new();
                    while let Some (v_d9cc2c36) = match pg_crud :: TryStreamExt :: try_next (& mut rows) . await { Ok (v_19f3d6e1) => match v_19f3d6e1 { Some (v_b27d7d79) => Some (match TblExampleRd :: try_from_sqlx_pg_pg_row_with_not_empty_unq_vec_tbl_example_sel (& v_b27d7d79 , & prms . payload . sel) { Ok (v_90535a1d) => v_90535a1d , Err (er_0) => { { let er = TblExampleRmEr :: Pg { pg : er_0 , loc : loc_lib :: loc :: Loc :: new (file ! () . to_owned () , line ! () , column ! () , Some (loc_lib :: loc :: Occr { file : String :: from ("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs") , line : 1353 , col : 25 , })) } ; let mut res = axum :: response :: IntoResponse :: into_response (axum :: Json (TblExampleRmResVrts :: from_h (er))) ; * res . status_mut () = http :: StatusCode :: INTERNAL_SERVER_ERROR ; return res ; } } }) , None => None , } , Err (er_0) => { let er = TblExampleRmEr :: Pg { pg : er_0 , loc : loc_lib :: loc :: Loc :: new (file ! () . to_owned () , line ! () , column ! () , Some (loc_lib :: loc :: Occr { file : String :: from ("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs") , line : 3265 , col : 37 , })) } ; let mut res = axum :: response :: IntoResponse :: into_response (axum :: Json (TblExampleRmResVrts :: from_h (er))) ; * res . status_mut () = http :: StatusCode :: INTERNAL_SERVER_ERROR ; return res ; } } { acc_d16ac269 . push (v_d9cc2c36) ; }
                    acc_d16ac269
                }
            };
            let mut res = axum::response::IntoResponse::into_response(axum::Json(
                TblExampleRmResVrts::Desirable(v),
            ));
            *res.status_mut() = http::StatusCode::OK;
            res
        }
        #[allow(clippy::single_call_fn)]
        pub async fn rm(
            app_state: axum::extract::State<
                std::sync::Arc<dyn pg_crud::CombinationOfAppStateLogicTraits>,
            >,
            req: axum::extract::Request,
        ) -> axum::response::Response {
            Self::rm_h(app_state, req, Self::tbl_name()).await
        }
        #[allow(clippy::single_call_fn)]
        async fn try_rm_h(
            endpoint_loc: &str,
            prms: TblExampleRmPrms,
            tbl: &str,
        ) -> Result<Vec<TblExampleRd>, TblExampleTryRmEr> {
            let payload = {
                match serde_json::to_string(&prms.payload) {
                    Ok(v_1772a83e) => v_1772a83e,
                    Err(er_0) => {
                        return Err(TblExampleTryRmEr::SerdeJsonToString {
                            serde_json_to_string: er_0,
                            loc: loc_lib::loc::Loc::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(loc_lib::loc::Occr {
                                    file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                                    line: 2396,
                                    col: 81,
                                }),
                            ),
                        });
                    }
                }
            };
            let url = format!("{endpoint_loc}/{tbl}/rm");
            let future = reqwest::Client::new()
                .post(&url)
                .header(&"commit".to_owned(), git_info::PROJECT_GIT_INFO.commit)
                .header(reqwest::header::CONTENT_TYPE, "application/json")
                .body(payload)
                .send();
            let res = match future.await {
                Ok(v_180559e9) => v_180559e9,
                Err(er_0) => {
                    return Err(TblExampleTryRmEr::Reqwest {
                        reqwest: er_0,
                        loc: loc_lib::loc::Loc::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(loc_lib::loc::Occr {
                                file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                                line: 2440,
                                col: 68,
                            }),
                        ),
                    });
                }
            };
            let er_0 = res.status();
            let er_1 = res.headers().clone();
            let er_2 = match res.text().await {
                Ok(v_6a62b2b9) => v_6a62b2b9,
                Err(er_2) => {
                    return Err(TblExampleTryRmEr::FailedToGetResText {
                        status_code: er_0,
                        headers: er_1,
                        reqwest: er_2,
                        loc: loc_lib::loc::Loc::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(loc_lib::loc::Occr {
                                file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                                line: 2461,
                                col: 78,
                            }),
                        ),
                    });
                }
            };
            let expected_res = match serde_json::from_str::<TblExampleRmResVrts>(&er_2) {
                Ok(v_563d2a75) => v_563d2a75,
                Err(er_3) => {
                    return Err(TblExampleTryRmEr::DeRes {
                        status_code: er_0,
                        headers: er_1,
                        res_text: er_2,
                        serde: er_3,
                        loc: loc_lib::loc::Loc::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(loc_lib::loc::Occr {
                                file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                                line: 2472,
                                col: 63,
                            }),
                        ),
                    });
                }
            };
            let rm_er_with_serde = match expected_res {
                TblExampleRmResVrts::Desirable(v) => {
                    return Ok(v);
                }
                TblExampleRmResVrts::CheckBodySize {
                    check_body_size,
                    loc,
                } => TblExampleRmErWithSerde::CheckBodySize {
                    check_body_size,
                    loc,
                },
                TblExampleRmResVrts::Pg { pg, loc } => TblExampleRmErWithSerde::Pg { pg, loc },
                TblExampleRmResVrts::SerdeJson { serde_json, loc } => {
                    TblExampleRmErWithSerde::SerdeJson { serde_json, loc }
                }
                TblExampleRmResVrts::HeaderContentTypeAppJsonNotFound { loc } => {
                    TblExampleRmErWithSerde::HeaderContentTypeAppJsonNotFound { loc }
                }
                TblExampleRmResVrts::CheckCommit { check_commit, loc } => {
                    TblExampleRmErWithSerde::CheckCommit { check_commit, loc }
                }
                TblExampleRmResVrts::NotUnqField { loc, not_unq_field } => {
                    TblExampleRmErWithSerde::NotUnqField { loc, not_unq_field }
                }
                TblExampleRmResVrts::Qp { er, loc } => TblExampleRmErWithSerde::Qp { er, loc },
                TblExampleRmResVrts::TryBind { try_bind, loc } => {
                    TblExampleRmErWithSerde::TryBind { try_bind, loc }
                }
            };
            Err(TblExampleTryRmEr::TblExampleRmErWithSerde {
                rm_er_with_serde,
                loc: loc_lib::loc::Loc::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(loc_lib::loc::Occr {
                        file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                        line: 2512,
                        col: 83,
                    }),
                ),
            })
        }
        pub async fn try_rm(
            endpoint_loc: &str,
            prms: TblExampleRmPrms,
        ) -> Result<Vec<TblExampleRd>, TblExampleTryRmEr> {
            Self::try_rm_h(endpoint_loc, prms, Self::tbl_name()).await
        }
        #[must_use]
        pub fn rm_payload_example() -> axum::response::Response {
            let mut res = axum::response::IntoResponse::into_response(axum::Json(
                <TblExampleRmPayload as pg_crud::DfltSomeOneEl>::dflt_some_one_el(),
            ));
            *res.status_mut() = http::StatusCode::OK;
            res
        }
        #[allow(clippy::single_call_fn)]
        async fn ro_h(
            app_state: axum::extract::State<
                std::sync::Arc<dyn pg_crud::CombinationOfAppStateLogicTraits>,
            >,
            req: axum::extract::Request,
            tbl: &str,
        ) -> axum::response::Response {
            let (parts, body) = req.into_parts();
            let headers = parts.headers;
            if !matches ! (headers . get (http :: header :: CONTENT_TYPE) , Some (v_e3f6eecd) if v_e3f6eecd == http :: header :: HeaderValue :: from_static ("application/json"))
            {
                let er = TblExampleRoEr::HeaderContentTypeAppJsonNotFound {
                    loc: loc_lib::loc::Loc::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(loc_lib::loc::Occr {
                            file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                            line: 2555,
                            col: 25,
                        }),
                    ),
                };
                let mut res = axum::response::IntoResponse::into_response(axum::Json(
                    TblExampleRoResVrts::from_h(er),
                ));
                *res.status_mut() = http::StatusCode::BAD_REQUEST;
                return res;
            }
            let body_bytes = match pg_crud::check_body_size::check_body_size(
                body,
                *app_state.get_maximum_size_of_http_body_in_bytes(),
            )
            .await
            {
                Ok(v_cfac9140) => v_cfac9140,
                Err(er_0) => {
                    let er = TblExampleRoEr::CheckBodySize {
                        check_body_size: er_0,
                        loc: loc_lib::loc::Loc::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(loc_lib::loc::Occr {
                                file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                                line: 2564,
                                col: 33,
                            }),
                        ),
                    };
                    let mut res = axum::response::IntoResponse::into_response(axum::Json(
                        TblExampleRoResVrts::from_h(er),
                    ));
                    *res.status_mut() = http::StatusCode::BAD_REQUEST;
                    return res;
                }
            };
            let prms = TblExampleRoPrms {
                payload: match serde_json::from_slice::<TblExampleRoPayload>(&body_bytes) {
                    Ok(v_9e6fcd2d) => v_9e6fcd2d,
                    Err(er_0) => {
                        let er = TblExampleRoEr::SerdeJson {
                            serde_json: er_0,
                            loc: loc_lib::loc::Loc::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(loc_lib::loc::Occr {
                                    file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                                    line: 2609,
                                    col: 37,
                                }),
                            ),
                        };
                        let mut res = axum::response::IntoResponse::into_response(axum::Json(
                            TblExampleRoResVrts::from_h(er),
                        ));
                        *res.status_mut() = http::StatusCode::BAD_REQUEST;
                        return res;
                    }
                },
            };
            let query_string = pg_crud::gen_ro_query_string(
                tbl,
                &match Self::gen_sel_qp(&prms.payload.sel) {
                    Ok(v_357219fb) => v_357219fb,
                    Err(er_0) => {
                        let er = TblExampleRoEr::Qp {
                            er: er_0,
                            loc: loc_lib::loc::Loc::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(loc_lib::loc::Occr {
                                    file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                                    line: 911,
                                    col: 74,
                                }),
                            ),
                        };
                        let mut res = axum::response::IntoResponse::into_response(axum::Json(
                            TblExampleRoResVrts::from_h(er),
                        ));
                        *res.status_mut() = http::StatusCode::BAD_REQUEST;
                        return res;
                    }
                },
                &match pg_crud::PgTypeWhFlt::qp(&prms.payload.pk_col, &mut 0, &Self::pk(), false) {
                    Ok(v_be9e7b7d) => v_be9e7b7d,
                    Err(er_0) => {
                        let er = TblExampleRoEr::Qp {
                            er: er_0,
                            loc: loc_lib::loc::Loc::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(loc_lib::loc::Occr {
                                    file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                                    line: 2319,
                                    col: 80,
                                }),
                            ),
                        };
                        let mut res = axum::response::IntoResponse::into_response(axum::Json(
                            TblExampleRoResVrts::from_h(er),
                        ));
                        *res.status_mut() = http::StatusCode::BAD_REQUEST;
                        return res;
                    }
                },
            );
            let binded_query = {
                let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
                match pg_crud::PgTypeWhFlt::qb(prms.payload.pk_col, query) {
                    Ok(v_80ee6983) => {
                        query = v_80ee6983;
                    }
                    Err(er_0) => {
                        let er = TblExampleRoEr::TryBind {
                            try_bind: er_0,
                            loc: loc_lib::loc::Loc::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(loc_lib::loc::Occr {
                                    file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                                    line: 3002,
                                    col: 25,
                                }),
                            ),
                        };
                        let mut res = axum::response::IntoResponse::into_response(axum::Json(
                            TblExampleRoResVrts::from_h(er),
                        ));
                        *res.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                        return res;
                    }
                }
                query
            };
            let mut pool_connection = match app_state.get_pg_pool().acquire().await {
                Ok(v_4535ee48) => v_4535ee48,
                Err(er_0) => {
                    let er = TblExampleRoEr::Pg {
                        pg: er_0,
                        loc: loc_lib::loc::Loc::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(loc_lib::loc::Occr {
                                file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                                line: 3165,
                                col: 29,
                            }),
                        ),
                    };
                    let mut res = axum::response::IntoResponse::into_response(axum::Json(
                        TblExampleRoResVrts::from_h(er),
                    ));
                    *res.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                    return res;
                }
            };
            let executor_acquire = match sqlx::Acquire::acquire(&mut pool_connection).await {
                Ok(v_61ae8f84) => v_61ae8f84,
                Err(er_0) => {
                    let er = TblExampleRoEr::Pg {
                        pg: er_0,
                        loc: loc_lib::loc::Loc::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(loc_lib::loc::Occr {
                                file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                                line: 3165,
                                col: 29,
                            }),
                        ),
                    };
                    let mut res = axum::response::IntoResponse::into_response(axum::Json(
                        TblExampleRoResVrts::from_h(er),
                    ));
                    *res.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                    return res;
                }
            };
            let v = {
                match binded_query . fetch_one (executor_acquire . as_mut ()) . await { Ok (v_b27d7d79) => { match TblExampleRd :: try_from_sqlx_pg_pg_row_with_not_empty_unq_vec_tbl_example_sel (& v_b27d7d79 , & prms . payload . sel) { Ok (v_90535a1d) => v_90535a1d , Err (er_0) => { { let er = TblExampleRoEr :: Pg { pg : er_0 , loc : loc_lib :: loc :: Loc :: new (file ! () . to_owned () , line ! () , column ! () , Some (loc_lib :: loc :: Occr { file : String :: from ("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs") , line : 1353 , col : 25 , })) } ; let mut res = axum :: response :: IntoResponse :: into_response (axum :: Json (TblExampleRoResVrts :: from_h (er))) ; * res . status_mut () = http :: StatusCode :: INTERNAL_SERVER_ERROR ; return res ; } } } } , Err (er_0) => { let er = TblExampleRoEr :: Pg { pg : er_0 , loc : loc_lib :: loc :: Loc :: new (file ! () . to_owned () , line ! () , column ! () , Some (loc_lib :: loc :: Occr { file : String :: from ("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs") , line : 3276 , col : 78 , })) } ; let mut res = axum :: response :: IntoResponse :: into_response (axum :: Json (TblExampleRoResVrts :: from_h (er))) ; * res . status_mut () = http :: StatusCode :: INTERNAL_SERVER_ERROR ; return res ; } }
            };
            let mut res = axum::response::IntoResponse::into_response(axum::Json(
                TblExampleRoResVrts::Desirable(v),
            ));
            *res.status_mut() = http::StatusCode::OK;
            res
        }
        #[allow(clippy::single_call_fn)]
        pub async fn ro(
            app_state: axum::extract::State<
                std::sync::Arc<dyn pg_crud::CombinationOfAppStateLogicTraits>,
            >,
            req: axum::extract::Request,
        ) -> axum::response::Response {
            Self::ro_h(app_state, req, Self::tbl_name()).await
        }
        #[allow(clippy::single_call_fn)]
        async fn try_ro_h(
            endpoint_loc: &str,
            prms: TblExampleRoPrms,
            tbl: &str,
        ) -> Result<TblExampleRd, TblExampleTryRoEr> {
            let payload = {
                match serde_json::to_string(&prms.payload) {
                    Ok(v_1772a83e) => v_1772a83e,
                    Err(er_0) => {
                        return Err(TblExampleTryRoEr::SerdeJsonToString {
                            serde_json_to_string: er_0,
                            loc: loc_lib::loc::Loc::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(loc_lib::loc::Occr {
                                    file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                                    line: 2396,
                                    col: 81,
                                }),
                            ),
                        });
                    }
                }
            };
            let url = format!("{endpoint_loc}/{tbl}/ro");
            let future = reqwest::Client::new()
                .post(&url)
                .header(&"commit".to_owned(), git_info::PROJECT_GIT_INFO.commit)
                .header(reqwest::header::CONTENT_TYPE, "application/json")
                .body(payload)
                .send();
            let res = match future.await {
                Ok(v_180559e9) => v_180559e9,
                Err(er_0) => {
                    return Err(TblExampleTryRoEr::Reqwest {
                        reqwest: er_0,
                        loc: loc_lib::loc::Loc::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(loc_lib::loc::Occr {
                                file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                                line: 2440,
                                col: 68,
                            }),
                        ),
                    });
                }
            };
            let er_0 = res.status();
            let er_1 = res.headers().clone();
            let er_2 = match res.text().await {
                Ok(v_6a62b2b9) => v_6a62b2b9,
                Err(er_2) => {
                    return Err(TblExampleTryRoEr::FailedToGetResText {
                        status_code: er_0,
                        headers: er_1,
                        reqwest: er_2,
                        loc: loc_lib::loc::Loc::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(loc_lib::loc::Occr {
                                file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                                line: 2461,
                                col: 78,
                            }),
                        ),
                    });
                }
            };
            let expected_res = match serde_json::from_str::<TblExampleRoResVrts>(&er_2) {
                Ok(v_563d2a75) => v_563d2a75,
                Err(er_3) => {
                    return Err(TblExampleTryRoEr::DeRes {
                        status_code: er_0,
                        headers: er_1,
                        res_text: er_2,
                        serde: er_3,
                        loc: loc_lib::loc::Loc::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(loc_lib::loc::Occr {
                                file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                                line: 2472,
                                col: 63,
                            }),
                        ),
                    });
                }
            };
            let ro_er_with_serde = match expected_res {
                TblExampleRoResVrts::Desirable(v) => {
                    return Ok(v);
                }
                TblExampleRoResVrts::CheckBodySize {
                    check_body_size,
                    loc,
                } => TblExampleRoErWithSerde::CheckBodySize {
                    check_body_size,
                    loc,
                },
                TblExampleRoResVrts::Pg { pg, loc } => TblExampleRoErWithSerde::Pg { pg, loc },
                TblExampleRoResVrts::SerdeJson { serde_json, loc } => {
                    TblExampleRoErWithSerde::SerdeJson { serde_json, loc }
                }
                TblExampleRoResVrts::HeaderContentTypeAppJsonNotFound { loc } => {
                    TblExampleRoErWithSerde::HeaderContentTypeAppJsonNotFound { loc }
                }
                TblExampleRoResVrts::CheckCommit { check_commit, loc } => {
                    TblExampleRoErWithSerde::CheckCommit { check_commit, loc }
                }
                TblExampleRoResVrts::NotUnqField { loc, not_unq_field } => {
                    TblExampleRoErWithSerde::NotUnqField { loc, not_unq_field }
                }
                TblExampleRoResVrts::Qp { er, loc } => TblExampleRoErWithSerde::Qp { er, loc },
                TblExampleRoResVrts::TryBind { try_bind, loc } => {
                    TblExampleRoErWithSerde::TryBind { try_bind, loc }
                }
            };
            Err(TblExampleTryRoEr::TblExampleRoErWithSerde {
                ro_er_with_serde,
                loc: loc_lib::loc::Loc::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(loc_lib::loc::Occr {
                        file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                        line: 2512,
                        col: 83,
                    }),
                ),
            })
        }
        pub async fn try_ro(
            endpoint_loc: &str,
            prms: TblExampleRoPrms,
        ) -> Result<TblExampleRd, TblExampleTryRoEr> {
            Self::try_ro_h(endpoint_loc, prms, Self::tbl_name()).await
        }
        #[must_use]
        pub fn ro_payload_example() -> axum::response::Response {
            let mut res = axum::response::IntoResponse::into_response(axum::Json(
                <TblExampleRoPayload as pg_crud::DfltSomeOneEl>::dflt_some_one_el(),
            ));
            *res.status_mut() = http::StatusCode::OK;
            res
        }
        #[allow(clippy::single_call_fn)]
        async fn um_h(
            app_state: axum::extract::State<
                std::sync::Arc<dyn pg_crud::CombinationOfAppStateLogicTraits>,
            >,
            req: axum::extract::Request,
            tbl: &str,
        ) -> axum::response::Response {
            let (parts, body) = req.into_parts();
            let headers = parts.headers;
            if !matches ! (headers . get (http :: header :: CONTENT_TYPE) , Some (v_e3f6eecd) if v_e3f6eecd == http :: header :: HeaderValue :: from_static ("application/json"))
            {
                let er = TblExampleUmEr::HeaderContentTypeAppJsonNotFound {
                    loc: loc_lib::loc::Loc::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(loc_lib::loc::Occr {
                            file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                            line: 2555,
                            col: 25,
                        }),
                    ),
                };
                let mut res = axum::response::IntoResponse::into_response(axum::Json(
                    TblExampleUmResVrts::from_h(er),
                ));
                *res.status_mut() = http::StatusCode::BAD_REQUEST;
                return res;
            }
            let body_bytes = match pg_crud::check_body_size::check_body_size(
                body,
                *app_state.get_maximum_size_of_http_body_in_bytes(),
            )
            .await
            {
                Ok(v_cfac9140) => v_cfac9140,
                Err(er_0) => {
                    let er = TblExampleUmEr::CheckBodySize {
                        check_body_size: er_0,
                        loc: loc_lib::loc::Loc::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(loc_lib::loc::Occr {
                                file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                                line: 2564,
                                col: 33,
                            }),
                        ),
                    };
                    let mut res = axum::response::IntoResponse::into_response(axum::Json(
                        TblExampleUmResVrts::from_h(er),
                    ));
                    *res.status_mut() = http::StatusCode::BAD_REQUEST;
                    return res;
                }
            };
            let prms = TblExampleUmPrms {
                payload: match serde_json::from_slice::<TblExampleUmPayload>(&body_bytes) {
                    Ok(v_9e6fcd2d) => v_9e6fcd2d,
                    Err(er_0) => {
                        let er = TblExampleUmEr::SerdeJson {
                            serde_json: er_0,
                            loc: loc_lib::loc::Loc::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(loc_lib::loc::Occr {
                                    file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                                    line: 2609,
                                    col: 37,
                                }),
                            ),
                        };
                        let mut res = axum::response::IntoResponse::into_response(axum::Json(
                            TblExampleUmResVrts::from_h(er),
                        ));
                        *res.status_mut() = http::StatusCode::BAD_REQUEST;
                        return res;
                    }
                },
            };
            let upd_for_query_vec = prms
                .payload
                .0
                .into_iter()
                .map(TblExampleUpdForQuery::from_h)
                .collect::<Vec<TblExampleUpdForQuery>>();
            let query_string = {
                let mut incr: u64 = 0;
                let els = {
                    let mut acc_b86a253a = String::new();
                    {
                        let mut is_col_0_upd_exist = false;
                        for el_a72f3eac in &upd_for_query_vec {
                            if el_a72f3eac.col_0.is_some() {
                                is_col_0_upd_exist = true;
                                break;
                            }
                        }
                        if is_col_0_upd_exist {
                            acc_b86a253a . push_str (& pg_crud :: gen_col_eqs_case_acc_else_col_end_comma_um_qp ("col_0" , & { let mut acc_8ad06c8c = String :: default () ; for el_a72f3eac in & upd_for_query_vec { if let Some (v_3ea04126) = & el_a72f3eac . col_0 { acc_8ad06c8c . push_str (& pg_crud :: gen_when_col_id_then_v_um_qp (Self :: pk () , & match el_a72f3eac . upd_qp_pk (& mut incr) { Ok (v_00890100) => v_00890100 , Err (er_0) => { { let er = TblExampleUmEr :: Qp { er : er_0 , loc : loc_lib :: loc :: Loc :: new (file ! () . to_owned () , line ! () , column ! () , Some (loc_lib :: loc :: Occr { file : String :: from ("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs") , line : 2319 , col : 80 , })) } ; let mut res = axum :: response :: IntoResponse :: into_response (axum :: Json (TblExampleUmResVrts :: from_h (er))) ; * res . status_mut () = http :: StatusCode :: BAD_REQUEST ; return res ; } } } , & match TblExampleUpdForQuery :: upd_qp_col_0 (v_3ea04126 , & mut incr) { Ok (v_8797585c) => v_8797585c , Err (er_0) => { { let er = TblExampleUmEr :: Qp { er : er_0 , loc : loc_lib :: loc :: Loc :: new (file ! () . to_owned () , line ! () , column ! () , Some (loc_lib :: loc :: Occr { file : String :: from ("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs") , line : 2319 , col : 80 , })) } ; let mut res = axum :: response :: IntoResponse :: into_response (axum :: Json (TblExampleUmResVrts :: from_h (er))) ; * res . status_mut () = http :: StatusCode :: BAD_REQUEST ; return res ; } } })) ; } } acc_8ad06c8c })) ;
                        }
                    }
                    {
                        let mut is_col_1_upd_exist = false;
                        for el_a72f3eac in &upd_for_query_vec {
                            if el_a72f3eac.col_1.is_some() {
                                is_col_1_upd_exist = true;
                                break;
                            }
                        }
                        if is_col_1_upd_exist {
                            acc_b86a253a . push_str (& pg_crud :: gen_col_eqs_case_acc_else_col_end_comma_um_qp ("col_1" , & { let mut acc_8ad06c8c = String :: default () ; for el_a72f3eac in & upd_for_query_vec { if let Some (v_3ea04126) = & el_a72f3eac . col_1 { acc_8ad06c8c . push_str (& pg_crud :: gen_when_col_id_then_v_um_qp (Self :: pk () , & match el_a72f3eac . upd_qp_pk (& mut incr) { Ok (v_00890100) => v_00890100 , Err (er_0) => { { let er = TblExampleUmEr :: Qp { er : er_0 , loc : loc_lib :: loc :: Loc :: new (file ! () . to_owned () , line ! () , column ! () , Some (loc_lib :: loc :: Occr { file : String :: from ("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs") , line : 2319 , col : 80 , })) } ; let mut res = axum :: response :: IntoResponse :: into_response (axum :: Json (TblExampleUmResVrts :: from_h (er))) ; * res . status_mut () = http :: StatusCode :: BAD_REQUEST ; return res ; } } } , & match TblExampleUpdForQuery :: upd_qp_col_1 (v_3ea04126 , & mut incr) { Ok (v_8797585c) => v_8797585c , Err (er_0) => { { let er = TblExampleUmEr :: Qp { er : er_0 , loc : loc_lib :: loc :: Loc :: new (file ! () . to_owned () , line ! () , column ! () , Some (loc_lib :: loc :: Occr { file : String :: from ("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs") , line : 2319 , col : 80 , })) } ; let mut res = axum :: response :: IntoResponse :: into_response (axum :: Json (TblExampleUmResVrts :: from_h (er))) ; * res . status_mut () = http :: StatusCode :: BAD_REQUEST ; return res ; } } })) ; } } acc_8ad06c8c })) ;
                        }
                    }
                    {
                        let mut is_col_2_upd_exist = false;
                        for el_a72f3eac in &upd_for_query_vec {
                            if el_a72f3eac.col_2.is_some() {
                                is_col_2_upd_exist = true;
                                break;
                            }
                        }
                        if is_col_2_upd_exist {
                            acc_b86a253a . push_str (& pg_crud :: gen_col_eqs_case_acc_else_col_end_comma_um_qp ("col_2" , & { let mut acc_8ad06c8c = String :: default () ; for el_a72f3eac in & upd_for_query_vec { if let Some (v_3ea04126) = & el_a72f3eac . col_2 { acc_8ad06c8c . push_str (& pg_crud :: gen_when_col_id_then_v_um_qp (Self :: pk () , & match el_a72f3eac . upd_qp_pk (& mut incr) { Ok (v_00890100) => v_00890100 , Err (er_0) => { { let er = TblExampleUmEr :: Qp { er : er_0 , loc : loc_lib :: loc :: Loc :: new (file ! () . to_owned () , line ! () , column ! () , Some (loc_lib :: loc :: Occr { file : String :: from ("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs") , line : 2319 , col : 80 , })) } ; let mut res = axum :: response :: IntoResponse :: into_response (axum :: Json (TblExampleUmResVrts :: from_h (er))) ; * res . status_mut () = http :: StatusCode :: BAD_REQUEST ; return res ; } } } , & match TblExampleUpdForQuery :: upd_qp_col_2 (v_3ea04126 , & mut incr) { Ok (v_8797585c) => v_8797585c , Err (er_0) => { { let er = TblExampleUmEr :: Qp { er : er_0 , loc : loc_lib :: loc :: Loc :: new (file ! () . to_owned () , line ! () , column ! () , Some (loc_lib :: loc :: Occr { file : String :: from ("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs") , line : 2319 , col : 80 , })) } ; let mut res = axum :: response :: IntoResponse :: into_response (axum :: Json (TblExampleUmResVrts :: from_h (er))) ; * res . status_mut () = http :: StatusCode :: BAD_REQUEST ; return res ; } } })) ; } } acc_8ad06c8c })) ;
                        }
                    }
                    let _: Option<char> = acc_b86a253a.pop();
                    acc_b86a253a
                };
                let pks = {
                    let mut acc_a95eb175 = String::new();
                    for el_a72f3eac in &upd_for_query_vec {
                        if { use std :: fmt :: Write as _ ; write ! (acc_a95eb175 , "{}," , match el_a72f3eac . upd_qp_pk (& mut incr) { Ok (v_f269a3b2) => v_f269a3b2 , Err (er_0) => { { let er = TblExampleUmEr :: Qp { er : er_0 , loc : loc_lib :: loc :: Loc :: new (file ! () . to_owned () , line ! () , column ! () , Some (loc_lib :: loc :: Occr { file : String :: from ("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs") , line : 1927 , col : 69 , })) } ; let mut res = axum :: response :: IntoResponse :: into_response (axum :: Json (TblExampleUmResVrts :: from_h (er))) ; * res . status_mut () = http :: StatusCode :: BAD_REQUEST ; return res ; } } }) } . is_err () { let er_0 = pg_crud :: QpEr :: WriteIntoBuffer { loc : loc_lib :: loc ! () } ; let er = TblExampleUmEr :: Qp { er : er_0 , loc : loc_lib :: loc :: Loc :: new (file ! () . to_owned () , line ! () , column ! () , Some (loc_lib :: loc :: Occr { file : String :: from ("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs") , line : 2319 , col : 80 , })) } ; let mut res = axum :: response :: IntoResponse :: into_response (axum :: Json (TblExampleUmResVrts :: from_h (er))) ; * res . status_mut () = http :: StatusCode :: BAD_REQUEST ; return res ; }
                    }
                    let _: Option<char> = acc_a95eb175.pop();
                    acc_a95eb175
                };
                let return_cols = {
                    let mut acc_fd44b0aa = String::new();
                    for el_a72f3eac in &upd_for_query_vec {
                        match el_a72f3eac.sel_only_updd_ids_qp(&mut incr) {
                            Ok(v_4f536654) => {
                                acc_fd44b0aa.push_str(&v_4f536654);
                            }
                            Err(er_0) => {
                                let er = TblExampleUmEr::Qp {
                                    er: er_0,
                                    loc: loc_lib::loc::Loc::new(
                                        file!().to_owned(),
                                        line!(),
                                        column!(),
                                        Some(loc_lib::loc::Occr {
                                            file: String::from(
                                                "pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs",
                                            ),
                                            line: 2319,
                                            col: 80,
                                        }),
                                    ),
                                };
                                let mut res = axum::response::IntoResponse::into_response(
                                    axum::Json(TblExampleUmResVrts::from_h(er)),
                                );
                                *res.status_mut() = http::StatusCode::BAD_REQUEST;
                                return res;
                            }
                        }
                    }
                    acc_fd44b0aa
                };
                pg_crud::gen_um_query_string(tbl, &els, Self::pk(), &pks, &return_cols)
            };
            let binded_query = {
                let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
                for el_a72f3eac in &upd_for_query_vec {
                    if let Some(v_2edaa480) = &el_a72f3eac.col_0 {
                        if let Err(er_981062db) = query.try_bind(el_a72f3eac.pk_col) {
                            let er_0 = er_981062db.to_string();
                            let er = TblExampleUmEr::TryBind {
                                try_bind: er_0,
                                loc: loc_lib::loc::Loc::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(loc_lib::loc::Occr {
                                        file: String::from(
                                            "pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs",
                                        ),
                                        line: 3002,
                                        col: 25,
                                    }),
                                ),
                            };
                            let mut res = axum::response::IntoResponse::into_response(axum::Json(
                                TblExampleUmResVrts::from_h(er),
                            ));
                            *res.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                            return res;
                        }
                        match <pg_crud::I16AsNnInt2 as pg_crud::PgType>::upd_qb(
                            v_2edaa480.v.clone(),
                            query,
                        ) {
                            Ok(v_600e67dc) => {
                                query = v_600e67dc;
                            }
                            Err(er_0) => {
                                let er = TblExampleUmEr::TryBind {
                                    try_bind: er_0,
                                    loc: loc_lib::loc::Loc::new(
                                        file!().to_owned(),
                                        line!(),
                                        column!(),
                                        Some(loc_lib::loc::Occr {
                                            file: String::from(
                                                "pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs",
                                            ),
                                            line: 3002,
                                            col: 25,
                                        }),
                                    ),
                                };
                                let mut res = axum::response::IntoResponse::into_response(
                                    axum::Json(TblExampleUmResVrts::from_h(er)),
                                );
                                *res.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                                return res;
                            }
                        }
                    }
                }
                for el_a72f3eac in &upd_for_query_vec {
                    if let Some(v_2edaa480) = &el_a72f3eac.col_1 {
                        if let Err(er_981062db) = query.try_bind(el_a72f3eac.pk_col) {
                            let er_0 = er_981062db.to_string();
                            let er = TblExampleUmEr::TryBind {
                                try_bind: er_0,
                                loc: loc_lib::loc::Loc::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(loc_lib::loc::Occr {
                                        file: String::from(
                                            "pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs",
                                        ),
                                        line: 3002,
                                        col: 25,
                                    }),
                                ),
                            };
                            let mut res = axum::response::IntoResponse::into_response(axum::Json(
                                TblExampleUmResVrts::from_h(er),
                            ));
                            *res.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                            return res;
                        }
                        match <pg_crud::OptI16AsNlInt2 as pg_crud::PgType>::upd_qb(
                            v_2edaa480.v.clone(),
                            query,
                        ) {
                            Ok(v_600e67dc) => {
                                query = v_600e67dc;
                            }
                            Err(er_0) => {
                                let er = TblExampleUmEr::TryBind {
                                    try_bind: er_0,
                                    loc: loc_lib::loc::Loc::new(
                                        file!().to_owned(),
                                        line!(),
                                        column!(),
                                        Some(loc_lib::loc::Occr {
                                            file: String::from(
                                                "pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs",
                                            ),
                                            line: 3002,
                                            col: 25,
                                        }),
                                    ),
                                };
                                let mut res = axum::response::IntoResponse::into_response(
                                    axum::Json(TblExampleUmResVrts::from_h(er)),
                                );
                                *res.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                                return res;
                            }
                        }
                    }
                }
                for el_a72f3eac in &upd_for_query_vec {
                    if let Some(v_2edaa480) = &el_a72f3eac.col_2 {
                        if let Err(er_981062db) = query.try_bind(el_a72f3eac.pk_col) {
                            let er_0 = er_981062db.to_string();
                            let er = TblExampleUmEr::TryBind {
                                try_bind: er_0,
                                loc: loc_lib::loc::Loc::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(loc_lib::loc::Occr {
                                        file: String::from(
                                            "pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs",
                                        ),
                                        line: 3002,
                                        col: 25,
                                    }),
                                ),
                            };
                            let mut res = axum::response::IntoResponse::into_response(axum::Json(
                                TblExampleUmResVrts::from_h(er),
                            ));
                            *res.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                            return res;
                        }
                        match <pg_crud::I32AsNnInt4 as pg_crud::PgType>::upd_qb(
                            v_2edaa480.v.clone(),
                            query,
                        ) {
                            Ok(v_600e67dc) => {
                                query = v_600e67dc;
                            }
                            Err(er_0) => {
                                let er = TblExampleUmEr::TryBind {
                                    try_bind: er_0,
                                    loc: loc_lib::loc::Loc::new(
                                        file!().to_owned(),
                                        line!(),
                                        column!(),
                                        Some(loc_lib::loc::Occr {
                                            file: String::from(
                                                "pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs",
                                            ),
                                            line: 3002,
                                            col: 25,
                                        }),
                                    ),
                                };
                                let mut res = axum::response::IntoResponse::into_response(
                                    axum::Json(TblExampleUmResVrts::from_h(er)),
                                );
                                *res.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                                return res;
                            }
                        }
                    }
                }
                for el_a72f3eac in &upd_for_query_vec {
                    match <pg_crud::SqlxTypesUuidUuidAsNnUuidV4InitByPg as pg_crud::PgType>::upd_qb(
                        el_a72f3eac.pk_col,
                        query,
                    ) {
                        Ok(v_c40a4522) => {
                            query = v_c40a4522;
                        }
                        Err(er_0) => {
                            let er = TblExampleUmEr::TryBind {
                                try_bind: er_0,
                                loc: loc_lib::loc::Loc::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(loc_lib::loc::Occr {
                                        file: String::from(
                                            "pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs",
                                        ),
                                        line: 3002,
                                        col: 25,
                                    }),
                                ),
                            };
                            let mut res = axum::response::IntoResponse::into_response(axum::Json(
                                TblExampleUmResVrts::from_h(er),
                            ));
                            *res.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                            return res;
                        }
                    }
                }
                for el_a72f3eac in &upd_for_query_vec {
                    if let Some(v_47030ac2) = &el_a72f3eac.col_0 {
                        match <pg_crud::I16AsNnInt2 as pg_crud::PgType>::sel_only_updd_ids_qb(
                            &v_47030ac2.v,
                            query,
                        ) {
                            Ok(v_c5b79b95) => {
                                query = v_c5b79b95;
                            }
                            Err(er_0) => {
                                let er = TblExampleUmEr::TryBind {
                                    try_bind: er_0,
                                    loc: loc_lib::loc::Loc::new(
                                        file!().to_owned(),
                                        line!(),
                                        column!(),
                                        Some(loc_lib::loc::Occr {
                                            file: String::from(
                                                "pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs",
                                            ),
                                            line: 3002,
                                            col: 25,
                                        }),
                                    ),
                                };
                                let mut res = axum::response::IntoResponse::into_response(
                                    axum::Json(TblExampleUmResVrts::from_h(er)),
                                );
                                *res.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                                return res;
                            }
                        }
                    }
                }
                for el_a72f3eac in &upd_for_query_vec {
                    if let Some(v_47030ac2) = &el_a72f3eac.col_1 {
                        match <pg_crud::OptI16AsNlInt2 as pg_crud::PgType>::sel_only_updd_ids_qb(
                            &v_47030ac2.v,
                            query,
                        ) {
                            Ok(v_c5b79b95) => {
                                query = v_c5b79b95;
                            }
                            Err(er_0) => {
                                let er = TblExampleUmEr::TryBind {
                                    try_bind: er_0,
                                    loc: loc_lib::loc::Loc::new(
                                        file!().to_owned(),
                                        line!(),
                                        column!(),
                                        Some(loc_lib::loc::Occr {
                                            file: String::from(
                                                "pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs",
                                            ),
                                            line: 3002,
                                            col: 25,
                                        }),
                                    ),
                                };
                                let mut res = axum::response::IntoResponse::into_response(
                                    axum::Json(TblExampleUmResVrts::from_h(er)),
                                );
                                *res.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                                return res;
                            }
                        }
                    }
                }
                for el_a72f3eac in &upd_for_query_vec {
                    if let Some(v_47030ac2) = &el_a72f3eac.col_2 {
                        match <pg_crud::I32AsNnInt4 as pg_crud::PgType>::sel_only_updd_ids_qb(
                            &v_47030ac2.v,
                            query,
                        ) {
                            Ok(v_c5b79b95) => {
                                query = v_c5b79b95;
                            }
                            Err(er_0) => {
                                let er = TblExampleUmEr::TryBind {
                                    try_bind: er_0,
                                    loc: loc_lib::loc::Loc::new(
                                        file!().to_owned(),
                                        line!(),
                                        column!(),
                                        Some(loc_lib::loc::Occr {
                                            file: String::from(
                                                "pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs",
                                            ),
                                            line: 3002,
                                            col: 25,
                                        }),
                                    ),
                                };
                                let mut res = axum::response::IntoResponse::into_response(
                                    axum::Json(TblExampleUmResVrts::from_h(er)),
                                );
                                *res.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                                return res;
                            }
                        }
                    }
                }
                query
            };
            let mut pool_connection = match app_state.get_pg_pool().acquire().await {
                Ok(v_4535ee48) => v_4535ee48,
                Err(er_0) => {
                    let er = TblExampleUmEr::Pg {
                        pg: er_0,
                        loc: loc_lib::loc::Loc::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(loc_lib::loc::Occr {
                                file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                                line: 3165,
                                col: 29,
                            }),
                        ),
                    };
                    let mut res = axum::response::IntoResponse::into_response(axum::Json(
                        TblExampleUmResVrts::from_h(er),
                    ));
                    *res.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                    return res;
                }
            };
            let executor_acquire = match sqlx::Acquire::acquire(&mut pool_connection).await {
                Ok(v_61ae8f84) => v_61ae8f84,
                Err(er_0) => {
                    let er = TblExampleUmEr::Pg {
                        pg: er_0,
                        loc: loc_lib::loc::Loc::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(loc_lib::loc::Occr {
                                file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                                line: 3165,
                                col: 29,
                            }),
                        ),
                    };
                    let mut res = axum::response::IntoResponse::into_response(axum::Json(
                        TblExampleUmResVrts::from_h(er),
                    ));
                    *res.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                    return res;
                }
            };
            let v = {
                let mut executor = match sqlx::Acquire::begin(executor_acquire).await {
                    Ok(v_1aaca28f) => v_1aaca28f,
                    Err(er_0) => {
                        let er = TblExampleUmEr::Pg {
                            pg: er_0,
                            loc: loc_lib::loc::Loc::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(loc_lib::loc::Occr {
                                    file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                                    line: 2207,
                                    col: 65,
                                }),
                            ),
                        };
                        let mut res = axum::response::IntoResponse::into_response(axum::Json(
                            TblExampleUmResVrts::from_h(er),
                        ));
                        *res.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                        return res;
                    }
                };
                let v = {
                    let mut rows = binded_query.fetch(executor.as_mut());
                    let mut acc_d16ac269 = Vec::new();
                    while let Some(v_d9cc2c36) = match pg_crud::TryStreamExt::try_next(&mut rows)
                        .await
                    {
                        Ok(v_19f3d6e1) => match v_19f3d6e1 {
                            Some(v_b27d7d79) => Some(
                                match <TblExampleRdIds as sqlx::FromRow<
                                    '_,
                                    sqlx::postgres::PgRow,
                                >>::from_row(&v_b27d7d79)
                                {
                                    Ok(v_33759463) => v_33759463,
                                    Err(er_0) => {
                                        drop(rows);
                                        {
                                            if let Err(er_1) = executor.rollback().await {
                                                let er = TblExampleUmEr::RowAndRollback {
                                                    row: er_0,
                                                    rollback: er_1,
                                                    loc: loc_lib::loc::Loc::new(
                                                        file!().to_owned(),
                                                        line!(),
                                                        column!(),
                                                        Some(loc_lib::loc::Occr {
                                                            file: String::from(
                                                                "pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs",
                                                            ),
                                                            line: 3204,
                                                            col: 45,
                                                        }),
                                                    ),
                                                };
                                                let mut res =
                                                    axum::response::IntoResponse::into_response(
                                                        axum::Json(TblExampleUmResVrts::from_h(er)),
                                                    );
                                                *res.status_mut() =
                                                    http::StatusCode::INTERNAL_SERVER_ERROR;
                                                return res;
                                            }
                                            let er = TblExampleUmEr::Pg {
                                                pg: er_0,
                                                loc: loc_lib::loc::Loc::new(
                                                    file!().to_owned(),
                                                    line!(),
                                                    column!(),
                                                    Some(loc_lib::loc::Occr {
                                                        file: String::from(
                                                            "pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs",
                                                        ),
                                                        line: 3204,
                                                        col: 45,
                                                    }),
                                                ),
                                            };
                                            let mut res =
                                                axum::response::IntoResponse::into_response(
                                                    axum::Json(TblExampleUmResVrts::from_h(er)),
                                                );
                                            *res.status_mut() =
                                                http::StatusCode::INTERNAL_SERVER_ERROR;
                                            return res;
                                        }
                                    }
                                },
                            ),
                            None => None,
                        },
                        Err(er_0) => {
                            drop(rows);
                            {
                                if let Err(er_1) = executor.rollback().await {
                                    let er = TblExampleUmEr::RowAndRollback {
                                        row: er_0,
                                        rollback: er_1,
                                        loc: loc_lib::loc::Loc::new(
                                            file!().to_owned(),
                                            line!(),
                                            column!(),
                                            Some(loc_lib::loc::Occr {
                                                file: String::from(
                                                    "pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs",
                                                ),
                                                line: 3219,
                                                col: 37,
                                            }),
                                        ),
                                    };
                                    let mut res = axum::response::IntoResponse::into_response(
                                        axum::Json(TblExampleUmResVrts::from_h(er)),
                                    );
                                    *res.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                                    return res;
                                }
                                let er = TblExampleUmEr::Pg {
                                    pg: er_0,
                                    loc: loc_lib::loc::Loc::new(
                                        file!().to_owned(),
                                        line!(),
                                        column!(),
                                        Some(loc_lib::loc::Occr {
                                            file: String::from(
                                                "pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs",
                                            ),
                                            line: 3219,
                                            col: 37,
                                        }),
                                    ),
                                };
                                let mut res = axum::response::IntoResponse::into_response(
                                    axum::Json(TblExampleUmResVrts::from_h(er)),
                                );
                                *res.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                                return res;
                            }
                        }
                    } {
                        acc_d16ac269.push(v_d9cc2c36);
                    }
                    acc_d16ac269
                };
                if let Err(er_0) = executor.commit().await {
                    let er = TblExampleUmEr::Pg {
                        pg: er_0,
                        loc: loc_lib::loc::Loc::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(loc_lib::loc::Occr {
                                file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                                line: 2217,
                                col: 65,
                            }),
                        ),
                    };
                    let mut res = axum::response::IntoResponse::into_response(axum::Json(
                        TblExampleUmResVrts::from_h(er),
                    ));
                    *res.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                    return res;
                }
                v
            };
            let mut res = axum::response::IntoResponse::into_response(axum::Json(
                TblExampleUmResVrts::Desirable(v),
            ));
            *res.status_mut() = http::StatusCode::OK;
            res
        }
        #[allow(clippy::single_call_fn)]
        pub async fn um(
            app_state: axum::extract::State<
                std::sync::Arc<dyn pg_crud::CombinationOfAppStateLogicTraits>,
            >,
            req: axum::extract::Request,
        ) -> axum::response::Response {
            Self::um_h(app_state, req, Self::tbl_name()).await
        }
        #[allow(clippy::single_call_fn)]
        async fn try_um_h(
            endpoint_loc: &str,
            prms: TblExampleUmPrms,
            tbl: &str,
        ) -> Result<Vec<TblExampleRdIds>, TblExampleTryUmEr> {
            let payload = {
                match serde_json::to_string(&prms.payload) {
                    Ok(v_1772a83e) => v_1772a83e,
                    Err(er_0) => {
                        return Err(TblExampleTryUmEr::SerdeJsonToString {
                            serde_json_to_string: er_0,
                            loc: loc_lib::loc::Loc::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(loc_lib::loc::Occr {
                                    file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                                    line: 2396,
                                    col: 81,
                                }),
                            ),
                        });
                    }
                }
            };
            let url = format!("{endpoint_loc}/{tbl}/um");
            let future = reqwest::Client::new()
                .patch(&url)
                .header(&"commit".to_owned(), git_info::PROJECT_GIT_INFO.commit)
                .header(reqwest::header::CONTENT_TYPE, "application/json")
                .body(payload)
                .send();
            let res = match future.await {
                Ok(v_180559e9) => v_180559e9,
                Err(er_0) => {
                    return Err(TblExampleTryUmEr::Reqwest {
                        reqwest: er_0,
                        loc: loc_lib::loc::Loc::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(loc_lib::loc::Occr {
                                file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                                line: 2440,
                                col: 68,
                            }),
                        ),
                    });
                }
            };
            let er_0 = res.status();
            let er_1 = res.headers().clone();
            let er_2 = match res.text().await {
                Ok(v_6a62b2b9) => v_6a62b2b9,
                Err(er_2) => {
                    return Err(TblExampleTryUmEr::FailedToGetResText {
                        status_code: er_0,
                        headers: er_1,
                        reqwest: er_2,
                        loc: loc_lib::loc::Loc::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(loc_lib::loc::Occr {
                                file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                                line: 2461,
                                col: 78,
                            }),
                        ),
                    });
                }
            };
            let expected_res = match serde_json::from_str::<TblExampleUmResVrts>(&er_2) {
                Ok(v_563d2a75) => v_563d2a75,
                Err(er_3) => {
                    return Err(TblExampleTryUmEr::DeRes {
                        status_code: er_0,
                        headers: er_1,
                        res_text: er_2,
                        serde: er_3,
                        loc: loc_lib::loc::Loc::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(loc_lib::loc::Occr {
                                file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                                line: 2472,
                                col: 63,
                            }),
                        ),
                    });
                }
            };
            let um_er_with_serde = match expected_res {
                TblExampleUmResVrts::Desirable(v) => {
                    return Ok(v);
                }
                TblExampleUmResVrts::CheckBodySize {
                    check_body_size,
                    loc,
                } => TblExampleUmErWithSerde::CheckBodySize {
                    check_body_size,
                    loc,
                },
                TblExampleUmResVrts::Pg { pg, loc } => TblExampleUmErWithSerde::Pg { pg, loc },
                TblExampleUmResVrts::SerdeJson { serde_json, loc } => {
                    TblExampleUmErWithSerde::SerdeJson { serde_json, loc }
                }
                TblExampleUmResVrts::HeaderContentTypeAppJsonNotFound { loc } => {
                    TblExampleUmErWithSerde::HeaderContentTypeAppJsonNotFound { loc }
                }
                TblExampleUmResVrts::CheckCommit { check_commit, loc } => {
                    TblExampleUmErWithSerde::CheckCommit { check_commit, loc }
                }
                TblExampleUmResVrts::Qp { er, loc } => TblExampleUmErWithSerde::Qp { er, loc },
                TblExampleUmResVrts::RowAndRollback { row, rollback, loc } => {
                    TblExampleUmErWithSerde::RowAndRollback { row, rollback, loc }
                }
                TblExampleUmResVrts::TryBind { try_bind, loc } => {
                    TblExampleUmErWithSerde::TryBind { try_bind, loc }
                }
            };
            Err(TblExampleTryUmEr::TblExampleUmErWithSerde {
                um_er_with_serde,
                loc: loc_lib::loc::Loc::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(loc_lib::loc::Occr {
                        file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                        line: 2512,
                        col: 83,
                    }),
                ),
            })
        }
        pub async fn try_um(
            endpoint_loc: &str,
            prms: TblExampleUmPrms,
        ) -> Result<Vec<TblExampleRdIds>, TblExampleTryUmEr> {
            Self::try_um_h(endpoint_loc, prms, Self::tbl_name()).await
        }
        #[must_use]
        pub fn um_payload_example() -> axum::response::Response {
            let mut res = axum::response::IntoResponse::into_response(axum::Json(
                <TblExampleUmPayload as pg_crud::DfltSomeOneEl>::dflt_some_one_el(),
            ));
            *res.status_mut() = http::StatusCode::OK;
            res
        }
        #[allow(clippy::single_call_fn)]
        async fn uo_h(
            app_state: axum::extract::State<
                std::sync::Arc<dyn pg_crud::CombinationOfAppStateLogicTraits>,
            >,
            req: axum::extract::Request,
            tbl: &str,
        ) -> axum::response::Response {
            let (parts, body) = req.into_parts();
            let headers = parts.headers;
            if !matches ! (headers . get (http :: header :: CONTENT_TYPE) , Some (v_e3f6eecd) if v_e3f6eecd == http :: header :: HeaderValue :: from_static ("application/json"))
            {
                let er = TblExampleUoEr::HeaderContentTypeAppJsonNotFound {
                    loc: loc_lib::loc::Loc::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(loc_lib::loc::Occr {
                            file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                            line: 2555,
                            col: 25,
                        }),
                    ),
                };
                let mut res = axum::response::IntoResponse::into_response(axum::Json(
                    TblExampleUoResVrts::from_h(er),
                ));
                *res.status_mut() = http::StatusCode::BAD_REQUEST;
                return res;
            }
            let body_bytes = match pg_crud::check_body_size::check_body_size(
                body,
                *app_state.get_maximum_size_of_http_body_in_bytes(),
            )
            .await
            {
                Ok(v_cfac9140) => v_cfac9140,
                Err(er_0) => {
                    let er = TblExampleUoEr::CheckBodySize {
                        check_body_size: er_0,
                        loc: loc_lib::loc::Loc::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(loc_lib::loc::Occr {
                                file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                                line: 2564,
                                col: 33,
                            }),
                        ),
                    };
                    let mut res = axum::response::IntoResponse::into_response(axum::Json(
                        TblExampleUoResVrts::from_h(er),
                    ));
                    *res.status_mut() = http::StatusCode::BAD_REQUEST;
                    return res;
                }
            };
            let prms = TblExampleUoPrms {
                payload: match serde_json::from_slice::<TblExampleUpd>(&body_bytes) {
                    Ok(v_9e6fcd2d) => v_9e6fcd2d,
                    Err(er_0) => {
                        let er = TblExampleUoEr::SerdeJson {
                            serde_json: er_0,
                            loc: loc_lib::loc::Loc::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(loc_lib::loc::Occr {
                                    file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                                    line: 2609,
                                    col: 37,
                                }),
                            ),
                        };
                        let mut res = axum::response::IntoResponse::into_response(axum::Json(
                            TblExampleUoResVrts::from_h(er),
                        ));
                        *res.status_mut() = http::StatusCode::BAD_REQUEST;
                        return res;
                    }
                },
            };
            let upd_for_query = TblExampleUpdForQuery::from_h(prms.payload);
            let query_string = {
                let mut incr: u64 = 0;
                let cols = {
                    let mut acc_683e37b8 = String::new();
                    if let Some(v_2d144436) = &upd_for_query.col_0 {
                        acc_683e37b8.push_str(&pg_crud::gen_col_queals_v_comma_uo_qp(
                            "col_0",
                            &match TblExampleUpdForQuery::upd_qp_col_0(v_2d144436, &mut incr) {
                                Ok(v_1ec12051) => v_1ec12051,
                                Err(er_0) => {
                                    let er = TblExampleUoEr::Qp {
                                        er: er_0,
                                        loc: loc_lib::loc::Loc::new(
                                            file!().to_owned(),
                                            line!(),
                                            column!(),
                                            Some(loc_lib::loc::Occr {
                                                file: String::from(
                                                    "pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs",
                                                ),
                                                line: 2319,
                                                col: 80,
                                            }),
                                        ),
                                    };
                                    let mut res = axum::response::IntoResponse::into_response(
                                        axum::Json(TblExampleUoResVrts::from_h(er)),
                                    );
                                    *res.status_mut() = http::StatusCode::BAD_REQUEST;
                                    return res;
                                }
                            },
                        ));
                    }
                    if let Some(v_2d144436) = &upd_for_query.col_1 {
                        acc_683e37b8.push_str(&pg_crud::gen_col_queals_v_comma_uo_qp(
                            "col_1",
                            &match TblExampleUpdForQuery::upd_qp_col_1(v_2d144436, &mut incr) {
                                Ok(v_1ec12051) => v_1ec12051,
                                Err(er_0) => {
                                    let er = TblExampleUoEr::Qp {
                                        er: er_0,
                                        loc: loc_lib::loc::Loc::new(
                                            file!().to_owned(),
                                            line!(),
                                            column!(),
                                            Some(loc_lib::loc::Occr {
                                                file: String::from(
                                                    "pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs",
                                                ),
                                                line: 2319,
                                                col: 80,
                                            }),
                                        ),
                                    };
                                    let mut res = axum::response::IntoResponse::into_response(
                                        axum::Json(TblExampleUoResVrts::from_h(er)),
                                    );
                                    *res.status_mut() = http::StatusCode::BAD_REQUEST;
                                    return res;
                                }
                            },
                        ));
                    }
                    if let Some(v_2d144436) = &upd_for_query.col_2 {
                        acc_683e37b8.push_str(&pg_crud::gen_col_queals_v_comma_uo_qp(
                            "col_2",
                            &match TblExampleUpdForQuery::upd_qp_col_2(v_2d144436, &mut incr) {
                                Ok(v_1ec12051) => v_1ec12051,
                                Err(er_0) => {
                                    let er = TblExampleUoEr::Qp {
                                        er: er_0,
                                        loc: loc_lib::loc::Loc::new(
                                            file!().to_owned(),
                                            line!(),
                                            column!(),
                                            Some(loc_lib::loc::Occr {
                                                file: String::from(
                                                    "pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs",
                                                ),
                                                line: 2319,
                                                col: 80,
                                            }),
                                        ),
                                    };
                                    let mut res = axum::response::IntoResponse::into_response(
                                        axum::Json(TblExampleUoResVrts::from_h(er)),
                                    );
                                    *res.status_mut() = http::StatusCode::BAD_REQUEST;
                                    return res;
                                }
                            },
                        ));
                    }
                    let _: Option<char> = acc_683e37b8.pop();
                    acc_683e37b8
                };
                let pk_qp = match upd_for_query.upd_qp_pk(&mut incr) {
                    Ok(v_f269a3b2) => v_f269a3b2,
                    Err(er_0) => {
                        let er = TblExampleUoEr::Qp {
                            er: er_0,
                            loc: loc_lib::loc::Loc::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(loc_lib::loc::Occr {
                                    file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                                    line: 1927,
                                    col: 69,
                                }),
                            ),
                        };
                        let mut res = axum::response::IntoResponse::into_response(axum::Json(
                            TblExampleUoResVrts::from_h(er),
                        ));
                        *res.status_mut() = http::StatusCode::BAD_REQUEST;
                        return res;
                    }
                };
                let return_cols = match upd_for_query.sel_only_updd_ids_qp(&mut incr) {
                    Ok(v_7f0d86a1) => v_7f0d86a1,
                    Err(er_0) => {
                        let er = TblExampleUoEr::Qp {
                            er: er_0,
                            loc: loc_lib::loc::Loc::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(loc_lib::loc::Occr {
                                    file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                                    line: 2319,
                                    col: 80,
                                }),
                            ),
                        };
                        let mut res = axum::response::IntoResponse::into_response(axum::Json(
                            TblExampleUoResVrts::from_h(er),
                        ));
                        *res.status_mut() = http::StatusCode::BAD_REQUEST;
                        return res;
                    }
                };
                pg_crud::gen_uo_query_string(tbl, &cols, Self::pk(), &pk_qp, &return_cols)
            };
            let binded_query = {
                let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
                if let Some(v_ed87c152) = &upd_for_query.col_0 {
                    match <pg_crud::I16AsNnInt2 as pg_crud::PgType>::upd_qb(
                        v_ed87c152.v.clone(),
                        query,
                    ) {
                        Ok(v_result) => {
                            query = v_result;
                        }
                        Err(er_0) => {
                            let er = TblExampleUoEr::TryBind {
                                try_bind: er_0,
                                loc: loc_lib::loc::Loc::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(loc_lib::loc::Occr {
                                        file: String::from(
                                            "pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs",
                                        ),
                                        line: 3002,
                                        col: 25,
                                    }),
                                ),
                            };
                            let mut res = axum::response::IntoResponse::into_response(axum::Json(
                                TblExampleUoResVrts::from_h(er),
                            ));
                            *res.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                            return res;
                        }
                    }
                }
                if let Some(v_ed87c152) = &upd_for_query.col_1 {
                    match <pg_crud::OptI16AsNlInt2 as pg_crud::PgType>::upd_qb(
                        v_ed87c152.v.clone(),
                        query,
                    ) {
                        Ok(v_result) => {
                            query = v_result;
                        }
                        Err(er_0) => {
                            let er = TblExampleUoEr::TryBind {
                                try_bind: er_0,
                                loc: loc_lib::loc::Loc::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(loc_lib::loc::Occr {
                                        file: String::from(
                                            "pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs",
                                        ),
                                        line: 3002,
                                        col: 25,
                                    }),
                                ),
                            };
                            let mut res = axum::response::IntoResponse::into_response(axum::Json(
                                TblExampleUoResVrts::from_h(er),
                            ));
                            *res.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                            return res;
                        }
                    }
                }
                if let Some(v_ed87c152) = &upd_for_query.col_2 {
                    match <pg_crud::I32AsNnInt4 as pg_crud::PgType>::upd_qb(
                        v_ed87c152.v.clone(),
                        query,
                    ) {
                        Ok(v_result) => {
                            query = v_result;
                        }
                        Err(er_0) => {
                            let er = TblExampleUoEr::TryBind {
                                try_bind: er_0,
                                loc: loc_lib::loc::Loc::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(loc_lib::loc::Occr {
                                        file: String::from(
                                            "pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs",
                                        ),
                                        line: 3002,
                                        col: 25,
                                    }),
                                ),
                            };
                            let mut res = axum::response::IntoResponse::into_response(axum::Json(
                                TblExampleUoResVrts::from_h(er),
                            ));
                            *res.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                            return res;
                        }
                    }
                }
                match <pg_crud::SqlxTypesUuidUuidAsNnUuidV4InitByPg as pg_crud::PgType>::upd_qb(
                    upd_for_query.pk_col,
                    query,
                ) {
                    Ok(v_d64bac39) => {
                        query = v_d64bac39;
                    }
                    Err(er_0) => {
                        let er = TblExampleUoEr::TryBind {
                            try_bind: er_0,
                            loc: loc_lib::loc::Loc::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(loc_lib::loc::Occr {
                                    file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                                    line: 3002,
                                    col: 25,
                                }),
                            ),
                        };
                        let mut res = axum::response::IntoResponse::into_response(axum::Json(
                            TblExampleUoResVrts::from_h(er),
                        ));
                        *res.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                        return res;
                    }
                }
                if let Some(v_b2902425) = &upd_for_query.col_0 {
                    match <pg_crud::I16AsNnInt2 as pg_crud::PgType>::sel_only_updd_ids_qb(
                        &v_b2902425.v,
                        query,
                    ) {
                        Ok(v_result) => {
                            query = v_result;
                        }
                        Err(er_0) => {
                            let er = TblExampleUoEr::TryBind {
                                try_bind: er_0,
                                loc: loc_lib::loc::Loc::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(loc_lib::loc::Occr {
                                        file: String::from(
                                            "pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs",
                                        ),
                                        line: 3002,
                                        col: 25,
                                    }),
                                ),
                            };
                            let mut res = axum::response::IntoResponse::into_response(axum::Json(
                                TblExampleUoResVrts::from_h(er),
                            ));
                            *res.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                            return res;
                        }
                    }
                }
                if let Some(v_b2902425) = &upd_for_query.col_1 {
                    match <pg_crud::OptI16AsNlInt2 as pg_crud::PgType>::sel_only_updd_ids_qb(
                        &v_b2902425.v,
                        query,
                    ) {
                        Ok(v_result) => {
                            query = v_result;
                        }
                        Err(er_0) => {
                            let er = TblExampleUoEr::TryBind {
                                try_bind: er_0,
                                loc: loc_lib::loc::Loc::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(loc_lib::loc::Occr {
                                        file: String::from(
                                            "pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs",
                                        ),
                                        line: 3002,
                                        col: 25,
                                    }),
                                ),
                            };
                            let mut res = axum::response::IntoResponse::into_response(axum::Json(
                                TblExampleUoResVrts::from_h(er),
                            ));
                            *res.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                            return res;
                        }
                    }
                }
                if let Some(v_b2902425) = &upd_for_query.col_2 {
                    match <pg_crud::I32AsNnInt4 as pg_crud::PgType>::sel_only_updd_ids_qb(
                        &v_b2902425.v,
                        query,
                    ) {
                        Ok(v_result) => {
                            query = v_result;
                        }
                        Err(er_0) => {
                            let er = TblExampleUoEr::TryBind {
                                try_bind: er_0,
                                loc: loc_lib::loc::Loc::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(loc_lib::loc::Occr {
                                        file: String::from(
                                            "pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs",
                                        ),
                                        line: 3002,
                                        col: 25,
                                    }),
                                ),
                            };
                            let mut res = axum::response::IntoResponse::into_response(axum::Json(
                                TblExampleUoResVrts::from_h(er),
                            ));
                            *res.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                            return res;
                        }
                    }
                }
                query
            };
            let mut pool_connection = match app_state.get_pg_pool().acquire().await {
                Ok(v_4535ee48) => v_4535ee48,
                Err(er_0) => {
                    let er = TblExampleUoEr::Pg {
                        pg: er_0,
                        loc: loc_lib::loc::Loc::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(loc_lib::loc::Occr {
                                file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                                line: 3165,
                                col: 29,
                            }),
                        ),
                    };
                    let mut res = axum::response::IntoResponse::into_response(axum::Json(
                        TblExampleUoResVrts::from_h(er),
                    ));
                    *res.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                    return res;
                }
            };
            let executor_acquire = match sqlx::Acquire::acquire(&mut pool_connection).await {
                Ok(v_61ae8f84) => v_61ae8f84,
                Err(er_0) => {
                    let er = TblExampleUoEr::Pg {
                        pg: er_0,
                        loc: loc_lib::loc::Loc::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(loc_lib::loc::Occr {
                                file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                                line: 3165,
                                col: 29,
                            }),
                        ),
                    };
                    let mut res = axum::response::IntoResponse::into_response(axum::Json(
                        TblExampleUoResVrts::from_h(er),
                    ));
                    *res.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                    return res;
                }
            };
            let v = {
                let mut executor = match sqlx::Acquire::begin(executor_acquire).await {
                    Ok(v_1aaca28f) => v_1aaca28f,
                    Err(er_0) => {
                        let er = TblExampleUoEr::Pg {
                            pg: er_0,
                            loc: loc_lib::loc::Loc::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(loc_lib::loc::Occr {
                                    file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                                    line: 2207,
                                    col: 65,
                                }),
                            ),
                        };
                        let mut res = axum::response::IntoResponse::into_response(axum::Json(
                            TblExampleUoResVrts::from_h(er),
                        ));
                        *res.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                        return res;
                    }
                };
                let v = {
                    match binded_query.fetch_one(executor.as_mut()).await {
                        Ok(v_b27d7d79) => {
                            match < TblExampleRdIds as sqlx :: FromRow < '_ , sqlx :: postgres :: PgRow >> :: from_row (& v_b27d7d79) { Ok (v_33759463) => v_33759463 , Err (er_0) => { { if let Err (er_1) = executor . rollback () . await { let er = TblExampleUoEr :: RowAndRollback { row : er_0 , rollback : er_1 , loc : loc_lib :: loc :: Loc :: new (file ! () . to_owned () , line ! () , column ! () , Some (loc_lib :: loc :: Occr { file : String :: from ("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs") , line : 3230 , col : 37 , })) } ; let mut res = axum :: response :: IntoResponse :: into_response (axum :: Json (TblExampleUoResVrts :: from_h (er))) ; * res . status_mut () = http :: StatusCode :: INTERNAL_SERVER_ERROR ; return res ; } let er = TblExampleUoEr :: Pg { pg : er_0 , loc : loc_lib :: loc :: Loc :: new (file ! () . to_owned () , line ! () , column ! () , Some (loc_lib :: loc :: Occr { file : String :: from ("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs") , line : 3230 , col : 37 , })) } ; let mut res = axum :: response :: IntoResponse :: into_response (axum :: Json (TblExampleUoResVrts :: from_h (er))) ; * res . status_mut () = http :: StatusCode :: INTERNAL_SERVER_ERROR ; return res ; } } }
                        }
                        Err(er_0) => {
                            if let Err(er_1) = executor.rollback().await {
                                let er = TblExampleUoEr::RowAndRollback {
                                    row: er_0,
                                    rollback: er_1,
                                    loc: loc_lib::loc::Loc::new(
                                        file!().to_owned(),
                                        line!(),
                                        column!(),
                                        Some(loc_lib::loc::Occr {
                                            file: String::from(
                                                "pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs",
                                            ),
                                            line: 3230,
                                            col: 37,
                                        }),
                                    ),
                                };
                                let mut res = axum::response::IntoResponse::into_response(
                                    axum::Json(TblExampleUoResVrts::from_h(er)),
                                );
                                *res.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                                return res;
                            }
                            let er = TblExampleUoEr::Pg {
                                pg: er_0,
                                loc: loc_lib::loc::Loc::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(loc_lib::loc::Occr {
                                        file: String::from(
                                            "pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs",
                                        ),
                                        line: 3230,
                                        col: 37,
                                    }),
                                ),
                            };
                            let mut res = axum::response::IntoResponse::into_response(axum::Json(
                                TblExampleUoResVrts::from_h(er),
                            ));
                            *res.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                            return res;
                        }
                    }
                };
                if let Err(er_0) = executor.commit().await {
                    let er = TblExampleUoEr::Pg {
                        pg: er_0,
                        loc: loc_lib::loc::Loc::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(loc_lib::loc::Occr {
                                file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                                line: 2217,
                                col: 65,
                            }),
                        ),
                    };
                    let mut res = axum::response::IntoResponse::into_response(axum::Json(
                        TblExampleUoResVrts::from_h(er),
                    ));
                    *res.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                    return res;
                }
                v
            };
            let mut res = axum::response::IntoResponse::into_response(axum::Json(
                TblExampleUoResVrts::Desirable(v),
            ));
            *res.status_mut() = http::StatusCode::OK;
            res
        }
        #[allow(clippy::single_call_fn)]
        pub async fn uo(
            app_state: axum::extract::State<
                std::sync::Arc<dyn pg_crud::CombinationOfAppStateLogicTraits>,
            >,
            req: axum::extract::Request,
        ) -> axum::response::Response {
            Self::uo_h(app_state, req, Self::tbl_name()).await
        }
        #[allow(clippy::single_call_fn)]
        async fn try_uo_h(
            endpoint_loc: &str,
            prms: TblExampleUoPrms,
            tbl: &str,
        ) -> Result<TblExampleRdIds, TblExampleTryUoEr> {
            let payload = {
                match serde_json::to_string(&prms.payload) {
                    Ok(v_1772a83e) => v_1772a83e,
                    Err(er_0) => {
                        return Err(TblExampleTryUoEr::SerdeJsonToString {
                            serde_json_to_string: er_0,
                            loc: loc_lib::loc::Loc::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(loc_lib::loc::Occr {
                                    file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                                    line: 2396,
                                    col: 81,
                                }),
                            ),
                        });
                    }
                }
            };
            let url = format!("{endpoint_loc}/{tbl}/uo");
            let future = reqwest::Client::new()
                .patch(&url)
                .header(&"commit".to_owned(), git_info::PROJECT_GIT_INFO.commit)
                .header(reqwest::header::CONTENT_TYPE, "application/json")
                .body(payload)
                .send();
            let res = match future.await {
                Ok(v_180559e9) => v_180559e9,
                Err(er_0) => {
                    return Err(TblExampleTryUoEr::Reqwest {
                        reqwest: er_0,
                        loc: loc_lib::loc::Loc::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(loc_lib::loc::Occr {
                                file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                                line: 2440,
                                col: 68,
                            }),
                        ),
                    });
                }
            };
            let er_0 = res.status();
            let er_1 = res.headers().clone();
            let er_2 = match res.text().await {
                Ok(v_6a62b2b9) => v_6a62b2b9,
                Err(er_2) => {
                    return Err(TblExampleTryUoEr::FailedToGetResText {
                        status_code: er_0,
                        headers: er_1,
                        reqwest: er_2,
                        loc: loc_lib::loc::Loc::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(loc_lib::loc::Occr {
                                file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                                line: 2461,
                                col: 78,
                            }),
                        ),
                    });
                }
            };
            let expected_res = match serde_json::from_str::<TblExampleUoResVrts>(&er_2) {
                Ok(v_563d2a75) => v_563d2a75,
                Err(er_3) => {
                    return Err(TblExampleTryUoEr::DeRes {
                        status_code: er_0,
                        headers: er_1,
                        res_text: er_2,
                        serde: er_3,
                        loc: loc_lib::loc::Loc::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(loc_lib::loc::Occr {
                                file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                                line: 2472,
                                col: 63,
                            }),
                        ),
                    });
                }
            };
            let uo_er_with_serde = match expected_res {
                TblExampleUoResVrts::Desirable(v) => {
                    return Ok(v);
                }
                TblExampleUoResVrts::CheckBodySize {
                    check_body_size,
                    loc,
                } => TblExampleUoErWithSerde::CheckBodySize {
                    check_body_size,
                    loc,
                },
                TblExampleUoResVrts::Pg { pg, loc } => TblExampleUoErWithSerde::Pg { pg, loc },
                TblExampleUoResVrts::SerdeJson { serde_json, loc } => {
                    TblExampleUoErWithSerde::SerdeJson { serde_json, loc }
                }
                TblExampleUoResVrts::HeaderContentTypeAppJsonNotFound { loc } => {
                    TblExampleUoErWithSerde::HeaderContentTypeAppJsonNotFound { loc }
                }
                TblExampleUoResVrts::CheckCommit { check_commit, loc } => {
                    TblExampleUoErWithSerde::CheckCommit { check_commit, loc }
                }
                TblExampleUoResVrts::Qp { er, loc } => TblExampleUoErWithSerde::Qp { er, loc },
                TblExampleUoResVrts::RowAndRollback { row, rollback, loc } => {
                    TblExampleUoErWithSerde::RowAndRollback { row, rollback, loc }
                }
                TblExampleUoResVrts::TryBind { try_bind, loc } => {
                    TblExampleUoErWithSerde::TryBind { try_bind, loc }
                }
            };
            Err(TblExampleTryUoEr::TblExampleUoErWithSerde {
                uo_er_with_serde,
                loc: loc_lib::loc::Loc::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(loc_lib::loc::Occr {
                        file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                        line: 2512,
                        col: 83,
                    }),
                ),
            })
        }
        pub async fn try_uo(
            endpoint_loc: &str,
            prms: TblExampleUoPrms,
        ) -> Result<TblExampleRdIds, TblExampleTryUoEr> {
            Self::try_uo_h(endpoint_loc, prms, Self::tbl_name()).await
        }
        #[must_use]
        pub fn uo_payload_example() -> axum::response::Response {
            let mut res = axum::response::IntoResponse::into_response(axum::Json(
                <TblExampleUpd as pg_crud::DfltSomeOneEl>::dflt_some_one_el(),
            ));
            *res.status_mut() = http::StatusCode::OK;
            res
        }
        #[allow(clippy::single_call_fn)]
        async fn dm_h(
            app_state: axum::extract::State<
                std::sync::Arc<dyn pg_crud::CombinationOfAppStateLogicTraits>,
            >,
            req: axum::extract::Request,
            tbl: &str,
        ) -> axum::response::Response {
            let (parts, body) = req.into_parts();
            let headers = parts.headers;
            if !matches ! (headers . get (http :: header :: CONTENT_TYPE) , Some (v_e3f6eecd) if v_e3f6eecd == http :: header :: HeaderValue :: from_static ("application/json"))
            {
                let er = TblExampleDmEr::HeaderContentTypeAppJsonNotFound {
                    loc: loc_lib::loc::Loc::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(loc_lib::loc::Occr {
                            file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                            line: 2555,
                            col: 25,
                        }),
                    ),
                };
                let mut res = axum::response::IntoResponse::into_response(axum::Json(
                    TblExampleDmResVrts::from_h(er),
                ));
                *res.status_mut() = http::StatusCode::BAD_REQUEST;
                return res;
            }
            let body_bytes = match pg_crud::check_body_size::check_body_size(
                body,
                *app_state.get_maximum_size_of_http_body_in_bytes(),
            )
            .await
            {
                Ok(v_cfac9140) => v_cfac9140,
                Err(er_0) => {
                    let er = TblExampleDmEr::CheckBodySize {
                        check_body_size: er_0,
                        loc: loc_lib::loc::Loc::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(loc_lib::loc::Occr {
                                file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                                line: 2564,
                                col: 33,
                            }),
                        ),
                    };
                    let mut res = axum::response::IntoResponse::into_response(axum::Json(
                        TblExampleDmResVrts::from_h(er),
                    ));
                    *res.status_mut() = http::StatusCode::BAD_REQUEST;
                    return res;
                }
            };
            let prms = TblExampleDmPrms {
                payload: match serde_json::from_slice::<TblExampleDmPayload>(&body_bytes) {
                    Ok(v_9e6fcd2d) => v_9e6fcd2d,
                    Err(er_0) => {
                        let er = TblExampleDmEr::SerdeJson {
                            serde_json: er_0,
                            loc: loc_lib::loc::Loc::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(loc_lib::loc::Occr {
                                    file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                                    line: 2609,
                                    col: 37,
                                }),
                            ),
                        };
                        let mut res = axum::response::IntoResponse::into_response(axum::Json(
                            TblExampleDmResVrts::from_h(er),
                        ));
                        *res.status_mut() = http::StatusCode::BAD_REQUEST;
                        return res;
                    }
                },
            };
            let query_string = pg_crud::gen_dm_query_string(
                tbl,
                &{
                    let mut incr: u64 = 0;
                    match pg_crud::PgTypeWhFlt::qp(&prms.payload.wh_many, &mut incr, &"", false) {
                        Ok(v_d1627695) => v_d1627695,
                        Err(er_0) => {
                            let er = TblExampleDmEr::Qp {
                                er: er_0,
                                loc: loc_lib::loc::Loc::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(loc_lib::loc::Occr {
                                        file: String::from(
                                            "pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs",
                                        ),
                                        line: 1300,
                                        col: 21,
                                    }),
                                ),
                            };
                            let mut res = axum::response::IntoResponse::into_response(axum::Json(
                                TblExampleDmResVrts::from_h(er),
                            ));
                            *res.status_mut() = http::StatusCode::BAD_REQUEST;
                            return res;
                        }
                    }
                },
                Self::pk(),
            );
            let binded_query = {
                let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
                match pg_crud::PgTypeWhFlt::qb(prms.payload.wh_many, query) {
                    Ok(v_03a58371) => {
                        query = v_03a58371;
                    }
                    Err(er_0) => {
                        let er = TblExampleDmEr::TryBind {
                            try_bind: er_0,
                            loc: loc_lib::loc::Loc::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(loc_lib::loc::Occr {
                                    file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                                    line: 1324,
                                    col: 68,
                                }),
                            ),
                        };
                        let mut res = axum::response::IntoResponse::into_response(axum::Json(
                            TblExampleDmResVrts::from_h(er),
                        ));
                        *res.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                        return res;
                    }
                }
                query
            };
            let mut pool_connection = match app_state.get_pg_pool().acquire().await {
                Ok(v_4535ee48) => v_4535ee48,
                Err(er_0) => {
                    let er = TblExampleDmEr::Pg {
                        pg: er_0,
                        loc: loc_lib::loc::Loc::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(loc_lib::loc::Occr {
                                file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                                line: 3165,
                                col: 29,
                            }),
                        ),
                    };
                    let mut res = axum::response::IntoResponse::into_response(axum::Json(
                        TblExampleDmResVrts::from_h(er),
                    ));
                    *res.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                    return res;
                }
            };
            let executor_acquire = match sqlx::Acquire::acquire(&mut pool_connection).await {
                Ok(v_61ae8f84) => v_61ae8f84,
                Err(er_0) => {
                    let er = TblExampleDmEr::Pg {
                        pg: er_0,
                        loc: loc_lib::loc::Loc::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(loc_lib::loc::Occr {
                                file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                                line: 3165,
                                col: 29,
                            }),
                        ),
                    };
                    let mut res = axum::response::IntoResponse::into_response(axum::Json(
                        TblExampleDmResVrts::from_h(er),
                    ));
                    *res.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                    return res;
                }
            };
            let v = {
                let mut executor = match sqlx::Acquire::begin(executor_acquire).await {
                    Ok(v_1aaca28f) => v_1aaca28f,
                    Err(er_0) => {
                        let er = TblExampleDmEr::Pg {
                            pg: er_0,
                            loc: loc_lib::loc::Loc::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(loc_lib::loc::Occr {
                                    file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                                    line: 2207,
                                    col: 65,
                                }),
                            ),
                        };
                        let mut res = axum::response::IntoResponse::into_response(axum::Json(
                            TblExampleDmResVrts::from_h(er),
                        ));
                        *res.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                        return res;
                    }
                };
                let v = {
                    let mut rows = binded_query.fetch(executor.as_mut());
                    let mut acc_d16ac269 = Vec::new();
                    while let Some (v_d9cc2c36) = match pg_crud :: TryStreamExt :: try_next (& mut rows) . await { Ok (v_19f3d6e1) => match v_19f3d6e1 { Some (v_b27d7d79) => match sqlx :: Row :: try_get :: < < pg_crud :: SqlxTypesUuidUuidAsNnUuidV4InitByPg as pg_crud :: PgType > :: Rd , & str > (& v_b27d7d79 , Self :: pk ()) { Ok (v_69ecb6a9) => Some (v_69ecb6a9) , Err (er_0) => { drop (rows) ; { if let Err (er_1) = executor . rollback () . await { let er = TblExampleDmEr :: RowAndRollback { row : er_0 , rollback : er_1 , loc : loc_lib :: loc :: Loc :: new (file ! () . to_owned () , line ! () , column ! () , Some (loc_lib :: loc :: Occr { file : String :: from ("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs") , line : 3213 , col : 45 , })) } ; let mut res = axum :: response :: IntoResponse :: into_response (axum :: Json (TblExampleDmResVrts :: from_h (er))) ; * res . status_mut () = http :: StatusCode :: INTERNAL_SERVER_ERROR ; return res ; } let er = TblExampleDmEr :: Pg { pg : er_0 , loc : loc_lib :: loc :: Loc :: new (file ! () . to_owned () , line ! () , column ! () , Some (loc_lib :: loc :: Occr { file : String :: from ("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs") , line : 3213 , col : 45 , })) } ; let mut res = axum :: response :: IntoResponse :: into_response (axum :: Json (TblExampleDmResVrts :: from_h (er))) ; * res . status_mut () = http :: StatusCode :: INTERNAL_SERVER_ERROR ; return res ; } } } , None => None , } , Err (er_0) => { drop (rows) ; { if let Err (er_1) = executor . rollback () . await { let er = TblExampleDmEr :: RowAndRollback { row : er_0 , rollback : er_1 , loc : loc_lib :: loc :: Loc :: new (file ! () . to_owned () , line ! () , column ! () , Some (loc_lib :: loc :: Occr { file : String :: from ("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs") , line : 3219 , col : 37 , })) } ; let mut res = axum :: response :: IntoResponse :: into_response (axum :: Json (TblExampleDmResVrts :: from_h (er))) ; * res . status_mut () = http :: StatusCode :: INTERNAL_SERVER_ERROR ; return res ; } let er = TblExampleDmEr :: Pg { pg : er_0 , loc : loc_lib :: loc :: Loc :: new (file ! () . to_owned () , line ! () , column ! () , Some (loc_lib :: loc :: Occr { file : String :: from ("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs") , line : 3219 , col : 37 , })) } ; let mut res = axum :: response :: IntoResponse :: into_response (axum :: Json (TblExampleDmResVrts :: from_h (er))) ; * res . status_mut () = http :: StatusCode :: INTERNAL_SERVER_ERROR ; return res ; } } } { acc_d16ac269 . push (v_d9cc2c36) ; }
                    acc_d16ac269
                };
                if let Err(er_0) = executor.commit().await {
                    let er = TblExampleDmEr::Pg {
                        pg: er_0,
                        loc: loc_lib::loc::Loc::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(loc_lib::loc::Occr {
                                file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                                line: 2217,
                                col: 65,
                            }),
                        ),
                    };
                    let mut res = axum::response::IntoResponse::into_response(axum::Json(
                        TblExampleDmResVrts::from_h(er),
                    ));
                    *res.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                    return res;
                }
                v
            };
            let mut res = axum::response::IntoResponse::into_response(axum::Json(
                TblExampleDmResVrts::Desirable(v),
            ));
            *res.status_mut() = http::StatusCode::OK;
            res
        }
        #[allow(clippy::single_call_fn)]
        pub async fn dm(
            app_state: axum::extract::State<
                std::sync::Arc<dyn pg_crud::CombinationOfAppStateLogicTraits>,
            >,
            req: axum::extract::Request,
        ) -> axum::response::Response {
            Self::dm_h(app_state, req, Self::tbl_name()).await
        }
        #[allow(clippy::single_call_fn)]
        async fn try_dm_h(
            endpoint_loc: &str,
            prms: TblExampleDmPrms,
            tbl: &str,
        ) -> Result<
            Vec<<pg_crud::SqlxTypesUuidUuidAsNnUuidV4InitByPg as pg_crud::PgType>::Rd>,
            TblExampleTryDmEr,
        > {
            let payload = {
                match serde_json::to_string(&prms.payload) {
                    Ok(v_1772a83e) => v_1772a83e,
                    Err(er_0) => {
                        return Err(TblExampleTryDmEr::SerdeJsonToString {
                            serde_json_to_string: er_0,
                            loc: loc_lib::loc::Loc::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(loc_lib::loc::Occr {
                                    file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                                    line: 2396,
                                    col: 81,
                                }),
                            ),
                        });
                    }
                }
            };
            let url = format!("{endpoint_loc}/{tbl}/dm");
            let future = reqwest::Client::new()
                .delete(&url)
                .header(&"commit".to_owned(), git_info::PROJECT_GIT_INFO.commit)
                .header(reqwest::header::CONTENT_TYPE, "application/json")
                .body(payload)
                .send();
            let res = match future.await {
                Ok(v_180559e9) => v_180559e9,
                Err(er_0) => {
                    return Err(TblExampleTryDmEr::Reqwest {
                        reqwest: er_0,
                        loc: loc_lib::loc::Loc::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(loc_lib::loc::Occr {
                                file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                                line: 2440,
                                col: 68,
                            }),
                        ),
                    });
                }
            };
            let er_0 = res.status();
            let er_1 = res.headers().clone();
            let er_2 = match res.text().await {
                Ok(v_6a62b2b9) => v_6a62b2b9,
                Err(er_2) => {
                    return Err(TblExampleTryDmEr::FailedToGetResText {
                        status_code: er_0,
                        headers: er_1,
                        reqwest: er_2,
                        loc: loc_lib::loc::Loc::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(loc_lib::loc::Occr {
                                file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                                line: 2461,
                                col: 78,
                            }),
                        ),
                    });
                }
            };
            let expected_res = match serde_json::from_str::<TblExampleDmResVrts>(&er_2) {
                Ok(v_563d2a75) => v_563d2a75,
                Err(er_3) => {
                    return Err(TblExampleTryDmEr::DeRes {
                        status_code: er_0,
                        headers: er_1,
                        res_text: er_2,
                        serde: er_3,
                        loc: loc_lib::loc::Loc::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(loc_lib::loc::Occr {
                                file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                                line: 2472,
                                col: 63,
                            }),
                        ),
                    });
                }
            };
            let dm_er_with_serde = match expected_res {
                TblExampleDmResVrts::Desirable(v) => {
                    return Ok(v);
                }
                TblExampleDmResVrts::CheckBodySize {
                    check_body_size,
                    loc,
                } => TblExampleDmErWithSerde::CheckBodySize {
                    check_body_size,
                    loc,
                },
                TblExampleDmResVrts::Pg { pg, loc } => TblExampleDmErWithSerde::Pg { pg, loc },
                TblExampleDmResVrts::SerdeJson { serde_json, loc } => {
                    TblExampleDmErWithSerde::SerdeJson { serde_json, loc }
                }
                TblExampleDmResVrts::HeaderContentTypeAppJsonNotFound { loc } => {
                    TblExampleDmErWithSerde::HeaderContentTypeAppJsonNotFound { loc }
                }
                TblExampleDmResVrts::CheckCommit { check_commit, loc } => {
                    TblExampleDmErWithSerde::CheckCommit { check_commit, loc }
                }
                TblExampleDmResVrts::Qp { er, loc } => TblExampleDmErWithSerde::Qp { er, loc },
                TblExampleDmResVrts::RowAndRollback { row, rollback, loc } => {
                    TblExampleDmErWithSerde::RowAndRollback { row, rollback, loc }
                }
                TblExampleDmResVrts::TryBind { try_bind, loc } => {
                    TblExampleDmErWithSerde::TryBind { try_bind, loc }
                }
            };
            Err(TblExampleTryDmEr::TblExampleDmErWithSerde {
                dm_er_with_serde,
                loc: loc_lib::loc::Loc::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(loc_lib::loc::Occr {
                        file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                        line: 2512,
                        col: 83,
                    }),
                ),
            })
        }
        pub async fn try_dm(
            endpoint_loc: &str,
            prms: TblExampleDmPrms,
        ) -> Result<
            Vec<<pg_crud::SqlxTypesUuidUuidAsNnUuidV4InitByPg as pg_crud::PgType>::Rd>,
            TblExampleTryDmEr,
        > {
            Self::try_dm_h(endpoint_loc, prms, Self::tbl_name()).await
        }
        #[must_use]
        pub fn dm_payload_example() -> axum::response::Response {
            let mut res = axum::response::IntoResponse::into_response(axum::Json(
                <TblExampleDmPayload as pg_crud::DfltSomeOneEl>::dflt_some_one_el(),
            ));
            *res.status_mut() = http::StatusCode::OK;
            res
        }
        #[allow(clippy::single_call_fn)]
        async fn dlo_h(
            app_state: axum::extract::State<
                std::sync::Arc<dyn pg_crud::CombinationOfAppStateLogicTraits>,
            >,
            req: axum::extract::Request,
            tbl: &str,
        ) -> axum::response::Response {
            let (parts, body) = req.into_parts();
            let headers = parts.headers;
            if !matches ! (headers . get (http :: header :: CONTENT_TYPE) , Some (v_e3f6eecd) if v_e3f6eecd == http :: header :: HeaderValue :: from_static ("application/json"))
            {
                let er = TblExampleDloEr::HeaderContentTypeAppJsonNotFound {
                    loc: loc_lib::loc::Loc::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(loc_lib::loc::Occr {
                            file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                            line: 2555,
                            col: 25,
                        }),
                    ),
                };
                let mut res = axum::response::IntoResponse::into_response(axum::Json(
                    TblExampleDloResVrts::from_h(er),
                ));
                *res.status_mut() = http::StatusCode::BAD_REQUEST;
                return res;
            }
            let body_bytes = match pg_crud::check_body_size::check_body_size(
                body,
                *app_state.get_maximum_size_of_http_body_in_bytes(),
            )
            .await
            {
                Ok(v_cfac9140) => v_cfac9140,
                Err(er_0) => {
                    let er = TblExampleDloEr::CheckBodySize {
                        check_body_size: er_0,
                        loc: loc_lib::loc::Loc::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(loc_lib::loc::Occr {
                                file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                                line: 2564,
                                col: 33,
                            }),
                        ),
                    };
                    let mut res = axum::response::IntoResponse::into_response(axum::Json(
                        TblExampleDloResVrts::from_h(er),
                    ));
                    *res.status_mut() = http::StatusCode::BAD_REQUEST;
                    return res;
                }
            };
            let prms = TblExampleDloPrms {
                payload: match serde_json::from_slice::<TblExampleDloPayload>(&body_bytes) {
                    Ok(v_9e6fcd2d) => v_9e6fcd2d,
                    Err(er_0) => {
                        let er = TblExampleDloEr::SerdeJson {
                            serde_json: er_0,
                            loc: loc_lib::loc::Loc::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(loc_lib::loc::Occr {
                                    file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                                    line: 2609,
                                    col: 37,
                                }),
                            ),
                        };
                        let mut res = axum::response::IntoResponse::into_response(axum::Json(
                            TblExampleDloResVrts::from_h(er),
                        ));
                        *res.status_mut() = http::StatusCode::BAD_REQUEST;
                        return res;
                    }
                },
            };
            let query_string = pg_crud::gen_dlo_query_string(tbl, Self::pk());
            let binded_query = {
                let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
                match pg_crud::PgTypeWhFlt::qb(prms.payload.pk_col, query) {
                    Ok(v_3099ea0f) => {
                        query = v_3099ea0f;
                    }
                    Err(er_0) => {
                        let er = TblExampleDloEr::TryBind {
                            try_bind: er_0,
                            loc: loc_lib::loc::Loc::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(loc_lib::loc::Occr {
                                    file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                                    line: 3002,
                                    col: 25,
                                }),
                            ),
                        };
                        let mut res = axum::response::IntoResponse::into_response(axum::Json(
                            TblExampleDloResVrts::from_h(er),
                        ));
                        *res.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                        return res;
                    }
                }
                query
            };
            let mut pool_connection = match app_state.get_pg_pool().acquire().await {
                Ok(v_4535ee48) => v_4535ee48,
                Err(er_0) => {
                    let er = TblExampleDloEr::Pg {
                        pg: er_0,
                        loc: loc_lib::loc::Loc::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(loc_lib::loc::Occr {
                                file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                                line: 3165,
                                col: 29,
                            }),
                        ),
                    };
                    let mut res = axum::response::IntoResponse::into_response(axum::Json(
                        TblExampleDloResVrts::from_h(er),
                    ));
                    *res.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                    return res;
                }
            };
            let executor_acquire = match sqlx::Acquire::acquire(&mut pool_connection).await {
                Ok(v_61ae8f84) => v_61ae8f84,
                Err(er_0) => {
                    let er = TblExampleDloEr::Pg {
                        pg: er_0,
                        loc: loc_lib::loc::Loc::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(loc_lib::loc::Occr {
                                file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                                line: 3165,
                                col: 29,
                            }),
                        ),
                    };
                    let mut res = axum::response::IntoResponse::into_response(axum::Json(
                        TblExampleDloResVrts::from_h(er),
                    ));
                    *res.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                    return res;
                }
            };
            let v = {
                let mut executor = match sqlx::Acquire::begin(executor_acquire).await {
                    Ok(v_1aaca28f) => v_1aaca28f,
                    Err(er_0) => {
                        let er = TblExampleDloEr::Pg {
                            pg: er_0,
                            loc: loc_lib::loc::Loc::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(loc_lib::loc::Occr {
                                    file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                                    line: 2207,
                                    col: 65,
                                }),
                            ),
                        };
                        let mut res = axum::response::IntoResponse::into_response(axum::Json(
                            TblExampleDloResVrts::from_h(er),
                        ));
                        *res.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                        return res;
                    }
                };
                let v = {
                    match binded_query.fetch_one(executor.as_mut()).await {
                        Ok(v_b27d7d79) => {
                            match sqlx :: Row :: try_get :: < < pg_crud :: SqlxTypesUuidUuidAsNnUuidV4InitByPg as pg_crud :: PgType > :: Rd , & str > (& v_b27d7d79 , Self :: pk ()) { Ok (v_69ecb6a9) => v_69ecb6a9 , Err (er_0) => { { if let Err (er_1) = executor . rollback () . await { let er = TblExampleDloEr :: RowAndRollback { row : er_0 , rollback : er_1 , loc : loc_lib :: loc :: Loc :: new (file ! () . to_owned () , line ! () , column ! () , Some (loc_lib :: loc :: Occr { file : String :: from ("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs") , line : 3230 , col : 37 , })) } ; let mut res = axum :: response :: IntoResponse :: into_response (axum :: Json (TblExampleDloResVrts :: from_h (er))) ; * res . status_mut () = http :: StatusCode :: INTERNAL_SERVER_ERROR ; return res ; } let er = TblExampleDloEr :: Pg { pg : er_0 , loc : loc_lib :: loc :: Loc :: new (file ! () . to_owned () , line ! () , column ! () , Some (loc_lib :: loc :: Occr { file : String :: from ("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs") , line : 3230 , col : 37 , })) } ; let mut res = axum :: response :: IntoResponse :: into_response (axum :: Json (TblExampleDloResVrts :: from_h (er))) ; * res . status_mut () = http :: StatusCode :: INTERNAL_SERVER_ERROR ; return res ; } } }
                        }
                        Err(er_0) => {
                            if let Err(er_1) = executor.rollback().await {
                                let er = TblExampleDloEr::RowAndRollback {
                                    row: er_0,
                                    rollback: er_1,
                                    loc: loc_lib::loc::Loc::new(
                                        file!().to_owned(),
                                        line!(),
                                        column!(),
                                        Some(loc_lib::loc::Occr {
                                            file: String::from(
                                                "pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs",
                                            ),
                                            line: 3230,
                                            col: 37,
                                        }),
                                    ),
                                };
                                let mut res = axum::response::IntoResponse::into_response(
                                    axum::Json(TblExampleDloResVrts::from_h(er)),
                                );
                                *res.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                                return res;
                            }
                            let er = TblExampleDloEr::Pg {
                                pg: er_0,
                                loc: loc_lib::loc::Loc::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(loc_lib::loc::Occr {
                                        file: String::from(
                                            "pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs",
                                        ),
                                        line: 3230,
                                        col: 37,
                                    }),
                                ),
                            };
                            let mut res = axum::response::IntoResponse::into_response(axum::Json(
                                TblExampleDloResVrts::from_h(er),
                            ));
                            *res.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                            return res;
                        }
                    }
                };
                if let Err(er_0) = executor.commit().await {
                    let er = TblExampleDloEr::Pg {
                        pg: er_0,
                        loc: loc_lib::loc::Loc::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(loc_lib::loc::Occr {
                                file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                                line: 2217,
                                col: 65,
                            }),
                        ),
                    };
                    let mut res = axum::response::IntoResponse::into_response(axum::Json(
                        TblExampleDloResVrts::from_h(er),
                    ));
                    *res.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                    return res;
                }
                v
            };
            let mut res = axum::response::IntoResponse::into_response(axum::Json(
                TblExampleDloResVrts::Desirable(v),
            ));
            *res.status_mut() = http::StatusCode::OK;
            res
        }
        #[allow(clippy::single_call_fn)]
        pub async fn dlo(
            app_state: axum::extract::State<
                std::sync::Arc<dyn pg_crud::CombinationOfAppStateLogicTraits>,
            >,
            req: axum::extract::Request,
        ) -> axum::response::Response {
            Self::dlo_h(app_state, req, Self::tbl_name()).await
        }
        #[allow(clippy::single_call_fn)]
        async fn try_dlo_h(
            endpoint_loc: &str,
            prms: TblExampleDloPrms,
            tbl: &str,
        ) -> Result<
            <pg_crud::SqlxTypesUuidUuidAsNnUuidV4InitByPg as pg_crud::PgType>::Rd,
            TblExampleTryDloEr,
        > {
            let payload = {
                match serde_json::to_string(&prms.payload) {
                    Ok(v_1772a83e) => v_1772a83e,
                    Err(er_0) => {
                        return Err(TblExampleTryDloEr::SerdeJsonToString {
                            serde_json_to_string: er_0,
                            loc: loc_lib::loc::Loc::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(loc_lib::loc::Occr {
                                    file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                                    line: 2396,
                                    col: 81,
                                }),
                            ),
                        });
                    }
                }
            };
            let url = format!("{endpoint_loc}/{tbl}/dlo");
            let future = reqwest::Client::new()
                .delete(&url)
                .header(&"commit".to_owned(), git_info::PROJECT_GIT_INFO.commit)
                .header(reqwest::header::CONTENT_TYPE, "application/json")
                .body(payload)
                .send();
            let res = match future.await {
                Ok(v_180559e9) => v_180559e9,
                Err(er_0) => {
                    return Err(TblExampleTryDloEr::Reqwest {
                        reqwest: er_0,
                        loc: loc_lib::loc::Loc::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(loc_lib::loc::Occr {
                                file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                                line: 2440,
                                col: 68,
                            }),
                        ),
                    });
                }
            };
            let er_0 = res.status();
            let er_1 = res.headers().clone();
            let er_2 = match res.text().await {
                Ok(v_6a62b2b9) => v_6a62b2b9,
                Err(er_2) => {
                    return Err(TblExampleTryDloEr::FailedToGetResText {
                        status_code: er_0,
                        headers: er_1,
                        reqwest: er_2,
                        loc: loc_lib::loc::Loc::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(loc_lib::loc::Occr {
                                file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                                line: 2461,
                                col: 78,
                            }),
                        ),
                    });
                }
            };
            let expected_res = match serde_json::from_str::<TblExampleDloResVrts>(&er_2) {
                Ok(v_563d2a75) => v_563d2a75,
                Err(er_3) => {
                    return Err(TblExampleTryDloEr::DeRes {
                        status_code: er_0,
                        headers: er_1,
                        res_text: er_2,
                        serde: er_3,
                        loc: loc_lib::loc::Loc::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(loc_lib::loc::Occr {
                                file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                                line: 2472,
                                col: 63,
                            }),
                        ),
                    });
                }
            };
            let dlo_er_with_serde = match expected_res {
                TblExampleDloResVrts::Desirable(v) => {
                    return Ok(v);
                }
                TblExampleDloResVrts::CheckBodySize {
                    check_body_size,
                    loc,
                } => TblExampleDloErWithSerde::CheckBodySize {
                    check_body_size,
                    loc,
                },
                TblExampleDloResVrts::Pg { pg, loc } => TblExampleDloErWithSerde::Pg { pg, loc },
                TblExampleDloResVrts::SerdeJson { serde_json, loc } => {
                    TblExampleDloErWithSerde::SerdeJson { serde_json, loc }
                }
                TblExampleDloResVrts::HeaderContentTypeAppJsonNotFound { loc } => {
                    TblExampleDloErWithSerde::HeaderContentTypeAppJsonNotFound { loc }
                }
                TblExampleDloResVrts::CheckCommit { check_commit, loc } => {
                    TblExampleDloErWithSerde::CheckCommit { check_commit, loc }
                }
                TblExampleDloResVrts::RowAndRollback { row, rollback, loc } => {
                    TblExampleDloErWithSerde::RowAndRollback { row, rollback, loc }
                }
                TblExampleDloResVrts::TryBind { try_bind, loc } => {
                    TblExampleDloErWithSerde::TryBind { try_bind, loc }
                }
            };
            Err(TblExampleTryDloEr::TblExampleDloErWithSerde {
                dlo_er_with_serde,
                loc: loc_lib::loc::Loc::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(loc_lib::loc::Occr {
                        file: String::from("pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs"),
                        line: 2512,
                        col: 83,
                    }),
                ),
            })
        }
        pub async fn try_dlo(
            endpoint_loc: &str,
            prms: TblExampleDloPrms,
        ) -> Result<
            <pg_crud::SqlxTypesUuidUuidAsNnUuidV4InitByPg as pg_crud::PgType>::Rd,
            TblExampleTryDloEr,
        > {
            Self::try_dlo_h(endpoint_loc, prms, Self::tbl_name()).await
        }
        #[must_use]
        pub fn dlo_payload_example() -> axum::response::Response {
            let mut res = axum::response::IntoResponse::into_response(axum::Json(
                <TblExampleDloPayload as pg_crud::DfltSomeOneEl>::dflt_some_one_el(),
            ));
            *res.status_mut() = http::StatusCode::OK;
            res
        }
        pub fn routes(
            app_state: std::sync::Arc<dyn pg_crud::CombinationOfAppStateLogicTraits>,
        ) -> axum::Router {
            Self::routes_h(app_state, Self::tbl_name())
        }
        #[allow(clippy::single_call_fn)]
        fn routes_h(
            app_state: std::sync::Arc<dyn pg_crud::CombinationOfAppStateLogicTraits>,
            tbl: &str,
        ) -> axum::Router {
            axum::Router::new().nest(
                &format!("/{tbl}"),
                axum::Router::new()
                    .route(
                        "/cm",
                        axum::routing::post({
                            let tbl_owned = tbl.to_owned();
                            async move |app_state_99328dfe: axum::extract::State<
                                std::sync::Arc<dyn pg_crud::CombinationOfAppStateLogicTraits>,
                            >,
                                        req: axum::extract::Request| {
                                Self::cm_h(app_state_99328dfe, req, &tbl_owned).await
                            }
                        }),
                    )
                    .route(
                        "/cm_payload_example",
                        axum::routing::get(async move || Self::cm_payload_example()),
                    )
                    .route(
                        "/co",
                        axum::routing::post({
                            let tbl_owned = tbl.to_owned();
                            async move |app_state_99328dfe: axum::extract::State<
                                std::sync::Arc<dyn pg_crud::CombinationOfAppStateLogicTraits>,
                            >,
                                        req: axum::extract::Request| {
                                Self::co_h(app_state_99328dfe, req, &tbl_owned).await
                            }
                        }),
                    )
                    .route(
                        "/co_payload_example",
                        axum::routing::get(async move || Self::co_payload_example()),
                    )
                    .route(
                        "/rm",
                        axum::routing::post({
                            let tbl_owned = tbl.to_owned();
                            async move |app_state_99328dfe: axum::extract::State<
                                std::sync::Arc<dyn pg_crud::CombinationOfAppStateLogicTraits>,
                            >,
                                        req: axum::extract::Request| {
                                Self::rm_h(app_state_99328dfe, req, &tbl_owned).await
                            }
                        }),
                    )
                    .route(
                        "/rm_payload_example",
                        axum::routing::get(async move || Self::rm_payload_example()),
                    )
                    .route(
                        "/ro",
                        axum::routing::post({
                            let tbl_owned = tbl.to_owned();
                            async move |app_state_99328dfe: axum::extract::State<
                                std::sync::Arc<dyn pg_crud::CombinationOfAppStateLogicTraits>,
                            >,
                                        req: axum::extract::Request| {
                                Self::ro_h(app_state_99328dfe, req, &tbl_owned).await
                            }
                        }),
                    )
                    .route(
                        "/ro_payload_example",
                        axum::routing::get(async move || Self::ro_payload_example()),
                    )
                    .route(
                        "/um",
                        axum::routing::patch({
                            let tbl_owned = tbl.to_owned();
                            async move |app_state_99328dfe: axum::extract::State<
                                std::sync::Arc<dyn pg_crud::CombinationOfAppStateLogicTraits>,
                            >,
                                        req: axum::extract::Request| {
                                Self::um_h(app_state_99328dfe, req, &tbl_owned).await
                            }
                        }),
                    )
                    .route(
                        "/um_payload_example",
                        axum::routing::get(async move || Self::um_payload_example()),
                    )
                    .route(
                        "/uo",
                        axum::routing::patch({
                            let tbl_owned = tbl.to_owned();
                            async move |app_state_99328dfe: axum::extract::State<
                                std::sync::Arc<dyn pg_crud::CombinationOfAppStateLogicTraits>,
                            >,
                                        req: axum::extract::Request| {
                                Self::uo_h(app_state_99328dfe, req, &tbl_owned).await
                            }
                        }),
                    )
                    .route(
                        "/uo_payload_example",
                        axum::routing::get(async move || Self::uo_payload_example()),
                    )
                    .route(
                        "/dm",
                        axum::routing::delete({
                            let tbl_owned = tbl.to_owned();
                            async move |app_state_99328dfe: axum::extract::State<
                                std::sync::Arc<dyn pg_crud::CombinationOfAppStateLogicTraits>,
                            >,
                                        req: axum::extract::Request| {
                                Self::dm_h(app_state_99328dfe, req, &tbl_owned).await
                            }
                        }),
                    )
                    .route(
                        "/dm_payload_example",
                        axum::routing::get(async move || Self::dm_payload_example()),
                    )
                    .route(
                        "/dlo",
                        axum::routing::delete({
                            let tbl_owned = tbl.to_owned();
                            async move |app_state_99328dfe: axum::extract::State<
                                std::sync::Arc<dyn pg_crud::CombinationOfAppStateLogicTraits>,
                            >,
                                        req: axum::extract::Request| {
                                Self::dlo_h(app_state_99328dfe, req, &tbl_owned).await
                            }
                        }),
                    )
                    .route(
                        "/dlo_payload_example",
                        axum::routing::get(async move || Self::dlo_payload_example()),
                    )
                    .with_state(app_state),
            )
        }
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
    pub struct TblExampleCmPayload(pub Vec<TblExampleCr>);
    impl pg_crud::DfltSomeOneEl for TblExampleCmPayload {
        fn dflt_some_one_el() -> Self {
            Self(vec![pg_crud::DfltSomeOneEl::dflt_some_one_el()])
        }
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug)]
    pub struct TblExampleCmPrms {
        pub payload: TblExampleCmPayload,
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug, serde :: Serialize, serde :: Deserialize)]
    pub enum TblExampleCmResVrts {
        Desirable(Vec<TblExampleRdIds>),
        CheckBodySize {
            check_body_size: pg_crud::check_body_size::BodySizeErWithSerde,
            loc: loc_lib::loc::Loc,
        },
        Pg {
            pg: String,
            loc: loc_lib::loc::Loc,
        },
        SerdeJson {
            serde_json: String,
            loc: loc_lib::loc::Loc,
        },
        HeaderContentTypeAppJsonNotFound {
            loc: loc_lib::loc::Loc,
        },
        CheckCommit {
            check_commit: pg_crud::check_commit::CommitErWithSerde,
            loc: loc_lib::loc::Loc,
        },
        Qp {
            er: pg_crud::QpErWithSerde,
            loc: loc_lib::loc::Loc,
        },
        RowAndRollback {
            row: String,
            rollback: String,
            loc: loc_lib::loc::Loc,
        },
        TryBind {
            try_bind: String,
            loc: loc_lib::loc::Loc,
        },
    }
    impl TblExampleCmResVrts {
        fn from_h(v: TblExampleCmEr) -> Self {
            match v.into_serde_version() {
                TblExampleCmErWithSerde::CheckBodySize {
                    check_body_size,
                    loc,
                } => Self::CheckBodySize {
                    check_body_size,
                    loc,
                },
                TblExampleCmErWithSerde::Pg { pg, loc } => Self::Pg { pg, loc },
                TblExampleCmErWithSerde::SerdeJson { serde_json, loc } => {
                    Self::SerdeJson { serde_json, loc }
                }
                TblExampleCmErWithSerde::HeaderContentTypeAppJsonNotFound { loc } => {
                    Self::HeaderContentTypeAppJsonNotFound { loc }
                }
                TblExampleCmErWithSerde::CheckCommit { check_commit, loc } => {
                    Self::CheckCommit { check_commit, loc }
                }
                TblExampleCmErWithSerde::Qp { er, loc } => Self::Qp { er, loc },
                TblExampleCmErWithSerde::RowAndRollback { row, rollback, loc } => {
                    Self::RowAndRollback { row, rollback, loc }
                }
                TblExampleCmErWithSerde::TryBind { try_bind, loc } => {
                    Self::TryBind { try_bind, loc }
                }
            }
        }
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug, thiserror :: Error, loc_lib :: Location)]
    pub enum TblExampleCmEr {
        CheckBodySize {
            #[eo_loc]
            check_body_size: pg_crud::check_body_size::BodySizeEr,
            loc: loc_lib::loc::Loc,
        },
        Pg {
            #[eo_to_err_string]
            pg: sqlx::Error,
            loc: loc_lib::loc::Loc,
        },
        SerdeJson {
            #[eo_to_err_string]
            serde_json: serde_json::Error,
            loc: loc_lib::loc::Loc,
        },
        HeaderContentTypeAppJsonNotFound {
            loc: loc_lib::loc::Loc,
        },
        CheckCommit {
            #[eo_loc]
            check_commit: pg_crud::check_commit::CommitEr,
            loc: loc_lib::loc::Loc,
        },
        Qp {
            #[eo_loc]
            er: pg_crud::QpEr,
            loc: loc_lib::loc::Loc,
        },
        RowAndRollback {
            #[eo_to_err_string]
            row: sqlx::Error,
            #[eo_to_err_string]
            rollback: sqlx::Error,
            loc: loc_lib::loc::Loc,
        },
        TryBind {
            #[eo_to_err_string_serde]
            try_bind: String,
            loc: loc_lib::loc::Loc,
        },
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug, thiserror :: Error, loc_lib :: Location)]
    pub enum TblExampleTryCmEr {
        SerdeJsonToString {
            #[eo_to_err_string]
            serde_json_to_string: serde_json::Error,
            loc: loc_lib::loc::Loc,
        },
        FailedToGetResText {
            #[eo_to_err_string]
            status_code: reqwest::StatusCode,
            #[eo_to_err_string]
            headers: reqwest::header::HeaderMap,
            #[eo_to_err_string]
            reqwest: reqwest::Error,
            loc: loc_lib::loc::Loc,
        },
        DeRes {
            #[eo_to_err_string]
            status_code: reqwest::StatusCode,
            #[eo_to_err_string]
            headers: reqwest::header::HeaderMap,
            #[eo_to_err_string_serde]
            res_text: String,
            #[eo_to_err_string]
            serde: serde_json::Error,
            loc: loc_lib::loc::Loc,
        },
        Reqwest {
            #[eo_to_err_string]
            reqwest: reqwest::Error,
            loc: loc_lib::loc::Loc,
        },
        TblExampleCmErWithSerde {
            #[eo_to_err_string]
            cm_er_with_serde: TblExampleCmErWithSerde,
            loc: loc_lib::loc::Loc,
        },
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug)]
    pub struct TblExampleCoPrms {
        pub payload: TblExampleCr,
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug, serde :: Serialize, serde :: Deserialize)]
    pub enum TblExampleCoResVrts {
        Desirable(TblExampleRdIds),
        CheckBodySize {
            check_body_size: pg_crud::check_body_size::BodySizeErWithSerde,
            loc: loc_lib::loc::Loc,
        },
        Pg {
            pg: String,
            loc: loc_lib::loc::Loc,
        },
        SerdeJson {
            serde_json: String,
            loc: loc_lib::loc::Loc,
        },
        HeaderContentTypeAppJsonNotFound {
            loc: loc_lib::loc::Loc,
        },
        CheckCommit {
            check_commit: pg_crud::check_commit::CommitErWithSerde,
            loc: loc_lib::loc::Loc,
        },
        Qp {
            er: pg_crud::QpErWithSerde,
            loc: loc_lib::loc::Loc,
        },
        RowAndRollback {
            row: String,
            rollback: String,
            loc: loc_lib::loc::Loc,
        },
        TryBind {
            try_bind: String,
            loc: loc_lib::loc::Loc,
        },
    }
    impl TblExampleCoResVrts {
        fn from_h(v: TblExampleCoEr) -> Self {
            match v.into_serde_version() {
                TblExampleCoErWithSerde::CheckBodySize {
                    check_body_size,
                    loc,
                } => Self::CheckBodySize {
                    check_body_size,
                    loc,
                },
                TblExampleCoErWithSerde::Pg { pg, loc } => Self::Pg { pg, loc },
                TblExampleCoErWithSerde::SerdeJson { serde_json, loc } => {
                    Self::SerdeJson { serde_json, loc }
                }
                TblExampleCoErWithSerde::HeaderContentTypeAppJsonNotFound { loc } => {
                    Self::HeaderContentTypeAppJsonNotFound { loc }
                }
                TblExampleCoErWithSerde::CheckCommit { check_commit, loc } => {
                    Self::CheckCommit { check_commit, loc }
                }
                TblExampleCoErWithSerde::Qp { er, loc } => Self::Qp { er, loc },
                TblExampleCoErWithSerde::RowAndRollback { row, rollback, loc } => {
                    Self::RowAndRollback { row, rollback, loc }
                }
                TblExampleCoErWithSerde::TryBind { try_bind, loc } => {
                    Self::TryBind { try_bind, loc }
                }
            }
        }
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug, thiserror :: Error, loc_lib :: Location)]
    pub enum TblExampleCoEr {
        CheckBodySize {
            #[eo_loc]
            check_body_size: pg_crud::check_body_size::BodySizeEr,
            loc: loc_lib::loc::Loc,
        },
        Pg {
            #[eo_to_err_string]
            pg: sqlx::Error,
            loc: loc_lib::loc::Loc,
        },
        SerdeJson {
            #[eo_to_err_string]
            serde_json: serde_json::Error,
            loc: loc_lib::loc::Loc,
        },
        HeaderContentTypeAppJsonNotFound {
            loc: loc_lib::loc::Loc,
        },
        CheckCommit {
            #[eo_loc]
            check_commit: pg_crud::check_commit::CommitEr,
            loc: loc_lib::loc::Loc,
        },
        Qp {
            #[eo_loc]
            er: pg_crud::QpEr,
            loc: loc_lib::loc::Loc,
        },
        RowAndRollback {
            #[eo_to_err_string]
            row: sqlx::Error,
            #[eo_to_err_string]
            rollback: sqlx::Error,
            loc: loc_lib::loc::Loc,
        },
        TryBind {
            #[eo_to_err_string_serde]
            try_bind: String,
            loc: loc_lib::loc::Loc,
        },
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug, thiserror :: Error, loc_lib :: Location)]
    pub enum TblExampleTryCoEr {
        SerdeJsonToString {
            #[eo_to_err_string]
            serde_json_to_string: serde_json::Error,
            loc: loc_lib::loc::Loc,
        },
        FailedToGetResText {
            #[eo_to_err_string]
            status_code: reqwest::StatusCode,
            #[eo_to_err_string]
            headers: reqwest::header::HeaderMap,
            #[eo_to_err_string]
            reqwest: reqwest::Error,
            loc: loc_lib::loc::Loc,
        },
        DeRes {
            #[eo_to_err_string]
            status_code: reqwest::StatusCode,
            #[eo_to_err_string]
            headers: reqwest::header::HeaderMap,
            #[eo_to_err_string_serde]
            res_text: String,
            #[eo_to_err_string]
            serde: serde_json::Error,
            loc: loc_lib::loc::Loc,
        },
        Reqwest {
            #[eo_to_err_string]
            reqwest: reqwest::Error,
            loc: loc_lib::loc::Loc,
        },
        TblExampleCoErWithSerde {
            #[eo_to_err_string]
            co_er_with_serde: TblExampleCoErWithSerde,
            loc: loc_lib::loc::Loc,
        },
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
    pub struct TblExampleRmPayload {
        pub wh_many: StdOptOptTblExampleWhMany,
        pub sel: pg_crud::NotEmptyUnqVec<TblExampleSel>,
        pub order_by: pg_crud::OrderBy<TblExampleSel>,
        pub pgn: pg_crud::PgnStartsWithZero,
    }
    impl pg_crud::DfltSomeOneEl for TblExampleRmPayload {
        fn dflt_some_one_el() -> Self {
            Self {
                wh_many: pg_crud::DfltSomeOneEl::dflt_some_one_el(),
                sel: pg_crud::DfltSomeOneEl::dflt_some_one_el(),
                order_by: pg_crud::OrderBy {
                    col: TblExampleSel::PkCol(pg_crud::DfltSomeOneEl::dflt_some_one_el()),
                    order: Some(pg_crud::DfltSomeOneEl::dflt_some_one_el()),
                },
                pgn: pg_crud::DfltSomeOneEl::dflt_some_one_el(),
            }
        }
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug)]
    pub struct TblExampleRmPrms {
        pub payload: TblExampleRmPayload,
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug, serde :: Serialize, serde :: Deserialize)]
    pub enum TblExampleRmResVrts {
        Desirable(Vec<TblExampleRd>),
        CheckBodySize {
            check_body_size: pg_crud::check_body_size::BodySizeErWithSerde,
            loc: loc_lib::loc::Loc,
        },
        Pg {
            pg: String,
            loc: loc_lib::loc::Loc,
        },
        SerdeJson {
            serde_json: String,
            loc: loc_lib::loc::Loc,
        },
        HeaderContentTypeAppJsonNotFound {
            loc: loc_lib::loc::Loc,
        },
        CheckCommit {
            check_commit: pg_crud::check_commit::CommitErWithSerde,
            loc: loc_lib::loc::Loc,
        },
        NotUnqField {
            loc: loc_lib::loc::Loc,
            not_unq_field: TblExampleSel,
        },
        Qp {
            er: pg_crud::QpErWithSerde,
            loc: loc_lib::loc::Loc,
        },
        TryBind {
            try_bind: String,
            loc: loc_lib::loc::Loc,
        },
    }
    impl TblExampleRmResVrts {
        fn from_h(v: TblExampleRmEr) -> Self {
            match v.into_serde_version() {
                TblExampleRmErWithSerde::CheckBodySize {
                    check_body_size,
                    loc,
                } => Self::CheckBodySize {
                    check_body_size,
                    loc,
                },
                TblExampleRmErWithSerde::Pg { pg, loc } => Self::Pg { pg, loc },
                TblExampleRmErWithSerde::SerdeJson { serde_json, loc } => {
                    Self::SerdeJson { serde_json, loc }
                }
                TblExampleRmErWithSerde::HeaderContentTypeAppJsonNotFound { loc } => {
                    Self::HeaderContentTypeAppJsonNotFound { loc }
                }
                TblExampleRmErWithSerde::CheckCommit { check_commit, loc } => {
                    Self::CheckCommit { check_commit, loc }
                }
                TblExampleRmErWithSerde::NotUnqField { loc, not_unq_field } => {
                    Self::NotUnqField { loc, not_unq_field }
                }
                TblExampleRmErWithSerde::Qp { er, loc } => Self::Qp { er, loc },
                TblExampleRmErWithSerde::TryBind { try_bind, loc } => {
                    Self::TryBind { try_bind, loc }
                }
            }
        }
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug, thiserror :: Error, loc_lib :: Location)]
    pub enum TblExampleRmEr {
        CheckBodySize {
            #[eo_loc]
            check_body_size: pg_crud::check_body_size::BodySizeEr,
            loc: loc_lib::loc::Loc,
        },
        Pg {
            #[eo_to_err_string]
            pg: sqlx::Error,
            loc: loc_lib::loc::Loc,
        },
        SerdeJson {
            #[eo_to_err_string]
            serde_json: serde_json::Error,
            loc: loc_lib::loc::Loc,
        },
        HeaderContentTypeAppJsonNotFound {
            loc: loc_lib::loc::Loc,
        },
        CheckCommit {
            #[eo_loc]
            check_commit: pg_crud::check_commit::CommitEr,
            loc: loc_lib::loc::Loc,
        },
        NotUnqField {
            loc: loc_lib::loc::Loc,
            #[eo_to_err_string_serde]
            not_unq_field: TblExampleSel,
        },
        Qp {
            #[eo_loc]
            er: pg_crud::QpEr,
            loc: loc_lib::loc::Loc,
        },
        TryBind {
            #[eo_to_err_string_serde]
            try_bind: String,
            loc: loc_lib::loc::Loc,
        },
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug, thiserror :: Error, loc_lib :: Location)]
    pub enum TblExampleTryRmEr {
        SerdeJsonToString {
            #[eo_to_err_string]
            serde_json_to_string: serde_json::Error,
            loc: loc_lib::loc::Loc,
        },
        FailedToGetResText {
            #[eo_to_err_string]
            status_code: reqwest::StatusCode,
            #[eo_to_err_string]
            headers: reqwest::header::HeaderMap,
            #[eo_to_err_string]
            reqwest: reqwest::Error,
            loc: loc_lib::loc::Loc,
        },
        DeRes {
            #[eo_to_err_string]
            status_code: reqwest::StatusCode,
            #[eo_to_err_string]
            headers: reqwest::header::HeaderMap,
            #[eo_to_err_string_serde]
            res_text: String,
            #[eo_to_err_string]
            serde: serde_json::Error,
            loc: loc_lib::loc::Loc,
        },
        Reqwest {
            #[eo_to_err_string]
            reqwest: reqwest::Error,
            loc: loc_lib::loc::Loc,
        },
        NotUnqField {
            loc: loc_lib::loc::Loc,
            #[eo_to_err_string_serde]
            not_unq_field: TblExampleSel,
        },
        TblExampleRmErWithSerde {
            #[eo_to_err_string]
            rm_er_with_serde: TblExampleRmErWithSerde,
            loc: loc_lib::loc::Loc,
        },
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
    pub struct TblExampleRoPayload {
        pub pk_col: pg_crud::SqlxTypesUuidUuidAsNnUuidV4InitByPgRd,
        pub sel: pg_crud::NotEmptyUnqVec<TblExampleSel>,
    }
    impl pg_crud::DfltSomeOneEl for TblExampleRoPayload {
        fn dflt_some_one_el() -> Self {
            Self {
                pk_col: pg_crud::DfltSomeOneEl::dflt_some_one_el(),
                sel: pg_crud::DfltSomeOneEl::dflt_some_one_el(),
            }
        }
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug)]
    pub struct TblExampleRoPrms {
        pub payload: TblExampleRoPayload,
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug, serde :: Serialize, serde :: Deserialize)]
    pub enum TblExampleRoResVrts {
        Desirable(TblExampleRd),
        CheckBodySize {
            check_body_size: pg_crud::check_body_size::BodySizeErWithSerde,
            loc: loc_lib::loc::Loc,
        },
        Pg {
            pg: String,
            loc: loc_lib::loc::Loc,
        },
        SerdeJson {
            serde_json: String,
            loc: loc_lib::loc::Loc,
        },
        HeaderContentTypeAppJsonNotFound {
            loc: loc_lib::loc::Loc,
        },
        CheckCommit {
            check_commit: pg_crud::check_commit::CommitErWithSerde,
            loc: loc_lib::loc::Loc,
        },
        NotUnqField {
            loc: loc_lib::loc::Loc,
            not_unq_field: TblExampleSel,
        },
        Qp {
            er: pg_crud::QpErWithSerde,
            loc: loc_lib::loc::Loc,
        },
        TryBind {
            try_bind: String,
            loc: loc_lib::loc::Loc,
        },
    }
    impl TblExampleRoResVrts {
        fn from_h(v: TblExampleRoEr) -> Self {
            match v.into_serde_version() {
                TblExampleRoErWithSerde::CheckBodySize {
                    check_body_size,
                    loc,
                } => Self::CheckBodySize {
                    check_body_size,
                    loc,
                },
                TblExampleRoErWithSerde::Pg { pg, loc } => Self::Pg { pg, loc },
                TblExampleRoErWithSerde::SerdeJson { serde_json, loc } => {
                    Self::SerdeJson { serde_json, loc }
                }
                TblExampleRoErWithSerde::HeaderContentTypeAppJsonNotFound { loc } => {
                    Self::HeaderContentTypeAppJsonNotFound { loc }
                }
                TblExampleRoErWithSerde::CheckCommit { check_commit, loc } => {
                    Self::CheckCommit { check_commit, loc }
                }
                TblExampleRoErWithSerde::NotUnqField { loc, not_unq_field } => {
                    Self::NotUnqField { loc, not_unq_field }
                }
                TblExampleRoErWithSerde::Qp { er, loc } => Self::Qp { er, loc },
                TblExampleRoErWithSerde::TryBind { try_bind, loc } => {
                    Self::TryBind { try_bind, loc }
                }
            }
        }
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug, thiserror :: Error, loc_lib :: Location)]
    pub enum TblExampleRoEr {
        CheckBodySize {
            #[eo_loc]
            check_body_size: pg_crud::check_body_size::BodySizeEr,
            loc: loc_lib::loc::Loc,
        },
        Pg {
            #[eo_to_err_string]
            pg: sqlx::Error,
            loc: loc_lib::loc::Loc,
        },
        SerdeJson {
            #[eo_to_err_string]
            serde_json: serde_json::Error,
            loc: loc_lib::loc::Loc,
        },
        HeaderContentTypeAppJsonNotFound {
            loc: loc_lib::loc::Loc,
        },
        CheckCommit {
            #[eo_loc]
            check_commit: pg_crud::check_commit::CommitEr,
            loc: loc_lib::loc::Loc,
        },
        NotUnqField {
            loc: loc_lib::loc::Loc,
            #[eo_to_err_string_serde]
            not_unq_field: TblExampleSel,
        },
        Qp {
            #[eo_loc]
            er: pg_crud::QpEr,
            loc: loc_lib::loc::Loc,
        },
        TryBind {
            #[eo_to_err_string_serde]
            try_bind: String,
            loc: loc_lib::loc::Loc,
        },
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug, thiserror :: Error, loc_lib :: Location)]
    pub enum TblExampleTryRoEr {
        SerdeJsonToString {
            #[eo_to_err_string]
            serde_json_to_string: serde_json::Error,
            loc: loc_lib::loc::Loc,
        },
        FailedToGetResText {
            #[eo_to_err_string]
            status_code: reqwest::StatusCode,
            #[eo_to_err_string]
            headers: reqwest::header::HeaderMap,
            #[eo_to_err_string]
            reqwest: reqwest::Error,
            loc: loc_lib::loc::Loc,
        },
        DeRes {
            #[eo_to_err_string]
            status_code: reqwest::StatusCode,
            #[eo_to_err_string]
            headers: reqwest::header::HeaderMap,
            #[eo_to_err_string_serde]
            res_text: String,
            #[eo_to_err_string]
            serde: serde_json::Error,
            loc: loc_lib::loc::Loc,
        },
        Reqwest {
            #[eo_to_err_string]
            reqwest: reqwest::Error,
            loc: loc_lib::loc::Loc,
        },
        NotUnqField {
            loc: loc_lib::loc::Loc,
            #[eo_to_err_string_serde]
            not_unq_field: TblExampleSel,
        },
        TblExampleRoErWithSerde {
            #[eo_to_err_string]
            ro_er_with_serde: TblExampleRoErWithSerde,
            loc: loc_lib::loc::Loc,
        },
    }
    #[derive(Debug, serde :: Serialize, utoipa :: ToSchema)]
    pub struct TblExampleUmPayload(Vec<TblExampleUpd>);
    #[derive(Debug, thiserror :: Error, loc_lib :: Location)]
    pub enum TblExampleUmPayloadTryNewEr {
        NotUnqPk {
            #[eo_to_err_string]
            not_unq_pk: pg_crud::SqlxTypesUuidUuidAsNnUuidV4InitByPgUpd,
            #[eo_to_err_string]
            loc: loc_lib::loc::Loc,
        },
    }
    impl TblExampleUmPayload {
        pub fn try_new(v: Vec<TblExampleUpd>) -> Result<Self, TblExampleUmPayloadTryNewEr> {
            let mut acc_6bf275fc = Vec::new();
            for el_35facc3a in &v {
                if acc_6bf275fc.contains(&&el_35facc3a.pk_col) {
                    return Err(TblExampleUmPayloadTryNewEr::NotUnqPk {
                        not_unq_pk: el_35facc3a.pk_col,
                        loc: loc_lib::loc!(),
                    });
                }
                acc_6bf275fc.push(&el_35facc3a.pk_col);
            }
            Ok(Self(v))
        }
    }
    #[allow(unused_qualifications)]
    #[allow(clippy::absolute_paths)]
    #[allow(clippy::arbitrary_source_item_ordering)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for TblExampleUmPayload {
            fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                let raw = <Vec<TblExampleUpd> as _serde::Deserialize>::deserialize(__deserializer)?;
                Self::try_new(raw).map_err(|er| _serde::de::Error::custom(format!("{er:?}")))
            }
        }
    };
    impl pg_crud::DfltSomeOneEl for TblExampleUmPayload {
        fn dflt_some_one_el() -> Self {
            Self(vec![pg_crud::DfltSomeOneEl::dflt_some_one_el()])
        }
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug)]
    pub struct TblExampleUmPrms {
        pub payload: TblExampleUmPayload,
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug, serde :: Serialize, serde :: Deserialize)]
    pub enum TblExampleUmResVrts {
        Desirable(Vec<TblExampleRdIds>),
        CheckBodySize {
            check_body_size: pg_crud::check_body_size::BodySizeErWithSerde,
            loc: loc_lib::loc::Loc,
        },
        Pg {
            pg: String,
            loc: loc_lib::loc::Loc,
        },
        SerdeJson {
            serde_json: String,
            loc: loc_lib::loc::Loc,
        },
        HeaderContentTypeAppJsonNotFound {
            loc: loc_lib::loc::Loc,
        },
        CheckCommit {
            check_commit: pg_crud::check_commit::CommitErWithSerde,
            loc: loc_lib::loc::Loc,
        },
        Qp {
            er: pg_crud::QpErWithSerde,
            loc: loc_lib::loc::Loc,
        },
        RowAndRollback {
            row: String,
            rollback: String,
            loc: loc_lib::loc::Loc,
        },
        TryBind {
            try_bind: String,
            loc: loc_lib::loc::Loc,
        },
    }
    impl TblExampleUmResVrts {
        fn from_h(v: TblExampleUmEr) -> Self {
            match v.into_serde_version() {
                TblExampleUmErWithSerde::CheckBodySize {
                    check_body_size,
                    loc,
                } => Self::CheckBodySize {
                    check_body_size,
                    loc,
                },
                TblExampleUmErWithSerde::Pg { pg, loc } => Self::Pg { pg, loc },
                TblExampleUmErWithSerde::SerdeJson { serde_json, loc } => {
                    Self::SerdeJson { serde_json, loc }
                }
                TblExampleUmErWithSerde::HeaderContentTypeAppJsonNotFound { loc } => {
                    Self::HeaderContentTypeAppJsonNotFound { loc }
                }
                TblExampleUmErWithSerde::CheckCommit { check_commit, loc } => {
                    Self::CheckCommit { check_commit, loc }
                }
                TblExampleUmErWithSerde::Qp { er, loc } => Self::Qp { er, loc },
                TblExampleUmErWithSerde::RowAndRollback { row, rollback, loc } => {
                    Self::RowAndRollback { row, rollback, loc }
                }
                TblExampleUmErWithSerde::TryBind { try_bind, loc } => {
                    Self::TryBind { try_bind, loc }
                }
            }
        }
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug, thiserror :: Error, loc_lib :: Location)]
    pub enum TblExampleUmEr {
        CheckBodySize {
            #[eo_loc]
            check_body_size: pg_crud::check_body_size::BodySizeEr,
            loc: loc_lib::loc::Loc,
        },
        Pg {
            #[eo_to_err_string]
            pg: sqlx::Error,
            loc: loc_lib::loc::Loc,
        },
        SerdeJson {
            #[eo_to_err_string]
            serde_json: serde_json::Error,
            loc: loc_lib::loc::Loc,
        },
        HeaderContentTypeAppJsonNotFound {
            loc: loc_lib::loc::Loc,
        },
        CheckCommit {
            #[eo_loc]
            check_commit: pg_crud::check_commit::CommitEr,
            loc: loc_lib::loc::Loc,
        },
        Qp {
            #[eo_loc]
            er: pg_crud::QpEr,
            loc: loc_lib::loc::Loc,
        },
        RowAndRollback {
            #[eo_to_err_string]
            row: sqlx::Error,
            #[eo_to_err_string]
            rollback: sqlx::Error,
            loc: loc_lib::loc::Loc,
        },
        TryBind {
            #[eo_to_err_string_serde]
            try_bind: String,
            loc: loc_lib::loc::Loc,
        },
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug, thiserror :: Error, loc_lib :: Location)]
    pub enum TblExampleTryUmEr {
        SerdeJsonToString {
            #[eo_to_err_string]
            serde_json_to_string: serde_json::Error,
            loc: loc_lib::loc::Loc,
        },
        FailedToGetResText {
            #[eo_to_err_string]
            status_code: reqwest::StatusCode,
            #[eo_to_err_string]
            headers: reqwest::header::HeaderMap,
            #[eo_to_err_string]
            reqwest: reqwest::Error,
            loc: loc_lib::loc::Loc,
        },
        DeRes {
            #[eo_to_err_string]
            status_code: reqwest::StatusCode,
            #[eo_to_err_string]
            headers: reqwest::header::HeaderMap,
            #[eo_to_err_string_serde]
            res_text: String,
            #[eo_to_err_string]
            serde: serde_json::Error,
            loc: loc_lib::loc::Loc,
        },
        Reqwest {
            #[eo_to_err_string]
            reqwest: reqwest::Error,
            loc: loc_lib::loc::Loc,
        },
        TblExampleUmErWithSerde {
            #[eo_to_err_string]
            um_er_with_serde: TblExampleUmErWithSerde,
            loc: loc_lib::loc::Loc,
        },
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug)]
    pub struct TblExampleUoPrms {
        pub payload: TblExampleUpd,
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug, serde :: Serialize, serde :: Deserialize)]
    pub enum TblExampleUoResVrts {
        Desirable(TblExampleRdIds),
        CheckBodySize {
            check_body_size: pg_crud::check_body_size::BodySizeErWithSerde,
            loc: loc_lib::loc::Loc,
        },
        Pg {
            pg: String,
            loc: loc_lib::loc::Loc,
        },
        SerdeJson {
            serde_json: String,
            loc: loc_lib::loc::Loc,
        },
        HeaderContentTypeAppJsonNotFound {
            loc: loc_lib::loc::Loc,
        },
        CheckCommit {
            check_commit: pg_crud::check_commit::CommitErWithSerde,
            loc: loc_lib::loc::Loc,
        },
        Qp {
            er: pg_crud::QpErWithSerde,
            loc: loc_lib::loc::Loc,
        },
        RowAndRollback {
            row: String,
            rollback: String,
            loc: loc_lib::loc::Loc,
        },
        TryBind {
            try_bind: String,
            loc: loc_lib::loc::Loc,
        },
    }
    impl TblExampleUoResVrts {
        fn from_h(v: TblExampleUoEr) -> Self {
            match v.into_serde_version() {
                TblExampleUoErWithSerde::CheckBodySize {
                    check_body_size,
                    loc,
                } => Self::CheckBodySize {
                    check_body_size,
                    loc,
                },
                TblExampleUoErWithSerde::Pg { pg, loc } => Self::Pg { pg, loc },
                TblExampleUoErWithSerde::SerdeJson { serde_json, loc } => {
                    Self::SerdeJson { serde_json, loc }
                }
                TblExampleUoErWithSerde::HeaderContentTypeAppJsonNotFound { loc } => {
                    Self::HeaderContentTypeAppJsonNotFound { loc }
                }
                TblExampleUoErWithSerde::CheckCommit { check_commit, loc } => {
                    Self::CheckCommit { check_commit, loc }
                }
                TblExampleUoErWithSerde::Qp { er, loc } => Self::Qp { er, loc },
                TblExampleUoErWithSerde::RowAndRollback { row, rollback, loc } => {
                    Self::RowAndRollback { row, rollback, loc }
                }
                TblExampleUoErWithSerde::TryBind { try_bind, loc } => {
                    Self::TryBind { try_bind, loc }
                }
            }
        }
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug, thiserror :: Error, loc_lib :: Location)]
    pub enum TblExampleUoEr {
        CheckBodySize {
            #[eo_loc]
            check_body_size: pg_crud::check_body_size::BodySizeEr,
            loc: loc_lib::loc::Loc,
        },
        Pg {
            #[eo_to_err_string]
            pg: sqlx::Error,
            loc: loc_lib::loc::Loc,
        },
        SerdeJson {
            #[eo_to_err_string]
            serde_json: serde_json::Error,
            loc: loc_lib::loc::Loc,
        },
        HeaderContentTypeAppJsonNotFound {
            loc: loc_lib::loc::Loc,
        },
        CheckCommit {
            #[eo_loc]
            check_commit: pg_crud::check_commit::CommitEr,
            loc: loc_lib::loc::Loc,
        },
        Qp {
            #[eo_loc]
            er: pg_crud::QpEr,
            loc: loc_lib::loc::Loc,
        },
        RowAndRollback {
            #[eo_to_err_string]
            row: sqlx::Error,
            #[eo_to_err_string]
            rollback: sqlx::Error,
            loc: loc_lib::loc::Loc,
        },
        TryBind {
            #[eo_to_err_string_serde]
            try_bind: String,
            loc: loc_lib::loc::Loc,
        },
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug, thiserror :: Error, loc_lib :: Location)]
    pub enum TblExampleTryUoEr {
        SerdeJsonToString {
            #[eo_to_err_string]
            serde_json_to_string: serde_json::Error,
            loc: loc_lib::loc::Loc,
        },
        FailedToGetResText {
            #[eo_to_err_string]
            status_code: reqwest::StatusCode,
            #[eo_to_err_string]
            headers: reqwest::header::HeaderMap,
            #[eo_to_err_string]
            reqwest: reqwest::Error,
            loc: loc_lib::loc::Loc,
        },
        DeRes {
            #[eo_to_err_string]
            status_code: reqwest::StatusCode,
            #[eo_to_err_string]
            headers: reqwest::header::HeaderMap,
            #[eo_to_err_string_serde]
            res_text: String,
            #[eo_to_err_string]
            serde: serde_json::Error,
            loc: loc_lib::loc::Loc,
        },
        Reqwest {
            #[eo_to_err_string]
            reqwest: reqwest::Error,
            loc: loc_lib::loc::Loc,
        },
        TblExampleUoErWithSerde {
            #[eo_to_err_string]
            uo_er_with_serde: TblExampleUoErWithSerde,
            loc: loc_lib::loc::Loc,
        },
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
    pub struct TblExampleDmPayload {
        pub wh_many: StdOptOptTblExampleWhMany,
    }
    impl pg_crud::DfltSomeOneEl for TblExampleDmPayload {
        fn dflt_some_one_el() -> Self {
            Self {
                wh_many: pg_crud::DfltSomeOneEl::dflt_some_one_el(),
            }
        }
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug)]
    pub struct TblExampleDmPrms {
        pub payload: TblExampleDmPayload,
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug, serde :: Serialize, serde :: Deserialize)]
    pub enum TblExampleDmResVrts {
        Desirable(Vec<<pg_crud::SqlxTypesUuidUuidAsNnUuidV4InitByPg as pg_crud::PgType>::Rd>),
        CheckBodySize {
            check_body_size: pg_crud::check_body_size::BodySizeErWithSerde,
            loc: loc_lib::loc::Loc,
        },
        Pg {
            pg: String,
            loc: loc_lib::loc::Loc,
        },
        SerdeJson {
            serde_json: String,
            loc: loc_lib::loc::Loc,
        },
        HeaderContentTypeAppJsonNotFound {
            loc: loc_lib::loc::Loc,
        },
        CheckCommit {
            check_commit: pg_crud::check_commit::CommitErWithSerde,
            loc: loc_lib::loc::Loc,
        },
        Qp {
            er: pg_crud::QpErWithSerde,
            loc: loc_lib::loc::Loc,
        },
        RowAndRollback {
            row: String,
            rollback: String,
            loc: loc_lib::loc::Loc,
        },
        TryBind {
            try_bind: String,
            loc: loc_lib::loc::Loc,
        },
    }
    impl TblExampleDmResVrts {
        fn from_h(v: TblExampleDmEr) -> Self {
            match v.into_serde_version() {
                TblExampleDmErWithSerde::CheckBodySize {
                    check_body_size,
                    loc,
                } => Self::CheckBodySize {
                    check_body_size,
                    loc,
                },
                TblExampleDmErWithSerde::Pg { pg, loc } => Self::Pg { pg, loc },
                TblExampleDmErWithSerde::SerdeJson { serde_json, loc } => {
                    Self::SerdeJson { serde_json, loc }
                }
                TblExampleDmErWithSerde::HeaderContentTypeAppJsonNotFound { loc } => {
                    Self::HeaderContentTypeAppJsonNotFound { loc }
                }
                TblExampleDmErWithSerde::CheckCommit { check_commit, loc } => {
                    Self::CheckCommit { check_commit, loc }
                }
                TblExampleDmErWithSerde::Qp { er, loc } => Self::Qp { er, loc },
                TblExampleDmErWithSerde::RowAndRollback { row, rollback, loc } => {
                    Self::RowAndRollback { row, rollback, loc }
                }
                TblExampleDmErWithSerde::TryBind { try_bind, loc } => {
                    Self::TryBind { try_bind, loc }
                }
            }
        }
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug, thiserror :: Error, loc_lib :: Location)]
    pub enum TblExampleDmEr {
        CheckBodySize {
            #[eo_loc]
            check_body_size: pg_crud::check_body_size::BodySizeEr,
            loc: loc_lib::loc::Loc,
        },
        Pg {
            #[eo_to_err_string]
            pg: sqlx::Error,
            loc: loc_lib::loc::Loc,
        },
        SerdeJson {
            #[eo_to_err_string]
            serde_json: serde_json::Error,
            loc: loc_lib::loc::Loc,
        },
        HeaderContentTypeAppJsonNotFound {
            loc: loc_lib::loc::Loc,
        },
        CheckCommit {
            #[eo_loc]
            check_commit: pg_crud::check_commit::CommitEr,
            loc: loc_lib::loc::Loc,
        },
        Qp {
            #[eo_loc]
            er: pg_crud::QpEr,
            loc: loc_lib::loc::Loc,
        },
        RowAndRollback {
            #[eo_to_err_string]
            row: sqlx::Error,
            #[eo_to_err_string]
            rollback: sqlx::Error,
            loc: loc_lib::loc::Loc,
        },
        TryBind {
            #[eo_to_err_string_serde]
            try_bind: String,
            loc: loc_lib::loc::Loc,
        },
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug, thiserror :: Error, loc_lib :: Location)]
    pub enum TblExampleTryDmEr {
        SerdeJsonToString {
            #[eo_to_err_string]
            serde_json_to_string: serde_json::Error,
            loc: loc_lib::loc::Loc,
        },
        FailedToGetResText {
            #[eo_to_err_string]
            status_code: reqwest::StatusCode,
            #[eo_to_err_string]
            headers: reqwest::header::HeaderMap,
            #[eo_to_err_string]
            reqwest: reqwest::Error,
            loc: loc_lib::loc::Loc,
        },
        DeRes {
            #[eo_to_err_string]
            status_code: reqwest::StatusCode,
            #[eo_to_err_string]
            headers: reqwest::header::HeaderMap,
            #[eo_to_err_string_serde]
            res_text: String,
            #[eo_to_err_string]
            serde: serde_json::Error,
            loc: loc_lib::loc::Loc,
        },
        Reqwest {
            #[eo_to_err_string]
            reqwest: reqwest::Error,
            loc: loc_lib::loc::Loc,
        },
        TblExampleDmErWithSerde {
            #[eo_to_err_string]
            dm_er_with_serde: TblExampleDmErWithSerde,
            loc: loc_lib::loc::Loc,
        },
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug, Clone, Copy, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
    pub struct TblExampleDloPayload {
        pub pk_col: pg_crud::SqlxTypesUuidUuidAsNnUuidV4InitByPgRd,
    }
    impl pg_crud::DfltSomeOneEl for TblExampleDloPayload {
        fn dflt_some_one_el() -> Self {
            Self {
                pk_col: pg_crud::DfltSomeOneEl::dflt_some_one_el(),
            }
        }
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug, Clone, Copy)]
    pub struct TblExampleDloPrms {
        pub payload: TblExampleDloPayload,
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug, serde :: Serialize, serde :: Deserialize)]
    pub enum TblExampleDloResVrts {
        Desirable(<pg_crud::SqlxTypesUuidUuidAsNnUuidV4InitByPg as pg_crud::PgType>::Rd),
        CheckBodySize {
            check_body_size: pg_crud::check_body_size::BodySizeErWithSerde,
            loc: loc_lib::loc::Loc,
        },
        Pg {
            pg: String,
            loc: loc_lib::loc::Loc,
        },
        SerdeJson {
            serde_json: String,
            loc: loc_lib::loc::Loc,
        },
        HeaderContentTypeAppJsonNotFound {
            loc: loc_lib::loc::Loc,
        },
        CheckCommit {
            check_commit: pg_crud::check_commit::CommitErWithSerde,
            loc: loc_lib::loc::Loc,
        },
        RowAndRollback {
            row: String,
            rollback: String,
            loc: loc_lib::loc::Loc,
        },
        TryBind {
            try_bind: String,
            loc: loc_lib::loc::Loc,
        },
    }
    impl TblExampleDloResVrts {
        fn from_h(v: TblExampleDloEr) -> Self {
            match v.into_serde_version() {
                TblExampleDloErWithSerde::CheckBodySize {
                    check_body_size,
                    loc,
                } => Self::CheckBodySize {
                    check_body_size,
                    loc,
                },
                TblExampleDloErWithSerde::Pg { pg, loc } => Self::Pg { pg, loc },
                TblExampleDloErWithSerde::SerdeJson { serde_json, loc } => {
                    Self::SerdeJson { serde_json, loc }
                }
                TblExampleDloErWithSerde::HeaderContentTypeAppJsonNotFound { loc } => {
                    Self::HeaderContentTypeAppJsonNotFound { loc }
                }
                TblExampleDloErWithSerde::CheckCommit { check_commit, loc } => {
                    Self::CheckCommit { check_commit, loc }
                }
                TblExampleDloErWithSerde::RowAndRollback { row, rollback, loc } => {
                    Self::RowAndRollback { row, rollback, loc }
                }
                TblExampleDloErWithSerde::TryBind { try_bind, loc } => {
                    Self::TryBind { try_bind, loc }
                }
            }
        }
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug, thiserror :: Error, loc_lib :: Location)]
    pub enum TblExampleDloEr {
        CheckBodySize {
            #[eo_loc]
            check_body_size: pg_crud::check_body_size::BodySizeEr,
            loc: loc_lib::loc::Loc,
        },
        Pg {
            #[eo_to_err_string]
            pg: sqlx::Error,
            loc: loc_lib::loc::Loc,
        },
        SerdeJson {
            #[eo_to_err_string]
            serde_json: serde_json::Error,
            loc: loc_lib::loc::Loc,
        },
        HeaderContentTypeAppJsonNotFound {
            loc: loc_lib::loc::Loc,
        },
        CheckCommit {
            #[eo_loc]
            check_commit: pg_crud::check_commit::CommitEr,
            loc: loc_lib::loc::Loc,
        },
        RowAndRollback {
            #[eo_to_err_string]
            row: sqlx::Error,
            #[eo_to_err_string]
            rollback: sqlx::Error,
            loc: loc_lib::loc::Loc,
        },
        TryBind {
            #[eo_to_err_string_serde]
            try_bind: String,
            loc: loc_lib::loc::Loc,
        },
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug, thiserror :: Error, loc_lib :: Location)]
    pub enum TblExampleTryDloEr {
        SerdeJsonToString {
            #[eo_to_err_string]
            serde_json_to_string: serde_json::Error,
            loc: loc_lib::loc::Loc,
        },
        FailedToGetResText {
            #[eo_to_err_string]
            status_code: reqwest::StatusCode,
            #[eo_to_err_string]
            headers: reqwest::header::HeaderMap,
            #[eo_to_err_string]
            reqwest: reqwest::Error,
            loc: loc_lib::loc::Loc,
        },
        DeRes {
            #[eo_to_err_string]
            status_code: reqwest::StatusCode,
            #[eo_to_err_string]
            headers: reqwest::header::HeaderMap,
            #[eo_to_err_string_serde]
            res_text: String,
            #[eo_to_err_string]
            serde: serde_json::Error,
            loc: loc_lib::loc::Loc,
        },
        Reqwest {
            #[eo_to_err_string]
            reqwest: reqwest::Error,
            loc: loc_lib::loc::Loc,
        },
        TblExampleDloErWithSerde {
            #[eo_to_err_string]
            dlo_er_with_serde: TblExampleDloErWithSerde,
            loc: loc_lib::loc::Loc,
        },
    }
    #[derive(Debug, thiserror :: Error, loc_lib :: Location)]
    pub enum TblExamplePrepPgEr {
        CrExtensionIfNotExistsUuidOssp {
            #[eo_to_err_string]
            er: sqlx::Error,
            loc: loc_lib::loc::Loc,
        },
        PrepPg {
            #[eo_to_err_string]
            er: sqlx::Error,
            loc: loc_lib::loc::Loc,
        },
    }
    #[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
    pub struct TblExampleCr {
        pub col_0: <pg_crud::I16AsNnInt2 as pg_crud::PgType>::Cr,
        pub col_1: <pg_crud::OptI16AsNlInt2 as pg_crud::PgType>::Cr,
        pub col_2: <pg_crud::I32AsNnInt4 as pg_crud::PgType>::Cr,
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    impl TblExampleCr {
        fn cr_qp(&self, incr: &mut u64) -> Result<String, pg_crud::QpEr> {
            let mut acc = String::new();
            match < pg_crud :: SqlxTypesUuidUuidAsNnUuidV4InitByPg as pg_crud :: PgType > :: cr_qp (& < < pg_crud :: SqlxTypesUuidUuidAsNnUuidV4InitByPg as pg_crud :: PgType > :: Cr as pg_crud :: DfltSomeOneEl > :: dflt_some_one_el () , incr) { Ok (v_c3f0b59a) => { if { use std :: fmt :: Write as _ ; write ! (acc , "{v_c3f0b59a},") } . is_err () { return Err (pg_crud :: QpEr :: WriteIntoBuffer { loc : loc_lib :: loc ! () }) ; } } , Err (er_0) => { return Err (er_0) ; } }
            match <pg_crud::I16AsNnInt2 as pg_crud::PgType>::cr_qp(&self.col_0, incr) {
                Ok(v_c3f0b59a) => {
                    if {
                        use std::fmt::Write as _;
                        write!(acc, "{v_c3f0b59a},")
                    }
                    .is_err()
                    {
                        return Err(pg_crud::QpEr::WriteIntoBuffer {
                            loc: loc_lib::loc!(),
                        });
                    }
                }
                Err(er_0) => {
                    return Err(er_0);
                }
            }
            match <pg_crud::OptI16AsNlInt2 as pg_crud::PgType>::cr_qp(&self.col_1, incr) {
                Ok(v_c3f0b59a) => {
                    if {
                        use std::fmt::Write as _;
                        write!(acc, "{v_c3f0b59a},")
                    }
                    .is_err()
                    {
                        return Err(pg_crud::QpEr::WriteIntoBuffer {
                            loc: loc_lib::loc!(),
                        });
                    }
                }
                Err(er_0) => {
                    return Err(er_0);
                }
            }
            match <pg_crud::I32AsNnInt4 as pg_crud::PgType>::cr_qp(&self.col_2, incr) {
                Ok(v_c3f0b59a) => {
                    if {
                        use std::fmt::Write as _;
                        write!(acc, "{v_c3f0b59a},")
                    }
                    .is_err()
                    {
                        return Err(pg_crud::QpEr::WriteIntoBuffer {
                            loc: loc_lib::loc!(),
                        });
                    }
                }
                Err(er_0) => {
                    return Err(er_0);
                }
            }
            let _: Option<char> = acc.pop();
            Ok(acc)
        }
        fn cr_qb(
            self,
            mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>,
        ) -> Result<sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>, String>
        {
            match < pg_crud :: SqlxTypesUuidUuidAsNnUuidV4InitByPg as pg_crud :: PgType > :: cr_qb (< < pg_crud :: SqlxTypesUuidUuidAsNnUuidV4InitByPg as pg_crud :: PgType > :: Cr as pg_crud :: DfltSomeOneEl > :: dflt_some_one_el () , query) { Ok (v_3c55d2e1) => { query = v_3c55d2e1 ; } , Err (er_0) => { return Err (er_0) ; } }
            match <pg_crud::I16AsNnInt2 as pg_crud::PgType>::cr_qb(self.col_0, query) {
                Ok(v_3c55d2e1) => {
                    query = v_3c55d2e1;
                }
                Err(er_0) => {
                    return Err(er_0);
                }
            }
            match <pg_crud::OptI16AsNlInt2 as pg_crud::PgType>::cr_qb(self.col_1, query) {
                Ok(v_3c55d2e1) => {
                    query = v_3c55d2e1;
                }
                Err(er_0) => {
                    return Err(er_0);
                }
            }
            match <pg_crud::I32AsNnInt4 as pg_crud::PgType>::cr_qb(self.col_2, query) {
                Ok(v_3c55d2e1) => {
                    query = v_3c55d2e1;
                }
                Err(er_0) => {
                    return Err(er_0);
                }
            }
            Ok(query)
        }
    }
    impl pg_crud::DfltSomeOneEl for TblExampleCr {
        fn dflt_some_one_el() -> Self {
            Self {
                col_0: pg_crud::DfltSomeOneEl::dflt_some_one_el(),
                col_1: pg_crud::DfltSomeOneEl::dflt_some_one_el(),
                col_2: pg_crud::DfltSomeOneEl::dflt_some_one_el(),
            }
        }
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug, Clone, serde :: Serialize, utoipa :: ToSchema)]
    pub struct TblExampleWhMany {
        pk_col: Option<
            pg_crud::PgTypeWh<
                <pg_crud::SqlxTypesUuidUuidAsNnUuidV4InitByPg as pg_crud::PgType>::Wh,
            >,
        >,
        col_0: Option<pg_crud::PgTypeWh<<pg_crud::I16AsNnInt2 as pg_crud::PgType>::Wh>>,
        col_1: Option<pg_crud::PgTypeWh<<pg_crud::OptI16AsNlInt2 as pg_crud::PgType>::Wh>>,
        col_2: Option<pg_crud::PgTypeWh<<pg_crud::I32AsNnInt4 as pg_crud::PgType>::Wh>>,
    }
    #[derive(Debug, thiserror :: Error, loc_lib :: Location)]
    pub enum TblExampleWhManyTryNewEr {
        NoFieldsProvided {
            #[eo_to_err_string]
            loc: loc_lib::loc::Loc,
        },
    }
    impl TblExampleWhMany {
        pub fn try_new(
            pk_col: Option<
                pg_crud::PgTypeWh<
                    <pg_crud::SqlxTypesUuidUuidAsNnUuidV4InitByPg as pg_crud::PgType>::Wh,
                >,
            >,
            col_0: Option<pg_crud::PgTypeWh<<pg_crud::I16AsNnInt2 as pg_crud::PgType>::Wh>>,
            col_1: Option<pg_crud::PgTypeWh<<pg_crud::OptI16AsNlInt2 as pg_crud::PgType>::Wh>>,
            col_2: Option<pg_crud::PgTypeWh<<pg_crud::I32AsNnInt4 as pg_crud::PgType>::Wh>>,
        ) -> Result<Self, TblExampleWhManyTryNewEr> {
            if matches!((&pk_col, &col_0, &col_1, &col_2), (None, None, None, None)) {
                return Err(TblExampleWhManyTryNewEr::NoFieldsProvided {
                    loc: loc_lib::loc!(),
                });
            }
            Ok(Self {
                pk_col,
                col_0,
                col_1,
                col_2,
            })
        }
    }
    #[derive(serde :: Deserialize)]
    #[allow(clippy::arbitrary_source_item_ordering)]
    struct TblExampleWhManyRaw {
        pk_col: Option<
            pg_crud::PgTypeWh<
                <pg_crud::SqlxTypesUuidUuidAsNnUuidV4InitByPg as pg_crud::PgType>::Wh,
            >,
        >,
        col_0: Option<pg_crud::PgTypeWh<<pg_crud::I16AsNnInt2 as pg_crud::PgType>::Wh>>,
        col_1: Option<pg_crud::PgTypeWh<<pg_crud::OptI16AsNlInt2 as pg_crud::PgType>::Wh>>,
        col_2: Option<pg_crud::PgTypeWh<<pg_crud::I32AsNnInt4 as pg_crud::PgType>::Wh>>,
    }
    #[allow(unused_qualifications)]
    #[allow(clippy::absolute_paths)]
    #[allow(clippy::arbitrary_source_item_ordering)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for TblExampleWhMany {
            fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                let raw =
                    <TblExampleWhManyRaw as _serde::Deserialize>::deserialize(__deserializer)?;
                Self::try_new(raw.pk_col, raw.col_0, raw.col_1, raw.col_2)
                    .map_err(|er| _serde::de::Error::custom(format!("{er:?}")))
            }
        }
    };
    impl pg_crud::DfltSomeOneEl for TblExampleWhMany {
        fn dflt_some_one_el() -> Self {
            Self {
                pk_col: Some(pg_crud::DfltSomeOneEl::dflt_some_one_el()),
                col_0: Some(pg_crud::DfltSomeOneEl::dflt_some_one_el()),
                col_1: Some(pg_crud::DfltSomeOneEl::dflt_some_one_el()),
                col_2: Some(pg_crud::DfltSomeOneEl::dflt_some_one_el()),
            }
        }
    }
    #[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
    pub struct StdOptOptTblExampleWhMany(pub Option<TblExampleWhMany>);
    #[allow(clippy::arbitrary_source_item_ordering)]
    impl<'lt> pg_crud::PgTypeWhFlt<'lt> for StdOptOptTblExampleWhMany {
        fn qp(
            &self,
            incr: &mut u64,
            _: &dyn std::fmt::Display,
            _: bool,
        ) -> Result<String, pg_crud::QpEr> {
            Ok(match &self.0 {
                Some(v) => {
                    let mut extra_prms = String::from("where");
                    let mut is_first_push_to_extra_prms_already_happend = false;
                    if let Some(v_da0f0616) = &v.pk_col {
                        match pg_crud::PgTypeWhFlt::qp(
                            v_da0f0616,
                            incr,
                            &"pk_col",
                            is_first_push_to_extra_prms_already_happend,
                        ) {
                            Ok(v_9e3f8fdd) => {
                                extra_prms.push_str(&v_9e3f8fdd);
                                is_first_push_to_extra_prms_already_happend = true;
                            }
                            Err(er_0) => {
                                return Err(er_0);
                            }
                        }
                    }
                    if let Some(v_da0f0616) = &v.col_0 {
                        match pg_crud::PgTypeWhFlt::qp(
                            v_da0f0616,
                            incr,
                            &"col_0",
                            is_first_push_to_extra_prms_already_happend,
                        ) {
                            Ok(v_9e3f8fdd) => {
                                extra_prms.push_str(&v_9e3f8fdd);
                                is_first_push_to_extra_prms_already_happend = true;
                            }
                            Err(er_0) => {
                                return Err(er_0);
                            }
                        }
                    }
                    if let Some(v_da0f0616) = &v.col_1 {
                        match pg_crud::PgTypeWhFlt::qp(
                            v_da0f0616,
                            incr,
                            &"col_1",
                            is_first_push_to_extra_prms_already_happend,
                        ) {
                            Ok(v_9e3f8fdd) => {
                                extra_prms.push_str(&v_9e3f8fdd);
                                is_first_push_to_extra_prms_already_happend = true;
                            }
                            Err(er_0) => {
                                return Err(er_0);
                            }
                        }
                    }
                    if let Some(v_da0f0616) = &v.col_2 {
                        match pg_crud::PgTypeWhFlt::qp(
                            v_da0f0616,
                            incr,
                            &"col_2",
                            is_first_push_to_extra_prms_already_happend,
                        ) {
                            Ok(v_9e3f8fdd) => {
                                extra_prms.push_str(&v_9e3f8fdd);
                            }
                            Err(er_0) => {
                                return Err(er_0);
                            }
                        }
                    }
                    extra_prms
                }
                None => String::default(),
            })
        }
        fn qb(
            self,
            mut query: sqlx::query::Query<'lt, sqlx::Postgres, sqlx::postgres::PgArguments>,
        ) -> Result<sqlx::query::Query<'lt, sqlx::Postgres, sqlx::postgres::PgArguments>, String>
        {
            if let Some(v_27176ffb) = self.0 {
                if let Some(v_b12d6fe0) = v_27176ffb.pk_col {
                    match pg_crud::PgTypeWhFlt::qb(v_b12d6fe0, query) {
                        Ok(v_edaee3b2) => {
                            query = v_edaee3b2;
                        }
                        Err(er_0) => {
                            return Err(er_0);
                        }
                    }
                }
                if let Some(v_b12d6fe0) = v_27176ffb.col_0 {
                    match pg_crud::PgTypeWhFlt::qb(v_b12d6fe0, query) {
                        Ok(v_edaee3b2) => {
                            query = v_edaee3b2;
                        }
                        Err(er_0) => {
                            return Err(er_0);
                        }
                    }
                }
                if let Some(v_b12d6fe0) = v_27176ffb.col_1 {
                    match pg_crud::PgTypeWhFlt::qb(v_b12d6fe0, query) {
                        Ok(v_edaee3b2) => {
                            query = v_edaee3b2;
                        }
                        Err(er_0) => {
                            return Err(er_0);
                        }
                    }
                }
                if let Some(v_b12d6fe0) = v_27176ffb.col_2 {
                    match pg_crud::PgTypeWhFlt::qb(v_b12d6fe0, query) {
                        Ok(v_edaee3b2) => {
                            query = v_edaee3b2;
                        }
                        Err(er_0) => {
                            return Err(er_0);
                        }
                    }
                }
            }
            Ok(query)
        }
    }
    impl pg_crud::DfltSomeOneEl for StdOptOptTblExampleWhMany {
        fn dflt_some_one_el() -> Self {
            Self(Some(pg_crud::DfltSomeOneEl::dflt_some_one_el()))
        }
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum TblExampleSel {
        #[serde(rename(serialize = "pk_col", deserialize = "pk_col"))]
        PkCol(<pg_crud::SqlxTypesUuidUuidAsNnUuidV4InitByPg as pg_crud::PgType>::Sel),
        #[serde(rename(serialize = "col_0", deserialize = "col_0"))]
        Col0(<pg_crud::I16AsNnInt2 as pg_crud::PgType>::Sel),
        #[serde(rename(serialize = "col_1", deserialize = "col_1"))]
        Col1(<pg_crud::OptI16AsNlInt2 as pg_crud::PgType>::Sel),
        #[serde(rename(serialize = "col_2", deserialize = "col_2"))]
        Col2(<pg_crud::I32AsNnInt4 as pg_crud::PgType>::Sel),
    }
    impl std::fmt::Display for TblExampleSel {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{}",
                serde_json::to_string(&self).unwrap_or_else(|el_2636212f| format!(
                    "cannot serialize into json: {el_2636212f:?}"
                ))
            )
        }
    }
    impl loc_lib::ToErrString for TblExampleSel {
        fn to_err_string(&self) -> String {
            format!("{self}")
        }
    }
    impl pg_crud::AllEnumVrtsArrDfltSomeOneEl for TblExampleSel {
        fn all_vrts_dflt_some_one_el() -> Vec<Self> {
            vec![
                Self::PkCol(pg_crud::DfltSomeOneEl::dflt_some_one_el()),
                Self::Col0(pg_crud::DfltSomeOneEl::dflt_some_one_el()),
                Self::Col1(pg_crud::DfltSomeOneEl::dflt_some_one_el()),
                Self::Col2(pg_crud::DfltSomeOneEl::dflt_some_one_el()),
            ]
        }
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct TblExampleRd {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub pk_col: Option<
            pg_crud::V<<pg_crud::SqlxTypesUuidUuidAsNnUuidV4InitByPg as pg_crud::PgType>::Rd>,
        >,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub col_0: Option<pg_crud::V<<pg_crud::I16AsNnInt2 as pg_crud::PgType>::Rd>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub col_1: Option<pg_crud::V<<pg_crud::OptI16AsNlInt2 as pg_crud::PgType>::Rd>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub col_2: Option<pg_crud::V<<pg_crud::I32AsNnInt4 as pg_crud::PgType>::Rd>>,
    }
    impl TblExampleRd {
        fn try_from_sqlx_pg_pg_row_with_not_empty_unq_vec_tbl_example_sel(
            v: &sqlx::postgres::PgRow,
            sel: &pg_crud::NotEmptyUnqVec<TblExampleSel>,
        ) -> Result<Self, sqlx::Error> {
            let mut pk_col: Option<
                pg_crud::V<<pg_crud::SqlxTypesUuidUuidAsNnUuidV4InitByPg as pg_crud::PgType>::Rd>,
            > = None;
            let mut col_0: Option<pg_crud::V<<pg_crud::I16AsNnInt2 as pg_crud::PgType>::Rd>> = None;
            let mut col_1: Option<pg_crud::V<<pg_crud::OptI16AsNlInt2 as pg_crud::PgType>::Rd>> =
                None;
            let mut col_2: Option<pg_crud::V<<pg_crud::I32AsNnInt4 as pg_crud::PgType>::Rd>> = None;
            for el_dca9f0b7 in sel.to_vec() {
                match el_dca9f0b7 {
                    TblExampleSel::PkCol(_) => match sqlx::Row::try_get::<
                        <pg_crud::SqlxTypesUuidUuidAsNnUuidV4InitByPg as pg_crud::PgType>::Rd,
                        &str,
                    >(v, "pk_col")
                    {
                        Ok(v_470178a2) => {
                            pk_col = Some(pg_crud::V { v: v_470178a2 });
                        }
                        Err(er_0) => {
                            return Err(er_0);
                        }
                    },
                    TblExampleSel::Col0(_) => match sqlx::Row::try_get::<
                        <pg_crud::I16AsNnInt2 as pg_crud::PgType>::Rd,
                        &str,
                    >(v, "col_0")
                    {
                        Ok(v_470178a2) => {
                            col_0 = Some(pg_crud::V { v: v_470178a2 });
                        }
                        Err(er_0) => {
                            return Err(er_0);
                        }
                    },
                    TblExampleSel::Col1(_) => match sqlx::Row::try_get::<
                        <pg_crud::OptI16AsNlInt2 as pg_crud::PgType>::Rd,
                        &str,
                    >(v, "col_1")
                    {
                        Ok(v_470178a2) => {
                            col_1 = Some(pg_crud::V { v: v_470178a2 });
                        }
                        Err(er_0) => {
                            return Err(er_0);
                        }
                    },
                    TblExampleSel::Col2(_) => match sqlx::Row::try_get::<
                        <pg_crud::I32AsNnInt4 as pg_crud::PgType>::Rd,
                        &str,
                    >(v, "col_2")
                    {
                        Ok(v_470178a2) => {
                            col_2 = Some(pg_crud::V { v: v_470178a2 });
                        }
                        Err(er_0) => {
                            return Err(er_0);
                        }
                    },
                }
            }
            Ok(Self {
                pk_col,
                col_0,
                col_1,
                col_2,
            })
        }
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct TblExampleRdIds {
        pub pk_col: <pg_crud::SqlxTypesUuidUuidAsNnUuidV4InitByPg as pg_crud::PgType>::RdIds,
        pub col_0: Option<<pg_crud::I16AsNnInt2 as pg_crud::PgType>::RdIds>,
        pub col_1: Option<<pg_crud::OptI16AsNlInt2 as pg_crud::PgType>::RdIds>,
        pub col_2: Option<<pg_crud::I32AsNnInt4 as pg_crud::PgType>::RdIds>,
    }
    impl<'lt, R: ::sqlx::Row<Database = sqlx::Postgres>> ::sqlx::FromRow<'lt, R> for TblExampleRdIds
    where
        &'lt ::std::primitive::str: ::sqlx::ColumnIndex<R>,
        <pg_crud::SqlxTypesUuidUuidAsNnUuidV4InitByPg as pg_crud::PgType>::RdIds:
            ::sqlx::decode::Decode<'lt, R::Database>,
        <pg_crud::I16AsNnInt2 as pg_crud::PgType>::RdIds: ::sqlx::decode::Decode<'lt, R::Database>,
        <pg_crud::OptI16AsNlInt2 as pg_crud::PgType>::RdIds:
            ::sqlx::decode::Decode<'lt, R::Database>,
        <pg_crud::I32AsNnInt4 as pg_crud::PgType>::RdIds: ::sqlx::decode::Decode<'lt, R::Database>,
    {
        fn from_row(__row: &'lt R) -> ::sqlx::Result<Self> {
            let pk_col = match sqlx::Row::try_get::<
                <pg_crud::SqlxTypesUuidUuidAsNnUuidV4InitByPg as pg_crud::PgType>::RdIds,
                &str,
            >(__row, "pk_col")
            {
                Ok(v_283179dd) => v_283179dd,
                Err(er_0) => {
                    return Err(er_0);
                }
            };
            let col_0 =
                sqlx::Row::try_get::<<pg_crud::I16AsNnInt2 as pg_crud::PgType>::RdIds, &str>(
                    __row, "col_0",
                )
                .ok();
            let col_1 = sqlx::Row::try_get::<
                <pg_crud::OptI16AsNlInt2 as pg_crud::PgType>::RdIds,
                &str,
            >(__row, "col_1")
            .ok();
            let col_2 =
                sqlx::Row::try_get::<<pg_crud::I32AsNnInt4 as pg_crud::PgType>::RdIds, &str>(
                    __row, "col_2",
                )
                .ok();
            Ok(Self {
                pk_col,
                col_0,
                col_1,
                col_2,
            })
        }
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug, serde :: Serialize, utoipa :: ToSchema)]
    pub struct TblExampleUpd {
        pk_col: pg_crud::SqlxTypesUuidUuidAsNnUuidV4InitByPgUpd,
        col_0: Option<pg_crud::V<<pg_crud::I16AsNnInt2 as pg_crud::PgType>::Upd>>,
        col_1: Option<pg_crud::V<<pg_crud::OptI16AsNlInt2 as pg_crud::PgType>::Upd>>,
        col_2: Option<pg_crud::V<<pg_crud::I32AsNnInt4 as pg_crud::PgType>::Upd>>,
    }
    #[derive(Debug, thiserror :: Error, loc_lib :: Location)]
    pub enum TblExampleUpdTryNewEr {
        NoFieldsProvided {
            #[eo_to_err_string]
            loc: loc_lib::loc::Loc,
        },
    }
    impl TblExampleUpd {
        #[allow(clippy::redundant_pattern_matching)]
        pub fn try_new(
            pk_col: pg_crud::SqlxTypesUuidUuidAsNnUuidV4InitByPgUpd,
            col_0: Option<pg_crud::V<<pg_crud::I16AsNnInt2 as pg_crud::PgType>::Upd>>,
            col_1: Option<pg_crud::V<<pg_crud::OptI16AsNlInt2 as pg_crud::PgType>::Upd>>,
            col_2: Option<pg_crud::V<<pg_crud::I32AsNnInt4 as pg_crud::PgType>::Upd>>,
        ) -> Result<Self, TblExampleUpdTryNewEr> {
            if matches!((&col_0, &col_1, &col_2), (None, None, None)) {
                return Err(TblExampleUpdTryNewEr::NoFieldsProvided {
                    loc: loc_lib::loc!(),
                });
            }
            Ok(Self {
                pk_col,
                col_0,
                col_1,
                col_2,
            })
        }
    }
    #[derive(serde :: Deserialize)]
    #[allow(clippy::arbitrary_source_item_ordering)]
    struct TblExampleUpdRaw {
        pk_col: pg_crud::SqlxTypesUuidUuidAsNnUuidV4InitByPgUpd,
        col_0: Option<pg_crud::V<<pg_crud::I16AsNnInt2 as pg_crud::PgType>::Upd>>,
        col_1: Option<pg_crud::V<<pg_crud::OptI16AsNlInt2 as pg_crud::PgType>::Upd>>,
        col_2: Option<pg_crud::V<<pg_crud::I32AsNnInt4 as pg_crud::PgType>::Upd>>,
    }
    #[allow(unused_qualifications)]
    #[allow(clippy::absolute_paths)]
    #[allow(clippy::arbitrary_source_item_ordering)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for TblExampleUpd {
            fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                let raw = <TblExampleUpdRaw as _serde::Deserialize>::deserialize(__deserializer)?;
                Self::try_new(raw.pk_col, raw.col_0, raw.col_1, raw.col_2)
                    .map_err(|er| _serde::de::Error::custom(format!("{er:?}")))
            }
        }
    };
    impl pg_crud::DfltSomeOneEl for TblExampleUpd {
        fn dflt_some_one_el() -> Self {
            Self {
                pk_col: pg_crud::DfltSomeOneEl::dflt_some_one_el(),
                col_0: Some(pg_crud::V {
                    v: pg_crud::DfltSomeOneEl::dflt_some_one_el(),
                }),
                col_1: Some(pg_crud::V {
                    v: pg_crud::DfltSomeOneEl::dflt_some_one_el(),
                }),
                col_2: Some(pg_crud::V {
                    v: pg_crud::DfltSomeOneEl::dflt_some_one_el(),
                }),
            }
        }
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug, serde :: Serialize, utoipa :: ToSchema)]
    pub struct TblExampleUpdForQuery {
        pk_col: pg_crud::SqlxTypesUuidUuidAsNnUuidV4InitByPgUpdForQuery,
        col_0: Option<pg_crud::V<<pg_crud::I16AsNnInt2 as pg_crud::PgType>::UpdForQuery>>,
        col_1: Option<pg_crud::V<<pg_crud::OptI16AsNlInt2 as pg_crud::PgType>::UpdForQuery>>,
        col_2: Option<pg_crud::V<<pg_crud::I32AsNnInt4 as pg_crud::PgType>::UpdForQuery>>,
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    impl TblExampleUpdForQuery {
        fn upd_qp_pk(&self, incr: &mut u64) -> Result<String, pg_crud::QpEr> {
            match <pg_crud::SqlxTypesUuidUuidAsNnUuidV4InitByPg as pg_crud::PgType>::upd_qp(
                &self.pk_col,
                "",
                TblExample::pk(),
                "",
                incr,
            ) {
                Ok(v) => Ok(v),
                Err(er_0) => Err(er_0),
            }
        }
        fn upd_qp_col_0(
            v: &pg_crud::V<<pg_crud::I16AsNnInt2 as pg_crud::PgType>::UpdForQuery>,
            incr: &mut u64,
        ) -> Result<String, pg_crud::QpEr> {
            match <pg_crud::I16AsNnInt2 as pg_crud::PgType>::upd_qp(
                &v.v, "col_0", "col_0", "", incr,
            ) {
                Ok(v_f75dfd93) => Ok(v_f75dfd93),
                Err(er_0) => Err(er_0),
            }
        }
        fn upd_qp_col_1(
            v: &pg_crud::V<<pg_crud::OptI16AsNlInt2 as pg_crud::PgType>::UpdForQuery>,
            incr: &mut u64,
        ) -> Result<String, pg_crud::QpEr> {
            match <pg_crud::OptI16AsNlInt2 as pg_crud::PgType>::upd_qp(
                &v.v, "col_1", "col_1", "", incr,
            ) {
                Ok(v_f75dfd93) => Ok(v_f75dfd93),
                Err(er_0) => Err(er_0),
            }
        }
        fn upd_qp_col_2(
            v: &pg_crud::V<<pg_crud::I32AsNnInt4 as pg_crud::PgType>::UpdForQuery>,
            incr: &mut u64,
        ) -> Result<String, pg_crud::QpEr> {
            match <pg_crud::I32AsNnInt4 as pg_crud::PgType>::upd_qp(
                &v.v, "col_2", "col_2", "", incr,
            ) {
                Ok(v_f75dfd93) => Ok(v_f75dfd93),
                Err(er_0) => Err(er_0),
            }
        }
        fn sel_only_updd_ids_qp(&self, incr: &mut u64) -> Result<String, pg_crud::QpEr> {
            let mut acc = String::new();
            acc . push_str (& match < pg_crud :: SqlxTypesUuidUuidAsNnUuidV4InitByPg as pg_crud :: PgType > :: sel_only_updd_ids_qp (& self . pk_col , "pk_col" , incr ,) { Ok (v) => v , Err (er_0) => { { return Err (er_0) ; } } }) ;
            if let Some(v_90f79b11) = &self.col_0 {
                acc.push_str(
                    &match <pg_crud::I16AsNnInt2 as pg_crud::PgType>::sel_only_updd_ids_qp(
                        &v_90f79b11.v,
                        "col_0",
                        incr,
                    ) {
                        Ok(v_47a6f597) => v_47a6f597,
                        Err(er_0) => {
                            return Err(er_0);
                        }
                    },
                );
            }
            if let Some(v_90f79b11) = &self.col_1 {
                acc.push_str(
                    &match <pg_crud::OptI16AsNlInt2 as pg_crud::PgType>::sel_only_updd_ids_qp(
                        &v_90f79b11.v,
                        "col_1",
                        incr,
                    ) {
                        Ok(v_47a6f597) => v_47a6f597,
                        Err(er_0) => {
                            return Err(er_0);
                        }
                    },
                );
            }
            if let Some(v_90f79b11) = &self.col_2 {
                acc.push_str(
                    &match <pg_crud::I32AsNnInt4 as pg_crud::PgType>::sel_only_updd_ids_qp(
                        &v_90f79b11.v,
                        "col_2",
                        incr,
                    ) {
                        Ok(v_47a6f597) => v_47a6f597,
                        Err(er_0) => {
                            return Err(er_0);
                        }
                    },
                );
            }
            let _: Option<char> = acc.pop();
            Ok(acc)
        }
        fn from_h(v: TblExampleUpd) -> Self {
            Self { pk_col : < pg_crud :: SqlxTypesUuidUuidAsNnUuidV4InitByPg as pg_crud :: PgType > :: UpdForQuery :: from (v . pk_col) , col_0 : v . col_0 . map (| v_0e64c53a | pg_crud :: V { v : < pg_crud :: I16AsNnInt2 as pg_crud :: PgType > :: UpdForQuery :: from (v_0e64c53a . v) }) , col_1 : v . col_1 . map (| v_0e64c53a | pg_crud :: V { v : < pg_crud :: OptI16AsNlInt2 as pg_crud :: PgType > :: UpdForQuery :: from (v_0e64c53a . v) }) , col_2 : v . col_2 . map (| v_0e64c53a | pg_crud :: V { v : < pg_crud :: I32AsNnInt4 as pg_crud :: PgType > :: UpdForQuery :: from (v_0e64c53a . v) }) }
        }
    }
}
pub use tbl_example_gen_pg_tbl_mod::*;
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, Clone, Copy, optml :: Optml)]
# [pg_crud :: gen_pg_tbl_config { { "cm_write_into_file" : "False" , "co_write_into_file" : "False" , "rm_write_into_file" : "False" , "ro_write_into_file" : "False" , "um_write_into_file" : "False" , "uo_write_into_file" : "False" , "dm_write_into_file" : "False" , "dlo_write_into_file" : "False" , "tests_write_into_file" : "False" , "cmn_write_into_file" : "False" , "whole_write_into_file" : "False" } }]
# [pg_crud :: cm_er_vrts { enum CmErVrts { } }]
# [pg_crud :: co_er_vrts { enum CoErVrts { } }]
# [pg_crud :: rm_er_vrts { enum RmErVrts { } }]
# [pg_crud :: ro_er_vrts { enum RoErVrts { } }]
# [pg_crud :: um_er_vrts { enum UmErVrts { } }]
# [pg_crud :: uo_er_vrts { enum UoErVrts { } }]
# [pg_crud :: dm_er_vrts { enum DmErVrts { } }]
# [pg_crud :: dlo_er_vrts { enum DloErVrts { } }]
# [pg_crud :: cmn_er_vrts { enum CmnErVrts { CheckCommit { # [eo_loc] check_commit : pg_crud :: check_commit :: CommitEr , loc : loc_lib :: loc :: Loc , } , } }]
# [pg_crud :: cm_logic { }]
# [pg_crud :: co_logic { }]
# [pg_crud :: rm_logic { }]
# [pg_crud :: ro_logic { }]
# [pg_crud :: um_logic { }]
# [pg_crud :: uo_logic { }]
# [pg_crud :: dm_logic { }]
# [pg_crud :: dlo_logic { }]
# [pg_crud :: cmn_logic { }]
pub struct TblExample {
    pub pk_col: pg_crud::SqlxTypesUuidUuidAsNnUuidV4InitByPg,
    pub col_0: pg_crud::I16AsNnInt2,
    pub col_1: pg_crud::OptI16AsNlInt2,
    pub col_2: pg_crud::I32AsNnInt4,
}
