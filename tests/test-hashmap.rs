#[macro_use]
extern crate literator;

use std::collections::HashMap;

#[test]
fn test_hashmap() {
    let h: HashMap<_, _> = container! {
        1 => 'x',
        2 => 'y',
    };

    assert_eq!(h[&1], 'x');
    assert_eq!(h[&2], 'y');
    assert_eq!(h.len(), 2);
}
