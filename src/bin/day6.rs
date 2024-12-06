use std::fmt;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ //

fn main() {

    const INPUT: &str = include_str!("../doc/day6.txt");
    
    /* Solving Part 1 based on the input */ {
        let result: i32 = part1(INPUT);
        println!("The solution to part 1 is: \"{result}\".");
    }
        
    /* Solving Part 1 based on the input */ {
        let result: i32 = part2(INPUT);
        println!("The solution to part 2 is: \"{result}\".");
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ Helper Definitions ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ //

//` Direction

#[derive(Debug, Clone)]
enum Direction { Up, Right, Down, Left }

impl Direction {

    fn from_str(input: &char) -> Option<Direction> {
        return match input {
            '^' => Some(Direction::Up),
            '>' => Some(Direction::Right),
            'V' => Some(Direction::Down),
            '<' => Some(Direction::Left),
            _ => None
        };
    }

    fn turn(&mut self) {
        *self = match *self {
            Direction::Up    => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down  => Direction::Left,
            Direction::Left  => Direction::Up,
        };
    }
}

impl<'a> fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Direction::Up    => write!(f, "^"),
            Direction::Right => write!(f, ">"),
            Direction::Down  => write!(f, "V"),
            Direction::Left  => write!(f, "<"),
        }
    }
}

//` Guard

#[derive(Debug, Clone)]
struct Guard { direction: Direction, }

impl<'a> fmt::Display for Guard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.direction)
    }
}

//` Map Tiles

enum MapTile {
    Obstacle,
    Empty,
    Guard(Guard),
    Traveled,
}

impl MapTile {

    fn move_guard(&mut self) -> Option<Guard> {
        if let MapTile::Guard(guard) = std::mem::replace(self, MapTile::Traveled) {
            return Some(guard);
        }
        
        return None;
    }

    fn has_guard(&self) -> bool { matches!(self, MapTile::Guard(_)) }
}

impl std::fmt::Debug for MapTile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MapTile::Obstacle => write!(f, "#"),
            MapTile::Empty => write!(f, " "),
            MapTile::Guard(guard) => write!(f, "{guard}"),
            MapTile::Traveled => write!(f, "+"),
        }
    }
}

impl PartialEq for MapTile {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            ( MapTile::Obstacle, MapTile::Obstacle ) => true,
            ( MapTile::Empty,    MapTile::Empty    ) => true,
            ( MapTile::Guard(_), MapTile::Guard(_) ) => true,
            ( MapTile::Traveled, MapTile::Traveled ) => true,
            _ => false,
        }
    }
}

impl<'a> fmt::Display for MapTile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MapTile::Obstacle => write!(f, "#"),
            MapTile::Empty => write!(f, "."),
            MapTile::Guard(guard) => write!(f, "{guard}"),
            MapTile::Traveled => write!(f, "+"),
        }
    }
}

//` Point

#[derive(Debug, Clone)]
struct Point { x: i32, y: i32 }

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ Part 1 out of 2 ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ //

//` Part 1

pub fn part1(input: &str) -> i32 { 

    let mut guard_position: Option<Point> = None;

    let mut map: Vec<Vec<MapTile>> = Vec::new();
    /* Getting and creating the map */ {
        for (y, line) in input.lines().enumerate() { let y = y as i32;

            let mut map_row = Vec::new();
            for (x, character) in line.chars().enumerate() { let x = x as i32;

                let tile = match character {
                    '.' => MapTile::Empty,
                    '#' => MapTile::Obstacle,
                    '^' => MapTile::Guard(Guard { direction : Direction::Up    }),
                    '>' => MapTile::Guard(Guard { direction : Direction::Right }),
                    'V' => MapTile::Guard(Guard { direction : Direction::Down  }),
                    '<' => MapTile::Guard(Guard { direction : Direction::Left  }),
                    _ => panic!("Unrecognised input: \"{character}\".")
                };
        
                if tile.has_guard() { guard_position = Some(Point {x, y}); }
                map_row.push(tile);
            }

            map.push(map_row);
        }
    }

    if guard_position.is_none() { panic!("No guard found on the map"); }
    let mut guard_position = guard_position.unwrap();

    /* Simulating the guard on the input */ {
        loop {
            println!("");
            println!("Map at this point ({:?}) in time:", guard_position);
            for row in map.iter() {
                for tile in row.iter() {
                    print!("{tile}");
                }
                println!();
            }

            let new_tile_point = match &map[guard_position.y as usize][guard_position.x as usize] {
                MapTile::Guard(guard) => match guard.direction {
                    Direction::Up    => Point { x: guard_position.x,     y: guard_position.y - 1 },
                    Direction::Right => Point { x: guard_position.x + 1, y: guard_position.y     },
                    Direction::Down  => Point { x: guard_position.x,     y: guard_position.y + 1 },
                    Direction::Left  => Point { x: guard_position.x - 1 ,y: guard_position.y     }
                },
                _ => panic!("expected old_tile to be of type, but this wasn't the case.")
            };

            if new_tile_point.y < 0 {
                println!("Case : new_tile_point.y < 0 --> Satisfied");
                map[guard_position.y as usize][guard_position.x as usize] = MapTile::Traveled; 
                break; 
            }
            if new_tile_point.x < 0 {
                println!("Case : new_tile_point.x < 0 --> Satisfied");
                map[guard_position.y as usize][guard_position.x as usize] = MapTile::Traveled; 
                break; 
            }
            if new_tile_point.y >= map.len() as i32 {
                println!("Case : new_tile_point.y >= map.len() --> Satisfied");
                map[guard_position.y as usize][guard_position.x as usize] = MapTile::Traveled; 
                break; 
            }
            if new_tile_point.x >= map[new_tile_point.y as usize].len() as i32 {
                println!("Case : new_tile_point.x >= map[new_tile_point.y as usize].len() --> Satisfied");
                map[guard_position.y as usize][guard_position.x as usize] = MapTile::Traveled; 
                break; 
            }
            
    
            match &map[new_tile_point.y as usize][new_tile_point.x as usize] {
                MapTile::Obstacle => {
                    println!("Next tile is of type MapTile::Obstacle.");
                    match &mut map[guard_position.y as usize][guard_position.x as usize] {
                        MapTile::Guard(guard) => { 
                            println!("guard.direction: {} (before)", guard.direction);
                            guard.direction.turn();
                            println!("guard.direction: {} (after)", guard.direction);
                        },
                        _ => panic!("The old tile should always be of type MapTile::Guard(guard).")
                    }
                },
                MapTile::Empty | MapTile::Traveled => {
                    println!("Next tile is of type MapTile::Empty or MapTile::Traveled.");
                    let guard = map[guard_position.y as usize][guard_position.x as usize].move_guard().unwrap();
                    map[new_tile_point.y as usize][new_tile_point.x as usize] = MapTile::Guard(guard);
                    guard_position = new_tile_point;
                }
                _ => panic!("Unknown case.")
            }
        }
    }

    let result;
    /* Calculating the amount of distinct tiles the guard was on */ {
        let mut total = 0;
        for row in map.iter() { 
            for tile in row.iter() { 
                if *tile == MapTile::Traveled { 
                    total += 1; 
                } 
            } 
        }

        result = total;
    }
    
    return result;
}


// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ Part 2 out of 2 ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ //


pub fn part2(_: &str) -> i32 { 0 }


// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ Tests ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ //


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."), 41);
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ //
