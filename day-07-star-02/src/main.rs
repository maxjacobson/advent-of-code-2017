// actually build the tree
// walk the tree until we find an imbalanced child
// return the amount it is off from being balanced

static PUZZLE_INPUT: &'static str = include_str!("../puzzle_input.txt");
static EXAMPLE_INPUT: &'static str = include_str!("../example_input.txt");

#[macro_use]
extern crate lazy_static;
extern crate regex;
use regex::Regex;

fn main() {
    println!("{}", proper_weight_of_imbalanced_node(PUZZLE_INPUT));
}

#[derive(Clone, Debug)]
struct Program {
    name: String,
    children_names: Option<Vec<String>>,
    children: Vec<Program>,
    weight: i32,
}

impl Program {
    fn from_line(line: &str) -> Program {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(\w+) \((\d+)\)( -> [\w,\s]+)?").unwrap();
        }

        lazy_static! {
            static ref RE2: Regex = Regex::new(r" -> (.+)").unwrap();
        }

        let children_names = match RE.captures(line).unwrap().get(3) {
            Some(cap) => {
                Some(
                    RE2.captures(cap.as_str())
                        .unwrap()
                        .get(1)
                        .unwrap()
                        .as_str()
                        .split(", ")
                        .map(|word| word.to_owned())
                        .collect(),
                )
            }
            None => None,
        };

        let name = RE.captures(line)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .to_owned();

        let weight = RE.captures(line)
            .unwrap()
            .get(2)
            .unwrap()
            .as_str()
            .parse::<i32>()
            .unwrap();

        Program {
            name: name,
            children_names: children_names,
            children: vec![],
            weight: weight,
        }
    }

    fn is_parent(&self) -> bool {
        self.children_names.is_some()
    }

    fn has_child(&self, other_program: &Program) -> bool {
        match self.children_names {
            Some(ref children_names) => children_names.contains(&other_program.name),
            None => false,
        }
    }

    fn enrich(&mut self, input: &str) {
        match self.children_names {
            Some(ref children_names) => {
                for child_name in children_names {
                    let mut child = find_by_name(&input, child_name);
                    child.enrich(&input);
                    self.children.push(child);
                }
            }
            _ => {}
        }
    }

    fn full_weight(&self) -> i32 {
        self.children.iter().fold(self.weight, |sum, child| {
            sum + child.full_weight()
        })
    }

    fn children_weights(&self) -> (std::collections::HashMap<i32, i32>, Vec<(String, i32)>) {
        let mut occurrences_frequency = std::collections::HashMap::new();
        let mut mapping = vec![];
        self.children.iter().for_each(|child| {
            let full_weight = child.full_weight();
            let value = occurrences_frequency.entry(full_weight).or_insert(0);
            *value += 1;
            mapping.push((child.name.clone(), full_weight));
        });

        (occurrences_frequency, mapping)
    }

    fn find_imbalance(&self) -> i32 {
        println!("Checking {:?} for imbalance", self.name);

        let (occurrences_frequency, mapping) = self.children_weights();

        if !occurrences_frequency.is_empty() {
            let mut values: Vec<&i32> = occurrences_frequency.keys().collect();
            values.sort();
            if values.first().unwrap() != values.last().unwrap() {
                // we are imbalanced, baby
                let abberant_value = occurrences_frequency
                    .iter()
                    .min_by(|x, y| x.1.cmp(y.1))
                    .unwrap()
                    .0;
                let normal_value = occurrences_frequency
                    .iter()
                    .max_by(|x, y| x.1.cmp(y.1))
                    .unwrap()
                    .0;

                let aberrant_name = &mapping
                    .iter()
                    .find(|mapping_item| &mapping_item.1 == abberant_value)
                    .unwrap()
                    .0;

                println!(
                    "Abberant value: {}; Abberant name: {}, Normal value: {}",
                    abberant_value,
                    aberrant_name,
                    normal_value
                );

                let child = self.children
                    .iter()
                    .find(|child| &child.name == aberrant_name)
                    .unwrap();

                let (child_occurrences_frequency, _) = child.children_weights();
                let child_is_balanced = {
                    if child_occurrences_frequency.is_empty() {
                        true
                    } else {
                        let mut child_values: Vec<&i32> =
                            child_occurrences_frequency.keys().collect();
                        child_values.sort();
                        child_values.first().unwrap() == child_values.last().unwrap()
                    }
                };

                return if child_is_balanced {
                    // base case
                    child.weight + normal_value - abberant_value
                } else {
                    // recursion
                    child.find_imbalance()
                };
            }
        }

        panic!("omg")
    }
}

fn root(input: &str) -> Option<String> {
    for program in input.lines().map(|line| Program::from_line(line)) {
        if program.is_parent() {
            if input.lines().map(|line| Program::from_line(line)).any(
                |any_program| any_program.has_child(&program),
            ) == false
            {
                return Some(program.name);
            }
        }
    }

    None
}

// find by name and remove from pool
fn find_by_name(input: &str, name: &str) -> Program {
    let programs: Vec<Program> = input.lines().map(|line| Program::from_line(line)).collect();
    let idx = programs
        .iter()
        .position(|program| program.name == name)
        .unwrap();
    programs[idx].clone()
}

fn proper_weight_of_imbalanced_node(input: &str) -> i32 {
    let root_name = root(input).unwrap();

    let mut tree = find_by_name(input, &root_name);
    tree.enrich(&input);

    println!("{:?}", tree);

    tree.find_imbalance()
}

#[test]
fn example() {
    assert_eq!(root(EXAMPLE_INPUT), Some(String::from("tknk")));
    assert_eq!(proper_weight_of_imbalanced_node(EXAMPLE_INPUT), 60);
}
