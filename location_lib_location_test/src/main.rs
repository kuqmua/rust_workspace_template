#![allow(clippy::wildcard_imports)] // split test fixtures share the private facade vocabulary

mod create_location_test_text;
mod display_struct;
mod error_one;
mod error_two;
mod error_unnamed_one;
mod loc_test_text_max_len;
mod location_test_count;
mod location_test_flag;
mod location_test_text;
mod serde_struct;
pub(crate) use create_location_test_text::create_location_test_text;
pub use display_struct::DisplayStruct;
pub use error_one::*;
pub use error_two::*;
pub use error_unnamed_one::*;
pub(crate) use loc_test_text_max_len::LOC_TEST_TEXT_MAX_LEN;
pub use location_test_count::LocationTestCount;
pub use location_test_flag::LocationTestFlag;
pub use location_test_text::*;
pub use serde_struct::SerdeStruct;

fn main() {
    let error = ErrorOne::Variant {
        eo_display_field: DisplayStruct {
            display: create_location_test_text(String::from(constants_str::PG_CRUD_V_FIELD)),
            something: LocationTestFlag::from(true),
        },
        eo_serde: SerdeStruct {
            one: create_location_test_text(String::from(constants_str::PG_CRUD_V_FIELD)),
            two: LocationTestFlag::from(true),
            three: LocationTestCount::from(42),
        },
        eo_location_field: ErrorTwo::Variant {
            eo_display_with_serde_field: create_location_test_text(String::from(
                constants_str::PG_CRUD_V_FIELD,
            )),
            location: location_macros::location!(),
        },
        eo_vec_display_field: vec![
            DisplayStruct {
                display: LocationTestText(String::from(constants_str::VALUE_08708789)),
                something: LocationTestFlag::from(true),
            },
            DisplayStruct {
                display: LocationTestText(String::from(constants_str::VALUE_7565757)),
                something: LocationTestFlag::from(true),
            },
        ],
        eo_vec_serde: vec![
            SerdeStruct {
                one: LocationTestText(String::from(constants_str::PG_CRUD_V_FIELD)),
                two: LocationTestFlag::from(true),
                three: LocationTestCount::from(42),
            },
            SerdeStruct {
                one: LocationTestText(String::from(constants_str::VALUE_97697697)),
                two: LocationTestFlag::from(false),
                three: LocationTestCount::from(422),
            },
        ],
        eo_vec_location_field: vec![
            ErrorUnnamedOne::Something(ErrorTwo::Variant {
                eo_display_with_serde_field: LocationTestText(String::from(
                    constants_str::PG_CRUD_V_FIELD,
                )),
                location: location_macros::location!(),
            }),
            ErrorUnnamedOne::Something(ErrorTwo::Variant {
                eo_display_with_serde_field: LocationTestText(String::from(
                    constants_str::VALUE_123,
                )),
                location: location_macros::location!(),
            }),
        ],
        hashmap_string_string: std::collections::HashMap::from([
            (
                create_location_test_text(String::from(constants_str::KESDFSFDSFSD)),
                DisplayStruct {
                    display: create_location_test_text(String::from(constants_str::VASFDSDFSDFLUE)),
                    something: LocationTestFlag::from(true),
                },
            ),
            (
                create_location_test_text(String::from(constants_str::KSDFSDFSDFSDFEY)),
                DisplayStruct {
                    display: create_location_test_text(String::from(
                        constants_str::VALSFDSFDSFDSUE,
                    )),
                    something: LocationTestFlag::from(true),
                },
            ),
        ]),
        hashmap_string_serde: std::collections::HashMap::from([
            (
                create_location_test_text(String::from(constants_str::KDFGSDFGDSFGEY)),
                SerdeStruct {
                    one: create_location_test_text(String::from(constants_str::VALUSDFGDSGDSFGDE)),
                    two: LocationTestFlag::from(true),
                    three: LocationTestCount::from(42),
                },
            ),
            (
                create_location_test_text(String::from(constants_str::KSDFGDSFGSDFGEY)),
                SerdeStruct {
                    one: create_location_test_text(String::from(constants_str::VALSDFGDSGDUE)),
                    two: LocationTestFlag::from(true),
                    three: LocationTestCount::from(42),
                },
            ),
        ]),
        hashmap_string_location: std::collections::HashMap::from([
            (
                create_location_test_text(String::from(constants_str::KSDFGADSFGSDFGDFGEY)),
                ErrorUnnamedOne::Something(ErrorTwo::Variant {
                    eo_display_with_serde_field: create_location_test_text(String::from(
                        constants_str::VASDFGDGDFGLUE,
                    )),
                    location: location_macros::location!(),
                }),
            ),
            (
                create_location_test_text(String::from(constants_str::KESDFGSDGFDFGY)),
                ErrorUnnamedOne::Something(ErrorTwo::Variant {
                    eo_display_with_serde_field: create_location_test_text(String::from(
                        constants_str::VALSDFGDSAFGDSGUE,
                    )),
                    location: location_macros::location!(),
                }),
            ),
        ]),
        location: location_macros::location!(),
    };
    println!("{error:?}");
}
