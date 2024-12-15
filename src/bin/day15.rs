// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ //

/// The main function that executes both parts and prints the output the functions computed given the provided input
/// file.
fn main() {
    const INPUT: &str = include_str!("../doc/day15.txt");
    println!("The solution to part 1 is: \"{}\".", part1::solve(INPUT));
    println!("The solution to part 2 is: \"{}\".", part2::solve(INPUT));
}


// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ Part 1 out of 2 ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ //


/// The part that solves the first part of the puzzle.
mod part1 {

    use std::fmt;
    use std::ops::Add;
    use std::collections::VecDeque;

    //` - - - - - struct Tile - - - - -

    /// An abstraction for any tile on the map
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum Tile { Wall, Robot, Crate, Empty, } impl Tile {
        
        /// Returns a `Tile` based on the `char` provided. Panics on unrecognised `char`.
        pub fn from_char(character: char) -> Tile {
            return match character {
                '#' => Tile::Wall,
                'O' => Tile::Crate,
                '.' => Tile::Empty,
                '@' => Tile::Robot,
                symbol => panic!(
                    "Unknown type of tile in input data. Expected: '#', 'O', '.', or '@', but found {symbol}."
                )
            };
        }
    
    } impl<'a> fmt::Display for Tile {

        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Tile::Wall  => write!(f, "{}", '#'),
                Tile::Crate => write!(f, "{}", 'O'),
                Tile::Empty => write!(f, "{}", '.'),
                Tile::Robot => write!(f, "{}", '@'),
            }
        }
    }

    //` - - - - - struct Instruction - - - - -

    /// An abstraction for the direction the robot can go into
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum Instruction { Up, Right, Down, Left, } impl Instruction {
        
        /// Returns a `Tile` based on the `char` provided. Panics on unrecognised `char`.
        pub fn from_char(character: char) -> Instruction {
            return match character {
                '^' => Instruction::Up,
                '>' => Instruction::Right,
                'V' | 'v' => Instruction::Down,
                '<' => Instruction::Left,
                symbol => panic!(
                    "Unknown type of instruction in input data. Expected: '^', '>', 'V', 'v', or '<' but found '{}'",
                    symbol
                )
            };
        }

    } impl<'a> fmt::Display for Instruction {
        
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Instruction::Up    => write!(f, "{}", '^'),
                Instruction::Right => write!(f, "{}", '>'),
                Instruction::Down  => write!(f, "{}", 'V'),
                Instruction::Left  => write!(f, "{}", '<'),
            }
        }
    }

    //` - - - - - struct Point - - - - -

    /// An abstraction for a point in 2D space.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
    pub struct Point { 
        
        /// The x-coordinate of the point
        pub x: usize, 
        
        /// The y-coordinate of the point
        pub y: usize 

    } impl Add<Instruction> for Point { type Output = Point;
    
        fn add(self, instruction: Instruction) -> Point {
            return match instruction {
                Instruction::Up    => Point { x: self.x,     y: self.y - 1 },
                Instruction::Right => Point { x: self.x + 1, y: self.y     },
                Instruction::Down  => Point { x: self.x,     y: self.y + 1 },
                Instruction::Left  => Point { x: self.x - 1, y: self.y     },
            };
        }

    } impl<'a> fmt::Display for Point {
        
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }

    //` - - - - - fn Solve - - - - -

    /// The function that solves the puzzle.
    pub fn solve(input: &str) -> usize { 
        
        let mut lines = input.lines().enumerate();
        
        let mut robot_position = None;
        let mut map = Vec::new();
        /* Parsing the input data and creating the map from input data */ {
            for (y, line) in &mut lines {
                if line.is_empty() { break; }
                
                let mut row = Vec::new();
                for (x, character) in line.chars().enumerate() {
                    let tile = Tile::from_char(character);
                    row.push(tile);

                    if tile != Tile::Robot { continue; }
                    if robot_position.is_some() { panic!("There be at most 1 robot in input data, found more.") } 
                    
                    robot_position = Some(Point {x, y})
                }

                map.push(row);
            }
        }
    
        // Checking integrity of the state we expect the data to be in
        if robot_position.is_none() { panic!("Must be at least 1 robot in input data, found 0.") }
        let mut robot_position = robot_position.unwrap();
        assert!(map[robot_position.y][robot_position.x] == Tile::Robot);

        let mut instructions = Vec::new();
        /* Parsing the input data and creating the instructions from input data */ {
            for (_, line) in lines { 
                for character in line.chars() {
                    let instruction = Instruction::from_char(character);
                    instructions.push(instruction);
                }
            }
        }

        /* Simulating the robot on the instructions */ {
            for instruction in instructions.into_iter() {
  
                let mut ran_into_wall = false;
                let mut queue = VecDeque::new();
                let mut next_point = robot_position;
                queue.push_front(next_point);
                loop {
                    
                    next_point = next_point + instruction;
                    queue.push_front(next_point);
                    
                    if map[next_point.y][next_point.x] == Tile::Empty { break; }
                    if map[next_point.y][next_point.x] == Tile::Wall { 
                        ran_into_wall = true;
                        break;
                    }
                }
                
                if ran_into_wall { continue; } // ignore instruction.

                let mut previous_point = None;
                for point in queue {
                    if previous_point.is_none() {
                        previous_point = Some(point);
                        continue;
                    }

                    map[previous_point.unwrap().y][previous_point.unwrap().x] = map[point.y][point.x];
                    previous_point = Some(point);
                }
                
                map[previous_point.unwrap().y][previous_point.unwrap().x] = Tile::Empty;
                robot_position = robot_position + instruction;
            }
        }

        let mut result = 0;
        /* Calculating the GPS values for each crate and adding them up. */ {
            for (y, row) in map.into_iter().enumerate() {
                for (x, tile) in row.into_iter().enumerate() {
                    result += match tile {
                        Tile::Empty => 0,
                        Tile::Wall  => 0,
                        Tile::Robot => 0,
                        Tile::Crate => x + 100*y,
                    };
                }
            }
        }

        return result;
    }

}


// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ Part 2 out of 2 ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ //


/// The part that solves the first part of the puzzle.
mod part2 {

    use std::ops::{Add, Sub};
    use std::fmt;
    use std::collections::VecDeque;
    use super::part1::{Instruction, Point};

    //` - - - - - struct Instruction - - - - -

    impl Instruction {

        /// Returns true if the instruction is of type `Instruction::Down`, or of type `Instruction::Up`, and false
        /// otherwise.
        fn is_vertical(&self) -> bool {
            return match self {
                Instruction::Down | Instruction::Up    => true,
                Instruction::Left | Instruction::Right => false,
            }
        }
    }

    //` - - - - - struct Point - - - - -

    impl Point {
        
        /// Gives a point that is one to the left
        fn one_to_the_left(&self)  -> Point { return self.clone() - (1, 0) }

        /// Gives a point that is one to the right
        fn one_to_the_right(&self) -> Point { return self.clone() + (1, 0) }

    } impl Add<(usize, usize)> for Point { type Output = Point;
    
        fn add(self, other: (usize, usize)) -> Point {
            return Point {
                x: self.x + other.0,
                y: self.y + other.1,
            };
        }

    } impl Sub<(usize, usize)> for Point { type Output = Point;
        
        fn sub(self, other: (usize, usize)) -> Point {
            return Point {
                x: self.x - other.0,
                y: self.y - other.1,
            };
        }

    } impl Add<Point> for Point { type Output = Point;
    
        fn add(self, other: Point) -> Point {
            return Point {
                x: self.x + other.x,
                y: self.y + other.y,
            };
        }

    } impl Sub<Point> for Point { type Output = Point;
    
        fn sub(self, other: Point) -> Point {
            return Point {
                x: self.x - other.x,
                y: self.y - other.y,
            };
        }

    } impl Sub<Instruction> for Point { type Output = Point;
        
        fn sub(self, instruction: Instruction) -> Point {
            return match instruction {
                Instruction::Up    => Point { x: self.x,     y: self.y + 1 },
                Instruction::Right => Point { x: self.x - 1, y: self.y     },
                Instruction::Down  => Point { x: self.x,     y: self.y - 1 },
                Instruction::Left  => Point { x: self.x + 1, y: self.y     },
            };
        }

    }

