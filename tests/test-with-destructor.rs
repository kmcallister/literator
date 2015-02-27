#[macro_use]
extern crate literator;

use std::sync::atomic::{AtomicUsize, ATOMIC_USIZE_INIT, Ordering};

static DROPS: AtomicUsize = ATOMIC_USIZE_INIT;

fn get() -> usize {
    DROPS.load(Ordering::SeqCst)
}

struct Foo(&'static str);

impl Drop for Foo {
    fn drop(&mut self) {
        println!("dropping {}", self.0);
        DROPS.fetch_add(1, Ordering::SeqCst);
    }
}

#[test]
fn test_with_destructor() {
    {
        assert_eq!(get(), 0);
        let mut v = literator![Foo("earth"), Foo("water"), Foo("wind"), Foo("fire")];
        assert_eq!(get(), 0);
        v.next();
        assert_eq!(get(), 1);
        v.next();
        assert_eq!(get(), 2);
        println!("dropping the iterator");
    }
    assert_eq!(get(), 4);
}
