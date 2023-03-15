use stringable::stringable::Stringable;

#[test]
fn test_to_uppercase() {
    let input = "Hello, World!";
    let stringable = Stringable::new(input).to_uppercase();
    let result = stringable.get_value();
    assert_eq!(result, "HELLO, WORLD!");
}
