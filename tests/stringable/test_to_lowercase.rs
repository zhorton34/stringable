use stringable::stringable::Stringable;

#[test]
fn test_to_lowercase() {
    let input = "HELLO, WORLD!";
    let mut stringable = Stringable::new(input);
    let result = stringable.to_lowercase().get_value();
    assert_eq!(result, "hello, world!");
}
