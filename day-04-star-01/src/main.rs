fn main() {
    let puzzle_input = include_str!("../puzzle_input.txt");
    let count = puzzle_input.lines().filter(|line| is_valid(line)).count();
    println!("{}", count);
}

fn is_valid(passphrase: &str) -> bool {
    let mut histogram = std::collections::HashMap::new();
    for word in passphrase.split_whitespace() {
        let count = histogram.entry(word).or_insert(0);
        *count += 1;

        if *count > 1 {
            return false
        }
    }

    true
}

#[test]
fn example_one() {
    assert_eq!(is_valid("aa bb cc dd ee"), true)
}

#[test]
fn example_two() {
    assert_eq!(is_valid("aa bb cc dd aa"), false)
}

#[test]
fn example_three() {
    assert_eq!(is_valid("aa bb cc dd aaa"), true)
}
