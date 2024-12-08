// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ //


fn main() {
    const INPUT: &str = include_str!("../doc/day8.txt");
    println!("The solution to part 1 is: \"{}\".", part1::solve(INPUT));
    println!("The solution to part 2 is: \"{}\".", part2::solve(INPUT));
}


// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ Part 1 out of 2 ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ //


mod part1 {
    
    use std::{collections::{HashMap, HashSet}, vec};

    #[derive(Clone, Debug, PartialEq, Eq, Hash)]
    struct Point { x: usize, y: usize }

    #[derive(Clone, Debug, PartialEq, Eq, Hash)]
    struct Size { width: usize, height: usize }

    pub fn solve(input: &str) -> usize { 

        let map_size;
        /* Getting the map sizes */ {
            let height =  input.lines().collect::<Vec<_>>().len();
            let width = input.len() / height;
            map_size = Size { width, height };
        }

        let mut lookup_cache: HashMap<char, Vec<Point>> = HashMap::new();
        /* Getting the antennas from the map, storing where they are and grouping them by type */ {
            for (y, line) in input.lines().rev().enumerate() {
                for (x, character) in line.chars().enumerate() {
                    
                    if character == '.' { continue; } 

                    if let Some(value) = lookup_cache.get_mut(&character) {
                        value.push( Point { x, y } );
                        continue;
                    }
                    
                    lookup_cache.insert(character, vec![ Point {x, y} ] );
                }
            }
        }

        let mut antinode_locations: HashSet<Point> = HashSet::new();
        /* Calculating the amount of antinode locations onto the map */ {
            for antenna_locations in lookup_cache.values() {
                for antenna1 in antenna_locations {
                    for antenna2 in antenna_locations {

                        if *antenna1 == *antenna2 { continue; }

                        let x_diff: i32 = (antenna1.x as i32) - (antenna2.x as i32);
                        let new_x: i32 = (antenna1.x as i32) + x_diff;
                        if new_x < 0 || new_x >= map_size.width  as i32 { continue; }

                        let y_diff: i32 = (antenna1.y as i32) - (antenna2.y as i32);
                        let new_y: i32  = (antenna1.y as i32) + y_diff;
                        if new_y < 0 || new_y >= map_size.height as i32 { continue; }
                        
                        let point = Point { x: new_x as usize, y: new_y as usize };
                        antinode_locations.insert(point);
                    }
                }
            }
        }

        return antinode_locations.len();
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
        assert_eq!(part1::solve("............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"), 14);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2::solve("............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"), 34);
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ //
