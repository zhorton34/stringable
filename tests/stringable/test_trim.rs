use stringable::stringable::Stringable;

#[test]
fn test_trim() {
    let input = "   Hello, World!   ";
    let mut stringable = Stringable::new(input);
    let result = stringable.trim().get_value();
    assert_eq!(result, "Hello, World!");
}
