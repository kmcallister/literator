#[macro_use]
extern crate literator;

#[test]
fn test_literator() {
    let mut it = literator!['x', 'y', 'z'];
    assert_eq!(it.next(), Some('x'));
    assert_eq!(it.next(), Some('y'));
    assert_eq!(it.next(), Some('z'));
    assert_eq!(it.next(), None);
}
