#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldAttribute {
    EoHashMapKStringVLoc,
    EoHashMapKStringVToErrString,
    EoHashMapKStringVToErrStringSerde,
    EoLoc,
    EoToErrString,
    EoToErrStringSerde,
    EoVecLoc,
    EoVecToErrString,
    EoVecToErrStringSerde,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SerdeVersionOfNamedSynVariantTokenStream;
