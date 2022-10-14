use std::borrow::Borrow;
use std::marker::PhantomData;
use std::ptr::NonNull;

pub struct OwnRef<'a, T> {
    pointer: NonNull<T>,
    _phantom_lifetime: PhantomData<&'a ()>,
    _phantom_value: PhantomData<T>,
}

impl<'a, T> OwnRef<'a, T> {
    pub unsafe fn new<'b>(reference: &'b mut T, _phantom_lifetime: &'a ()) -> Self {
        Self {
            pointer: NonNull::new_unchecked(reference as *mut T),
            _phantom_lifetime: PhantomData,
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

impl<'a, T> Borrow<T> for OwnRef<'a, T> {
    fn borrow(&self) -> &T {
        self.get()
    }
}

impl<T> Drop for OwnRef<'_, T> {
    fn drop(&mut self) {
        let _drop = unsafe { std::ptr::read(self.pointer.as_ptr()) };
    }
}
