use std::marker::PhantomData;
use std::ops::Deref;
use std::ptr::NonNull;

pub struct OwnRef<'a, T> {
    pointer: NonNull<T>,
    _phantom_lifetime: PhantomData<&'a ()>,
    _phantom_value: PhantomData<T>,
}

impl<'a, T> OwnRef<'a, T> {
    pub(crate) unsafe fn new<'b>(reference: &'b T, _phantom_lifetime: PhantomData<&'a ()>) -> Self {
        Self {
            pointer: NonNull::from(reference),
            _phantom_lifetime,
            _phantom_value: PhantomData,
        }
    }

    pub(crate) fn from_pointer(
        pointer: NonNull<T>,
        _phantom_lifetime: PhantomData<&'a ()>,
    ) -> Self {
        Self {
            pointer,
            _phantom_lifetime,
            _phantom_value: PhantomData,
        }
    }

    pub fn get(&self) -> &T {
        unsafe { self.pointer.as_ref() }
    }

    pub fn take(self) -> T {
        let result = unsafe { std::ptr::read(self.pointer.as_ptr()) };
        std::mem::forget(self); // do not run destructor
        result
    }
}

impl<'a, T> Deref for OwnRef<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

impl<T> Drop for OwnRef<'_, T> {
    fn drop(&mut self) {
        let _drop = unsafe { std::ptr::read(self.pointer.as_ptr()) };
    }
}
