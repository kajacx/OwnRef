use own_ref::{new_own_ref, OwnRef};

fn main() {
    let text = misbehaving("Hello");

    println!("Oh no, deallocated memory: {}", text.as_str());
}

fn misbehaving(text: &str) -> OwnRef<String> {
    let owned = text.to_owned();

    new_own_ref!(as_own_ref, owned);

    as_own_ref
}
