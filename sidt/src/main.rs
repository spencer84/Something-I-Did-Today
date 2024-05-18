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

    let dated_entry: String =  formatted_date + " " + &entry+"\n";

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

    let last_entry: Vec<_> = buf.lines().collect();

    for line in lines{
        println!("{:?}",line);
    }

    
    // Write to text file
    file.write_all(&dated_entry.as_bytes()).expect("Could not write.");

    // How to handle if two entries on the same day?

    println!("{:?}",&dated_entry);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}