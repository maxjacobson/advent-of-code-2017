fn main() {
    let input = include_str!("../puzzle_input.txt");
    println!("{}", how_many_cycles(input));
}

#[derive(Debug)]
struct Bank {
    values: Vec<u32>,
}

impl Bank {
    fn from_input(input: &str) -> Bank {
        Bank {
            values: input
                .split_whitespace()
                .map(|word| word.parse::<u32>().unwrap())
                .collect(),
        }
    }

    fn to_s(&self) -> String {
        format!("{:?}", self.values)
    }

    fn how_many_cycles(&mut self) -> usize {
        let mut count = 0;
        let mut revisions: Vec<String> = vec![self.to_s()];

        // println!("{:?}", self.to_s());
        loop {
            self.reallocate();
            // println!("{:?}", self.to_s());
            count += 1;
            let representation = self.to_s();

            if revisions.contains(&representation) {
                // println!("Returning {}", count);
                return count;
            }

            revisions.push(representation);
        }
    }

    fn reallocate(&mut self) {
        // determine which block is the largest

        let largest_index = self.values
            .iter()
            .enumerate()
            .max_by(|x, y| {
                (x.1.cmp(y.1)).then_with(|| y.0.cmp(&x.0))
            })
            .unwrap()
            .0;

        let largest_value = self.values[largest_index];
        // println!("Redistributing {} from index {}", largest_value, largest_index);

        //println!("{:?}", self.values);

        // println!("Resetting idx {} to 0", largest_index);
        // reset that block to 0
        self.values[largest_index] = 0;
        // println!("{:?}", self.values);

        // distribute the data across all the blocks
        let idxes = (0..self.values.len()).cycle().skip(largest_index + 1);
        for (_, idx) in (0..largest_value).zip(idxes) {
            // println!("Incrementing idx {} by 1", idx);

            self.values[idx] += 1;

            //println!("{:?}", self.values);
        }
    }
}

fn how_many_cycles(input: &str) -> usize {
    let mut bank = Bank::from_input(input);

    // println!("{:?}", bank);

    bank.how_many_cycles()
}

#[test]
fn example_one() {
    let input = include_str!("../example_input.txt");

    assert_eq!(how_many_cycles(input), 5);
}

// #[test]
// fn test_bank() {
//     let mut bank = Bank::from_input("0 2 7 0");

//     assert_eq!(bank.values, vec![0, 2, 7, 0]);

//     bank.reallocate();

//     assert_eq!(bank.values, vec![2, 4, 1, 2]);

//     bank.reallocate();

//     assert_eq!(bank.values, vec![3, 1, 2, 3]);

//     assert_eq!(bank.to_s(), String::from("[3, 1, 2, 3]"));
// }
