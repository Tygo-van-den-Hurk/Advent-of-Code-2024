// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ //

/// The main function that executes both parts and prints the output the functions computed given the provided input
/// file.
fn main() {
    const INPUT: &str = include_str!("../doc/day14.txt");
    println!("The solution to part 1 is: \"{}\".", part1::solve(INPUT));
    println!("The solution to part 2 is: \"{}\".", part2::solve(INPUT));
}


// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ Part 1 out of 2 ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ //


/// The part that solves the first part of the puzzle.
mod part1 {

    use std::str::FromStr;
    use std::ops::Add;
    use std::num::ParseIntError;

    /// Custom error type
    #[derive(Debug)]
    pub enum ParseError {
        InvalidFormat,
        ParseIntError(ParseIntError),
    } impl From<ParseIntError> for ParseError {
        fn from(err: ParseIntError) -> Self {
            ParseError::ParseIntError(err)
        }
    }

    /// The dimensions of the bathroom that the robots guard.
    #[cfg(test)]
    const BATHROOM_DIMENSIONS: Dimensions = Dimensions { width: 11, height: 7 };
    
    /// The dimensions of the bathroom that the robots guard.
    #[cfg(not(test))]
    const BATHROOM_DIMENSIONS: Dimensions = Dimensions { width: 101, height: 103 };

    /// The amount of seconds to simulate.
    // #[cfg(test)]
    const SIMULATION_DURATION: usize = 100;

    // #[cfg(not(test))]
    // const SIMULATION_DURATION: usize = 100;

    /// An abstraction for a point in 2D space.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
    pub struct Dimensions { 
        /// The width of something
        width: i32, 
        /// The height of something
        height: i32 
    }
    
    //` - - - - - struct Point - - - - -

    /// An abstraction for a point in 2D space.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
    pub struct Point { 
        /// The x-coordinate of the point
        x: i32, 
        /// The y-coordinate of the point
        y: i32 
    
    } impl Add<Velocity> for Point { type Output = Point;
    
        fn add(self, velocity: Velocity) -> Point {
            Point {
                x: self.x + velocity.x,
                y: self.y + velocity.y,
            }
        }

    } impl FromStr for Point { 
        type Err = ParseError;
        
        fn from_str(input: &str) -> Result<Self, Self::Err> {
            let parts: Vec<&str> = input.split(',').collect();
            
            if parts.len() != 2 { return Err(ParseError::InvalidFormat); }
    
            let x = parts[0].parse::<i32>()?;
            let y = parts[1].parse::<i32>()?;
    
            Ok(Point { x, y })
        }
    }
    
    //` - - - - - struct Velocity - - - - -

    /// An abstraction for a point in 2D space.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
    pub struct Velocity { x: i32, y: i32 } impl FromStr for Velocity { 
        type Err = ParseError;
        
        fn from_str(input: &str) -> Result<Self, Self::Err> {
            let parts: Vec<&str> = input.split(',').collect();
            
            if parts.len() != 2 { return Err(ParseError::InvalidFormat); }
    
            let x = parts[0].parse::<i32>()?;
            let y = parts[1].parse::<i32>()?;
    
            Ok(Velocity { x, y })
        }
    }
    
    //` - - - - - struct Robot - - - - -

    /// An abstraction for the robots that guard the bathroom.
    #[derive(Debug, Clone, Hash)]
    pub struct Robot {
        /// The position the robot has in 2D space.
        position: Point,  
        /// The velocity the robot has.
        velocity: Velocity,

    } impl Robot {
        
        fn simulate_movement(&mut self) {
            
            let mut new_position = self.position + self.velocity;

            if new_position.x < 0 { 
                new_position.x = BATHROOM_DIMENSIONS.width + new_position.x; 
            } if new_position.x >= BATHROOM_DIMENSIONS.width  { 
                new_position.x = -1 * (BATHROOM_DIMENSIONS.width - new_position.x); 
            } 
            
            if new_position.y < 0 { 
                new_position.y = BATHROOM_DIMENSIONS.height + new_position.y; 
            } if new_position.y >= BATHROOM_DIMENSIONS.height { 
                new_position.y = -1 * (BATHROOM_DIMENSIONS.height - new_position.y); 
            }
            self.position = new_position;
        }
    
    } impl FromStr for Robot { 
        type Err = ParseError;

        fn from_str(input_string: &str) -> Result<Self, Self::Err> {

            let (position_string, velocity_string) = input_string.split_once(" ")
                .ok_or(ParseError::InvalidFormat)?;
            
            let position_string = position_string.split_once("=")
                .ok_or(ParseError::InvalidFormat)?;
            let position = Point::from_str(position_string.1)?;

            let velocity_string = velocity_string.split_once("=")
                .ok_or(ParseError::InvalidFormat)?;
            let velocity = Velocity::from_str(velocity_string.1)?;
    
            Ok(Robot { position, velocity })
        }
        
    }
    
    //` - - - - - fn solve - - - - -

    /// The function that solves the puzzle.
    pub fn solve(input: &str) -> usize { 

        let mut robots = Vec::new();
        for line in input.lines() { 
            let robot = Robot::from_str(line).unwrap();
            robots.push(robot);
        }

        // let mut bathroom = vec![vec![0; bathroom_dimensions.width as usize]; bathroom_dimensions.height as usize];
        for _ in 0..SIMULATION_DURATION {
            for robot in robots.iter_mut() {
                robot.simulate_movement();
            }
        }

        let mut quartiles = vec![0; 4];
        let left_half  = 0..BATHROOM_DIMENSIONS.width/2;
        let right_half = (BATHROOM_DIMENSIONS.width/2)+1..BATHROOM_DIMENSIONS.width;
        let upper_half = 0..BATHROOM_DIMENSIONS.height/2;
        let lower_half = (BATHROOM_DIMENSIONS.height/2)+1..BATHROOM_DIMENSIONS.height;
        for robot in robots {
            if upper_half.contains(&robot.position.y) && left_half.contains(&robot.position.x) {
                quartiles[0] += 1;
            } else if upper_half.contains(&robot.position.y) && right_half.contains(&robot.position.x) {
                quartiles[1] += 1;
            } else if lower_half.contains(&robot.position.y) && right_half.contains(&robot.position.x) {
                quartiles[2] += 1;
            } else if lower_half.contains(&robot.position.y) && left_half.contains(&robot.position.x) {
                quartiles[3] += 1;
            }
        }

        let mut result = 1;
        for quartile in quartiles { result *= quartile }

        return result;
    }

}


// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ Part 2 out of 2 ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ //


/// The part that solves the first part of the puzzle.
mod part2 {

    /// The function that solves the puzzle.
    pub fn solve(input: &str) -> usize { input.len() }

}


// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ Tests ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ //


/// Tests both part solutions for correctness.
#[cfg(test)]
mod tests {
    use super::*;

    // The puzzles test input
    const INPUT: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    #[test]
    fn part1() {
       let expected = 12;
       let result = part1::solve(INPUT);
       assert_eq!(expected, result, "Expected {expected}, but got {result} instead.")
    }
}


// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ //
