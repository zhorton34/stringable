use stringable::stringable::Stringable;

#[test]
fn test_class_basename() {
    let s = Stringable::new("App\\Controllers\\HomeController");
    assert_eq!(s.class_basename().get_value(), "HomeController");
}
