use stringable::stringable::Stringable;

#[test]
fn test_before() {
    let input = "Hello, world!";
    let result = Stringable::new(input).before(",").get_value();
    assert_eq!(result, "Hello");
}
