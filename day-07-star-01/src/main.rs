#[macro_use]
extern crate lazy_static;
extern crate regex;
use regex::Regex;

fn main() {
    let input = include_str!("../puzzle_input.txt");
    println!("{}", root(input).unwrap());
}

#[derive(Debug)]
struct Program<'a> {
    name: &'a str,
    children: Option<Vec<&'a str>>,
}

impl<'a> Program<'a> {
    fn from_line(input: &str) -> Program {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(\w+) \(\d+\)( -> [\w,\s]+)?").unwrap();
        }

        lazy_static! {
            static ref RE2: Regex = Regex::new(r" -> (.+)").unwrap();
        }

        let caps = RE.captures(input).unwrap();
        // println!("{:?}", caps);

        let children = match caps.get(2) {
            Some(cap) => {
                Some(
                    RE2.captures(cap.as_str())
                        .unwrap()
                        .get(1)
                        .unwrap()
                        .as_str()
                        .split(", ")
                        .collect(),
                )
            }
            None => None,
        };

        let ret = Program {
            name: caps.get(1).unwrap().as_str(),
            children: children,
        };

        // println!("{:?}", ret);

        ret
    }

    fn is_parent(&self) -> bool {
        self.children.is_some()
    }

    fn has_child(&self, other_program: &Program) -> bool {
        match self.children {
            Some(ref children) => children.contains(&other_program.name),
            None => false,
        }
    }
}

fn root(input: &str) -> Option<&str> {
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

#[test]
fn example() {
    let input = include_str!("../example_input.txt");
    assert_eq!(root(input), Some("tknk"))
}
