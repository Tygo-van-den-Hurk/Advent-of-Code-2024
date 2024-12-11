// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ //


fn main() {
    const INPUT: &str = include_str!("../doc/day11.txt");
    println!("The solution to part 1 is: \"{}\".", part1::solve(INPUT));
    println!("The solution to part 2 is: \"{}\".", part2::solve(INPUT));
}


// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ Part 1 out of 2 ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ //


mod part1 {

    pub fn solve(input: &str) -> usize { input.len() }

}


// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ Part 2 out of 2 ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ //


mod part2 {
    
    pub fn solve(input: &str) -> usize { input.len() }

}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ Tests ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ //


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_simple() {
        assert_eq!(part1::solve(""), 1);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1::solve(""), 36);
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ //
