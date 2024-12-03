const INPUT: &str = include_str!("../doc/day3.txt");

fn main() {
    
    /* Solving Part 1 based on the input */ {
        let result: i32 = part1(INPUT);
        println!("The solution to part 1 is: \"{result}\".");
    }
        
    /* Solving Part 1 based on the input */ {
        let result: i32 = part2(INPUT);
        println!("The solution to part 2 is: \"{result}\".");
    }
}

//` Part 1

pub fn part1(input: &str) -> i32 { 0 }

//` Part 2

pub fn part2(input: &str) -> i32 { 0 }

//` Test

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"), 161);
    }
}

