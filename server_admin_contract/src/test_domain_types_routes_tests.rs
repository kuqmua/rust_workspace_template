#[test]
fn test_route_contract_tests() {
    let metadata =
        <crate::admin_sign_in_route::AdminSignInRoute as frontend_contract::typed_route::TypedRoute>::metadata();
    assert_eq!(
        metadata.authentication(),
        frontend_contract::authentication_requirement::AuthenticationRequirement::Public
    );
    assert_eq!(metadata.path().as_ref(), "/auth/sign_in");
}
