use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::ptr::NonNull;

use crate::OwnRef;

pub struct OwnRefMut<'a, T> {
    pointer: NonNull<T>,
    _phantom_lifetime: PhantomData<&'a ()>,
    _phantom_value: PhantomData<T>,
}

impl<'a, T> OwnRefMut<'a, T> {
    pub unsafe fn new<'b>(reference: &'b mut T, _phantom_reference: PhantomData<&'a ()>) -> Self {
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

    pub fn into_own_ref(self) -> OwnRef<'a, T> {
        /* SAFETY:
         * No references to self to to self.pointer exists, because we are consuming self by value.
         * Returned object still has the same lifetime, so it cannot outlive the original value.
         * Lastly, we forget ourselfs, so the data at pointer is not deallocated twice.
         */
        let result = unsafe { OwnRef::new(self.pointer.as_ref(), self._phantom_lifetime) };
        std::mem::forget(self);
        result
    }

    pub fn take(self) -> T {
        let result = unsafe { std::ptr::read(self.pointer.as_ptr()) };
        std::mem::forget(self); // do not run the destructor
        result
    }
}

impl<'a, T> Deref for OwnRefMut<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

impl<'a, T> DerefMut for OwnRefMut<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.get_mut()
    }
}

impl<T> Drop for OwnRefMut<'_, T> {
    fn drop(&mut self) {
        let _drop = unsafe { std::ptr::read(self.pointer.as_ptr()) };
    }
}
