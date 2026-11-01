// Borrowing: use a value through a reference without taking ownership.
fn main() {
    let mut s = String::from("hello");

    let len = length(&s); // immutable borrow
    println!("{s} has length {len}");

    append_world(&mut s); // mutable borrow (only one at a time)
    println!("{s}");
}

fn length(s: &String) -> usize {
    s.len()
}

fn append_world(s: &mut String) {
    s.push_str(", world");
}
