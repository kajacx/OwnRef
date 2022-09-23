//#[macro_export]
macro_rules! new_own_ref {
    ($variable:ident) => {
        unsafe {
            crate::structs::own_ref::OwnRef::new(::std::ptr::NonNull::new_unchecked(
                &mut $variable as *mut _,
            ))
        }
    };
}

pub(crate) use new_own_ref;
