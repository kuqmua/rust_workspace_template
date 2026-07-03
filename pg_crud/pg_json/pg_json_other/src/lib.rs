#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct BoolAsJsonbBoolean(pg_crud_cmn::JsonFieldRights);

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct PgJsonOtherLeaf(wh_flts::RgxCase);

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct StringAsJsonbString(pg_crud_cmn::JsonFieldRights);

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct UuidUuidAsJsonbString(pg_crud_cmn::JsonFieldRights);
