const PG_TBL_STRING_WRAPPER_MAX_LEN: usize = 1_048_576;
pub trait CombinationOfAppStateLogicTraits:
    config_lib::GetEnableApiGitCommitCheck
    + config_lib::GetMaximumSizeOfHttpBodyInBytes
    + config_lib::GetSrcPlaceType
    + config_lib::GetChronoTimezone
    + app_state::GetSqlxPgPool
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
#[derive(Debug, Clone, Copy, newtype::Newtype)]
#[newtype(as_ref_inner, display)]
pub struct PgTblNameRef<'lt>(&'lt str);
impl<'lt, T> From<&'lt T> for PgTblNameRef<'lt>
where
    T: AsRef<str> + ?Sized,
{
    fn from(value: &'lt T) -> Self {
        Self(value.as_ref())
    }
}
#[derive(Debug, Clone, Copy, newtype::Newtype)]
#[newtype(as_ref_inner, display)]
pub struct PgTblSqlFragmentRef<'lt>(&'lt str);
impl<'lt, T> From<&'lt T> for PgTblSqlFragmentRef<'lt>
where
    T: AsRef<str> + ?Sized,
{
    fn from(value: &'lt T) -> Self {
        Self(value.as_ref())
    }
}
#[derive(Debug, Clone, newtype::Newtype)]
#[newtype(display)]
pub struct PgTblQueryString(String);
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PgTblStringWrapperTryFromStringEr {
    TooLong { len: usize, max: usize },
}
impl std::fmt::Display for PgTblStringWrapperTryFromStringEr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TooLong { len, max } => {
                write!(
                    f,
                    "pg tbl string wrapper length {len} exceeds maximum {max}"
                )
            }
        }
    }
}
impl From<PgTblStringWrapperTryFromStringEr> for PgTblQueryString {
    fn from(value: PgTblStringWrapperTryFromStringEr) -> Self {
        Self(value.to_string())
    }
}
impl TryFrom<String> for PgTblQueryString {
    type Error = PgTblStringWrapperTryFromStringEr;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > PG_TBL_STRING_WRAPPER_MAX_LEN {
            return Err(Self::Error::TooLong {
                len: value.len(),
                max: PG_TBL_STRING_WRAPPER_MAX_LEN,
            });
        }
        Ok(Self(value))
    }
}
impl std::ops::Deref for PgTblQueryString {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[derive(Debug, Clone, newtype::Newtype)]
#[newtype(display)]
pub struct PgTblQpFragment(String);
impl From<PgTblStringWrapperTryFromStringEr> for PgTblQpFragment {
    fn from(value: PgTblStringWrapperTryFromStringEr) -> Self {
        Self(value.to_string())
    }
}
impl TryFrom<String> for PgTblQpFragment {
    type Error = PgTblStringWrapperTryFromStringEr;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > PG_TBL_STRING_WRAPPER_MAX_LEN {
            return Err(Self::Error::TooLong {
                len: value.len(),
                max: PG_TBL_STRING_WRAPPER_MAX_LEN,
            });
        }
        Ok(Self(value))
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
        InsertValuesFmt::Raw => PgTblQueryString::try_from(format!(
            "insert into {tbl} ({cols}) values {values} returning {cols_to_return}"
        ))
        .unwrap_or_else(PgTblQueryString::from),
        InsertValuesFmt::Wrapped => PgTblQueryString::try_from(format!(
            "insert into {tbl} ({cols}) values ({values}) returning {cols_to_return}"
        ))
        .unwrap_or_else(PgTblQueryString::from),
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
            PgTblQueryString::try_from(format!("select {sel_string} from {tbl} {wh_string}"))
                .unwrap_or_else(PgTblQueryString::from)
        }
        SelectWhereFmt::Where => {
            PgTblQueryString::try_from(format!("select {sel_string} from {tbl} where {wh_string}"))
                .unwrap_or_else(PgTblQueryString::from)
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
        UpdateSelectorFmt::Eq => PgTblQueryString::try_from(format!(
            "update {tbl} set {cols_or_els} where {pk_field_name} = {pk_selector} returning {cols_to_return}"
        )).unwrap_or_else(PgTblQueryString::from),
        UpdateSelectorFmt::InList => PgTblQueryString::try_from(format!(
            "update {tbl} set {cols_or_els} where {pk_field_name} in ({pk_selector}) returning {cols_to_return}"
        )).unwrap_or_else(PgTblQueryString::from),
    }
}
fn gen_delete_query_string(
    tbl: PgTblNameRef<'_>,
    pk_field_name: PgTblSqlFragmentRef<'_>,
    wh_string: Option<PgTblSqlFragmentRef<'_>>,
) -> PgTblQueryString {
    wh_string.map_or_else(
        || {
            PgTblQueryString::try_from(format!(
                "delete from {tbl} where {pk_field_name} = $1 returning {pk_field_name}"
            ))
            .unwrap_or_else(PgTblQueryString::from)
        },
        |v| {
            PgTblQueryString::try_from(format!("delete from {tbl} {v} returning {pk_field_name}"))
                .unwrap_or_else(PgTblQueryString::from)
        },
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
    let mut qp = String::with_capacity(
        col.as_ref()
            .len()
            .saturating_add(value.as_ref().len())
            .saturating_add(5),
    );
    if std::fmt::Write::write_fmt(&mut qp, format_args!("{col} = {value},")).is_err() {
        return PgTblQpFragment::try_from(String::default()).unwrap_or_else(PgTblQpFragment::from);
    }
    PgTblQpFragment::try_from(qp).unwrap_or_else(PgTblQpFragment::from)
}
#[must_use]
pub fn gen_when_col_id_then_v_um_qp(
    col: PgTblSqlFragmentRef<'_>,
    id: PgTblSqlFragmentRef<'_>,
    value: PgTblSqlFragmentRef<'_>,
) -> PgTblQpFragment {
    let mut qp = String::with_capacity(
        col.as_ref()
            .len()
            .saturating_add(id.as_ref().len())
            .saturating_add(value.as_ref().len())
            .saturating_add(15),
    );
    if std::fmt::Write::write_fmt(&mut qp, format_args!("when {col} = {id} then {value} ")).is_err()
    {
        return PgTblQpFragment::try_from(String::default()).unwrap_or_else(PgTblQpFragment::from);
    }
    PgTblQpFragment::try_from(qp).unwrap_or_else(PgTblQpFragment::from)
}
#[must_use]
pub fn gen_col_eqs_case_acc_else_col_end_comma_um_qp(
    col: PgTblSqlFragmentRef<'_>,
    acc: PgTblSqlFragmentRef<'_>,
) -> PgTblQpFragment {
    let mut qp = String::with_capacity(
        col.as_ref()
            .len()
            .saturating_mul(2)
            .saturating_add(acc.as_ref().len())
            .saturating_add(19),
    );
    if std::fmt::Write::write_fmt(&mut qp, format_args!("{col} = case {acc}else {col} end,"))
        .is_err()
    {
        return PgTblQpFragment::try_from(String::default()).unwrap_or_else(PgTblQpFragment::from);
    }
    PgTblQpFragment::try_from(qp).unwrap_or_else(PgTblQpFragment::from)
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
        super::PgTblNameRef::from(v)
    }
    fn sql(v: &'static str) -> super::PgTblSqlFragmentRef<'static> {
        super::PgTblSqlFragmentRef::from(v)
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
