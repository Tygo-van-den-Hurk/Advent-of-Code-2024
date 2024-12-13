// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ //

/// The main function that executes both parts and prints the output the functions computed given the provided input
/// file.
fn main() {
    const INPUT: &str = include_str!("../doc/day13.txt");
    println!("The solution to part 1 is: \"{}\".", part1::solve(INPUT));
    println!("The solution to part 2 is: \"{}\".", part2::solve(INPUT));
}


// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ Part 1 out of 2 ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ //


/// The part that solves the first part of the puzzle.
mod part1 {

    /// An abstraction for a point in 2D space.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
    struct Point { x: usize, y: usize }
    
    /// An abstraction for a button.
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    struct Button { 
        /// the amount that activating this button adds to the x value of the claw.
        moves_x: usize,
        /// the amount that activating this button adds to the y value of the claw.
        moves_y: usize,
    }

    /// An abstraction for the claw machine. Has a `Point` onto which the price is, and two botton that move the claw 
    /// (which location is stored as a `Point`as well) a certain amount 
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    struct ClawMachine {

        /// The configuration for button A.
        a_button: Button,
        /// The configuration for button B.
        b_button: Button,
        /// The `Point` where the price is located.
        prize_loc: Point,
    
    } impl ClawMachine {
        
        /// Basically solves a linear system. <br>
        /// - See: [Cramer's rule](https://en.wikipedia.org/wiki/Cramer%27s_rule)
        fn solution(&self) -> Option<usize> {

            let a1 = self.a_button.moves_x as i64;
            let a2 = self.a_button.moves_y as i64;
            let b1 = self.b_button.moves_x as i64;
            let b2 = self.b_button.moves_y as i64;
            let c1 = self.prize_loc.x as i64;
            let c2 = self.prize_loc.y as i64;

            let x;
            /* Calculating x */ {
                let numerator = c1*b2 - b1*c2;
                let denominator = a1*b2 - b1*a2;
                if denominator == 0 { return None }
                if numerator % denominator != 0 { return None }
                x = numerator / denominator;
                if x < 0 { return None; }
            }
            
            let y;
            /* Calculating y */ {
                let numerator = a1*c2 - c1*a2;
                let denominator = a1*b2 - b1*a2;
                if denominator == 0 { return None }
                if numerator % denominator != 0 { return None }
                y = numerator / denominator;
                if y < 0 { return None; }
            }

            assert_eq!(a1*x + b1*y, c1, "The function is not implemented correctly.");
            assert_eq!(a2*x + b2*y, c2, "The function is not implemented correctly.");

            let result = ( x*3 + y ) as usize;
            return Some(result);
        }
    }

    /// Parses the input, but panics if it is malformed.
    fn parse(input: &str) -> Vec<ClawMachine> {
        
        const PANIC_MESSAGE: &str = "Malformed input: next machine configuration was only half completed.";
        
        let mut machines = Vec::new();
        let mut lines = input.lines();
        loop {

            let a_button; 
            /* first line (A button configuration) */ {
                if let Some(line) = lines.next() {
                    println!("case 1: {line}");
                    let (x_half, y_half) = line.split_once(":").unwrap().1.split_once(",").unwrap();
                    let moves_x = x_half.split_once("+").unwrap().1.parse::<usize>().unwrap();
                    let moves_y = y_half.split_once("+").unwrap().1.parse::<usize>().unwrap();
                    a_button = Button { moves_x, moves_y };
                } else { return machines; }
            }

            let b_button; 
            /* second line (B button configuration) */ {
                if let Some(line) = lines.next() {
                    println!("case 2: {line}");
                    let (x_half, y_half) = line.split_once(":").unwrap().1.split_once(",").unwrap();
                    let moves_x = x_half.split_once("+").unwrap().1.parse::<usize>().unwrap();
                    let moves_y = y_half.split_once("+").unwrap().1.parse::<usize>().unwrap();
                    b_button = Button { moves_x, moves_y };
                } else { panic!(
                    "{PANIC_MESSAGE} The A button was configured, but the B button and prize location are missing."
                ) }
            }

            let claw_machine; 
            /* third line (price location) */ {
                if let Some(line) = lines.next() {
                    println!("case 3: {line}");
                    let (x_half, y_half) = line.split_once(":").unwrap().1.split_once(",").unwrap();
                    let prize_x = x_half.split_once("=").unwrap().1.parse::<usize>().unwrap();
                    let prize_y = y_half.split_once("=").unwrap().1.parse::<usize>().unwrap();
                    let prize_loc = Point { x: prize_x, y: prize_y };
                    claw_machine = ClawMachine { a_button, b_button, prize_loc, };
                    machines.push(claw_machine);
                } else { panic!(
                    "{PANIC_MESSAGE} The A & B buttons were configured but the prize location was missing"
                ) }    
            }
            
            /* forth line (empty line, and optional) */ {
                if let Some(line) = lines.next() {
                    println!("case 4: {line}");
                    if ! line.is_empty() { panic!(
                        "Malformed input: expected empty line after complete configuration."
                    )}
                } else { return machines; }
            }
        }
        
    }

    /// The function that solves the puzzle.
    pub fn solve(input: &str) -> usize { 

        let mut total = 0;
        /* getting the minimal cost for a possible machine, and adding that to the total */ {
            let machines = parse(input);
            for machine in machines.into_iter() {
                if let Some(minimal_cost) = machine.solution() { 
                    total += minimal_cost;
                }
            }
        }

        return total;
    }
}


// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ Part 2 out of 2 ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ //


/// The part that solves the first part of the puzzle.
mod part2 {

    /// The function that solves the puzzle 
    pub fn solve(input: &str) -> usize { input.len() }

}


// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ Tests ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ //


/// Tests both part solutions for correctness.
#[cfg(test)]
mod tests {
    use super::*;

    // The puzzles test input
    const INPUT: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    #[test]
    fn part1() {
       let expected = 480;
       let result = part1::solve(INPUT);
       assert_eq!(expected, result, "Expected {expected}, but got {result} instead.")
    }
}


// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ //
