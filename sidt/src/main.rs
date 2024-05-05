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

    let row: Vec<(String, String)> = (formatted_date,entry);

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
let mut writer = SerializedFileWriter::new(file, schema, Default::default(), row).unwrap();
let mut row_group_writer = writer.next_row_group().unwrap();
while let Some(mut col_writer) = row_group_writer.next_column().unwrap() {
    col_writer.close().unwrap()
}
row_group_writer.close().unwrap();
writer.close().unwrap();
    println!("{:?}",formatted_date + " " + &entry);
}