use own_ref::internals::{lifetime_of, new_own_ref};

fn main() {
    let text = "Hello".to_string();

    let _phantom = ();
    let owned_ref = unsafe { new_own_ref(&text, lifetime_of(&_phantom)) };
    std::mem::forget(text);

    assert_eq!(owned_ref.as_str(), "Hello");
}

fn fail() -> i32 {
    // TODO: to check the message of the warning, the compilation needs to fail.

    // Only then will trybuild check the (errors and) warnings.
}
