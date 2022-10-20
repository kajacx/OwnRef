use own_ref::new_own_ref;

fn main() {
    let text = "Hello".to_string();

    new_own_ref!(_own_ref, text);

    deprecated(); // We should still see a deprecated waring here
                  // (it should not be suppressed by the #[allow(deprecated)] in the new_own_ref macro)
}

#[deprecated]
fn deprecated() {}

fn fail() -> i32 {
    // TODO: to check the message of the warning, the compilation needs to fail.

    // Only then will trybuild check the (errors and) warnings.
}
