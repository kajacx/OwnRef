use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::ptr::NonNull;

pub struct OwnRefMut<'a, T> {
    pointer: NonNull<T>,
    _phantom_lifetime: PhantomData<&'a ()>,
    _phantom_value: PhantomData<T>,
}

impl<'a, T> OwnRefMut<'a, T> {
    pub unsafe fn new<'b>(reference: &'b mut T, _phantom_reference: &'a ()) -> Self {
        Self {
            pointer: NonNull::new_unchecked(reference as *mut T),
            _phantom_lifetime: PhantomData,
            _phantom_value: PhantomData,
        }
    }

    pub fn get(&self) -> &T {
        unsafe { self.pointer.as_ref() }
    }

    pub fn get_mut(&mut self) -> &mut T {
        unsafe { self.pointer.as_mut() }
    }

    pub fn take(self) -> T {
        let result = unsafe { std::ptr::read(self.pointer.as_ptr()) };
        std::mem::forget(self); // do not run the destructor
        result
    }
}

impl<'a, T> Deref for OwnRefMut<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.get()
    }
}

impl<'a, T> DerefMut for OwnRefMut<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        self.get_mut()
    }
}

impl<T> Drop for OwnRefMut<'_, T> {
    fn drop(&mut self) {
        let _drop = unsafe { std::ptr::read(self.pointer.as_ptr()) };
    }
}
