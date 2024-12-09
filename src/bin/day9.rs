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

    pub fn solve(input: &str) -> usize { input.len() }

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
