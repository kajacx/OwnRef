macro_rules! prepare_ref {
    () => {
        let __own_ref_phantom_lifetime_marker = ();
    };
}

macro_rules! new_own_ref {
    ($variable:ident) => {
        unsafe {
            crate::structs::own_ref::OwnRef::new(
                &mut $variable,
                //&__own_ref_phantom_lifetime_marker,
                todo!(),
            );
        }
    };
}

pub(crate) use new_own_ref;
