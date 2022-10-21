use own_ref::new_own_ref;

fn main() {
    let text = "Hello".to_owned();

    new_own_ref!(owned_ref, text);

    assert_eq!(text, owned_ref.take());
}
