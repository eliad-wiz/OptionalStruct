use optional_struct::*;

#[optional_struct]
#[optional_derive(Clone, PartialEq, Debug)]
struct Config {
    delay: Option<u32>,
    path: String,
}

#[test]
fn test_optional_derive() {
    let opt = OptionalConfig {
        delay: None,
        path: Some("/tmp".to_owned()),
    };

    // Clone works
    let opt2 = opt.clone();
    // PartialEq works
    assert_eq!(opt, opt2);
}
