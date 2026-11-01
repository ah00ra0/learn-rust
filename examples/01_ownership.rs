// Ownership: every value has exactly one owner.
fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // ownership moves to s2; s1 is no longer valid
    println!("{s2}");

    let s3 = s2.clone(); // explicit deep copy
    println!("{s2} {s3}");

    let n1 = 5;
    let n2 = n1; // integers are Copy, so both stay valid
    println!("{n1} {n2}");

    takes_ownership(s3);
    // println!("{s3}"); // error: s3 was moved into the function
}

fn takes_ownership(s: String) {
    println!("got {s}");
}
