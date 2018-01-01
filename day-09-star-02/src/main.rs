fn main() {
    let puzzle_input = include_str!("../puzzle_input.txt");
    println!("{}", count_garbage_characters(puzzle_input));
}

#[derive(Debug, PartialEq)]
enum Thing {
    Group(Vec<Thing>),
    Garbage(u32),
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
                Thing::Garbage(_) => panic!("wtf, wrong thing for state"),
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
                let amount_of_garbage = Thing::consume_until_garbage_ends(chars);
                Some(Thing::Garbage(amount_of_garbage))
            }
            ConsumeAction::Group => Some(Thing::from_chars(chars)),
            ConsumeAction::Done => {
                chars.next(); // consume the closing curly
                None
            }
        }
    }

    fn consume_until_garbage_ends(chars: &mut std::iter::Peekable<std::str::Chars>) -> u32{
        let mut amount = 0;
        loop {
            match chars.peek() {
                Some(&'!') => {
                    chars.next(); // consume the exclamation point
                    chars.next(); // consume the following character, which was "cancelled"
                }
                Some(&'>') => {
                    chars.next(); // consume the end of garbage character
                    return amount;
                }
                Some(_) => {
                    amount += 1;
                    chars.next(); // consume the garbage
                }
                None => panic!("wtf - the garbage never ends"),
            }
        }
    }

    fn count_garbage_characters(&self) -> u32 {
        match self {
            &Thing::Group(ref children) => {
                return children.iter().fold(0, |acc, ref child| {
                        acc + child.count_garbage_characters()
                    })
            }
            &Thing::Garbage(amount) => return amount,
        }
    }
}

fn count_garbage_characters(input: &str) -> u32 {
    Thing::from_input(input).count_garbage_characters()
}
