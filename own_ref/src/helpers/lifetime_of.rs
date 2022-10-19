use std::marker::PhantomData;

pub fn lifetime_of<'a, T>(_: &'a T) -> PhantomData<&'a T> {
    PhantomData
}
