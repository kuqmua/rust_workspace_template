// todo is there is no point to add extra info to enum like this
// eo_location_field: {
//     eo_display_with_serde_field: v
// }
// https://github.com/kuqmua/rust_workspace_template/blob/ebb9f680ea508fb5df5ee5d2791e96ca34610bc2/location_test/src/main.rs#L85 2024-05-06 09:17:23
// impl display like this this
// eo_location_field
// https://github.com/kuqmua/rust_workspace_template/blob/ebb9f680ea508fb5df5ee5d2791e96ca34610bc2/location_test/src/main.rs#L85 2024-05-06 09:17:23
const LOC_TEST_TEXT_MAX_LEN: usize = 1_048_576;
#[derive(Debug, thiserror::Error, location::Location, optml::Optml)]
pub enum ErrorOne {
    //use to_err_string::ToErrString for hashmap ks instead of Display
    //todo even for String in serialize deserialize version of error must be using to_err_string::ToErrString impl instead of std::fmt::Display
    //todo test on using only location as pnly field in named variant
    Variant {
        #[eo_to_err_string]
        eo_display_field: DisplayStruct, //IN SERIALIZE DESERIALIZE String
        #[eo_to_err_string_serde]
        eo_serde: SerdeStruct,
        #[eo_location]
        eo_location_field: ErrorTwo, //IN SERIALIZE DESERIALIZE nested
        #[eo_vec_to_err_string] //todo remove w under Vec
        eo_vec_display_field: Vec<DisplayStruct>, //IN SERIALIZE DESERIALIZE Vec<String>
        #[eo_vec_to_err_string_serde]
        eo_vec_serde: Vec<SerdeStruct>,
        #[eo_vec_location]
        eo_vec_location_field: Vec<ErrorUnnamedOne>, //IN SERIALIZE DESERIALIZE Vec<nested>
        #[eo_hashmap_k_string_v_to_err_string]
        hashmap_string_string: std::collections::HashMap<LocationTestText, DisplayStruct>,
        #[eo_hashmap_k_string_v_to_err_string_serde]
        hashmap_string_serde: std::collections::HashMap<LocationTestText, SerdeStruct>,
        #[eo_hashmap_k_string_v_location]
        hashmap_string_location: std::collections::HashMap<LocationTestText, ErrorUnnamedOne>,
        location: location_lib::location::Location,
    },
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    serde::Serialize,
    optml::Optml,
    newtype::BoundedString,
    newtype::ToErrStringAsRefStr,
)]
#[bounded_string(max = LOC_TEST_TEXT_MAX_LEN )]
pub struct LocationTestText(String);
impl<'de> serde::Deserialize<'de> for LocationTestText {
    fn deserialize<Deserializer>(deserializer: Deserializer) -> Result<Self, Deserializer::Error>
    where
        Deserializer: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    optml::Optml,
    newtype::FromInner,
    newtype::ToErrString,
)]
pub struct LocationTestFlag(bool);
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    optml::Optml,
    newtype::FromInner,
    newtype::ToErrString,
)]
pub struct LocationTestCount(u32);
#[derive(Debug, thiserror::Error, location::Location, optml::Optml)]
pub enum ErrorTwo {
    Another {
        #[eo_to_err_string_serde]
        sdasdasd: LocationTestText,
        location: location_lib::location::Location,
    },
    Variant {
        #[eo_to_err_string_serde]
        eo_display_with_serde_field: LocationTestText,
        location: location_lib::location::Location,
    },
}
#[derive(Debug, thiserror::Error, location::Location, optml::Optml)]
pub enum ErrorUnnamedOne {
    Something(ErrorTwo),
}
#[derive(Debug, optml::Optml)]
pub struct DisplayStruct {
    pub display: LocationTestText,
    pub something: LocationTestFlag,
}
//todo or maybe two different traits - display foreign type and convert into serializable and deserializable type
impl to_err_string::ToErrString for DisplayStruct {
    fn to_err_string(&self) -> to_err_string::ToErrStringValue {
        to_err_string::ToErrStringValue::try_from(format!("{self:?}"))
            .unwrap_or_else(to_err_string::ToErrStringValue::from)
    }
}
//todo rename fields
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, serde::Serialize, serde::Deserialize, optml::Optml)]
pub struct SerdeStruct {
    pub one: LocationTestText,
    pub three: LocationTestCount,
    pub two: LocationTestFlag,
}
impl to_err_string::ToErrString for SerdeStruct {
    fn to_err_string(&self) -> to_err_string::ToErrStringValue {
        to_err_string::ToErrStringValue::try_from(format!("{self:?}"))
            .unwrap_or_else(to_err_string::ToErrStringValue::from)
    }
}
fn main() {
    let error = ErrorOne::Variant {
        eo_display_field: DisplayStruct {
            display: LocationTestText(String::from(str_constants::PG_CRUD_V_FIELD)),
            something: LocationTestFlag::from(true),
        },
        eo_serde: SerdeStruct {
            one: LocationTestText(String::from(str_constants::PG_CRUD_V_FIELD)),
            two: LocationTestFlag::from(true),
            three: LocationTestCount::from(42),
        },
        eo_location_field: ErrorTwo::Variant {
            eo_display_with_serde_field: LocationTestText(String::from(
                str_constants::PG_CRUD_V_FIELD,
            )),
            location: location_macros::location!(),
        },
        eo_vec_display_field: vec![
            DisplayStruct {
                display: LocationTestText(String::from(str_constants::VALUE_08708789)),
                something: LocationTestFlag::from(true),
            },
            DisplayStruct {
                display: LocationTestText(String::from(str_constants::VALUE_7565757)),
                something: LocationTestFlag::from(true),
            },
        ],
        eo_vec_serde: vec![
            SerdeStruct {
                one: LocationTestText(String::from(str_constants::PG_CRUD_V_FIELD)),
                two: LocationTestFlag::from(true),
                three: LocationTestCount::from(42),
            },
            SerdeStruct {
                one: LocationTestText(String::from(str_constants::VALUE_97697697)),
                two: LocationTestFlag::from(false),
                three: LocationTestCount::from(422),
            },
        ],
        eo_vec_location_field: vec![
            ErrorUnnamedOne::Something(ErrorTwo::Variant {
                eo_display_with_serde_field: LocationTestText(String::from(
                    str_constants::PG_CRUD_V_FIELD,
                )),
                location: location_macros::location!(),
            }),
            ErrorUnnamedOne::Something(ErrorTwo::Variant {
                eo_display_with_serde_field: LocationTestText(String::from(
                    str_constants::VALUE_123,
                )),
                location: location_macros::location!(),
            }),
        ],
        hashmap_string_string: std::collections::HashMap::from([
            (
                LocationTestText(String::from(str_constants::KESDFSFDSFSD)),
                DisplayStruct {
                    display: LocationTestText(String::from(str_constants::VASFDSDFSDFLUE)),
                    something: LocationTestFlag::from(true),
                },
            ),
            (
                LocationTestText(String::from(str_constants::KSDFSDFSDFSDFEY)),
                DisplayStruct {
                    display: LocationTestText(String::from(str_constants::VALSFDSFDSFDSUE)),
                    something: LocationTestFlag::from(true),
                },
            ),
        ]),
        hashmap_string_serde: std::collections::HashMap::from([
            (
                LocationTestText(String::from(str_constants::KDFGSDFGDSFGEY)),
                SerdeStruct {
                    one: LocationTestText(String::from(str_constants::VALUSDFGDSGDSFGDE)),
                    two: LocationTestFlag::from(true),
                    three: LocationTestCount::from(42),
                },
            ),
            (
                LocationTestText(String::from(str_constants::KSDFGDSFGSDFGEY)),
                SerdeStruct {
                    one: LocationTestText(String::from(str_constants::VALSDFGDSGDUE)),
                    two: LocationTestFlag::from(true),
                    three: LocationTestCount::from(42),
                },
            ),
        ]),
        hashmap_string_location: std::collections::HashMap::from([
            (
                LocationTestText(String::from(str_constants::KSDFGADSFGSDFGDFGEY)),
                ErrorUnnamedOne::Something(ErrorTwo::Variant {
                    eo_display_with_serde_field: LocationTestText(String::from(
                        str_constants::VASDFGDGDFGLUE,
                    )),
                    location: location_macros::location!(),
                }),
            ),
            (
                LocationTestText(String::from(str_constants::KESDFGSDGFDFGY)),
                ErrorUnnamedOne::Something(ErrorTwo::Variant {
                    eo_display_with_serde_field: LocationTestText(String::from(
                        str_constants::VALSDFGDSAFGDSGUE,
                    )),
                    location: location_macros::location!(),
                }),
            ),
        ]),
        location: location_macros::location!(),
    };
    println!("{error:?}");
}
