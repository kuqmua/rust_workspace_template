#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddSerdeSkipSerializingIfVecIsEmptyAnn {
    False,
    True,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IdentPattern {
    ArrNlWithIdentifier,
    ArrNnWithId,
    StdrtNlWithoutId,
    StdrtNnWithId,
    StdrtNnWithoutId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IsStdrtWithId {
    False,
    True,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NewTypeOrStructDcl {
    NewType,
    StructDcl,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PgJsonSubtype {
    Cr,
    CrForQuery,
    Rd,
    RdIds,
    RdInn,
    Sel,
    Tt,
    Upd,
    UpdForQuery,
    Wh,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PgJsonSubtypeTtOrCr {
    Cr,
    Tt,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PgTypeSubtype {
    Rd,
    Upd,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RdWithOrWithoutAnnOrInn {
    Inn,
    WithSerdeOptIsNoneAnn,
    WithoutSerdeOptIsNoneAnn,
}
