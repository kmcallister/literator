# Container initialization

This library provides a macro for initializing any container implementing
[`FromIterator`](http://doc.rust-lang.org/std/iter/trait.FromIterator.html).

```rust
#[macro_use] extern crate literator;

use std::collections::HashMap;

fn main() {
    let v: Vec<_> = container![1, 2, 3];
    assert_eq!(&v, &[1, 2, 3]);

    let h: HashMap<_, _> = container! {
        (1, 'x'),
        (2, 'y'),
    };
    assert_eq!(h[1], 'x');
    assert_eq!(h[2], 'y');
    assert_eq!(h.len(), 2);
}
```

A Perl-ish sugar for pairs is also available:

```rust
let h: HashMap<_, _> = container! {
    1 => 'x',
    2 => 'y',
};
```

# Iterator literals

`container!` is built on top of an "iterator literal" macro:

```rust
let mut it = literator!['x', 'y', 'z'];
assert_eq!(it.next(), Some('x'));
assert_eq!(it.next(), Some('y'));
```

`literator!` works without heap allocation. The elements are moved into a
fixed-size array and then back out during iteration. Currently it supports [up
to 32 entries](https://github.com/kmcallister/fixed-size-array).  Once variadic
generics are available, there should be no limit.
