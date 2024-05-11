use std::env;
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

    // Write to text file

    let path = "journal.txt";
    fs::write(path, &dated_entry);

    println!("{:?}",&dated_entry);
}