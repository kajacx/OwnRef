#[macro_export]
macro_rules! new_own_ref {
    (mut $ref_name:ident, $data_variable:ident) => {
        let _own_ref_phantom_scope = ();
        let mut $ref_name = unsafe {
            // SAFETY:
            // We know the OwnRef will not outlive the current stack,
            // because of the _own_ref_phantom_scope lifetime.
            // Also, the data cannot cannot be used after "moving it" into the OwnRef
            // because we forget it immediately.
            // Finally, the data will not be deallocated twice,
            // because we forget it and not drop it.
            #[allow(deprecated)]
            own_ref::internals::new_own_ref_mut(
                &mut $data_variable,
                own_ref::internals::lifetime_of(&_own_ref_phantom_scope),
            )
        };
        std::mem::forget($data_variable);
    };
    ($ref_name:ident, $data_variable:ident) => {
        let _own_ref_phantom_scope = ();
        let $ref_name = unsafe {
            // SAFETY:
            // We know the OwnRef will not outlive the current stack,
            // because of the _own_ref_phantom_scope lifetime.
            // Also, the data cannot cannot be used after "moving it" into the OwnRef
            // because we forget it immediately.
            // Finally, the data will not be deallocated twice,
            // because we forget it and not drop it.
            #[allow(deprecated)]
            own_ref::internals::new_own_ref(
                &$data_variable,
                own_ref::internals::lifetime_of(&_own_ref_phantom_scope),
            )
        };
        std::mem::forget($data_variable);
    };
}
