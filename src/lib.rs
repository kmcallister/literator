#![feature(raw)]

extern crate fixed_size_array;

use std::{ptr, raw};
use std::marker::PhantomData;
use std::raw::Repr;

use fixed_size_array::FixedSizeArray;

/// An iterator constructed by the `literator!` macro.
pub struct Literator<Array, Elem>
    where Array: FixedSizeArray<Elem=Elem>
{
    pos: isize,
    array: Option<Array>,
    phantom: PhantomData<[Elem; 17]>,
}

impl<Array, Elem> Literator<Array, Elem>
    where Array: FixedSizeArray<Elem=Elem>
{
    pub fn new(array: Array) -> Literator<Array, Elem> {
        Literator {
            pos: 0,
            array: Some(array),
            phantom: PhantomData,
        }
    }

    fn as_mut_slice<'a>(&'a mut self) -> &'a mut [Elem] {
        self.array.as_mut().unwrap().as_mut_slice()
    }
}

impl<Array, Elem> Iterator for Literator<Array, Elem>
    where Array: FixedSizeArray<Elem=Elem>
{
    type Item = Elem;

    fn next(&mut self) -> Option<Elem> {
        let raw::Slice { data, len } = self.as_mut_slice().repr();
        let i = self.pos;
        if i as usize >= len {
            None
        } else {
            self.pos += 1;
            Some(unsafe {
                // We know we'll never use this array location again,
                // so it's okay to force a move out.
                ptr::read(data.offset(i) as *mut Elem)
            })
        }
    }
}

impl<Array, Elem> Drop for Literator<Array, Elem>
    where Array: FixedSizeArray<Elem=Elem>
{
    fn drop(&mut self) {
        unsafe {
            let raw::Slice { data, len } = self.as_mut_slice().repr();
            let mut i = self.pos;
            while (i as usize) < len {
                // drop remaining elements
                ptr::read(data.offset(i) as *mut Elem);
                i += 1;
            }
            ptr::write(&mut self.array, None);
        }
    }
}

/// Given any number of values, produces an iterator that yields those
/// values one by one.
#[macro_export]
macro_rules! literator {
    ($($x:expr),*) => (
        $crate::Literator::new([$($x),*])
    )
}

/// Initialize any `FromIter` container from a sequence of elements.
///
/// If the elements are pairs, you can use the sugar `x => y`.
#[macro_export]
macro_rules! container {
    ($($x:expr),*) => (
        ::std::iter::FromIterator::from_iter(literator!($($x),*))
    );
    ($($x:expr),*,) => (
        container!($($x),*)
    );
    ($( $x:expr => $y:expr),*) => (
        container!($(($x, $y)),*)
    );
    ($( $x:expr => $y:expr),*,) => (
        container!($( $x => $y ),*)
    );
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    #[test]
    fn smoke_test() {
        let v: Vec<_> = container![1, 2, 3];
        assert_eq!(&v, &[1, 2, 3]);

        let v: Vec<_> = container![1, 2, 3,];
        assert_eq!(&v, &[1, 2, 3]);

        let h: HashMap<_, _> = container! {
            1 => 'x',
            2 => 'y'
        };
        let mut v: Vec<_> = h.iter().collect();
        v.sort();
        assert_eq!(&v, &[(&1, &'x'), (&2, &'y')]);

        let h: HashMap<_, _> = container! {
            1 => 'x',
            2 => 'y',
        };
        let mut v: Vec<_> = h.iter().collect();
        v.sort();
        assert_eq!(&v, &[(&1, &'x'), (&2, &'y')]);
    }
}
