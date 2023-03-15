use stringable::stringable::Stringable;

#[test]
fn test_contains() {
    let s = Stringable::new("This is a test.");
    assert_eq!(s.contains("test"), true);
    assert_eq!(s.contains("example"), false);
}