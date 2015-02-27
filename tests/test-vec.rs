#[macro_use]
extern crate literator;

#[test]
fn test_vec() {
    let v: Vec<_> = container![1, 2, 3];
    assert_eq!(&v, &[1, 2, 3]);
}
