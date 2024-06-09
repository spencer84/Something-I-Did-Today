use std::{env, fs::{File, OpenOptions}, io::{self,  BufRead, BufReader, Lines, Write}, num::ParseIntError, char};
use chrono::prelude::*;
use std::path::Path;

fn main(){
    let args: Vec<String> = env::args().collect();
    println!("{:?}",args);

    let path: &str = "/Users/samspencer/Documents/Rust/sidt/sidt/sidt/journal.txt";

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
 
 let text: &[String] = &args[1..];

 let entry: String = text.join(" ");

 let local_date: DateTime<Local> = Local::now();

 let formatted_date: String = local_date.format("%Y-%m-%d").to_string();

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
fn get_date(arg: &str) -> String
    {
        let date: String = arg.to_string();
        if contains_numbers(date){
            let formatted_date: String;
            let seperator_option = get_seperator(date);
            
            let numeric_string = match seperator_option {
                Some(String) => &date.to_string().split(seperator_option.unwrap()).collect(),
                None => &date.to_string()
            };
        }
        else{
            // Get today's date
        }

        formatted_date
    }

    // Evaluate if the input string has a numeric input
fn contains_numbers(string: String) -> bool
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
fn get_seperator(string: String) -> Option<char>{
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
