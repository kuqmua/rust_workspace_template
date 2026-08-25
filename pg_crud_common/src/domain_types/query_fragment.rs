#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::AsRefStr,
    newtype::DerefTarget,
    newtype::Display,
    newtype::IntoInner,
)]
#[allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the private parent module assembles query fragments without widening public API"
)]
pub struct QueryPartFragment(String);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    newtype::FromInner,
)]
pub struct ReadQueryBindIndexNonZeroU32(std::num::NonZeroU32);
impl From<crate::domain_types::PgCrudStringWrapperTryFromStringError> for QueryPartFragment {
    fn from(value: crate::domain_types::PgCrudStringWrapperTryFromStringError) -> Self {
        Self(value.to_string())
    }
}
impl TryFrom<String> for QueryPartFragment {
    type Error = crate::domain_types::PgCrudStringWrapperTryFromStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > crate::domain_types::PG_CRUD_STRING_WRAPPER_MAX_LEN {
            return Err(Self::Error::TooLong {
                len: value.len(),
                max: crate::domain_types::PG_CRUD_STRING_WRAPPER_MAX_LEN,
            });
        }
        Ok(Self(value))
    }
}
impl std::fmt::Write for QueryPartFragment {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        if self
            .0
            .len()
            .checked_add(s.len())
            .is_none_or(|length| length > crate::domain_types::PG_CRUD_STRING_WRAPPER_MAX_LEN)
        {
            return Err(std::fmt::Error);
        }
        self.0.push_str(s);
        Ok(())
    }
}
impl QueryPartFragment {
    pub(super) fn append_read_bind_index(
        &mut self,
        bind_index: ReadQueryBindIndexNonZeroU32,
    ) -> Result<(), crate::domain_types::ReadQueryPlanError> {
        let mut digits = [constants_u8::ZERO; 10usize];
        let mut value = bind_index.0.get();
        let mut start = digits.len();
        while value != constants_u32::ZERO {
            start = start.saturating_sub(constants_usize::ONE);
            let quotient = value
                .checked_div(10u32)
                .ok_or(crate::domain_types::ReadQueryPlanError)?;
            let digit = match value.saturating_sub(quotient.saturating_mul(10u32)) {
                constants_u32::ZERO => b'0',
                1u32 => b'1',
                2u32 => b'2',
                3u32 => b'3',
                4u32 => b'4',
                5u32 => b'5',
                6u32 => b'6',
                7u32 => b'7',
                8u32 => b'8',
                9u32 => b'9',
                _ => return Err(crate::domain_types::ReadQueryPlanError),
            };
            *digits
                .get_mut(start)
                .ok_or(crate::domain_types::ReadQueryPlanError)? = digit;
            value = quotient;
        }
        digits
            .get(start..)
            .ok_or(crate::domain_types::ReadQueryPlanError)?
            .iter()
            .for_each(|digit| self.0.push(char::from(*digit)));
        Ok(())
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy)]
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
        f.debug_tuple(constants_str::SQLCOLUMNREF).finish()
    }
}
impl std::fmt::Display for SqlColumnRef<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn write_does_not_grow_fragment_above_limit() {
        let mut fragment = super::QueryPartFragment::try_from(
            "x".repeat(crate::domain_types::PG_CRUD_STRING_WRAPPER_MAX_LEN),
        )
        .expect("63af01f6 write_does_not_grow_fragment_above_limit invariant must hold");
        let write_result = std::fmt::Write::write_str(&mut fragment, constants_str::X);
        assert_eq!(write_result, Err(std::fmt::Error));
        assert_eq!(
            fragment.as_ref().len(),
            crate::domain_types::PG_CRUD_STRING_WRAPPER_MAX_LEN
        );
    }
}
