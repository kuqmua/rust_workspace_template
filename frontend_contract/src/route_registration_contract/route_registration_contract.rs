pub trait RouteRegistrationContract: Copy {
    fn method(self) -> super::super::RouteMethod;
    fn path(self) -> super::RegisteredRoutePath;
}
