use own_ref::new_own_ref;

fn main() {
    let external;

    if 1 > 2 {
        let scoped = "Hello".to_owned();
        new_own_ref!(as_own_ref, scoped);
        external = as_own_ref;
    } else {
        return;
    }

    println!("Oh no, out of scope memory: {}", external.as_str());
}
