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

    pub fn solve(input: &str) -> usize {
        
        const AMOUNT_OF_BLINKS: usize = 25;

        let mut stones: Vec<Stone> = Vec::new();
        for digits in input.split(" ").into_iter() {
            let digits = digits.parse().unwrap();
            let stone = Stone { digits };
            stones.push(stone);
        }

        for _ in 0..AMOUNT_OF_BLINKS {
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
    
    pub fn solve(input: &str) -> usize { input.len() }

}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ Tests ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ //


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1::solve("125 17"), 55312);
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ //
