use super::BetweenTryNewError;

#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    schemars::JsonSchema,
    optimal_memory_layout::OptimalMemoryLayout,
)]
pub struct Between<T>
where
    T: sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres>,
{
    start: T,
    end: T,
}
impl<T> utoipa::__dev::ComposeSchema for Between<T>
where
    T: sqlx::Type<sqlx::Postgres>
        + for<'encode_lt> sqlx::Encode<'encode_lt, sqlx::Postgres>
        + utoipa::ToSchema,
{
    fn compose(
        _new_generics: Vec<utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>>,
    ) -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
        utoipa::openapi::ObjectBuilder::new()
            .property(
                constants_str::PG_CRUD_START_FIELD,
                <T as utoipa::PartialSchema>::schema(),
            )
            .property(
                constants_str::PG_CRUD_END_FIELD,
                <T as utoipa::PartialSchema>::schema(),
            )
            .required(constants_str::PG_CRUD_START_FIELD)
            .required(constants_str::PG_CRUD_END_FIELD)
            .build()
            .into()
    }
}
impl<T> utoipa::ToSchema for Between<T>
where
    T: sqlx::Type<sqlx::Postgres>
        + for<'encode_lt> sqlx::Encode<'encode_lt, sqlx::Postgres>
        + utoipa::ToSchema,
{
    fn name() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed(constants_str::PG_CRUD_BETWEEN_SCHEMA_NAME)
    }
}
impl<T: sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres> + PartialOrd>
    Between<T>
{
    pub fn try_new(start: T, end: T) -> Result<Self, BetweenTryNewError<T>> {
        if start < end {
            Ok(Self { start, end })
        } else {
            Err(BetweenTryNewError::StartMoreOrEqToEnd {
                start,
                end,
                location: location_macros::location!(),
            })
        }
    }
}
#[allow(unused_qualifications)]
#[allow(clippy::absolute_paths)]
#[allow(clippy::arbitrary_source_item_ordering)]
const _: () = {
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de, T> _serde::Deserialize<'de> for Between<T>
    where
        T: std::fmt::Debug
            + _serde::Deserialize<'de>
            + PartialOrd
            + sqlx::Type<sqlx::Postgres>
            + for<'__> sqlx::Encode<'__, sqlx::Postgres>,
    {
        fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[derive(optimal_memory_layout::OptimalMemoryLayout)]
            #[expect(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                f0,
                f1,
                __ignore,
            }
            #[derive(optimal_memory_layout::OptimalMemoryLayout)]
            #[doc(hidden)]
            struct __FieldVisitor;
            impl _serde::de::Visitor<'_> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __f: &mut _serde::__private229::Formatter<'_>,
                ) -> _serde::__private229::fmt::Result {
                    _serde::__private229::Formatter::write_str(
                        __f,
                        constants_str::PG_CRUD_FIELD_IDENTIFIER,
                    )
                }
                fn visit_u64<__E>(self, v: u64) -> Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match v {
                        1u64 => Ok(__Field::f0),
                        2u64 => Ok(__Field::f1),
                        _ => Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(self, v: &str) -> Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match v {
                        constants_str::PG_CRUD_START_FIELD => Ok(__Field::f0),
                        constants_str::PG_CRUD_END_FIELD => Ok(__Field::f1),
                        _ => Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(self, v: &[u8]) -> Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match v {
                        b"start" => Ok(__Field::f0),
                        b"end" => Ok(__Field::f1),
                        _ => Ok(__Field::__ignore),
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                }
            }
            #[derive(optimal_memory_layout::OptimalMemoryLayout)]
            #[doc(hidden)]
            struct __Visitor<'de, T>
            where
                T: _serde::Deserialize<'de>
                    + sqlx::Type<sqlx::Postgres>
                    + for<'__> sqlx::Encode<'__, sqlx::Postgres>,
            {
                marker: _serde::__private229::PhantomData<Between<T>>,
                lt: _serde::__private229::PhantomData<&'de ()>,
            }
            impl<'de, T> _serde::de::Visitor<'de> for __Visitor<'de, T>
            where
                T: std::fmt::Debug
                    + _serde::Deserialize<'de>
                    + PartialOrd
                    + sqlx::Type<sqlx::Postgres>
                    + for<'__> sqlx::Encode<'__, sqlx::Postgres>,
            {
                type Value = Between<T>;
                fn expecting(
                    &self,
                    __f: &mut _serde::__private229::Formatter<'_>,
                ) -> _serde::__private229::fmt::Result {
                    _serde::__private229::Formatter::write_str(
                        __f,
                        constants_str::PG_CRUD_BETWEEN_STRUCT_NAME,
                    )
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let Some(f0) = _serde::de::SeqAccess::next_element::<T>(&mut __seq)? else {
                        return Err(_serde::de::Error::invalid_length(
                            constants_usize::ONE,
                            &constants_str::PG_CRUD_BETWEEN_EXPECTING,
                        ));
                    };
                    let Some(f1) = _serde::de::SeqAccess::next_element::<T>(&mut __seq)? else {
                        return Err(_serde::de::Error::invalid_length(
                            2usize,
                            &constants_str::PG_CRUD_BETWEEN_EXPECTING,
                        ));
                    };
                    match Between::try_new(f0, f1) {
                        Ok(v) => Ok(v),
                        Err(error) => Err(serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
                #[inline]
                fn visit_map<__A>(self, mut __map: __A) -> Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut f0: Option<T> = None;
                    let mut f1: Option<T> = None;
                    while let Some(__k) = _serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                        match __k {
                            __Field::f0 => {
                                if Option::is_some(&f0) {
                                    return Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            constants_str::PG_CRUD_START_FIELD,
                                        ),
                                    );
                                }
                                f0 = Some(_serde::de::MapAccess::next_value::<T>(&mut __map)?);
                            }
                            __Field::f1 => {
                                if Option::is_some(&f1) {
                                    return Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            constants_str::PG_CRUD_END_FIELD,
                                        ),
                                    );
                                }
                                f1 = Some(_serde::de::MapAccess::next_value::<T>(&mut __map)?);
                            }
                            __Field::__ignore => {
                                let _: serde::de::IgnoredAny =
                                    _serde::de::MapAccess::next_value::<_serde::de::IgnoredAny>(
                                        &mut __map,
                                    )?;
                            }
                        }
                    }
                    let f0_v = match f0 {
                        Some(v) => v,
                        None => _serde::__private229::de::missing_field(
                            constants_str::PG_CRUD_START_FIELD,
                        )?,
                    };
                    let f1_v = match f1 {
                        Some(v) => v,
                        None => _serde::__private229::de::missing_field(
                            constants_str::PG_CRUD_END_FIELD,
                        )?,
                    };
                    match Between::try_new(f0_v, f1_v) {
                        Ok(v) => Ok(v),
                        Err(error) => Err(serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
            }
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                constants_str::PG_CRUD_BETWEEN_SCHEMA_NAME,
                constants_str::PG_CRUD_SERDE_BETWEEN_FIELDS,
                __Visitor {
                    marker: _serde::__private229::PhantomData::<Self>,
                    lt: _serde::__private229::PhantomData,
                },
            )
        }
    }
};
impl<
    T: pg_crud_common::domain_types::DefaultSomeOneElement
        + sqlx::Type<sqlx::Postgres>
        + for<'__> sqlx::Encode<'__, sqlx::Postgres>,
