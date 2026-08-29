#![allow(clippy::wildcard_imports)] // split test fixtures share the private facade vocabulary

pub mod create_location_test_text;
pub mod display_struct;
pub mod error_one;
pub mod error_two;
pub mod error_unnamed_one;
pub mod loc_test_text_max_len;
pub mod location_test_count;
pub mod location_test_flag;
pub mod location_test_text;
pub mod serde_struct;

fn main() {
    let error = error_one::ErrorOne::Variant {
        eo_display_field: display_struct::DisplayStruct {
            display: create_location_test_text::create_location_test_text(String::from(
                constants_str::catalog::PG_CRUD_V_FIELD,
            )),
            something: location_test_flag::LocationTestFlag::from(true),
        },
        eo_serde: serde_struct::SerdeStruct {
            one: create_location_test_text::create_location_test_text(String::from(
                constants_str::catalog::PG_CRUD_V_FIELD,
            )),
            two: location_test_flag::LocationTestFlag::from(true),
            three: location_test_count::LocationTestCount::from(42),
        },
        eo_location_field: error_two::ErrorTwo::Variant {
            eo_display_with_serde_field: create_location_test_text::create_location_test_text(
                String::from(constants_str::catalog::PG_CRUD_V_FIELD),
            ),
            location: location_macros::location!(),
        },
        eo_vec_display_field: vec![
            display_struct::DisplayStruct {
                display: location_test_text::LocationTestText(String::from(
                    constants_str::integration_fixtures::VALUE_08708789,
                )),
                something: location_test_flag::LocationTestFlag::from(true),
            },
            display_struct::DisplayStruct {
                display: location_test_text::LocationTestText(String::from(
                    constants_str::integration_fixtures::VALUE_7565757,
                )),
                something: location_test_flag::LocationTestFlag::from(true),
            },
        ],
        eo_vec_serde: vec![
            serde_struct::SerdeStruct {
                one: location_test_text::LocationTestText(String::from(
                    constants_str::catalog::PG_CRUD_V_FIELD,
                )),
                two: location_test_flag::LocationTestFlag::from(true),
                three: location_test_count::LocationTestCount::from(42),
            },
            serde_struct::SerdeStruct {
                one: location_test_text::LocationTestText(String::from(
                    constants_str::integration_fixtures::VALUE_97697697,
                )),
                two: location_test_flag::LocationTestFlag::from(false),
                three: location_test_count::LocationTestCount::from(422),
            },
        ],
        eo_vec_location_field: vec![
            error_unnamed_one::ErrorUnnamedOne::Something(error_two::ErrorTwo::Variant {
                eo_display_with_serde_field: location_test_text::LocationTestText(String::from(
                    constants_str::catalog::PG_CRUD_V_FIELD,
                )),
                location: location_macros::location!(),
            }),
            error_unnamed_one::ErrorUnnamedOne::Something(error_two::ErrorTwo::Variant {
                eo_display_with_serde_field: location_test_text::LocationTestText(String::from(
                    constants_str::integration_fixtures::VALUE_123,
                )),
                location: location_macros::location!(),
            }),
        ],
        hashmap_string_string: std::collections::HashMap::from([
            (
                create_location_test_text::create_location_test_text(String::from(
                    constants_str::catalog::KESDFSFDSFSD,
                )),
                display_struct::DisplayStruct {
                    display: create_location_test_text::create_location_test_text(String::from(
                        constants_str::catalog::VASFDSDFSDFLUE,
                    )),
                    something: location_test_flag::LocationTestFlag::from(true),
                },
            ),
            (
                create_location_test_text::create_location_test_text(String::from(
                    constants_str::catalog::KSDFSDFSDFSDFEY,
                )),
                display_struct::DisplayStruct {
                    display: create_location_test_text::create_location_test_text(String::from(
                        constants_str::catalog::VALSFDSFDSFDSUE,
                    )),
                    something: location_test_flag::LocationTestFlag::from(true),
                },
            ),
        ]),
        hashmap_string_serde: std::collections::HashMap::from([
            (
                create_location_test_text::create_location_test_text(String::from(
                    constants_str::catalog::KDFGSDFGDSFGEY,
                )),
                serde_struct::SerdeStruct {
                    one: create_location_test_text::create_location_test_text(String::from(
                        constants_str::catalog::VALUSDFGDSGDSFGDE,
                    )),
                    two: location_test_flag::LocationTestFlag::from(true),
                    three: location_test_count::LocationTestCount::from(42),
                },
            ),
            (
                create_location_test_text::create_location_test_text(String::from(
                    constants_str::catalog::KSDFGDSFGSDFGEY,
                )),
                serde_struct::SerdeStruct {
                    one: create_location_test_text::create_location_test_text(String::from(
                        constants_str::catalog::VALSDFGDSGDUE,
                    )),
                    two: location_test_flag::LocationTestFlag::from(true),
                    three: location_test_count::LocationTestCount::from(42),
                },
            ),
        ]),
        hashmap_string_location: std::collections::HashMap::from([
            (
                create_location_test_text::create_location_test_text(String::from(
                    constants_str::catalog::KSDFGADSFGSDFGDFGEY,
                )),
                error_unnamed_one::ErrorUnnamedOne::Something(error_two::ErrorTwo::Variant {
                    eo_display_with_serde_field:
                        create_location_test_text::create_location_test_text(String::from(
                            constants_str::catalog::VASDFGDGDFGLUE,
                        )),
                    location: location_macros::location!(),
                }),
            ),
            (
                create_location_test_text::create_location_test_text(String::from(
                    constants_str::catalog::KESDFGSDGFDFGY,
                )),
                error_unnamed_one::ErrorUnnamedOne::Something(error_two::ErrorTwo::Variant {
                    eo_display_with_serde_field:
                        create_location_test_text::create_location_test_text(String::from(
                            constants_str::catalog::VALSDFGDSAFGDSGUE,
                        )),
                    location: location_macros::location!(),
                }),
            ),
        ]),
        location: location_macros::location!(),
    };
    println!("{error:?}");
}
