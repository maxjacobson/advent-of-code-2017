fn main() {
    let puzzle_input = include_str!("../puzzle_input.txt");
    let count = puzzle_input.lines().filter(|line| is_valid(line)).count();
    println!("{}", count);
}

fn is_anagram(a: &str, b: &str) -> bool {
    let mut a_chars: Vec<char> = a.chars().collect();
    a_chars.sort();
    let mut b_chars: Vec<char> = b.chars().collect();
    b_chars.sort();

    a_chars == b_chars
}

fn is_valid(passphrase: &str) -> bool {
    passphrase.split_whitespace().enumerate().all(|(index, word)| {
        for (other_index, other_word) in passphrase.split_whitespace().enumerate() {
            if other_index == index { continue; }
            if is_anagram(word, other_word) { return false }
        }

        true
    })
}

#[test]
fn example_one() {
    assert!(is_valid("abcde fghij"))
}

#[test]
fn example_two() {
    assert!(!is_valid("abcde xyz ecdab"))
}

#[test]
fn example_three() {
    assert!(is_valid("a ab abc abd abf abj"))
}

#[test]
fn example_four() {
    assert!(is_valid("iiii oiii ooii oooi oooo"))
}

#[test]
fn example_five() {
    assert!(!is_valid("oiii ioii iioi iiio"))
}
