use std::borrow::Borrow;
use std::marker::PhantomData;
use std::ptr::NonNull;

pub struct OwnRef<'a, T: ?Sized> {
    pointer: NonNull<T>,
    _phantom_lifetime: PhantomData<&'a ()>,
    _phantom_value: PhantomData<T>,
}

impl<'a, T: ?Sized> OwnRef<'a, T> {
    pub unsafe fn new<'b>(reference: &'b mut T, _phantom_reference: &'a ()) -> Self {
        Self {
            pointer: NonNull::new_unchecked(reference as *mut T),
            _phantom_lifetime: PhantomData,
            _phantom_value: PhantomData,
        }
    }
}

impl<'a, T: ?Sized> Borrow<T> for OwnRef<'a, T> {
    fn borrow(&self) -> &T {
        unsafe { self.pointer.as_ref() }
    }
}
