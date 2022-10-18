use crate::{macros::new_own_ref::new_own_ref, structs::own_ref_mut::OwnRefMut};

#[test]
fn creating_new_own_ref_should_compile() {
    let text: String = "Hello".to_owned();

    new_own_ref!(own_ref, text);

    assert_eq!(own_ref.get(), "Hello");
}

#[test]
fn should_be_able_to_mutate() {
    let mut text = "Hello".to_string();

    new_own_ref!(mut own_ref, text);

    fn mutate_and_return(mut mut_ref: OwnRefMut<'_, String>) -> String {
        mut_ref.push_str(" world");
        mut_ref.take()
    }

    let result = mutate_and_return(own_ref);

    assert_eq!(result.as_str(), "Hello world");
}

/*#[test]
fn main() {
    let text = "Hello".to_string();

    new_own_ref!(mut own_ref, text);

    own_ref.push_str(" world");

    assert_eq!(own_ref.as_str(), "Hello");
}*/
