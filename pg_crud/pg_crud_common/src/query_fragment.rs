#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    optml::Optml,
    newtype::AsRefStr,
    newtype::DerefTarget,
    newtype::Display,
)]
#[allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the private parent module assembles query fragments without widening public API"
)]
pub struct QueryPartFragment(pub(super) String);
impl QueryPartFragment {
    #[must_use]
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl From<crate::PgCrudStringWrapperTryFromStringError> for QueryPartFragment {
    fn from(value: crate::PgCrudStringWrapperTryFromStringError) -> Self {
        Self(value.to_string())
    }
}
impl TryFrom<String> for QueryPartFragment {
    type Error = crate::PgCrudStringWrapperTryFromStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > crate::PG_CRUD_STRING_WRAPPER_MAX_LEN {
            return Err(Self::Error::TooLong {
                len: value.len(),
                max: crate::PG_CRUD_STRING_WRAPPER_MAX_LEN,
            });
        }
        Ok(Self(value))
    }
}
impl std::fmt::Write for QueryPartFragment {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.0.push_str(s);
        Ok(())
    }
}
#[derive(Clone, Copy)]
pub struct SqlColumnRef<'column_lt>(&'column_lt dyn std::fmt::Display);
impl<'column_lt, T> From<&'column_lt T> for SqlColumnRef<'column_lt>
where
    T: std::fmt::Display,
{
    fn from(value: &'column_lt T) -> Self {
        Self(value)
    }
}
impl std::fmt::Debug for SqlColumnRef<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple(str_constants::SQLCOLUMNREF).finish()
    }
}
impl std::fmt::Display for SqlColumnRef<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
