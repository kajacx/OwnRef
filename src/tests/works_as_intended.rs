#[macro_use(crate::macros::new_own_ref::new_own_ref)]
use crate::macros::new_own_ref::new_own_ref;

#[test]
fn creating_new_own_ref_should_compile() {
    let mut text: String = "Hello".to_string();

    let own_ref = new_own_ref!(text);
}
