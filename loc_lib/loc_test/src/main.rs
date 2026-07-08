// todo is there is no point to add extra info to enum like this
// eo_loc_field: {
//     eo_display_with_serde_field: v
// }
// https://github.com/kuqmua/rust_workspace_template/blob/ebb9f680ea508fb5df5ee5d2791e96ca34610bc2/loc_test/src/main.rs#L85 2024-05-06 09:17:23
// impl display like this this
// eo_loc_field
// https://github.com/kuqmua/rust_workspace_template/blob/ebb9f680ea508fb5df5ee5d2791e96ca34610bc2/loc_test/src/main.rs#L85 2024-05-06 09:17:23
#[derive(Debug, thiserror::Error, loc_lib::Location, optml::Optml)]
pub enum ErOne {
    //use loc_lib::ToErrString for hashmap ks instead of Display
    //todo even for String in serialize deserialize version of er must be using loc_lib::ToErrString impl instead of std::fmt::Display
    //todo test on using only loc as pnly field in named vrt
    Vrt {
        #[eo_to_err_string]
        eo_display_field: DisplayStruct, //IN SERIALIZE DESERIALIZE String
        #[eo_to_err_string_serde]
        eo_serde: SerdeStruct,
        #[eo_loc]
        eo_loc_field: ErTwo, //IN SERIALIZE DESERIALIZE nested
        #[eo_vec_to_err_string] //todo remove w under Vec
        eo_vec_display_field: Vec<DisplayStruct>, //IN SERIALIZE DESERIALIZE Vec<String>
        #[eo_vec_to_err_string_serde]
        eo_vec_serde: Vec<SerdeStruct>,
        #[eo_vec_loc]
        eo_vec_loc_field: Vec<ErUnnamedOne>, //IN SERIALIZE DESERIALIZE Vec<nested>
        #[eo_hashmap_k_string_v_to_err_string]
        hashmap_string_string: std::collections::HashMap<LocTestText, DisplayStruct>,
        #[eo_hashmap_k_string_v_to_err_string_serde]
        hashmap_string_serde: std::collections::HashMap<LocTestText, SerdeStruct>,
        #[eo_hashmap_k_string_v_loc]
        hashmap_string_loc: std::collections::HashMap<LocTestText, ErUnnamedOne>,
        loc: loc_lib::loc::Loc,
    },
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    optml::Optml,
    newtype::Newtype,
)]
#[newtype(to_err_string_as_ref_str)]
pub struct LocTestText(pub String);
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    optml::Optml,
    newtype::Newtype,
)]
#[newtype(from, to_err_string)]
pub struct LocTestFlag(pub bool);
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    optml::Optml,
    newtype::Newtype,
)]
#[newtype(from, to_err_string)]
pub struct LocTestCount(pub u32);
#[derive(Debug, thiserror::Error, loc_lib::Location, optml::Optml)]
pub enum ErTwo {
    Another {
        #[eo_to_err_string_serde]
        sdasdasd: LocTestText,
        loc: loc_lib::loc::Loc,
    },
    Vrt {
        #[eo_to_err_string_serde]
        eo_display_with_serde_field: LocTestText,
        loc: loc_lib::loc::Loc,
    },
}
#[derive(Debug, thiserror::Error, loc_lib::Location, optml::Optml)]
pub enum ErUnnamedOne {
    Something(ErTwo),
}
#[derive(Debug, optml::Optml)]
pub struct DisplayStruct {
    pub display: LocTestText,
    pub something: LocTestFlag,
}
//todo or mb two different traits - display foreign type and convert into serializable and deserializable type
impl loc_lib::ToErrString for DisplayStruct {
    fn to_err_string(&self) -> loc_lib::ToErrStringValue {
        loc_lib::ToErrStringValue(format!("{self:?}"))
    }
}
//todo rename fields
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, serde::Serialize, serde::Deserialize, optml::Optml)]
pub struct SerdeStruct {
    pub one: LocTestText,
    pub three: LocTestCount,
    pub two: LocTestFlag,
}
impl loc_lib::ToErrString for SerdeStruct {
    fn to_err_string(&self) -> loc_lib::ToErrStringValue {
        loc_lib::ToErrStringValue(format!("{self:?}"))
    }
}
fn main() {
    let er = ErOne::Vrt {
        eo_display_field: DisplayStruct {
            display: LocTestText(String::from("v")),
            something: LocTestFlag(true),
        },
        eo_serde: SerdeStruct {
            one: LocTestText(String::from("v")),
            two: LocTestFlag(true),
            three: LocTestCount(42),
        },
        eo_loc_field: ErTwo::Vrt {
            eo_display_with_serde_field: LocTestText(String::from("v")),
            loc: loc_lib::loc!(),
        },
        eo_vec_display_field: vec![
            DisplayStruct {
                display: LocTestText(String::from("08708789")),
                something: LocTestFlag(true),
            },
            DisplayStruct {
                display: LocTestText(String::from("7565757")),
                something: LocTestFlag(true),
            },
        ],
        eo_vec_serde: vec![
            SerdeStruct {
                one: LocTestText(String::from("v")),
                two: LocTestFlag(true),
                three: LocTestCount(42),
            },
            SerdeStruct {
                one: LocTestText(String::from("97697697")),
                two: LocTestFlag(false),
                three: LocTestCount(422),
            },
        ],
        eo_vec_loc_field: vec![
            ErUnnamedOne::Something(ErTwo::Vrt {
                eo_display_with_serde_field: LocTestText(String::from("v")),
                loc: loc_lib::loc!(),
            }),
            ErUnnamedOne::Something(ErTwo::Vrt {
                eo_display_with_serde_field: LocTestText(String::from("123")),
                loc: loc_lib::loc!(),
            }),
        ],
        hashmap_string_string: std::collections::HashMap::from([
            (
                LocTestText(String::from("kesdfsfdsfsd")),
                DisplayStruct {
                    display: LocTestText(String::from("vasfdsdfsdflue")),
                    something: LocTestFlag(true),
                },
            ),
            (
                LocTestText(String::from("ksdfsdfsdfsdfey")),
                DisplayStruct {
                    display: LocTestText(String::from("valsfdsfdsfdsue")),
                    something: LocTestFlag(true),
                },
            ),
        ]),
        hashmap_string_serde: std::collections::HashMap::from([
            (
                LocTestText(String::from("kdfgsdfgdsfgey")),
                SerdeStruct {
                    one: LocTestText(String::from("valusdfgdsgdsfgde")),
                    two: LocTestFlag(true),
                    three: LocTestCount(42),
                },
            ),
            (
                LocTestText(String::from("ksdfgdsfgsdfgey")),
                SerdeStruct {
                    one: LocTestText(String::from("valsdfgdsgdue")),
                    two: LocTestFlag(true),
                    three: LocTestCount(42),
                },
            ),
        ]),
        hashmap_string_loc: std::collections::HashMap::from([
            (
                LocTestText(String::from("ksdfgadsfgsdfgdfgey")),
                ErUnnamedOne::Something(ErTwo::Vrt {
                    eo_display_with_serde_field: LocTestText(String::from("vasdfgdgdfglue")),
                    loc: loc_lib::loc!(),
                }),
            ),
            (
                LocTestText(String::from("kesdfgsdgfdfgy")),
                ErUnnamedOne::Something(ErTwo::Vrt {
                    eo_display_with_serde_field: LocTestText(String::from("valsdfgdsafgdsgue")),
                    loc: loc_lib::loc!(),
                }),
            ),
        ]),
        loc: loc_lib::loc!(),
    };
    println!("{er:?}");
}
