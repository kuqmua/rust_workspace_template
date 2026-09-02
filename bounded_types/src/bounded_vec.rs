#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype::DerefTarget,
    proc_macro_newtype::IntoIterator,
)]
pub struct BoundedVec<T, const MIN: usize, const MAX: usize>(Vec<T>);
impl<T, const MIN: usize, const MAX: usize> BoundedVec<T, MIN, MAX> {
    #[cfg(test)]
    pub(crate) const fn allocation_capacity(&self) -> usize {
        self.0.capacity()
    }

    pub fn validate_bounds() -> Result<(), crate::bounded_value_error::BoundedValueError> {
        crate::validate_len::validate_len::<MIN, MAX>(crate::bounded_len::BoundedLen::from(MIN))
    }

    #[must_use]
    pub const fn as_slice(&self) -> &[T] {
        self.0.as_slice()
    }

    #[must_use]
    pub fn into_inner(self) -> Vec<T> {
        self.0
    }

    #[must_use]
    pub fn len(&self) -> crate::bounded_len::BoundedLen {
        crate::bounded_len::BoundedLen::from(self.0.len())
    }

    pub fn try_push(
        &mut self,
        value: T,
    ) -> Result<(), crate::bounded_value_error::BoundedValueError> {
        let next_len = self
            .0
            .len()
            .checked_add(constants_usize::ONE)
            .ok_or_else(|| crate::bounded_value_error::BoundedValueError::AboveMax {
                actual: crate::bounded_len::BoundedLen::from(usize::MAX),
                max: crate::bounded_len::BoundedLen::from(MAX),
            })?;
        crate::validate_len::validate_len::<0, MAX>(crate::bounded_len::BoundedLen::from(next_len))
            .map(|()| self.0.push(value))
    }
}
impl<T> BoundedVec<T, 0, { usize::MAX }> {
    pub fn try_from_collection_vec(
        value: Vec<T>,
    ) -> Result<Self, crate::bounded_value_error::BoundedValueError> {
        let bounded_value =
            BoundedVec::<T, 0, { crate::collection_max_len::COLLECTION_MAX_LEN }>::try_from(value)?;
        Self::try_from(bounded_value.into_inner())
    }

    #[must_use]
    pub fn from_max_iter<Values>(values: Values) -> Self
    where
        Values: IntoIterator<Item = T>,
    {
        let value_iter = values.into_iter();
        let capacity = value_iter
            .size_hint()
            .0
            .min(crate::serde_prealloc_max_items::SERDE_PREALLOC_MAX_ITEMS);
        let mut bounded = Self::from([]);
        bounded.0.reserve(capacity);
        value_iter.for_each(|value| bounded.push_max_capacity(value));
        bounded
    }

    pub fn push_max_capacity(&mut self, value: T) {
        self.0.push(value);
    }
}
impl<T, const MAX: usize> Default for BoundedVec<T, 0, MAX> {
    fn default() -> Self {
        Self::from([])
    }
}
impl<T, const MAX: usize> From<[T; 0]> for BoundedVec<T, 0, MAX> {
    fn from(value: [T; 0]) -> Self {
        let _: [T; 0] = value;
        Self(Vec::new())
    }
}
impl<T, const MIN: usize, const MAX: usize> TryFrom<Vec<T>> for BoundedVec<T, MIN, MAX> {
    type Error = crate::bounded_value_error::BoundedValueError;

    fn try_from(value: Vec<T>) -> Result<Self, Self::Error> {
        crate::validate_len::validate_len::<MIN, MAX>(crate::bounded_len::BoundedLen::from(
            value.len(),
        ))
        .map(|()| Self(value))
    }
}
impl<T, const MIN: usize, const MAX: usize> AsRef<[T]> for BoundedVec<T, MIN, MAX> {
    fn as_ref(&self) -> &[T] {
        self.as_slice()
    }
}
impl<T: serde::Serialize, const MIN: usize, const MAX: usize> serde::Serialize
    for BoundedVec<T, MIN, MAX>
{
    fn serialize<Serializer>(
        &self,
        serializer: Serializer,
    ) -> Result<Serializer::Ok, Serializer::Error>
    where
        Serializer: serde::Serializer,
    {
        serde::Serialize::serialize(&self.0, serializer)
    }
}
impl<'de, T: serde::Deserialize<'de>, const MIN: usize, const MAX: usize> serde::Deserialize<'de>
    for BoundedVec<T, MIN, MAX>
{
    fn deserialize<Deserializer>(deserializer: Deserializer) -> Result<Self, Deserializer::Error>
    where
        Deserializer: serde::Deserializer<'de>,
    {
        deserializer.deserialize_seq(
            super::bounded_vec_visitor_phantom_data::BoundedVecVisitorPhantomData::from(
                std::marker::PhantomData,
            ),
        )
    }
}
impl<T: utoipa::PartialSchema, const MIN: usize, const MAX: usize> utoipa::__dev::ComposeSchema
    for BoundedVec<T, MIN, MAX>
{
    fn compose(
        _new_generics: Vec<utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>>,
    ) -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
        let builder = utoipa::openapi::ArrayBuilder::new()
            .items(<T as utoipa::PartialSchema>::schema())
            .min_items(Some(MIN));
        if MAX == usize::MAX {
            builder.build().into()
        } else {
            builder.max_items(Some(MAX)).build().into()
        }
    }
}
impl<T: utoipa::ToSchema, const MIN: usize, const MAX: usize> utoipa::ToSchema
    for BoundedVec<T, MIN, MAX>
{
    fn name() -> std::borrow::Cow<'static, str> {
        let mut name = T::name().into_owned();
        name.push('_');
        name.push_str(constants_str::BOUNDEDVEC);
        name.push('_');
        name.push_str(MIN.to_string().as_str());
        name.push('_');
        name.push_str(MAX.to_string().as_str());
        std::borrow::Cow::Owned(name)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_vector_growth_respects_capacity() {
        let mut value = super::BoundedVec::<u8, 0, 1>::default();
        assert!(matches!(value.try_push(1u8), Ok(())));
        assert!(matches!(
            value.try_push(2u8),
            Err(crate::bounded_value_error::BoundedValueError::AboveMax { .. })
        ));
    }

    #[test]
    fn test_max_iterator_uses_bounded_size_hint_capacity() {
        let value = super::BoundedVec::from_max_iter([1u8, 2u8, 3u8]);
        assert!(value.allocation_capacity() >= 3usize);
    }
}
