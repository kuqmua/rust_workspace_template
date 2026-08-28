#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
use super::{HEALTH_COMPONENTS_MAX_LEN, HealthComponent, HealthComponentsError};

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, PartialEq, Eq, serde::Serialize,
)]
pub struct HealthComponents(pub(super) Vec<HealthComponent>);
impl utoipa::PartialSchema for HealthComponents {
    fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
        <bounded_types::BoundedVec<
            HealthComponent,
            { constants_usize::ZERO },
            HEALTH_COMPONENTS_MAX_LEN,
        > as utoipa::PartialSchema>::schema()
    }
}
impl utoipa::ToSchema for HealthComponents {}
impl From<[HealthComponent; 1]> for HealthComponents {
    fn from(value: [HealthComponent; 1]) -> Self {
        Self(Vec::from(value))
    }
}
impl From<[HealthComponent; 2]> for HealthComponents {
    fn from(value: [HealthComponent; 2]) -> Self {
        Self(Vec::from(value))
    }
}
impl TryFrom<Vec<HealthComponent>> for HealthComponents {
    type Error = HealthComponentsError;

    fn try_from(value: Vec<HealthComponent>) -> Result<Self, Self::Error> {
        bounded_types::BoundedVec::<
            HealthComponent,
            { constants_usize::ZERO },
            HEALTH_COMPONENTS_MAX_LEN,
        >::try_from(value)
        .map(bounded_types::BoundedVec::into_inner)
        .map(Self)
        .map_err(|_error| HealthComponentsError)
    }
}
impl<'de> serde::Deserialize<'de> for HealthComponents {
    fn deserialize<Deserializer>(deserializer: Deserializer) -> Result<Self, Deserializer::Error>
    where
        Deserializer: serde::Deserializer<'de>,
    {
        let value = <bounded_types::BoundedVec<
            HealthComponent,
            { constants_usize::ZERO },
            HEALTH_COMPONENTS_MAX_LEN,
        > as serde::Deserialize>::deserialize(deserializer)?
        .into_inner();
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
