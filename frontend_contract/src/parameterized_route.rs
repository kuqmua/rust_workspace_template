use super::{ParameterizedRoutePath, TypedRoute};

pub trait ParameterizedRoute: TypedRoute {
    type Parameter;
    fn path(parameter: &Self::Parameter) -> ParameterizedRoutePath;
}
