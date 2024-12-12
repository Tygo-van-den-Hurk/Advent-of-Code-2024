fn main() {
    const INPUT: &str = include_str!("../doc/day05.txt");
    // println!("The solution to part 1 is: \"{}\".", part1(INPUT));
    println!("The solution to part 2 is: \"{}\".", part2::solve(INPUT));
}

//` Rule

use regex::Regex;

struct Rule {
    first_page: i32,
    later_page: i32,
    regex: Regex
}

impl std::fmt::Debug for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{ first_page: {}, later_page: {}, regex: {} }}", 
            self.first_page, 
            self.later_page, 
            self.regex.as_str()
        )
    }
}

//` Part 1

pub fn part1(input: &str) -> i32 { 
    
    let splitted_input: Vec<&str> = input.split("\n\n").collect();
    let rules_input: Vec<&str> = splitted_input[0].lines().collect();
    let pages_input: Vec<&str> = splitted_input[1].lines().collect();
    
    let rules: Vec<Rule>;
    /* Building the list of regex rules. */ {
        let mut rules_so_far: Vec<Rule> = Vec::new();
        for rule in rules_input {
            let rule_split: Vec<&str> = rule.split("|").collect();
            let first_page = rule_split[0].parse::<i32>().unwrap();
            let later_page = rule_split[1].parse::<i32>().unwrap();
            let regex: Regex = Regex::new(&format!(r"{first_page},(\d*,)*{later_page}")).unwrap();
            let rule: Rule = Rule { first_page, later_page, regex };
            rules_so_far.push(rule);
        }
        rules = rules_so_far;
    }

    println!("Rules from the input: {:?}\n", rules);

    let matching_lines: Vec<&str>;
    /* Checking all regex rules for each  */ {
        let mut safe_pages_found_so_far: Vec<&str> = Vec::new();
        for pages in pages_input {
            let mut violates = false;
            for rule in &rules {
                println!("\tTesting rule: {:?} on: {pages}.", rule);
                
                let first_page_as_str:   &str = &rule.first_page.to_string();
                let first_page_as_regex: Regex = Regex::new(&format!(
                    "^{first_page_as_str},|,{first_page_as_str},|,{first_page_as_str}$")).unwrap();
                if ! first_page_as_regex.is_match(pages) { continue; }
                
                println!("\t\t{pages} contains {first_page_as_str}.");

                let later_page_as_str:   &str = &rule.later_page.to_string();
                let later_page_as_regex: Regex = Regex::new(&format!(
                    "^{later_page_as_str},|,{later_page_as_str},|,{later_page_as_str}$")).unwrap();
                if ! later_page_as_regex.is_match(pages) { continue; }

                println!("\t\t{pages} contains {later_page_as_str}.");

                if ! rule.regex.is_match(pages) { violates = true; } 
            }
            if violates { continue; }
            safe_pages_found_so_far.push(pages);
        }
        matching_lines = safe_pages_found_so_far;
    }

    println!("Lines that match the rules: {:?}\n", matching_lines);

    let result: i32;
    /* Getting the middle page numbers and summing them up */ {
        let mut total = 0;
        for matching_line in matching_lines {
            let pages: Vec<&str> = matching_line.split(",").collect();
            let middle_page: i32 = pages[pages.len() / 2].parse::<i32>().unwrap();
            total += middle_page;
        }
        result = total;
    }

    println!("The total of their middle pages: {result}");

    return result;
}

//` Part 2

pub mod part2 {

    use std::{cmp::Ordering, collections::HashSet, str::FromStr, usize};

    /// A simple abstraction of a Rule. A rule dictates if both pages exist in a collection, which of them should come
    /// first.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    struct Rule { first_page: Page, later_page: Page, }

    /// A simple abstraction of a Page. A page has a number by which it is sorted.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    struct Page { number: usize } impl Page {
        
        fn goes_before(&self, other: &Self, rules: &HashSet<Rule>) -> Ordering {
            if rules.contains(&Rule { first_page: self.clone(), later_page: other.clone() }) {
                Ordering::Less
            } else if rules.contains(&Rule { first_page: other.clone(), later_page: self.clone() }) {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        }

    } impl FromStr for Page { type Err = std::num::ParseIntError;
    
        fn from_str(string: &str) -> Result<Self, Self::Err> {
            let number = string.parse::<usize>()?;
            return Ok(Page { number });
        }
    }

    /// The function that solves the puzzle 
    pub fn solve(input: &str) -> usize { 

        let mut lines = input.lines();

        let mut rules: HashSet<Rule> = HashSet::new();
        /* Creating the rules */ {
            for line in &mut lines {
                if line.is_empty() { break; }
                let (first_page, later_page) = line.split_once("|").unwrap();
                let first_page = first_page.parse().unwrap();
                let later_page = later_page.parse().unwrap();
                rules.insert(Rule { first_page, later_page });
            }
        }

        let mut manual: Vec<Vec<Page>> = Vec::new();
        /* Creating the manual pages */ {
            for line in lines { 
                let pages = line.split(",")
                    .map(|page| page.parse::<Page>().unwrap() )
                    .collect();
                manual.push(pages);
            }
        }
    
        let broken_rules: Vec<Vec<Page>>;
        /* filtering for all the rules that are broken */ {
            broken_rules = manual.into_iter().filter( | pages | {
                for (index, first_page) in pages.iter().enumerate() {
                    for later_index in index..pages.len() {
                        let later_page = pages[later_index];
                        if rules.contains(&Rule { first_page: later_page, later_page: *first_page }) {
                            return true;
                        }
                    }
                }
                return false;
            }).collect();
        }

        return broken_rules.into_iter()
            .map(|mut pages| {
                pages.sort_by(|book1, book2| book1.goes_before(book2, &rules));
                return pages[pages.len() / 2].number
            }).sum();
    }
}

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
97,13,75,29,47"), 143);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2::solve("47|53
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
97,13,75,29,47"), 123);
    }
}
