use super::RouteFamily;

pub trait RouteInFamily<Family>
where
    Family: RouteFamily,
{
}
