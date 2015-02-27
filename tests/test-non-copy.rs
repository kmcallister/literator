use std::borrow::ToOwned;

#[macro_use]
extern crate literator;

#[test]
fn test_non_copy() {
    let v: Vec<String> = container!["foo".to_owned(), "bar".to_owned()];
    assert_eq!(&v, &["foo", "bar"]);
}
