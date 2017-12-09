fn main() {
    let input = include_str!("../puzzle_input.txt");
    println!("{}", how_many_steps(input));
}

fn how_many_steps(input: &str) -> i32 {
    let mut current_instruction_index: i32 = 0;
    let mut instructions: Vec<i32> = input
        .split_whitespace()
        .map(|word| word.parse::<i32>().unwrap())
        .collect();

    let mut step_count = 0;
    loop {
        // This might not have been necessary
        if current_instruction_index < 0 {
            return step_count;
        }

        match instructions.get_mut(current_instruction_index as usize) {
            Some(jump) => {
                step_count += 1;
                current_instruction_index += *jump;
                *jump += 1;
            }
            None => return step_count,
        }
    }
}

#[test]
fn example_one() {
    let input = include_str!("../example_one.txt");
    assert_eq!(how_many_steps(input), 5)
}
