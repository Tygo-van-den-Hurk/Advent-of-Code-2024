// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ //


fn main() {
    const INPUT: &str = include_str!("../doc/day9.txt");
    println!("The solution to part 1 is: \"{}\".", part1::solve(INPUT));
    println!("The solution to part 2 is: \"{}\".", part2::solve(INPUT));
}


// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ Part 1 out of 2 ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ //


mod part1 {

    use std::fmt;
    
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    enum FileSystemBlock { Empty, Filled(usize) } impl FileSystemBlock {
        
    } impl<'a> fmt::Display for FileSystemBlock {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                FileSystemBlock::Filled(_) => write!(f, "#"),
                FileSystemBlock::Empty => write!(f, " "),
            }
        }
    }

    fn has_gap(vec: &Vec<FileSystemBlock>) -> bool {
        let mut has_seen_empty_space = false;
        for file_system_block in vec {
            match file_system_block {
                FileSystemBlock::Empty => { has_seen_empty_space = true; },
                FileSystemBlock::Filled(_) => {  
                    if has_seen_empty_space { return true; }
                },
            }
        }

        return false;
    }

    pub fn solve(input: &str) -> usize { 

        let mut file_system: Vec<FileSystemBlock> = Vec::new();
        /* Creating the file system */ {
            let mut id = 0;
            let mut current_char_symbolises_empty_space: bool = false;
            for character in input.chars() {
                let block_length = (character as u32) - 48;
                if block_length > 9 { panic!("The character {} is not an integer", character); }
                for _ in 0..block_length {
                    if current_char_symbolises_empty_space { file_system.push(FileSystemBlock::Empty); }
                    else { file_system.push(FileSystemBlock::Filled(id));}
                }
                if !current_char_symbolises_empty_space { id += 1; }
                current_char_symbolises_empty_space = !current_char_symbolises_empty_space;
            }
        }

        /* Moving filled file system entries from the back to empty spaces in the front */ {
            loop { 
                let gap_index = file_system.iter().position( |x| *x == FileSystemBlock::Empty ).unwrap();
                let trailing_file_block_index = file_system.iter().rposition( 
                    |x| *x != FileSystemBlock::Empty
                ).unwrap();
                file_system.swap(gap_index, trailing_file_block_index);

                if ! has_gap(&file_system) { break; }
            }
        }

        let mut total = 0;
        /* Calculating the filesystem 'hash' */ {
            for (index, file_system_block) in file_system.iter().enumerate() {
                match file_system_block {
                    FileSystemBlock::Filled(id) => total += id * index,
                    FileSystemBlock::Empty => break,
                }

            }
        }

        return total;
    }

}


// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ Part 2 out of 2 ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ //


mod part2 {

    use std::fmt;

    /**
     * Represents a single block in the file system.
     */
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    struct FileSystemBlock { 
        size: u32, 
        id: u32, 
        is_file: bool, 
        has_been_moved: bool
    } impl<'a> fmt::Display for FileSystemBlock {

        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self.is_file {
                false => write!(f, "{}", ".".repeat(self.size as usize)),
                true  => match self.has_been_moved {
                    true  => write!(f, "{}", "M".repeat(self.size as usize)),
                    false => write!(f, "{}", format!("{}", self.id ).repeat(self.size as usize)),
                },
            }
        }
    }
    
    /**
     * Creates a Vec<FileSystemBlock> representing the state of the file system from input string.
     * Panics if the input is not a string of numbers.
     */
    fn parse_file_system(input: &str) -> Vec<FileSystemBlock> {
        let mut file_system: Vec<FileSystemBlock> = Vec::new();
        let mut id = 0;
        let mut current_char_symbolises_empty_space: bool = false;
        for character in input.chars() {
            
            let block_length = (character as u32) - 48;
            if block_length > 9 { panic!("The character {} is not an integer", character); }

            file_system.push(FileSystemBlock { 
                size: block_length, 
                id: id, 
                is_file: ! current_char_symbolises_empty_space, 
                has_been_moved: false 
            });
            
            if ! current_char_symbolises_empty_space { id += 1; }
            current_char_symbolises_empty_space = ! current_char_symbolises_empty_space;
        }
        
        return file_system;
    }

    fn restructure_file_system(file_system: &mut Vec<FileSystemBlock>) {
        loop { 
            
            if file_system.iter().position(|f| !f.has_been_moved) == None { return; }

            let file_system_block_to_move_index: usize;
            /* Getting the index of a file block that hasn't been attempted to move yet */ {
                let index = file_system.iter().rposition(|f| ! f.has_been_moved && f.is_file);
                if index.is_none() { return; }
                file_system_block_to_move_index = index.unwrap();
                file_system[file_system_block_to_move_index].has_been_moved = true;
            }

            let empty_file_system_block_index: usize;
            /* Getting the index of the first empty size that meets the requirements */ {
                let requested_minimum_space = file_system[file_system_block_to_move_index].size;
                let index = file_system.iter().position(|f| f.size >= requested_minimum_space && ! f.is_file );
                if index.is_none() { continue; }
                empty_file_system_block_index = index.unwrap();
                if empty_file_system_block_index > file_system_block_to_move_index { continue; }
            }          
            
            /* Swapping the files around if they are the same size */ {
                let file_block = & file_system[file_system_block_to_move_index];
                let empty_file_block = & file_system[empty_file_system_block_index];
                if file_block.size == empty_file_block.size {
                    file_system.swap(file_system_block_to_move_index, empty_file_system_block_index);
                    continue;
                }    
            }
            
            /* Creating and swapping File around if that's not the case. */ {
                let size_of_file = file_system[file_system_block_to_move_index].size;
                let size_of_empty_space = file_system[empty_file_system_block_index].size;
                let empty_file_block = FileSystemBlock {
                    is_file: false,
                    id: 0,
                    has_been_moved: true,
                    size: size_of_file
                };
                file_system.swap(empty_file_system_block_index, file_system_block_to_move_index);
                file_system[file_system_block_to_move_index] = empty_file_block;
                let remaining_space = size_of_empty_space - size_of_file;
                let mini_empty_file_block = FileSystemBlock {
                    is_file: false,
                    id: 0,
                    has_been_moved: true,
                    size: remaining_space,
                };
                let one_to_the_right = empty_file_system_block_index + 1;
                file_system.insert(one_to_the_right, mini_empty_file_block);
            }
        }
    }

    /**
     * Hashes the file system as described in the assignment.
     */
    fn hash_file_system(file_system: Vec<FileSystemBlock>) -> usize {
        let mut total: u64 = 0;
        let mut file_system_index = 0;
        for file_system_block in file_system.iter() {
            match file_system_block.is_file {
                true => {
                    for index in file_system_index .. file_system_index + file_system_block.size {
                        total += (file_system_block.id * index) as u64;
                    }
                    file_system_index += file_system_block.size;
                },
                false => file_system_index += file_system_block.size,
            }

        }

        return total as usize;
    }

    /**
     * Solves the part 2 of the 8th day challenge of Advent of Code.
     */
    pub fn solve(input: &str) -> usize { 
        let mut file_system: Vec<FileSystemBlock> = parse_file_system(input);
        restructure_file_system(&mut file_system);
        return hash_file_system(file_system);
    }

}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ Tests ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ //


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1::solve("2333133121414131402"), 1928);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2::solve("2333133121414131402"), 2858);
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ //
