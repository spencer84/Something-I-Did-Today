use std::{env, fs::{File, OpenOptions}, io::{self, BufRead, Write}, os::unix::fs::FileExt};
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

    let mut last_line = lines.last().unwrap().unwrap();
    let mut last_line_array:std::str::Split<&str>  = last_line.split(" ");
    println!("{:?}",&last_line_array);

    let last_date:&str  = last_line_array.next().unwrap();
    println!("{:?}",&last_date);

    // How to handle if two entries on the same day?
    // If last_date and formatted_date match, then get the previous entry and add to it (remove the new line)

    if last_date == formatted_date {
        // Get previous entry and remove suffix
        last_line.pop().unwrap().to_string();
        println!("{:?}",&last_line);
        // Combine previous entry and current entry
        let latest_entry = format!("{last_line} {entry}\n");
        // Add the full entry
        //file.write_all(&latest_entry.as_bytes()).expect("Could not write.");
        file.write_at(&latest_entry.as_bytes(), 500).expect("Could not write.");

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