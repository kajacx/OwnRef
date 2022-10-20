use std::marker::PhantomData;

#[deprecated(
    since = "0.1.0",
    note = "This is a helper method for the `new_own_ref` macro and is not intended to be called directly."
)]
pub fn lifetime_of<'a, T>(_: &'a T) -> PhantomData<&'a T> {
    PhantomData
}