> pg_crud_common::domain_types::DefaultSomeOneElement for Between<T>
{
    fn default_some_one_element() -> Self {
        Self {
            start: pg_crud_common::domain_types::DefaultSomeOneElement::default_some_one_element(),
            end: pg_crud_common::domain_types::DefaultSomeOneElement::default_some_one_element(),
        }
    }
}
impl<'lt, T: Send + sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres> + 'lt>
    pg_crud_common::domain_types::PgTypeWhereFilter<'lt> for Between<T>
{
    fn query_bind(
        self,
        mut query: pg_crud_common::domain_types::SqlxPostgresQuery<'lt>,
    ) -> Result<
        pg_crud_common::domain_types::SqlxPostgresQuery<'lt>,
        pg_crud_common::domain_types::SqlxPostgresQueryBindError,
    > {
        if let Err(error) = query.as_mut().try_bind(self.start) {
            return Err(pg_crud_common::domain_types::SqlxPostgresQueryBindError::from(error));
        }
        if let Err(error) = query.as_mut().try_bind(self.end) {
            return Err(pg_crud_common::domain_types::SqlxPostgresQueryBindError::from(error));
        }
        Ok(query)
    }
    fn query_part(
        &self,
        increment: &mut dyn pg_crud_common::domain_types::QueryPartIncrementMut,
        _: pg_crud_common::domain_types::SqlColumnRef<'_>,
        _: pg_crud_common::domain_types::AddOperator,
    ) -> Result<
        pg_crud_common::domain_types::QueryPartFragment,
        pg_crud_common::domain_types::QueryPartError,
    > {
        let start_increment =
            match pg_crud_common::domain_types::increment_checked_add_one_returning_increment(
                increment,
            ) {
                Ok(v) => v,
                Err(error) => {
                    return Err(error);
                }
            };
        let end_increment =
            match pg_crud_common::domain_types::increment_checked_add_one_returning_increment(
                increment,
            ) {
                Ok(v) => v,
                Err(error) => {
                    return Err(error);
                }
            };
        let mut query_part = String::with_capacity(32);
        if std::fmt::Write::write_fmt(
            &mut query_part,
            format_args!("between ${start_increment} and ${end_increment}"),
        )
        .is_err()
        {
            return Err(
                pg_crud_common::domain_types::QueryPartError::WriteIntoBuffer {
                    location: location_macros::location!(),
                },
            );
        }
        Ok(pg_crud_common::domain_types::QueryPartFragment::try_from(
            query_part,
        )?)
    }
}
