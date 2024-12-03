use regex::Regex;

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

pub fn part1(input: &str) -> i32 { 
    let mut total = 0;
    let regular_expression: Regex = Regex::new(r"mul\((\d*,\d*)\)").unwrap();
    for matching_instance in regular_expression.find_iter(&input) {
        let mut captured_substring: &str = matching_instance.as_str();
        println!("Captured string: {captured_substring}");
        captured_substring = &captured_substring[4..captured_substring.len()-1];
        println!("Substring of Captured string: {captured_substring}");
        let split_captured_substring: Vec<&str> = captured_substring.split(",").collect();
        let left = split_captured_substring[0].parse::<i32>().unwrap();
        let right = split_captured_substring[1].parse::<i32>().unwrap();
        total += left * right;
    }

    return total;
}

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

