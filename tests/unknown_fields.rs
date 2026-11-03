//! A response field added to the API after this release is kept in the model's `additional_properties`, in a stable
//! order, and written back when the model is serialized

use docspring::models::AuthenticationCheck;

#[test]
fn unknown_fields_are_kept_in_a_stable_order() {
    let names = [
        "zeta", "alpha", "mu", "beta", "omega", "delta", "kappa", "gamma",
    ];
    let mut json = serde_json::Map::new();
    json.insert("authenticated".to_owned(), serde_json::Value::Bool(true));
    for (index, name) in names.iter().enumerate() {
        json.insert((*name).to_owned(), serde_json::json!({ "index": index }));
    }
    let json = serde_json::Value::Object(json);

    let check: AuthenticationCheck = serde_json::from_value(json.clone()).unwrap();
    assert!(check.authenticated);
    let keys: Vec<&str> = check
        .additional_properties
        .keys()
        .map(String::as_str)
        .collect();
    // The order they came in with serde_json's preserve_order feature, sorted by name without it: never random
    let mut sorted = names.to_vec();
    sorted.sort_unstable();
    assert!(keys == names || keys == sorted, "{keys:?}");
    assert_eq!(
        check.additional_properties["kappa"],
        serde_json::json!({ "index": 6 })
    );

    assert_eq!(serde_json::to_value(&check).unwrap(), json);
    let known_only = AuthenticationCheck {
        authenticated: true,
        ..Default::default()
    };
    assert_eq!(
        serde_json::to_string(&known_only).unwrap(),
        r#"{"authenticated":true}"#
    );
}
