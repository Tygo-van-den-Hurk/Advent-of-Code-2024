const INPUT: &str = include_str!("../doc/day5.txt");

fn main() {
    
    /* Solving Part 1 based on the input */ {
        let result: String = part1(INPUT);
        println!("The solution to part 1 is: \"{result}\".");
    }
        
    /* Solving Part 1 based on the input */ {
        let result: String = part2(INPUT);
        println!("The solution to part 2 is: \"{result}\".");
    }
}

//` Part 1

pub fn part1(input: &str) -> String { "".to_string() }

//` Part 2

pub fn part2(input: &str) -> String { "".to_string() }

//` Test

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"), "75,47,61,53,29");
    }
}

