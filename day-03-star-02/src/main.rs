fn main() {
    println!("{}", first_value_larger_than(368078));
}

type Coordinate = (i64, i64);
type Square = i64;

#[derive(Debug, Clone)]
enum Direction {
    Right,
    Up,
    Left,
    Down,
}

type Breadcrumb = (Square, Coordinate);

#[derive(Debug)]
struct Spiral;

impl Spiral {
    fn new() -> Spiral {
        Spiral
    }

    fn move_once(&self, mut breadcrumb: Breadcrumb, direction: &Direction) -> Breadcrumb {
        println!("Moving in direction: {:?}", direction);
        match direction {
            &Direction::Right => (breadcrumb.1).0 += 1,
            &Direction::Up => (breadcrumb.1).1 += 1,
            &Direction::Left => (breadcrumb.1).0 -= 1,
            &Direction::Down => (breadcrumb.1).1 -= 1,
        }

        breadcrumb
    }

    fn first_value_larger_than(&self, cutoff: i64) -> i64 {
        let mut trail: Vec<Breadcrumb> = vec![(1, (1, 1))];
        let mut direction = Direction::Up;
        let mut current_square = 1;

        loop {
            println!("Trail: {:?}", trail);

            for _side in 1..3 {
                println!("Current square: {}", current_square);
                for step in 1..current_square {
                    let next_step_direction = if step == 1 {
                        match direction {
                            Direction::Right => Direction::Down,
                            Direction::Up => Direction::Right,
                            Direction::Left => Direction::Up,
                            Direction::Down => Direction::Left,
                        }
                    } else {
                        direction.clone()
                    };
                    let mut next_position = self.move_once(trail.last().unwrap().clone(), &next_step_direction);


                    next_position.0 += 1; // TODO: be smart here
                    if next_position.0 > cutoff {
                        return next_position.0;
                    }

                    trail.push(next_position)
                }

                direction = match direction {
                    Direction::Right => Direction::Up,
                    Direction::Up => Direction::Left,
                    Direction::Left => Direction::Down,
                    Direction::Down => Direction::Right,
                };
            }

            current_square += 1;
        }
    }



















    // when no more moves for this side and no more sides for this square
    //      step
    //      turn
    //      step i-1 times
    //      turn
    //      one less side for this square
    //
    // when no more moves and yes more sides for this square
    //      step i times
    //      turn
    //      one less side for this square

    // fn next(&mut self) -> i64 {
    //     // println!("Current state: {:?}", self);

    //     let mut next_square = self.squares.last().unwrap().clone();
    //     println!("Last square: {:?}", next_square.1);

    //     // TODO: make it go up by a dynamic amount depending on neighbor
    //     next_square.0 += 1;

    //     if self.remaining_sides_in_this_square == 0 && self.remaining_steps_in_this_direction == 0 {
    //         println!("I've completed square {}, time to start the next one", self.current_square);
    //         self.move_once(&mut next_square);
    //         self.turn();
    //         self.remaining_sides_in_this_square = 2;
    //         self.current_square += 1;
    //         self.remaining_steps_in_this_direction = self.current_square - 1;
    //     } else if self.remaining_sides_in_this_square > 0 && self.remaining_steps_in_this_direction > 0 {
    //         println!("I am working my way through square {}", self.current_square);
    //         self.move_once(&mut next_square);
    //         self.remaining_steps_in_this_direction -= 1;
    //     } else if self.remaining_sides_in_this_square > 0 && self.remaining_steps_in_this_direction == 0 {
    //         println!("I've reached the end of a side but I have more sides to go in square {}", self.current_square);
    //         self.remaining_sides_in_this_square -= 1;
    //         self.turn();
    //         self.move_once(&mut next_square);
    //         self.remaining_steps_in_this_direction = self.current_square - 1;
    //     } else if self.remaining_sides_in_this_square == 0 && self.remaining_steps_in_this_direction > 0 {
    //         panic!("now what? {:?}", self);
    //     } else {
    //         panic!("why? {:?}", self);
    //     }

    //     self.squares.push(next_square);

    //     next_square.0
    // }

    // fn turn(&mut self) {
    //     self.direction = match self.direction {
    //         Direction::Right => Direction::Up,
    //         Direction::Up => Direction::Left,
    //         Direction::Left => Direction::Down,
    //         Direction::Down => Direction::Right,
    //     }
    // }

    // fn move_once(&self, crumb: &mut Breadcrumb) {
    //     match self.direction {
    //         Direction::Right => (crumb.1).0 += 1,
    //         Direction::Up => (crumb.1).1 += 1,
    //         Direction::Left => (crumb.1).0 -= 1,
    //         Direction::Down => (crumb.1).1 -= 1,
    //     }
    // }
}

fn first_value_larger_than(cutoff: i64) -> i64 {
    let spiral = Spiral::new();
    spiral.first_value_larger_than(cutoff)
}

#[test]
fn example_one() {
    assert_eq!(first_value_larger_than(23), 25)
}
