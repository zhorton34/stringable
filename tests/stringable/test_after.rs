use stringable::stringable::Stringable;

#[test]
fn test_after() {
  let input = "Hello, world!";
  let stringable = Stringable::new(input).after(",");
  let result = stringable.get_value();
  assert_eq!(result, " world!");

}


