use own_ref::{new_own_ref, OwnRef, OwnRefMut};

#[test]
fn creating_new_own_ref_should_compile() {
    let text: String = "Hello".to_owned();

    new_own_ref!(own_ref, text);

    assert_eq!(own_ref.get(), "Hello");
}

#[test]
fn can_borrow_value_in_inner_function() {
    let text: String = "Hello".to_owned();

    new_own_ref!(own_ref, text);

    fn inner(arg: OwnRef<String>) {
        let data = vec!["Pretty", arg.get()];
        assert_eq!(data[1], "Hello")
    }

    inner(own_ref);
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

#[test]
fn should_convert_own_ref_mut_to_own_ref() {
    let mut text = "Hello".to_string();

    new_own_ref!(mut own_ref, text);

    own_ref.push_str(" world");

    fn takes_own_ref(owned_ref: OwnRef<String>) -> String {
        owned_ref.take()
    }

    let taken = takes_own_ref(own_ref.into_own_ref());

    assert_eq!(taken, "Hello world");
}

#[test]
fn can_create_multiple_own_refs() {
    let text1 = "Hello".to_owned();
    new_own_ref!(own_ref1, text1);

    let text2 = "world".to_owned();
    new_own_ref!(own_ref2, text2);

    assert_eq!(own_ref1.take(), "Hello");
    assert_eq!(own_ref2.take(), "world");
}
