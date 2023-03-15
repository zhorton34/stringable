use stringable::stringable::Stringable;

#[test]
fn test_between_first() {
    let s = Stringable::new("This is a test. And this is another test.");
    assert_eq!(s.between_first("This", "test").get_value(), " is a ");
}