fn main() {
    println!("{}", count_steps(368078));
}

fn coordinate_for(square: i32) -> Coordinate {
    if square == 1 {
        return (0, 0);
    }

    println!("Finding coordinate for square {}", square);
    for i in 0.. {
        let mut tracking_square = i * i;
        if tracking_square >= square {
            if i % 2 == 0 {
                // top left
                let mut coordinate = ((-(i / 2) + 1), i / 2);

                for _ in 1..i {
                    if tracking_square == square {
                        return coordinate;
                    }

                    coordinate.0 += 1;
                    tracking_square -= 1;

                    if tracking_square == square {
                        return coordinate;
                    }
                }

                for _ in 1..i {
                    if tracking_square == square {
                        return coordinate;
                    }

                    coordinate.1 -= 1;
                    tracking_square -= 1;

                    if tracking_square == square {
                        return coordinate;
                    }
                }
            } else {
                // bottom right
                let mut coordinate = (((i - 1) / 2), -(i - 1) / 2);

                for _ in 1..i {
                    if tracking_square == square {
                        return coordinate;
                    }

                    coordinate.0 -= 1;
                    tracking_square -= 1;

                    if tracking_square == square {
                        return coordinate;
                    }
                }

                for _ in 1..i {
                    if tracking_square == square {
                        return coordinate;
                    }

                    coordinate.1 += 1;
                    tracking_square -= 1;

                    if tracking_square == square {
                        return coordinate;
                    }
                }
            }
        }
    }

    panic!("omg");
}

fn count_steps(square: i32) -> i32 {
    manhattan_distance(&coordinate_for(square), &coordinate_for(1))
}

#[test]
fn example_one() {
    assert_eq!(count_steps(1), 0)
}

#[test]
fn example_two() {
    assert_eq!(count_steps(12), 3)
}

#[test]
fn example_three() {
    assert_eq!(count_steps(23), 2)
}

#[test]
fn example_four() {
    assert_eq!(count_steps(1024), 31)
}

type Coordinate = (i32, i32);

fn manhattan_distance(starting_position: &Coordinate, destination_position: &Coordinate) -> i32 {
    (starting_position.0 - destination_position.0).abs() +
        (starting_position.1 - destination_position.1).abs()
}

#[test]
fn test_manhattan_distance() {
    assert_eq!(manhattan_distance(&(1, 1), &(1, 1)), 0);
    assert_eq!(manhattan_distance(&(3, 1), &(1, 1)), 2);
    assert_eq!(manhattan_distance(&(3, 0), &(1, 1)), 3);
    assert_eq!(manhattan_distance(&(-3, 0), &(1, 1)), 5);
}
