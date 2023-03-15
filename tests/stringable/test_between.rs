use stringable::stringable::Stringable;

#[test]
fn test_between() {
    let s = Stringable::new("This is a test.");
    assert_eq!(s.between("This", "test").get_value(), " is a ");
}
