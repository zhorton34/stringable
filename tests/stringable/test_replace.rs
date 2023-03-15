use stringable::stringable::Stringable;

#[test]
fn test_replace() {
    let input = "Hello, World!";
    let mut stringable = Stringable::new(input);
    let result = stringable.replace("World", "Universe").get_value();
    assert_eq!(result, "Hello, Universe!");
}
