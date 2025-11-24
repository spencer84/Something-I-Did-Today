use std::num::ParseIntError;
use chrono::{prelude::*, NaiveDate, TimeDelta, DateTime, Local};
use crate::db::{change_date, get_tags, read_all_entries, read_selected_entries, read_selected_tags};

pub mod db;
pub mod settings;

/// Check if a string matches a flag pattern.
/// E.g. Contains '-' or '--'
pub fn is_flag_pattern(arg: &String) -> bool {
    if arg.starts_with("--") | arg.starts_with("-") {
        return true;
    }
    false
}

pub fn is_reserved_value(tag: &String) -> bool {
    let reserved_values = ["e","edit","r","read","help","y","yesterday","d","delete","cd","change-date","l","last","s","search"];
    reserved_values.contains(&tag.as_str())
}

// Try to get a date from the first argument. If first arg is not numeric/date type, then use the current date
pub fn get_date(arg: &str) -> Option<String> {
    let date: String = arg.to_string();

    if contains_numbers(&date) {
        let separator_option = get_separator(&date);

        // If there is a separator, split the string and re-join
        let parsed_date: Option<String> = match separator_option {
            Some(separator) => {
                let date_copy = date.clone();
                parse_separated_date(date_copy, separator)
            }
            // Otherwise just use the numeric string
            None => parse_numeric_string(date),
        };

        // If parsed date has a String value, return that after formatting
        println!("Parsed date:{}", parsed_date.clone().unwrap());
        parsed_date
    } else {
        None
    }
}

// Evaluate if the input string has a numeric input
pub fn contains_numbers(string: &String) -> bool {
    for num in 0..10 {
        let numeric_char: char = char::from_digit(num as u32, 10).unwrap();
        if string.contains(numeric_char) {
            return true;
        }
    }
    false
}

// Try to identify the separator used
fn get_separator(string: &String) -> Option<char> {
    if string.contains("\\") {
        let sep = '\\';
        return Some(sep);
    }
    if string.contains("/") {
        let sep = '/';
        return Some(sep);
    }
    if string.contains("-") {
        let sep = '-';
        return Some(sep);
    }
    if string.contains(".") {
        let sep = '.';
        return Some(sep);
    }
    if string.contains("|") {
        let sep = '|';
        return Some(sep);
    } else {
        return None;
    }
}

// Read a date string left to right--Accepts the following formats:
// D, DD, DDM, DDMM, DDMMYY, DDMMYYYY
fn parse_numeric_string(numeric_string: String) -> Option<String> {
    let local_date: DateTime<Local> = Local::now();
    let size = numeric_string.len();
    // Single day input
    if size == 2 || size == 1 {
        // For single day input, assume current month/year
        let month = local_date.month();
        let year = local_date.year();
        let day = match_day_string(numeric_string);
        match day {
            Some(day) => return std::option::Option::Some(format_date(day, month, year)),
            None => return None,
        }
    }
    if size > 2 && size < 5 {
        // For date/month input assume current year (how handle if not yet reached this date?)
        let year = local_date.year();
        // Split out the days
        let day = numeric_string[..2].parse::<i32>().unwrap();
        let month = numeric_string[2..].parse::<u32>().unwrap();

        if day_is_valid(day) && month_is_valid(month) {
            return std::option::Option::Some(format_date(day, month, year));
        } else {
            return None;
        }
    }
    // DDMMYY
    if size == 6 {
        let day = numeric_string[..2].parse::<i32>().unwrap();
        let month = numeric_string[2..4].parse::<u32>().unwrap();
        let year = numeric_string[4..].parse::<i32>().unwrap();
        if day_is_valid(day) && month_is_valid(month) && year_is_valid(year) {
            return std::option::Option::Some(format_date(day, month, year + 2000));
        } else {
            return None;
        }
    }
    // DDMMYYYY
    if size == 8 {
        println!("Made it to the date size 8");
        let day = numeric_string[..2].parse::<i32>().unwrap();
        let month = numeric_string[2..4].parse::<u32>().unwrap();
        let year = numeric_string[5..].parse::<i32>().unwrap();
        if day_is_valid(day) && month_is_valid(month) && year_is_valid(year) {
            return std::option::Option::Some(format_date(day, month, year));
        }
        // YYYYMMDD
        let day = numeric_string[7..].parse::<i32>().unwrap();
        let month = numeric_string[4..6].parse::<u32>().unwrap();
        let year = numeric_string[..4].parse::<i32>().unwrap();
        if day_is_valid(day) && month_is_valid(month) && year_is_valid(year) {
            return std::option::Option::Some(format_date(day, month, year));
        } else {
            return None;
        }
    }
    // TODO: Catch inputs of different date patterns
    else {
        return None;
    }
}

