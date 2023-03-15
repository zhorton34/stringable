use stringable::stringable::Stringable;

#[test]
fn test_after_last() {
    let input = "This is a test, and it's a good test.";
    let stringable = Stringable::new(input);
    let after_last_binding = stringable.after_last("test");
    let result = after_last_binding.get_value();
    assert_eq!(result, ".");
}
