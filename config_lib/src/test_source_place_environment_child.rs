#[test]
#[ignore = "invoked by the isolated source-place environment fixture in workspace_test_runner"]
fn test_source_place_environment_child() {
    let Ok(expected) = std::env::var(constants_str::SOURCE_PLACE_TEST_EXPECTED_ENV_KEY) else {
        return;
    };
    let source_place_type = if expected == constants_str::SRC_ALT {
        crate::source_place_type::SourcePlaceType::Src
    } else {
        assert_eq!(expected, constants_str::GITHUB_ALT);
        crate::source_place_type::SourcePlaceType::Github
    };
    assert_eq!(
        crate::source_place_type::SourcePlaceType::from_env_or_default(),
        source_place_type
    );
}
