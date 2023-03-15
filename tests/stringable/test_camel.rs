use stringable::stringable::Stringable;

#[test]
fn test_camel() {
    let s = Stringable::new("snake_case_string");
    assert_eq!(s.camel().get_value(), "snakeCaseString");
}