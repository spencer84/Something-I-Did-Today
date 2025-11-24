use rustyline::DefaultEditor;
use std::env::{self};

use chrono::{prelude::*, DateTime, Local, NaiveDate, TimeDelta};
use sidt::db::*;
use sidt::{assign_read_subarg, check_tag, get_date, get_help, is_reserved_value, update_date, ReadSubArg};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        get_help()
    } else {
        parse_args(&args)
    }

    fn parse_args(args: &[String]) {
        let first_arg: &String = &args[1];
        let current_time: i64 = Local::now().timestamp();

        // Match args
        match first_arg.as_str() {
            "-h" | "--help" => {
                // If arg passed, give specific help for that option
                get_help()
            }
            "-r" | "--read" => {
                // Check if second arg is a tag
                let second_arg = args.get(2);
                let parsed_read_arg = assign_read_subarg(&second_arg);
                match parsed_read_arg {
                    ReadSubArg::Tag(tag) => {
                        read_selected_tags(&tag, 5);
                    }
                    ReadSubArg::Numeric(number ) => {
                        read_selected_entries(number);
                    }
                    ReadSubArg::None => {
                        read_selected_entries(5);
                    }
                }
            }
            "-l" | "--last" => read_last_entry(),
            "-s" | "--search" => {
                let second_arg = args.get(2);
                match second_arg {
                    Some(_) => get_search_results(second_arg.unwrap()),
                    None => println!("Search term required..."),
                }
            }
            "-cd" | "--change-date" => update_date(args.to_vec()),
            "-d" | "--delete" => {
                // If there is a valid second arg (i.e. a specific date to be deleted), attempt to parse date
                let second_arg = args.get(2);

                let date: String;
                // Only delete date if second arg successfully parsed
                if second_arg.is_some() {
                    if get_date(&second_arg.unwrap()).is_some() {
                        date = get_date(&second_arg.unwrap()).unwrap();
                        delete_selected_entry(date);
                    }
                } else {
                    println!("No date specified");
                }
            }
            "-y" | "--yesterday" => {
                // Format yesterday's date
                let secs: i64 = -60 * 60 * 24;
                let nanos: u32 = 0;
                let delta: TimeDelta = TimeDelta::new(secs, nanos).unwrap();
                let yesterday = Local::now().checked_add_signed(delta).unwrap();
                println!("{}", &yesterday);

                let text: &[String] = &args[2..];

                let entry: String = text.join(" ");

                write_entry(
                    yesterday.format("%Y-%m-%d").to_string(),
                    entry,
                    yesterday.timestamp(),
                    current_time,
                );
            }
            "-e" | "--edit" => {
                // Try to parse date
                let second_arg = args.get(2);
                match second_arg {
                    Some(_) => {
                        let date = get_date(&second_arg.unwrap());
                        let entry = read_entry(date.clone()).unwrap();
                        let mut editor = DefaultEditor::new().unwrap();
                        match editor.readline_with_initial("", (&entry, "")) {
                            Ok(entry_result) => {
                                println!("New entry: {}", &entry_result);
                                update_entry(date.unwrap(), entry_result, current_time);
                            }
                            Err(error) => {
                                println!("Error: {}", error);
                            }
                        }
                    }
                    None => {
                        println!("Which entry to edit? Date argument missing...");
                    }
                }
                let _ = "Edit this entry!";
            }
            "-t" | "--tag" => {
                // TODO: Allow user to create a long form and short form of the tag
                // E.g. -m & --movie
                let tag: &String = &args[2];
                if is_reserved_value(tag){
                    println!("Tag is already a reserved flag...");
                }
                else {
                    create_tag(tag);
                }
                // -t movie -a m
                // this sets a long form and a short form

                // Create a tag for quick references in future
                // For example, lets say we want to create a tag to flag movies we've watched
                // We can create a movie tag with: sidt -t movie
                // Then we can reference it with two dashes:
                // sidt Hung out with friends --movie The Fast and Furious
                // This should render to: 18-10-2025: Hung out with friends
                // To access tag data: sidt -r --movie
                // This should render to: 18-10-2025: The Fast and Furious

                // TODO: Re-write this to handle specific args
                // let date_time = Local::now();
                // let formatted_date = date_time.format("%Y-%m-%d").to_string();

            }
            &_ => {
                // Try to handle Date arg
                let possible_date = get_date(&first_arg);

                //println!("Possible date value: {}",possible_date.as_ref().unwrap());

                // Need a datetime value for the entry_date (Stored as integer value)
                let date_time: DateTime<Local>;

                let formatted_date: String;

                let text: &[String];

                match possible_date {
                    Some(_) => {
                        // Can we move this logic to the date parsing function?
                        formatted_date = possible_date.unwrap();
                        let naive_date =
                            NaiveDate::parse_from_str(&formatted_date, "%Y-%m-%d").unwrap();
                        let naive_datetime = naive_date.and_time(NaiveTime::default());
                        date_time = Local.from_local_datetime(&naive_datetime).unwrap();
                        // Check for tags
                        let possible_tag: Option<String> = check_tag(&args[2]);
                        match possible_tag {
                            Some(tag) => {
                                println!("Tag: {}", &tag);
                                let tag_entry = &args[3..].join(" ");
                                write_tag(
                                    formatted_date,
                                    &tag,
                                    tag_entry,
                                    date_time.timestamp(),
                                    current_time,
                                )
                            }
                            None => {
                                text = &args[2..];
                                let entry: String = text.join(" ");
                                write_entry(
                                    formatted_date,
                                    entry,
                                    date_time.timestamp(),
                                    current_time,
                                );
                            }
                        }
                    }
                    _ => {
                        date_time = Local::now();
                        formatted_date = date_time.format("%Y-%m-%d").to_string();
                        text = &args[1..];
                        let entry: String = text.join(" ");
                        write_entry(formatted_date, entry, date_time.timestamp(), current_time);
                    }
                }
            }
        };
    }
}





