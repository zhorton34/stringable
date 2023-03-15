use stringable::stringable::Stringable;

#[test]
fn test_basename() {
    let input = "/path/to/file.txt";
    let result = Stringable::new(input).basename(Some(".txt")).get_value();
    assert_eq!(result, "file");

    let result_no_suffix = Stringable::new(input).basename(None).get_value();
    assert_eq!(result_no_suffix, "file.txt");
}
