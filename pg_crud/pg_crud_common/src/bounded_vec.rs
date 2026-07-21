#[derive(
    Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord, newtype::Display, newtype::FromInner,
)]
#[allow(clippy::module_name_repetitions)] // the public name remains explicit when imported outside this module
pub struct BoundedVecLen(usize);
impl BoundedVecLen {
    #[must_use]
    pub const fn get(self) -> usize {
        self.0
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
#[allow(clippy::module_name_repetitions)] // callers need an unambiguous error name in public signatures
pub enum BoundedVecError {
    #[error("bounded vector length {actual} exceeds limit {max}")]
    AboveMax {
        actual: BoundedVecLen,
        max: BoundedVecLen,
    },
    #[error("bounded vector length {actual} is below minimum {min}")]
    BelowMin {
        actual: BoundedVecLen,
        min: BoundedVecLen,
    },
    #[error("bounded vector minimum {min} exceeds maximum {max}")]
    InvalidBounds {
        min: BoundedVecLen,
        max: BoundedVecLen,
    },
}
#[derive(Clone, Debug, Eq, PartialEq, newtype::AsRefTarget, newtype::IntoInnerFrom)]
pub struct BoundedVec<T, const MIN: usize, const MAX: usize>(Vec<T>);
impl<T, const MIN: usize, const MAX: usize> BoundedVec<T, MIN, MAX> {
    #[must_use]
    pub const fn as_slice(&self) -> &[T] {
        self.0.as_slice()
    }
    #[must_use]
    pub fn len(&self) -> BoundedVecLen {
        BoundedVecLen::from(self.0.len())
    }
}
impl<T, const MIN: usize, const MAX: usize> TryFrom<Vec<T>> for BoundedVec<T, MIN, MAX> {
    type Error = BoundedVecError;
    fn try_from(value: Vec<T>) -> Result<Self, Self::Error> {
        if MIN > MAX {
            return Err(BoundedVecError::InvalidBounds {
                min: BoundedVecLen::from(MIN),
                max: BoundedVecLen::from(MAX),
            });
        }
        let actual = BoundedVecLen::from(value.len());
        if value.len() < MIN {
            Err(BoundedVecError::BelowMin {
                actual,
                min: BoundedVecLen::from(MIN),
            })
        } else if value.len() > MAX {
            Err(BoundedVecError::AboveMax {
                actual,
                max: BoundedVecLen::from(MAX),
            })
        } else {
            Ok(Self(value))
        }
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
        self.0.serialize(serializer)
    }
}
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
        if MIN > MAX {
            return Err(serde::de::Error::custom(BoundedVecError::InvalidBounds {
                min: BoundedVecLen::from(MIN),
                max: BoundedVecLen::from(MAX),
            }));
        }
        let mut values = Vec::with_capacity(seq.size_hint().unwrap_or(MIN).min(MAX));
        while let Some(value) = seq.next_element()? {
            if values.len() == MAX {
                return Err(serde::de::Error::custom(BoundedVecError::AboveMax {
                    actual: BoundedVecLen::from(MAX.saturating_add(1usize)),
                    max: BoundedVecLen::from(MAX),
                }));
            }
            values.push(value);
        }
        if values.len() < MIN {
            Err(serde::de::Error::custom(BoundedVecError::BelowMin {
                actual: BoundedVecLen::from(values.len()),
                min: BoundedVecLen::from(MIN),
            }))
        } else {
            Ok(BoundedVec(values))
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
        deserializer.deserialize_seq(StdPhantomDataBoundedVecVisitor(std::marker::PhantomData))
    }
}
impl<T: schemars::JsonSchema, const MIN: usize, const MAX: usize> schemars::JsonSchema
    for BoundedVec<T, MIN, MAX>
{
    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        let mut schema = generator.subschema_for::<Vec<T>>();
        let _previous_min = schema.insert(str_constants::MINITEMS.to_owned(), MIN.into());
        let _previous_max = schema.insert(str_constants::MAXITEMS.to_owned(), MAX.into());
        schema
    }
    fn schema_id() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Owned(format!("{}::BoundedVec<{MIN},{MAX}>", T::schema_id()))
    }
    fn schema_name() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Owned(format!("BoundedVec_{MIN}_{MAX}_{}", T::schema_name()))
    }
}
impl<'schema_lt, T: utoipa::PartialSchema, const MIN: usize, const MAX: usize>
    utoipa::ToSchema<'schema_lt> for BoundedVec<T, MIN, MAX>
{
    fn schema() -> (
        &'schema_lt str,
        utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>,
    ) {
        (
            str_constants::BOUNDEDVEC,
            utoipa::openapi::ArrayBuilder::new()
                .items(<T as utoipa::PartialSchema>::schema())
                .min_items(Some(MIN))
                .max_items(Some(MAX))
                .build()
                .into(),
        )
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn try_from_enforces_inclusive_bounds() {
        assert!(matches!(
            super::BoundedVec::<u8, 1, 2>::try_from(Vec::new()),
            Err(super::BoundedVecError::BelowMin { .. })
        ));
        assert_eq!(
            super::BoundedVec::<u8, 1, 2>::try_from(vec![1u8])
                .expect("0901ec3d")
                .as_slice(),
            &[1u8]
        );
        assert_eq!(
            super::BoundedVec::<u8, 1, 2>::try_from(vec![1u8, 2u8])
                .expect("324b4da9")
                .as_slice(),
            &[1u8, 2u8]
        );
        assert!(matches!(
            super::BoundedVec::<u8, 1, 2>::try_from(vec![1u8, 2u8, 3u8]),
            Err(super::BoundedVecError::AboveMax { .. })
        ));
    }
    #[test]
    fn invalid_bounds_are_rejected() {
        assert!(matches!(
            super::BoundedVec::<u8, 2, 1>::try_from(vec![1u8]),
            Err(super::BoundedVecError::InvalidBounds { .. })
        ));
    }
    #[test]
    fn serde_round_trip_and_limits_are_stable() {
        let value = <super::BoundedVec<u8, 1, 2> as serde::Deserialize>::deserialize(
            serde::de::value::SeqDeserializer::<_, serde::de::value::Error>::new(
                [1u8, 2u8].into_iter(),
            ),
        )
        .expect("9dcb60bc");
        assert_eq!(value.as_slice(), &[1u8, 2u8]);
        let below_min = <super::BoundedVec<u8, 1, 2> as serde::Deserialize>::deserialize(
            serde::de::value::SeqDeserializer::<_, serde::de::value::Error>::new(
                std::iter::empty::<u8>(),
            ),
        );
        let _error = below_min.expect_err(str_constants::CBBF6ACF);
        let error = <super::BoundedVec<u8, 1, 2> as serde::Deserialize>::deserialize(
            serde::de::value::SeqDeserializer::<_, serde::de::value::Error>::new(
                [1u8, 2u8, 3u8, 4u8].into_iter(),
            ),
        )
        .expect_err(str_constants::VALUE_91C59B94);
        assert!(error.to_string().contains("length 3 exceeds limit 2"));
    }
    #[test]
    fn schemas_match_runtime_bounds() {
        let schema = schemars::schema_for!(super::BoundedVec<u8, 1, 2>);
        assert_eq!(
            schema
                .get("minItems")
                .and_then(sqlx::types::JsonValue::as_u64),
            Some(1u64)
        );
        assert_eq!(
            schema
                .get("maxItems")
                .and_then(sqlx::types::JsonValue::as_u64),
            Some(2u64)
        );
        let open_api_schema = <super::BoundedVec<u8, 1, 2> as utoipa::ToSchema>::schema().1;
        let utoipa::openapi::RefOr::T(utoipa::openapi::schema::Schema::Array(array)) =
            open_api_schema
        else {
            panic!("06be97f2");
        };
        assert_eq!(array.min_items, Some(1usize));
        assert_eq!(array.max_items, Some(2usize));
    }
}
