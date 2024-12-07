// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ //


fn main() {
    const INPUT: &str = include_str!("../doc/day7.txt");
    println!("The solution to part 1 is: \"{}\".", part1::solve(INPUT));
    println!("The solution to part 2 is: \"{}\".", part2::solve(INPUT));
}


// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ Part 1 out of 2 ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ //


mod part1 {

    use std::collections::HashSet;

    #[derive(Debug, Clone)]
    struct Equation { total: usize, terms: Vec<usize> } impl Equation {
        
        fn is_possible(&self) -> bool { 
            
            fn recurse(terms: &Vec<usize>) -> HashSet<usize> {
                
                if terms.len() == 1 { return HashSet::from_iter(terms.iter().cloned()); }

                let mut clone = terms.clone();
                let first  = clone.remove(0);
                let second = clone.remove(0);
                
                let mut copy1 = clone.clone();
                copy1.insert(0, first + second);
                let set1 = recurse(&copy1);

                let mut copy2 = clone.clone();
                copy2.insert(0, first * second);
                let set2 = recurse(&copy2);

                let mut result = HashSet::new();
                result.extend(set1);
                result.extend(set2);

                return result;
            }

            let recursion_result = recurse(&self.terms);
            return recursion_result.contains(&self.total);
        }
    }

    pub fn solve(input: &str) -> usize { 

        let all_equations: Vec<Equation> = input.lines()
            .map( | line | {
                let (left_side, right_side) = line.split_once(":").unwrap();
                let total = left_side.parse::<usize>().unwrap();
                let terms = right_side.split(" ")
                    .filter( |string_slice| !string_slice.is_empty() )
                    .map(|element| { element.parse::<usize>().unwrap() } )
                    .collect();
                let equation = Equation { total, terms };
                return equation; 
            }).collect();

        let possible_equations: Vec<&Equation> = all_equations.iter()
            .filter(| equation | equation.is_possible() )
            .collect();

        return possible_equations.iter().fold(0, |total, equation| total + equation.total );
    }
}


// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ Part 2 out of 2 ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ //


mod part2 {

    use std::collections::HashSet;

    #[derive(Debug, Clone)]
    struct Equation { total: usize, terms: Vec<usize> } impl Equation {
        
        fn is_possible(&self) -> bool { 
            
            fn recurse(terms: &Vec<usize>) -> HashSet<usize> {
                
                if terms.len() == 1 { return HashSet::from_iter(terms.iter().cloned()); }

                let mut clone = terms.clone();
                let first  = clone.remove(0);
                let second = clone.remove(0);
                
                let mut copy1 = clone.clone();
                copy1.insert(0, first + second);
                let set1 = recurse(&copy1);

                let mut copy2 = clone.clone();
                copy2.insert(0, first * second);
                let set2 = recurse(&copy2);

                let mut copy3 = clone.clone();
                let mut first_number_as_string = first.to_string();
                first_number_as_string.push_str(&second.to_string());
                let combined_number = first_number_as_string.parse::<usize>().unwrap();
                copy3.insert(0, combined_number);
                let set3 = recurse(&copy3);

                let mut result = HashSet::new();
                result.extend(set1);
                result.extend(set2);
                result.extend(set3);

                return result;
            }

            let recursion_result = recurse(&self.terms);
            return recursion_result.contains(&self.total);
        }
    }

    pub fn solve(input: &str) -> usize { 

        let all_equations: Vec<Equation> = input.lines()
            .map( | line | {
                let (left_side, right_side) = line.split_once(":").unwrap();
                let total = left_side.parse::<usize>().unwrap();
                let terms = right_side.split(" ")
                    .filter( |string_slice| !string_slice.is_empty() )
                    .map(|element| { element.parse::<usize>().unwrap() } )
                    .collect();
                let equation = Equation { total, terms };
                return equation; 
            }).collect();

        let possible_equations: Vec<&Equation> = all_equations.iter()
            .filter(| equation | equation.is_possible() )
            .collect();

        return possible_equations.iter().fold(0, |total, equation| total + equation.total );
    }

}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ Tests ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ //


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1::solve("190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"), 3749);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2::solve("190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"), 11387);
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ //
