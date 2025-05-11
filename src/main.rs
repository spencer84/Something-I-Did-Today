use std::{char, env::{self}, io::Cursor, num::ParseIntError};
use chrono::{prelude::*, TimeDelta, NaiveDate};
use std::io::{self, Write};
// use termion::input::TermRead;
// use termion::event::Key;
// use termion::raw::IntoRawMode;

mod db; 
use crate::db::db::*;

fn main(){
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        get_help()
    }
    else {
        parse_args(&args)
    }

    fn parse_args(args: &[String]){
        let first_arg: &String = &args[1];
        let current_time: i64 = Local::now().timestamp();

        // Match args
        match first_arg.as_str() {
            "-h" | "--help" => {
                // If arg passed, give specific help for that option
                get_help()},
            "-r" | "--read" => print_lines(args.to_vec()),
            "-l" | "--last" => read_last_entry(),
            "-s" | "--search" => {
                //
                let second_arg = args.get(2);
                match second_arg {
                    Some(_) => get_search_results(second_arg.unwrap()),
                    None => println!("Search term required...")
                }
            },
            "-cd"| "--change-date" => update_date(args.to_vec()),
            "-d" | "--delete" => {
                // If there is a valid second arg (i.e. a specific date to be deleted), attempt to parse date
                let second_arg = args.get(2);

                let date: String;

                if second_arg.is_some() {
                    if get_date(&second_arg.unwrap()).is_some() {
                        date = get_date(&second_arg.unwrap()).unwrap()
                    }

                    else {
                        date = Local::now().format("%Y-%m-%d").to_string();
                    }
                }
                else {
                    date = Local::now().format("%Y-%m-%d").to_string();
                }

                // If new entry supplied, update that record instead of full deleting
                if args.len() > 2 {
                    let entry = args[2..].join(" ");
                    update_entry(date, entry);
                }
                else {
                    delete_selected_entry(date);
                }
            },
            "-y" | "--yesterday" => {
                // Format yesterday's date
                let secs: i64 = -60*60*24;
                let nanos: u32 = 0;
                let delta: TimeDelta = TimeDelta::new(secs, nanos).unwrap();
                let yesterday = Local::now().checked_add_signed(delta).unwrap();
                println!("{}",&yesterday);

                let text: &[String] = &args[2..];

                let entry: String = text.join(" ");

                write_entry(yesterday.format("%Y-%m-%d").to_string(), entry, yesterday.timestamp(), current_time);
            },
            "-e" | "--edit" => {
                let entry = "Edit this entry!";}
            //edit_entry(entry.to_string());},
            &_ => {

                // Try to handle Date arg
                let possible_date = get_date(&first_arg);

                //println!("Possible date value: {}",possible_date.as_ref().unwrap());

                // Need a datetime value for the entry_date (Stored as integer value)
                let date_time: DateTime<Local>;

                let formatted_date: String;

                let text: &[String];

                match possible_date {
                    Some(_) => {
                        formatted_date = possible_date.unwrap();
                        let naive_date = NaiveDate::parse_from_str(&formatted_date, "%Y-%m-%d").unwrap();
                        let naive_datetime = naive_date.and_time(NaiveTime::default());
                        date_time = Local.from_local_datetime(&naive_datetime).unwrap();
                        text = &args[2..];

                    }
                    _ => {
                        formatted_date = Local::now().format("%Y-%m-%d").to_string();
                        date_time = Local::now();
                        text = &args[1..];
                    }
                }



                let entry: String = text.join(" ");

                write_entry(formatted_date, entry, date_time.timestamp(), current_time);
            }
        };
    }


}

// // Quick function to read all lines from a file
// fn read_lines<P>(filename: P) {
//     read_entry();
// }
// where P: AsRef<Path>, {
//     let file = File::open(filename);
//     let lines = io::BufReader::new(file.unwrap()).lines();
//     lines.into_iter().filter_map(|x| x.ok() ).collect()
// }

