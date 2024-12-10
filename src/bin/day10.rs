// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ //


fn main() {
    const INPUT: &str = include_str!("../doc/day10.txt");
    println!("The solution to part 1 is: \"{}\".", part1::solve(INPUT));
    println!("The solution to part 2 is: \"{}\".", part2::solve(INPUT));
}


// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ Part 1 out of 2 ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ //


mod part1 {

    use std::{collections::{HashSet, VecDeque}, fmt};

    /**
     * An abstraction for a 2D point.
     */
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    struct Point { x: usize, y: usize }

    /**
     * An abstraction for a 2D point on a map.
     * has a height and a coordinate (`Point`).
     */
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    struct MapTile { height: usize, coordinates: Point} impl MapTile {
        
        /**
         * Returns a 2D array of `MapTile`s from the input.
         * Panics if there is a char that is not a digit.
         */
        fn parse(input: &str) -> Vec<Vec<MapTile>> {
            
            let mut map = Vec::new();
            for (y, line) in input.trim().lines().rev().enumerate() {

                let mut row = Vec::new();
                for (x, character) in line.chars().enumerate() {
                    row.push(MapTile {
                        height: (character as usize) - 48,
                        coordinates: Point { x, y },
                    });
                }

                map.push(row);
            }

            return map;
        }

        /**
         * Returns a list of all points on the map where the height is 0.
         */
        fn find_start_locations(map: &Vec<Vec<MapTile>>) -> Vec<Point> {
            
            let mut result = Vec::new();       
            for row in map.iter().rev() {
                for tile in row.iter() {
                    if tile.height == 0 {
                        result.push(tile.coordinates);
                    }
                }
            }

            return result;
        }

        fn can_move_to(&self, other: &MapTile) -> bool {
            return self.height + 1 == other.height;
        }

        /**
         * Tries to find the amount of find_trailheads for a given starting point.
         */
        fn find_trailheads(map: &Vec<Vec<MapTile>>, start_position: Point) -> usize {

            let process_tile_return_valid_neighbors = |tile: &MapTile| -> HashSet<&MapTile> {
                
                let mut valid_neighbors: HashSet<&MapTile> = HashSet::new();
                
                let can_go_to_top_neighbor = tile.coordinates.y + 1< map.len();
                if can_go_to_top_neighbor {
                    if tile.can_move_to(&map[tile.coordinates.y+1][tile.coordinates.x]) {
                        valid_neighbors.insert(&map[tile.coordinates.y+1][tile.coordinates.x]);
                    }
                } else { println!("\tCan't go up from {:?}", tile.coordinates); }
                
                let can_go_to_right_neighbor = tile.coordinates.x + 1 < map[tile.coordinates.y].len();
                if can_go_to_right_neighbor {
                    if tile.can_move_to(&map[tile.coordinates.y][tile.coordinates.x+1]) {
                        valid_neighbors.insert(&map[tile.coordinates.y][tile.coordinates.x+1]);
                    }
                } else { println!("\tCan't go right from {:?}", tile.coordinates); }

                let can_go_to_bottom_neighbor = tile.coordinates.y > 0;
                if can_go_to_bottom_neighbor {
                    if tile.can_move_to(&map[tile.coordinates.y-1][tile.coordinates.x]) {
                        valid_neighbors.insert(&map[tile.coordinates.y-1][tile.coordinates.x]);
                    }
                } else { println!("\tCan't go down from {:?}", tile.coordinates); }

                let can_go_to_left_neighbor = tile.coordinates.x > 0;
                if can_go_to_left_neighbor {
                    if tile.can_move_to(&map[tile.coordinates.y][tile.coordinates.x-1]) {
                        valid_neighbors.insert(&map[tile.coordinates.y][tile.coordinates.x-1]);
                    }
                } else { println!("\tCan't go left from {:?}", tile.coordinates); }
                
                return valid_neighbors;
            };

            let mut set_of_end_points: HashSet<&MapTile> = HashSet::new();

            let mut queue: VecDeque<Point> = VecDeque::new();
            queue.push_back(start_position);
            while ! queue.is_empty() {

                let point = queue.pop_front().unwrap();
                let tile = & map[point.y][point.x];
                println!("Searching from {:?} which has height {}...", point, tile.height);
                if tile.height == 9 { set_of_end_points.insert(tile); continue; }

                let valid_neighbors = process_tile_return_valid_neighbors(tile);
                for neighbor in valid_neighbors.iter() { queue.push_back(neighbor.coordinates); } 
            }

            return set_of_end_points.len();
        }
    
    } impl<'a> fmt::Display for MapTile {
    
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.height)
        }
    }

    
    pub fn solve(input: &str) -> usize { 
        let map = MapTile::parse(input);
        let starting_locations = MapTile::find_start_locations(&map);
        let mut total = 0;
        for starting_location in starting_locations { total += MapTile::find_trailheads(&map, starting_location); }
        return total;
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
    fn test_part1_simple() {
        assert_eq!(part1::solve("0123\n1234\n8765\n9876"), 1);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1::solve("89010123\n78121874\n87430965\n96549874\n45678903\n32019012\n01329801\n10456732"), 36);
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ //
