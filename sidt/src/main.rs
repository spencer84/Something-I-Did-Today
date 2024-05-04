use std::env;
use chrono::prelude::*;

fn main(){
    let args: Vec<String> = env::args().collect();
    println!("{:?}",args);

    let text = &args[1..];

    let entry = text.join(" ");

    let local_date: DateTime<Local> = Local::now();

    let formatted_date: String = local_date.format("%Y-%m-%d").to_string();

    println!("{:?}",entry);
}