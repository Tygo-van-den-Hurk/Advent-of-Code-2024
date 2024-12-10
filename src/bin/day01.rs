const INPUT: &str = include_str!("../doc/day01.txt");

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

    let amount_of_lines: usize = input.lines().count();
    let mut list0: Vec<&str> = Vec::with_capacity(amount_of_lines);
    let mut list1: Vec<&str> = Vec::with_capacity(amount_of_lines);

    /* Adding the numbers from the input to both lists. */ {
        for line in input.lines() {
            let split_line: Vec<&str> = line.split("   ").collect();
            list0.push(split_line[0]);
            list1.push(split_line[1]);
        }
    }

    let mut total_difference: i32 = 0;
    /* Calculating the total difference */ {
        list0.sort();
        list1.sort();
        for index in 0..list0.len() {
            let element_from_list0: i32 = list0[index].parse::<i32>().unwrap();
            let element_from_list1: i32= list1[index].parse::<i32>().unwrap();
            total_difference += (element_from_list0 - element_from_list1).abs();
        }
    }

    return total_difference;
}

//` Part 2

pub fn part2(input: &str) -> i32 { 
    
    let amount_of_lines: usize = input.lines().count();
    let mut list0: Vec<&str> = Vec::with_capacity(amount_of_lines);
    let mut list1: Vec<&str> = Vec::with_capacity(amount_of_lines);

    /* Adding the numbers from the input to both lists. */ {
        for line in input.lines() {
            let split_line: Vec<&str> = line.split("   ").collect();
            list0.push(split_line[0]);
            list1.push(split_line[1]);
        }
    }

    let mut total: usize = 0;
    /* Calculating the total difference */ {
        for element in list0 {
            let count = list1.iter().filter(|&x| *x == element).count();
            total += element.parse::<usize>().unwrap() * count;
        }
    }

    return total as i32;
}

//` Test

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result: i32 = part1("3   4
4   3
2   5
1   3
3   9
3   3");
        assert_eq!(result, 11);
    }

    
    #[test]
    fn test_part2() {
        let result: i32 = part2("3   4
4   3
2   5
1   3
3   9
3   3");
        assert_eq!(result, 31);
    }
}

