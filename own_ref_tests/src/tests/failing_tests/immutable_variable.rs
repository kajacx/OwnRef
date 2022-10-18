use own_ref::new_own_ref;

fn main() {
    let text = "Hello".to_string();

    new_own_ref!(mut owned_ref, text);

    owned_ref.push_str(" world");

    assert_eq!(owned_ref.as_str(), "Hello");
}
