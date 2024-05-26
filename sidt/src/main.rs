use std::{env, fs::{File, OpenOptions}, io::{self,  BufRead, BufReader, Lines, Write}};
use chrono::prelude::*;
use std::path::Path;

fn main(){
    let args: Vec<String> = env::args().collect();
    println!("{:?}",args);

    let text: &[String] = &args[1..];

    let entry: String = text.join(" ");

    let local_date: DateTime<Local> = Local::now();

    let formatted_date: String = local_date.format("%Y-%m-%d").to_string();

    let dated_entry: String =  formatted_date.clone() + " " + &entry;

    let path: &str = "/Users/samspencer/Documents/Rust/sidt/sidt/sidt/journal.txt";

    // Match args

    let first_arg: &String = &args[1];

    // Read from the existing text file
    // TODO: Handle if file doesn't exist (set up path for new file)
    let lines_vec: Vec<String> = read_lines(path);

    match first_arg.as_str() {
        "--help" => println!("{:?}","Help"),
        "t" => println!("{:?}","Today"),
        "r" => print_lines(&lines_vec),
        &_ => println!("{:?}","Only god can help you")
    };

    // Open journal file

    let mut file = OpenOptions::new()
    .write(true)
    .append(true)
    .open(path)
    .unwrap();
    


    // Identify last line
    let last_line = &lines_vec.last().unwrap();
    let mut last_line_array:std::str::Split<&str>  = last_line.split(" ");
    println!("{:?}",&last_line_array);
    



    let last_date:&str  = last_line_array.next().unwrap();
    println!("{:?}",&last_date);

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

// Quick function to read all lines from a file
fn read_lines<P>(filename: P) -> Vec<String>
where P: AsRef<Path>, {
    let file = File::open(filename);
    let lines = io::BufReader::new(file.unwrap()).lines();
    lines.into_iter().filter_map(|x| x.ok() ).collect()
}

// Print out all lines
fn print_lines(lines: &Vec<String>) {  
    for line in lines {
        println!("{}",&line)
    }
}

