#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub enum GeoJsonValidationError {
    #[error("GeoJSON coordinates are invalid")]
    Coordinates,
    #[error("GeoJSON document shape is invalid")]
    Document,
    #[error("GeoJSON text is invalid")]
    SerdeJson(#[source] crate::serde_json_geo_json_error::SerdeJsonGeoJsonError),
    #[error("GeoJSON document exceeds its maximum size")]
    TooLarge,
    #[error("GeoJSON geometry type is unsupported")]
    UnsupportedGeometry,
}
