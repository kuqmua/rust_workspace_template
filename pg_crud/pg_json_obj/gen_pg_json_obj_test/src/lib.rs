#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GenPgJsonObjTest<T>(pg_json_obj::PgJsonObjUnqVec<T>);
