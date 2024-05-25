use std::{env, fs::{File, OpenOptions}, io::{self, BufRead, Write}};
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

    let path: &str = "journal.txt";

    // Match args

    match_args(&args);

    // Open journal file

    let mut file = OpenOptions::new()
    .write(true)
    .append(true)
    .open(path)
    .unwrap();
    
    let Ok(lines) = read_lines(&path)
    else {
        println!("Cannot read file!");
        panic!()
    };

  
    // Print out all lines
    // TODO: Add a command for reading all lines

    // for line in lines{
    //     println!("{:?}",line.unwrap());
    // }

    let last_line = lines.last().unwrap().unwrap();
    let mut last_line_array:std::str::Split<&str>  = last_line.split(" ");
    println!("{:?}",&last_line_array);

    let last_date:&str  = last_line_array.next().unwrap();
    println!("{:?}",&last_date);

    // How to handle if two entries on the same day?
    // If last_date and formatted_date match, then get the previous entry and add to it (remove the new line)

    if last_date == formatted_date {
        // Get previous entry and remove suffix

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
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// Read out all lines

// Match args
fn match_args(args: &[String] ){

    let first_arg = &args[1];

match first_arg.as_str() {
    "--help" => println!("{:?}","Help"),
    "t" => println!("{:?}","Today"),
    &_ => println!("{:?}","Only god can help you")
};
}