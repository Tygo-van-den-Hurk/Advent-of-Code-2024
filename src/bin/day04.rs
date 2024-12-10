const INPUT: &str = include_str!("../doc/day04.txt");

fn main() {
    
    /* Solving Part 1 based on the input */ {
        println!("<solution-part-1>");
        let result: i32 = part1(INPUT);
        println!("\t<solution-part-1-result>{result}</solution-part-1>");
        println!("</solution-part-1>");
    }
        
    /* Solving Part 1 based on the input */ {
        println!("<solution-part-2>");
        let result: i32 = part2(INPUT);
        println!("\t<solution-part-2-result>{result}</solution-part-2>");
        println!("</solution-part-2>");
    }
}

//` Part 1

pub fn part1(input: &str) -> i32 {

    let puzzle: Vec<Vec<char>>;
    /* Creating the puzzle 2D Array searching space. */ {
        let mut temp = Vec::new();
        for line in input.lines() { temp.push( line.chars().collect() ); }
        puzzle = temp;
    }

    let mut total = 0;
    /* Counting the total instances of "XMAS" in the searching space. */ {
        for y in 0..puzzle.len() {
            for x in 0..puzzle[y].len() {
                if puzzle[y][x] != 'X' { continue; }
                println!("\t<x-found x=\"{x}\" y=\"{y}\">");
                let total_matches = get_matches( &puzzle, x as i32, y as i32 );
                println!("\t\t<search-total>{total_matches}</search-total>");
                println!("\t</x-found>");
                total += total_matches;
            }
        }
    }

    return total;
}

fn get_matches(puzzle: &Vec<Vec<char>>, x_coordinate: i32, y_coordinate: i32) -> i32 {

    let mut total = 0;

    let word = [ 'X', 'M', 'A', 'S' ];
    let multipliers: [i32; 3] = [ -1, 0, 1 ];
    let mut state: [[bool; 3]; 3] = [ 
        [ true, true,  true ],
        [ true, false, true ],
        [ true, true,  true ]
    ];

    for index in 1..word.len() {
        let letter = word[index];
        
        println!("\t\t<search for=\"{letter}\">");

        for y_multiplier in multipliers {
            for x_multiplier in multipliers {

                let x = x_coordinate + index as i32 * x_multiplier;
                let y = y_coordinate + index as i32 * y_multiplier;
                
                println!("\t\t\t<check for=\"{letter}\" at-x=\"{x}\" at-y=\"{y}\">");

                if y < 0 { 
                    println!("\t\t\t\t<problem> y less then 0 </problem>");
                    println!("\t\t\t</check>");
                    state[(y_multiplier + 1) as usize][(x_multiplier + 1) as usize] = false;
                    continue;
                }
                if y >= puzzle.len() as i32 { 
                    println!("\t\t\t\t<problem> y more then or equal to puzzle.len() </problem>");
                    println!("\t\t\t</check>");
                    state[(y_multiplier + 1) as usize][(x_multiplier + 1) as usize] = false;
                    continue;
                }
                
                if x < 0 { 
                    println!("\t\t\t\t<problem> x less then 0 </problem>");
                    println!("\t\t\t</check>");
                    state[(y_multiplier + 1) as usize][(x_multiplier + 1) as usize] = false;
                    continue; 
                }
                if x >= puzzle[y as usize].len() as i32{
                    println!("\t\t\t\t<problem> x more then or equal to puzzle[y].len() </problem>");
                    println!("\t\t\t</check>");
                    state[(y_multiplier + 1) as usize][(x_multiplier + 1) as usize] = false;
                    continue;
                }
                
                let result = puzzle[y as usize][x as usize];
                if result != letter { 
                    println!("\t\t\t\t<problem> expected {letter} found {result} </problem>");
                    println!("\t\t\t</check>");
                    state[(y_multiplier + 1) as usize][(x_multiplier + 1) as usize] = false;
                    continue;
                }
            }    
        }

        println!("\t\t</search>");
    }

    /* summing up the matches */ {
        for state_array in state {
            for individual_state in state_array {
                if individual_state { total += 1; }
            }
        }
    }

    return total;
}

//` Part 2

pub fn part2(input: &str) -> i32 { 

    let puzzle: Vec<Vec<char>>;
    /* Creating the puzzle 2D Array searching space. */ {
        let mut temp = Vec::new();
        for line in input.lines() { temp.push( line.chars().collect() ); }
        puzzle = temp;
    }

    let mut total = 0;
    /* Counting the total instances of "XMAS" in the searching space. */ {
        for y in 0..puzzle.len() {
            for x in 0..puzzle[y].len() {
                if puzzle[y][x] != 'A' { continue; }
                println!("\t<A-found x=\"{x}\" y=\"{y}\">");
                let total_matches = get_matches_2( &puzzle, x as i32, y as i32 );
                println!("\t\t<search-total>{total_matches}</search-total>");
                println!("\t</A-found>");
                total += total_matches;
            }
        }
    }

    return total;
}

fn get_matches_2(puzzle: &Vec<Vec<char>>, x_coordinate: i32, y_coordinate: i32) -> i32 {

    /* Checking space-constrained pre-conditions */ {
        if y_coordinate < 1 { return 0; }
        if y_coordinate + 1 >= puzzle.len() as i32 { return 0; }
        if x_coordinate < 1 { return 0; }
        if x_coordinate + 1 >= puzzle[y_coordinate as usize].len() as i32 { return 0; }
    }

    /* Checking if there is a match */ {
        let y = y_coordinate as usize;
        let x = x_coordinate as usize;
        if ( puzzle[y - 1][x - 1] == 'M' &&  puzzle[y + 1][x + 1] == 'S' ) &&
           ( puzzle[y - 1][x + 1] == 'M' &&  puzzle[y + 1][x - 1] == 'S' ) { return 1; }
        if ( puzzle[y + 1][x + 1] == 'M' &&  puzzle[y - 1][x - 1] == 'S' ) &&
           ( puzzle[y - 1][x + 1] == 'M' &&  puzzle[y + 1][x - 1] == 'S' ) { return 1; } 
        if ( puzzle[y - 1][x - 1] == 'M' &&  puzzle[y + 1][x + 1] == 'S' ) &&
           ( puzzle[y + 1][x - 1] == 'M' &&  puzzle[y - 1][x + 1] == 'S' ) { return 1; }
        if ( puzzle[y + 1][x + 1] == 'M' &&  puzzle[y - 1][x - 1] == 'S' ) &&
           ( puzzle[y + 1][x - 1] == 'M' &&  puzzle[y - 1][x + 1] == 'S' ) { return 1; } 
    }

    return 0;
}

//` Test

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"), 18);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"), 9);
    }
}

