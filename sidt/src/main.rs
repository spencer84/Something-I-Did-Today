use std::{env, fs::{OpenOptions,File}, io::{self, Result, Lines, BufRead, BufReader, Write}};
use chrono::prelude::*;
use std::path::Path;

fn main(){
    let args: Vec<String> = env::args().collect();
    println!("{:?}",args);

    let text: &[String] = &args[1..];

    let entry: String = text.join(" ");

    let local_date: DateTime<Local> = Local::now();

    let formatted_date: String = local_date.format("%Y-%m-%d").to_string();

    let dated_entry: String =  formatted_date.clone() + " " + &entry+"\n";

    let path: &str = "journal.txt";

    // Open journal file

    let mut file = OpenOptions::new()
    .write(true)
    .append(true)
    .open(path)
    .unwrap();

    let buf = BufReader::new(&file);

    
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
    let mut last_line: String = lines.last().unwrap().unwrap();
    println!("{:?}",&last_line);

    let last_date: String = last_line.split(" ").take(1).collect();
    println!("{:?}",&last_date);

    // How to handle if two entries on the same day?
    // If last_date and formatted_date match, then get the previous entry and add to it (remove the new line)

    if last_date == formatted_date {
        // Get previous entry
        let mut previous_entry = last_line.split(" ").next().unwrap();
        println!("{:?}",&previous_entry);
        // Add latest entry
        &previous_entry.strip_suffix("\n") + " " + &entry;
    } else{
        // Write to text file
        file.write_all(&dated_entry.as_bytes()).expect("Could not write.");
    }

    



    println!("{:?}",&dated_entry);
}

// Quick function to read all lines from a file
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}