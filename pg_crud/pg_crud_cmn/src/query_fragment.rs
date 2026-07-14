#[derive(Debug, Clone, PartialEq, Eq, optml::Optml, newtype::Newtype)]
#[newtype(as_ref_str, deref_target, display)]
#[allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the private parent module assembles query fragments without widening public API"
)]
pub struct QpFragment(pub(super) String);
impl QpFragment {
    #[must_use]
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl From<crate::PgCrudStringWrapperTryFromStringEr> for QpFragment {
    fn from(value: crate::PgCrudStringWrapperTryFromStringEr) -> Self {
        Self(value.to_string())
    }
}
impl TryFrom<String> for QpFragment {
    type Error = crate::PgCrudStringWrapperTryFromStringEr;
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
impl std::fmt::Write for QpFragment {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.0.push_str(s);
        Ok(())
    }
}
#[derive(Clone, Copy)]
pub struct SqlColRef<'col_lt>(&'col_lt dyn std::fmt::Display);
impl<'col_lt, T> From<&'col_lt T> for SqlColRef<'col_lt>
where
    T: std::fmt::Display,
{
    fn from(value: &'col_lt T) -> Self {
        Self(value)
    }
}
impl std::fmt::Debug for SqlColRef<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("SqlColRef").finish()
    }
}
impl std::fmt::Display for SqlColRef<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
