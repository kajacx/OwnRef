use std::marker::PhantomData;

use crate::{OwnRef, OwnRefMut};

#[deprecated(
    since = "0.1.0",
    note = "Do not construct OwnRef manually, instead use the `new_own_ref` macro."
)]
pub unsafe fn new_own_ref<'a, 'b, T>(
    reference: &'b T,
    _phantom_lifetime: PhantomData<&'a ()>,
) -> OwnRef<'a, T> {
    OwnRef::new(reference, _phantom_lifetime)
}

#[deprecated(
    since = "0.1.0",
    note = "Do not construct OwnRefMut manually, instead use the `new_own_ref` macro."
)]
pub unsafe fn new_own_ref_mut<'a, 'b, T>(
    reference: &'b mut T,
    _phantom_lifetime: PhantomData<&'a ()>,
) -> OwnRefMut<'a, T> {
    OwnRefMut::new(reference, _phantom_lifetime)
}
