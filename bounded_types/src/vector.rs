#[derive(Clone, Debug, Eq, PartialEq, newtype::DerefTarget, newtype::IntoIterator)]
pub struct BoundedVec<T, const MIN: usize, const MAX: usize>(Vec<T>);
impl<T, const MIN: usize, const MAX: usize> BoundedVec<T, MIN, MAX> {
    #[cfg(test)]
    pub(super) const fn allocation_capacity(&self) -> usize {
        self.0.capacity()
    }

    pub fn validate_bounds() -> Result<(), super::BoundedValueError> {
        super::validate_len::<MIN, MAX>(super::BoundedLen::from(MIN))
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
    pub fn len(&self) -> super::BoundedLen {
        super::BoundedLen::from(self.0.len())
    }

    pub fn try_push(&mut self, value: T) -> Result<(), super::BoundedValueError> {
        let next_len =
            self.0
                .len()
                .checked_add(1usize)
                .ok_or_else(|| super::BoundedValueError::AboveMax {
                    actual: super::BoundedLen::from(usize::MAX),
                    max: super::BoundedLen::from(MAX),
                })?;
        super::validate_len::<0, MAX>(super::BoundedLen::from(next_len))
            .map(|()| self.0.push(value))
    }
}
impl<T> BoundedVec<T, 0, { usize::MAX }> {
    #[must_use]
    pub fn from_max_iter<Values>(values: Values) -> Self
    where
        Values: IntoIterator<Item = T>,
    {
        let mut bounded = Self::default();
        values
            .into_iter()
            .for_each(|value| bounded.push_max_capacity(value));
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
    fn from(_value: [T; 0]) -> Self {
        Self(Vec::new())
    }
}
impl<T, const MIN: usize, const MAX: usize> TryFrom<Vec<T>> for BoundedVec<T, MIN, MAX> {
    type Error = super::BoundedValueError;

    fn try_from(value: Vec<T>) -> Result<Self, Self::Error> {
        super::validate_len::<MIN, MAX>(super::BoundedLen::from(value.len())).map(|()| Self(value))
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
#[derive(newtype::FromInner)]
struct StdPhantomDataBoundedVecVisitor<T, const MIN: usize, const MAX: usize>(
    std::marker::PhantomData<T>,
);
impl<'de, T: serde::Deserialize<'de>, const MIN: usize, const MAX: usize> serde::de::Visitor<'de>
    for StdPhantomDataBoundedVecVisitor<T, MIN, MAX>
{
    type Value = BoundedVec<T, MIN, MAX>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "an array with {MIN} to {MAX} items")
    }

    fn visit_seq<Seq>(self, mut seq: Seq) -> Result<Self::Value, Seq::Error>
    where
        Seq: serde::de::SeqAccess<'de>,
    {
        BoundedVec::<T, MIN, MAX>::validate_bounds().map_err(serde::de::Error::custom)?;
        let capacity = seq
            .size_hint()
            .unwrap_or(MIN)
            .min(MAX)
            .min(super::SERDE_PREALLOC_MAX_ITEMS);
        let mut values = Vec::with_capacity(capacity);
        loop {
            if values.len() == MAX {
                return seq.next_element::<serde::de::IgnoredAny>()?.map_or_else(
                    || BoundedVec::try_from(values).map_err(serde::de::Error::custom),
                    |_ignored| {
                        Err(serde::de::Error::custom(
                            super::BoundedValueError::AboveMax {
                                actual: super::BoundedLen::from(MAX.saturating_add(1usize)),
                                max: super::BoundedLen::from(MAX),
                            },
                        ))
                    },
                );
            }
            match seq.next_element()? {
                Some(value) => values.push(value),
                None => {
                    return BoundedVec::try_from(values).map_err(serde::de::Error::custom);
                }
            }
        }
    }
}
impl<'de, T: serde::Deserialize<'de>, const MIN: usize, const MAX: usize> serde::Deserialize<'de>
    for BoundedVec<T, MIN, MAX>
{
    fn deserialize<Deserializer>(deserializer: Deserializer) -> Result<Self, Deserializer::Error>
    where
        Deserializer: serde::Deserializer<'de>,
    {
        deserializer.deserialize_seq(StdPhantomDataBoundedVecVisitor::from(
            std::marker::PhantomData,
        ))
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
        name.push_str(str_constants::BOUNDEDVEC);
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
    fn vector_growth_respects_capacity() {
        let mut value = super::BoundedVec::<u8, 0, 1>::default();
        assert!(matches!(value.try_push(1u8), Ok(())));
        assert!(matches!(
            value.try_push(2u8),
            Err(super::super::BoundedValueError::AboveMax { .. })
        ));
    }
}
