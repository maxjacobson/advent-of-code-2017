use std::fs::File;
use std::io::prelude::*;

fn main() {
    println!("{}", checksum("puzzle_input.txt"))
}

fn checksum(filename: &str) -> u32 {
    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect(
        "something went wrong reading the file",
    );

    contents.lines().fold(0, |accumulator, line| {
        let mut values: Vec<u32> = line.split_whitespace()
            .map(|number_string| {
                number_string.parse::<u32>().expect("could not parse")
            })
            .collect();
        values.sort();

        let delta = 'outer: loop {
            let numerator = values.pop().expect("Expected there still to be values to check");
            for i in 0..(values.len()) {
                if numerator % values[i] == 0 {
                    break 'outer numerator / values[i]
                }
            }
        };

        accumulator + delta
    })
}

#[test]
fn example() {
    assert_eq!(checksum("example.txt"), 9)
}

#[test]
fn example_2() {
    assert_eq!(checksum("example2.txt"), 13)
}
