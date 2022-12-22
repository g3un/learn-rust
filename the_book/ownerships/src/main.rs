fn main() {
    let reference_to_nothing = dangle();
}

// Compile error
fn dangle() -> &String {
    let s = String::from("Hello");

    &s
}