fn parse_separated_date(separated_date: String, separator: char) -> Option<String> {
    println!("Parsing separated date:{}", separated_date);
    let parts = separated_date.split(separator).collect::<Vec<&str>>();
    // Handle 2 parts
    if parts.len() == 2 {
        // Assume day first
        let day = parts[0].parse::<i32>().unwrap();
        let month = parts[1].parse::<u32>().unwrap();
        if day_is_valid(day) && month_is_valid(month) {
            // Get current year
            let year = Local::now().year();
            return Some(format_date(day, month, year));
        }
        println!("Invalid date format!"); // TODO do we need this? Or do we log errors at a higher level?
        None
    }
    // Handle 3 parts
    else if parts.len() == 3 {
        // Check if YYYY-DD-MM
        if parts[0].len() == 4 {
            let year = parts[0].parse::<i32>().unwrap();
            let month = parts[1].parse::<u32>().unwrap();
            let day = parts[2].parse::<i32>().unwrap();
            if day_is_valid(day) && month_is_valid(month) && year_is_valid(year) {
                return Some(format_date(day, month, year));
            } else {
                None
            }
        } else if parts[0].len() == 2 {
            let day = parts[2].parse::<i32>().unwrap();
            let month = parts[1].parse::<u32>().unwrap();
            let year = parts[0].parse::<i32>().unwrap();
            if day_is_valid(day) && month_is_valid(month) && year_is_valid(year) {
                return Some(format_date(day, month, year));
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    }
}

// Try to parse the day portion of the numeric input into a date value
fn match_day_string(day_string: String) -> Option<i32> {
    // Cast to int
    let day = day_string.parse::<i32>().unwrap();
    // Check if between 1 - (current month days)
    if &day <= &31 && &day > &0 {
        return Some(day);
    } else {
        None
    }
}

// Check if day is valid
fn day_is_valid(day: i32) -> bool {
    if &day <= &31 && &day > &0 {
        true
    } else {
        false
    }
}

// Check if month is valid
fn month_is_valid(month: u32) -> bool {
    if &month <= &12 && &month > &0 {
        true
    } else {
        false
    }
}

// Check if year is valid (this is somewhat subjectve--what if someone wants to record what they did back in 1998; that's valid year...Maybe have an config option for this later?)
fn year_is_valid(year: i32) -> bool {
    let local_date: DateTime<Local> = Local::now();
    let current_year = local_date.year();
    if year < 100 {
        return year <= current_year - 2000 && year > 0;
    } else {
        return year <= current_year && year > 2000;
    }
}

// Return a date string for a numerically described date
fn format_date(day: i32, month: u32, year: i32) -> String {
    // Add date padding for month
    let padded_month: String = if month < 10 {
        "0".to_owned() + &month.to_string()
    } else {
        month.to_string()
    };

    // Add date padding for day
    let padded_day: String = if day < 10 {
        "0".to_owned() + &day.to_string()
    } else {
        day.to_string()
    };

    let date: String = year.to_string() + "-" + &padded_month + "-" + &padded_day.to_string();
    date
}

// Return a list of arguments and useful information about how to use the program
pub fn get_help() {
    println!("Usage: sidt <entry>");
    println!("Options: ");
    println!("-h, --help                        Print help");
    println!(
        "-r, --read <number>               Read last <number> lines (or use a/all for all entries"
    );
    println!("-y, --yesterday <entry>           Write an entry for yesterday's date");
    println!("-l, --last                        Read last entry");
    println!("-e, --edit <date>                 Edit a previous entry");
    println!("-cd, --change-date <old> <new>    Change an entry date");
    println!("-t, --tag <tag>                   Create a new tag for grouping entries");
}
pub fn update_date(args: Vec<String>) {
    // Extract second and third args

    let old: Option<&String> = args.get(2);

    let new: Option<&String> = args.get(3);

    if old.is_some() && new.is_some() {
        // Try to get date values from args

        let old_date = get_date(old.unwrap());

        let new_date = get_date(new.unwrap());

        if old_date.is_some() && new_date.is_some() {
            // Apply the changes to the db
            change_date(&old_date.unwrap(), &new_date.unwrap())
        } else {
            println!("Invalid date args")
        }
    } else {
        println!("Invalid date args")
    }
}
/// Returns an Optional value if a valid tag is found.
/// If the argument does not match any known tags, then the value is None.
pub fn check_tag(arg: &String) -> Option<String> {
    // Get all tags from db
    let tags = get_tags();
    if arg.contains("--") {
        let possible_tag = arg.strip_prefix("--").unwrap().to_string();
        return if tags.contains(&possible_tag) {
            Some(possible_tag.to_string())
        } else {
            None
        };
    } else if arg.contains("-") {
        let possible_tag = arg.strip_prefix("-").unwrap().to_string();
        return if tags.contains(&possible_tag) {
            Some(possible_tag.to_string())
        } else {
            None
        };
    } else {
        None
    }
}

fn print_lines(args: Vec<String>) {
    // Try to extract additional argument to identify number of lines to print
    if args.len() > 2 {
        let lines_to_print: Result<usize, ParseIntError> = args[2].trim().parse();
        if lines_to_print.is_ok() {
            let number = lines_to_print.unwrap();
            println!("Number of lines to print:{}", number);

            read_selected_entries(number);
        }
        // Handle arg for all records
        else if args[2].trim() == "a" || args[2].trim() == "all" {
            read_all_entries();
        } else {
            println!(
                "{}",
                "Read argument must be numeric value. E.g. r 5 to read the last 5 lines."
            )
        }
    }
    // Otherwise just print all lines
    else {
        let number: usize = 5;
        read_selected_entries(number);
    }
}

fn print_tags(tag: &String, args: Vec<String>) {
    // Try to extract additional argument to identify number of lines to print
    if args.len() > 2 {
        let lines_to_print: Result<usize, ParseIntError> = args[2].trim().parse();
        if lines_to_print.is_ok() {
            let number = lines_to_print.unwrap();
            println!("Number of lines to print:{}", number);

            read_selected_entries(number);
        }
        // Handle arg for all records
        else if args[2].trim() == "a" || args[2].trim() == "all" {
            read_all_entries();
        } else {
            println!(
                "{}",
                "Read argument must be numeric value. E.g. r 5 to read the last 5 lines."
            )
        }
    }
    // Otherwise just print all lines
    else {
        let number: usize = 5;
        read_selected_tags(tag, number);
    }
}

/// sidt -r -t
/// sidt -r
/// Sub args for the read arg
pub enum ReadSubArg {
    Tag(String),
    /// if tag, extract the tag value
    Numeric(usize),
    None,
}

/// All sub args for each arg
/// -r -> tag, numeric (number of lines)
/// free text -> tag, date
/// edit -> date, //TODO Add ability to edit tags
/// yesterday -> tag
/// search -> free text
/// delete -> tag, date
//
pub fn assign_read_subarg(arg_option: &Option<&String>) -> ReadSubArg {
    // When parsing args, consider the following order...
    // Does it have a flag pattern? (i.e. contains '-' or '--')
    // If yes, parse it as a flag
    // If not, is it numeric?
    // If yes, then read that many lines
    // if not, then we've exhausted the valid arguments for the read flag
    match arg_option {
        Some(arg) => {
            if is_flag_pattern(&arg) {
                let possible_tag = check_tag(&arg);
                match possible_tag {
                    Some(tag) => ReadSubArg::Tag(tag),
                    None => ReadSubArg::None,
                }
            } else if contains_numbers(&arg) {
                let lines_to_print: Result<usize, ParseIntError> = arg.trim().parse();
                if lines_to_print.is_ok() {
                    ReadSubArg::Numeric(lines_to_print.unwrap())
                } else {
                    ReadSubArg::None
                }
            } else {
                ReadSubArg::None
            }
        }
        None => ReadSubArg::None,
    }
}

#[test]
fn test_date1() {
    let date = "2025-05-27";
    assert_eq!("2025-05-27", get_date(date).get_or_insert_default());
}

#[test]
fn test_date2() {
    let date = "2705";
    assert_eq!("2025-05-27", get_date(date).get_or_insert_default());
}
