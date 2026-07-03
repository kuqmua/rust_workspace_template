#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Animal {
    cat_collection: AnimalField808,
    cat_collection_optional: AnimalField809,
    doggie_collection: AnimalField807,
    doggie_collection_optional: AnimalField806,
    signed_integer: AnimalField0,
    signed_integer_array: AnimalField2,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct AnimalField0;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct AnimalField2;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct AnimalField806;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct AnimalField807;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct AnimalField808;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct AnimalField809;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Cat {
    signed_integer: CatField0,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct CatField0;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Doggie {
    cat_collection: DoggieField808,
    cat_collection_optional: DoggieField809,
    cat_object: DoggieField806,
    cat_object_optional: DoggieField807,
    signed_integer: DoggieField0,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct DoggieField0;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct DoggieField806;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct DoggieField807;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct DoggieField808;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct DoggieField809;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct ServerRouteValidationStatus(route_validators::RouteValidationStatusCode);

impl From<route_validators::RouteValidationStatusCode> for ServerRouteValidationStatus {
    fn from(value: route_validators::RouteValidationStatusCode) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct ServerApiBoundary(ServerRouteValidationStatus);

impl ServerApiBoundary {
    #[must_use]
    pub const fn new(status_code: ServerRouteValidationStatus) -> Self {
        Self(status_code)
    }
}

impl Animal {
    #[must_use]
    pub const fn field_0(&self) -> AnimalField0 {
        self.signed_integer
    }

    #[must_use]
    pub const fn field_2(&self) -> AnimalField2 {
        self.signed_integer_array
    }

    #[must_use]
    pub const fn field_806(&self) -> AnimalField806 {
        self.doggie_collection_optional
    }

    #[must_use]
    pub const fn field_807(&self) -> AnimalField807 {
        self.doggie_collection
    }

    #[must_use]
    pub const fn field_808(&self) -> AnimalField808 {
        self.cat_collection
    }

    #[must_use]
    pub const fn field_809(&self) -> AnimalField809 {
        self.cat_collection_optional
    }

    #[must_use]
    pub const fn new() -> Self {
        Self {
            cat_collection: AnimalField808,
            cat_collection_optional: AnimalField809,
            doggie_collection: AnimalField807,
            doggie_collection_optional: AnimalField806,
            signed_integer: AnimalField0,
            signed_integer_array: AnimalField2,
        }
    }
}

impl Default for Animal {
    fn default() -> Self {
        Self::new()
    }
}

impl Cat {
    #[must_use]
    pub const fn field_0(&self) -> CatField0 {
        self.signed_integer
    }

    #[must_use]
    pub const fn new() -> Self {
        Self {
            signed_integer: CatField0,
        }
    }
}

impl Default for Cat {
    fn default() -> Self {
        Self::new()
    }
}

impl Doggie {
    #[must_use]
    pub const fn field_0(&self) -> DoggieField0 {
        self.signed_integer
    }

    #[must_use]
    pub const fn field_806(&self) -> DoggieField806 {
        self.cat_object
    }

    #[must_use]
    pub const fn field_807(&self) -> DoggieField807 {
        self.cat_object_optional
    }

    #[must_use]
    pub const fn field_808(&self) -> DoggieField808 {
        self.cat_collection
    }

    #[must_use]
    pub const fn field_809(&self) -> DoggieField809 {
        self.cat_collection_optional
    }

    #[must_use]
    pub const fn new() -> Self {
        Self {
            cat_collection: DoggieField808,
            cat_collection_optional: DoggieField809,
            cat_object: DoggieField806,
            cat_object_optional: DoggieField807,
            signed_integer: DoggieField0,
        }
    }
}

impl Default for Doggie {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn animal_exposes_source_active_field_markers() -> Result<(), String> {
        let animal = crate::Animal::new();
        if animal.field_0() == crate::AnimalField0
            && animal.field_2() == crate::AnimalField2
            && animal.field_806() == crate::AnimalField806
            && animal.field_807() == crate::AnimalField807
            && animal.field_808() == crate::AnimalField808
            && animal.field_809() == crate::AnimalField809
        {
            return Ok(());
        }
        Err(format!("{animal:?}"))
    }

    #[test]
    fn cat_exposes_source_active_field_markers() -> Result<(), String> {
        let cat = crate::Cat::new();
        if cat.field_0() == crate::CatField0 {
            return Ok(());
        }
        Err(format!("{cat:?}"))
    }

    #[test]
    fn doggie_exposes_source_active_field_markers() -> Result<(), String> {
        let doggie = crate::Doggie::new();
        if doggie.field_0() == crate::DoggieField0
            && doggie.field_806() == crate::DoggieField806
            && doggie.field_807() == crate::DoggieField807
            && doggie.field_808() == crate::DoggieField808
            && doggie.field_809() == crate::DoggieField809
        {
            return Ok(());
        }
        Err(format!("{doggie:?}"))
    }
}
