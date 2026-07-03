#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct PgCrudRouteValidationStatus(route_validators::RouteValidationStatusCode);

impl From<route_validators::RouteValidationStatusCode> for PgCrudRouteValidationStatus {
    fn from(value: route_validators::RouteValidationStatusCode) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PgCrudFacade<T> {
    equality: pg_crud_cmn::EqOprtr,
    json: pg_json::PgJsonFacade,
    json_object: pg_json_obj::PgJsonObjUnqVec<T>,
    route_status: PgCrudRouteValidationStatus,
    table: pg_tbl::PgTableLeaf,
    types: pg_types::PgTypesFacade,
    where_filter_format: wh_flts::EncodeFormat,
}

impl<T> PgCrudFacade<T> {
    #[must_use]
    pub const fn new(
        equality: pg_crud_cmn::EqOprtr,
        json: pg_json::PgJsonFacade,
        json_object: pg_json_obj::PgJsonObjUnqVec<T>,
        route_status: PgCrudRouteValidationStatus,
        table: pg_tbl::PgTableLeaf,
        types: pg_types::PgTypesFacade,
        where_filter_format: wh_flts::EncodeFormat,
    ) -> Self {
        Self {
            equality,
            json,
            json_object,
            route_status,
            table,
            types,
            where_filter_format,
        }
    }
}
