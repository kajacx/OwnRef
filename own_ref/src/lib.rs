mod macros;
mod structs;

use std::marker::PhantomData;

pub use structs::OwnRef;
pub use structs::OwnRefMut;

pub fn lifetime_of<'a, T>(_: &'a T) -> PhantomData<&'a T> {
    PhantomData
}
