use stringable::stringable::Stringable;

#[test]
fn test_dirname() {
    let s = Stringable::new("/path/to/file.txt");
    assert_eq!(s.dirname().get_value(), "/path/to");

    let s = Stringable::new("/path/to/dir/");
    assert_eq!(s.dirname().get_value(), "/path/to");
}
