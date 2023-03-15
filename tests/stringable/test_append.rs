use stringable::stringable::Stringable;

#[test]
fn test_append() {
    let mut input = Stringable::new("Hello");
    input.append(&[", ", "world!"]);
    let result = input.get_value();
    assert_eq!(result, "Hello, world!");
}