// Print out lines 
fn print_lines( args: Vec<String>) { 
    
    // Try to extract additional argument to identify number of lines to print
    if args.len() > 2 {
        let lines_to_print: Result<usize, ParseIntError> = args[2].trim().parse();
        if lines_to_print.is_ok() {
            let number  = lines_to_print.unwrap();
            println!("Number of lines to print:{}",number);

            read_selected_entries(number);
        }
        // Handle arg for all records
        else if args[2].trim() == "a" || args[2].trim() == "all" {
            read_all_entries();
        }
        else {
            println!("{}","Read argument must be numeric value. E.g. r 5 to read the last 5 lines.")
        }

    } 
    // Otherwise just print all lines
    
    else{
        let number: usize = 5;
        read_selected_entries(number);
    }

}


// Try to get a date from the first argument. If first arg is not numeric/date type, then use the current date
fn get_date(arg: &str) -> Option<String>
    {
        let date: String = arg.to_string();
        let parsed_date: Option<String>;
        
        if contains_numbers(&date){
            let seperator_option = get_seperator(&date);
            // If there is a separator, split the string and re-join
            let numeric_string: String = match seperator_option {
                Some(_) => {
                    let date_copy = date.clone();
                    let split_date = date_copy.split(seperator_option.unwrap());
                    split_date.collect()
                },
                // Otherwise just use the numeric string
                None => date
            };

            // If parsed date has a String value, return that after formatting
            parsed_date = parse_numeric_string(numeric_string);
            
            return parsed_date
        }

        else{
            None
        }

    }

    // Evaluate if the input string has a numeric input
fn contains_numbers(string: &String) -> bool
{
    for num in 0..10{
        let numeric_char: char = char::from_digit(num as u32,10).unwrap();
        if string.contains(numeric_char){
            return true
        }
    }
    return false
}

// Try to identify the separator used
fn get_seperator(string: &String) -> Option<char>{
    if string.contains("\\"){
        let sep = "\\".to_string();
        let character:Vec<char> = sep.chars().collect();
        return Some(character[0])
    }
    if string.contains("/"){
        let sep = "/".to_string();
        let character:Vec<char> = sep.chars().collect();
        return Some(character[0])
    }
    if string.contains("-"){
        let sep = "-".to_string();
        let character:Vec<char> = sep.chars().collect();
        return Some(character[0])
    }
    if string.contains("."){
        let sep = ".".to_string();
        let character:Vec<char> = sep.chars().collect();
        return Some(character[0])
    }
    if string.contains("|"){
        let sep = "|".to_string();
        let character:Vec<char> = sep.chars().collect();
        return Some(character[0])
    }
    else {
        return None
    }

}

// Read a date string left to right--Accepts the following formats:
// D, DD, DDM, DDMM, DDMMYY, DDMMYYYY
fn parse_numeric_string(numeric_string: String) -> Option<String> {
    let local_date: DateTime<Local> = Local::now();
    let size = numeric_string.len();
    if size == 2 || size == 1 {
        // For single day input, assume current month/year
        let month = local_date.month();
        let year = local_date.year();
        let day = match_day_string(numeric_string);
        match day {
            Some(day) => return std::option::Option::Some(format_date(day, month, year)),
            None => return None
        }
        }
    if size > 2 && size < 5 {
        // For date/month input assume current year (how handle if not yet reached this date?)
        let year = local_date.year();
        // Split out the days
        let day = numeric_string[..2].parse::<i32>().unwrap();
        let month = numeric_string[2..].parse::<u32>().unwrap();
    
    if day_is_valid(day) && month_is_valid(month){
        return std::option::Option::Some(format_date(day, month, year))
    }
    else{
        return None
    }
    }
    // DDMMYY
    if size == 6 {
        let day = numeric_string[..2].parse::<i32>().unwrap();
        let month = numeric_string[2..4].parse::<u32>().unwrap();
        let year = numeric_string[4..].parse::<i32>().unwrap();
        if day_is_valid(day) && month_is_valid(month) && year_is_valid(year) {
            return std::option::Option::Some(format_date(day, month, year+2000))
        }
        else{
            return None
        }
    }
    // DDMMYYYY
    if size == 8 {
        println!("Made it to the date size 8");
        let day = numeric_string[..2].parse::<i32>().unwrap();
        let month = numeric_string[2..4].parse::<u32>().unwrap();
        let year = numeric_string[5..].parse::<i32>().unwrap();
        if day_is_valid(day) && month_is_valid(month) && year_is_valid(year) {
            return std::option::Option::Some(format_date(day, month, year))
        }
        // YYYYMMDD
        let day = numeric_string[7..].parse::<i32>().unwrap();
        let month = numeric_string[4..6].parse::<u32>().unwrap();
        let year = numeric_string[..4].parse::<i32>().unwrap();
        if day_is_valid(day) && month_is_valid(month) && year_is_valid(year) {
            return std::option::Option::Some(format_date(day, month, year))
        }
        else{
            return None
        }
    }

    // TODO: Catch inputs of different date patterns


    else {
        return None
    }
}

