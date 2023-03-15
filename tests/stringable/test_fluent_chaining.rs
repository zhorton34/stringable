use stringable::stringable::Stringable;

#[test]
fn test_fluent_chaining() {
    let original = "This is a test, and it's a good test.   ";
    
    let result = Stringable::new(original)
        .replace("test", "example")
        .to_lowercase()
        .trim()
        .append(&[" Additional text."])
        .get_value()
        .to_string();

    assert_eq!(
        result, 
        "this is a example, and it's a good example. additional text."
    );
}
