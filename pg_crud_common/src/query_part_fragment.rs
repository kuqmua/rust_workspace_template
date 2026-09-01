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
pub struct QueryPartFragment(String);
impl
    From<crate::pg_crud_string_wrapper_try_from_string_error::PgCrudStringWrapperTryFromStringError>
    for QueryPartFragment
{
    fn from(
        value: crate::pg_crud_string_wrapper_try_from_string_error::PgCrudStringWrapperTryFromStringError,
    ) -> Self {
        Self(value.to_string())
    }
}
impl TryFrom<String> for QueryPartFragment {
    type Error =
        crate::pg_crud_string_wrapper_try_from_string_error::PgCrudStringWrapperTryFromStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > crate::pg_crud_string_wrapper_max_len::PG_CRUD_STRING_WRAPPER_MAX_LEN {
            return Err(Self::Error::TooLong {
                len: value.len(),
                max: crate::pg_crud_string_wrapper_max_len::PG_CRUD_STRING_WRAPPER_MAX_LEN,
            });
        }
        Ok(Self(value))
    }
}
impl std::fmt::Write for QueryPartFragment {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        if self.0.len().checked_add(s.len()).is_none_or(|length| {
            length > crate::pg_crud_string_wrapper_max_len::PG_CRUD_STRING_WRAPPER_MAX_LEN
        }) {
            return Err(std::fmt::Error);
        }
        self.0.push_str(s);
        Ok(())
    }
}
impl QueryPartFragment {
    pub(crate) fn append_read_bind_index(
        &mut self,
        bind_index: super::read_query_bind_index_non_zero_u32::ReadQueryBindIndexNonZeroU32,
    ) -> Result<(), crate::read_query_plan_error::ReadQueryPlanError> {
        let mut digits = [constants_u8::ZERO; 10usize];
        let mut value = bind_index.get();
        let mut start = digits.len();
        while value != constants_u32::ZERO {
            start = start.saturating_sub(constants_usize::ONE);
            let quotient = value
                .checked_div(10u32)
                .ok_or(crate::read_query_plan_error::ReadQueryPlanError::TooManyFragments)?;
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
                _ => {
                    return Err(crate::read_query_plan_error::ReadQueryPlanError::TooManyFragments);
                }
            };
            *digits
                .get_mut(start)
                .ok_or(crate::read_query_plan_error::ReadQueryPlanError::TooManyFragments)? = digit;
            value = quotient;
        }
        digits
            .get(start..)
            .ok_or(crate::read_query_plan_error::ReadQueryPlanError::TooManyFragments)?
            .iter()
            .for_each(|digit| self.0.push(char::from(*digit)));
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test_write_does_not_grow_fragment_above_limit() {
        let mut fragment = crate::query_part_fragment::QueryPartFragment::try_from(
            constants_str::X
                .repeat(crate::pg_crud_string_wrapper_max_len::PG_CRUD_STRING_WRAPPER_MAX_LEN),
        )
        .expect(constants_str::DIAGNOSTIC_63AF01F6);
        let write_result = std::fmt::Write::write_str(&mut fragment, constants_str::X);
        assert_eq!(write_result, Err(std::fmt::Error));
        assert_eq!(
            fragment.as_ref().len(),
            crate::pg_crud_string_wrapper_max_len::PG_CRUD_STRING_WRAPPER_MAX_LEN
        );
    }
}
