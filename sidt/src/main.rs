use std::env;
use chrono::prelude::*;
use std::{fs, path::Path, sync::Arc};
use parquet::{
    file::{
        properties::WriterProperties,
        writer::SerializedFileWriter,
    },
    schema::parser::parse_message_type,
};

fn main(){
    let args: Vec<String> = env::args().collect();
    println!("{:?}",args);

    let text = &args[1..];

    let entry = text.join(" ");

    let local_date: DateTime<Local> = Local::now();

    let formatted_date: String = local_date.format("%Y-%m-%d").to_string();

    // Write to parquet

    let path = "data/journal.parquet";

    let message_type = "
  message schema {
    REQUIRED DateTime date;
    OPTIONAL String text;
  }
";
let schema = Arc::new(parse_message_type(message_type).unwrap());
let file = fs::File::create(&path).unwrap();

    println!("{:?}",formatted_date + " " + &entry);
}