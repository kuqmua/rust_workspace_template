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
        eo_display_field: display_struct::DisplayStruct::new(
            create_location_test_text::create_location_test_text(String::from(
                constants_str::PG_CRUD_VALUES_FIELD,
            )),
            location_test_flag::LocationTestFlag::from(true),
        ),
        eo_serde: serde_struct::SerdeStruct::new(
            create_location_test_text::create_location_test_text(String::from(
                constants_str::PG_CRUD_VALUES_FIELD,
            )),
            location_test_count::LocationTestCount::from(42),
            location_test_flag::LocationTestFlag::from(true),
        ),
        eo_location_field: error_two::ErrorTwo::Variant {
            eo_display_with_serde_field: create_location_test_text::create_location_test_text(
                String::from(constants_str::PG_CRUD_VALUES_FIELD),
            ),
            location: proc_macro_location_bang::location!(),
        },
        eo_vec_display_field: vec![
            display_struct::DisplayStruct::new(
                location_test_text::LocationTestText::from(constants_str::VALUE_08708789),
                location_test_flag::LocationTestFlag::from(true),
            ),
            display_struct::DisplayStruct::new(
                location_test_text::LocationTestText::from(constants_str::VALUE_7565757),
                location_test_flag::LocationTestFlag::from(true),
            ),
        ],
        eo_vec_serde: vec![
            serde_struct::SerdeStruct::new(
                location_test_text::LocationTestText::from(constants_str::PG_CRUD_VALUES_FIELD),
                location_test_count::LocationTestCount::from(42),
                location_test_flag::LocationTestFlag::from(true),
            ),
            serde_struct::SerdeStruct::new(
                location_test_text::LocationTestText::from(constants_str::VALUE_97697697),
                location_test_count::LocationTestCount::from(422),
                location_test_flag::LocationTestFlag::from(false),
            ),
        ],
        eo_vec_location_field: vec![
            error_unnamed_one::ErrorUnnamedOne::Something(error_two::ErrorTwo::Variant {
                eo_display_with_serde_field: location_test_text::LocationTestText::from(
                    constants_str::PG_CRUD_VALUES_FIELD,
                ),
                location: proc_macro_location_bang::location!(),
            }),
            error_unnamed_one::ErrorUnnamedOne::Something(error_two::ErrorTwo::Variant {
                eo_display_with_serde_field: location_test_text::LocationTestText::from(
                    constants_str::VALUE_123,
                ),
                location: proc_macro_location_bang::location!(),
            }),
        ],
        hashmap_string_string: std::collections::HashMap::from([
            (
                create_location_test_text::create_location_test_text(String::from(
                    constants_str::KESDFSFDSFSD,
                )),
                display_struct::DisplayStruct::new(
                    create_location_test_text::create_location_test_text(String::from(
                        constants_str::VASFDSDFSDFLUE,
                    )),
                    location_test_flag::LocationTestFlag::from(true),
                ),
            ),
            (
                create_location_test_text::create_location_test_text(String::from(
                    constants_str::KSDFSDFSDFSDFEY,
                )),
                display_struct::DisplayStruct::new(
                    create_location_test_text::create_location_test_text(String::from(
                        constants_str::VALSFDSFDSFDSUE,
                    )),
                    location_test_flag::LocationTestFlag::from(true),
                ),
            ),
        ]),
        hashmap_string_serde: std::collections::HashMap::from([
            (
                create_location_test_text::create_location_test_text(String::from(
                    constants_str::KDFGSDFGDSFGEY,
                )),
                serde_struct::SerdeStruct::new(
                    create_location_test_text::create_location_test_text(String::from(
                        constants_str::VALUSDFGDSGDSFGDE,
                    )),
                    location_test_count::LocationTestCount::from(42),
                    location_test_flag::LocationTestFlag::from(true),
                ),
            ),
            (
                create_location_test_text::create_location_test_text(String::from(
                    constants_str::KSDFGDSFGSDFGEY,
                )),
                serde_struct::SerdeStruct::new(
                    create_location_test_text::create_location_test_text(String::from(
                        constants_str::VALSDFGDSGDUE,
                    )),
                    location_test_count::LocationTestCount::from(42),
                    location_test_flag::LocationTestFlag::from(true),
                ),
            ),
        ]),
        hashmap_string_location: std::collections::HashMap::from([
            (
                create_location_test_text::create_location_test_text(String::from(
                    constants_str::KSDFGADSFGSDFGDFGEY,
                )),
                error_unnamed_one::ErrorUnnamedOne::Something(error_two::ErrorTwo::Variant {
                    eo_display_with_serde_field:
                        create_location_test_text::create_location_test_text(String::from(
                            constants_str::VASDFGDGDFGLUE,
                        )),
                    location: proc_macro_location_bang::location!(),
                }),
            ),
            (
                create_location_test_text::create_location_test_text(String::from(
                    constants_str::KESDFGSDGFDFGY,
                )),
                error_unnamed_one::ErrorUnnamedOne::Something(error_two::ErrorTwo::Variant {
                    eo_display_with_serde_field:
                        create_location_test_text::create_location_test_text(String::from(
                            constants_str::VALSDFGDSAFGDSGUE,
                        )),
                    location: proc_macro_location_bang::location!(),
                }),
            ),
        ]),
        location: proc_macro_location_bang::location!(),
    };
    println!("{error:?}");
}

const _: fn(&str) -> Result<(), bounded_types::bounded_string_error::BoundedStringError> =
    bounded_types::bounded_string::BoundedString::<0, 0>::validate_str;
