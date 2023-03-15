use stringable::stringable::Stringable;

#[test]
fn test_contains_all() {
    let s = Stringable::new("This is a test.");
    assert_eq!(s.contains_all(&["This", "test"]), true);
    assert_eq!(s.contains_all(&["This", "example"]), false);
}
