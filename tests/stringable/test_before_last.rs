use stringable::stringable::Stringable;

#[test]
fn test_before_last() {
    let s = Stringable::new("This is a test.");
    assert_eq!(s.before_last("is").get_value(), "This ");
}
