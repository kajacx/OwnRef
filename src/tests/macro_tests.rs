use crate::macros::new_own_ref::new_own_ref;

#[test]
fn creating_new_own_ref_should_compile() {
    let mut text: String = "Hello".to_owned();

    new_own_ref!(own_ref, text);

    assert_eq!(own_ref.get(), "Hello");
}
