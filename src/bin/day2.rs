#![allow(unused_parens)]

use std::{fmt::{self}, i32};

const INPUT: &str = include_str!("../doc/day2.txt");

fn main() {
    
    /* Solving Part 1 based on the input */ {
        println!("Solving part 1:");
        let result: i32 = part1(INPUT);
        println!("The solution to part 1 is: \"{result}\".");
    }
        
    /* Solving Part 1 based on the input */ {
        println!("Solving part 2:");
        let result: i32 = part2(INPUT);
        println!("The solution to part 2 is: \"{result}\".");
    }
}

//` Direction of the values

enum Direction {
    Increasing,
    Decreasing
}

impl Direction {
    
    fn from_numbers(first: i32, second: i32) -> Direction {
        if first < second { return Direction::Increasing; }
        else { return Direction::Decreasing; }
    }
}

impl PartialEq for Direction {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Direction::Increasing, Direction::Increasing) => true,
            (Direction::Decreasing, Direction::Decreasing) => true,
            _ => false,
        }
    }
}

impl<'a> fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Direction::Increasing => write!(f, "Increasing"),
            Direction::Decreasing =>write!(f, "Decreasing"),
        }
    }
}

//` Report State
enum ReportState { 
    Safe, 
    Unsafe(String)
}

impl PartialEq for ReportState {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ReportState::Safe, ReportState::Safe) => true,
            (ReportState::Unsafe(reason1), ReportState::Unsafe(reason2)) => reason1 == reason2,
            _ => false,
        }
    }
}

impl fmt::Display for ReportState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReportState::Safe => write!(f, "Safe"),
            ReportState::Unsafe(reason) => write!(f, "Unsafe: {}", reason),
        }
    }
}

//` Part 1

pub fn part1(input: &str) -> i32 {

    let mut total_save_reports: i32 = 0;
    for report in input.lines() {
        let report_values: Vec<&str> = report.split(" ").collect();
        let report_state: ReportState = check_report(&report_values);
        if report_state == ReportState::Safe {
            total_save_reports += 1;
        }
        println!("\t- {report} -> {report_state}");
    }

    return total_save_reports;
}

fn check_report(report: &Vec<&str>) -> ReportState {

    const MIN_DIFFERENCE: i32 = 1;
    const MAX_DIFFERENCE: i32 = 3;

    let mut last_entry: i32 = report[0].parse::<i32>().unwrap();
    let direction: Direction = Direction::from_numbers(last_entry, report[1].parse::<i32>().unwrap());
    for index in 1..report.len() {
        let current_entry: i32 = report[index].parse::<i32>().unwrap(); 
        let difference = (current_entry - last_entry).abs();
        let current_direction = Direction::from_numbers(last_entry, current_entry);
        /* Checking conditions */ {
            if difference < MIN_DIFFERENCE { return ReportState::Unsafe(format!("change not large --> ({difference})")); }
            if difference > MAX_DIFFERENCE { return ReportState::Unsafe(format!("change too large --> ({difference})")); }
            if direction != current_direction { 
                return ReportState::Unsafe("direction mismatch".to_string());
            }
        }
        
        last_entry = current_entry;
    }

    // if no violations are found:
    return ReportState::Safe;
}

//` Part 2


pub fn part2(input: &str) -> i32 { 

    let mut rejects: Vec<&str> = Vec::new();
    let mut total_save_reports: i32 = 0;
    for report in input.lines() {

        let mut report_results: Vec<ReportState> = Vec::new();
        let report_values: Vec<i32> = ( report
            .split(" ")
            .map(|string| {string.parse::<i32>().unwrap()})
            .collect()
        );

        /* Trying ignoring none, or one of the report entries */ {
            println!("\t- Will now attempt to solve: {report}");
            println!("\t\t- ignoring no elements:");
            let state_of_not_ignoring_one = check_report_part2(&report_values,  Option::None);
            println!("\t\t- {state_of_not_ignoring_one}");
            report_results.push(state_of_not_ignoring_one);

            for index in 0..report_values.len() {
                let ignored_element = report_values[index];
                println!("\t\t- ignoring {ignored_element}");
                let report_state = check_report_part2(&report_values,  Option::Some(index as i32));
                println!("\t\t- {report_state}");
                report_results.push(report_state);
            }   
        }
        
        /* Seeing if one of the options is safe */ {
            if report_results
                .iter().filter(| result| { **result == ReportState::Safe })
                .collect::<Vec<&ReportState>>().len() > 0 {
                total_save_reports += 1;
                println!("\t- There existed a safe way. Total is now: {total_save_reports}.");
            } else {
                println!("\t- There didn't exist a safe way. Total remains: {total_save_reports}.");
                rejects.push(&report);
            }
        }        
    }

    println!("Rejects: {:?}.", rejects);
    return total_save_reports;
}

fn check_report_part2<'a>(report: &Vec<i32>, skip: Option<i32>) -> ReportState {

    let skippy = skip.unwrap_or_else(|| -1);
    println!("\t\t\t- Skippy: {skippy}");

    const MIN_DIFFERENCE: i32 = 1;
    const MAX_DIFFERENCE: i32 = 3;

    let start;
    if (skippy == 0) { start = 1; } else { start = 0; }
    let mut last_entry: i32 = report[start];

    let direction: Direction;
    if (skippy == 1) { direction = Direction::from_numbers(last_entry, report[start+2]); }
    else { direction = Direction::from_numbers(last_entry, report[start+1]);}

    println!("\t\t\t- Direction: {direction}");

    println!("\t\t\t- Start: {start}");

    for index in start+1..report.len() {
        
        if skippy == index as i32 { continue; }

        let current_entry: i32 = report[index]; 
        let difference = (current_entry - last_entry).abs();
        let current_direction = Direction::from_numbers(last_entry, current_entry);
        /* Checking conditions */ {
            if difference < MIN_DIFFERENCE    { return ReportState::Unsafe(format!("change too small, expected at least {MIN_DIFFERENCE}, but got: {difference}.")); }
            if difference > MAX_DIFFERENCE    { return ReportState::Unsafe(format!("change too large, expected at least {MAX_DIFFERENCE}, but got: {difference}.")); }
            if direction != current_direction { return ReportState::Unsafe(format!("direction mismatch, expected {direction} but got {current_direction}.")); }
        }
        
        last_entry = current_entry;
    }

    // if no violations are found:
    return ReportState::Safe;
}

//` Test

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        println!("Test case part 1:");
        let result: i32 = part1("7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9");
        assert_eq!(result, 2);
    }

    
    #[test]
    fn test_part2() {
        println!("Test case part 2:");
        assert_eq!(part2("7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"), 4);
    }

    #[test]
    fn test_part2_edgecase() {
        println!("Testing edge case for part 2:");
    assert_eq!(part2("48 46 47 49 51 54 56
1 1 2 3 4 5
1 2 3 4 5 5
5 1 2 3 4 5
1 4 3 2 1
1 6 7 8 9
1 2 3 4 3
9 8 7 6 7
7 10 8 10 11
29 28 27 25 26 25 22 20"), 10);
    }



    #[test]
    fn test_part2_failing() {
        println!("Testing edge case for part 2:");
    assert_eq!(part2("61 59 63 65 67
45 48 44 41 40
59 60 56 53 51
34 32 37 38 40 43 45 46"), 4);
    }
}