    //` - - - - - struct Tile - - - - -

    /// An abstraction for any tile on the map
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum Tile { Wall, Robot, LeftCrateHalf, RightCrateHalf, Empty, } impl Tile {
        
        /// Returns a `Tile` based on the `char` provided. Panics on unrecognised `char`.
        fn from_char(character: char) -> Tile {
            return match character {
                '#' => Tile::Wall,
                '[' => Tile::LeftCrateHalf,
                ']' => Tile::RightCrateHalf,
                '.' => Tile::Empty,
                '@' => Tile::Robot,
                symbol => panic!(
                    "Unknown type of tile in input data. Expected: '#', 'O', '.', or '@', but found {symbol}."
                )
            };
        }
    
    } impl<'a> fmt::Display for Tile {

        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Tile::Wall           => write!(f, "{}", '#'),
                Tile::LeftCrateHalf  => write!(f, "{}", '['),
                Tile::RightCrateHalf => write!(f, "{}", ']'),
                Tile::Empty          => write!(f, "{}", '.'),
                Tile::Robot          => write!(f, "{}", '@'),
            }
        }
    }

    //` - - - - - fn Solve - - - - -

    /// The function that solves the puzzle.
    pub fn solve(input: &str) -> usize { 
        
        let mut lines = input.lines().enumerate();
        
        let mut robot_position = None;
        let mut map = Vec::new();
        /* Parsing the input data and creating the map from input data */ {
            for (y, line) in &mut lines {
                if line.is_empty() { break; }
                
                let mut row = Vec::new();
                for (x, character) in line.chars().enumerate() {
                    match character {
                        'O' => {
                        
                            row.push(Tile::LeftCrateHalf);
                            row.push(Tile::RightCrateHalf);
                            continue;
                        
                        }, '@' => {

                            row.push(Tile::Robot);
                            row.push(Tile::Empty);
                            
                            if robot_position.is_some() { 
                                panic!("There be at most 1 robot in input data, found more.") 
                            } 

                            robot_position = Some(Point {x: x*2, y});
                            continue;
                        
                        }, character => {
                        
                            let tile = Tile::from_char(character);
                            row.push(tile);
                            row.push(tile); 

                        }
                    };
                }

                map.push(row);
            }
        }
    
        // Checking integrity of the state we expect the data to be in
        if robot_position.is_none() { panic!("Must be at least 1 robot in input data, found 0.") }
        let mut robot_position = robot_position.unwrap();
        assert!(map[robot_position.y][robot_position.x] == Tile::Robot);

        let mut instructions = Vec::new();
        /* Parsing the input data and creating the instructions from input data */ {
            for (_, line) in lines { 
                for character in line.chars() {
                    let instruction = Instruction::from_char(character);
                    instructions.push(instruction);
                }
            }
        }

        /* Simulating the instructions on the robot */ {
            for (index, instruction) in instructions.into_iter().enumerate() {

                println!();
                for row in map.iter() {
                    print!("\t");
                    for tile in row { print!("{tile}") }
                    println!()
                }
                println!();
                println!("Now executing instruction {} which is of type {instruction}", (index+1));

                let mut runs_into_a_wall = false;

                let mut processed = Vec::new();
                let mut clean_up = Vec::new();
                clean_up.push(robot_position);
                let mut queue = VecDeque::new();
                queue.push_back(robot_position);                
                while (! queue.is_empty()) && (! runs_into_a_wall) {
                    
                    let current_position = queue.pop_front().unwrap();
                    let next_position = current_position + instruction;
                    
                    println!("\t- Currently processing Point {next_position} of type: {}", map[next_position.y][next_position.x]);

                    let next_position_tile = map[next_position.y][next_position.x];
                    match next_position_tile {
                        Tile::Wall  => {
                            println!("\t  which is of type Tile::Wall");
                            runs_into_a_wall = true;
                        
                        }, Tile::Empty => {
                            println!("\t  which is of type Tile::Empty");
                            processed.push(next_position)
                        
                        }, Tile::LeftCrateHalf => {
                            println!("\t  which is of type Tile::LeftCrateHalf");
                            queue.push_back(next_position);
                            println!("\t\t- Added {next_position} of type: {} to queue", map[next_position.y][next_position.x]);
                            let neighbor = next_position.one_to_the_right();
                            if instruction.is_vertical() && !(queue.contains(&neighbor) || processed.contains(&neighbor)){
                                queue.push_back(neighbor);
                                println!("\t\t- Added {neighbor} of type: {} to queue", map[neighbor.y][neighbor.x]);
                                clean_up.push(neighbor);
                                println!("\t\t- Added {neighbor} of type: {} to clean list", map[neighbor.y][neighbor.x]);
                            }

                        }, Tile::RightCrateHalf => {
                            println!("\t  which is of type Tile::RightCrateHalf");
                            queue.push_back(next_position);
                            println!("\t\t- Added {next_position} of type: {} to queue", map[next_position.y][next_position.x]);
                            let neighbor = next_position.one_to_the_left();
                            if instruction.is_vertical() && !(queue.contains(&neighbor) || processed.contains(&neighbor)){
                                queue.push_back(neighbor);
                                println!("\t\t- Added {neighbor} of type: {} to queue", map[neighbor.y][neighbor.x]);
                                clean_up.push(neighbor);
                                println!("\t\t- Added {neighbor} of type: {} to clean list", map[neighbor.y][neighbor.x]);
                            }

                        }, Tile::Robot => (),
                    };

                    processed.push(current_position);
                }

                if runs_into_a_wall { continue; }

                println!("\t==> sorting processed based on the instruction direction");
                println!("\t    before {processed:?}");
                match instruction {
                    Instruction::Up    => processed.sort_by( |point_a, point_b| point_a.y.cmp(&point_b.y) ),
                    Instruction::Right => processed.sort_by( |point_a, point_b| point_b.x.cmp(&point_a.x) ),
                    Instruction::Down  => processed.sort_by( |point_a, point_b| point_b.y.cmp(&point_a.y) ),
                    Instruction::Left  => processed.sort_by( |point_a, point_b| point_a.x.cmp(&point_b.x) ),
                }
                println!("\t    after  {processed:?}");

                for point in processed.iter() {
                    let next_point = *point + instruction;
                    if ! processed.contains(&next_point) { continue };
                    println!(
                        "\t==> moving tile {point} '{}' into {next_point} '{}'.",
                        map[point.y][point.x],
                        map[next_point.y][next_point.x],
                    );
                    map[next_point.y][next_point.x] = map[point.y][point.x];
                    for row in map.iter() {
                        print!("\t    ");
                        for tile in row { print!("{tile}") }
                        println!()
                    }
                }

                for point in clean_up.into_iter() {
                    println!("\t==> Cleaning tile {point} to be empty");
                    map[point.y][point.x] = Tile::Empty;
                    for row in map.iter() {
                        print!("\t    ");
                        for tile in row { print!("{tile}") }
                        println!()
                    }
                }

                robot_position = robot_position + instruction;
            }
        }

        let mut result = 0;
        /* Calculating the GPS values for each crate and adding them up. */ {
            for (y, row) in map.into_iter().enumerate() {
                for (x, tile) in row.into_iter().enumerate() {
                    if tile != Tile::LeftCrateHalf { continue; }
                    result += x + 100*y;
                }
            }
        }

        return result;
    }
}


// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ Tests ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ //


/// Tests both part solutions for correctness.
#[cfg(test)]
mod tests {
    use super::*;

    // The easy puzzle test input
    const EASY_INPUT: &str = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

    // The medium puzzle test input
    const MEDIUM_INPUT: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
";

    #[test]
    fn part1_easy() {
       let expected = 2028;
       let result = part1::solve(EASY_INPUT);
       assert_eq!(expected, result, "Expected {expected}, but got {result} instead.")
    }
    
    #[test]
    fn part1_medium() {
       let expected = 10092;
       let result = part1::solve(MEDIUM_INPUT);
       assert_eq!(expected, result, "Expected {expected}, but got {result} instead.")
    }

    #[test]
    fn part2_medium() {
       let expected = 9021;
       let result = part2::solve(MEDIUM_INPUT);
       assert_eq!(expected, result, "Expected {expected}, but got {result} instead.")
    }
}


// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ //
