use super::TypeContract;

pub trait HasTypeContract {
    fn type_contract() -> TypeContract;
}
