use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).expect("stdin");
    let stats = textkit::stats(&input);
    println!("{}", serde_json::to_string_pretty(&stats).unwrap());
    println!("tag={} rustc={}", env!("BUILD_TAG"), env!("BUILD_RUSTC"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn build_tag_is_set() {
        assert!(!env!("BUILD_TAG").is_empty());
    }
}
