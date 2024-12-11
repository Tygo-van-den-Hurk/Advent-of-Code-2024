// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ //


fn main() {
    const INPUT: &str = include_str!("../doc/day11.txt");
    println!("The solution to part 1 is: \"{}\".", part1::solve(INPUT));
    println!("The solution to part 2 is: \"{}\".", part2::solve(INPUT));
}


// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ Part 1 out of 2 ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ //


mod part1 {

    /**
     * An abstraction of the magic stones in the puzzle.
     */
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    struct Stone { digits: usize } impl Stone {
        
        fn can_apply_rule1(&self) -> bool { self.digits == 0 }

        fn apply_rule1(&mut self) {
            
            if ! self.can_apply_rule1() { panic!("invalid use of rule 1."); }
            
            self.digits = 1;
        }

        fn can_apply_rule2(&self) -> bool { format!("{}", self.digits).len() % 2 == 0 }

        fn apply_rule2(&mut self) -> Stone {
            
            if ! self.can_apply_rule2() { panic!("invalid use of rule 2."); }
            
            let digit_as_str = format!("{}", self.digits);
            let ( left_digits, right_digits ) = digit_as_str.split_at(digit_as_str.len() / 2);
            self.digits = left_digits.parse().unwrap();
            return Stone { digits: right_digits.parse().unwrap() };
        }

        fn apply_rule3(&mut self) {
            
            if self.can_apply_rule1() { panic!("invalid use of rule 3, rule 1 applies instead."); }
            if self.can_apply_rule2() { panic!("invalid use of rule 3, rule 2 applies instead."); }

            self.digits *= 2024;
        }
    }

    const AMOUNT_OF_BLINKS: usize = 25;

    pub fn solve(input: &str) -> usize { solve_for(input, AMOUNT_OF_BLINKS) }

    pub fn solve_for(input: &str, amount_of_blinks: usize) -> usize {

        let mut stones: Vec<Stone> = Vec::new();
        for digits in input.split(" ").into_iter() {
            let digits = digits.parse().unwrap();
            let stone = Stone { digits };
            stones.push(stone);
        }

        for _ in 0..amount_of_blinks {
            let mut new_stones = Vec::new();
            for stone in stones.iter_mut() {
            
                if stone.can_apply_rule1() {
                    stone.apply_rule1();
                    continue;
                }

                if stone.can_apply_rule2() { 
                    let new_stone = stone.apply_rule2(); 
                    new_stones.push(new_stone);
                    continue;
                }

                stone.apply_rule3();
            }

            for new_stone in new_stones { stones.push(new_stone); }
        }

        return stones.len();
    }

}


// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ Part 2 out of 2 ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ //


mod part2 {

    use std::collections::{HashMap, VecDeque};

    /**
     * An abstraction of the magic stones in the puzzle.
     */
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    struct Stone { digits: usize } impl Stone {
        
        fn can_apply_rule1(&self) -> bool { self.digits == 0 }

        fn apply_rule1(&mut self) {
            
            if ! self.can_apply_rule1() { panic!("invalid use of rule 1."); }
            
            self.digits = 1;
        }

        fn can_apply_rule2(&self) -> bool { format!("{}", self.digits).len() % 2 == 0 }

        fn apply_rule2(&mut self) -> Stone {
            
            if ! self.can_apply_rule2() { panic!("invalid use of rule 2."); }
            
            let digit_as_str = format!("{}", self.digits);
            let ( left_digits, right_digits ) = digit_as_str.split_at(digit_as_str.len() / 2);
            self.digits = left_digits.parse().unwrap();
            return Stone { digits: right_digits.parse().unwrap() };
        }

        fn apply_rule3(&mut self) {
            
            if self.can_apply_rule1() { panic!("invalid use of rule 3, rule 1 applies instead."); }
            if self.can_apply_rule2() { panic!("invalid use of rule 3, rule 2 applies instead."); }

            self.digits *= 2024;
        }
    }

    /** A simple struct to pair a stone to a round. */
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    struct State { stone: Stone, round: usize }

    /** The amount of times that should be blinked. */
    const AMOUNT_OF_BLINKS: usize = 75;

    pub fn solve(input: &str) -> usize { solve_for(input, AMOUNT_OF_BLINKS) }

    pub fn solve_for(input: &str, amount_of_blinks: usize) -> usize {

        let mut queue: VecDeque<State> = VecDeque::new();
        /* Parsing the string into Stones */ {
            for digits in input.split(" ").into_iter() {
                queue.push_back(State { 
                    stone: Stone { digits: digits.parse().unwrap() }, 
                    round: 0,
                });
            }
        }
        
        let mut total = 0;
        let mut cache: HashMap<State, usize>= HashMap::new();
        while ! queue.is_empty() {
            let state = queue.pop_front().unwrap();
            let result = recurse(&mut cache, state, amount_of_blinks);
            total += result;
        }

        return total;
    }

    /** tries to calculate the final amount of stones, using the cache to speed things up. */
    fn recurse(cache: &mut HashMap<State, usize>, mut state: State, amount_of_blinks: usize) -> usize {

        if state.round >= amount_of_blinks {
            return 1;
        } else if let Some(cache_hit) = cache.get(&state) { 
            return *cache_hit;
        }

        let clone = state.clone();
        let stones_generated;
        let result;
        /* mutate the input */ {
            state.round += 1;
            if state.stone.can_apply_rule1() {
                state.stone.apply_rule1();
                stones_generated = 0;
            } else if state.stone.can_apply_rule2() { 
                let new_stone = state.stone.apply_rule2(); 
                let extra_state = State { stone: new_stone, round: state.round, };
                stones_generated = recurse(cache, extra_state, amount_of_blinks);
            } else {
                state.stone.apply_rule3();
                stones_generated = 0;
            }
        }

        result = recurse(cache, state, amount_of_blinks) + stones_generated;
        cache.insert(clone, result);
        
        return result;
    }
}


// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ Tests ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ //


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1::solve("125 17"), 55312);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2::solve_for("125 17", 25), 55312);
    }

    #[test]
    fn test_part2_vs_part1() {
        assert_eq!(
            part1::solve_for("125 17", 25), 
            part2::solve_for("125 17", 25),
        );
    }
    
}


// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ //
