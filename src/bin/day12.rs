// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ //


/// The main function that executes both parts and prints the output the functions computed given the provided input
/// file.
fn main() {
    const INPUT: &str = include_str!("../doc/day12.txt");
    println!("The solution to part 1 is: \"{}\".", part1::solve(INPUT));
    println!("The solution to part 2 is: \"{}\".", part2::solve(INPUT));
}


// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ Part 1 out of 2 ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ //


/// The part that solves the first part of the puzzle.
mod part1 {

    use std::{collections::{HashSet, VecDeque}, usize};

    /// An abstraction for a point in 2D space.
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    struct Point { x: usize, y: usize }

    /// An abstraction of the farm plots in the puzzle.
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    struct FarmPlot { 
        plant: char, 
        coordinates: Point, 
    }

    /// Parses a string of characters into a 2D array of `FarmPlot`s where each plant is the char in the string.
    fn create_farm_plots(input: &str) -> Vec<Vec<Option<FarmPlot>>> {
        
        let mut result: Vec<Vec<Option<FarmPlot>>> = Vec::new();
        for (y, line) in input.lines().enumerate() { 
            
            let mut row = Vec::new();
            for (x, character) in line.chars().enumerate() {
                
                row.push(Some(FarmPlot {
                    coordinates: Point {x, y},
                    plant: character,
                }));
            }

            result.push(row); 
        }
        
        return result;
    }

    /// Goes over the input searching for `FarmPlot`s that haven't been assigned a field and then returning the summed
    /// price of all the fields.
    fn calculate_price_of_fields(mut farm: Vec<Vec<Option<FarmPlot>>>) -> usize {
        
        const NO_PERIMETER_HERE: usize = 1;
        const THIS_ONE_FARM_PLOT: usize = 1;
        const FOUR_POSSIBLE_PERIMETERS: usize = 4;

        let mut total_price = 0;
        for y in 0..farm.len() {
            for x in 0..farm[y].len() {
                if let Some(farm_plot) = farm[y][x].take() {

                    let mut area = 0;
                    let mut perimeter = 0;
                    let mut character = '\0';

                    let mut visited_nodes = Vec::new();
                    let mut queue = VecDeque::new();
                    queue.push_back(farm_plot);
                    while ! queue.is_empty() {

                        let farm_plot = queue.pop_front().unwrap();
                        visited_nodes.push(farm_plot.coordinates.clone());
                        area += THIS_ONE_FARM_PLOT;
                        perimeter += FOUR_POSSIBLE_PERIMETERS;
                        character = farm_plot.plant;

                        if farm_plot.coordinates.y > 0 {
                            let point = Point { x: farm_plot.coordinates.x, y: farm_plot.coordinates.y - 1, };
                            if let Some(neighbor) = &farm[point.y][point.x] {
                                if neighbor.plant == farm_plot.plant {
                                    let moved_out = farm[point.y][point.x].take().unwrap();
                                    visited_nodes.push(point);
                                    queue.push_back(moved_out);
                                    perimeter -= NO_PERIMETER_HERE;
                                }
                            } else if visited_nodes.contains(&point) {
                                perimeter -= NO_PERIMETER_HERE;
                            }
                        }

                        if farm_plot.coordinates.x + 1 < farm[y].len()  {
                            let point = Point { x: farm_plot.coordinates.x + 1, y: farm_plot.coordinates.y, };
                            if let Some(neighbor) = &farm[point.y][point.x] {
                                if neighbor.plant == farm_plot.plant {
                                    let moved_out = farm[point.y][point.x].take().unwrap();
                                    visited_nodes.push(point);
                                    queue.push_back(moved_out);
                                    perimeter -= NO_PERIMETER_HERE;
                                }
                            } else if visited_nodes.contains(&point) {
                                perimeter -= NO_PERIMETER_HERE;
                            }
                        }

                        if farm_plot.coordinates.y + 1 < farm.len() {
                            let point = Point { x: farm_plot.coordinates.x, y: farm_plot.coordinates.y + 1, };
                            if let Some(neighbor) = &farm[point.y][point.x] {
                                if neighbor.plant == farm_plot.plant {
                                    let moved_out = farm[point.y][point.x].take().unwrap();
                                    visited_nodes.push(point);
                                    queue.push_back(moved_out);
                                    perimeter -= NO_PERIMETER_HERE;
                                }
                            } else if visited_nodes.contains(&point) {
                                perimeter -= NO_PERIMETER_HERE;
                            }
                        }
                        
                        if farm_plot.coordinates.x > 0 {
                            let point = Point { x: farm_plot.coordinates.x - 1, y: farm_plot.coordinates.y, };
                            if let Some(neighbor) = &farm[point.y][point.x] {
                                if neighbor.plant == farm_plot.plant {
                                    let moved_out = farm[point.y][point.x].take().unwrap();
                                    visited_nodes.push(point);
                                    queue.push_back(moved_out);
                                    perimeter -= NO_PERIMETER_HERE;
                                } 
                            } else if visited_nodes.contains(&point) {
                                perimeter -= NO_PERIMETER_HERE;
                            }
                        }
                    }

                    println!("A region of {} plants with price {} * {} = {}.", character, area, perimeter, area*perimeter);
                    total_price += area * perimeter;
                }
            } 
        }

        return total_price;
    }

    /// The function that solves the puzzle 
    pub fn solve(input: &str) -> usize { 
        let farm = create_farm_plots(input);
        let total_price = calculate_price_of_fields(farm);
        return total_price;
    }

}


// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ Part 2 out of 2 ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ //


/// The part that solves the first part of the puzzle.
mod part2 {

    /// The function that solves the puzzle 
    pub fn solve(input: &str) -> usize { input.len() }

}


// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ Tests ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ //


/// Tests both part solutions for correctness.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_simple() {
        assert_eq!(part1::solve("AAAA\nBBCD\nBBCC\nEEEC"), 140);
    }

    #[test]
    fn test_part1_medium() {
        assert_eq!(part1::solve("OOOOO\nOXOXO\nOOOOO\nOXOXO\nOOOOO"), 772);
    }

    #[test]
    fn test_part1_hard() {
        assert_eq!(part1::solve("RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"), 1930);
    }
}


// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ //
