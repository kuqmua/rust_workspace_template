pub use gen_pg_tbl::*;
pub trait CombinationOfAppStateLogicTraits:
    app_state::GetEnableApiGitCommitCheck
    + app_state::GetMaximumSizeOfHttpBodyInBytes
    + app_state::GetSrcPlaceType
    + app_state::GetTimezone
    + app_state::GetPgPool
    + Send
    + Sync
{
}
#[derive(Clone, Copy)]
enum InsertValuesFmt {
    Raw,
    Wrapped,
}
#[derive(Clone, Copy)]
enum SelectWhereFmt {
    Plain,
    Where,
}
#[derive(Clone, Copy)]
enum UpdateSelectorFmt {
    Eq,
    InList,
}
#[derive(Debug, Clone, Copy)]
pub struct PgTblNameRef<'lt>(pub &'lt str);
impl std::fmt::Display for PgTblNameRef<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct PgTblSqlFragmentRef<'lt>(pub &'lt str);
impl std::fmt::Display for PgTblSqlFragmentRef<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0)
    }
}
#[derive(Debug, Clone)]
pub struct PgTblQueryString(pub String);
impl std::fmt::Display for PgTblQueryString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}
impl std::ops::Deref for PgTblQueryString {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[derive(Debug, Clone)]
pub struct PgTblQpFragment(pub String);
impl std::fmt::Display for PgTblQpFragment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}
impl std::ops::Deref for PgTblQpFragment {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
fn gen_insert_query_string(
    tbl: PgTblNameRef<'_>,
    cols: PgTblSqlFragmentRef<'_>,
    values: PgTblSqlFragmentRef<'_>,
    cols_to_return: PgTblSqlFragmentRef<'_>,
    insert_values_fmt: InsertValuesFmt,
) -> PgTblQueryString {
    match insert_values_fmt {
        InsertValuesFmt::Raw => PgTblQueryString(format!(
            "insert into {tbl} ({cols}) values {values} returning {cols_to_return}"
        )),
        InsertValuesFmt::Wrapped => PgTblQueryString(format!(
            "insert into {tbl} ({cols}) values ({values}) returning {cols_to_return}"
        )),
    }
}
fn gen_select_query_string(
    tbl: PgTblNameRef<'_>,
    sel_string: PgTblSqlFragmentRef<'_>,
    wh_string: PgTblSqlFragmentRef<'_>,
    select_where_fmt: SelectWhereFmt,
) -> PgTblQueryString {
    match select_where_fmt {
        SelectWhereFmt::Plain => {
            PgTblQueryString(format!("select {sel_string} from {tbl} {wh_string}"))
        }
        SelectWhereFmt::Where => {
            PgTblQueryString(format!("select {sel_string} from {tbl} where {wh_string}"))
        }
    }
}
fn gen_update_query_string(
    tbl: PgTblNameRef<'_>,
    cols_or_els: PgTblSqlFragmentRef<'_>,
    pk_field_name: PgTblSqlFragmentRef<'_>,
    pk_selector: PgTblSqlFragmentRef<'_>,
    cols_to_return: PgTblSqlFragmentRef<'_>,
    update_selector_fmt: UpdateSelectorFmt,
) -> PgTblQueryString {
    match update_selector_fmt {
        UpdateSelectorFmt::Eq => PgTblQueryString(format!(
            "update {tbl} set {cols_or_els} where {pk_field_name} = {pk_selector} returning {cols_to_return}"
        )),
        UpdateSelectorFmt::InList => PgTblQueryString(format!(
            "update {tbl} set {cols_or_els} where {pk_field_name} in ({pk_selector}) returning {cols_to_return}"
        )),
    }
}
fn gen_delete_query_string(
    tbl: PgTblNameRef<'_>,
    pk_field_name: PgTblSqlFragmentRef<'_>,
    wh_string: Option<PgTblSqlFragmentRef<'_>>,
) -> PgTblQueryString {
    wh_string.map_or_else(
        || {
            PgTblQueryString(format!(
                "delete from {tbl} where {pk_field_name} = $1 returning {pk_field_name}"
            ))
        },
        |v| PgTblQueryString(format!("delete from {tbl} {v} returning {pk_field_name}")),
    )
}
#[must_use]
pub fn gen_cm_query_string(
    tbl: PgTblNameRef<'_>,
    cols: PgTblSqlFragmentRef<'_>,
    values: PgTblSqlFragmentRef<'_>,
    cols_to_return: PgTblSqlFragmentRef<'_>,
) -> PgTblQueryString {
    gen_insert_query_string(tbl, cols, values, cols_to_return, InsertValuesFmt::Raw)
}
#[must_use]
pub fn gen_co_query_string(
    tbl: PgTblNameRef<'_>,
    cols: PgTblSqlFragmentRef<'_>,
    values: PgTblSqlFragmentRef<'_>,
    cols_to_return: PgTblSqlFragmentRef<'_>,
) -> PgTblQueryString {
    gen_insert_query_string(tbl, cols, values, cols_to_return, InsertValuesFmt::Wrapped)
}
#[must_use]
pub fn gen_rm_query_string(
    tbl: PgTblNameRef<'_>,
    sel_string: PgTblSqlFragmentRef<'_>,
    wh_string: PgTblSqlFragmentRef<'_>,
) -> PgTblQueryString {
    gen_select_query_string(tbl, sel_string, wh_string, SelectWhereFmt::Plain)
}
#[must_use]
pub fn gen_ro_query_string(
    tbl: PgTblNameRef<'_>,
    sel_string: PgTblSqlFragmentRef<'_>,
    wh_string: PgTblSqlFragmentRef<'_>,
) -> PgTblQueryString {
    gen_select_query_string(tbl, sel_string, wh_string, SelectWhereFmt::Where)
}
#[must_use]
pub fn gen_col_queals_v_comma_uo_qp(
    col: PgTblSqlFragmentRef<'_>,
    value: PgTblSqlFragmentRef<'_>,
) -> PgTblQpFragment {
    PgTblQpFragment(format!("{col} = {value},"))
}
#[must_use]
pub fn gen_when_col_id_then_v_um_qp(
    col: PgTblSqlFragmentRef<'_>,
    id: PgTblSqlFragmentRef<'_>,
    value: PgTblSqlFragmentRef<'_>,
) -> PgTblQpFragment {
    PgTblQpFragment(format!("when {col} = {id} then {value} "))
}
#[must_use]
pub fn gen_col_eqs_case_acc_else_col_end_comma_um_qp(
    col: PgTblSqlFragmentRef<'_>,
    acc: PgTblSqlFragmentRef<'_>,
) -> PgTblQpFragment {
    PgTblQpFragment(format!("{col} = case {acc}else {col} end,"))
}
//todo extra param for cols_to_return instead of pk_field_name in "returning {pk_field_name}""
#[must_use]
pub fn gen_um_query_string(
    tbl: PgTblNameRef<'_>,
    els: PgTblSqlFragmentRef<'_>,
    pk_field_name: PgTblSqlFragmentRef<'_>,
    pks: PgTblSqlFragmentRef<'_>,
    cols_to_return: PgTblSqlFragmentRef<'_>,
) -> PgTblQueryString {
    gen_update_query_string(
        tbl,
        els,
        pk_field_name,
        pks,
        cols_to_return,
        UpdateSelectorFmt::InList,
    )
}
//todo extra param for cols_to_return instead of pk_field_name in "returning {pk_field_name}""
#[must_use]
pub fn gen_uo_query_string(
    tbl: PgTblNameRef<'_>,
    cols: PgTblSqlFragmentRef<'_>,
    pk_field_name: PgTblSqlFragmentRef<'_>,
    pk_qp: PgTblSqlFragmentRef<'_>,
    cols_to_return: PgTblSqlFragmentRef<'_>,
) -> PgTblQueryString {
    gen_update_query_string(
        tbl,
        cols,
        pk_field_name,
        pk_qp,
        cols_to_return,
        UpdateSelectorFmt::Eq,
    )
}
#[must_use]
pub fn gen_dm_query_string(
    tbl: PgTblNameRef<'_>,
    wh_string: PgTblSqlFragmentRef<'_>,
    pk_field_name: PgTblSqlFragmentRef<'_>,
) -> PgTblQueryString {
    gen_delete_query_string(tbl, pk_field_name, Some(wh_string))
}
#[must_use]
pub fn gen_dlo_query_string(
    tbl: PgTblNameRef<'_>,
    pk_field_name: PgTblSqlFragmentRef<'_>,
) -> PgTblQueryString {
    gen_delete_query_string(tbl, pk_field_name, None)
}
#[cfg(test)]
mod tests {
    fn tbl(v: &'static str) -> super::PgTblNameRef<'static> {
        super::PgTblNameRef(v)
    }
    fn sql(v: &'static str) -> super::PgTblSqlFragmentRef<'static> {
        super::PgTblSqlFragmentRef(v)
    }
    fn users_base() -> (
        super::PgTblNameRef<'static>,
        super::PgTblSqlFragmentRef<'static>,
    ) {
        (tbl("users"), sql("id"))
    }
    fn assert_q(actual: &str, expected: &'static str) {
        assert_eq!(actual, expected);
    }
    #[test]
    fn gen_cm_query_string_is_expected() {
        assert_q(
            &super::gen_cm_query_string(
                tbl("users"),
                sql("id,name"),
                sql("($1,$2),($3,$4)"),
                sql("id"),
            ),
            "insert into users (id,name) values ($1,$2),($3,$4) returning id",
        );
    }
    #[test]
    fn gen_co_query_string_is_expected() {
        assert_q(
            &super::gen_co_query_string(tbl("users"), sql("id,name"), sql("$1,$2"), sql("id")),
            "insert into users (id,name) values ($1,$2) returning id",
        );
    }
    #[test]
    fn gen_rm_query_string_is_expected() {
        assert_q(
            &super::gen_rm_query_string(tbl("users"), sql("id,name"), sql("order by id")),
            "select id,name from users order by id",
        );
    }
    #[test]
    fn gen_ro_query_string_is_expected() {
        assert_q(
            &super::gen_ro_query_string(tbl("users"), sql("id,name"), sql("id = $1")),
            "select id,name from users where id = $1",
        );
    }
    #[test]
    fn gen_col_queals_v_comma_uo_qp_is_expected() {
        assert_q(
            &super::gen_col_queals_v_comma_uo_qp(sql("name"), sql("$2")),
            "name = $2,",
        );
    }
    #[test]
    fn gen_when_col_id_then_v_um_qp_is_expected() {
        assert_q(
            &super::gen_when_col_id_then_v_um_qp(sql("id"), sql("$1"), sql("$2")),
            "when id = $1 then $2 ",
        );
    }
    #[test]
    fn gen_col_eqs_case_acc_else_col_end_comma_um_qp_is_expected() {
        assert_q(
            &super::gen_col_eqs_case_acc_else_col_end_comma_um_qp(
                sql("name"),
                sql("when id = $1 then $2 "),
            ),
            "name = case when id = $1 then $2 else name end,",
        );
    }
    #[test]
    fn gen_um_query_string_is_expected() {
        assert_q(
            &super::gen_um_query_string(
                tbl("users"),
                sql("name = case ... end,"),
                sql("id"),
                sql("$1,$2"),
                sql("id,name"),
            ),
            "update users set name = case ... end, where id in ($1,$2) returning id,name",
        );
    }
    #[test]
    fn gen_uo_query_string_is_expected() {
        assert_q(
            &super::gen_uo_query_string(
                tbl("users"),
                sql("name = $2"),
                sql("id"),
                sql("$1"),
                sql("id,name"),
            ),
            "update users set name = $2 where id = $1 returning id,name",
        );
    }
    #[test]
    fn gen_dm_query_string_is_expected() {
        assert_q(
            &super::gen_dm_query_string(tbl("users"), sql("where id in ($1,$2)"), sql("id")),
            "delete from users where id in ($1,$2) returning id",
        );
    }
    #[test]
    fn gen_dlo_query_string_is_expected() {
        let (tbl, pk) = users_base();
        assert_q(
            &super::gen_dlo_query_string(tbl, pk),
            "delete from users where id = $1 returning id",
        );
    }
    #[test]
    fn gen_um_query_string_wraps_pk_selector_for_in_clause() {
        let v = super::gen_um_query_string(
            tbl("users"),
            sql("name = case ... end,"),
            sql("id"),
            sql("$1,$2"),
            sql("id,name"),
        );
        assert!(v.contains("where id in ($1,$2)"));
    }
    #[test]
    fn gen_delete_query_string_uses_provided_filter_without_rewrite() {
        let (tbl, pk) = users_base();
        assert_q(
            &super::gen_delete_query_string(
                tbl,
                pk,
                Some(sql("where id in ($1,$2) and active = true")),
            ),
            "delete from users where id in ($1,$2) and active = true returning id",
        );
    }
    #[test]
    fn gen_update_query_string_eq_keeps_selector_without_extra_wrapping() {
        assert_q(
            &super::gen_update_query_string(
                tbl("users"),
                sql("name = $2"),
                sql("id"),
                sql("$1"),
                sql("id,name"),
                super::UpdateSelectorFmt::Eq,
            ),
            "update users set name = $2 where id = $1 returning id,name",
        );
    }
    #[test]
    fn gen_update_query_string_in_list_wraps_selector_once() {
        assert_q(
            &super::gen_update_query_string(
                tbl("users"),
                sql("name = case ... end,"),
                sql("id"),
                sql("$1,$2"),
                sql("id,name"),
                super::UpdateSelectorFmt::InList,
            ),
            "update users set name = case ... end, where id in ($1,$2) returning id,name",
        );
    }
}
