#[macro_export]
macro_rules! new_own_ref {
    ($ref_name:ident, $data_variable:ident) => {
        let _phantom = ();
        let $ref_name = unsafe {
            // SAFETY:
            // We know the OwnRef will not outlive the current stack,
            // because of the _phantom lifetime.
            // Also, the data cannot cannot be used after "moving it" into the OwnRef
            // because we forget it immediately.
            // Finally, the data will not be deallocated twice,
            // because we forget it and not drop it.
            crate::structs::own_ref::OwnRef::new(&mut $data_variable, &_phantom)
        };
        std::mem::forget($data_variable);
    };
}

pub(crate) use new_own_ref;
