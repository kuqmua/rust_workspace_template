#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, PartialEq, Eq, serde::Serialize,
)]
pub struct HealthComponents(pub(super) Vec<crate::health_component::HealthComponent>);
impl utoipa::PartialSchema for HealthComponents {
    fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
        <bounded_types::bounded_vec::BoundedVec<
            crate::health_component::HealthComponent,
            { constants_usize::ZERO },
            { crate::health_components_max_len::HEALTH_COMPONENTS_MAX_LEN },
        > as utoipa::PartialSchema>::schema()
    }
}
impl utoipa::ToSchema for HealthComponents {}
impl From<[crate::health_component::HealthComponent; 1]> for HealthComponents {
    fn from(value: [crate::health_component::HealthComponent; 1]) -> Self {
        Self(Vec::from(value))
    }
}
impl From<[crate::health_component::HealthComponent; 2]> for HealthComponents {
    fn from(value: [crate::health_component::HealthComponent; 2]) -> Self {
        Self(Vec::from(value))
    }
}
impl TryFrom<Vec<crate::health_component::HealthComponent>> for HealthComponents {
    type Error = crate::health_components_error::HealthComponentsError;

    fn try_from(value: Vec<crate::health_component::HealthComponent>) -> Result<Self, Self::Error> {
        bounded_types::bounded_vec::BoundedVec::<
            crate::health_component::HealthComponent,
            { constants_usize::ZERO },
            { crate::health_components_max_len::HEALTH_COMPONENTS_MAX_LEN },
        >::try_from(value)
        .map(bounded_types::bounded_vec::BoundedVec::into_inner)
        .map(Self)
        .map_err(|_error| crate::health_components_error::HealthComponentsError::TooMany)
    }
}
impl<'de> serde::Deserialize<'de> for HealthComponents {
    fn deserialize<Deserializer>(deserializer: Deserializer) -> Result<Self, Deserializer::Error>
    where
        Deserializer: serde::Deserializer<'de>,
    {
        let value = <bounded_types::bounded_vec::BoundedVec<
            crate::health_component::HealthComponent,
            { constants_usize::ZERO },
            { crate::health_components_max_len::HEALTH_COMPONENTS_MAX_LEN },
        > as serde::Deserialize>::deserialize(deserializer)?
        .into_inner();
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