// Try to parse the day portion of the numeric input into a date value
fn match_day_string(day_string: String) -> Option<i32>{
    // Cast to int
    let day = day_string.parse::<i32>().unwrap();
    // Check if between 1 - (current month days)
    if &day <= &31 && &day > & 0{
        return Some(day)
    }
    else{
        None
    } 
}

// Check if day is valid
fn day_is_valid(day: i32) -> bool{
    if &day <= &31 && &day > &0 {
        true
    }
    else{
        false
    }
}

// Check if month is valid
fn month_is_valid(month: u32) -> bool{
    if &month <= &12 && &month > &0 {
        true
    }
    else{
        false
    }
}

// Check if year is valid (this is somewhat subjectve--what if someone wants to record what they did back in 1998; that's valid year...Maybe have an config option for this later?)
fn year_is_valid(year: i32) -> bool{
    let local_date: DateTime<Local> = Local::now();
    let current_year = local_date.year();
    if year < 100{
        return year <= current_year-2000 && year > 0
    }
    else {
        return year <= current_year && year > 2000
    }
}

// Return a date string for a numerically described date
fn format_date(day: i32, month: u32, year: i32) -> String {

    // Add date padding for month 
    let padded_month: String = if month < 10 {
        "0".to_owned()+&month.to_string()
        } else {
            month.to_string()
        };

    // Add date padding for day
    let padded_day: String = if day < 10 {
        "0".to_owned()+&day.to_string()
        } else {
            day.to_string()
        };

    let date: String = year.to_string() + "-" + &padded_month + "-" + &padded_day.to_string();
    date
}

// Return a list of arguments and useful information about how to use the program
fn get_help() {
    println!("Usage: sidt <entry>");
    println!("Options: ");
    println!("-h, --help                        Print help");
    println!("-r, --read <number>               Read last <number> lines (or use a/all for all entries");
    println!("-y, --yesterday <entry>           Write an entry for yesterday's date");
    println!("-l, --last                        Read last entry");
    println!("-cd, --change-date <old> <new>    Change an entry date");

}

fn update_date(args: Vec<String>){
    // Extract second and third args

    let old: Option<&String> = args.get(2);

    let new: Option<&String> = args.get(3);

    if old.is_some() && new.is_some() {
        // Try to get date values from args

        let old_date = get_date(old.unwrap());
        
        let new_date = get_date(new.unwrap());

        if old_date.is_some() && new_date.is_some(){
            // Apply the changes to the db
            change_date(&old_date.unwrap(), &new_date.unwrap())
        } 
        else{
            println!("Invalid date args")

        }
    }
    else{
        println!("Invalid date args")
    }


}

// fn edit_entry(previous_entry: String) {
//     // Prepopulate the terminal with the previous entry
//     use std::io::Cursor;
//     let cursor = Cursor::new(previous_entry);
//     cursor.consume(previous_entry.len());
//
//
// }