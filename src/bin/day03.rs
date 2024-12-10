use regex::Regex;

const INPUT: &str = include_str!("../doc/day03.txt");

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

pub fn part2(input: &str) -> i32 { 

    let mut total = 0;

    let regular_expression: Regex;
    /* Building the Regular Expression */ {
        let do_regular_expression = r"do\(\)";
        let do_not_regular_expression = r"don't\(\)";
        let mul_regular_expression = r"mul\((\d*,\d*)\)";
        regular_expression = Regex::new(&format!("{}|{}|{}", 
            do_regular_expression, 
            do_not_regular_expression, 
            mul_regular_expression
        )).unwrap();
    }
     
    /* Processing the input */ {
        let mut summing_allowed = true;
        for matching_instance in regular_expression.find_iter(&input) {
            
            let mut captured_substring: &str = matching_instance.as_str();
            println!("Captured string: {captured_substring} while summing_allowed is {summing_allowed}.");
            if ! summing_allowed && captured_substring != "do()" { continue; }
            if captured_substring == "do()"    { summing_allowed = true;  continue; }
            if captured_substring == "don't()" { summing_allowed = false; continue; }
            
            captured_substring = &captured_substring[4..captured_substring.len()-1];
            println!("\t- Substring of Captured string: {captured_substring}");
            let split_captured_substring: Vec<&str> = captured_substring.split(",").collect();
            let left = split_captured_substring[0].parse::<i32>().unwrap();
            let right = split_captured_substring[1].parse::<i32>().unwrap();
            total += left * right;
        }
    }

    return total;
}

//` Test

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"), 161);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"), 48);
    }
}

