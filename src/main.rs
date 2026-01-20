use chrono::{prelude::*, DateTime, Local, NaiveDate, TimeDelta};
use rustyline::DefaultEditor;
use sidt::db::*;
use sidt::{assign_read_subarg, build_entry, get_context, get_date, get_help, get_yesterday, is_reserved_value, update_date, Context, ReadSubArg};
use std::env::{self};
use std::slice::Iter;
use log;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut args_iter = args.iter();
    args_iter.next(); // Skip the first env arg
    // Initialize logger
    env_logger::Builder::from_default_env()
        .init();
    log::trace!("args: {:?}", args);
    if args.len() < 2 {
        get_help()
    } else {
        parse_args(args_iter)
    }

    fn parse_args(mut args: Iter<String>) {
        let first_arg: &String = args.next().unwrap();

        // Match args
        match first_arg.as_str() {
            "-h" | "--help" => {
                // If arg passed, give specific help for that option
                get_help()
            }
            "-r" | "--read" => {
                // Check if second arg is a tag
                let second_arg = args.next();
                let parsed_read_arg = assign_read_subarg(&second_arg);
                match parsed_read_arg {
                    ReadSubArg::Tag(tag) => {
                        read_selected_tags(&tag, 5);
                    }
                    ReadSubArg::Numeric(number) => {
                        read_selected_entries(number);
                    }
                    ReadSubArg::None => {
                        read_selected_entries(5);
                    }
                }
            }
            "-l" | "--last" => read_last_entry(),
            "-s" | "--search" => {
                let next_arg = args.next();
                let context = get_context(next_arg);
                match context {
                    Some(context) => {
                        let mut search_phrase = vec![next_arg.unwrap().clone()];
                        search_phrase.append(&mut args.cloned().collect::<Vec<String>>());
                        log::info!("search phrase: {:?}", &search_phrase.join(" "));
                        if search_phrase.is_empty() {
                            println!("Search term required...");
                        } else {
                            get_search_results(context, &search_phrase.join(" "));
                        }
                    }
                    _ => {
                        println!("Invalid command.")
                    }
                }
            }
            "-cd" | "--change-date" => {
                let next_arg = args.next();
                let context = get_context(next_arg);
                match context {
                    Some(context) => {
                        let old_date = args.next();
                        let new_date = args.next();
                        update_date(&context, old_date, new_date);
                    }
                    _ => {
                        println!("Invalid command.")
                    }
                }
            }
            "-d" | "--delete" => {
                // If there is a valid second arg (i.e. a specific date to be deleted), attempt to parse date
                let second_arg = args.next();

                let date: String;
                // Only delete date if second arg successfully parsed
                if second_arg.is_some() {
                    if get_date(&second_arg.unwrap()).is_some() {
                        date = get_date(&second_arg.unwrap()).unwrap();
                        delete_selected_entry(date);
                    }
                } else {
                    println!("No entry specified.");
                }
            }
            "-y" | "--yesterday" => {
                let yesterday = get_yesterday();
                println!("{}", &yesterday);
                let context = get_context(Some(first_arg)).unwrap();
                let next_arg = args.next().expect("Entry required after arg.");

                match context {
                    Context::MainEntry => {
                        let mut entry = build_entry(Context::MainEntry, next_arg, args);
                        entry.set_date(yesterday);
                        write_entry(entry)
                    }
                    Context::Tag(tag) => {
                        let next_arg = args.next();
                        match next_arg {
                            Some(arg) => {
                                let mut entry = build_entry(Context::Tag(tag), &arg, args);
                                entry.set_date(yesterday);
                                write_entry(entry);
                            }
                            _ => {
                                println!("No entry specified.");
                            }
                        }
                    }
                }
            }
            "-e" | "--edit" => {
                // Try to parse date
                let second_arg = args.next();
                match second_arg {
                    Some(_) => {
                        let date = get_date(&second_arg.unwrap());
                        let entry = read_entry(date.clone()).unwrap();
                        let current_time: i64 = Local::now().timestamp();
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
                let tag: &String = args.next().unwrap();
                if is_reserved_value(tag) {
                    println!("Tag is already a reserved flag...");
                } else {
                    let next_arg = args.next();
                    let possible_short_tag = args.next();
                    match next_arg {
                        Some(arg) => {
                            if arg == "-a" {
                                create_tag(tag, possible_short_tag);
                            } else {
                                create_tag(tag, None)
                            }
                        }
                        None => {
                            create_tag(tag, None);
                        }
                    }
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
                let context = get_context(Some(first_arg)).unwrap();
                match context {
                    Context::MainEntry => {
                        let entry = build_entry(Context::MainEntry, first_arg, args);
                        write_entry(entry);
                    }
                    Context::Tag(tag) => {
                        let next_arg = args.next();
                        match next_arg {
                            Some(arg) => {
                                let entry = build_entry(Context::Tag(tag), &arg, args);
                                write_entry(entry);
                            }
                            _ => {
                                println!("No entry specified.");
                            }
                        }
                    }
                }
            }
        };
    }
}
