use std::{env, fs::OpenOptions, io::Write};
use chrono::prelude::*;
use std::fs;

fn main(){
    let args: Vec<String> = env::args().collect();
    println!("{:?}",args);

    let text: &[String] = &args[1..];

    let entry: String = text.join(" ");

    let local_date: DateTime<Local> = Local::now();

    let formatted_date: String = local_date.format("%Y-%m-%d").to_string();

    let dated_entry: String =  formatted_date + " " + &entry;

    let path = "journal.txt";

    // Open journal file

    let mut file = OpenOptions::new()
    .write(true)
    .append(true)
    .open(path)
    .unwrap();

    // Write to text file
    file.write_all(&dated_entry.as_bytes());
    //fs::write(path, );

    println!("{:?}",&dated_entry);
}