fn main() {
    let puzzle_input = include_str!("../puzzle_input.txt");
    println!("{}", score(puzzle_input));
}

#[derive(Debug, PartialEq)]
enum Thing {
    Group(Vec<Thing>),
    Garbage,
}

enum ConsumeAction {
    Garbage,
    Group,
    Done,
}

impl Thing {
    fn from_input(input: &str) -> Thing {
        let mut chars = input.trim().chars().peekable();
        let thing = Thing::from_chars(&mut chars);

        println!("{:?}", thing);

        thing
    }

    fn from_chars(chars: &mut std::iter::Peekable<std::str::Chars>) -> Thing {
        let mut group = Thing::Group(vec![]);

        'outer: loop {
            let c = chars.next().expect(
                "There must be a char when at start of a group",
            );
            assert_eq!(c, '{');

            match group {
                Thing::Group(ref mut children) => {
                    'children_consumer: loop {
                        if let Some(next_child) = Thing::consume_next_child(chars) {
                            children.push(next_child);
                            Thing::consume_comma(chars);
                        } else {
                            break 'outer;
                        }

                        // TODO: we need to handle consuming the comma when present
                    }
                }
                Thing::Garbage => panic!("wtf, wrong thing for state"),
            }
        }

        group
    }

    fn consume_comma(chars: &mut std::iter::Peekable<std::str::Chars>) {
        match chars.peek() {
            Some(&',') => {
                chars.next();
            }
            _ => {}
        }
    }

    fn consume_next_child(chars: &mut std::iter::Peekable<std::str::Chars>) -> Option<Thing> {
        let next_action = {
            match chars.peek() {
                Some(&'<') => ConsumeAction::Garbage,
                Some(&'{') => ConsumeAction::Group,
                Some(&'}') => ConsumeAction::Done,
                Some(other) => {
                    panic!(
                        "not able to consume children because of unexpected next character: {}",
                        other
                    );
                }
                None => panic!("not able to consume children because no next character"),
            }
        };

        match next_action {
            ConsumeAction::Garbage => {
                chars.next(); // consume the opening angle brace
                Thing::consume_until_garbage_ends(chars);
                Some(Thing::Garbage)
            }
            ConsumeAction::Group => Some(Thing::from_chars(chars)),
            ConsumeAction::Done => {
                chars.next(); // consume the closing curly
                None
            }
        }
    }

    fn consume_until_garbage_ends(chars: &mut std::iter::Peekable<std::str::Chars>) {
        loop {
            match chars.peek() {
                Some(&'!') => {
                    chars.next(); // consume the exclamation point
                    chars.next(); // consume the following character, which was "cancelled"
                }
                Some(&'>') => {
                    chars.next(); // consume the end of garbage character
                    return;
                }
                Some(_) => {
                    chars.next(); // consume the garbage
                }
                None => panic!("wtf - the garbage never ends"),
            }
        }
    }

    fn score(&self, parent_score: u32) -> u32 {
        match self {
            &Thing::Group(ref children) => {
                return parent_score +
                    children.iter().fold(0, |acc, ref child| {
                        acc + child.score(parent_score + 1)
                    })
            }
            &Thing::Garbage => return 0,
        }
    }
}

fn score(input: &str) -> u32 {
    Thing::from_input(input).score(1)
}

#[test]
fn example_one() {
    let thing = Thing::Group(vec![]);

    assert_eq!(thing.score(1), 1);

    assert_eq!(Thing::from_input("{}"), thing);

    // TODO:
    // assert_eq!(score("{}"), 1)
}


#[test]
fn example_two() {
    let thing = Thing::Group(vec![Thing::Group(vec![Thing::Group(vec![])])]);

    assert_eq!(thing.score(1), 6);

    assert_eq!(Thing::from_input("{{{}}}"), thing);

    // TODO:
    // assert_eq!(score("{{{}}}"), 6)
}

#[test]
fn example_two_and_half() {
    let thing = Thing::Group(vec![Thing::Group(vec![]), Thing::Group(vec![])]);

    assert_eq!(thing.score(1), 5);

    // TODO:
    // assert_eq!(score("{{},{}}"), 5)
}

#[test]
fn example_three() {
    let thing = Thing::Group(vec![
        Thing::Group(vec![
            Thing::Group(vec![]),
            Thing::Group(vec![]),
            Thing::Group(vec![Thing::Group(vec![])]),
        ]),
    ]);

    assert_eq!(thing.score(1), 16);
    // TODO:
    // assert_eq!(score("{{{},{},{{}}}}"), 16)
}

#[test]
fn example_four() {
    let thing = Thing::Group(vec![
        Thing::Garbage,
        Thing::Garbage,
        Thing::Garbage,
        Thing::Garbage,
    ]);
    assert_eq!(thing.score(1), 1);

    // TODO:
    // assert_eq!(score("{<a>,<a>,<a>,<a>}"), 1)
}

#[test]
fn example_five() {
    let thing = Thing::Group(vec![
        Thing::Group(vec![Thing::Garbage]),
        Thing::Group(vec![Thing::Garbage]),
        Thing::Group(vec![Thing::Garbage]),
        Thing::Group(vec![Thing::Garbage]),
    ]);
    assert_eq!(thing.score(1), 9);

    // {
    //   {
    //     <ab>
    //   },
    //   {
    //     <ab>
    //    },
    //    {
    //      <ab>
    //    },
    //    {
    //      <ab>
    //    }
    // }

    // TODO:
    // assert_eq!(score("{{<ab>},{<ab>},{<ab>},{<ab>}}"), 9)
}

#[test]
fn example_six() {
    let thing = Thing::Group(vec![
        Thing::Group(vec![Thing::Garbage]),
        Thing::Group(vec![Thing::Garbage]),
        Thing::Group(vec![Thing::Garbage]),
        Thing::Group(vec![Thing::Garbage]),
    ]);
    // {
    //   {
    //     <!!>
    //    },
    //    {
    //      <!!>
    //    },
    //    {
    //      <!!>
    //    },
    //    {
    //      <!!>
    //    }
    // }
    assert_eq!(thing.score(1), 9);
    // TODO:
    // assert_eq!(score("{{<!!>},{<!!>},{<!!>},{<!!>}}"), 9)
}

#[test]
fn example_seven() {
    let thing = Thing::Group(vec![Thing::Group(vec![Thing::Garbage])]);
    // {
    //   {
    //     <a!>},{<a!>},{<a!>},{<ab>
    //   }
    // }
    assert_eq!(thing.score(1), 3);
    // TODO:
    // assert_eq!(score("{{<a!>},{<a!>},{<a!>},{<ab>}}"), 3)
}
