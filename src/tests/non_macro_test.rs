#[cfg(test)]
use crate::structs::own_ref::OwnRef;

#[test]
fn creating_new_own_ref_should_compile() {
    let mut text: String = "Hello".to_owned();

    let phantom = ();
    let own_ref = unsafe { OwnRef::new(&mut text, &phantom) };
    std::mem::forget(text);

    assert_eq!(own_ref.get(), "Hello");
}

#[test]
fn can_take_value_in_inner_function() {
    let mut text: String = "Hello".to_owned();

    let phantom = ();
    let own_ref = unsafe { OwnRef::new(&mut text, &phantom) };
    std::mem::forget(text);

    fn inner(arg: OwnRef<String>) {
        let data = vec!["Pretty".to_owned(), arg.take()];
        assert_eq!(data[1], "Hello")
    }

    inner(own_ref);
}

#[test]
fn can_borrow_value_in_inner_function() {
    let mut text: String = "Hello".to_owned();

    let phantom = ();
    let own_ref = unsafe { OwnRef::new(&mut text, &phantom) };
    std::mem::forget(text);

    fn inner(arg: OwnRef<String>) {
        let data = vec!["Pretty", arg.get()];
        assert_eq!(data[1], "Hello")
    }

    inner(own_ref);
}
