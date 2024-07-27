use std::{char, env, fs::{File, OpenOptions}, io::{self,  BufRead, Write}, num::ParseIntError};
use chrono::prelude::*;
use std::path::Path;

fn main(){
    let args: Vec<String> = env::args().collect();
    println!("{:?}",args);

    let path: &str = "/Users/samspencer/Documents/Rust/sidt/sidt/sidt/journal.txt";
    let connection = sqlite::open(":memory:").unwrap();
    // Match args

    let first_arg: &String = &args[1];

    // Read from the existing text file
    // TODO: Handle if file doesn't exist (set up path for new file)
    let lines_vec: Vec<String> = read_lines(path);

    match first_arg.as_str() {
        "--help" => println!("{:?}","Help"),
        "t" => println!("{:?}","Today"),
        "r" => print_lines(&lines_vec, args),
        &_ => write_lines(path,args, lines_vec)
    };

   

}

// Quick function to read all lines from a file
fn read_lines<P>(filename: P) -> Vec<String>
where P: AsRef<Path>, {
    let file = File::open(filename);
    let lines = io::BufReader::new(file.unwrap()).lines();
    lines.into_iter().filter_map(|x| x.ok() ).collect()
}

// Print out lines 
fn print_lines(lines: &Vec<String>, args: Vec<String>) { 
    
    // Try to extract additional argument to identify number of lines to print
    if args.len() > 2 {
        let lines_to_print: Result<usize, ParseIntError> = args[2].trim().parse();
        if lines_to_print.is_ok() {
            let number = lines_to_print.unwrap();
            let selected_lines = lines.iter().rev().take(number);
            for line in selected_lines {
                println!("{}",line)
            }
        }
        else {
            println!("{}","Read argument must be numeric value. E.g. r 5 to read the last 5 lines.")
        }

    } 
    // Otherwise just print all lines
    
    else{
        for line in lines {
            println!("{}",&line)
        }
    }

}

// Write entry to file
fn write_lines(path: &str, args:Vec<String>, lines_vec: Vec<String>){
 
 // Open journal file
 let mut file = OpenOptions::new()
 .write(true)
 .append(true)
 .open(path)
 .unwrap();
 


 let attempt_date_extract: Option<String> = get_date(&args[1]);

 let journal_index;

 if attempt_date_extract.is_some(){
    journal_index = 2;
 }
 else {
    journal_index = 1;
 }

 let text: &[String] = &args[journal_index..];

 let entry: String = text.join(" ");

 let formatted_date: String = match attempt_date_extract {
    Some(date) => date,
    // Get today's date
    None => Local::now().format("%Y-%m-%d").to_string()
 };

 // Define which argument index to begin recording journal input from (whether date arg is used and can be skipped)



 // TODO: If first arg is successfully parsed as a date value, then skip that arg when writing content to file 

 let dated_entry: String =  formatted_date.clone() + " " + &entry;

 // Identify last line
 let last_line = &lines_vec.last().unwrap();
 let mut last_line_array:std::str::Split<&str>  = last_line.split(" ");

 let last_date:&str  = last_line_array.next().unwrap();

 // Handle if two entries on the same day
 // If last_date and formatted_date match, then get the previous entry and add to it (remove the new line)

 if last_date == formatted_date {


     // Combine previous entry and current entry
     let latest_entry = format!(" {entry}");
     // Add the full entry
     //file.write_all(&latest_entry.as_bytes()).expect("Could not write.");
     // Getting close. The problem now is that we need to overwite the previous entry

     // Combine prev
     file.write_all(&latest_entry.as_bytes()).expect("Could not write.");

 } else{
     // Write to text file
     let new_line: String = "\n".to_string();
     let dated_entry = new_line+&dated_entry;
     file.write_all(&dated_entry.as_bytes()).expect("Could not write.");
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