#[macro_export]
macro_rules! new_own_ref {
    (mut $ref_name:ident, $data_variable:ident) => {
        let _phantom = ();
        let mut $ref_name = unsafe {
            // SAFETY:
            // We know the OwnRef will not outlive the current stack,
            // because of the _phantom lifetime.
            // Also, the data cannot cannot be used after "moving it" into the OwnRef
            // because we forget it immediately.
            // Finally, the data will not be deallocated twice,
            // because we forget it and not drop it.
            #[allow(deprecated)]
            own_ref::OwnRefMut::new(&mut $data_variable, own_ref::lifetime_of(&_phantom))
        };
        std::mem::forget($data_variable);
    };
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
            #[allow(deprecated)]
            own_ref::OwnRef::new(&$data_variable, own_ref::lifetime_of(&_phantom))
        };
        std::mem::forget($data_variable);
    };
}

// pub(crate) use new_own_ref; TODO: remove for good?
