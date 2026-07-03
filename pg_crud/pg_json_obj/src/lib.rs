#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PgJsonObjUnqVec<T>(pg_json_obj_cmn::UnqVec<T>);

impl<T> From<PgJsonObjUnqVec<T>> for Vec<T> {
    fn from(value: PgJsonObjUnqVec<T>) -> Self {
        value.0.into()
    }
}

impl<T> From<pg_json_obj_cmn::UnqVec<T>> for PgJsonObjUnqVec<T> {
    fn from(value: pg_json_obj_cmn::UnqVec<T>) -> Self {
        Self(value)
    }
}

impl<T> TryFrom<Vec<T>> for PgJsonObjUnqVec<T>
where
    T: PartialEq,
{
    type Error = pg_json_obj_cmn::UnqVecTryNewEr<T>;

    fn try_from(value: Vec<T>) -> Result<Self, Self::Error> {
        pg_json_obj_cmn::UnqVec::try_from(value).map(Self)
    }
}
